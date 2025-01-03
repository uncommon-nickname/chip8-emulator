use super::system_bus::SystemBus;

const NUM_REGISTERS: usize = 16;
pub(super) const START_ADDR: u16 = 0x200;
const STACK_SIZE: usize = 16;
const VF_REGISTER: usize = 0x0F;

pub(super) struct Cpu
{
    /// Cpu registers labeled `V0` to `VF`.
    /// Register `VF` is used to store information about the result of
    /// operations.
    vx: [u8; NUM_REGISTERS],
    /// Program counter register.
    /// Used to store the address of the next instruction that has to
    /// be executed.
    pc: u16,
    /// Index register.
    /// Used to store memory addresses for use in operations.
    i: u16,
    /// A call stack.
    /// Used to keep track of the order of execution when CPU calls a
    /// subroutine. Used to keep the `PC` values to restore the state
    /// of the program, when `RET` call is found.
    stack: Vec<u16>,
}

enum ExecutionFlow
{
    Jump(u16),
    Next,
    SkipNext,
}

impl Cpu
{
    pub(super) fn new() -> Self
    {
        Self { vx: [0; NUM_REGISTERS], pc: START_ADDR, i: 0, stack: Vec::with_capacity(STACK_SIZE) }
    }

    pub(super) fn execute_operation(&mut self, system_bus: &mut SystemBus)
    {
        // The operation code is 16 bit, so we need to read two adjacent
        // `RAM` registers to retrieve the full information.
        let first = (system_bus.ram.read(self.pc) as u16) << 8;
        let second = system_bus.ram.read(self.pc + 1) as u16;
        let opcode = first | second;

        let parsed = ((opcode & 0xF000) >> 12,
                      (opcode & 0x0F00) >> 8, // x
                      (opcode & 0x00F0) >> 4, // y
                      (opcode & 0x000F)); //     n

        let pc_update = match parsed
        {
            // CLS: Clear the display.
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(system_bus),
            // RET: Return from subroutine.
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            // JP: Jump to the address `nnn`.
            (0x01, ..) => self.op_1nnn(opcode & 0x0FFF),
            // CALL: Call the subroutine at address `nnn`.
            (0x02, ..) => self.op_2nnn(opcode & 0x0FFF),
            // SE Vx, kk: Skip if equal.
            (0x03, ..) => self.op_3xkk(parsed.1, opcode & 0x00FF),
            // SNE: Vx, kk: Skip if not equal.
            (0x04, ..) => self.op_4xkk(parsed.1, opcode & 0x00FF),
            // LD Vx, kk: Vx = kk.
            (0x06, ..) => self.op_6xkk(parsed.1, opcode & 0x00FF),
            // ADD Vx, kk: Vx = Vx + kk.
            (0x07, ..) => self.op_7xkk(parsed.1, opcode & 0x00FF),
            // SE Vx, Vy: Skip if equal.
            (0x05, .., 0x00) => self.op_5xy0(parsed.1, parsed.2),
            // LD Vx, Vy: Vx = Vy.
            (0x08, .., 0x00) => self.op_8xy0(parsed.1, parsed.2),
            // OR Vx, Vy: Vx = Vx | Vy.
            (0x08, .., 0x01) => self.op_8xy1(parsed.1, parsed.2),
            // AND Vx, Vy: Vx = Vx & Vy.
            (0x08, .., 0x02) => self.op_8xy2(parsed.1, parsed.2),
            // XOR Vx, Vy: Vx = Vx ^ Vy.
            (0x08, .., 0x03) => self.op_8xy3(parsed.1, parsed.2),
            // ADD Vx, Vy: Vx = Vx + Vy.
            (0x08, .., 0x04) => self.op_8xy4(parsed.1, parsed.2),
            // SUB Vx, Vy: Vx = Vx - Vy.
            (0x08, .., 0x05) => self.op_8xy5(parsed.1, parsed.2),
            // SHR Vx: Vx >> 1.
            (0x08, .., 0x06) => self.op_8x06(parsed.1),
            // SUBN Vx, Vy: Vx = Vy - Vx.
            (0x08, .., 0x07) => self.op_8xy7(parsed.1, parsed.2),
            // SHL Vx: Vx << 1.
            (0x08, .., 0x0E) => self.op_8x0e(parsed.1),
            // SNE Vx, Vy: Skip if not equal.
            (0x09, .., 0x00) => self.op_9xy0(parsed.1, parsed.2),
            // LD I, nnn: I = nnn.
            (0x0A, ..) => self.op_annn(opcode & 0x0FFF),
            // JP V0, nnn: PC = V0 + nnn.
            (0x0B, ..) => self.op_bnnn(opcode & 0x0FFF),
            _ => ExecutionFlow::Next,
        };

        match pc_update
        {
            ExecutionFlow::Jump(addr) => self.pc = addr,
            ExecutionFlow::Next => self.pc += 2,
            ExecutionFlow::SkipNext => self.pc += 4,
        }
    }

    #[inline]
    fn op_00e0(&self, system_bus: &mut SystemBus) -> ExecutionFlow
    {
        system_bus.gpu.clear();
        ExecutionFlow::Next
    }

    #[inline]
    fn op_00ee(&mut self) -> ExecutionFlow
    {
        let addr = self.stack.pop().unwrap();
        ExecutionFlow::Jump(addr)
    }

    #[inline]
    fn op_1nnn(&mut self, nnn: u16) -> ExecutionFlow
    {
        ExecutionFlow::Jump(nnn)
    }

    #[inline]
    fn op_2nnn(&mut self, nnn: u16) -> ExecutionFlow
    {
        self.stack.push(self.pc + 2);
        ExecutionFlow::Jump(nnn)
    }

    #[inline]
    fn op_3xkk(&self, x: u16, kk: u16) -> ExecutionFlow
    {
        if self.vx[x as usize] == kk as u8
        {
            return ExecutionFlow::SkipNext;
        }
        ExecutionFlow::Next
    }

    #[inline]
    fn op_4xkk(&self, x: u16, kk: u16) -> ExecutionFlow
    {
        if self.vx[x as usize] != kk as u8
        {
            return ExecutionFlow::SkipNext;
        }
        ExecutionFlow::Next
    }

    #[inline]
    fn op_6xkk(&mut self, x: u16, kk: u16) -> ExecutionFlow
    {
        self.vx[x as usize] = kk as u8;
        ExecutionFlow::Next
    }

    #[inline]
    fn op_7xkk(&mut self, x: u16, kk: u16) -> ExecutionFlow
    {
        self.vx[x as usize] += kk as u8;
        ExecutionFlow::Next
    }

    #[inline]
    fn op_5xy0(&self, x: u16, y: u16) -> ExecutionFlow
    {
        if self.vx[x as usize] == self.vx[y as usize]
        {
            return ExecutionFlow::SkipNext;
        }
        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy0(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        self.vx[x as usize] = self.vx[y as usize];
        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy1(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        self.vx[x as usize] |= self.vx[y as usize];
        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy2(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        self.vx[x as usize] &= self.vx[y as usize];
        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy3(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        self.vx[x as usize] ^= self.vx[y as usize];
        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy4(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        let result = self.vx[x as usize] as u16 + self.vx[y as usize] as u16;

        self.vx[x as usize] = result as u8;
        self.vx[VF_REGISTER] = (result > u8::MAX as u16) as u8;

        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy5(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        let vx = self.vx[x as usize];
        let vy = self.vx[y as usize];

        self.vx[VF_REGISTER] = (vx > vy) as u8;
        self.vx[x as usize] = vx.wrapping_sub(vy);

        ExecutionFlow::Next
    }

    #[inline]
    fn op_8x06(&mut self, x: u16) -> ExecutionFlow
    {
        self.vx[VF_REGISTER] = self.vx[x as usize] & 1;
        self.vx[x as usize] >>= 1;

        ExecutionFlow::Next
    }

    #[inline]
    fn op_8xy7(&mut self, x: u16, y: u16) -> ExecutionFlow
    {
        let vx = self.vx[x as usize];
        let vy = self.vx[y as usize];

        self.vx[VF_REGISTER] = (vy > vx) as u8;
        self.vx[x as usize] = vy.wrapping_sub(vx);

        ExecutionFlow::Next
    }

    #[inline]
    fn op_8x0e(&mut self, x: u16) -> ExecutionFlow
    {
        self.vx[VF_REGISTER] = (self.vx[x as usize] & 0x80) >> 7;
        self.vx[x as usize] <<= 1;

        ExecutionFlow::Next
    }

    #[inline]
    fn op_9xy0(&self, x: u16, y: u16) -> ExecutionFlow
    {
        if self.vx[x as usize] != self.vx[y as usize]
        {
            return ExecutionFlow::SkipNext;
        }
        ExecutionFlow::Next
    }

    #[inline]
    fn op_annn(&mut self, nnn: u16) -> ExecutionFlow
    {
        self.i = nnn;
        ExecutionFlow::Next
    }

    #[inline]
    fn op_bnnn(&self, nnn: u16) -> ExecutionFlow
    {
        ExecutionFlow::Jump(self.vx[0] as u16 + nnn)
    }
}
