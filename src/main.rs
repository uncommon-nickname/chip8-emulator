mod chip8;

use chip8::Chip8Emulator;

fn main()
{
    let mut emulator = Chip8Emulator::new();
    emulator.run_cycle();
}
