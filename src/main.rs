use esp_idf_svc::hal::{delay::FreeRtos, prelude::Peripherals, gpio::{PinDriver, IOPin, Pull}};

static FN_THREAD_STACK_SIZE: usize = 2000;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();

    // Initialize Pin 2 as an output to drive the LED
    let mut led_pin = PinDriver::output(peripherals.pins.gpio2).unwrap();

    // Initialize Pin 0 as an input to drive the BUTTON
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio0.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(FN_THREAD_STACK_SIZE)
        .spawn(move || fn_thread())
        .unwrap();

    loop {
        if btn_pin.is_high() {
            led_pin.set_low().unwrap();
            log::warn!("LED OFF");
        } else {
            led_pin.set_high().unwrap();
            log::warn!("LED ON");
        }

        FreeRtos::delay_ms(100);
    }
}

fn fn_thread() {
    loop {
        log::info!("fn_thread test...");
        FreeRtos::delay_ms(100);
    }
}