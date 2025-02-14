//! Hyper Demon Practice Patcher
//! 
//! A Hyper Demon binary patching program that applies 
//! a set of useful patches to help practice the game.

/// ASM Patch trait module 
mod patch;
/// Predefined asm patches
mod patches;

use self::PatchError::*;
use patch::Patch;
use patches::Patches;

use prompted::input;

use std::fmt::{self, Display, Formatter};
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};

fn main() {
    println!("Hyper Demon Practice Patcher");
    println!("----------------------------");
    println!("by fluffiac :3          v0.1");
    println!();

    match patch_bin() {
        Err(e) => println!("{e}"),
        _ => (),
    }

    println!();
    let _ = input!("Press Enter to close...");
}

pub fn patch_bin() -> Result<(), PatchError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("hyperdemon.exe")
        .or(Err(MissingBinary))?;

    let mut game = Vec::new();
    file.read_to_end(&mut game).or(Err(ReadFail))?;

    println!("What would you like to do?");
    println!("(P)atch or (U)npatch");
    let action = input!("> ");

    println!();
    match action.chars().next() {
        Some(c) if c.to_ascii_lowercase() == 'p' => {
            println!("Patching...");
            Patches::patch(&mut game)?;
        }
        Some(c) if c.to_ascii_lowercase() == 'u' => {
            println!("Unpatching...");
            Patches::unpatch(&mut game)?;
        }
        _ => return Err(StrangeInputError),
    };

    println!("Saving...");
    file.rewind().or(Err(WriteFail))?;
    file.write(&game).or(Err(WriteFail))?;

    println!("All done!");
    Ok(())
}

pub enum PatchError {
    StrangeInputError,
    MissingBinary,
    ReadFail,
    WriteFail,
    AlreadyPatched,
    AlreadyUnpatched,
    BinaryModified,
}

impl Display for PatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mesg = match &self {
            StrangeInputError => return f.write_str("Nothing happened."),
            MissingBinary => "Err: Could not find hyperdemon.exe!",
            ReadFail => "Err: An error occured while reading the game binary!",
            WriteFail => "Err: Something went wrong while writing to the game binary!",
            AlreadyPatched => "Err: The game is already patched!",
            AlreadyUnpatched => "Err: The game is already unpatched!",
            BinaryModified => "Err: The game has been modifed (was there an update?)",
        };

        f.write_str("\x1b[31m")?; // red
        f.write_str(mesg)?;
        f.write_str("\x1b[0m")?; // remove red

        if matches!(self, Self::MissingBinary) {
            f.write_str(
                "\nMove this patcher program to \"steamapps\\common\\hyperdemon\" and run it there.",
            )?;
        }

        Ok(())
    }
}
