use embedded_hal::digital::{InputPin, OutputPin};
use uln2003::StepperMotor;
use esp_println::println;

/// This is a single module of a display.
/// 
/// It has two main functionalities:
/// 1. Receive a character and rotate it's stepper motor to print it;
/// 2. Do a reset sequence by rotating the motor until signal from hall sensor is detected;
/// 
/// To create a module we need to provide it's size, 4 output and 1 input pin for stepper motor and hall sensor accordingly. These cone from the i2c data bus module. 
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
    /// For 28byj-48 stepper motor in half stepping mode.
    const STEP_NUMBER: i32 = 4096;
    /// As fast as we can go.
    const MOTOR_DELAY_MS: u32 = 1;

    /// Creates a new module using provided size, motor output and hall sensor input pins.
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
    /// Goes to a specific motor position.
    async fn goto(&mut self,pos: i32)
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
    }
    /// Prints a character. Available characters are:
    /// * Upper or lowercase latin letters;
    /// * Digits;
    /// * Blank character represented with a space.
    /// 
    pub async fn print_char(&mut self,char: u8)
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
        self.goto(self.char_pos[flap_number]).await;
    }
    /// Does a reset sequence -- it rotates the motor until hall sensor signal is detected.
    pub async fn reset(&mut self)
    {
        while self.hall_sensor.is_high().expect("We should be able to read the state of hall sensor")
        {
            let _ = self.motor.step_for(1,Self::MOTOR_DELAY_MS);
        }
        let _ = self.motor.step_for(280,Self::MOTOR_DELAY_MS);
        self.position = 0;
    }
}

