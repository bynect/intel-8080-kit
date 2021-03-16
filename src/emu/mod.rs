const CYCLES: [usize; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5,
    7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 13, 5, 10, 10, 10, 4, 4, 10,
    13, 5, 5, 5, 7, 4, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5,
    5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 7, 7, 7, 7, 7, 7, 7, 7, 5,
    5, 5, 5, 5, 5, 7, 5, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4,
    4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4,
    4, 4, 4, 4, 4, 7, 4, 5, 10, 10, 10, 11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11, 5, 10, 10, 10,
    11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11, 5, 10, 10, 18, 11, 11, 7, 11, 5, 5, 10, 4, 11, 17,
    7, 11, 5, 10, 10, 4, 11, 11, 7, 11, 5, 5, 10, 4, 11, 17, 7, 11,
];

pub trait Memory {
    fn read_byte(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16;

    fn write_byte(&mut self, addr: u16, byte: u8);
    fn write_word(&mut self, addr: u16, word: u16);

    fn in_port(&self, port: u8) -> u8;
    fn out_port(&self, port: u8, byte: u8);
}

pub struct Emulator {
    /// Memory
    mem: Box<dyn Memory>,
    /// Halted
    halt: bool,
    /// Cycles count
    cycles: usize,
    /// Program counter
    pc: Word,
    /// Stack pointer
    sp: Word,
    /// Sign flag
    s_flag: Flag,
    /// Zero flag
    z_flag: Flag,
    /// Half-carry flag
    h_flag: Flag,
    /// Parity flag
    p_flag: Flag,
    /// Carry flag
    c_flag: Flag,
    a_reg: Register,
    b_reg: Register,
    c_reg: Register,
    d_reg: Register,
    e_reg: Register,
    h_reg: Register,
    l_reg: Register,
    /// Interrupts information
    int: InterruptInfo,
}

#[derive(Debug, Default)]
struct Register(pub Byte);

#[derive(Debug, Default)]
struct Flag(pub bool);

#[derive(Debug, Default)]
struct InterruptInfo {
    pub pending: bool,
    pub filp_flop: bool,
    pub vector: Byte,
    pub delay: Byte,
}

#[derive(Debug, Default, Clone, Copy)]
struct Word(pub u16);

#[derive(Debug, Default, Clone, Copy)]
struct Byte(pub u8);

impl Byte {
    pub fn bit_parity(&self) -> bool {
        let mut ones = 0;

        for i in 0..8 {
            ones += (self.0 >> i) & i;
        }

        (ones & 1) == 0
    }

    pub fn bit_carry(&self, other: Byte, alredy: bool, bits: i32) -> bool {
        let res = (self.0 as u16) + (other.0 as u16) + (alredy as u16);
        let carry = res ^ (self.0 as u16) ^ (other.0 as u16);

        carry & (1 << bits) != 0
    }
}

impl Emulator {
    pub fn new(mem: Box<dyn Memory>) -> Self {
        Self {
            mem,
            halt: false,
            cycles: 0,
            pc: Word::default(),
            sp: Word::default(),
            s_flag: Flag::default(),
            z_flag: Flag::default(),
            h_flag: Flag::default(),
            p_flag: Flag::default(),
            c_flag: Flag::default(),
            a_reg: Register::default(),
            b_reg: Register::default(),
            c_reg: Register::default(),
            d_reg: Register::default(),
            e_reg: Register::default(),
            h_reg: Register::default(),
            l_reg: Register::default(),
            int: InterruptInfo::default(),
        }
    }

    fn set_flags(&mut self, value: Byte) {
        self.z_flag.0 = value.0 == 0u8;
        self.s_flag.0 = (value.0 >> 7) == 1;
        self.p_flag.0 = value.bit_parity();
    }

    fn fetch_next_byte(&mut self) -> u8 {
        let byte = self.mem.read_byte(self.pc.0);
        self.pc.0 += 1;
        byte
    }

    fn fetch_next_word(&mut self) -> u16 {
        let word = self.mem.read_word(self.pc.0);
        self.pc.0 += 2;
        word
    }

    fn set_bc_pair(&mut self, value: u16) {
        self.b_reg.0 = Byte((value >> 8) as u8);
        self.c_reg.0 = Byte((value & 0xff) as u8);
    }

    fn get_bc_pair(&self) -> u16 {
        (self.b_reg.0 .0 as u16) << 8 | self.c_reg.0 .0 as u16
    }

    fn set_de_pair(&mut self, value: u16) {
        self.d_reg.0 = Byte((value >> 8) as u8);
        self.e_reg.0 = Byte((value & 0xff) as u8);
    }

    fn get_de_pair(&self) -> u16 {
        (self.d_reg.0 .0 as u16) << 8 | self.e_reg.0 .0 as u16
    }

    fn set_hl_pair(&mut self, value: u16) {
        self.l_reg.0 = Byte((value >> 8) as u8);
        self.h_reg.0 = Byte((value & 0xff) as u8);
    }

    fn get_hl_pair(&self) -> u16 {
        (self.h_reg.0 .0 as u16) << 8 | self.l_reg.0 .0 as u16
    }

    fn push_stack(&mut self, value: u16) {
        self.sp.0 -= 2;
        self.mem.write_word(self.sp.0, value);
    }

    fn pop_stack(&mut self) -> u16 {
        let value = self.mem.read_word(self.sp.0);
        self.sp.0 += 2;
        value
    }

    fn op_inr(&mut self, value: Byte) -> Byte {
        let res = Byte(value.0 + 1);
        self.h_flag.0 = (res.0 & 0x0f) == 0;
        self.set_flags(res);
        res
    }

    fn op_dcr(&mut self, value: Byte) -> Byte {
        let res = Byte(value.0 - 1);
        self.h_flag.0 = (res.0 & 0x0f) != 0x0f;
        self.set_flags(res);
        res
    }

    fn op_dad(&mut self, value: u16) {
        self.c_flag.0 = ((((self.get_hl_pair() + value) as u32) >> 16) & 1) != 0;
        self.set_hl_pair(self.get_hl_pair() + value);
    }

    fn op_add(&mut self, reg: Byte, value: Byte, alredy: bool) -> Byte {
        let res = Byte(reg.0 + value.0 + (alredy as u8));
        let byte = Byte(reg.0);
        self.c_flag.0 = byte.bit_carry(value, alredy, 8);
        self.h_flag.0 = byte.bit_carry(value, alredy, 4);

        self.set_flags(res);
        res
    }

    fn op_sub(&mut self, reg: Byte, value: Byte, alredy: bool) -> Byte {
        let res = self.op_add(reg, Byte(!value.0), !alredy);
        self.c_flag.0 = !self.c_flag.0;
        res
    }

    fn op_daa(&mut self) {
        let mut alredy = self.c_flag.0;
        let mut adj = 0;
        let lsb = self.a_reg.0 .0 & 0x0f;
        let msb = self.a_reg.0 .0 >> 4;

        if self.h_flag.0 || lsb > 9 {
            adj += 0x06;
        }

        if self.c_flag.0 || msb > 9 || (msb >= 9 && lsb > 9) {
            adj += 0x60;
            alredy = true;
        }

        self.a_reg.0 = self.op_add(self.a_reg.0, Byte(adj), alredy);
        self.c_flag.0 = alredy;
    }

    fn op_ana(&mut self, value: u8) {
        let res = Byte(self.a_reg.0 .0 & value);
        self.c_flag.0 = false;
        self.h_flag.0 = ((self.a_reg.0 .0 | value) & 0x08) != 0;

        self.set_flags(res);
        self.a_reg.0 = res;
    }

    fn op_xra(&mut self, value: u8) {
        self.a_reg.0 .0 ^= value;
        self.c_flag.0 = false;
        self.h_flag.0 = false;
        self.set_flags(self.a_reg.0);
    }

    fn op_ora(&mut self, value: u8) {
        self.a_reg.0 .0 |= value;
        self.c_flag.0 = false;
        self.h_flag.0 = false;
        self.set_flags(self.a_reg.0);
    }

    fn op_cmp(&mut self, value: u8) {
        let res = self.a_reg.0 .0 - value;
        self.c_flag.0 = ((res as u16) >> 8) != 0;
        self.h_flag.0 = (!(self.a_reg.0 .0 ^ res ^ value) & 0x10) != 0;
        self.set_flags(Byte(res & 0xff));
    }

    fn op_jmp(&mut self, addr: u16) {
        self.pc.0 = addr;
    }

    fn op_cond_jmp(&mut self, cond: bool) {
        let addr = self.fetch_next_word();

        if cond {
            self.op_jmp(addr);
        }
    }

    fn op_call(&mut self, addr: u16) {
        self.push_stack(self.pc.0);
        self.op_jmp(addr);
    }

    fn op_cond_call(&mut self, cond: bool) {
        let addr = self.fetch_next_word();

        if cond {
            self.op_call(addr);
            self.cycles += 6;
        }
    }

    fn op_ret(&mut self) {
        self.pc.0 = self.pop_stack();
    }

    fn op_cond_ret(&mut self, cond: bool) {
        if cond {
            self.op_ret();
            self.cycles += 6;
        }
    }

    pub fn exec(&mut self, op: u8) {
        self.cycles += CYCLES[op as usize];

        if self.int.delay.0 > 0 {
            self.int.delay.0 -= 1;
        }

        match op {
            0x00 => {}
            0x01 => {
                let word = self.fetch_next_word();
                self.set_bc_pair(word);
            }
            0x02 => {
                let value = self.get_bc_pair();
                self.mem.write_byte(value, self.a_reg.0 .0);
            }
            0x03 => {
                let value = self.get_bc_pair();
                self.set_bc_pair(value + 1);
            }
            0x04 => {
                self.b_reg.0 = self.op_inr(self.b_reg.0);
            }
            0x05 => {
                self.b_reg.0 = self.op_dcr(self.b_reg.0);
            }
            0x06 => {
                self.b_reg.0 = Byte(self.fetch_next_byte());
            }
            0x07 => {
                self.c_flag.0 = (self.a_reg.0 .0 >> 7) != 0;
                self.a_reg.0 .0 = (self.a_reg.0 .0 << 1) | self.c_flag.0 as u8;
            }
            0x09 => {
                self.op_dad(self.get_bc_pair());
            }
            0x0a => {
                self.a_reg.0 .0 = self.mem.read_byte(self.get_bc_pair());
            }
            0x0b => {
                let value = self.get_bc_pair();
                self.set_bc_pair(value - 1);
            }
            0x0c => {
                self.c_reg.0 = self.op_inr(self.c_reg.0);
            }
            0x0d => {
                self.c_reg.0 = self.op_dcr(self.c_reg.0);
            }
            0x0e => {
                self.c_reg.0 = Byte(self.fetch_next_byte());
            }
            0x0f => {
                self.c_flag.0 = (self.a_reg.0 .0 & 1) != 0;
                self.a_reg.0 .0 = (self.a_reg.0 .0 >> 1) | ((self.c_flag.0 as u8) << 7)
            }
            0x11 => {
                let word = self.fetch_next_word();
                self.set_de_pair(word);
            }
            0x12 => {
                let addr = self.get_de_pair();
                self.mem.write_byte(addr, self.a_reg.0 .0);
            }
            0x13 => {
                let value = self.get_de_pair();
                self.set_de_pair(value + 1);
            }
            0x14 => {
                self.d_reg.0 = self.op_inr(self.d_reg.0);
            }
            0x15 => {
                self.d_reg.0 = self.op_dcr(self.d_reg.0);
            }
            0x16 => {
                self.d_reg.0 = Byte(self.fetch_next_byte());
            }
            0x17 => {
                let alredy = self.c_flag.0;
                self.c_flag.0 = (self.a_reg.0 .0 >> 7) != 0;
                self.a_reg.0 .0 = (self.a_reg.0 .0 << 1) | (alredy as u8);
            }
            0x19 => {
                self.op_dad(self.get_de_pair());
            }
            0x1a => {
                self.a_reg.0 .0 = self.mem.read_byte(self.get_de_pair());
            }
            0x1b => {
                let value = self.get_de_pair();
                self.set_de_pair(value - 1);
            }
            0x1c => {
                self.e_reg.0 = self.op_inr(self.e_reg.0);
            }
            0x1d => {
                self.e_reg.0 = self.op_dcr(self.e_reg.0);
            }
            0x1e => {
                self.e_reg.0 = Byte(self.fetch_next_byte());
            }
            0x1f => {
                let alredy = self.c_flag.0;
                self.c_flag.0 = (self.a_reg.0 .0 & 1) != 0;
                self.a_reg.0 .0 = (self.a_reg.0 .0 >> 1) | ((alredy as u8) << 7);
            }
            0x21 => {
                let word = self.fetch_next_word();
                self.set_hl_pair(word);
            }
            0x22 => {
                let addr = self.fetch_next_word();
                self.mem.write_word(addr, self.get_hl_pair());
            }
            0x23 => {
                let value = self.get_hl_pair();
                self.set_hl_pair(value + 1);
            }
            0x24 => {
                self.h_reg.0 = self.op_inr(self.h_reg.0);
            }
            0x25 => {
                self.h_reg.0 = self.op_dcr(self.h_reg.0);
            }
            0x26 => {
                self.h_reg.0 = Byte(self.fetch_next_byte());
            }
            0x27 => {
                self.op_daa();
            }
            0x29 => {
                self.op_dad(self.get_hl_pair());
            }
            0x2a => {
                let addr = self.fetch_next_word();
                self.set_hl_pair(self.mem.read_word(addr))
            }
            0x2b => {
                let value = self.get_hl_pair();
                self.set_hl_pair(value - 1);
            }
            0x2c => {
                self.l_reg.0 = self.op_inr(self.l_reg.0);
            }
            0x2d => {
                self.l_reg.0 = self.op_dcr(self.l_reg.0);
            }
            0x2e => {
                self.l_reg.0 = Byte(self.fetch_next_byte());
            }
            0x2f => {
                self.a_reg.0 .0 = !self.a_reg.0 .0;
            }
            0x31 => {
                let word = self.fetch_next_word();
                self.sp = Word(word);
            }
            0x32 => {
                let addr = self.fetch_next_word();
                self.mem.write_byte(addr, self.a_reg.0 .0);
            }
            0x33 => {
                self.sp.0 += 1;
            }
            0x34 => {
                let byte = self.mem.read_byte(self.get_hl_pair());
                let value = self.op_inr(Byte(byte));
                self.mem.write_byte(self.get_hl_pair(), value.0);
            }
            0x35 => {
                let byte = self.mem.read_byte(self.get_hl_pair());
                let value = self.op_dcr(Byte(byte));
                self.mem.write_byte(self.get_hl_pair(), value.0);
            }
            0x36 => {
                let value = self.fetch_next_byte();
                self.mem.write_byte(self.get_hl_pair(), value);
            }
            0x37 => {
                self.c_flag.0 = true;
            }
            0x39 => {
                self.op_dad(self.sp.0);
            }
            0x3a => {
                let addr = self.fetch_next_word();
                self.a_reg.0 .0 = self.mem.read_byte(addr);
            }
            0x3b => {
                self.sp.0 -= 1;
            }
            0x3c => {
                self.a_reg.0 = self.op_inr(self.a_reg.0);
            }
            0x3d => {
                self.a_reg.0 = self.op_dcr(self.a_reg.0);
            }
            0x3e => {
                self.a_reg.0 = Byte(self.fetch_next_byte());
            }
            0x3f => {
                self.c_flag.0 = !self.c_flag.0;
            }
            0x40 => {
                self.b_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x41 => {
                self.b_reg.0 = self.c_reg.0;
            }
            0x42 => {
                self.b_reg.0 = self.d_reg.0;
            }
            0x43 => {
                self.b_reg.0 = self.e_reg.0;
            }
            0x44 => {
                self.b_reg.0 = self.h_reg.0;
            }
            0x45 => {
                self.b_reg.0 = self.l_reg.0;
            }
            0x46 => {
                self.b_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x47 => {
                self.b_reg.0 = self.a_reg.0;
            }
            0x48 => {
                self.c_reg.0 = self.b_reg.0;
            }
            0x49 => {
                self.c_reg.0 = self.c_reg.0;
            }
            0x4a => {
                self.c_reg.0 = self.d_reg.0;
            }
            0x4b => {
                self.c_reg.0 = self.e_reg.0;
            }
            0x4c => {
                self.c_reg.0 = self.h_reg.0;
            }
            0x4d => {
                self.c_reg.0 = self.l_reg.0;
            }
            0x4e => {
                self.c_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x4f => {
                self.c_reg.0 = self.a_reg.0;
            }
            0x50 => {
                self.d_reg.0 = self.b_reg.0;
            }
            0x51 => {
                self.d_reg.0 = self.c_reg.0;
            }
            0x52 => {
                self.d_reg.0 = self.d_reg.0;
            }
            0x53 => {
                self.d_reg.0 = self.e_reg.0;
            }
            0x54 => {
                self.d_reg.0 = self.h_reg.0;
            }
            0x55 => {
                self.d_reg.0 = self.l_reg.0;
            }
            0x56 => {
                self.d_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x57 => {
                self.d_reg.0 = self.a_reg.0;
            }
            0x58 => {
                self.e_reg.0 = self.b_reg.0;
            }
            0x59 => {
                self.e_reg.0 = self.c_reg.0;
            }
            0x5a => {
                self.e_reg.0 = self.d_reg.0;
            }
            0x5b => {
                self.e_reg.0 = self.e_reg.0;
            }
            0x5c => {
                self.e_reg.0 = self.h_reg.0;
            }
            0x5d => {
                self.e_reg.0 = self.l_reg.0;
            }
            0x5e => {
                self.e_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x5f => {
                self.e_reg.0 = self.a_reg.0;
            }
            0x60 => {
                self.h_reg.0 = self.b_reg.0;
            }
            0x61 => {
                self.h_reg.0 = self.c_reg.0;
            }
            0x62 => {
                self.h_reg.0 = self.d_reg.0;
            }
            0x63 => {
                self.h_reg.0 = self.e_reg.0;
            }
            0x64 => {
                self.h_reg.0 = self.h_reg.0;
            }
            0x65 => {
                self.h_reg.0 = self.l_reg.0;
            }
            0x66 => {
                self.h_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x67 => {
                self.h_reg.0 = self.a_reg.0;
            }
            0x68 => {
                self.l_reg.0 = self.b_reg.0;
            }
            0x69 => {
                self.l_reg.0 = self.c_reg.0;
            }
            0x6a => {
                self.l_reg.0 = self.d_reg.0;
            }
            0x6b => {
                self.l_reg.0 = self.e_reg.0;
            }
            0x6c => {
                self.l_reg.0 = self.h_reg.0;
            }
            0x6d => {
                self.l_reg.0 = self.l_reg.0;
            }
            0x6e => {
                self.l_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x6f => {
                self.l_reg.0 = self.a_reg.0;
            }
            0x70 => {
                self.mem.write_byte(self.get_hl_pair(), self.b_reg.0 .0);
            }
            0x71 => {
                self.mem.write_byte(self.get_hl_pair(), self.c_reg.0 .0);
            }
            0x72 => {
                self.mem.write_byte(self.get_hl_pair(), self.d_reg.0 .0);
            }
            0x73 => {
                self.mem.write_byte(self.get_hl_pair(), self.e_reg.0 .0);
            }
            0x74 => {
                self.mem.write_byte(self.get_hl_pair(), self.h_reg.0 .0);
            }
            0x75 => {
                self.mem.write_byte(self.get_hl_pair(), self.l_reg.0 .0);
            }
            0x76 => {
                self.halt = true;
            }
            0x77 => {
                self.mem.write_byte(self.get_hl_pair(), self.a_reg.0 .0);
            }
            0x78 => {
                self.a_reg.0 = self.b_reg.0;
            }
            0x79 => {
                self.a_reg.0 = self.c_reg.0;
            }
            0x7a => {
                self.a_reg.0 = self.d_reg.0;
            }
            0x7b => {
                self.a_reg.0 = self.e_reg.0;
            }
            0x7c => {
                self.a_reg.0 = self.h_reg.0;
            }
            0x7d => {
                self.a_reg.0 = self.l_reg.0;
            }
            0x7e => {
                self.a_reg.0 .0 = self.mem.read_byte(self.get_hl_pair());
            }
            0x7f => {
                self.a_reg.0 = self.a_reg.0;
            }
            0x80 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.b_reg.0, false);
            }
            0x81 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.c_reg.0, false);
            }
            0x82 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.d_reg.0, false);
            }
            0x83 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.e_reg.0, false);
            }
            0x84 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.h_reg.0, false);
            }
            0x85 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.l_reg.0, false);
            }
            0x86 => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.a_reg.0 = self.op_add(self.a_reg.0, Byte(value), false);
            }
            0x87 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.a_reg.0, false);
            }
            0x88 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.b_reg.0, self.c_flag.0);
            }
            0x89 => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.c_reg.0, self.c_flag.0);
            }
            0x8a => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.d_reg.0, self.c_flag.0);
            }
            0x8b => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.e_reg.0, self.c_flag.0);
            }
            0x8c => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.h_reg.0, self.c_flag.0);
            }
            0x8d => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.l_reg.0, self.c_flag.0);
            }
            0x8e => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.a_reg.0 = self.op_add(self.a_reg.0, Byte(value), self.c_flag.0);
            }
            0x8f => {
                self.a_reg.0 = self.op_add(self.a_reg.0, self.a_reg.0, self.c_flag.0);
            }
            0x90 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.b_reg.0, false);
            }
            0x91 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.c_reg.0, false);
            }
            0x92 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.d_reg.0, false);
            }
            0x93 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.e_reg.0, false);
            }
            0x94 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.h_reg.0, false);
            }
            0x95 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.l_reg.0, false);
            }
            0x96 => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.a_reg.0 = self.op_sub(self.a_reg.0, Byte(value), false);
            }
            0x97 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.a_reg.0, false);
            }
            0x98 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.b_reg.0, self.c_flag.0);
            }
            0x99 => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.c_reg.0, self.c_flag.0);
            }
            0x9a => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.d_reg.0, self.c_flag.0);
            }
            0x9b => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.e_reg.0, self.c_flag.0);
            }
            0x9c => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.h_reg.0, self.c_flag.0);
            }
            0x9d => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.l_reg.0, self.c_flag.0);
            }
            0x9e => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.a_reg.0 = self.op_sub(self.a_reg.0, Byte(value), self.c_flag.0);
            }
            0x9f => {
                self.a_reg.0 = self.op_sub(self.a_reg.0, self.a_reg.0, self.c_flag.0);
            }
            0xa0 => {
                self.op_ana(self.b_reg.0 .0);
            }
            0xa1 => {
                self.op_ana(self.c_reg.0 .0);
            }
            0xa2 => {
                self.op_ana(self.d_reg.0 .0);
            }
            0xa3 => {
                self.op_ana(self.e_reg.0 .0);
            }
            0xa4 => {
                self.op_ana(self.h_reg.0 .0);
            }
            0xa5 => {
                self.op_ana(self.l_reg.0 .0);
            }
            0xa6 => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.op_ana(value);
            }
            0xa7 => {
                self.op_ana(self.a_reg.0 .0);
            }
            0xa8 => {
                self.op_xra(self.b_reg.0 .0);
            }
            0xa9 => {
                self.op_xra(self.c_reg.0 .0);
            }
            0xaa => {
                self.op_xra(self.d_reg.0 .0);
            }
            0xab => {
                self.op_xra(self.e_reg.0 .0);
            }
            0xac => {
                self.op_xra(self.h_reg.0 .0);
            }
            0xad => {
                self.op_xra(self.l_reg.0 .0);
            }
            0xae => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.op_xra(value);
            }
            0xaf => {
                self.op_xra(self.a_reg.0 .0);
            }
            0xb0 => {
                self.op_ora(self.b_reg.0 .0);
            }
            0xb1 => {
                self.op_ora(self.c_reg.0 .0);
            }
            0xb2 => {
                self.op_ora(self.d_reg.0 .0);
            }
            0xb3 => {
                self.op_ora(self.e_reg.0 .0);
            }
            0xb4 => {
                self.op_ora(self.h_reg.0 .0);
            }
            0xb5 => {
                self.op_ora(self.l_reg.0 .0);
            }
            0xb6 => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.op_ora(value);
            }
            0xb7 => {
                self.op_ora(self.a_reg.0 .0);
            }
            0xb8 => {
                self.op_cmp(self.b_reg.0 .0);
            }
            0xb9 => {
                self.op_cmp(self.c_reg.0 .0);
            }
            0xba => {
                self.op_cmp(self.d_reg.0 .0);
            }
            0xbb => {
                self.op_cmp(self.e_reg.0 .0);
            }
            0xbc => {
                self.op_cmp(self.h_reg.0 .0);
            }
            0xbd => {
                self.op_cmp(self.l_reg.0 .0);
            }
            0xbe => {
                let value = self.mem.read_byte(self.get_hl_pair());
                self.op_cmp(value);
            }
            0xbf => {
                self.op_cmp(self.a_reg.0 .0);
            }
            0xc0 => {
                self.op_cond_ret(!self.z_flag.0);
            }
            0xc1 => {
                let value = self.pop_stack();
                self.set_bc_pair(value)
            }
            0xc2 => {
                self.op_cond_jmp(!self.z_flag.0);
            }
            0xc3 => {
                let addr = self.fetch_next_word();
                self.op_jmp(addr);
            }
            0xc4 => {
                self.op_cond_call(!self.z_flag.0);
            }
            0xc5 => {
                self.push_stack(self.get_bc_pair());
            }
            0xc6 => {
                let value = self.fetch_next_byte();
                self.a_reg.0 = self.op_add(self.a_reg.0, Byte(value), false);
            }
            0xc7 => {
                self.op_call(0x00);
            }
            0xc8 => {
                self.op_cond_ret(self.z_flag.0);
            }
            0xc9 => {
                self.op_ret();
            }
            0xca => {
                self.op_cond_jmp(self.z_flag.0);
            }
            0xcc => {
                self.op_cond_call(self.z_flag.0);
            }
            0xcd => {
                let addr = self.fetch_next_word();
                self.op_call(addr);
            }
            0xce => {
                let value = self.fetch_next_byte();
                self.a_reg.0 = self.op_add(self.a_reg.0, Byte(value), self.c_flag.0);
            }
            0xcf => {
                self.op_call(0x08);
            }
            0xd0 => {
                self.op_cond_ret(!self.c_flag.0);
            }
            0xd1 => {
                let value = self.pop_stack();
                self.set_de_pair(value)
            }
            0xd2 => {
                self.op_cond_jmp(!self.c_flag.0);
            }
            0xd3 => {
                let port = self.fetch_next_byte();
                self.mem.out_port(port, self.a_reg.0 .0);
            }
            0xd4 => {
                self.op_cond_call(!self.c_flag.0);
            }
            0xd5 => {
                self.push_stack(self.get_de_pair());
            }
            0xd6 => {
                let value = self.fetch_next_byte();
                self.a_reg.0 = self.op_sub(self.a_reg.0, Byte(value), false);
            }
            0xd7 => {
                self.op_call(0x10);
            }
            0xd8 => {
                self.op_cond_ret(self.c_flag.0);
            }
            0xda => {
                self.op_cond_jmp(self.c_flag.0);
            }
            0xdb => {
                let port = self.fetch_next_byte();
                self.a_reg.0 .0 = self.mem.in_port(port);
            }
            0xdc => {
                self.op_cond_call(self.c_flag.0);
            }
            0xde => {
                let value = self.fetch_next_byte();
                self.a_reg.0 = self.op_sub(self.a_reg.0, Byte(value), self.c_flag.0);
            }
            0xdf => {
                self.op_call(0x18);
            }
            0xe0 => {
                self.op_cond_ret(!self.p_flag.0);
            }
            0xe1 => {
                let value = self.pop_stack();
                self.set_hl_pair(value)
            }
            0xe2 => {
                self.op_cond_jmp(!self.p_flag.0);
            }
            0xe3 => {
                let value = self.mem.read_word(self.sp.0);
                self.mem.write_word(self.sp.0, self.get_hl_pair());
                self.set_hl_pair(value);
            }
            0xe4 => {
                self.op_cond_call(!self.p_flag.0);
            }
            0xe5 => {
                self.push_stack(self.get_hl_pair());
            }
            0xe6 => {
                let value = self.fetch_next_byte();
                self.op_ana(value);
            }
            0xe7 => {
                self.op_call(0x20);
            }
            0xe8 => {
                self.op_cond_ret(self.p_flag.0);
            }
            0xe9 => {
                self.pc.0 = self.get_hl_pair();
            }
            0xea => {
                self.op_cond_jmp(self.p_flag.0);
            }
            0xeb => {
                let value = self.get_de_pair();
                self.set_de_pair(self.get_hl_pair());
                self.set_hl_pair(value);
            }
            0xec => {
                self.op_cond_call(self.p_flag.0);
            }
            0xee => {
                let value = self.fetch_next_byte();
                self.op_xra(value);
            }
            0xef => {
                self.op_call(0x28);
            }
            0xf0 => {
                self.op_cond_ret(!self.s_flag.0);
            }
            0xf1 => {
                let value = self.pop_stack();
                self.a_reg.0 .0 = (value >> 8) as u8;

                let psw = (value & 0xff) as u8;
                self.s_flag.0 = ((psw >> 7) & 1) != 0;
                self.z_flag.0 = ((psw >> 6) & 1) != 0;
                self.h_flag.0 = ((psw >> 4) & 1) != 0;
                self.p_flag.0 = ((psw >> 2) & 1) != 0;
                self.c_flag.0 = ((psw >> 0) & 1) != 0;
            }
            0xf2 => {
                self.op_cond_jmp(!self.s_flag.0);
            }
            0xf3 => {
                self.int.filp_flop = true;
            }
            0xf4 => {
                self.op_cond_call(!self.s_flag.0);
            }
            0xf5 => {
                let mut psw: u8 = 0;
                psw |= (self.s_flag.0 as u8) << 7;
                psw |= (self.z_flag.0 as u8) << 6;
                psw |= (self.h_flag.0 as u8) << 4;
                psw |= (self.p_flag.0 as u8) << 2;
                psw |= 1 << 1;
                psw |= (self.c_flag.0 as u8) << 0;

                self.push_stack((self.a_reg.0 .0 as u16) << 8 | (psw as u16));
            }
            0xf6 => {
                let value = self.fetch_next_byte();
                self.op_ora(value);
            }
            0xf7 => {
                self.op_call(0x30);
            }
            0xf8 => {
                self.op_cond_ret(self.s_flag.0);
            }
            0xf9 => {
                self.sp.0 = self.get_hl_pair();
            }
            0xfa => {
                self.op_cond_jmp(self.s_flag.0);
            }
            0xfb => {
                self.int.filp_flop = false;
                self.int.delay.0 = 1;
            }
            0xfc => {
                self.op_cond_call(self.s_flag.0);
            }
            0xfe => {
                let value = self.fetch_next_byte();
                self.op_cmp(value);
            }
            0xff => {
                self.op_call(0x38);
            }

            // Undocumented ops
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {}
            0xd9 => {
                self.op_ret();
            }
            0xdd | 0xed | 0xfd => {
                let addr = self.fetch_next_word();
                self.op_call(addr);
            }
            0xcb => {
                let addr = self.fetch_next_word();
                self.op_jmp(addr);
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            let op = self.fetch_next_byte();
            self.exec(op);
        }
    }
}
