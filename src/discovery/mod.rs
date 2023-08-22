use std::{time::Duration, thread, sync::{Arc, atomic::AtomicBool}, net::{UdpSocket, IpAddr, Ipv4Addr}};

use crate::beacon::Beacon;
use std::sync::atomic::Ordering;

mod announcer;
mod receiver;

pub fn start_discovery(
    verbose: bool,
    base_beacon: Beacon,
    period: Duration
){
    let continue_trigger = Arc::new(AtomicBool::new(true));

    let ctrigger_int = continue_trigger.clone();
    ctrlc::set_handler(move || {
        println!("Shuttng down");
        ctrigger_int.store(false, Ordering::SeqCst)
    }).unwrap();

    let socket = UdpSocket::bind("[::]:3005")
        .expect("Unable to bind v6 socket to [::]:3005");

    socket.set_broadcast(true)
        .expect("Unable to allow socket to broadcast");

    socket.set_multicast_loop_v4(false)
        .expect("Unable to disable multicast loop v4");

    socket.set_multicast_loop_v6(false)
        .expect("Unable to disable multicast loop v6");

    println!("Starting discovery");

    let verbose_rec = verbose;
    let ctrigger_rec = continue_trigger.clone();
    let socket_rec = socket.try_clone().unwrap();
    thread::spawn(move || receiver::receiver_task(verbose_rec, ctrigger_rec, socket_rec));

    announcer::announcer_task(verbose, continue_trigger, base_beacon, period, socket)
}