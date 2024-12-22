pub(super) struct Timer
{
    cycles: u8,
}

impl Timer
{
    pub(super) fn new() -> Self
    {
        Self { cycles: 0 }
    }

    pub(super) fn decrement(&mut self)
    {
        let _ = self.cycles.saturating_sub(1);
    }
}
