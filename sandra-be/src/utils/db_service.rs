use super::models::Camera;
use crate::utils::models::{AccessLevel, Group, User};
use futures::stream::TryStreamExt;
use mongodb::{
  bson::{doc, oid::ObjectId, to_document},
  options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument},
  Client, Collection, IndexModel,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;

#[derive(Clone)]
pub struct DBService {
  user_collection: Collection<User>,
  group_collection: Collection<Group>,
}

static USER_COLLECTION: &str = "users";
static GROUP_COLLECTION: &str = "groups";

impl DBService {
  pub async fn init() -> Self {
    let uri = "mongodb://root:example@localhost:27017";
    let client = Some(
      Client::with_uri_str(uri)
        .await
        .expect("Could not connect to database"),
    );

    let sandra_db = client.unwrap().database("sandra");
    let user_collection = sandra_db.collection(USER_COLLECTION);
    let group_collection = sandra_db.collection(GROUP_COLLECTION);

    if user_collection
      .count_documents(doc! {})
      .await
      .expect("Could not get doc count")
      == 0
    {
      // Define the unique index on "username" field
      let index_options = IndexOptions::builder().unique(true).build();
      let index_model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(index_options)
        .build();

      // Create the index on "username" field
      user_collection
        .create_index(index_model)
        .await
        .expect("Could not create unique index for users");
    }

    DBService {
      user_collection: user_collection,
      group_collection: group_collection,
    }
  }

  pub async fn create_new_user(
    &self,
    username: Option<String>,
    password: Option<String>,
    gid: Option<ObjectId>,
  ) -> Result<bool, String> {
    match gid {
      Some(value) => {
        let group = self
          .group_collection
          .find_one(doc! { "id": gid.unwrap() })
          .await
          .expect("Error checking login user collection");
        let n_user = User {
          username,
          password,
          gid: Some(value),
          group_data: group,
          access_level: Some(AccessLevel::Admin),
        };
        self
          .user_collection
          .insert_one(n_user)
          .await
          .expect("Could not create new user");
        Ok(true)
      }
      None => {
        let n_g_id = ObjectId::new();
        let n_group = Group {
          id: n_g_id,
          cameras: Vec::new(),
        };
        let n_user = User {
          username,
          password,
          gid: Some(n_g_id),
          group_data: Some(n_group.clone()),
          access_level: Some(AccessLevel::Admin),
        };
        self
          .group_collection
          .insert_one(n_group)
          .await
          .expect("Could not create new group");
        self
          .user_collection
          .insert_one(n_user)
          .await
          .expect("Could not create new user");
        Ok(true)
      }
    }
  }

  pub async fn check_login(
    &self,
    username: Option<String>,
    password: Option<String>,
  ) -> Option<User> {
    let mut user = self
      .user_collection
      .find_one(doc! { "username": username.unwrap(), "password": password.unwrap() })
      .await
      .expect("Error checking login user collection")?;
    let group = self
      .group_collection
      .find_one(doc! { "id": user.gid.unwrap().to_string() })
      .await
      .expect("Error checking login user collection");
    user.group_data = group;
    Some(user.to_owned())
  }

  pub async fn add_camera_by_gid(
    &self,
    gid: Option<mongodb::bson::oid::ObjectId>, // bad practice, gives any auth user control to add camera to any group. SHould do this base on auth token.
    n_camera: Option<Camera>,
  ) -> Option<Vec<Camera>> {
    let filter = doc! { "id": gid.unwrap() };

    let n_cam_doc = to_document(&n_camera.unwrap()).expect("Convert of camera to document failed");
    let update_doc = doc! { "$push": { "cameras": n_cam_doc} };
    let n_group = self
      .group_collection
      .find_one_and_update(filter, update_doc)
      .return_document(ReturnDocument::After)
      .await
      .expect("Could not fund one and update for new camera");
    Some(n_group.unwrap().cameras)
  }

  pub async fn __get_all_users(&self) -> Result<Vec<User>, String> {
    let cursor = match self.user_collection.find(doc! {}).await {
      Ok(d) => d,
      Err(e) => {
        return Err("Error finding all for user collection".to_string());
      }
    };

    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
  }
}

impl Deref for DBService {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self
  }
}
