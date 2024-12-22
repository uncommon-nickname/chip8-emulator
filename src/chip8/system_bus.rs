use super::gpu::Gpu;
use super::ram::Ram;
use super::timer::Timer;

pub(super) struct SystemBus
{
    pub(super) ram: Ram,
    pub(super) delay_timer: Timer,
    pub(super) sound_timer: Timer,
    pub(super) gpu: Gpu,
}

impl SystemBus
{
    pub(super) fn new() -> Self
    {
        Self { ram: Ram::preload(),
               delay_timer: Timer::new(),
               sound_timer: Timer::new(),
               gpu: Gpu::new() }
    }
}
