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

/// The raw frame data for a single spinner animation.
///
/// Each spinner is an ordered list of string frames displayed in sequence to
/// create the animation effect. `SpinnerData` is produced by deserializing a
/// spinners JSON file and is not directly constructable.
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

/// A collection of named [`Spinner`] animations.
///
/// The default set is loaded from the bundled `spinners.json` and includes
/// dozens of terminal spinner styles. Additional spinners can be merged in
/// from an external file with [`Spinners::import_spinners`].
///
/// # Examples
///
/// ```
/// use vsleep::core::Spinners;
///
/// let spinners = Spinners::default();
/// assert!(spinners.contains_spinner("aesthetic"));
/// ```
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
    /// Returns `true` if a spinner with the given name exists in the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use vsleep::core::Spinners;
    ///
    /// let spinners = Spinners::default();
    /// assert!(spinners.contains_spinner("aesthetic"));
    /// assert!(!spinners.contains_spinner("nonexistent"));
    /// ```
    pub fn contains_spinner(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Returns a [`Spinner`] for the given name, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use vsleep::core::Spinners;
    ///
    /// let spinners = Spinners::default();
    /// assert!(spinners.get_spinner("aesthetic").is_some());
    /// assert!(spinners.get_spinner("nonexistent").is_none());
    /// ```
    pub fn get_spinner(&self, name: &str) -> Option<Spinner> {
        self.0.get(name).map(|data| Spinner::new(name, data))
    }

    /// Loads additional spinners from a JSON file, merging them into the collection.
    ///
    /// The file must be a JSON object mapping spinner names to objects with a
    /// `"frames"` array of strings. Existing spinners with the same name are
    /// replaced.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] if the file cannot be opened, or [`Error::Json`]
    /// if the file content is not valid spinner JSON.
    ///
    /// [`Error::Io`]: crate::core::Error::Io
    /// [`Error::Json`]: crate::core::Error::Json
    #[cfg(feature = "serde")]
    pub fn import_spinners<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let s: Spinners = serde_json::from_reader(reader)?;
        self.extend(s);

        Ok(())
    }

    /// Returns an iterator over the spinner names in the collection.
    ///
    /// Names are yielded in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use vsleep::core::Spinners;
    ///
    /// let spinners = Spinners::default();
    /// for name in spinners.keys() {
    ///     println!("{name}");
    /// }
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &'_ String> {
        self.0.keys()
    }
}

/// A single named spinner animation with a current frame position.
///
/// Obtain a `Spinner` from [`Spinners::get_spinner`] rather than constructing
/// one directly.
///
/// # Examples
///
/// ```
/// use vsleep::core::Spinners;
///
/// let spinners = Spinners::default();
/// let mut spinner = spinners.get_spinner("aesthetic").unwrap();
/// let frame = spinner.frame();
/// spinner.step_frame();
/// ```
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
    /// Creates a new `Spinner` from a name and its frame data.
    pub fn new(name: &str, s: &SpinnerData) -> Self {
        Spinner {
            name: name.into(),
            cur_frame: 0,
            frames: s.frames.clone(),
        }
    }

    /// Returns the spinner's name as it appears in the spinner collection.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the current animation frame as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use vsleep::core::Spinners;
    ///
    /// let spinners = Spinners::default();
    /// let spinner = spinners.get_spinner("aesthetic").unwrap();
    /// assert!(!spinner.frame().is_empty());
    /// ```
    pub fn frame(&self) -> String {
        self.frames[self.cur_frame].to_string()
    }

    /// Returns the total number of frames in the animation.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Advances to the next animation frame, wrapping back to the first after the last.
    ///
    /// # Examples
    ///
    /// ```
    /// use vsleep::core::Spinners;
    ///
    /// let spinners = Spinners::default();
    /// let mut spinner = spinners.get_spinner("aesthetic").unwrap();
    /// spinner.step_frame();
    /// ```
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
