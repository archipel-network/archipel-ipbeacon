use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;
use std::str::FromStr;
use beacon::Beacon;
use clap::Parser;
use discovery::start_discovery;
use ud3tn_aap::Agent;

mod beacon;
mod discovery;

#[derive(Debug, Parser)]
#[command(about="Start ipndv8 daemon", long_about = None)]
struct CLIArgs {
    /// Log more
    #[arg(short, long)]
    verbose: bool,

    /// Socket of archipel core runtime to configure
    #[arg(short, long="socket", default_value="/run/archipel-core/archipel-core.socket")]
    socket_path: PathBuf,

    /// Duration in seconds between two advertizments
    #[arg(short, long="period", value_name="DURATION", default_value="30")]
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
    address: Option<String>,

    /// Only listen and emit on ipv4
    #[arg(short='4', long="ipv4")]
    ipv4_only: bool,

    /// Only listen and emit on ipv6
    #[arg(short='6', long="ipv6")]
    ipv6_only: bool,

    /// Broadcast beacons instead of multicast
    #[arg(short, long)]
    broadcast: bool
}

#[derive(Debug, Clone)]
pub enum IpConfig {
    Ipv4Only,
    Ipv6Only,
    Both
}

fn main() {
    println!("Archipel IPBeacon");
    println!("A neighbor discovery daemon for ud3tn");

    let args = CLIArgs::parse();
    let period = Duration::from_secs(args.period_secs);

    let ud3tn_socket = UnixStream::connect(&args.socket_path)
        .expect("Unable to connect to socket");

    let aap = Agent::connect(ud3tn_socket, "ipbeacon".into())
        .expect("Unable to connect to Archipel core");

    let node_id = aap.node_eid.clone();

    let ip_config = if args.ipv4_only {
        IpConfig::Ipv4Only
    } else if args.ipv6_only {
        IpConfig::Ipv6Only
    } else {
        IpConfig::Both
    };

    let mut base_beacon = Beacon::new();
    
    base_beacon.node_id = Some(node_id.clone());

    base_beacon.period = Some(period);

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

    if args.verbose {
        println!("Base beacon advertizment : {:#?}", base_beacon);
    }
    
    start_discovery(
        args.verbose, 
        ip_config,
        args.broadcast,
        base_beacon,
        period,
        node_id,
        aap);

}