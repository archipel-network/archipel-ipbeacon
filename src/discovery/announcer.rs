use std::{thread, time::Duration, sync::{Arc, atomic::AtomicBool}, net::UdpSocket};

use crate::{beacon::Beacon, IpConfig};
use std::sync::atomic::Ordering;

pub fn announcer_task(
    verbose: bool,
    ip_config:IpConfig,
    continue_trigger: Arc<AtomicBool>,
    base_beacon: Beacon, 
    period: Duration,
    socket: UdpSocket
) {
    let mut beacon = base_beacon;

    while continue_trigger.load(Ordering::SeqCst) {

        let buf = beacon.as_bytes().unwrap();

        if matches!(ip_config, IpConfig::Both) || matches!(ip_config, IpConfig::Ipv6Only) {
            match socket.send_to(&buf,"[ff02::1]:3005") {
                Ok(_) => if verbose { println!("Emitted v6 beacon #{}", beacon.sequence_number) },
                Err(e) => println!("Error sending v6 beacon : {}", e),
            }
        }

        if matches!(ip_config, IpConfig::Both) || matches!(ip_config, IpConfig::Ipv4Only) {
            match socket.send_to(&buf,"255.255.255.255:3005") {
                Ok(_) => if verbose { println!("Emitted v4 beacon #{}", beacon.sequence_number) },
                Err(e) => println!("Error sending v4 beacon : {}", e),
            }
        }
        
        beacon = beacon.next();
        thread::sleep(period);

    }
}