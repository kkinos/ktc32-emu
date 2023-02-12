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
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = std::fs::read_to_string(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;

    let f = f.split_whitespace().collect::<Vec<_>>();
    let mut program: Vec<u8> = vec![];
    for line in f {
        let mut hex = hex::decode(line).with_context(|| format!("could not decode to hex"))?;
        program.append(&mut hex);
    }

    let mut emu = Emulator::new(program);
    emu.memory.init();

    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .with_context(|| format!("failed to read command"))?;
        let command = input.trim();

        match command {
            "run" => {
                emu.run().with_context(|| format!("stop emulator"))?;
                input.clear();
            }
            "s" => {
                emu.step().with_context(|| format!("stop emulator"))?;
                input.clear();
            }
            "step" => {
                input.clear();
                print!("num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| format!("failed to read command"))?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        for _i in 0..n {
                            emu.step().with_context(|| format!("stop emulator"))?;
                        }
                        input.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input.clear();
                    }
                };
            }
            "b" | "breakpoint" => {
                input.clear();
                print!("break point address > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| format!("failed to read command"))?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        emu.break_point = n;
                        input.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input.clear();
                    }
                };
            }
            "m" | "mem" => {
                input.clear();
                print!("address > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| format!("failed to read command"))?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        println!("mem[{}] = {:08x}", n, emu.memory.memory_array[n as usize]);
                        input.clear();
                    }
                    Err(_) => {
                        println!("invalid address");
                        input.clear();
                    }
                };
            }
            "r" | "reg" => {
                input.clear();
                print!("register num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| format!("failed to read command"))?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        println!("register[{}] = {:08x}", n, emu.cpu.register[n as usize]);
                        input.clear();
                    }
                    Err(_) => {
                        println!("invalid num");
                        input.clear();
                    }
                };
            }
            "finish" => {
                println!("finish emulator");
                break;
            }
            _ => {
                println!("command not found {}", command);
                input.clear();
            }
        }
    }

    Ok(())
}
