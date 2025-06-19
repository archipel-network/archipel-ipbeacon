use std::{net::UdpSocket, net::SocketAddr, sync::{atomic::AtomicBool, Arc}, thread, time::Duration};

use crate::{beacon::Beacon, IpConfig};
use std::sync::atomic::Ordering;

pub fn announcer_task(
    verbose: bool,
    ip_config:IpConfig,
    broadcast: bool,
    continue_trigger: Arc<AtomicBool>,
    base_beacon: Beacon, 
    period: Duration,
    socket: UdpSocket,
    extra_unicast: Vec<SocketAddr>
) {
    let mut beacon = base_beacon;

    while continue_trigger.load(Ordering::SeqCst) {

        let buf = beacon.as_bytes().unwrap();

        if matches!(ip_config, IpConfig::Both) || matches!(ip_config, IpConfig::Ipv6Only) {

            let addr = match broadcast {
                true => "[ff02::1]:3005",
                false => "[ff02::d4cd:0305:3af1:aeef:75de]:3005",
            };

            match socket.send_to(&buf, addr) {
                Ok(_) => if verbose { println!("Emitted v6 beacon #{}", beacon.sequence_number) },
                Err(e) => println!("Error sending v6 beacon : {}", e),
            }
        }

        if matches!(ip_config, IpConfig::Both) || matches!(ip_config, IpConfig::Ipv4Only) {

            let addr = match broadcast {
                true => "255.255.255.255:3005",
                false => "224.0.0.108:3005",
            };

            match socket.send_to(&buf, addr) {
                Ok(_) => if verbose { println!("Emitted v4 beacon #{}", beacon.sequence_number) },
                Err(e) => println!("Error sending v4 beacon : {}", e),
            }
        }

        for direct in &extra_unicast {
            let addr = [direct.to_owned()];
            if let Err(e) = socket.send_to(&buf, &addr.as_slice()) {
                eprintln!("Failed to send direct beacon to {direct}: {e}");
            } else if verbose {
                println!("Direct beacon #{} emitted for {}", beacon.sequence_number, direct)
            }
        }
        
        beacon = beacon.next();
        thread::sleep(period);

    }
}