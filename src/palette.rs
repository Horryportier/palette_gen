use hex_color::HexColor;
use oklab::RGB;


use crate::error::PaletteError;

#[derive(Debug)]
pub struct Palette(pub Vec<HexColor>);

impl Palette {
    pub fn new(colors: Vec<HexColor>) -> Palette {
        Palette(colors)
    }

    pub fn pallet_from_img(path: String) -> Result<Palette, PaletteError> {
        let img = image::open(path)?;

        let extracted_palette = okolors::run(
            &okolors::OklabCounts::try_from_image(&img, u8::MAX)?,
            1,
            8,
            0.05,
            64,
            0,
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

        let hex = rgb
            .iter()
            .map(|f| hex_color::HexColor::rgb(f.r, f.g, f.b))
            .collect::<Vec<HexColor>>();

        Ok(Palette::new(hex))
    }

    pub fn serialize(self, serializer: fn(Palette) -> Result<String, PaletteError>) -> Result<String, PaletteError> {
        serializer(self)
    }
}
