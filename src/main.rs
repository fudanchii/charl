use gpiod::{Chip, Lines, Options, Output};
use std::path::Path;
use std::{fmt::Write, io, thread, time::Duration};

const RS_PIN: u32 = 10; //15;
const RW_PIN: u32 = 9; //14;
const EN_PIN: u32 = 11; //13;
                        // const OE_PIN: u32 = 12;
const D4_PIN: u32 = 5; //11;
const D5_PIN: u32 = 6; //10;
const D6_PIN: u32 = 13; // 9;
const D7_PIN: u32 = 19; // 8;

struct GpioCharDisplayDriver {
    rs: Lines<Output>,
    en: Lines<Output>,
    data: Lines<Output>,
}

impl GpioCharDisplayDriver {
    pub fn init(dpath: impl AsRef<Path>) -> io::Result<Self> {
        let mut chip = Chip::new(dpath)?;
        let rs = Self::init_output_ctrl(&mut chip, RS_PIN)?;
        let _ = Self::init_output_ctrl(&mut chip, RW_PIN)?;
        let en = Self::init_output_ctrl(&mut chip, EN_PIN)?;
        let data = Self::init_data(&mut chip)?;

        Ok(Self { rs, en, data })
    }

    fn init_output_ctrl(chip: &mut Chip, pin: u32) -> io::Result<Lines<Output>> {
        let opts = Options::output([pin]).values([false]);
        chip.request_lines(opts)
    }

    fn init_data(chip: &mut Chip) -> io::Result<Lines<Output>> {
        let opts = Options::output([D7_PIN, D6_PIN, D5_PIN, D4_PIN]).values(0u8);
        chip.request_lines(opts)
    }

    fn write_4bits(&mut self, data: u8) -> io::Result<()> {
        self.data.set_values(data)
    }
}

impl lcd::Hardware for GpioCharDisplayDriver {
    fn rs(&mut self, hi: bool) {
        let result = self.rs.set_values([hi]);

        if result.is_err() {
            println!("cannot set rs to {:?}: {:?}", hi, result.unwrap_err());
        }
    }

    fn enable(&mut self, hi: bool) {
        let result = self.en.set_values([hi]);
        if result.is_err() {
            println!("cannot set en to {:?}: {:?}", hi, result.unwrap_err());
        }
    }

    fn data(&mut self, data: u8) {
        if let Err(err) = self.write_4bits(data) {
            println!("cannot write data: {:?}", err);
        }
    }
}

impl lcd::Delay for GpioCharDisplayDriver {
    fn delay_us(&mut self, delay_usec: u32) {
        thread::sleep(Duration::from_micros(delay_usec.into()));
    }
}

fn main() -> io::Result<()> {
    let device = GpioCharDisplayDriver::init("gpiochip4")?;
    let mut display = lcd::Display::new(device);

    println!("Initializing LCD...");
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    println!("LCD ready.");

    display.position(0, 0);
    write!(&mut display, "hello, world!").unwrap();

    Ok(())
}
