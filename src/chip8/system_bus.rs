use super::ram::Ram;

pub(super) struct SystemBus
{
    pub(super) ram: Ram,
}

impl SystemBus
{
    pub(super) fn new() -> Self
    {
        Self { ram: Ram::preload() }
    }
}
