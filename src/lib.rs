use gpiod::{Chip, Lines, Options, Output};
use std::path::Path;
use std::{io, thread, time::Duration};

pub mod layout;

const HIGH: bool = true;
const LOW: bool = false;

pub struct GpioCharDisplayDriver {
    rs: Lines<Output>,
    en: Lines<Output>,
    data: Lines<Output>,
}

pub struct PinMap {
    pub rs: u32,
    pub rw: u32,
    pub en: u32,

    pub d4: u32,
    pub d5: u32,
    pub d6: u32,
    pub d7: u32,
}

impl GpioCharDisplayDriver {
    pub fn init(dpath: impl AsRef<Path>, pin: PinMap) -> io::Result<Self> {
        let mut chip = Chip::new(dpath)?;

        let rs = Self::init_output_ctrl(&mut chip, pin.rs, HIGH)?;
        let _ = Self::init_output_ctrl(&mut chip, pin.rw, LOW)?;
        let en = Self::init_output_ctrl(&mut chip, pin.en, LOW)?;
        let data = Self::init_data(&mut chip, pin)?;

        Ok(Self { rs, en, data })
    }

    fn init_output_ctrl(chip: &mut Chip, pin: u32, init: bool) -> io::Result<Lines<Output>> {
        let opts = Options::output([pin]).values([init]);
        chip.request_lines(opts)
    }

    fn init_data(chip: &mut Chip, pin: PinMap) -> io::Result<Lines<Output>> {
        let opts = Options::output([pin.d4, pin.d5, pin.d6, pin.d7]).values(0u8);
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
