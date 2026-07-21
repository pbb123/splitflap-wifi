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

pub type PeriphMutex = embassy_sync::mutex::Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, SharedPeripherals>;