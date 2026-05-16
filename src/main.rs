use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;

static SPINNERS_FILE: &str = "../spinners.json";
static DEFAULT_SPINNER: &str = "aesthetic";

type Frames = Vec<String>;

#[derive(Deserialize, Debug)]
struct SpinnerData {
    frames: Frames,
}

type Spinners = HashMap<String, SpinnerData>;

struct Spinner {
    name: String,
    frame_count: usize,
    cur_frame: usize,
    frames: Frames,
}

impl Spinner {
    pub fn new(name: &str, s: &SpinnerData) -> Self {
        Spinner {
            name: name.into(),
            frame_count: s.frames.len(),
            cur_frame: 0,
            frames: s.frames.clone(),
        }
    }
}

fn main() {
    let interval = Duration::from_secs(1);
    let total_seconds = 10;

    let spinners = match get_spinners(SPINNERS_FILE) {
        Ok(s) => s,
        Err(err) => {
            println!("error: {}", err);
            std::process::exit(1);
        }
    };

    let mut s = Spinner::new(DEFAULT_SPINNER, &spinners[DEFAULT_SPINNER]);

    println!("using {}", s.name);
    for i in 1..=total_seconds {
        thread::sleep(interval);

        let frame = s.frames[s.cur_frame].clone();
        thread::spawn(move || {
            print_status(i, frame);
        });

        s.cur_frame = advance_frame(s.cur_frame, s.frame_count);
    }
}

fn advance_frame(cur_frame: usize, frame_count: usize) -> usize {
    match cur_frame {
        n if n == frame_count - 1 => 0,
        _ => cur_frame + 1,
    }
}

fn get_spinners<P: AsRef<Path>>(path: P) -> Result<Spinners, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let s = serde_json::from_reader(reader)?;

    Ok(s)
}

fn print_status(count: i32, frame: String) {
    println!("{} {}", frame, count);
}
