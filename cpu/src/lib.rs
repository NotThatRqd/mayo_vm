use device::{Device, MemoryStick};
use instructions::{ADD_REG_REG, MOV_LIT_R1, MOV_LIT_R2};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod device;
pub mod instructions;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
enum Register {
    Ip = 0,
    Acc,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Register {
    fn as_byte_offset(&self) -> usize {
        // multiplied by two because registers are two bytes big
        *self as usize * 2
    }
}

pub struct Cpu<T: Device> {
    memory: T,
    // ten registers * 2 bytes per
    registers: MemoryStick<{10 * 2}>
}

impl<T: Device> Cpu<T> {
    pub fn new(memory: T) -> Self {
        Cpu {
            memory,
            registers: MemoryStick::try_new().unwrap()
        }
    }

    pub fn debug(&self) {
        for reg in Register::iter() {
            let value = self.get_register(reg);
            println!("{reg:?}: 0x{value:04X?}");
        }
        println!();
    }

    fn get_register(&self, reg: Register) -> u16 {
        self.registers.read_u16(reg.as_byte_offset())
    }

    fn set_register(&mut self, reg: Register, val: u16) {
        self.registers.write_u16(reg.as_byte_offset(), val).unwrap();
    }

    /// Fetches the next instruction that the IP is pointing to and increases it by 1
    fn fetch(&mut self) -> u8 {
        let next_instruction_address = self.get_register(Register::Ip);
        let next_instruction = self.memory.read_u8(next_instruction_address as usize);

        // increase instruction pointer
        self.set_register(Register::Ip, next_instruction_address + 1);

        next_instruction
    }

    /// Fetches the next u16 that the IP is pointing to and increases it by 2
    fn fetch16(&mut self) -> u16 {
        let next_instruction_address = self.get_register(Register::Ip);
        let next_instruction = self.memory.read_u16(next_instruction_address as usize);

        // increase instruction pointer
        self.set_register(Register::Ip, next_instruction_address + 2);

        next_instruction
    }

    /// This function will map 0 to the IP, 1 to the Acc, 2 to R1, 3 to R2, etc.
    fn fetch_register_offset(&mut self) -> usize {
        // multiplied by two because registers are two bytes long
        self.fetch() as usize * 2
    }

    fn execute(&mut self, instruction: u8) {
        match instruction {
            MOV_LIT_R1 => {
                let literal = self.fetch16();
                self.set_register(Register::R1, literal);
            }
            MOV_LIT_R2 => {
                let literal = self.fetch16();
                self.set_register(Register::R2, literal);
            }
            ADD_REG_REG => {
                let first_register_offset = self.fetch_register_offset();
                let second_register_offset = self.fetch_register_offset();

                let register_value_1 = self.registers.read_u16(first_register_offset);
                let register_value_2 = self.registers.read_u16(second_register_offset);

                self.set_register(Register::Acc, register_value_1 + register_value_2);
            }
            _ => panic!("unknown instruction {instruction:02X?}")
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }
}