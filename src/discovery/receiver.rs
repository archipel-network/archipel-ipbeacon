use std::{thread, time::Duration, sync::{atomic::AtomicBool, Arc}, net::{UdpSocket, SocketAddr, IpAddr}, io::ErrorKind, collections::HashMap};
use std::sync::atomic::Ordering;

use ud3tn_aap::{Agent, config::{ConfigBundle, Contact, ContactDataRate}};

use crate::{beacon::{Beacon, NodeIdentifier}, IpConfig};

pub fn receiver_task(
    verbose: bool,
    ip_config: IpConfig,
    continue_trigger: Arc<AtomicBool>,
    socket: UdpSocket,
    self_node_id: NodeIdentifier,
    mut aap: Agent
) {
    socket.set_nonblocking(true)
        .expect("Receiver socket can't be set non-blocking");

    let mut buf = [0_u8; 100_000];
    let mut seq_nums:HashMap<SocketAddr, u64> = HashMap::new();

    while continue_trigger.load(Ordering::SeqCst) {

        match socket.recv_from(&mut buf) {
            Ok((bytes_red, source)) => {
                if bytes_red > 0 {
                    try_beacon(
                        verbose, 
                        &ip_config,
                        &buf[0..bytes_red], 
                        source,
                        &mut seq_nums,
                        &self_node_id,
                        &mut aap
                    )
                }
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::WouldBlock => {}
                    _ => println!("Errer receiving packet from socket {}", e)
                }
            },
        }

        thread::sleep(Duration::from_millis(300));
    }
}

fn try_beacon(
    verbose: bool,
    ip_config: &IpConfig,
    buf: &[u8],
    source: SocketAddr,
    seq_num_index: &mut HashMap<SocketAddr, u64>,
    self_node_id: &NodeIdentifier,
    aap: &mut Agent
){
    match Beacon::parse(buf) {
        Ok(beacon) => {

            if let Some(node_id) = &beacon.node_id {
                if *node_id == *self_node_id {
                    if verbose {
                        println!("Received beacon from current node id, ignoring");
                    }
                    return;
                }
            }

            let is_fresh = match seq_num_index.get(&source) {
                Some(seq_num) => beacon.sequence_number > *seq_num, //bug Should tke into account sequence number overflowing (reset to 0)
                None => {
                    println!("New neighbour discovered at {}", source);
                    true
                },
            };

            if !is_fresh {
                return;
            }

            if matches!(ip_config, IpConfig::Ipv6Only) {
                if let IpAddr::V6(ipv6) = source.ip() {
                    if ipv6.to_ipv4().is_some() {
                        if verbose {
                            println!("Received beacon from ipv4, ignoring");
                        }
                        return;
                    }
                }
            }

            seq_num_index.insert(source, beacon.sequence_number);

            if verbose {
                println!("Received beacon #{} from {}", beacon.sequence_number, source);
                println!("{:?}", beacon)
            }

            add_contact(verbose, beacon, source, aap);

        },
        Err(e) => if verbose { println!("Invalid beacon received {}", e) },
    };
}

fn add_contact(verbose: bool, beacon: Beacon, source: SocketAddr, aap:&mut Agent){

    let node_id = match beacon.node_id {
        Some(it) => it,
        None => {
            if verbose {  println!("Missing node id in beacon") }
            return;
        },
    };

    let tcpclv3port = beacon.services.iter().filter_map(|it| match it {
        crate::beacon::Service::TCPCLv3(port) => Some(port),
        _ => None
    }).next();

    let tcpclv3port = match tcpclv3port {
        Some(it) => it,
        None => {
            if verbose {  println!("No TCPCLv3 service available at {}", node_id) }
            return;
        },
    };

    let cla = match source.ip() {
        std::net::IpAddr::V4(ipv4) => format!("tcpclv3:{}:{}",ipv4,tcpclv3port),
        std::net::IpAddr::V6(ipv6) => {
            match ipv6.to_ipv4(){
                Some(ipv4) => format!("tcpclv3:{}:{}", ipv4, tcpclv3port),
                None => format!("tcpclv3:[{}]:{}", ipv6, tcpclv3port),
            }
        },
    };

    let duration = beacon.period.map(|it| it*2).unwrap_or(Duration::from_secs(30));

    if verbose {
        println!("Adding contact to {} with cla {} during {}s", &node_id, &cla, duration.as_secs());
    }

    let config_bundle = ConfigBundle::AddContact {
        eid: node_id,
        reliability: None,
        cla_address: cla,
        reaches_eid: Vec::new(),
        contacts: vec![
            Contact::from_now_during(
                duration,
                ContactDataRate::Unlimited)
        ],
    };

    let result = aap.send_config(config_bundle);

    if let Err(e) = result {
        println!("Error adding comtact ton ud3tn config {}", e);
    }
}