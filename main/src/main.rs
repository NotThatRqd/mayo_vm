use cpu::{device::{Device, MemoryStick}, instructions::*, Cpu};

const IP: u8 = 0;
const ACC: u8 = 1;
const R1: u8 = 2;
const R2: u8 = 3;

fn main() {
    let mut m = MemoryStick::<{256*256}>::try_new().unwrap();

    // Convenience function to write u8s to memory
    let mut index = 0;
    let mut add = |value: u8| {
        m.write_u8(index, value).unwrap();
        index += 1;
    };

    // Move the memory at 0x0100 into register R1
    add(MOV_MEM_REG);
    add(0x01);
    add(0x00);
    add(R1);

    add(MOV_LIT_REG);
    add(0x00);
    add(0x01);
    add(R2);

    add(ADD_REG_REG);
    add(R1);
    add(R2);

    add(MOV_REG_MEM);
    add(ACC);
    add(0x01);
    add(0x00);

    // If the acc != 0x0003 then jump to 0x0000
    add(JMP_NOT_EQ);
    add(0x00);
    add(0x03);
    add(0x00);
    add(0x00);
    
    let mut cpu = Cpu::new(m);

    // panics when it reaches the end of the program
    // todo: make cleaner
    loop {
        cpu.debug();
        cpu.view_memory_at(cpu.get_register(cpu::Register::Ip) as usize, 8);
        cpu.view_memory_at(0x0100, 8);
        println!();
        cpu.step();
    }
}
