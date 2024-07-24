use mongodb::bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };
use strum_macros::{ Display, EnumString };
use std::fmt;

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
  id: String,
  name: String,
  desc: String,
  src_ip: String,
  username: String,
  password: String,
  onvif_port: String,
  rtsp_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
  pub id: ObjectId,
  pub cameras: Vec<Camera>,
}
