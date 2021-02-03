use core::fmt;

use num_traits::int::PrimInt;

mod instruction;
mod operations;
mod program_counter;

use crate::interpreter::config::FONT;
use crate::interpreter::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use instruction::Instruction;
use operations::MAIN_TABLE as OP_TABLE;
use program_counter::ProgramCounter;

pub struct Chip8 {
    memory: [u8; 4096],
    pc: ProgramCounter,
    v: [u8; 16],
    i: u16,
    stack: Vec<u16>,
    pub screen: Screen,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub input: Vec<u8>,
    c48_mode: bool,
}

pub struct Screen {
    pub display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub refresh: bool,
}

enum Reg {
    V(usize),
    I,
}

impl Screen {
    fn new() -> Screen {
        return Screen{
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            refresh: false,
        }
    }

    fn clear(&mut self) {
        self.display = [false; 2048];
    }

    pub fn should_refresh(&mut self) -> bool {
        if !self.refresh {
            return false;
        }

        self.refresh = false;
        true
    }
}

impl Chip8 {
    pub fn new(c48_mode: bool) -> Chip8 {
        return Chip8 {
            memory: [0; 4096],
            pc: ProgramCounter::new_with_value(0x200), // program starts at 0x200
            v: [0; 16],
            i: 0,
            stack: Vec::<u16>::new(),
            screen: Screen::new(),
            delay_timer: 0,
            sound_timer: 0,
            input: Vec::<u8>::new(),
            c48_mode,
        }
        .load_font();
    }

    pub fn load_program_to_memory(mut self, path: &str) -> Result<Chip8, std::io::Error> {
        let f = std::fs::read(path)?;
        let mut addr = 0x200; // program starts at 0x200

        for b in f {
            self.memory[addr] = b;
            addr += 1;
        }

        Ok(self)
    }

    fn load_font(mut self) -> Chip8 {
        let mut addr = 0x050;

        for &f in FONT.iter() {
            self.memory[addr] = f;
            addr += 1;
        }

        self
    }

    pub fn run_instruction(&mut self, is_debug: bool) {
        let instr = self.fetch();
        if is_debug {
            println!("Executed instr: Ox{:X}", instr.to_raw_instr())
        }

        self.decode_and_execute(instr)
    }

    fn decode_and_execute(&mut self, instr: Instruction) {
        OP_TABLE[instr.first_nibble() as usize](self, instr);
    }

    fn fetch(&mut self) -> Instruction {
        let addr = self.pc.value() as usize;

        // read 2 successive bytes from memory.
        let instr = Instruction::new_from_bytes(self.memory[addr], self.memory[addr + 1]);
        self.pc.increment();

        return instr;
    }

    fn set_reg_to<T: PrimInt>(&mut self, r: Reg, val: T) {
        match r {
            Reg::V(x) => {
                self.v[x] = val.to_u8().unwrap_or_default();
            }
            Reg::I => {
                self.i = val.to_u16().unwrap_or_default();
            }
        }
    }

    fn add_to_reg<T: PrimInt>(&mut self, r: Reg, val: T) {
        match r {
            Reg::V(x) => {
                self.v[x] = self.v[x].wrapping_add(val.to_u8().unwrap_or_default());
            }
            Reg::I => {
                self.i = self.i.wrapping_add(val.to_u16().unwrap_or_default());
            }
        }
    }

    fn set_register_flag_if_else_0(&mut self, condition: bool) {
        if condition {
            self.set_reg_to(Reg::V(15), 1u8)
        } else {
            self.set_reg_to(Reg::V(15), 0u8)
        }
    }

    fn draw(&mut self, instr: Instruction) {
        let (x, y) = instr.x_y();
        let mut display_x = (self.v[x] % 64) as usize;
        let mut display_y = (self.v[y] % 32) as usize;

        let bytes_to_read = instr.last_nibble();
        let addr = self.i as usize;
        self.set_reg_to(Reg::V(15), 0u8);

        for i in 0..bytes_to_read as usize {
            let mut sprite_byte = self.memory[addr + i];

            // iterate over all bits of current sprite byte.
            for _ in 0..8 {
                let current_pos = (display_y * DISPLAY_WIDTH) + display_x;

                // break if out of bounds.
                if current_pos >= 2048 {
                    break;
                }

                // if sprite bit is set, flip the display point;
                // if both are on, set flag register.
                if sprite_byte & 0x80 == 0x80 {
                    if self.screen.display[current_pos] {
                        self.screen.display[current_pos] = false;
                        self.set_reg_to(Reg::V(15), 1u8);
                    } else {
                        self.screen.display[current_pos] = true;
                    }
                }
                sprite_byte <<= 1;
                display_x += 1;
            }

            // reset the X position.
            display_x = display_x.checked_sub(8).unwrap_or(0);
            display_y += 1;
        }
    }

    fn handle_unknown_instr(&mut self, instr: Instruction) {
        println!(
            "Warning: Unknown instruction 0x{:X} at program counter {}; skipping",
            instr.to_raw_instr(),
            self.pc.value()
        )
    }
}

impl std::fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("CHIP-8 State");
        ds.field("Program Counter: ", &self.pc.value());

        for i in 0..16 {
            let name = format!("Register V[{}]", i);
            ds.field(name.as_str(), &self.v[i]);
        }

        ds.field("Register I", &self.i);
        ds.field("Delay Time", &self.delay_timer);
        ds.field("Sound Time", &self.sound_timer);
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;
    use super::Instruction;
    #[test]
    fn set_register_instructions_are_decoded_and_executed() {
        let mut machine = Chip8::new(false);

        machine.decode_and_execute(Instruction::new_from_bytes(0x65, 0x42));
        assert_eq!(machine.v[5], 0x42);

        machine.decode_and_execute(Instruction::new_from_bytes(0x86, 0x50));
        assert_eq!(machine.v[6], 0x42);
    }

    #[test]
    fn pressed_key_instruction_is_decoded_and_executed() {
        let mut machine = Chip8::new(false);

        machine.decode_and_execute(Instruction::new_from_bytes(0x65, 0xA));
        assert_eq!(machine.v[5], 0xA);

        machine.input.push(0xA);
        machine.decode_and_execute(Instruction::new_from_bytes(0xE5, 0x9E));
        assert_eq!(machine.pc.value(), 0x202);
    }

    #[test]
    fn b_c_d_instruction_is_decoded_and_executed() {
        let mut machine = Chip8::new(false);

        machine.decode_and_execute(Instruction::new_from_bytes(0x61, 0x7B));
        assert_eq!(machine.v[1], 0x7B);
        machine.decode_and_execute(Instruction::new_from_bytes(0xA6, 0x66));
        assert_eq!(machine.i, 0x666);
        machine.decode_and_execute(Instruction::new_from_bytes(0xF1, 0x33));

        assert_eq!(machine.memory[0x666], 1);
        assert_eq!(machine.memory[0x667], 2);
        assert_eq!(machine.memory[0x668], 3);
    }

    #[test]
    fn draw_instructions_are_decoded_and_executed() {
        let mut machine = Chip8::new(false);

        machine.decode_and_execute(Instruction::new_from_bytes(0x60, 0x1));
        assert_eq!(machine.v[0], 0x1);
        machine.decode_and_execute(Instruction::new_from_bytes(0x61, 0x1));
        assert_eq!(machine.v[1], 0x1);

        machine.decode_and_execute(Instruction::new_from_bytes(0xA6, 0x66));
        assert_eq!(machine.i, 0x666);
        machine.memory[0x666] = 0b01010101;

        machine.decode_and_execute(Instruction::new_from_bytes(0xD0, 0x11));
        assert_eq!(machine.screen.display[65], false);
        assert_eq!(machine.screen.display[66], true);
        assert_eq!(machine.screen.display[67], false);
        assert_eq!(machine.screen.display[68], true);
        assert_eq!(machine.screen.display[69], false);
        assert_eq!(machine.screen.display[70], true);
        assert_eq!(machine.screen.display[71], false);
        assert_eq!(machine.screen.display[72], true);
    }
}
