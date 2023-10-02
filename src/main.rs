use palette::Palette;

use crate::serializers::TXT_SER;

mod palette;
mod error;
mod serializers;

fn main() {
   let test_pic = "/home/horry/Pictures/Wallpapers/victor_belmont.png";

   let pal  = Palette::pallet_from_img(test_pic.into()).unwrap();
   println!("{}",pal.serialize(TXT_SER).unwrap());
}
