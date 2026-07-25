use embedded_hal::digital::{InputPin, OutputPin};
use uln2003::StepperMotor;
use esp_hal::gpio::Output;
use esp_println::println;


pub struct Module<O: embedded_hal::digital::OutputPin, I: embedded_hal::digital::InputPin>
{
    _size: i32,
    pub position: i32,
    pub motor: uln2003::ULN2003<O,O,O,O,embassy_time::Delay>,
    char_pos: [i32;37],
    pub hall_sensor: I,

}

impl<O: OutputPin,I : InputPin> Module <O,I>
{
    const STEP_NUMBER: i32 = 4096;
    const MOTOR_DELAY_MS: u32 = 1;

    pub fn new <'a>(size: i32, motor: uln2003::ULN2003<O,O,O,O,embassy_time::Delay>, hall_sensor: I) -> Module<O,I>
    {
        return Module
        {
            _size: size,
            position: 0,
            motor: motor,
            char_pos:  [0, 111, 221, 332, 443, 554, 664, 775, 886, 996, 1107, 1218, 1328, 1439, 1550, 1661, 1771, 1882, 1993, 
            2103, 2214, 2325, 2435, 2546, 2657, 2768, 2878, 2989, 3100, 3210, 3321, 3432, 3542, 3653, 3764, 3875, 3985],
            hall_sensor
        }
    }
    fn goto(&mut self,pos: i32)
    {
        let mut distance = pos - self.position;
        if distance<0
        {
            distance+=Self::STEP_NUMBER;
        }
        println!("pos: {pos}");
        println!("distance: {distance}");
        let _ = self.motor.step_for(distance,Self::MOTOR_DELAY_MS);
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
        const ASCII_9: u8 = 57;

        const FLAP_BLANK: u8 = 0;
        const FLAP_A: u8 = 1;
        const FLAP_0: u8 = 27;
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
        while self.hall_sensor.is_high().expect("We should be able to read the state of hall sensor")
        {
            let _ = self.motor.step_for(1,Self::MOTOR_DELAY_MS);
        }
        let _ = self.motor.step_for(280,Self::MOTOR_DELAY_MS);
        self.position = 0;
    }
}

