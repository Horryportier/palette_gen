use std::{path::PathBuf, fs};

use clap::Parser;
use colorful::Colorful;
use crate::{args::Arguments, serializers::TXT_SER, palette::{Palette, PaletteOptions}};

static  PALETTE_SAVE_FILE: fn() -> &'static str = || option_env!("PALETTE_SAVE_FILE").expect("PALETTE_SAVE_FILE is not set. Set it or pass it as an argument!");

pub struct Cli;

impl Cli {
   pub fn execute() {
        let args = Arguments::parse();

        let safe_file = match args.clone().safe_path {
            Some(path) => path,
            None => { let env = PALETTE_SAVE_FILE(); println!("{env}"); PathBuf::from(env)}
        };

        if !args.quiet {
            println!("{} {}","generating pallet from".rgb(12, 235, 71), args.img_path.to_str().unwrap().rgb(12, 120, 235))
        }
        
        let pal_options = PaletteOptions::from(args.clone());

        let palette = Palette::pallet_from_img(args.img_path, pal_options).expect("Couldn't generate palette from image");
        if !args.quiet {
            println!("{}", "Palette".rgb(12, 235, 71));
            palette.preaty_print()
        }

        let serialize = palette.serialize(TXT_SER).expect("couln't serialize pallet");
        println!("{} {}", "Saving to".rgb(12, 235, 71), safe_file.to_str().unwrap().rgb(12, 120, 235));
        fs::write(safe_file, serialize).expect("couldn't write to file");
       
        println!("{}", "Saved Succesfully".rgb(12, 235, 71));
    }
}
