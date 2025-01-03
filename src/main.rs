mod chip8;

use chip8::Chip8Emulator;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args
{
    #[arg(short, long, default_value_t = 60)]
    timer_hz: usize,

    #[arg(short, long, default_value_t = 1000)]
    cpu_hz: usize,

    #[arg(short, long, default_value_t = String::from("./roms/pong.chip8"))]
    file: String,
}

fn main()
{
    let args = Args::parse();

    let mut emulator = Chip8Emulator::new(args.timer_hz, args.cpu_hz);

    emulator.load_rom(&args.file);
    emulator.run();
}
