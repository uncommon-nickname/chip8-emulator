const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

pub(super) struct Gpu
{
    vram: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
}

impl Gpu
{
    pub(super) fn new() -> Self
    {
        Self { vram: [false; SCREEN_HEIGHT * SCREEN_WIDTH] }
    }

    pub(super) fn clear(&mut self)
    {
        self.vram.fill(false);
    }
}
