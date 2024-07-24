use onvif::{ soap::client::{ Client, ClientBuilder, Credentials } }; //discovery::Device,
use schema::event::{ self, CreatePullPointSubscription, PullMessages };
use url::Url;
use chrono::{ DateTime, Duration, Utc };
use b_2::{ NotificationMessageHolderType };
use std::num::ParseIntError;
pub struct Source {
  pub src_ip: &'static str,
  pub proto_url: &'static str,
  pub username: &'static str,
  pub password: &'static str,
  // pub device_service_path: &'static str,
  pub event_service_path: &'static str,
}

impl Default for Source {
  fn default() -> Self {
    Source {
      src_ip: "127.0.0.1:8080",
      proto_url: "http://",
      // device_service_path: "/onvif/device_service",
      username: "admin",
      password: "password",
      event_service_path: "/onvif/event_service",
    }
  }
}

pub async fn subscribe(
  source: Source,
  on_event: fn(ev: &NotificationMessageHolderType)
) -> fn() -> Result<bool, ParseIntError> {
  let camera_ip = "10.0.20.232:8000";
  let username = source.username;
  let password = source.password;

  async fn do_authenitcate(
    username: &'static str,
    password: &'static str,
    camera_ip: &'static str
  ) -> (Client, PullMessages) {
    let camera: Source = Source {
      src_ip: camera_ip,
      username: username,
      password: password,
      ..Default::default()
    };

    let creds: Credentials = Credentials {
      username: camera.username.to_string(),
      password: camera.password.to_string(),
    };

    let mut ev_path = camera.proto_url.to_string();
    ev_path.push_str(&camera.src_ip);
    ev_path.push_str(&camera.event_service_path);

    let event_client = ClientBuilder::new(&Url::parse(ev_path.as_str()).unwrap())
      .credentials(Some(creds))
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
    let creds: Credentials = Credentials {
      username: camera.username.to_string(),
      password: camera.password.to_string(),
    };
    let inst_pull_msg_client = ClientBuilder::new(&uri)
      .credentials(Some(creds))
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

  let (mut pull_msg_client, mut pull_messages_request) = do_authenitcate(
    &username,
    &password,
    &camera_ip
  ).await;

  // Check Loop
  let mut termination_time: DateTime<Utc> = Utc::now();
  loop {
    // Here check if termination time is past now to reauth.
    if Some(termination_time).is_some() {
      let lookAheadTime = Utc::now() + Duration::seconds(1);
      if termination_time < lookAheadTime {
        let (new_pull_msg_client, new_pull_messages_request) = do_authenitcate(
          &username,
          &password,
          &camera_ip
        ).await;
        pull_msg_client = new_pull_msg_client;
        pull_messages_request = new_pull_messages_request;
      }
    }

    // Do check for pull messages
    let pull_messages_response = event::pull_messages(
      &pull_msg_client,
      &pull_messages_request
    ).await;
    let msg = match pull_messages_response {
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
      // do checks here, if there are REAL events only to do the callback..
      let ref ev_msg = &msg.notification_message[0];
      on_event(*ev_msg);
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
}
