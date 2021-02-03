use sdl2::pixels::Color;

use crate::interpreter::MAIN_LOOP_FREQUENCY;

// Used to calculate the actual screen size from configuration.
const DEFAULT_SCREEN_SIZE_COEFF: u32 = 10;

// Default frequency to use.
const DEFAULT_EMU_FREQUENCY: u32 = 500;

const DEFAULT_BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const DEFAULT_FOREGROUND_COLOR: Color = Color::RGB(0, 255, 102);

// 16 characters, each represented by 5 bytes
pub const FONT: [u8; 0x10 * 5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Config {
    pub screen_size: u32,
    emu_speed: u32,
    pub background_color: Color,
    pub foreground_color: Color,
    pub debug_mode: bool,
    pub c48_mode: bool,
}

impl Config {
    pub fn from_args(matches: &clap::ArgMatches<'_>) -> Config {
        if matches.is_present("debug") {
            println!("{}\n{}", "Entering debug mode...", crate::DEBUG_MSG)
        }

        Config {
            screen_size: Config::set_screen_size(matches),
            emu_speed: Config::set_emu_frequency(matches),
            background_color: Config::set_color(matches, "bg-color")
                .unwrap_or(DEFAULT_BACKGROUND_COLOR),
            foreground_color: Config::set_color(matches, "fg-color")
                .unwrap_or(DEFAULT_FOREGROUND_COLOR),
            debug_mode: matches.is_present("debug"),
            c48_mode: matches.is_present("c48"),
        }
    }

    // Assuming each instruction takes 2 cycles.
    pub fn instructions_per_cycle(&self) -> u32 {
        return (self.emu_speed / MAIN_LOOP_FREQUENCY) / 2;
    }

    fn set_color(m: &clap::ArgMatches<'_>, arg: &str) -> Option<Color> {
        match m.value_of(arg) {
            Some(v) => {
                let rgb: Vec<&str> = v.split(",").collect();
                Some(Color::RGB(
                    rgb[0].parse::<u8>().unwrap(),
                    rgb[1].parse::<u8>().unwrap(),
                    rgb[2].parse::<u8>().unwrap(),
                ))
            }
            _ => None,
        }
    }

    fn set_screen_size(m: &clap::ArgMatches<'_>) -> u32 {
        match m.value_of("screen-size") {
            Some(v) => match v {
                "medium" => 12,
                "large" => 16,
                _ => DEFAULT_SCREEN_SIZE_COEFF,
            },
            _ => DEFAULT_SCREEN_SIZE_COEFF,
        }
    }

    fn set_emu_frequency(m: &clap::ArgMatches<'_>) -> u32 {
        match m.value_of("freq") {
            Some(v) => v.parse::<u32>().unwrap(),
            _ => DEFAULT_EMU_FREQUENCY,
        }
    }
}
