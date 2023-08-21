#![forbid(unsafe_code)]
use std::time::Duration;
use std::str::FromStr;
use std::fmt::Write;

use clap::Parser;

use crate::beacon::Beacon;

mod beacon;

#[derive(Debug, Parser)]
#[command(about="Advertize ud3tn node using IPNDv7 on the network", long_about = None)]
struct CLIArgs {
    /// Advertized node ID
    #[arg(long)]
    node_id: Option<String>,

    /// Show more debug messages
    #[arg(short, long)]
    verbose: bool,

    /// Duration in seconds between two advertizments
    #[arg(short, long="period", value_name="DURATION", default_value_t=30)]
    period_secs: u64,
    
    /// Add a TCPCLv4 convergence layer to advertizments
    #[arg(long, value_name="PORT")]
    tcpclv4: Option<u16>,
    
    /// Add a TCPCLv3 convergence layer to advertizments
    #[arg(long, value_name="PORT")]
    tcpclv3: Option<u16>,
    
    /// Add a Minimal TCP convergence layer to advertizments
    #[arg(long, value_name="PORT")]
    mtcpcl: Option<u16>,

    /// Add a geolocation service to advertizments
    #[arg(long="geo", value_name="LAT,LON")]
    geolocation: Option<String>,

    /// Add physical address service to advertizments
    #[arg(long, value_name="ADDRESS")]
    address: Option<String>
}

fn main() {
    let args = CLIArgs::parse();

    let mut base_beacon = Beacon::new();
    
    base_beacon.node_id = args.node_id;

    base_beacon.period = Some(Duration::from_secs(args.period_secs));

    if let Some(port) = args.tcpclv3 {
        base_beacon.services.push(beacon::Service::TCPCLv3Service(port));
    }

    if let Some(port) = args.tcpclv4 {
        base_beacon.services.push(beacon::Service::TCPCLv4Service(port));
    }

    if let Some(port) = args.mtcpcl {
        base_beacon.services.push(beacon::Service::MTCPCLService(port));
    }

    if let Some(str) = args.geolocation {
        let parts:Vec<f32> = str.split(",")
                                .map(f32::from_str)
                                .map(|it| it.expect("Failed to parse location part"))
                                .collect();

        base_beacon.services.push(beacon::Service::GeoLocation(
            *parts.get(0).expect("Missing latitude"),
            *parts.get(1).expect("Missing longitude")
        ));
    }

    if let Some(address) = args.address {
        base_beacon.services.push(beacon::Service::Address(address));
    }

    let mut s = String::new();
    for &byte in base_beacon.as_bytes().unwrap().iter() {
        write!(&mut s, "{:02X} ", byte).expect("Unable to write");
    }

    println!("{:?}", base_beacon);
    println!("{}", s);
    println!("{} bytes length", base_beacon.as_bytes().unwrap().len())
}