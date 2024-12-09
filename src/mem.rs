pub const MAX_MEM: usize = 1024 * 64;

pub struct Mem {
    pub data: [u8; MAX_MEM],
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            data: [0x00; MAX_MEM],
        }
    }
}
