mod common;
use std::{ffi::OsStr, path::Path};

use asm::compile_file;
use common::*;
mod hack;
use hack::*;
mod asm;
use clap::Parser;
use io::*;
use machine::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Whether to run in windowed mode
    #[arg(long, default_value_t = false)]
    quiet: bool,

    /// Path to instruction file
    file: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Res {
    let args = Args::parse();

    let (instructions, _debug_info) = {
        let path = Path::new(&args.file);
        if path.extension().and_then(OsStr::to_str) == Some("asm") {
            compile_file(path, args.debug)?
        } else {
            (read_instructions(path)?, None)
        }
    };

    let mut machine = Machine::new();
    machine.load_instructions(instructions);

    if !args.quiet {
        run_io(machine)?;
    } else {
        machine.run()?;
    }

    Ok(())
}

pub fn run_asm(asm: &str, machine: &mut Machine) -> Res<> {
    use crate::asm::compile;
    let (instructions, _) = compile(asm.lines().map(|x| x.into()).collect(), false)?;

    machine.load_instructions(instructions);

    machine.run()?;

    Ok(())
}