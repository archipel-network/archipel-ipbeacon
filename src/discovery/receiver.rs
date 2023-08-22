use std::{thread, time::Duration, sync::{atomic::AtomicBool, Arc}, net::{UdpSocket, SocketAddr}, io::ErrorKind, collections::HashMap};
use std::sync::atomic::Ordering;

use crate::beacon::Beacon;

pub fn receiver_task(
    verbose: bool,
    continue_trigger: Arc<AtomicBool>,
    socket: UdpSocket
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
                        &mut seq_nums)
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
    seq_num_index: &mut HashMap<SocketAddr, u64>
){
    match Beacon::parse(buf) {
        Ok(beacon) => {

        let is_fresh = match seq_num_index.get(&source) {
            Some(seq_num) => beacon.sequence_number > *seq_num, //bug Should tke into account sequence number overflowing (reset to 0)
            None => true,
        };

        if !is_fresh {
            return;
        }

        seq_num_index.insert(source, beacon.sequence_number);

        println!("{:?}", beacon)

        //todo configure contact un ud3tn

        },
        Err(e) => if verbose { println!("Invalid beacon received {}", e) },
    };
}