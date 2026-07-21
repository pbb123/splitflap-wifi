
use crate::module::Module;
use embassy_sync::{blocking_mutex::{CriticalSectionMutex, raw::{NoopRawMutex, RawMutex}}, mutex::Mutex};
use esp_hal::{};
use heapless::Vec;
use port_expander::{PortMutex, dev::{pcf8575::{self, Pcf8575}}};

use embedded_hal_bus::i2c::CriticalSectionDevice;

use core::cell::RefCell;

pub struct Display<'a,const SIZE: usize>
{
 modules: Vec<Module<'a>,SIZE>,
}

impl<const SIZE: usize> Display<'_,SIZE>
{
    pub fn new(i2c: esp_hal::i2c::master::I2c<'_,esp_hal::Async>) -> Self 
    {
        let modules = Vec::new();
        let i2c_bus:critical_section::Mutex<RefCell<esp_hal::i2c::master::I2c<'_, esp_hal::Async>>> = critical_section::Mutex::new(RefCell::new(i2c));
        let ic2_device1 = CriticalSectionDevice::new(&i2c_bus);
        let ic2_device2 = CriticalSectionDevice::new(&i2c_bus);

        let mut pcf85751: Pcf8575<critical_section::Mutex<_>> = pcf8575::Pcf8575::with_mutex(ic2_device1,false,false,false);
        let mut pcf85752: Pcf8575<critical_section::Mutex<_>> = pcf8575::Pcf8575::with_mutex(ic2_device2,false,false,false);

        Display 
        { 
            modules
        }
    }
}