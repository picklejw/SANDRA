use mongodb::bson::oid::ObjectId;
use onvif::soap::client::Credentials; //discovery::Device,

use serde::{ Deserialize, Serialize };
use strum_macros::{ Display, EnumString };
use std::{ collections::HashMap, fmt };
use url::Url;
use uuid::Uuid;

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
  // pub url: Url,
  pub name: String,
  pub dev_srv_url: Result<Url, String>,
  pub ev_srv_url: Result<Url, String>,
  pub media_urls: Result<String, String>,
  pub credentials: Option<Credentials>,
  // pub addr: String,
  // pub port: String,
}

#[derive(Clone, Debug)]
pub struct Onvif_Ev_Msg {
  pub src_ip: String,
  pub topic: String,
  pub events: HashMap<String, String>,
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
