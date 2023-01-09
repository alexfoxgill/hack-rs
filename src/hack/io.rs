extern crate minifb;
use std::{path::Path, time::Duration};

use minifb::{Key, Window, WindowOptions};

use crate::{
    common::{read_lines, Res},
    hackword::HackWord,
    machine::Machine,
};

pub const SCREEN_MEM_START: u16 = 0x4000;
pub const KB_MEM_SLOT: u16 = 0x6000;
pub const SCREEN_WIDTH: usize = 512;
pub const SCREEN_HEIGHT: usize = 256;

pub fn run_io(mut machine: Machine) -> Res<Machine> {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) && machine.step()? {
        write_to_screen(&machine, &mut buffer);
        update_keyboard(&window, &mut machine);
        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)?;
    }

    Ok(machine)
}

fn write_to_screen(machine: &Machine, buffer: &mut Vec<u32>) {
    for row in 0..SCREEN_HEIGHT {
        for w in 0..(SCREEN_WIDTH / 16) {
            let location = SCREEN_MEM_START as usize + (row * (SCREEN_WIDTH as usize / 16)) + w;
            let word = machine.memory[location];
            for i in 0..16u8 {
                let col = i as usize + (w * 16);
                // each word is mapped 'backwards'
                let pix = if word.bit(15 - i) { 0 } else { 0xFFFFFF };
                buffer[(row * SCREEN_WIDTH) + col] = pix;
            }
        }
    }
}

fn update_keyboard(window: &Window, machine: &mut Machine) {
    let code: i16 = window
        .get_keys()
        .iter()
        .find_map(|k| {
            Some(match k {
                Key::Key0 => b'0'.into(),
                Key::Key1 => b'1'.into(),
                Key::Key2 => b'2'.into(),
                Key::Key3 => b'3'.into(),
                Key::Key4 => b'4'.into(),
                Key::Key5 => b'5'.into(),
                Key::Key6 => b'6'.into(),
                Key::Key7 => b'7'.into(),
                Key::Key8 => b'8'.into(),
                Key::Key9 => b'9'.into(),
                Key::A => b'A'.into(),
                Key::B => b'B'.into(),
                Key::C => b'C'.into(),
                Key::D => b'D'.into(),
                Key::E => b'E'.into(),
                Key::F => b'F'.into(),
                Key::G => b'G'.into(),
                Key::H => b'H'.into(),
                Key::I => b'I'.into(),
                Key::J => b'J'.into(),
                Key::K => b'K'.into(),
                Key::L => b'L'.into(),
                Key::M => b'M'.into(),
                Key::N => b'N'.into(),
                Key::O => b'O'.into(),
                Key::P => b'P'.into(),
                Key::Q => b'Q'.into(),
                Key::R => b'R'.into(),
                Key::S => b'S'.into(),
                Key::T => b'T'.into(),
                Key::U => b'U'.into(),
                Key::V => b'V'.into(),
                Key::W => b'W'.into(),
                Key::X => b'X'.into(),
                Key::Y => b'Y'.into(),
                Key::Z => b'Z'.into(),
                Key::F1 => 141,
                Key::F2 => 142,
                Key::F3 => 143,
                Key::F4 => 144,
                Key::F5 => 145,
                Key::F6 => 146,
                Key::F7 => 147,
                Key::F8 => 148,
                Key::F9 => 149,
                Key::F10 => 150,
                Key::F11 => 151,
                Key::F12 => 152,
                Key::Escape => 140,
                Key::Space => b' '.into(),
                Key::Enter => 128,
                Key::Up => 131,
                Key::Down => 133,
                Key::Left => 130,
                Key::Right => 132,
                Key::Backspace => 129,
                Key::Tab => b'\t'.into(),
                Key::Home => 134,
                Key::End => 135,
                Key::PageUp => 136,
                Key::PageDown => 137,
                Key::Insert => 138,
                Key::Delete => 139,
                // Key::Grave => b'~'.into(),
                Key::Minus => b'-'.into(),
                Key::Equal => b'='.into(),
                Key::LeftBracket => b'['.into(),
                Key::RightBracket => b']'.into(),
                Key::Backslash => b'\\'.into(),
                Key::Semicolon => b';'.into(),
                Key::Apostrophe => b'\''.into(),
                Key::Comma => b','.into(),
                Key::Period => b'.'.into(),
                Key::Slash => b'/'.into(),
                Key::Backquote => b'`'.into(),
                _ => return None,
            })
        })
        .unwrap_or(0);

    machine.memory[KB_MEM_SLOT as usize] = HackWord(code);
}

pub fn read_instructions(path: impl AsRef<Path>) -> Res<Vec<HackWord>> {
    let lines = read_lines(path)?;
    lines.iter().map(|x| x.parse()).collect()
}
