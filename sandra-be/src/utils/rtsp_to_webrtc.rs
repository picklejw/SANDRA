use crate::utils::models::{AddCameraFeedParam, SPDIncomming};
use ctrlc;
use std::collections::HashMap;
use std::process;
use std::process::Command;
use std::sync::{Arc, Mutex};

pub struct RtspController {
  pub rtsp_url: String,
  pub name: String,
}

impl RtspController {
  pub async fn get_codec_info(&self, suuid: String) -> Result<String, String> {
    let url = format!("http://127.0.0.1:8081/stream/codec/{}", suuid); // TODO get port for service
    match reqwest::get(&url).await {
      Ok(response) => {
        if response.status().is_success() {
          let rec_body = response.text().await.expect("");
          println!("Response Body: {}", rec_body);
          Ok(rec_body)
        } else {
          println!("Request failed with status: {}", response.status());
          Err(
            response
              .text()
              .await
              .expect("Could not parse server response"),
          )
        }
      }
      Err(e) => {
        println!("Error occurred: {}", e);
        Err(e.to_string())
      }
    }
  }

  pub async fn get_remote_spd(
    &self,
    suuid: String,
    body_fwd: SPDIncomming,
  ) -> Result<String, String> {
    match reqwest::Client::new()
      .post("http://127.0.0.1:8081/stream/receiver/".to_string() + &suuid) // TODO get port for service
      .form(&body_fwd)
      .send()
      .await
    {
      Ok(response) => {
        if response.status().is_success() {
          let rec_body = response.text().await.unwrap();
          println!("Response str Body: {}", rec_body);
          Ok(rec_body)
        } else {
          Err(
            response
              .text()
              .await
              .expect("Could not parse server response"),
          )
        }
      }
      Err(e) => Err(e.to_string()),
    }
  }
}

pub struct WebRTCManager {
  rtsp_controller: Mutex<HashMap<String, Arc<RtspController>>>,
}

impl WebRTCManager {
  pub fn new(port: &str) -> Self {
    let command = format!("HTTP_PORT='{}' ./bin/RTSPtoWebRTC", port);
    let child = Command::new("sh")
      .arg("-c")
      .arg(&command)
      .spawn()
      .expect("Could not spawn webrtc process");

    let child_arc_mutex = Arc::new(Mutex::new(child));

    let mut inst = WebRTCManager {
      rtsp_controller: Mutex::new(HashMap::new()),
    };

    let child_ctrlc = Arc::clone(&child_arc_mutex);

    ctrlc::set_handler(move || {
      println!("Cleaning up before exit...");

      let mut to_stop = child_ctrlc.lock().expect("Failed to lock WebRTCManager");
      if let Err(_) = to_stop.wait() {
        println!("Safely stopped webrtc runner");
      }
      if let Err(_) = to_stop.kill() {
        println!("Killed webrtc runner");
      }
      process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    inst
  }

  pub async fn add_rtsp_url(&self, name: String, url: String) -> Result<bool, String> {
    let body = AddCameraFeedParam {
      name: name.clone(),
      url: url.clone(),
    };
    match reqwest::Client::new()
      .post("http://127.0.0.1:8081/add_camera_feed".to_string()) // todo  update port
      .body(serde_json::to_string(&body).unwrap())
      .send()
      .await
    {
      Ok(response) => {
        if response.status().is_success() {
          let mut controllers_lock = self.rtsp_controller.lock().unwrap();
          let n_ctrlr = RtspController {
            rtsp_url: url.clone(),
            name: name.clone(),
          };
          controllers_lock.insert(name, Arc::new(n_ctrlr));
          Ok(true)
        } else {
          Err(
            response
              .text()
              .await
              .expect("Could not parse server error response"),
          )
        }
      }
      Err(e) => Err(e.to_string()),
    }
  }

  pub fn get_rtsp_controller(&self, name: &str) -> Option<Arc<RtspController>> {
    let controllers_lock = self.rtsp_controller.lock().unwrap();
    controllers_lock.get(name).cloned()
  }
}
