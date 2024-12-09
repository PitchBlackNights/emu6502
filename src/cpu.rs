use crate::mem::Mem;

#[derive(Debug, PartialEq, Eq)]
pub struct CPU {
    pub prg_counter: u16,
    pub stack_ptr: u8,
    pub accum: u8,
    pub x: u8,
    pub y: u8,

    pub carry: bool,
    pub zero: bool,
    pub irq_disable: bool,
    pub decimal_mode: bool,
    pub break_cmd: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            prg_counter: 0x0000,
            stack_ptr: 0x00,
            accum: 0x00,
            x: 0x00,
            y: 0x00,

            carry: false,
            zero: false,
            irq_disable: false,
            decimal_mode: false,
            break_cmd: false,
            overflow: false,
            negative: false,
        }
    }

    //TODO: Make STEP function, that executes the next intruction in Mem, then returns the number of cycles used

    pub fn reset(&mut self, mem: &Mem) {
        //! https://www.pagetable.com/?p=410
        //!
        //! Grab RESET vector from 0xFFFC (Low) & 0xFFFD (High),
        //! then set `prg_counter` to RESET vector and execute.
        //!
        //! `stack_ptr` is also set to 0xFD, no reason why
        self.prg_counter = ((mem.data[0xFFFD] as u16) << 8) + mem.data[0xFFFC] as u16;
        self.stack_ptr = 0xFD;
    }

    pub fn force_reset(&mut self, mem: &mut Mem) {
        self.prg_counter = 0x0000;
        self.stack_ptr = 0xFF;
        self.accum = 0x00;
        self.x = 0x00;
        self.y = 0x00;

        self.carry = false;
        self.zero = false;
        self.irq_disable = false;
        self.decimal_mode = false;
        self.break_cmd = false;
        self.overflow = false;
        self.negative = false;

        mem.data = [0x00; crate::mem::MAX_MEM];
    }

    pub fn print_state(&self) {
        println!(
            "CPU {{\n    prg_counter: 0x{:04X},\n    stack_ptr: 0x{:02X},\n    accum: 0x{:02X},\n    x: 0x{:02X},\n    y: 0x{:02X},\n    carry: {},\n    zero: {},\n    irq_disable: {},\n    decimal_mode: {},\n    break_cmd: {},\n    overflow: {},\n    negative: {}\n}}",
            self.prg_counter,
            self.stack_ptr,
            self.accum,
            self.x,
            self.y,
            self.carry,
            self.zero,
            self.irq_disable,
            self.decimal_mode,
            self.break_cmd,
            self.overflow,
            self.negative,
        )
    }
}
