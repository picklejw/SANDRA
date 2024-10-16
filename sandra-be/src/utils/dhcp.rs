use dhcp4r::server::Server;
use dhcp4r::{options, packet, server};
use pnet::datalink;
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{Ipv4Addr, UdpSocket};
use std::ops::Add;
use std::process::Command;
use std::str::FromStr;
use std::time::{Duration, Instant};

// untested and incomplete, does not work on osx due to security restrctions
pub fn start_dhcp() {
  if cfg!(target_os = "macos") {
    println!("This is macOS, you need to setup your own DHCP server. Sorry");
    return;
  }
  let new_ip = "192.168.58.1";
  let broadcast_ip = "10.33.32.255";
  let interface_n = read_user_selection_for_interface();
  set_ip_to_interface(interface_n.trim(), new_ip);
  create_dhcp_service(new_ip, broadcast_ip)
}

fn read_user_selection_for_interface() -> String {
  // List available network interfaces
  let interfaces = datalink::interfaces();
  for interface in interfaces {
    println!("Interface: {}", interface.name);
  }

  let mut interface_name = String::new();
  print!("Enter the interface name: ");
  io::stdout().flush().unwrap(); // Ensure the prompt is displayed
  io::stdin()
    .read_line(&mut interface_name)
    .expect("Failed to read line");
  return interface_name;
}

fn set_ip_to_interface(inteface_str: &str, ip_str: &str) {
  // Change the IP address using a system command
  let output = Command::new("ip")
    .args(&["addr", "add", &ip_str, "dev", &inteface_str])
    .output();

  match output {
    Ok(output) => {
      if output.status.success() {
        println!("Successfully set IP address to {}", ip_str);
      } else {
        // If the command failed, print the error output
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", stderr);
      }
    }
    Err(e) => {
      eprintln!("Failed to execute command: {}", e);
    }
  }

  // // Check if the command was successful
  // if output.status.success() {
  // } else {
  //   eprintln!("Failed to set IP address: {:?}", output);
  // }
}

// DHCP Stuff
struct MyServer {
  ip_addr: String,
  leases: HashMap<Ipv4Addr, ([u8; 6], Option<Instant>)>, // Ipv4Addr -> (MAC address, lease duration) mapping
  last_lease: u32,
  lease_duration: Duration,
}

impl server::Handler for MyServer {
  fn handle_request(&mut self, server: &server::Server, in_packet: packet::Packet) {
    const LEASE_NUM: u32 = 252;
    const IP_START: [u8; 4] = [10, 33, 32, 2];
    const IP_START_NUM: u32 = u32::from_be_bytes(IP_START);

    match in_packet.message_type() {
      Ok(options::MessageType::Discover) => {
        // Otherwise prefer existing (including expired if available)
        if let Some(ip) = self.current_lease(&in_packet.chaddr) {
          println!("Sending Reply to discover");
          reply(server, options::MessageType::Offer, in_packet, &ip);
          return;
        }
        // Otherwise choose a free ip if available
        for _ in 0..LEASE_NUM {
          self.last_lease = (self.last_lease + 1) % LEASE_NUM;
          if self.available(&in_packet.chaddr, &(IP_START_NUM + &self.last_lease).into()) {
            println!("Sending Reply to discover");
            reply(
              server,
              options::MessageType::Offer,
              in_packet,
              &(IP_START_NUM + &self.last_lease).into(),
            );
            break;
          }
        }
      }

      Ok(options::MessageType::Request) => {
        // Ignore requests to alternative DHCP server
        if !server.for_this_server(&in_packet) {
          //println!("Not for this server");
          // return;
        }

        let req_ip = match in_packet.option(options::REQUESTED_IP_ADDRESS) {
          Some(options::DhcpOption::RequestedIpAddress(x)) => *x,
          _ => in_packet.ciaddr,
        };
        for (ip, (mac, _)) in &self.leases {
          println!("IP: {:?}, MAC: {:?}", ip, mac);
        }
        if let Some(ip) = self.current_lease(&in_packet.chaddr) {
          println!("Found Current Lease");
          reply(server, options::MessageType::Ack, in_packet, &ip);
          return;
        }
        if !&self.available(&in_packet.chaddr, &req_ip) {
          println!("Sending Reply to Request");
          nak(server, in_packet, "Requested IP not available");
          return;
        }
        self.leases.insert(
          req_ip,
          (
            in_packet.chaddr,
            Some(Instant::now().add(self.lease_duration)),
          ),
        );
        println!("Sending Reply to Request");
        reply(server, options::MessageType::Ack, in_packet, &req_ip);
      }

      Ok(options::MessageType::Release) | Ok(options::MessageType::Decline) => {
        // Ignore requests to alternative DHCP server
        if !server.for_this_server(&in_packet) {
          return;
        }
        if let Some(ip) = self.current_lease(&in_packet.chaddr) {
          self.leases.remove(&ip);
        }
      }

      // TODO - not necessary but support for dhcp4r::INFORM might be nice
      _ => {}
    }
  }
}

impl MyServer {
  fn available(&self, chaddr: &[u8; 6], addr: &Ipv4Addr) -> bool {
    const IP_START: [u8; 4] = [10, 33, 32, 2];
    const LEASE_NUM: u32 = 252;
    const IP_START_NUM: u32 = u32::from_be_bytes(IP_START);
    let pos: u32 = (*addr).into();
    pos >= IP_START_NUM
      && pos < IP_START_NUM + LEASE_NUM
      && (match self.leases.get(addr) {
        Some((mac, expiry)) => {
          *mac == *chaddr || expiry.map_or(true, |exp| Instant::now().gt(&exp))
        }
        None => true,
      })
  }
  fn current_lease(&self, chaddr: &[u8; 6]) -> Option<Ipv4Addr> {
    for (i, v) in &self.leases {
      if v.0 == *chaddr {
        return Some(*i);
      }
    }
    None
  }
}

fn reply(
  s: &server::Server,
  msg_type: options::MessageType,
  req_packet: packet::Packet,
  offer_ip: &Ipv4Addr,
) {
  const LEASE_DURATION_SECS: u32 = 86400;
  const SUBNET_MASK: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 0);
  const ROUTER_IP: Ipv4Addr = Ipv4Addr::new(10, 33, 32, 1);
  const DNS_IPS: [Ipv4Addr; 1] = [
    // Google DNS servers
    Ipv4Addr::new(8, 8, 8, 8),
  ];
  let _ = s.reply(
    msg_type,
    vec![
      options::DhcpOption::IpAddressLeaseTime(LEASE_DURATION_SECS),
      options::DhcpOption::SubnetMask(SUBNET_MASK),
      options::DhcpOption::Router(vec![ROUTER_IP]),
      options::DhcpOption::DomainNameServer(DNS_IPS.to_vec()),
    ],
    *offer_ip,
    req_packet,
  );
}

fn nak(s: &server::Server, req_packet: packet::Packet, message: &str) {
  let _ = s.reply(
    options::MessageType::Nak,
    vec![options::DhcpOption::Message(message.to_string())],
    Ipv4Addr::new(0, 0, 0, 0),
    req_packet,
  );
}

fn create_dhcp_service(ip_addr: &str, broadcast_addr: &str) {
  const LEASE_DURATION_SECS: u32 = 86400;
  let leases: HashMap<Ipv4Addr, ([u8; 6], Option<Instant>)> = HashMap::new();
  let socket = UdpSocket::bind(ip_addr.to_owned() + ":67").unwrap();
  socket.set_broadcast(true).unwrap();
  let ms = MyServer {
    ip_addr: ip_addr.to_string(),
    leases,
    last_lease: 0,
    lease_duration: Duration::new(LEASE_DURATION_SECS as u64, 0),
  };
  Server::serve(
    socket,
    Ipv4Addr::from_str(ip_addr).expect("Could not parse ip_addr to Ipv4Addr"),
    Ipv4Addr::from_str(broadcast_addr).expect("Could not parse broadcast to Ipv4Addr"),
    ms,
  );
}
