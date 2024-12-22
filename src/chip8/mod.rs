mod cpu;
mod gpu;
mod ram;
mod system_bus;
mod timer;

use cpu::Cpu;
use system_bus::SystemBus;

pub(crate) struct Chip8Emulator
{
    cpu: Cpu,
    system_bus: SystemBus,
}

impl Chip8Emulator
{
    pub(crate) fn new() -> Self
    {
        Self { cpu: Cpu::new(), system_bus: SystemBus::new() }
    }

    pub(crate) fn run_cycle(&mut self)
    {
        self.system_bus.delay_timer.decrement();
        self.system_bus.sound_timer.decrement();
        self.cpu.execute_operation(&mut self.system_bus);
    }
}
