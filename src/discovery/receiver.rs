use std::{thread, time::Duration, sync::{atomic::AtomicBool, Arc}, net::{UdpSocket, SocketAddr}, io::ErrorKind, collections::HashMap};
use std::sync::atomic::Ordering;

use crate::beacon::{Beacon, NodeIdentifier};

pub fn receiver_task(
    verbose: bool,
    continue_trigger: Arc<AtomicBool>,
    socket: UdpSocket,
    self_node_id: NodeIdentifier
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
                        &buf[0..bytes_red], 
                        source,
                        &mut seq_nums,
                        &self_node_id)
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
    buf: &[u8],
    source: SocketAddr,
    seq_num_index: &mut HashMap<SocketAddr, u64>,
    self_node_id: &NodeIdentifier
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

            seq_num_index.insert(source, beacon.sequence_number);

            if verbose {
                println!("Received beacon #{} from {}", beacon.sequence_number, source);
                println!("{:?}", beacon)
            }

            add_contact(beacon);

        },
        Err(e) => if verbose { println!("Invalid beacon received {}", e) },
    };
}

fn add_contact(beacon: Beacon){
    //todo configure contact un ud3tn
}