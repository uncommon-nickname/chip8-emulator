use super::system_bus::SystemBus;

const NUM_REGISTERS: usize = 16;
const START_ADDR: u16 = 0x200;
const STACK_SIZE: usize = 16;

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
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(system_bus),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, ..) => self.op_1nnn(opcode & 0x0FFF),
            (0x02, ..) => self.op_2nnn(opcode & 0x0FFF),
            (0x03, ..) => self.op_3xkk(parsed.1, opcode & 0x00FF),
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
    fn op_1nnn(&mut self, addr: u16) -> ExecutionFlow
    {
        ExecutionFlow::Jump(addr)
    }

    #[inline]
    fn op_2nnn(&mut self, addr: u16) -> ExecutionFlow
    {
        self.stack.push(self.pc + 2);
        ExecutionFlow::Jump(addr)
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
}
