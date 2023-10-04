use std::path::PathBuf;

use colorful::Colorful;
use hex_color::HexColor;
use oklab::RGB;

use crate::{args::Arguments, error::PaletteError};

#[derive(Debug)]
pub struct Palette(pub Vec<HexColor>);

impl Palette {
    pub fn new(colors: Vec<HexColor>) -> Palette {
        Palette(colors)
    }

    pub fn pallet_from_img(path: PathBuf, options: PaletteOptions) -> Result<Palette, PaletteError> {
        let img = image::open(path)?;

        let extracted_palette = okolors::run(
            &okolors::OklabCounts::try_from_image(&img, u8::MAX)?,
            options.trials,
            options.k,
            options.convergence_threshold,
            options.max_iter,
            options.seed,
        );
        let rgb = extracted_palette
            .centroids
            .iter()
            .map(|f| {
                oklab::oklab_to_srgb(oklab::Oklab {
                    l: f.l,
                    a: f.a,
                    b: f.b,
                })
            })
            .collect::<Vec<RGB<u8>>>();

        let mut hex = rgb
            .iter()
            .map(|f| hex_color::HexColor::rgb(f.r, f.g, f.b))
            .collect::<Vec<HexColor>>();

        // this puts darkest colors in the as first and birhtest as last
        hex.sort();
        Ok(Palette::new(hex))
    }

    pub fn preaty_print(&self) {
        self.0.iter().for_each(|f| {
            let (r, g, b, _) = f.split();
            println!("{}", f.to_string().rgb(r, g, b))
        });
    }

    pub fn serialize(
        self,
        serializer: fn(Palette) -> Result<String, PaletteError>,
    ) -> Result<String, PaletteError> {
        serializer(self)
    }
}

pub struct PaletteOptions {
    pub trials: u32,
    pub k: u8,
    pub convergence_threshold: f32,
    pub max_iter: u32,
    pub seed: u64,
}

impl From<Arguments> for PaletteOptions {
    fn from(value: Arguments) -> Self {
        PaletteOptions {
            trials: value.trials.unwrap_or(1),
            k: value.k.unwrap_or(8),
            convergence_threshold: value.convergence_threshold.unwrap_or(0.05),
            max_iter: value.max_iter.unwrap_or(64),
            seed: value.seed.unwrap_or(0),
        }
    }
}
