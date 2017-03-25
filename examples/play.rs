extern crate sfml;
extern crate sfml_modstream;

use sfml::audio::{SoundStatus, SoundStreamPlayer};
use sfml_modstream::ModStream;
use std::fmt;
use std::io::Write;
use std::path::Path;

struct HrTime {
    seconds: f64,
}

impl fmt::Display for HrTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:02.0}:{:04.1}",
               (self.seconds / 60.).floor(),
               self.seconds % 60.)
    }
}

fn play_song(path: &Path) {
    let mut stream = ModStream::load(&path).unwrap();
    let duration = HrTime { seconds: stream.get_duration_seconds() };
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();
    let filename = path.file_name().unwrap().to_string_lossy();
    while player.status() == SoundStatus::Playing {
        let offset = HrTime { seconds: player.playing_offset().as_seconds() as f64 };
        print!("Playing {}: {}/{}\r", filename, offset, duration);
        let _ = std::io::stdout().flush();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // Leave the output of the previous song on the previous line
    println!()
}

fn main() {
    let args = std::env::args_os().skip(1);

    if args.len() == 0 {
        println!("You might want to supply some song paths as arguments. Just saying.");
    }

    for arg in args {
        play_song(arg.as_ref());
    }
}
