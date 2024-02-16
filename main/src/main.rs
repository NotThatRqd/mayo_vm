use cpu::{device::{Device, MemoryStick}, instructions::{ADD_REG_REG, MOV_LIT_R1, MOV_LIT_R2}, Cpu};

fn main() {
    let mut m = MemoryStick::<256>::try_new().unwrap();

    //m.write_u8(0, 0xAB).unwrap();
    //m.write_u8(1, 0xCD).unwrap();
    //m.write_u16(0, 0xABCD).unwrap();

    // Move 0x1234 into R1
    m.write_u8(0, MOV_LIT_R1).unwrap();
    m.write_u8(1, 0x12).unwrap();
    m.write_u8(2, 0x34).unwrap();

    // Move 0xABCD into R2
    m.write_u8(3, MOV_LIT_R2).unwrap();
    m.write_u8(4, 0xAB).unwrap();
    m.write_u8(5, 0xCD).unwrap();

    m.write_u8(6, ADD_REG_REG).unwrap();
    m.write_u8(7, 0x02).unwrap(); // R1
    m.write_u8(8, 0x03).unwrap(); // R2

    let bytes: Vec<&u8> = m.iter_u8().collect();
    println!("the memory: {:02X?}", bytes);

    let mut cpu = Cpu::new(m);

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
}
