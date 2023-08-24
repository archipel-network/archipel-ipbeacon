use std::{time::Duration, thread, sync::{Arc, atomic::AtomicBool}, net::{UdpSocket, Ipv4Addr, Ipv6Addr}, io::{Read, Write}, str::FromStr};

use ud3tn_aap::Agent;

use crate::{beacon::{Beacon, NodeIdentifier}, IpConfig};
use std::sync::atomic::Ordering;

mod announcer;
mod receiver;

pub fn start_discovery<T: Read + Write>(
    verbose: bool,
    ip_config: IpConfig,
    broadcast: bool,
    base_beacon: Beacon,
    period: Duration,
    node_id: NodeIdentifier,
    aap: Agent<T>
){
    let continue_trigger = Arc::new(AtomicBool::new(true));

    let ctrigger_int = continue_trigger.clone();
    ctrlc::set_handler(move || {
        println!("Shuttng down");
        ctrigger_int.store(false, Ordering::SeqCst)
    }).unwrap();

    let bind_addr = match ip_config {
        IpConfig::Ipv4Only => "0.0.0.0:3005",
        IpConfig::Both => "[::]:3005",
        IpConfig::Ipv6Only => "[::]:3005",
    };

    let socket = UdpSocket::bind(bind_addr)
        .expect(&format!("Unable to bind v6 socket to {}", bind_addr));

    socket.set_broadcast(true)
        .expect("Unable to allow socket to broadcast");

    match ip_config {
        IpConfig::Ipv4Only => {
            socket.set_multicast_loop_v4(false)
                .expect("Unable to disable multicast loop v4");
        },
        IpConfig::Ipv6Only => {
            socket.set_multicast_loop_v6(false)
            .expect("Unable to disable multicast loop v6");
        },
        IpConfig::Both => {
            socket.set_multicast_loop_v4(false)
                .expect("Unable to disable multicast loop v4");
            socket.set_multicast_loop_v6(false)
            .expect("Unable to disable multicast loop v6");
        },
    }

    socket.join_multicast_v4(
        &Ipv4Addr::from_str("224.0.0.108").unwrap(), 
        &Ipv4Addr::UNSPECIFIED)
        .expect("Unable to join ipv4 multicast group");

    socket.join_multicast_v6(
        &Ipv6Addr::from_str("ff02::d4cd:0305:3af1:aeef:75de").unwrap(), 
        0)
        .expect("Unable to join ipv6 multicast group");

    println!("Starting discovery");

    let verbose_emit = verbose;
    let ip_config_emit = ip_config.clone();
    let ctrigger_emit = continue_trigger.clone();
    let socket_emit = socket.try_clone().unwrap();
    thread::spawn(move || announcer::announcer_task(
        verbose_emit,
        ip_config_emit,
        broadcast,
        ctrigger_emit,
        base_beacon,
        period,
        socket_emit
    ));

    receiver::receiver_task(
        verbose, 
        ip_config,
        continue_trigger, 
        socket,
        node_id,
        aap)

}