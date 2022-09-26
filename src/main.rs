use anyhow::{Context, Ok, Result};
use clap::Parser;
use hex;
use std::io::{self, Write};

mod emulator;
use emulator::Emulator;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "ktc32 emulator")]
struct Cli {
    #[clap(parse(from_os_str))]
    file_path: std::path::PathBuf,

    #[clap(short, long)]
    end_of_address: u32,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = std::fs::read_to_string(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;
    let file = file.split_whitespace().collect::<Vec<_>>();

    let mut memory_data: Vec<u8> = vec![];
    for s in file {
        let mut h = hex::decode(s).context("could not decode to hex")?;
        memory_data.append(&mut h);
    }

    let mut emu = Emulator::new(&memory_data, args.end_of_address);
    emu.memory.init();

    let mut input_command = String::new();
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_command)
            .with_context(|| format!("failed to read command"))?;
        let command = input_command.trim();

        match command {
            "run" | "r" => {
                emu.run();
                println!("memory[84] = {}", emu.memory.data[84]);
                input_command.clear();
            }
            "step" | "s" => {
                emu.step();
                input_command.clear();
            }

            "finish" | "f" => {
                println!("finish this emulator");
                break;
            }
            _ => {
                println!("command not found {}", command);
                input_command.clear();
            }
        }
    }

    Ok(())
}
