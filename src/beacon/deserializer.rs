use std::time::Duration;

use serde::{Deserialize, de::{Visitor, Error}};
use super::flags::{SOURCE_EID_PRESENT, SERVICE_BLOCK_PRESENT, BEACON_PERIOD_PRESENT};
use super::Service;
use super::Beacon;

impl<'de> Deserialize<'de> for super::Beacon {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D)
        -> Result<Self, D::Error> {
        deserializer.deserialize_seq(BeaconVisitor)
    }
}

struct BeaconVisitor;

impl<'de> Visitor<'de> for BeaconVisitor {
    type Value = Beacon;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a beacon definition")
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A)
        -> Result<Self::Value, A::Error> {

            let version:u8 = seq.next_element()?
                .ok_or(Error::missing_field("beacon version"))?;

            if version != 8 {
                return Err(Error::custom(format!("Unsupported beacon format {}", version)));
            }

            let flags: u8 = seq.next_element()?
                .ok_or(Error::missing_field("beacon flags"))?;

            let sequence_number: u64 = seq.next_element()?
                .ok_or(Error::missing_field("beacon sequence number"))?;

            let node_id:Option<String> = 
                if flags & SOURCE_EID_PRESENT == SOURCE_EID_PRESENT {
                    Some(seq.next_element()?
                            .ok_or(Error::missing_field("beacon source node ID"))?)
                } else {
                    None
                };
            
            let services:Vec<Service> = 
                if flags & SERVICE_BLOCK_PRESENT == SERVICE_BLOCK_PRESENT {
                    seq.next_element()?
                        .ok_or(Error::missing_field("beacon service block"))?
                } else {
                    Vec::new()
                };
            
            let period:Option<Duration> =
                if flags & BEACON_PERIOD_PRESENT == BEACON_PERIOD_PRESENT {
                    let duration = Duration::from_secs(
                        seq.next_element()?
                            .ok_or(Error::missing_field("beacon period"))?);

                    Some(duration)
                } else {
                    None
                };

            Ok(Beacon { version, node_id, sequence_number, services, period })

    }

}

impl<'de> Deserialize<'de> for super::Service {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D)
        -> Result<Self, D::Error> {
            deserializer.deserialize_tuple(2, ServiceVisitor)
    }
}

struct ServiceVisitor;

impl<'de> Visitor<'de> for ServiceVisitor {
    type Value = super::Service;

    fn expecting(&self, formatter: &mut std::fmt::Formatter)
        -> std::fmt::Result {
            write!(formatter, "a service definition tuple (tag, data)")
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A)
        -> Result<Self::Value, A::Error> {
            let tag:u8 = seq.next_element()?
                .ok_or(Error::missing_field("service tag"))?;

            match tag {
                0 => {
                    let port = seq.next_element()?
                        .ok_or(Error::missing_field("convergence layer port"))?;
                    Ok(super::Service::TCPCLv4(port))
                },

                1 => {
                    let port = seq.next_element()?
                        .ok_or(Error::missing_field("convergence layer port"))?;
                    Ok(super::Service::TCPCLv3(port))
                },

                2 => {
                    let port = seq.next_element()?
                        .ok_or(Error::missing_field("convergence layer port"))?;
                    Ok(super::Service::MTCPCL(port))
                },

                64 => {
                    let latlon: (f32, f32) = seq.next_element()?
                        .ok_or(Error::missing_field("geo location data"))?;
                    Ok(super::Service::GeoLocation(latlon.0, latlon.1))
                },

                65 => {
                    let addr: String = seq.next_element()?
                        .ok_or(Error::missing_field("address string"))?;
                    Ok(super::Service::Address(addr))
                },

                unknown_tag => {
                    let data = seq.next_element()?
                        .ok_or(Error::missing_field("service data"))?;
                    Ok(super::Service::Unknown(unknown_tag, data))
                }
            }
    }
}