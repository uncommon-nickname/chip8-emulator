pub(super) const RAM_SIZE: usize = 4096;
const FONTS_SIZE: usize = 80;

const FONTS: [u8; FONTS_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub(super) struct Ram
{
    /// Chip8 used 4Kb of RAM.
    /// The memory is segmented into two parts:
    /// - 0x000-0x1FF: addresses reserved by interpreter to store internal info (like preloaded fonts).
    /// - 0x200-0xFFF: program space which can be used to store `ROM` and program data.
    memory: [u8; RAM_SIZE],
}

impl Ram
{
    pub(super) fn preload() -> Self
    {
        let mut ram = Self { memory: [0; RAM_SIZE] };
        ram.memory[..FONTS_SIZE].copy_from_slice(&FONTS);
        ram
    }

    pub(super) fn read(&mut self, addr: u16) -> u8
    {
        self.memory[addr as usize]
    }

    pub(super) fn write_bulk(&mut self, data: &mut [u8], start: usize)
    {
        self.memory[start..start + data.len()].copy_from_slice(data);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn ram_preloaded_with_fonts()
    {
        let ram = Ram::preload();
        assert_eq!(ram.memory[..FONTS_SIZE], FONTS);
    }

    #[test]
    fn reading_a_byte_from_ram()
    {
        let mut ram = Ram::preload();
        ram.memory[0x200] = 0x01;

        assert_eq!(ram.read(0x200), 0x01);
    }

    #[test]
    fn write_ram_bulk()
    {
        let mut ram = Ram::preload();
        let mut buff = [1, 2, 3, 4];

        ram.write_bulk(&mut buff, 0x200);

        assert_eq!(ram.memory[0x200..0x204], [1, 2, 3, 4]);
    }
}
