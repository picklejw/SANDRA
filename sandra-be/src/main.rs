use utils::sub_events;
mod utils;
use b_2::NotificationMessageHolderType;

#[tokio::main]
async fn main() {
  let feed = sub_events::Source{
    src_ip: "admin",
    username: "admin",
    password: "password2011",
    ..Default::default()
  };
  fn read_events (ev: &NotificationMessageHolderType) {
    println!("ev!!");
    println!("{}", ev.topic.inner_text);
    let ev_attib = ev.message.msg.data.simple_item.iter().next().unwrap();
    println!("{:?}", ev_attib.name);
    println!("{:?}", ev_attib.value);
  }
  sub_events::subscribe(feed, read_events).await;
}


