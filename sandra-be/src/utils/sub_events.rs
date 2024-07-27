use onvif::{ discovery, soap::client::{ Client, ClientBuilder, Credentials } }; //discovery::Device,
use onvif_utils::event::{ self, CreatePullPointSubscription, PullMessages };
use onvif_utils::devicemgmt::get_capabilities;
use url::Url;
use chrono::{ DateTime, Duration, Utc };
use b_2::{ NotificationMessageHolderType };
use std::{ collections::HashMap, fmt::Debug, num::ParseIntError };
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
use crate::utils::models::{ CameraNet, Onvif_Ev_Msg };
use std::sync::{ Arc };
use std::sync::mpsc::channel;
use std::thread;
use tokio::sync::{ Mutex };
use local_ip_address::{ local_ip, list_afinet_netifas };
use tokio::sync::broadcast;
use tokio::task;

fn contains_any(s: &str, substrings: &[&str]) -> bool {
  substrings.iter().any(|&sub| s.contains(sub))
}

struct SubscribeEvents {
  notifier: Arc<Mutex<broadcast::Sender<Onvif_Ev_Msg>>>,
  device: CameraNet,
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

pub async fn discover_onvif(username: String, password: String) -> Vec<CameraNet> {
  let mut all_local_ips: Vec<&IpAddr> = Vec::new();
  let network_interfaces = list_afinet_netifas().unwrap();
  for (_, ip) in network_interfaces.iter() {
    all_local_ips.push(ip);
  }

  use futures_util::stream::StreamExt;
  let mut onvif_results: Vec<CameraNet> = Vec::new();
  let credentials = Credentials {
    username,
    password,
  };

  for lan_ip in all_local_ips {
    let mut disc_build = discovery::DiscoveryBuilder::default();
    if lan_ip.is_loopback() || is_ipv6_link_local(lan_ip) {
      continue;
    }

    disc_build.listen_address(lan_ip.to_owned());

    let devices: Vec<onvif::discovery::Device> = disc_build.run().await.unwrap().collect().await;
    for dev in devices {
      for d_url in dev.urls {
        let ttr = ClientBuilder::new(&d_url)
          .credentials(Some(credentials.clone()))
          // .auth_type(auth_type.clone())
          .build();
        let ev_srv: Result<url::Url, String> = match
          onvif_utils::devicemgmt::get_capabilities(&ttr, &Default::default()).await
        {
          Ok(capabilities) => {
            let ev_addr = &capabilities.capabilities.events[0].x_addr;
            Url::parse(&ev_addr).map_err(|e| e.to_string())
          }
          Err(error) => {
            println!("Failed to fetch capabilities: {}", error);
            Err(error.to_string())
          }
        };

        onvif_results.push(CameraNet {
          dev_srv_url: Ok(d_url),
          ev_srv_url: ev_srv,
          credentials: Some(credentials.clone()),
        });
      }
    }
  }

  println!("Done looking");
  onvif_results
}

pub async fn auto_discover_and_subscribe(default_username: String, default_pw: String) {
  // tokio::spawn(async {
  let onvif_cameras = discover_onvif(default_username, default_pw).await.into_iter();
  for camera in onvif_cameras {
    tokio::spawn(async move {
      let mut n_sub = SubscribeEvents::new(camera);
      n_sub.sub_events(|msg: &Onvif_Ev_Msg| {
        println!("Received message: {:#?}", msg);
      }).await;
    });
  }
  // });
}
