mod chip8;

use chip8::Chip8Emulator;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args
{
    #[arg(default_value_t = 60)]
    timer_hz: usize,

    #[arg(default_value_t = 1000)]
    cpu_hz: usize,
}

fn main()
{
    let args = Args::parse();

    let mut emulator = Chip8Emulator::new(args.timer_hz, args.cpu_hz);
    emulator.run();
}
