extern crate sfml;
extern crate sfml_modstream;

use sfml::audio::{SoundStatus, SoundStreamPlayer};
use sfml_modstream::{Metadata, ModStream};
use std::fmt;
use std::io::Write;
use std::path::Path;

struct HrTime {
    seconds: f64,
}

impl fmt::Display for HrTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02.0}:{:04.1}",
            (self.seconds / 60.).floor(),
            self.seconds % 60.
        )
    }
}

struct OptStrFmt<'a> {
    str: &'a str,
    prepend: &'static str,
    append: &'static str,
}

impl<'a> OptStrFmt<'a> {
    fn new(prepend: &'static str, str: &'a str, append: &'static str) -> Self {
        Self {
            str,
            prepend,
            append,
        }
    }
}

impl<'a> std::fmt::Display for OptStrFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.str.is_empty() {
            write!(f, "{}{}{}", self.prepend, self.str, self.append)
        } else {
            Ok(())
        }
    }
}

fn play_song(path: &Path) {
    let mut stream = ModStream::from_file(path).unwrap();
    let duration = HrTime {
        seconds: stream.get_duration_seconds(),
    };
    let filename = path.file_name().unwrap().to_string_lossy();
    let title = stream.metadata(Metadata::Title).unwrap();
    let artist = stream.metadata(Metadata::Artist).unwrap();
    let date = stream.metadata(Metadata::Date).unwrap();
    let message = stream.metadata(Metadata::Message).unwrap();
    let tracker = stream.metadata(Metadata::Tracker).unwrap();
    let made_in_string = match (tracker.as_str().is_empty(), date.as_str().is_empty()) {
        (true, true) => String::new(),
        (false, true) => format!("Made in {}\n", tracker),
        (true, false) => format!("Made in {}\n", date),
        (false, false) => format!("Made in {} on {}\n", tracker, date),
    };
    println!(
        "Playing {} {}[{}]\n\
        {}\
        {}",
        title,
        OptStrFmt::new(" by ", artist.as_str(), " "),
        filename,
        made_in_string,
        message
    );
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();
    while player.status() == SoundStatus::PLAYING {
        let offset = HrTime {
            seconds: f64::from(player.playing_offset().as_seconds()),
        };
        print!("{}/{}\r", offset, duration);
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
