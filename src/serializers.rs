use crate::{error::PaletteError, palette::Palette};

pub static TXT_SER: fn(Palette) -> Result<String, PaletteError> =
    |p| Ok(p.0.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n"));
