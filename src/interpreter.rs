use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::{audio::AudioQueue, audio::AudioSpecDesired, rect::Rect};
use sdl2::{event::Event, keyboard::Scancode, EventPump};
use sdl2::{render::Canvas, video::Window};

mod chip8;
pub mod config;

use crate::InterpErr;
use chip8::Chip8;
use config::Config;

// Keeping the main loop at the timer frequency;
// the instruction execution is then "synced" to
// this frequency, e.g. for 500 Hz interpreter frequency,
// this amounts to 500 / 60 ~= 8 cycles, or (assuming
// each opcode execution takes 2 cycles) 4 instructions.
const MAIN_LOOP_FREQUENCY: u32 = 60;

// How many microseconds to sleep, in order to sync at 60 Hz;
// the duration of actual code execution should be subtracted.
const SLEEP_TIME: u128 = ((100 / MAIN_LOOP_FREQUENCY) * 10000) as u128;

// Display size, i.e. how many 'points'.
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Interpreter {
    machine: Chip8,
    video: Canvas<Window>,
    audio: AudioQueue<i16>,
    events: EventPump,
    config: Config,
    debug: Debug,
}

struct Debug {
    running: bool,
    step_exec: bool,
}

impl Interpreter {
    pub fn new(
        sdl_ctx: &sdl2::Sdl,
        rom: Option<&str>,
        config: Config,
    ) -> Result<Interpreter, InterpErr> {
        let emu = Interpreter {
            machine: Chip8::new(config.c48_mode).load_program_to_memory(rom.unwrap())?,
            video: Interpreter::initiate_video(sdl_ctx, &config)?,
            audio: Interpreter::initiate_audio(sdl_ctx)?,
            events: sdl_ctx.event_pump()?,
            config,
            debug: Debug {
                running: true,
                step_exec: false,
            },
        };

        return Ok(emu);
    }


    pub fn run(&mut self) -> Result<(), InterpErr> {
        let mut previous_time: Instant;

        // run the main loop at 60 Hz
        'main_loop: loop {
            previous_time = Instant::now();

            let event_iter: Vec<Event> = self.events.poll_iter().collect();
            for event in event_iter {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    Event::KeyUp { scancode, .. } => {
                        if self.config.debug_mode {
                            self.handle_debug_input(scancode.unwrap());
                        }
                    }
                    _ => {}
                }
            }

            // if in debug mode & paused, skip execution
            if self.is_paused() {
                continue;
            }

            self.handle_timers();
            self.register_pressed_keys();

            for _ in 0..self.config.instructions_per_cycle() {
                self.machine.run_instruction(self.debug.step_exec)
            }

            self.refresh_screen()?;
            self.reset_exec_step();

            self.handle_loop_sync(Instant::now().duration_since(previous_time));

        }

        Ok(())
    }


    fn initiate_video(sdl_ctx: &sdl2::Sdl, config: &Config) -> Result<Canvas<Window>, InterpErr> {
        let video_subsys = sdl_ctx.video()?;

        let win = video_subsys
            .window(
                crate_name!(),
                64 * config.screen_size,
                32 * config.screen_size,
            )
            .position_centered()
            .build()?;

        let mut canvas = win.into_canvas().software().build()?;
        canvas.set_draw_color(config.background_color);
        canvas.clear();
        canvas.set_draw_color(config.foreground_color);

        canvas.present();
        Ok(canvas)
    }

    fn initiate_audio(sdl_ctx: &sdl2::Sdl) -> Result<AudioQueue<i16>, InterpErr> {
        let audio_subsys = sdl_ctx.audio()?;
        let audio_queue = audio_subsys.open_queue::<i16, _>(
            None,
            &AudioSpecDesired {
                freq: Some(44_100),
                channels: Some(1),
                samples: Some(4),
            },
        )?;
        audio_queue.queue(&generate_sound());
        Ok(audio_queue)
    }

    fn refresh_screen(&mut self) -> Result<(), InterpErr> {
        if !self.machine.screen.should_refresh() {
            return Ok(());
        }

        self.video.set_draw_color(self.config.background_color);
        self.video.clear();
        self.video.set_draw_color(self.config.foreground_color);

        for x in 0..64 {
            for y in 0..32 {
                let xy = (y * DISPLAY_WIDTH) + x;

                if self.machine.screen.display[xy] {
                    let r = Rect::new(
                        (x as u32 * self.config.screen_size) as i32,
                        (y as u32 * self.config.screen_size) as i32,
                        self.config.screen_size,
                        self.config.screen_size,
                    );
                    self.video.fill_rect(r)?;
                    self.video.draw_rect(r)?;
                }
            }
        }

        self.video.present();
        Ok(())
    }

    fn handle_timers(&mut self) {
        if self.machine.delay_timer > 0 {
            self.machine.delay_timer -= 1;
        }

        // as long as sound timer is > 0, emit beep
        if self.machine.sound_timer > 0 {
            self.machine.sound_timer -= 1;
            self.audio.queue(&generate_sound());
            self.audio.resume();
        } else {
            self.audio.pause();
        }
    }

    fn register_pressed_keys(&mut self) {
        self.machine.input.clear();
        let keyb_state = self.events.keyboard_state();
        for k in keyb_state.pressed_scancodes().into_iter() {
            match k {
                Scancode::Num1 => self.machine.input.push(0x1),
                Scancode::Num2 => self.machine.input.push(0x2),
                Scancode::Num3 => self.machine.input.push(0x3),
                Scancode::Num4 => self.machine.input.push(0xC),
                Scancode::Q => self.machine.input.push(0x4),
                Scancode::W => self.machine.input.push(0x5),
                Scancode::E => self.machine.input.push(0x6),
                Scancode::R => self.machine.input.push(0xD),
                Scancode::A => self.machine.input.push(0x7),
                Scancode::S => self.machine.input.push(0x8),
                Scancode::D => self.machine.input.push(0x9),
                Scancode::F => self.machine.input.push(0xE),
                Scancode::Y => self.machine.input.push(0xA),
                Scancode::X => self.machine.input.push(0x0),
                Scancode::C => self.machine.input.push(0xB),
                Scancode::V => self.machine.input.push(0xF),
                _ => {}
            }
        }
    }

    fn handle_debug_input(&mut self, scancode: Scancode) {
        match scancode {
            // Debug
            Scancode::P => println!("{:?}", self.machine),
            Scancode::End => {
                println!(
                    "Toggling interpreter state to - running: {}",
                    !self.debug.running
                );
                self.toggle_state()
            }
            Scancode::PageDown => {
                // ignore if interpreter not paused
                if self.debug.running {
                    return;
                }

                println!("Running next cycle");
                self.debug.step_exec = true;
            }
            _ => (),
        }
    }

    fn handle_loop_sync(&mut self, elapsed: Duration) {
        sleep(Duration::from_micros(
            SLEEP_TIME.checked_sub(elapsed.as_micros()).unwrap_or(0u128) as u64,
        ));
    }

    fn is_paused(&self) -> bool {
        self.config.debug_mode && !self.debug.running && !self.debug.step_exec
    }

    fn toggle_state(&mut self) {
        self.debug.running = !self.debug.running
    }

    fn reset_exec_step(&mut self) {
        if self.debug.step_exec {
            self.debug.step_exec = false
        }
    }
}

// generate a square wave
fn generate_sound() -> Vec<i16> {
    let tone_volume = 1_000i16;
    let period = 48_000 / 256;
    let mut result = Vec::new();

    for x in 0..8_000 {
        result.push(if (x / period) % 2 == 0 {
            tone_volume
        } else {
            -tone_volume
        });
    }
    result
}
