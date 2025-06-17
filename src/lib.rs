#![no_std]
#![feature(more_qualified_paths)]

use macro_bits::serializable_enum;

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    /// The EtherType as described in IEEE 802.3
    pub enum EtherType: u16 {
        #[default]
        IPv4 => 0x0800,
        ARP => 0x0806,
        WoLAN => 0x0842,
        IPv6 => 0x86DD,
        Eapol => 0x888e
    }
}
