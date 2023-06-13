#![no_std]
#![feature(more_qualified_paths)]

use bin_utils::*;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// The EtherType as described in IEEE 802.3
pub enum EtherType {
    IPv4,
    ARP,
    WoLAN,
    IPv6,

    Unknown(u16),
}
enum_to_int! {
    u16,
    EtherType,

    0x0800,
    EtherType::IPv4,
    0x0806,
    EtherType::ARP,
    0x0842,
    EtherType::WoLAN,
    0x86DD,
    EtherType::IPv6
}
impl ReadFixed<2> for EtherType {
    fn from_bytes(data: &[u8; 2]) -> Result<Self, ParserError> {
        Ok(u16::from_be_bytes(*data).into())
    }
}
impl WriteFixed<2> for EtherType {
    fn to_bytes(&self) -> [u8; 2] {
        <Self as Into<u16>>::into(*self).to_be_bytes()
    }
} 