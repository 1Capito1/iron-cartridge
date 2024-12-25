#[cfg(test)]
mod tests {
    use crate::cpu::cpu::Cpu;
    use crate::cpu::flags::Flags;

    fn setup_cpu() -> Cpu {
        Cpu::new()
    }

    #[test]
    fn test_cpu_initial_state() {
        let cpu = setup_cpu();
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.stack_pointer, 0xFF);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.idx, 0);
        assert_eq!(cpu.idy, 0);
        assert_eq!(cpu.flags, Flags::default());
    }

    #[test]
    fn test_lda_instruction() {
        let mut cpu = setup_cpu();
        cpu.lda(0x42);
        assert_eq!(cpu.accumulator, 0x42);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);

        cpu.lda(0x00);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.flags.zero, true);

        cpu.lda(0x80); // Test negative flag.
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn test_stack_operations() {
        let mut cpu = setup_cpu();
        cpu.push_to_stack(0x42);
        assert_eq!(cpu.stack_pointer, 0xFE);
        assert_eq!(cpu.read_memory(0x01FF), 0x42);

        let value = cpu.pull_from_stack();
        assert_eq!(value, 0x42);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_transfer_instructions() {
        let mut cpu = setup_cpu();
        cpu.lda(0x55);
        cpu.tax();
        assert_eq!(cpu.idx, 0x55);

        cpu.tay();
        assert_eq!(cpu.idy, 0x55);

        cpu.ldx(0x80);
        cpu.txa();
        assert_eq!(cpu.accumulator, 0x80);
    }

    #[test]
    fn test_arithmetic_instructions() {
        let mut cpu = setup_cpu();
        cpu.lda(0x10);
        cpu.adc(0x20);
        assert_eq!(cpu.accumulator, 0x30);
        assert_eq!(cpu.flags.carry, false);

        cpu.adc(0xF0); // Overflow test.
        assert_eq!(cpu.accumulator, 0x20);
        assert_eq!(cpu.flags.carry, true);

        cpu.sbc(0x10);
        assert_eq!(cpu.accumulator, 0x10);
        assert_eq!(cpu.flags.carry, true);

        cpu.sbc(0x20);
        assert_eq!(cpu.accumulator, 0xF0);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn test_logical_instructions() {
        let mut cpu = setup_cpu();
        cpu.lda(0b1100_1100);
        cpu.and(0b1010_1010);
        assert_eq!(cpu.accumulator, 0b1000_1000);

        cpu.eor(0b1111_0000);
        assert_eq!(cpu.accumulator, 0b0111_1000);

        cpu.ora(0b0000_1111);
        assert_eq!(cpu.accumulator, 0b0111_1111);
    }

    #[test]
    fn test_shift_instructions() {
        let mut cpu = setup_cpu();
        let mut value = 0b0100_0000;
        cpu.asl(&mut value);
        assert_eq!(value, 0b1000_0000);
        assert_eq!(cpu.flags.carry, false);

        let mut value = 0b1000_0001;
        cpu.lsr(&mut value);
        assert_eq!(value, 0b0100_0000);
        assert_eq!(cpu.flags.carry, true);

        let mut value = 0b1000_0000;
        cpu.rol(&mut value);
        assert_eq!(value, 0b0000_0001);
        assert_eq!(cpu.flags.carry, true);

        let mut value = 0b0000_0001;
        cpu.ror(&mut value);
        assert_eq!(value, 0b1000_0000);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn test_branch_instructions() {
        let mut cpu = setup_cpu();
        cpu.program_counter = 0x1000;

        cpu.flags.carry = false;
        cpu.bcc(0x10);
        assert_eq!(cpu.program_counter, 0x1010);

        cpu.flags.carry = true;
        cpu.bcc(0x10); // Should not branch.
        assert_eq!(cpu.program_counter, 0x1010);

        cpu.flags.zero = true;
        cpu.beq(0x20);
        assert_eq!(cpu.program_counter, 0x1030);
    }

    #[test]
    fn test_memory_operations() {
        let mut cpu = setup_cpu();
        cpu.sta(0x2000);
        assert_eq!(cpu.read_memory(0x2000), cpu.accumulator);
    }
}
