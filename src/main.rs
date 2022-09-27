use anyhow::{Context, Result};
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
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_command)
            .with_context(|| format!("failed to read command"))?;
        let command = input_command.trim();

        match command {
            "run" => {
                emu.run();
                input_command.clear();
            }
            "s" => {
                emu.step();
                input_command.clear();
            }
            "step" => {
                input_command.clear();
                print!("num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input_command)
                    .with_context(|| format!("failed to read command"))?;

                match input_command.trim().parse::<u32>() {
                    Ok(n) => {
                        for _i in 0..n {
                            emu.step();
                        }
                        input_command.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input_command.clear();
                    }
                };
            }
            "b" => {
                input_command.clear();
                print!("break point address > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input_command)
                    .with_context(|| format!("failed to read command"))?;

                match input_command.trim().parse::<u32>() {
                    Ok(n) => {
                        emu.break_point = n;
                        input_command.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input_command.clear();
                    }
                };
            }
            "m" | "mem" => {
                input_command.clear();
                print!("address > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input_command)
                    .with_context(|| format!("failed to read command"))?;

                match input_command.trim().parse::<u32>() {
                    Ok(n) => {
                        println!("mem[{}] = {}", n, emu.memory.data[n as usize]);
                        input_command.clear();
                    }
                    Err(_) => {
                        println!("invalid address");
                        input_command.clear();
                    }
                };
            }
            "r" | "reg" => {
                input_command.clear();
                print!("register num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input_command)
                    .with_context(|| format!("failed to read command"))?;

                match input_command.trim().parse::<u32>() {
                    Ok(n) => {
                        println!("register[{}] = {}", n, emu.cpu.register[n as usize]);
                        input_command.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input_command.clear();
                    }
                };
            }
            "finish" => {
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
