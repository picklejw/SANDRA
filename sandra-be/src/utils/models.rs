use mongodb::bson::oid::ObjectId;
use onvif::soap::client::Credentials; //discovery::Device,

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub username: Option<String>,
  pub password: Option<String>,
  pub gid: Option<ObjectId>,
  pub access_level: Option<AccessLevel>,
  pub group_data: Option<Group>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AccessLevel {
  Admin = 0,
  Manager = 1,
}

impl fmt::Display for AccessLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Camera {
  pub name: String,
  pub rtsp_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
  pub id: ObjectId,
  pub cameras: Vec<Camera>,
}

#[derive(Clone)]
pub struct CameraNet {
  pub name: String,
  pub dev_srv_url: Result<Url, String>,
  pub ev_srv_url: Result<Url, String>,
  pub media_urls: Result<String, String>,
  pub credentials: Option<Credentials>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OnvifEvMsg {
  pub src_uri: String,
  pub topic: String,
  pub events: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WsMsg {
  pub ev: String,
  pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SPDIncomming {
  pub suuid: Option<String>,
  pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCameraFeedParam {
  pub url: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsJSON {
  pub ev: String,
  pub msg: String,
}

#[derive(Clone)]
pub struct WsUser {
  pub username: String,
  pub send_msg: tokio::sync::mpsc::Sender<std::string::String>,
  pub gid: String,
}
