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
    #[arg(long, default_value_t = true)]
    io: bool,

    /// Path to instruction file
    file: String,
}

fn main() -> Res {
    let args = Args::parse();

    let instructions = {
        let path = Path::new(&args.file);
        if path.extension().and_then(OsStr::to_str) == Some("asm") {
            compile_file(path)?
        } else {
            read_instructions(path)?
        }
    };

    let mut machine = Machine::new(instructions);

    if args.io {
        run_io(machine)?;
    } else {
        machine.run()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::asm::compile;

    use super::*;
    
    #[test]
    fn run_asm() {
        let asm: Vec<String> = r#"
            (LOOP)
            @SCREEN // set address register to top-left pixel
            M=1     // blacken pixel
            @LOOP
            0;JMP
        "#.lines().map(|x| x.into()).collect();

        let instructions = compile(asm).unwrap();

        let machine = Machine::new(instructions);
        run_io(machine).unwrap();
    }
}