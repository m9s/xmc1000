#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 1 Output Modification Register"]
    pub omr: OMR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Port 1 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    #[doc = "0x14 - Port 1 Input/Output Control Register 4"]
    pub iocr4: IOCR4,
    _reserved1: [u8; 12usize],
    #[doc = "0x24 - Port 1 Input Register"]
    pub in_: IN,
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - Port 1 Pad Hysteresis Control Register 0"]
    pub phcr0: PHCR0,
    _reserved3: [u8; 28usize],
    #[doc = "0x60 - Port 1 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved4: [u8; 12usize],
    #[doc = "0x70 - Port 1 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 1 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "Port 1 Input/Output Control Register 0"]
pub struct IOCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "Port 1 Input/Output Control Register 4"]
pub struct IOCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "Port 1 Pad Hysteresis Control Register 0"]
pub struct PHCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Pad Hysteresis Control Register 0"]
pub mod phcr0;
#[doc = "Port 1 Pin Function Decision Control Register"]
pub struct PDISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "Port 1 Output Register"]
pub struct OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Output Register"]
pub mod out;
#[doc = "Port 1 Output Modification Register"]
pub struct OMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Output Modification Register"]
pub mod omr;
#[doc = "Port 1 Input Register"]
pub struct IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Input Register"]
pub mod in_;
#[doc = "Port 1 Pin Power Save Register"]
pub struct PPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Pin Power Save Register"]
pub mod pps;
#[doc = "Port 1 Pin Hardware Select Register"]
pub struct HWSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 Pin Hardware Select Register"]
pub mod hwsel;
