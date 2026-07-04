use embedded_stepper::{stepper,motors};
use esp_println::println;


pub struct Character<'a>
{
    _size: i32,
    pub position: i32,
    pub motor: stepper::Stepper<motors::StepperMotor4<esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>>, esp_hal::delay::Delay>,
    char_pos: [i32;37],
    pub hall_sensor: esp_hal::gpio::Output<'a>,

}

impl Character <'_>
{
    pub fn new <'a>(size: i32, motor: stepper::Stepper<motors::StepperMotor4<esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>>, esp_hal::delay::Delay>, hall_sensor: esp_hal::gpio::Output<'a>) -> Character<'a>
    {
        return Character
        {
            _size: size,
            position: 0,
            motor: motor,
            char_pos:  [0, 56, 111, 167, 222, 277, 333, 388, 443, 499, 554, 609, 665, 720, 775, 831, 886, 941, 997, 1052, 1108, 1163, 1218, 1274, 1329, 1384, 1440, 1495, 1550, 1606, 1661, 1716, 1772, 1827, 1882, 1938, 1993],
            hall_sensor
        }
    }
    fn goto(&mut self,pos: i32)
    {
        let mut distance = pos - self.position;
        if distance<0
        {
            distance+=2048;
        }
        println!("pos: {pos}");
        println!("distance: {distance}");
        let _ = self.motor.step(distance);
        self.position = pos;
        //let _ = self.motor.deenergise();
    }
    pub fn print_char(&mut self,char: u8)
    {
        self.goto(self.char_pos[(char as i32-65+1) as usize]+25);
    }
}