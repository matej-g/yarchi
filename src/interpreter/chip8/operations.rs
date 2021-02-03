use super::instruction::Instruction;
use super::{Chip8, Reg};

use rand::prelude::*;

pub const MAIN_TABLE: [fn(&mut Chip8, Instruction); 16] = [
    op_table_0, op_1nnn, op_2nnn, op_3xkk, op_4xkk, op_5xy0, op_6xkk, op_7xkk, op_table_8, op_9xy0,
    op_annn, op_bnnn, op_cxkk, op_dxyn, op_table_e, op_table_f,
];

fn op_table_0(c: &mut Chip8, instr: Instruction) {
    match instr.kk() {
        0xE0 => {
            c.screen.clear();
            c.screen.refresh = true;
        }
        0xEE => c.pc.set_to(c.stack.pop().unwrap_or_default()),
        _ => c.handle_unknown_instr(instr),
    }
}

fn op_table_8(c: &mut Chip8, instr: Instruction) {
    let (x, y) = instr.x_y();
    match instr.last_nibble() {
        0x0 => c.set_reg_to(Reg::V(x), c.v[y]),
        0x1 => c.set_reg_to(Reg::V(x), c.v[x] | c.v[y]),
        0x2 => c.set_reg_to(Reg::V(x), c.v[x] & c.v[y]),
        0x3 => c.set_reg_to(Reg::V(x), c.v[x] ^ c.v[y]),
        0x4 => {
            let (val, overflow) = c.v[x].overflowing_add(c.v[y]);
            c.set_register_flag_if_else_0(overflow);
            c.set_reg_to(Reg::V(x), val);
        }
        0x5 => {
            let (val, overflow) = c.v[x].overflowing_sub(c.v[y]);
            c.set_register_flag_if_else_0(!overflow);
            c.set_reg_to(Reg::V(x), val);
        }
        0x6 => {
            if !c.c48_mode {
                c.set_reg_to(Reg::V(x), c.v[y])
            };
            let (val, overflow) = c.v[x].overflowing_shr(1);
            c.set_register_flag_if_else_0(overflow);
            c.set_reg_to(Reg::V(x), val);
        }
        0x7 => {
            let (val, overflow) = c.v[y].overflowing_sub(c.v[x]);
            c.set_register_flag_if_else_0(!overflow);
            c.set_reg_to(Reg::V(x), val);
        }
        0xE => {
            if !c.c48_mode {
                c.set_reg_to(Reg::V(x), c.v[y])
            };
            let (val, overflow) = c.v[x].overflowing_shl(1);
            c.set_register_flag_if_else_0(!overflow);
            c.set_reg_to(Reg::V(x), val);
        }
        _ => c.handle_unknown_instr(instr),
    }
}

fn op_table_e(c: &mut Chip8, instr: Instruction) {
    match instr.kk() {
        0x9E => c.pc.increment_if(c.input.contains(&c.v[instr.x()])),
        0xA1 => c.pc.increment_if(!c.input.contains(&c.v[instr.x()])),
        _ => c.handle_unknown_instr(instr),
    }
}
fn op_table_f(c: &mut Chip8, instr: Instruction) {
    let x = instr.x();
    match instr.kk() {
        0x07 => c.set_reg_to(Reg::V(x), c.delay_timer),
        0x0A => {
            c.pc.decrement_if((&c.input).is_empty());
            let key = c.input.first().unwrap_or(&0u8).to_owned();
            c.set_reg_to(Reg::V(x), key)
        }
        0x15 => c.delay_timer = c.v[x],
        0x18 => c.sound_timer = c.v[x],
        0x1E => {
            c.add_to_reg(Reg::I, c.v[x]);

            if c.i > 0x1000 {
                c.set_reg_to(Reg::V(15), 1u8);
            }
        }
        0x29 => {
            // get last nibble only for char.
            let ch = (c.v[x] & 0xF) as u16;
            // start address + offset to given character
            c.set_reg_to(Reg::I, 0x050 + (5 * ch));
        }
        0x33 => {
            let val = c.v[x];
            c.memory[c.i as usize] = val / 100;
            c.memory[(c.i + 1) as usize] = (val % 100) / 10;
            c.memory[(c.i + 2) as usize] = (val % 100) % 10;
        }
        0x55 => {
            for n in 0..x + 1 {
                c.memory[(c.i as usize + n) as usize] = c.v[n];
            }
        }
        0x65 => {
            for n in 0..x + 1 {
                c.set_reg_to(Reg::V(n), c.memory[(c.i as usize + n) as usize]);
            }
        }
        _ => c.handle_unknown_instr(instr),
    }
}

fn op_1nnn(c: &mut Chip8, instr: Instruction) {
    c.pc.set_to(instr.nnn())
}

fn op_2nnn(c: &mut Chip8, instr: Instruction) {
    c.stack.push(c.pc.value());
    c.pc.set_to(instr.nnn());
}

fn op_3xkk(c: &mut Chip8, instr: Instruction) {
    c.pc.increment_if(c.v[instr.x()] == instr.kk());
}

fn op_4xkk(c: &mut Chip8, instr: Instruction) {
    c.pc.increment_if(c.v[instr.x()] != instr.kk());
}

fn op_5xy0(c: &mut Chip8, instr: Instruction) {
    let (x, y) = instr.x_y();
    c.pc.increment_if(c.v[x] == c.v[y]);
}

fn op_6xkk(c: &mut Chip8, instr: Instruction) {
    c.set_reg_to(Reg::V(instr.x()), instr.kk());
}
fn op_7xkk(c: &mut Chip8, instr: Instruction) {
    c.add_to_reg(Reg::V(instr.x()), instr.kk());
}
fn op_9xy0(c: &mut Chip8, instr: Instruction) {
    let (x, y) = instr.x_y();
    c.pc.increment_if(c.v[x] != c.v[y]);
}
fn op_annn(c: &mut Chip8, instr: Instruction) {
    c.set_reg_to(Reg::I, instr.nnn());
}

// ambiguous OP: either BNNN or BXKK
fn op_bnnn(c: &mut Chip8, instr: Instruction) {
    if c.c48_mode {
        c.pc.set_to((instr.kk() + c.v[instr.x()]) as u16)
    } else {
        c.pc.set_to(instr.nnn() + c.v[0] as u16)
    }
}

fn op_cxkk(c: &mut Chip8, instr: Instruction) {
    let r: u8 = thread_rng().gen();
    c.set_reg_to(Reg::V(instr.x()), r & instr.kk());
}

fn op_dxyn(c: &mut Chip8, instr: Instruction) {
    c.draw(instr);
    c.screen.refresh = true;
}
