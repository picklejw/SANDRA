use crate::utils::models::{ CameraNet, Onvif_Ev_Msg };
use b_2::NotificationMessageHolderType;
use chrono::{ DateTime, Duration, Utc };
use local_ip_address::{ list_afinet_netifas, local_ip };
use onvif::{ discovery, soap::client::{ Client, ClientBuilder, Credentials } }; //discovery::Device,
use onvif_utils::devicemgmt::get_capabilities;
use onvif_utils::event::{ self, CreatePullPointSubscription, PullMessages };
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::{ collections::HashMap, fmt::Debug, num::ParseIntError };
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio::task;
use url::form_urlencoded;
use url::{ ParseError, Url };
use uuid::Uuid;

fn contains_any(s: &str, substrings: &[&str]) -> bool {
  substrings.iter().any(|&sub| s.contains(sub))
}

// #[derive(serde::Serialize)]
pub struct CameraList {
  pub devices: Vec<SubscribeEvents>,
  d_username: String,
  d_password: String,
}

impl CameraList {
  fn add_device(&mut self, dev: SubscribeEvents) {
    &self.devices.push(dev);
  }
  pub fn get_all_devices(&self) -> &Vec<SubscribeEvents> {
    &self.devices
  }

  pub async fn scan_for_devices(username: String, password: String) -> Self {
    let discovered_cameras = discover_onvif(username.clone(), password.clone()).await.into_iter();
    let mut sub_ev: Vec<SubscribeEvents> = Vec::new();
    for camera in discovered_cameras {
      let mut n_sub = SubscribeEvents::new(camera);
      sub_ev.push(n_sub);
    }
    CameraList {
      d_username: username,
      d_password: password,
      devices: sub_ev,
    }
  }
}

pub struct SubscribeEvents {
  pub notifier: Arc<Mutex<broadcast::Sender<Onvif_Ev_Msg>>>,
  pub device: CameraNet,
}
impl SubscribeEvents {
  fn new(device: CameraNet) -> Self {
    let (tx, _) = broadcast::channel(10);
    let notifier = Arc::new(Mutex::new(tx));

    let l_n_cn = Arc::clone(&notifier);
    let cln_dev = device.clone();
    task::spawn(async move {
      SubscribeEvents::start_watching(l_n_cn, cln_dev).await;
    });

    SubscribeEvents { notifier, device }
  }

  pub async fn sub_events<F>(&self, handler: F) -> task::JoinHandle<()>
    where F: Fn(&Onvif_Ev_Msg) + 'static + Send + Sync
  {
    let tx = self.notifier.clone();
    let mut rx = tx.lock().await.subscribe();
    task::spawn(async move {
      while let Ok(message) = rx.recv().await {
        handler(&message);
      }
    })
  }

  async fn start_watching(
    notifier: Arc<tokio::sync::Mutex<tokio::sync::broadcast::Sender<Onvif_Ev_Msg>>>,
    source: CameraNet
  ) {
    tokio::spawn(async move {
      async fn do_authenitcate(camera: &CameraNet) -> (Client, PullMessages) {
        let creds: Credentials = camera.credentials
          .clone()
          .expect("Need credentials for ONVIF event subscription");

        let event_client = ClientBuilder::new(
          &camera.ev_srv_url
            .clone()
            .expect(
              "Could not find event service url, if using autoconfig then open a ticket or make a PR :)"
            )
        )
          .credentials(Some(creds.clone()))
          .auth_type(onvif::soap::client::AuthType::UsernameToken)
          .build();

        let create_pull_sub_request = CreatePullPointSubscription {
          initial_termination_time: None,
          filter: Some(b_2::FilterType {
            topic_expression: Some(b_2::TopicExpressionType {
              dialect: "http://www.onvif.org/ver10/tev/topicExpression/ConcreteSet".to_string(),
              inner_text: "tns1:RuleEngine//.".to_string(),
            }),
          }),
          subscription_policy: None,
        };
        let create_pull_puint_sub_response = event::create_pull_point_subscription(
          &event_client,
          &create_pull_sub_request
        ).await;
        let camera_sub = create_pull_puint_sub_response.unwrap();

        let uri: Url = Url::parse(&camera_sub.subscription_reference.address).unwrap();
        let inst_pull_msg_client = ClientBuilder::new(&uri)
          .credentials(Some(creds.clone()))
          .auth_type(onvif::soap::client::AuthType::UsernameToken)
          .build();
        let inst_pull_messages_request = PullMessages {
          message_limit: 32,
          timeout: xsd_types::types::Duration {
            seconds: 1.0,
            ..Default::default()
          },
        };
        return (inst_pull_msg_client, inst_pull_messages_request);
      }
      let (mut pull_msg_client, mut pull_messages_request) = do_authenitcate(&source).await;

      // Check Loop
      let mut termination_time: DateTime<Utc> = Utc::now();
      loop {
        // Here check if termination time is past now to reauth.
        if Some(termination_time).is_some() {
          let look_ahead_time = Utc::now() + Duration::seconds(1);
          if termination_time < look_ahead_time {
            let (new_pull_msg_client, new_pull_messages_request) = do_authenitcate(&source).await;
            pull_msg_client = new_pull_msg_client;
            pull_messages_request = new_pull_messages_request;
          }
        }

        // Do check for pull messages
        let pull_messages_response = event::pull_messages(
          &pull_msg_client,
          &pull_messages_request
        ).await;
        let mut msg = match pull_messages_response {
          Ok(msg) => msg,
          Err(e) => {
            println!("Error: {:?}", e);
            continue;
          }
        };
        // Set termination time to keep track of when we need to re auth.
        let now = Utc::now();
        let remote_now = msg.current_time.to_chrono_datetime().to_utc();
        let diff = remote_now - now;
        let remote_exp = msg.termination_time.to_chrono_datetime().to_utc();
        termination_time = remote_exp - diff;

        if !msg.notification_message.is_empty() {
          let nn = &msg.notification_message[0];
          let mut events: HashMap<String, String> = HashMap::new();
          // println!("{:#?}", nn);

          let ev_name = nn.topic.inner_text.clone().split('/').last().unwrap().to_string();

          let smart_substrings = ["Detect"];
          if contains_any(&ev_name, &smart_substrings) {
            events.insert(
              "Smart_Event".to_string(),
              nn.topic.inner_text.clone().split('/').last().unwrap().to_string()
            );
          }
          // let ev_msg_data = nn.message.msg.data.simple_item.into_iter();
          for msg_dat in nn.message.msg.data.simple_item.iter() {
            if msg_dat.name == "IsMotion" {
              // && msg_dat.value == "false"
              // do_skip = true;
              continue;
            }
            events.insert(msg_dat.name.to_owned(), msg_dat.value.to_owned());
          }

          if events.len() > 0 {
            let _ = notifier.lock().await.send(Onvif_Ev_Msg {
              src_ip: source.ev_srv_url.clone().unwrap().to_string(),
              topic: String::from(&nn.topic.inner_text),
              events,
            });
          }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
      }
    });
  }
}

fn is_ipv6_link_local(ip: &IpAddr) -> bool {
  match ip {
    IpAddr::V6(addr) => {
      let link_local_prefix = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0);
      let mask = Ipv6Addr::new(0xffc0, 0, 0, 0, 0, 0, 0, 0);

      (addr & mask) == link_local_prefix
    }
    _ => false,
  }
}

fn format_rtsp_with_auth(username: &str, password: &str, rtsp_url: String) -> String {
  let proto_str = "rtsp://";
  let parts: Vec<&str> = rtsp_url.split(&proto_str).collect();
  let encoded_usr = form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
  let encoded_pw = form_urlencoded::byte_serialize(password.as_bytes()).collect::<String>();
  proto_str.to_owned() + &encoded_usr + ":" + &encoded_pw + "@" + parts[1]
}

struct GotCapabilities {
  ev_srv: Result<Url, String>,
}

pub async fn discover_onvif(username: String, password: String) -> Vec<CameraNet> {
  let mut all_local_ips: Vec<&IpAddr> = Vec::new();
  let network_interfaces = list_afinet_netifas().unwrap();
  for (_, ip) in network_interfaces.iter() {
    all_local_ips.push(ip);
  }

  use futures_util::stream::StreamExt;
  let mut onvif_results: Vec<CameraNet> = Vec::new();
  let credentials = Credentials {
    username: username.clone(),
    password: password.clone(),
  };

  for lan_ip in all_local_ips {
    let mut disc_build = discovery::DiscoveryBuilder::default();
    if lan_ip.is_loopback() || is_ipv6_link_local(lan_ip) {
      continue;
    }

    disc_build.listen_address(lan_ip.to_owned());
    if let Ok(safe_build) = disc_build.run().await {
      println!("{}", lan_ip);
      let devices: Vec<onvif::discovery::Device> = safe_build.collect().await;
      println!("{:#?}", &devices);
      for dev in devices {
        for d_url in dev.urls {
          let ttr = ClientBuilder::new(&d_url)
            .credentials(Some(credentials.clone()))
            // .auth_type(auth_type.clone())
            .build();
          let profiles = onvif_utils::media::get_profiles(&ttr, &Default::default()).await;
          let stream_uri_response = onvif_utils::media::get_stream_uri(
            &ttr,
            &(onvif_utils::media::GetStreamUri {
              profile_token: onvif_utils::onvif::ReferenceToken(
                profiles.as_ref().unwrap().profiles.first().unwrap().token.0.clone()
              ),
              stream_setup: onvif_utils::onvif::StreamSetup {
                stream: onvif_utils::onvif::StreamType::RtpUnicast,
                transport: onvif_utils::onvif::Transport {
                  protocol: onvif_utils::onvif::TransportProtocol::Rtsp,
                  tunnel: vec![],
                },
              },
            })
          ).await;
          let stream_uri: String = stream_uri_response.expect(
            "Could not get RTSP uri"
          ).media_uri.uri;
          let capabilities: Result<GotCapabilities, String> = match
            onvif_utils::devicemgmt::get_capabilities(&ttr, &Default::default()).await
          {
            Ok(capabilities) => {
              // capabilities.capabilities.media[0].x_addr;
              let ev_addr = &capabilities.capabilities.events[0].x_addr;

              Ok(GotCapabilities {
                ev_srv: Url::parse(&ev_addr).map_err(|e| e.to_string()),
              })
            }
            Err(error) => {
              println!("Failed to fetch capabilities: {}", error);
              Err(error.to_string())
            }
          };

          match capabilities {
            Ok(got_cap) => {
              onvif_results.push(CameraNet {
                name: Uuid::new_v4().to_string(),
                dev_srv_url: Ok(d_url),
                ev_srv_url: got_cap.ev_srv,
                media_urls: Ok(format_rtsp_with_auth(&username, &password, stream_uri)),
                credentials: Some(credentials.clone()),
              });
            }
            Err(err) => {
              onvif_results.push(CameraNet {
                name: Uuid::new_v4().to_string(),
                dev_srv_url: Ok(d_url),
                ev_srv_url: Err("Could not get capabilities".to_string()),
                media_urls: Err("Could not get capabilities".to_string()),
                credentials: Some(credentials.clone()),
              });
            }
          }
        }
      }
    }
  }

  println!("Done looking");
  onvif_results
}

// pub async fn auto_discover_and_subscribe(default_username: String, default_pw: String) {
//   // tokio::spawn(async {
//   let onvif_cameras = discover_onvif(default_username, default_pw).await.into_iter();
//   for camera in onvif_cameras {
//     println!("{:#?}", camera.ev_srv_url);
//     tokio::spawn(async move {
//       let mut n_sub = SubscribeEvents::new(camera);
//       n_sub.sub_events(|msg: &Onvif_Ev_Msg| {
//         println!("Received message: {:#?}", msg);
//       }).await;
//     });
//   }
//   // });
// }
