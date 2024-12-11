#![forbid(unsafe_code)]
use std::io::Write;
use std::time::Duration;
use std::str::FromStr;

use clap::Parser;

use crate::beacon::Beacon;

mod beacon;

#[derive(Debug, Parser)]
#[command(about="Create IPNDv8 beacon and output it to stdout", long_about = None)]
struct CLIArgs {
    /// Advertized node ID
    #[arg(long)]
    node_id: Option<String>,

    /// Duration in seconds between two advertizments
    #[arg(short, long="period", value_name="DURATION")]
    period_secs: Option<u64>,
    
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

    base_beacon.period = args.period_secs.map(Duration::from_secs);

    if let Some(port) = args.tcpclv3 {
        base_beacon.services.push(beacon::Service::TCPCLv3(port));
    }

    if let Some(port) = args.tcpclv4 {
        base_beacon.services.push(beacon::Service::TCPCLv4(port));
    }

    if let Some(port) = args.mtcpcl {
        base_beacon.services.push(beacon::Service::MTCPCL(port));
    }

    if let Some(str) = args.geolocation {
        let parts:Vec<f32> = str.split(",")
                                .map(f32::from_str)
                                .map(|it| it.expect("Failed to parse location part"))
                                .collect();

        base_beacon.services.push(beacon::Service::GeoLocation(
            *parts.first().expect("Missing latitude"),
            *parts.get(1).expect("Missing longitude")
        ));
    }

    if let Some(address) = args.address {
        base_beacon.services.push(beacon::Service::Address(address));
    }

    std::io::stdout().write_all(&base_beacon.as_bytes().unwrap()).unwrap();
}