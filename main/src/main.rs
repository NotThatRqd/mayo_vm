use cpu::MemDevice;
use cpu::MemoryStick;

fn main() {
    let mut m = unsafe { MemoryStick::<8>::new_unchecked() };

    //m.write_u8(0, 0xAB).unwrap();
    //m.write_u8(1, 0xCD).unwrap();
    m.write_u16(0, 0xABCD).unwrap();

    let bytes: Vec<&u8> = m.iter_u8().collect();
    println!("the memory: {:02X?}", bytes);

    let bytes_big: Vec<u16> = m.iter_u16().collect();
    println!("the memory big: {:04X?}", bytes_big);
}
