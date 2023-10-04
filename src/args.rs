use std::path::PathBuf;

#[derive(Debug, clap::Parser, Default, Clone)]
#[command(author="Horry Portier", version, about, long_about = None)]
pub struct Arguments {
    /// Path of image to generate palette from
    #[clap(short, long)]
    pub img_path: PathBuf,

    /// Safe path for generated palette.
    /// If not porvided [PALATTE_SAFE_FILE] env will be used
    #[clap(short, long)]
    pub safe_path: Option<PathBuf>,

    /// don't show output
    #[clap(short, long)]
    pub quiet: bool,

    /// number of trails [def 1]
    #[clap(short, long)]
    pub trials: Option<u32>,
    /// k-means value responsible for nubmer of colors generated [def 8]
    #[clap(short, long="k_means")]
    pub k: Option<u8>,
    /// convergance_threshold [def 0.05]
    #[clap(short, long)]
    pub convergence_threshold: Option<f32>,
    /// maximal value of iteration [def 64]
    #[clap(short, long)]
    pub max_iter: Option<u32>,
    /// seed value  [def 0]
    #[clap(long)]
    pub seed: Option<u64>,
}
