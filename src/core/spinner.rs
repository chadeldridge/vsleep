use std::{collections::HashMap, fmt};

#[cfg(feature = "serde")]
use std::{fs::File, io::BufReader, path::Path};

#[cfg(feature = "serde")]
use super::error::Result;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
const BUNDLED_SPINNERS: &str = include_str!("spinners.json");

type Frames = Vec<String>;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpinnerData {
    frames: Frames,
}

impl fmt::Display for SpinnerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.frames.join(" | "))
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Spinners(HashMap<String, SpinnerData>);

#[cfg(feature = "serde")]
impl Default for Spinners {
    fn default() -> Self {
        serde_json::from_str(BUNDLED_SPINNERS).expect("bundled spinners.json is invalid")
    }
}

impl fmt::Display for Spinners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut names: Vec<&String> = self.0.keys().collect();
        names.sort();
        write!(
            f,
            "{}",
            names
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl From<HashMap<String, SpinnerData>> for Spinners {
    fn from(map: HashMap<String, SpinnerData>) -> Self {
        Spinners(map)
    }
}

impl IntoIterator for Spinners {
    type Item = (String, SpinnerData);
    type IntoIter = std::collections::hash_map::IntoIter<String, SpinnerData>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<(String, SpinnerData)> for Spinners {
    fn extend<I: IntoIterator<Item = (String, SpinnerData)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl FromIterator<(String, SpinnerData)> for Spinners {
    fn from_iter<I: IntoIterator<Item = (String, SpinnerData)>>(iter: I) -> Self {
        Spinners(HashMap::from_iter(iter))
    }
}

impl Spinners {
    pub fn contains_spinner(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn get_spinner(&self, name: &str) -> Option<Spinner> {
        self.0.get(name).map(|data| Spinner::new(name, data))
    }

    #[cfg(feature = "serde")]
    pub fn import_spinners<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let s: Spinners = serde_json::from_reader(reader)?;
        self.extend(s);

        Ok(())
    }

    pub fn keys(&self) -> impl Iterator<Item = &'_ String> {
        self.0.keys()
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Spinner {
    /// Spinner name as referenced in the JSON.
    name: String,
    /// The frame to display.
    cur_frame: usize,
    /// An array of frames in order to display.
    frames: Frames,
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.frames[self.cur_frame])
    }
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

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<SpinnerData>();
    assert_send::<Spinners>();
    assert_send::<Spinner>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<SpinnerData>();
    assert_sync::<Spinners>();
    assert_sync::<Spinner>();
}
