
use crate::{QBiPcfPin, module::Module};
use heapless::Vec;

use crate::{PcfPin};

pub struct Display<'a,const SIZE: usize>
{
    //pcf: Pcf<'a>,
    //pcf_pins: PcfParts<'a>,
    modules: Vec<Module<QBiPcfPin<'a>,QBiPcfPin<'a>>,SIZE>,
}

impl<'a,const SIZE: usize> Display<'a,SIZE>
{
    /// Creates a new display with specified number of modules
    pub fn new() -> Self
    {
        Self {modules: Vec::new()}
    }


    fn make_module(pin1: QBiPcfPin<'a>,pin2: QBiPcfPin<'a>,pin3: QBiPcfPin<'a>,pin4: QBiPcfPin<'a>,pin5: QBiPcfPin<'a>) -> Module<QBiPcfPin<'a>,QBiPcfPin<'a>>
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