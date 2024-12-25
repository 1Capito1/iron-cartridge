#![allow(dead_code)]
use crate::cpu::flags::Flags;
use crate::library;
use crate::memory::memory::Memory;
pub struct Cpu {
    pub address_bus: u16,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub idx: u8,
    pub idy: u8,
    pub flags: Flags,
    pub memory: Memory,
}

impl Cpu {
    const STACK_LOCATION_OFFSET: u16 = 0x100;
    const SIGN_BIT: u8 = 7;
    pub fn new() -> Self {
        Self {
            address_bus: 0,
            program_counter: 0,
            stack_pointer: 0xFF,
            accumulator: 0,
            idx: 0,
            idy: 0,
            flags: Flags::default(),
            memory: Memory::new(),
        }
    }
    pub fn fetch(&self) {
        unimplemented!();
    }
    pub fn decode(&self) {
        unimplemented!();
    }
    pub fn execute(&self) {
        unimplemented!();
    }

    /// helper function to determine if the flags zero and negative need to be updated after an instruction
    pub fn update_flags(&mut self, register: u8) {
        self.flags.zero = register == 0;
        self.flags.negative = library::isolate_bit_u8(register, Self::SIGN_BIT) != 0;
    }
    pub fn read_memory(&self, location: u16) -> u8 {
        self.memory[location]
    }
    pub fn push_to_stack(&mut self, value: u8) {
        self.memory[Self::STACK_LOCATION_OFFSET + self.stack_pointer as u16] = value;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    pub fn pull_from_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read_memory(self.stack_location())
    }
    pub fn stack_location(&self) -> u16 {
        Self::STACK_LOCATION_OFFSET + self.stack_pointer as u16
    }
    pub fn reset(&mut self) {
        self.stack_pointer = 0xFF;
    }
    // Load store instructions
    pub fn lda(&mut self, value: u8) {
        self.accumulator = value;
        self.update_flags(self.accumulator);
    }
    pub fn ldx(&mut self, value: u8) {
        self.idx = value;
        self.update_flags(self.idx);
    }
    pub fn ldy(&mut self, value: u8) {
        self.idy = value;
        self.update_flags(self.idy);
    }
    pub fn sta(&mut self, address: u16) {
        self.memory[address] = self.accumulator;
    }
    pub fn stx(&mut self, address: u16) {
        self.memory[address] = self.idx;
    }
    pub fn sty(&mut self, address: u16) {
        self.memory[address] = self.idy;
    }

    // Transfer instructions
    pub fn tax(&mut self) {
        self.idx = self.accumulator;
        self.update_flags(self.idx);
    }
    pub fn tay(&mut self) {
        self.idy = self.accumulator;
        self.update_flags(self.idy);
    }
    pub fn txa(&mut self) {
        self.accumulator = self.idx;
        self.update_flags(self.accumulator);
    }
    pub fn tya(&mut self) {
        self.accumulator = self.idy;
        self.update_flags(self.accumulator);
    }

    // Stack Operations
    pub fn tsx(&mut self) {
        self.idx = self.stack_pointer;
        self.update_flags(self.idx);
    }
    pub fn txs(&mut self) {
        self.stack_pointer = self.idx;
    }
    pub fn pha(&mut self) {
        self.push_to_stack(self.accumulator);
        self.accumulator -= 1;
    }
    pub fn php(&mut self) {
        self.push_to_stack(self.flags.into_u8());
        self.stack_pointer -= 1;
    }
    pub fn pla(&mut self) {
        self.stack_pointer += 1;
        self.accumulator = self.read_memory(self.stack_location());
        self.update_flags(self.accumulator);
    }
    pub fn plp(&mut self) {
        self.stack_pointer += 1;
        self.flags.from_u8(self.read_memory(self.stack_location()));
    }

    // logical instructions
    pub fn and(&mut self, rhs: u8) {
        self.accumulator &= rhs;
    }
    pub fn eor(&mut self, rhs: u8) {
        self.accumulator ^= rhs;
    }
    pub fn ora(&mut self, rhs: u8) {
        self.accumulator |= rhs;
    }
    pub fn bit(&mut self, rhs: u8) {
        let res = self.accumulator & rhs;
        self.flags.zero = res == 0;
        self.flags.overflow = library::isolate_bit_u8(res, 6) == 1;
        self.flags.negative = library::isolate_bit_u8(res, Self::SIGN_BIT) == 1;
    }

    // Arithematic instructions
    pub fn adc(&mut self, rhs: u8) {
        let (res, carry) = self.accumulator.overflowing_add(rhs);
        self.accumulator = res;
        self.flags.carry = carry;
        self.update_flags(self.accumulator);
        let res_bit_7 = library::isolate_bit_u8(res, Self::SIGN_BIT) != 0;
        let accumulator_bit_7 = library::isolate_bit_u8(self.accumulator, Self::SIGN_BIT) != 0;
        let rhs_bit_7 = library::isolate_bit_u8(rhs, Self::SIGN_BIT) != 0;

        self.flags.overflow = (accumulator_bit_7 == rhs_bit_7) && (res_bit_7 != accumulator_bit_7);
    }
    pub fn sbc(&mut self, rhs: u8) {
        let carry = if self.flags.carry { 0 } else { 1 }; // Carry inverted for SBC
        let (intermediate_result, borrow1) = self.accumulator.overflowing_sub(rhs);
        let (result, borrow2) = intermediate_result.overflowing_sub(carry);
        self.accumulator = result;

        self.flags.carry = !(borrow1 || borrow2); // Carry set when no borrow
        self.update_flags(self.accumulator);

        let res_bit_7 = library::isolate_bit_u8(result, Self::SIGN_BIT) != 0;
        let accumulator_bit_7 = library::isolate_bit_u8(self.accumulator, Self::SIGN_BIT) != 0;
        let rhs_bit_7 = library::isolate_bit_u8(rhs, Self::SIGN_BIT) != 0;
        self.flags.overflow = (accumulator_bit_7 != rhs_bit_7) && (res_bit_7 != accumulator_bit_7);
    }
    pub fn cmp(&mut self, rhs: u8) {
        let result = self.accumulator.wrapping_sub(rhs);
        self.flags.carry = self.accumulator >= rhs;
        self.flags.zero = self.accumulator == rhs;
        self.flags.negative = library::isolate_bit_u8(result, Self::SIGN_BIT) != 0;
    }
    pub fn cmx(&mut self, rhs: u8) {
        let result = self.idx.wrapping_sub(rhs);
        self.flags.carry = self.idx >= rhs;
        self.flags.zero = self.idx == rhs;
        self.flags.negative = library::isolate_bit_u8(result, Self::SIGN_BIT) != 0;
    }
    pub fn cmy(&mut self, rhs: u8) {
        let result = self.idy.wrapping_sub(rhs);
        self.flags.carry = self.idy >= rhs;
        self.flags.zero = self.idy == rhs;
        self.flags.negative = library::isolate_bit_u8(result, Self::SIGN_BIT) != 0;
    }

    // increments/decrements
    pub fn inc(&mut self, location: u16) {
        let carry;
        (self.memory[location], carry) = self.memory[location].overflowing_add(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.memory[location], Self::SIGN_BIT) != 0;
    }
    pub fn inx(&mut self) {
        let carry;
        (self.idx, carry) = self.idx.overflowing_add(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.idx, Self::SIGN_BIT) != 0;
    }
    pub fn iny(&mut self) {
        let carry;
        (self.idy, carry) = self.idx.overflowing_add(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.idy, Self::SIGN_BIT) != 0;
    }

    pub fn dec(&mut self, location: u16) {
        let carry;
        (self.memory[location], carry) = self.memory[location].overflowing_sub(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.memory[location], Self::SIGN_BIT) != 0;
    }
    pub fn dex(&mut self) {
        let carry;
        (self.idx, carry) = self.idx.overflowing_sub(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.idx, Self::SIGN_BIT) != 0;
    }
    pub fn dey(&mut self) {
        let carry;
        (self.idy, carry) = self.idy.overflowing_add(1);
        self.flags.carry = carry;
        self.flags.negative = library::isolate_bit_u8(self.idy, Self::SIGN_BIT) != 0;
    }

    // shifting operations
    pub fn asl(&mut self, value: &mut u8) {
        let carry;
        (*value, carry) = value.overflowing_mul(2);
        self.flags.carry = carry;
        self.flags.zero = *value == 0;
        self.flags.negative = library::isolate_bit_u8(*value, Self::SIGN_BIT) != 0;
    }

    pub fn lsr(&mut self, value: &mut u8) {
        self.flags.carry = library::isolate_bit_u8(*value, 0) != 0;
        *value >>= 1;
        self.flags.zero = *value == 0;
        self.flags.negative = false;
    }

    pub fn rol(&mut self, value: &mut u8) {
        self.flags.carry = library::isolate_bit_u8(*value, Self::SIGN_BIT) != 0; // carry is sign
                                                                                 // bit before rotation
        *value = value.rotate_left(1);
        self.flags.zero = *value == 0;
        self.flags.negative = library::isolate_bit_u8(*value, Self::SIGN_BIT) != 0;
    }

    pub fn ror(&mut self, value: &mut u8) {
        self.flags.carry = library::isolate_bit_u8(*value, 0) != 0;
        *value = value.rotate_right(1);
        self.flags.zero = *value == 0;
        self.flags.negative = library::isolate_bit_u8(*value, Self::SIGN_BIT) != 0;
    }

    // Jumps
    pub fn jmp(&mut self, location: u16) {
        self.program_counter = location;
    }
    pub fn jsr(&mut self, location: u16) {
        let [lo, hi] = self.program_counter.to_le_bytes();
        self.push_to_stack(lo);
        self.push_to_stack(hi);
        self.program_counter = location;
    }
    pub fn rts(&mut self) {
        let lo = self.pull_from_stack();
        let hi = self.pull_from_stack();
        self.program_counter = u16::from_le_bytes([lo, hi]);
    }

    // Branching
    pub fn branch_if(&mut self, condition: bool, immediate: i8) {
        if condition {
            self.program_counter = self.program_counter.wrapping_add(immediate as i16 as u16);
        }
    }
    pub fn bcc(&mut self, immediate: i8) {
        self.branch_if(!self.flags.carry, immediate);
    }
    pub fn bcs(&mut self, immediate: i8) {
        self.branch_if(self.flags.carry, immediate);
    }
    pub fn beq(&mut self, immediate: i8) {
        self.branch_if(self.flags.zero, immediate);
    }
    pub fn bmi(&mut self, immediate: i8) {
        self.branch_if(self.flags.negative, immediate);
    }
    pub fn bne(&mut self, immediate: i8) {
        self.branch_if(!self.flags.zero, immediate);
    }
    pub fn bpl(&mut self, immediate: i8) {
        self.branch_if(!self.flags.negative, immediate);
    }
    pub fn bvc(&mut self, immediate: i8) {
        self.branch_if(!self.flags.overflow, immediate);
    }
    pub fn bvs(&mut self, immediate: i8) {
        self.branch_if(self.flags.overflow, immediate);
    }

    // status flag changes
    pub fn clc(&mut self) {
        self.flags.carry = false
    }
    pub fn cld(&mut self) {
        self.flags.decimal_mode = false
    }
    pub fn cli(&mut self) {
        self.flags.interrupt_disable = false
    }
    pub fn clv(&mut self) {
        self.flags.overflow = false
    }
    pub fn sec(&mut self) {
        self.flags.carry = true
    }
    pub fn sed(&mut self) {
        self.flags.decimal_mode = true
    }
    pub fn sei(&mut self) {
        self.flags.interrupt_disable = true
    }

    // system functions
    pub fn brk(&mut self) {
        let [lo, hi] = (self.program_counter + 2).to_le_bytes();
        self.push_to_stack(hi);
        self.push_to_stack(lo);
        self.flags.break_command = true;
        let bitflags = self.flags.into_u8();
        self.push_to_stack(bitflags);

        let lo = self.memory[0xFFFE];
        let hi = self.memory[0xFFFF];
        self.program_counter = u16::from_le_bytes([lo, hi]);
    }

    pub fn nop(&self) {}

    pub fn rti(&mut self) {
        let flags = self.pull_from_stack();
        let lo = self.pull_from_stack();
        let hi = self.pull_from_stack();

        self.flags.from_u8(flags);
        self.program_counter = u16::from_le_bytes([lo, hi]);
    }
}
