#![feature(impl_trait_in_assoc_type)]
#![feature(trivial_bounds)]
#![no_std]

pub mod net;
pub mod web;
pub mod module;
pub mod display;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub struct SharedPeripherals {
    pub out1: esp_hal::gpio::Output<'static>,
    pub out2: esp_hal::gpio::Output<'static>,
    pub out3: esp_hal::gpio::Output<'static>,
    pub out4: esp_hal::gpio::Output<'static>,
}
use port_expander::Pcf8575;
use esp_hal::Async;
use embedded_hal_bus::i2c::CriticalSectionDevice;
use core::cell::RefCell;

pub type PeriphMutex = embassy_sync::mutex::Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, SharedPeripherals>;

pub type I2cBus = critical_section::Mutex<RefCell<esp_hal::i2c::master::I2c<'static, esp_hal::Async>>>;

pub type I2cDev = CriticalSectionDevice<'static, esp_hal::i2c::master::I2c<'static, esp_hal::Async>>;

pub type PcfPin<'a,IO> = port_expander::Pin<'a, IO, critical_section::Mutex<core::cell::RefCell<port_expander::dev::pcf8575::Driver<embedded_hal_bus::i2c::CriticalSectionDevice<'a, esp_hal::i2c::master::I2c<'a, esp_hal::Async>>>>>>;

pub type Pcf<'a> = Pcf8575<critical_section::Mutex<RefCell<port_expander::dev::pcf8575::Driver<embedded_hal_bus::i2c::CriticalSectionDevice<'a, esp_hal::i2c::master::I2c<'a, Async>>>>>>;

pub type PcfParts<'a> =  port_expander::dev::pcf8575::Parts<'a, embedded_hal_bus::i2c::CriticalSectionDevice<'a, esp_hal::i2c::master::I2c<'a, Async>>, critical_section::Mutex<RefCell<port_expander::dev::pcf8575::Driver<embedded_hal_bus::i2c::CriticalSectionDevice<'a, esp_hal::i2c::master::I2c<'a, Async>>>>>>;

pub type QBiPcfPin<'a> = PcfPin<'a,port_expander::mode::QuasiBidirectional>;