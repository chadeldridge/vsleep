use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

const BUNDLED_SPINNERS: &str = include_str!("spinners.json");

type Frames = Vec<String>;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct SpinnerData {
    frames: Frames,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Spinners(HashMap<String, SpinnerData>);

impl Default for Spinners {
    fn default() -> Self {
        serde_json::from_str(BUNDLED_SPINNERS).expect("bundled spinners.json is invalid")
    }
}

impl Spinners {
    pub fn get_spinner(&self, name: &str) -> Option<Spinner> {
        match self.0.contains_key(name) {
            true => Some(Spinner::new(name, &self.0[name])),
            _ => None,
        }
    }

    pub fn import_spinners<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let s = serde_json::from_reader(reader)?;
        self.extend(s);

        Ok(())
    }

    pub fn extend(&mut self, s: Spinners) {
        self.0.extend(s.0);
    }

    pub fn keys(&self) -> impl Iterator<Item = &'_ String> {
        self.0.keys()
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Spinner {
    /// Spinner name as referenced in the JSON.
    name: String,
    /// The frame to display.
    cur_frame: usize,
    /// An array of frames in order to display.
    frames: Frames,
}

impl Spinner {
    pub fn new(name: &str, s: &SpinnerData) -> Self {
        Spinner {
            name: name.into(),
            cur_frame: 0,
            frames: s.frames.clone(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn frame(&self) -> String {
        self.frames[self.cur_frame].to_string()
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn step_frame(&mut self) {
        match self.cur_frame {
            n if n == self.frame_count() - 1 => self.cur_frame = 0,
            _ => self.cur_frame += 1,
        }
    }
}
