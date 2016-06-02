extern crate sfml;
extern crate sfml_modstream;

use sfml::audio::{SoundStatus, SoundStreamPlayer};
use sfml_modstream::ModStream;

fn main() {
    let path = std::env::args().skip(1).next().expect("Need path to mod music file as argument");
    let mut stream = ModStream::load(path).unwrap();
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();
    while player.get_status() == SoundStatus::Playing {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
