use image::ImageError;

#[derive(thiserror::Error, Debug)]
pub enum PaletteError {
    #[error("could not load image {0}")]
    ImageError(#[from] ImageError),
    #[error("could not extract color palette {0}")]
    ExtractError(#[from] std::num::TryFromIntError),

    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error)
}
