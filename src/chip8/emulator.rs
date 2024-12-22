use super::cpu::Cpu;
use super::system_bus::SystemBus;

pub struct Chip8Emulator
{
    cpu: Cpu,
    system_bus: SystemBus,
}

impl Chip8Emulator
{
    pub fn new() -> Self
    {
        Self { cpu: Cpu::new(), system_bus: SystemBus::new() }
    }

    pub fn run_cycle(&mut self)
    {
        self.cpu.execute_operation(&mut self.system_bus);
    }
}
