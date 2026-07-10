use embedded_stepper::{stepper,motors};
use esp_println::println;
pub struct Character<'a>
{
    _size: i32,
    pub position: i32,
    pub motor: stepper::Stepper<motors::StepperMotor4<esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>>, esp_hal::delay::Delay>,
    char_pos: [i32;37],
    pub hall_sensor: esp_hal::gpio::Input<'a>,

}

impl Character <'_>
{
    const STEP_NUMBER: i32 = 2048;
    pub fn new <'a>(size: i32, motor: stepper::Stepper<motors::StepperMotor4<esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>, esp_hal::gpio::Output<'a>>, esp_hal::delay::Delay>, hall_sensor: esp_hal::gpio::Input<'a>) -> Character<'a>
    {
        let char_pos = (0..37).map(|i| i*size/Self::STEP_NUMBER);
        return Character
        {
            _size: size,
            position: 0,
            motor: motor,
            char_pos:  [0, 56, 111, 167, 222, 277, 333, 388, 443, 499, 554, 609, 665, 720, 775, 831, 
            886, 941, 997, 1052, 1108, 1163, 1218, 1274, 1329, 1384, 1440, 1495, 1550, 1606, 1661, 1716, 1772, 1827, 1882, 1938, 1993],
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
        const ASCII_SPACE: u8 = 32;
        const ASCII_A: u8 = 65;
        const ASCII_Z: u8 = 90;
        const ASCII_a: u8 = 97;
        const ASCII_z: u8 = 122;
        const ASCII_0: u8 = 48;
        const ASCII_9: u8 = 48;

        const FLAP_BLANK: u8 = 0;
        const FLAP_A: u8 = 1;
        const FLAP_0: u8 = 28;
        let flap_number = match char
        {
            ASCII_SPACE =>       FLAP_BLANK as usize,
            ASCII_A..=ASCII_Z => (char-ASCII_A+FLAP_A) as usize,
            ASCII_a..=ASCII_z => (char-ASCII_a+FLAP_A) as usize,
            ASCII_0..=ASCII_9 => (char-ASCII_0+FLAP_0) as usize,
            _ => todo!()
        };
        self.goto(self.char_pos[flap_number]);
    }
    pub fn reset(&mut self)
    {
        self.motor.set_speed(10);
        while self.hall_sensor.is_high()
         {
             let _ = self.motor.step(1);
         }
        let _ = self.motor.step(140);
        self.position =0;
    }
}

