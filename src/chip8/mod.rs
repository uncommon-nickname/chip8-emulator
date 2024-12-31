mod cpu;
mod gpu;
mod ram;
mod system_bus;
mod timer;

use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use cpu::{Cpu, START_ADDR};
use system_bus::SystemBus;

pub(crate) struct Chip8Emulator
{
    cpu: Cpu,
    system_bus: SystemBus,
    timer_hz: usize,
    cpu_hz: usize,
}

impl Chip8Emulator
{
    pub(crate) fn new(timer_hz: usize, cpu_hz: usize) -> Self
    {
        Self { cpu: Cpu::new(), system_bus: SystemBus::new(), timer_hz, cpu_hz }
    }

    pub(crate) fn load_rom(&mut self, file: &str)
    {
        let mut handle = File::open(file).expect("ROM file was not found.");
        let mut buff = [0; 3584];

        handle.read(&mut buff).expect("Could not read the ROM contents.");

        self.system_bus.ram.write_bulk(&mut buff, START_ADDR as usize);
    }

    pub(crate) fn run(&mut self)
    {
        let sleep_duration = Duration::from_secs_f64(1.0 / (self.cpu_hz as f64));
        let cycles_needed_for_timer_update =
            (self.cpu_hz as f64 / self.timer_hz as f64).ceil() as usize;

        let mut cpu_cycles: usize = 0;
        loop
        {
            if cpu_cycles == cycles_needed_for_timer_update
            {
                self.system_bus.delay_timer.decrement();
                self.system_bus.sound_timer.decrement();
                cpu_cycles = 0;
            }

            self.cpu.execute_operation(&mut self.system_bus);
            cpu_cycles += 1;

            sleep(sleep_duration);
        }
    }
}
