use serde::{Serialize, ser::SerializeSeq, ser::SerializeTuple};
use super::flags::{SOURCE_EID_PRESENT, SERVICE_BLOCK_PRESENT, BEACON_PERIOD_PRESENT};

impl Serialize for super::Beacon {
    fn serialize<S: serde::Serializer>(&self, serializer: S)
        -> Result<S::Ok, S::Error> {

        let (flags, length) = {
            let mut f = 0_u8;
            let mut l = 3;

            if self.node_id.is_some() {
                f |= SOURCE_EID_PRESENT;
                l += 1;
            }

            if !self.services.is_empty() {
                f |= SERVICE_BLOCK_PRESENT;
                l += 1;
            }

            if self.period.is_some() {
                f |= BEACON_PERIOD_PRESENT;
                l += 1;
            }

            (f, l)
        };

        let mut beacon = serializer.serialize_seq(Some(length))?;

        beacon.serialize_element(&self.version)?;

        beacon.serialize_element(&flags)?;

        beacon.serialize_element(&self.sequence_number)?;

        if let Some(node_id) = &self.node_id {
            beacon.serialize_element(node_id)?;
        }

        if !self.services.is_empty() {
            beacon.serialize_element(&self.services)?;
        }

        if let Some(period) = &self.period {
            beacon.serialize_element(&period.as_secs())?;
        }

        beacon.end()
    }
}

impl Serialize for super::Service {
    fn serialize<S: serde::Serializer>(&self, serializer: S)
        -> Result<S::Ok, S::Error> {

        let mut base = serializer.serialize_tuple(2)?;

        let flag = match self {
            super::Service::TCPCLv4Service(_) => 0_u8,
            super::Service::TCPCLv3Service(_) => 1_u8,
            super::Service::MTCPCLService(_) => 2_u8,
            super::Service::GeoLocation(_, _) => 64_u8,
            super::Service::Address(_) => 65_u8,
            super::Service::Unknown(tag, _) => *tag,
        };

        base.serialize_element(&flag)?;

        match self {
            super::Service::TCPCLv4Service(port)
                => base.serialize_element(port)?,

            super::Service::TCPCLv3Service(port)
                => base.serialize_element(port)?,

            super::Service::MTCPCLService(port)
                => base.serialize_element(port)?,

            super::Service::GeoLocation(lat, lon)
                => base.serialize_element(&(lat, lon))?,

            super::Service::Address(addr)
                => base.serialize_element(addr)?,

            super::Service::Unknown(_, val)
                => base.serialize_element(val)?,
        };

        base.end()
    }
}