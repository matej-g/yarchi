#![warn(clippy::all, rust_2018_idioms)]

#[macro_use]
extern crate clap;

use crate::interpreter::config::Config;
use crate::interpreter::Interpreter;

use clap::{App, Arg, crate_authors, crate_description};

mod interpreter;

const DEBUG_MSG: &str = "
Available actions upon pressing:
- P - prints current state of CHIP-8
- End - pause/resume emulation
- PgDown - executes next cycle (4 instructions; possible only if emulation is paused)
";

type InterpErr = Box<dyn std::error::Error>;
type InterpResult<T> = Result<T, InterpErr>;

fn main() -> InterpResult<()> {
    let long_debug_msg = format!(
        "{}\n{}",
        "Enables debug mode, which allows for pausing emulation and executing cycles step-by-step.",
        DEBUG_MSG
    );

    let about_with_controls = format!(
        "{}\n{}",
        crate_description!(),
        "
To control the interpreter, use the left side of your keyboard.
The controls are mapped to the following keys:

|1|2|3|4|
|Q|W|E|R|
|A|S|D|F|
|Z|X|C|V|
",
    );

    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(about_with_controls.as_str())
        .author(crate_authors!())
        .arg(
            Arg::with_name("screen-size")
                .takes_value(true)
                .long("screen-size")
                .short("s")
                .help("Sets the screen size to small (640x320, default), medium (768x384) or large (1024x512)")
                .possible_values(&["small", "medium", "large"])
        )
        .arg(
            Arg::with_name("freq")
                .takes_value(true)
                .long("interpreter-frequency")
                .short("f")
                .help("Adjusts emulation speed to specified Hz value. Valid values: 200-1000 Hz. Default: 500 Hz.")
                .validator(is_valid_emu_frequency)
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .short("d")
                .help("Enables debug mode, which allows for pausing emulation and executing cycles step-by-step.
For all available commands, print information with --help.")
                .long_help(long_debug_msg.as_str())
        )
        .arg(
            Arg::with_name("c48")
                .long("chip-48-mode")
                .short("c")
                .help("Executes certain instructions in a mode compatible with CHIP-48. Required for some programs.")
        )
        .arg(
            Arg::with_name("fg-color")
            .takes_value(true)
            .long("foreground-color")
            .help("Changes foreground color to specified RGB value. Format: R,G,B")
            .validator(is_valid_rgb_color)
        )
        .arg(
            Arg::with_name("bg-color")
            .takes_value(true)
            .long("background-color")
            .help("Changes background color to specified RGB value. Format: R,G,B")
            .validator(is_valid_rgb_color)
        )
        .arg(Arg::with_name("INPUT").required(true).help("Path to ROM which should be run"));

    let matches = app.get_matches();

    let sdl_ctx = sdl2::init()?;
    let mut interpreter = Interpreter::new(
        &sdl_ctx,
        matches.value_of("INPUT"),
        Config::from_args(&matches),
    )?;
    interpreter.run()?;

    Ok(())
}

fn is_valid_emu_frequency(freq: String) -> Result<(), String> {
    match freq.parse::<u16>() {
        Ok(f) => {
            if f < 200 || f > 1000 {
                return Err(
                    "invalid interpreter frequency specified: must be in range 200 - 1000 Hz"
                        .to_string(),
                );
            }

            Ok(())
        }
        Err(e) => Err(format!("parsing interpreter frequency failed: {}", e)),
    }
}

fn is_valid_rgb_color(rgb: String) -> Result<(), String> {
    let vals: Vec<&str> = rgb.split(",").collect();
    if vals.len() != 3 {
        return Err("invalid RGB value provided. Valid format: R,G,B".to_string());
    }

    for v in vals {
        match v.parse::<u8>() {
            Err(e) => return Err(format!("parsing RGB values failed: {}", e)),
            _ => return Ok(()),
        }
    }

    Ok(())
}
