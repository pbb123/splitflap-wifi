
use crate::module::Module;
use embassy_sync::{blocking_mutex::{CriticalSectionMutex, raw::{NoopRawMutex, RawMutex}}, mutex::Mutex};
use esp_hal::{Async};
use heapless::Vec;
use port_expander::{Pin, PortMutex, dev::{pcf8575::{self, Driver, Pcf8575}}, mode::QuasiBidirectional};

use embedded_hal_bus::i2c::CriticalSectionDevice;

use core::cell::RefCell;

use crate::{PcfPin,PcfParts,Pcf};

pub struct Display<'a,const SIZE: usize>
{
    //pcf: Pcf<'a>,
    //pcf_pins: PcfParts<'a>,
    modules: Vec<Module<PcfPin<'a,QuasiBidirectional>,PcfPin<'a,QuasiBidirectional>>,SIZE>,
}

impl<'a,const SIZE: usize> Display<'a,SIZE>
{
    /// Creates a new display with specified number of modules
    pub fn new() -> Self
    {
        Self {modules: Vec::new()}
    }


    fn make_module(pin1: PcfPin<'a,QuasiBidirectional>,pin2: PcfPin<'a,QuasiBidirectional>,pin3: PcfPin<'a,QuasiBidirectional>,pin4: PcfPin<'a,QuasiBidirectional>,pin5: PcfPin<'a,QuasiBidirectional>) -> Module<PcfPin<'a,QuasiBidirectional>,PcfPin<'a,QuasiBidirectional>>
    {
        let motor = uln2003::ULN2003::new
        (
            pin1,
            pin2,
            pin3,
            pin4,
            Some(embassy_time::Delay)
        );
        let hall_sensor: PcfPin<_> = pin5;
        Module::new(37,motor , hall_sensor)
    }
}