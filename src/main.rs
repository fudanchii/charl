use std::path::Path;
use std::{fmt::Write, thread, time::Duration};
use libgpiod::{line, request::Request, chip::Chip};

const RS_PIN: u32 = 15;
const RW_PIN: u32 = 14;
const EN_PIN: u32 = 13;
const OE_PIN: u32 = 12;
const D4_PIN: u32 = 11;
const D5_PIN: u32 = 10;
const D6_PIN: u32 =  9;
const D7_PIN: u32 =  8;

static VALUES: &'static [line::Value] = &[line::Value::InActive, line::Value::Active];

struct GpioCharDisplayDriver {
    chip: Chip,
    rs: Request,
    rw: Request,
    en: Request,
    data: Request,
}

impl GpioCharDisplayDriver {
    pub fn init<P: AsRef<Path>>(dpath: &P) -> libgpiod::Result<Self> {
        let mut chip = Chip::open(dpath)?;
        let rs = Self::init_output_ctrl(&mut chip, RS_PIN)?;
        let rw = Self::init_output_ctrl(&mut chip, RW_PIN)?;
        let en = Self::init_output_ctrl(&mut chip, EN_PIN)?;
        let data = Self::init_data(&mut chip)?;

        Self::init_output_set_active(&mut chip, OE_PIN)?;

        Ok(Self {
            chip,
            rs,
            rw,
            en,
            data,
        })
    }

    fn init_output_set_active(chip: &mut Chip, pin: line::Offset) -> libgpiod::Result<()> {
        let mut line_params = line::Settings::new()?;
        line_params
            .set_direction(line::Direction::Output)?
            .set_bias(Some(line::Bias::Disabled))?
            .set_drive(line::Drive::PushPull)?;

        let mut line_conf = line::Config::new()?;
        line_conf.add_line_settings(&[pin], line_params)?;

        let mut request = chip.request_lines(None, &line_conf)?;
        request.set_value(pin, line::Value::Active)?;

        Ok(())
    }

    fn init_output_ctrl(chip: &mut Chip, pin: line::Offset) -> libgpiod::Result<Request> {
        let mut line_params = line::Settings::new()?;
        line_params
            .set_direction(line::Direction::Output)?
            .set_bias(Some(line::Bias::Disabled))?
            .set_drive(line::Drive::PushPull)?
            .set_output_value(line::Value::InActive)?;

        let mut line_conf = line::Config::new()?;
        line_conf.add_line_settings(&[pin], line_params)?;

        chip.request_lines(None, &line_conf)
    }

    fn init_data(chip: &mut Chip) -> libgpiod::Result<Request> {
        let mut line_params = line::Settings::new()?;
        line_params
            .set_direction(line::Direction::Output)?
            .set_bias(Some(line::Bias::Disabled))?
            .set_drive(line::Drive::PushPull)?
            .set_output_value(line::Value::InActive)?;

        let mut line_conf = line::Config::new()?;
        line_conf.add_line_settings(&[D4_PIN, D5_PIN, D6_PIN, D7_PIN], line_params)?;

        chip.request_lines(None, &line_conf)
    }

    fn write_4bits(&mut self, data: u8) -> libgpiod::Result<()> {
        let data: usize = data.into();
        self.data.set_value(D4_PIN, VALUES[data & 1])?;
        self.data.set_value(D5_PIN, VALUES[(data >> 1) & 1])?;
        self.data.set_value(D6_PIN, VALUES[(data >> 2) & 1])?;
        self.data.set_value(D7_PIN, VALUES[(data >> 3) & 1])?;
        Ok(())
    }
}

impl lcd::Hardware for GpioCharDisplayDriver {
    fn rs(&mut self, hi: bool) {
        let result = if hi {
            self.rs.set_value(RS_PIN, line::Value::Active)
        } else {
            self.rs.set_value(RS_PIN, line::Value::InActive)
        };

        if result.is_err() {
            println!("cannot set rs to {:?}: {:?}", hi, result.unwrap_err());
        }
    }

    fn enable(&mut self, hi: bool) {
        let result = if hi {
            self.en.set_value(EN_PIN, line::Value::Active)
        } else {
            self.en.set_value(EN_PIN, line::Value::InActive)
        };

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

fn main() -> libgpiod::Result<()> {
    let device = GpioCharDisplayDriver::init(&"/dev/gpiochip2")?;
    let mut display = lcd::Display::new(device);

    println!("Initializing LCD...");
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    println!("LCD ready.");

    write!(&mut display, "Hello, my number today is {: >4}", 42).unwrap();

    Ok(())
}

