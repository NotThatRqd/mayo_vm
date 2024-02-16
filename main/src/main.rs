use cpu::{device::{Device, MemoryStick}, instructions::*, Cpu};

const IP: u8 = 0;
const ACC: u8 = 1;
const R1: u8 = 2;
const R2: u8 = 3;

fn main() {
    let mut m = MemoryStick::<{256*256}>::try_new().unwrap();

    // Convenience function to write u8s to memory
    let mut index = 0;
    let mut write_u8_to_memory = |value: u8| {
        m.write_u8(index, value).unwrap();
        index += 1;
    };

    // Move 0x1234 into R1
    write_u8_to_memory(MOV_LIT_REG);
    write_u8_to_memory(0x12);
    write_u8_to_memory(0x34);
    write_u8_to_memory(R1);

    // Move 0xABCD into R2
    write_u8_to_memory(MOV_LIT_REG);
    write_u8_to_memory(0xAB);
    write_u8_to_memory(0xCD);
    write_u8_to_memory(R2);

    // Add R1 and R2
    write_u8_to_memory(ADD_REG_REG);
    write_u8_to_memory(R1);
    write_u8_to_memory(R2);

    // Move the value in the Accumulator to 0x0100 in memory
    write_u8_to_memory(MOV_REG_MEM);
    write_u8_to_memory(ACC);
    write_u8_to_memory(0x01);
    write_u8_to_memory(0x00);
    
    let mut cpu = Cpu::new(m);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(cpu.get_register(cpu::Register::Ip) as usize, 8);
    cpu.view_memory_at(0x0100, 8);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(cpu.get_register(cpu::Register::Ip) as usize, 8);
    cpu.view_memory_at(0x0100, 8);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(cpu.get_register(cpu::Register::Ip) as usize, 8);
    cpu.view_memory_at(0x0100, 8);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(cpu.get_register(cpu::Register::Ip) as usize, 8);
    cpu.view_memory_at(0x0100, 8);
}
