#[derive(Default, Debug, PartialEq)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}
// 7	Negative (N)	Set if the most significant bit of the result is 1 (sign bit)
// 6	Overflow (V)	Set if an overflow occurred in signed arithmetic operations
// 5	Unused (U)	Always set to 1 in the status register
// 4	Break Command (B)	Set when a BRK (break) instruction is executed
// 3	Decimal Mode (D)	Set if the CPU is in decimal mode (rarely used in NES)
// 2	Interrupt Disable (I)	Set to disable maskable interrupts
// 1	Zero (Z)	Set if the result of an operation is zero
// 0	Carry (C)	Set if a carry/borrow has occurred in arithmetic operations

impl Flags {
    pub fn into_u8(&self) -> u8 {
        (self.negative as u8) << 7 |
        (self.overflow as u8) << 6 |
        1 << 5 | // bit 5 is always set
        (self.break_command as u8) << 4 |
        (self.decimal_mode as u8) << 3 |
        (self.interrupt_disable as u8) << 2 |
        (self.zero as u8) << 1 |
        (self.carry as u8) << 0
    }
    pub fn from_u8(&mut self, val: u8) {
        self.negative = (val as u8) << 7 == 1;
        self.overflow = (val as u8) << 6 == 1;
        self.break_command = (val as u8) << 4 == 1;
        self.decimal_mode = (val as u8) << 3 == 1;
        self.interrupt_disable = (val as u8) << 2 == 1;
        self.zero = (val as u8) << 1 == 1;
        self.carry = (val as u8) << 0 == 1;
    }
}
