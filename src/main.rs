use charl::{GpioCharDisplayDriver, PinMap};
use lcd::{DisplayBlink, DisplayCursor, DisplayMode};

use std::{fmt::Write, io, thread, time::Duration};

use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;

use systemstat::{saturating_sub_bytes, Platform, System};

const RS_PIN: u32 = 10; //15;
const RW_PIN: u32 = 9; //14;
const EN_PIN: u32 = 11; //13;
                        // const OE_PIN: u32 = 12;
const D4_PIN: u32 = 5; //11;
const D5_PIN: u32 = 6; //10;
const D6_PIN: u32 = 13; // 9;
const D7_PIN: u32 = 19; // 8;

fn main() -> io::Result<()> {
    let pinmap = PinMap {
        rs: RS_PIN,
        rw: RW_PIN,
        en: EN_PIN,
        d4: D4_PIN,
        d5: D5_PIN,
        d6: D6_PIN,
        d7: D7_PIN,
    };

    let device = GpioCharDisplayDriver::init("gpiochip0", pinmap)?;
    let mut display = lcd::Display::new(device);

    let sys = System::new();

    println!("Initializing LCD...");
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    println!("LCD ready.");

    display.display(
        DisplayMode::DisplayOn,
        DisplayCursor::CursorOn,
        DisplayBlink::BlinkOn,
    );

    let mut cpu = sys.cpu_load_aggregate().unwrap();
    let mut cpu_measurement_counter = 0;

    let mut line3 = "                    ".to_string();
    let mut line4 = "                    ".to_string();

    loop {
        let current_time = Utc::now().with_timezone(&Tokyo);
        display.position(0, 0);

        let line1 = format!(
            "{}  {}",
            current_time.format("%Y/%m/%d"),
            current_time.format("%H:%M:%S")
        );

        let line2 = format!(
            "{:<17}{}",
            current_time.format("%A"),
            current_time.format("%:::z")
        );

        if let Ok(mem) = sys.memory() {
            line3 = format!("{}/{}", saturating_sub_bytes(mem.total, mem.free), mem.free);
        }

        if cpu_measurement_counter == 0 {
            if let Ok(load) = cpu.done() {
                line4 = format!(
                    "usr:{0:.1}%  sys:{1:.1}%",
                    load.user * 100.0,
                    load.system * 100.0
                );
            }

            cpu = sys.cpu_load_aggregate().unwrap();
        }

        write!(&mut display, "{}", line1).unwrap();
        write!(&mut display, "{:^20}", line3).unwrap();
        write!(&mut display, "{}", line2).unwrap();
        write!(&mut display, "{:^20}", line4).unwrap();

        display.display(
            DisplayMode::DisplayOn,
            DisplayCursor::CursorOff,
            DisplayBlink::BlinkOff,
        );

        cpu_measurement_counter += 1;
        if cpu_measurement_counter > 5 {
            cpu_measurement_counter = 0;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
