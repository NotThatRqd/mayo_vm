use std::num::NonZeroU16;

/// A device that can be "plugged in" to the CPU (or other devices that accept devices! ;) )
pub trait Device {
    // The reason read_u8 and read_u16 do not have their returns wrapped
    // in Option's is because that would mean we are sending more than
    // 16/8 bits when this should be a 16 bit machine.

    // todo: fact check
    // write_u8 and write_u16 can return non-zero Result's because the
    // compiler will use the value of zero as the unit Ok variant of the Result,
    // so we're still only sending 16/8 bits but it is a bit more standardized.

    fn read_u8(&self, index: usize) -> u8;
    fn read_u16(&self, index: usize) -> u16;

    /// A return value of [Ok] is a success, otherwise there is a
    /// non-zero error value. This is not an absolute rule and devices
    /// can use the return for whatever they want.
    fn write_u8(&mut self, index: usize, val: u8) -> Result<(), NonZeroU16>;

    /// A return value of [Ok] is a success, otherwise there is a
    /// non-zero error value. This is not an absolute rule and devices
    /// can use the return for whatever they want.
    fn write_u16(&mut self, index: usize, val: u16) -> Result<(), NonZeroU16>;
}

const fn is_even(n: usize) -> bool {
    n % 2 == 0
}

/// The most boring implementation of [MemDevice] that is also functional.
/// Internally, it holds an array of u8s that is `LENGTH` long (`[u8; LENGTH]`).
/// MemoryStick also has [MemoryStick::iter_u8] that iterates over its contents
/// as u8s.
pub struct MemoryStick<const LENGTH: usize> {
    /// The length of this must be even or things WILL BREAK!!
    /// This is going to be interpreted as u16's sometimes and
    /// u16's take up two bytes, so it must have an even number
    /// of bytes or we will have an extra byte with no second part.
    internal_mem: [u8; LENGTH],
}

impl<const LENGTH: usize> MemoryStick<LENGTH> {
    /// The length of the memory this [MemoryStick] holds (in bytes)
    /// must be even because it may be interpreted as u16's which
    /// take up two bytes. It must also be less than or equal to
    /// [u16::MAX]+1 in order to be index-able by a u16.
    ///
    /// See also: [MemoryStick::new_unchecked]
    pub const fn try_new() -> Result<Self, &'static str> {
        if is_even(LENGTH) && LENGTH <= u16::MAX as usize + 1 {
            Ok(Self {
                internal_mem: [0; LENGTH],
            })
        } else {
            Err("length must be even and less than or equal to (u16::MAX)+1")
        }
    }

    /// `LENGTH` must be even and less than [u16::MAX]+1 or else
    /// things will break. See explanations at [MemoryStick::try_new]
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            internal_mem: [0; LENGTH],
        }
    }

    pub fn iter_u8(&self) -> impl Iterator<Item = &u8> {
        self.internal_mem.iter()
    }
}

impl<const LENGTH: usize> Device for MemoryStick<LENGTH> {
    /// Panics on out of bounds
    fn read_u8(&self, offset: usize) -> u8 {
        self.internal_mem[offset]
    }

    /// Panics on out of bounds
    fn read_u16(&self, offset: usize) -> u16 {
        let len = self.internal_mem.len();
        if offset > len - 2 {
            panic!("an offset of {offset} is out of bounds for a u16 (2 bytes long) read on a {len} bytes long array");
        }
        u16::from_be_bytes(self.internal_mem[offset..=offset + 1].try_into().unwrap())
    }

    /// An error of 1 means out of bounds
    fn write_u8(&mut self, offset: usize, val: u8) -> Result<(), NonZeroU16> {
        if offset > self.internal_mem.len() - 1 {
            return Err(NonZeroU16::new(1).unwrap());
        }

        self.internal_mem[offset] = val;
        Ok(())
    }

    /// An error of 1 means out of bounds
    fn write_u16(&mut self, offset: usize, val: u16) -> Result<(), NonZeroU16> {
        if offset > self.internal_mem.len() - 2 {
            return Err(NonZeroU16::new(1).unwrap());
        }

        let bytes = val.to_be_bytes();

        self.internal_mem[offset] = bytes[0];
        self.internal_mem[offset + 1] = bytes[1];

        Ok(())
    }
}