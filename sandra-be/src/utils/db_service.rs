use crate::utils::models::{ AccessLevel, Group, User };
use bson::Document;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{ bson::{ self, doc }, Client, Collection };
use std::ops::Deref;

#[derive(Clone)]
pub struct DBService {
  user_collection: Collection<User>,
  group_collection: Collection<Group>,
}

impl DBService {
  pub async fn init() -> Self {
    let uri = "mongodb://root:example@localhost:27017";
    let client = Some(Client::with_uri_str(uri).await.expect("Could not connect to database"));

    let sandra_db = client.unwrap().database("sandra");

    DBService {
      user_collection: sandra_db.collection("users"),
      group_collection: sandra_db.collection("groups"),
    }
  }

  pub async fn create_new_user(
    &self,
    username: Option<String>,
    password: Option<String>,
    gid: Option<ObjectId>
  ) -> Result<bool, String> {
    match gid {
      Some(value) => {
        let n_user = User {
          username,
          password,
          gid: Some(value),
          access_level: Some(AccessLevel::Admin),
        };
        self.user_collection.insert_one(n_user).await.expect("Could not create new user");
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
          access_level: Some(AccessLevel::Admin),
        };
        self.group_collection.insert_one(n_group).await.expect("Could not create new group");
        self.user_collection.insert_one(n_user).await.expect("Could not create new user");
        Ok(true)
      }
    }
    // self.sandra_db.unwrap().collection(collection_name)
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
