#[derive(Copy, Clone)]
pub struct Instruction(u16);

impl Instruction {
    pub fn new_from_bytes(b1: u8, b2: u8) -> Instruction {
        return Instruction(u16::from_be_bytes([b1, b2]));
    }

    pub fn to_raw_instr(self) -> u16 {
        return self.0;
    }

    pub fn first_nibble(self) -> u8 {
        return ((self.0 & 0xF000) >> 12) as u8;
    }

    pub fn last_nibble(self) -> u8 {
        return (self.0 & 0xF) as u8;
    }

    pub fn x(self) -> usize {
        return ((self.0 & 0x0F00) >> 8) as usize;
    }

    pub fn x_y(self) -> (usize, usize) {
        return (
            ((self.0 & 0x0F00) >> 8) as usize,
            ((self.0 & 0x0F0) >> 4) as usize,
        );
    }

    pub fn nnn(self) -> u16 {
        return self.0 & 0x0FFF;
    }

    pub fn kk(self) -> u8 {
        return (self.0 & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn instruction_is_constructed_correctly() {
        let instr = Instruction::new_from_bytes(0x12, 0x34);
        assert_eq!(instr.to_raw_instr(), 0x1234);
    }

    #[test]
    fn instruction_is_read_correctly() {
        let instr = Instruction::new_from_bytes(0x12, 0x34);

        assert_eq!(instr.first_nibble(), 0x1);
        assert_eq!(instr.last_nibble(), 0x4);
        assert_eq!(instr.x(), 0x2);
        assert_eq!(instr.x_y(), (0x2, 0x3));
        assert_eq!(instr.nnn(), 0x234);
        assert_eq!(instr.kk(), 0x34);
    }
}
