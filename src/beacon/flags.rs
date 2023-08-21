/// Source EID of sender node is present (should always be set)
pub const SOURCE_EID_PRESENT: u8 = 0b0000_0001;

/// Service block is present
pub const SERVICE_BLOCK_PRESENT: u8 = 0b0000_0010;

/// Beacon Period field is present
pub const BEACON_PERIOD_PRESENT: u8 = 0b0000_0100;

/// Bits 4 - 7 are reserved for future specifications
pub const RESERVED_BITS: u8 = 0b1111_1000;