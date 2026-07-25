#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::gpio::Level;

use esp_println::{dbg};

use core::cell::RefCell;
use embedded_hal_bus::i2c::CriticalSectionDevice;

use port_expander::{dev::{pcf8575::{self}}};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    dbg!(info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]


#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o esp32c3-mini-1 -o vscode -o unstable-hal -o embassy -o alloc -o wifi
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO2
    // - GPIO8
    // - GPIO9
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO12;
    let _ = peripherals.GPIO13;
    let _ = peripherals.GPIO14;
    let _ = peripherals.GPIO15;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO17;

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);

    let timg0: TimerGroup<'_, esp_hal::peripherals::TIMG0<'_>> = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);


    let _out1 = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let _out2 = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let _out3 = Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default());
    let _out4 = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());


    let i2c = esp_hal::i2c::master::I2c::new
    (   peripherals.I2C0,
        esp_hal::i2c::master::Config::default()
    ).expect("Expected to inicialise I2C")
    .with_sda(peripherals.GPIO1).
    with_scl(peripherals.GPIO2)
    .into_async();

    use wifi_test::mk_static;
    use wifi_test::{I2cBus,I2cDev,Pcf};

    let i2c_bus: &mut I2cBus = mk_static!(I2cBus,critical_section::Mutex::new(RefCell::new(i2c))); 
    let i2c_dev: I2cDev = CriticalSectionDevice::new(i2c_bus);
    let pcf: &mut Pcf = mk_static!(Pcf,pcf8575::Pcf8575::with_mutex(i2c_dev,false,false,false));
    let pcf_pins = pcf.split();



    let motor_outs =  (pcf_pins.p00,pcf_pins.p01,pcf_pins.p02,pcf_pins.p03);

    //let hall_sensor = Input::new(peripherals.GPIO0, InputConfig::default().with_pull(Pull::Up));
    let hall_sensor = pcf_pins.p04;

    let (_ap_stack,_sta_stack) = wifi_test::net::init(peripherals.WIFI, spawner.clone(),motor_outs,hall_sensor).await;
    loop {Timer::after(Duration::from_millis(1000)).await}
}



// for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
