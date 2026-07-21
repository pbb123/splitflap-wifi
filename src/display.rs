
use crate::module::Module;
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use heapless::Vec;
use port_expander::{PortMutex, dev::pcf8575::Pcf8575};

pub struct Display<'a,const SIZE: usize>
{
 modules: Vec<Module<'a>,SIZE>,
}

impl<const SIZE: usize> Display<'_,SIZE>
{
    pub fn new() -> Self 
    {
        let modules = Vec::new();
        for module_id in 0..SIZE
        {
            
        }
        Display 
        { 
            modules
        }
    }
}