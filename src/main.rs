use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

mod emulator;
use emulator::Emulator;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "KTC32 emulator")]
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
        let mut hex = hex::decode(line).with_context(|| "could not decode to hex".to_string())?;
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
            .with_context(|| "failed to read command".to_string())?;
        let command = input.trim();

        match command {
            "run" => {
                emu.run().with_context(|| "stop emulator".to_string())?;
                input.clear();
            }
            "s" => {
                emu.step().with_context(|| "stop emulator".to_string())?;
                input.clear();
            }
            "step" => {
                input.clear();
                print!("num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| "failed to read command".to_string())?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        for _i in 0..n {
                            emu.step().with_context(|| "stop emulator".to_string())?;
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
                    .with_context(|| "failed to read command".to_string())?;

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
                    .with_context(|| "failed to read command".to_string())?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        if n >= emulator::memory::MEMORY_SIZE {
                            println!("invalid address");
                            input.clear();
                        } else {
                            println!("mem[{}] = 0x{:02x}", n, emu.memory.memory_array[n as usize]);
                            input.clear();
                        }
                    }
                    Err(_) => {
                        println!("invalid address");
                        input.clear();
                    }
                };
            }
            "wm" | "writemem" => {
                input.clear();
                print!("address > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| "failed to read command".to_string())?;

                match input.trim().parse::<u32>() {
                    Ok(n) => {
                        if n >= emulator::memory::MEMORY_SIZE {
                            println!("invalid address");
                            input.clear();
                        } else {
                            input.clear();
                            print!("data > ");
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut input)
                                .with_context(|| "failed to read command".to_string())?;

                            match input.trim().parse::<u8>() {
                                Ok(d) => {
                                    emu.memory.memory_array[n as usize] = d;
                                    println!(
                                        "mem[{}] = 0x{:02x}",
                                        n, emu.memory.memory_array[n as usize]
                                    );
                                    input.clear();
                                }
                                Err(_) => {
                                    println!("invalid data");
                                    input.clear();
                                }
                            };
                        }
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
                    .with_context(|| "failed to read command".to_string())?;

                match input.trim().parse::<u8>() {
                    Ok(n) => {
                        if n >= 32 {
                            println!("invalid num");
                            input.clear();
                        } else {
                            println!("register[{}] = 0x{:08x}", n, emu.cpu.get_reg(n));
                            input.clear();
                        }
                    }
                    Err(_) => {
                        println!("invalid num");
                        input.clear();
                    }
                };
            }
            "wr" | "writereg" => {
                input.clear();
                print!("register num > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| "failed to read command".to_string())?;

                match input.trim().parse::<u8>() {
                    Ok(n) => {
                        if n >= 32 {
                            println!("invalid num");
                            input.clear();
                        } else {
                            input.clear();
                            print!("data > ");
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut input)
                                .with_context(|| "failed to read command".to_string())?;

                            match input.trim().parse::<u32>() {
                                Ok(d) => {
                                    emu.cpu.set_reg(n, d);
                                    println!("register[{}] = 0x{:08x}", n, emu.cpu.get_reg(n));
                                    input.clear();
                                }
                                Err(_) => {
                                    println!("invalid data");
                                    input.clear();
                                }
                            };
                        }
                    }
                    Err(_) => {
                        println!("invalid address");
                        input.clear();
                    }
                };
            }
            "h" | "help" => {
                println!("run           : continue to execute until break point");
                println!();
                println!("s, step       : step execute");
                println!();
                println!("b, breakpoint : set breakpoint");
                println!();
                println!("m, mem        : display data in memory");
                println!();
                println!("wm, writemem  : write data to memory");
                println!();
                println!("r, reg        : display data in register");
                println!();
                println!("wr, writereg  : write data to register");
                println!();
                println!("h, help       : show this message");
                println!();
                println!("finish        : finish emulator");

                input.clear();
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
