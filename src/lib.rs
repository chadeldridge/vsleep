//! A verbose, visual replacement for the Unix `sleep` command.
//!
//! Displays an animated spinner in the terminal while sleeping, with optional
//! elapsed and remaining time readouts. The library exposes the timer and
//! spinner primitives so they can be reused in other CLI tools.
//!
//! # Example
//!
//! ```no_run
//! use std::time::Duration;
//! use vsleep::core::{Spinners, Timer};
//!
//! let spinners = Spinners::default();
//! let mut spinner = spinners.get_spinner("aesthetic").unwrap();
//! let mut timer = Timer::new(Duration::from_secs(5), Duration::from_secs(1));
//!
//! timer.run(|tick| {
//!     println!("{} {}s remaining", spinner.frame(), tick.remaining);
//!     spinner.step_frame();
//! });
//! ```

pub mod core;
