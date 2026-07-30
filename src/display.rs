
use crate::{QBiPcfPin, module::Module};
use heapless::Vec;
use core::iter::zip;

use crate::{PcfPin};

pub struct Display<'a,const SIZE: usize>
{
    modules: Vec<Module<QBiPcfPin<'a>,QBiPcfPin<'a>>,SIZE>,
}

impl<'a,const SIZE: usize> Display<'a,SIZE>
{
    /// Creates a new empty display with specified number of modules
    pub fn new(modules:Vec<Module<QBiPcfPin<'a>,QBiPcfPin<'a>>,SIZE>) -> Self
    {
        Self {modules}
    }

    pub fn add_module(&mut self,pin1: QBiPcfPin<'a>,pin2: QBiPcfPin<'a>,pin3: QBiPcfPin<'a>,pin4: QBiPcfPin<'a>,pin5: QBiPcfPin<'a>)

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
        let module = Module::new(37,motor , hall_sensor);
        let _ = self.modules.push(module);
    }

    /// Prints provided word. If len(word)>SIZE then only first SIZE letters are displayed.
    pub async fn print_word(&mut self, word: &str)
    {
        let chars = word.bytes();
        let futures: Vec<_, SIZE> = Vec::from_iter(
        zip(self.modules.iter_mut(),chars)
        .map(|(module,c)| module.print_char(c)));
        
        embassy_futures::join::join_array(futures.into_array().unwrap_or_default()).await;
    }
    /// Resets all the modules
    pub async fn reset(&mut self)
    {
        embassy_futures::join::join_array(
            Vec::<_,SIZE>::from_iter(
                self.modules.iter_mut()
                .map(|module | module.reset()))
                .into_array().unwrap_or_default()
            )
            .await;   
    }
}