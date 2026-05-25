use std::{fmt, thread, time::Duration};

use chrono::{DateTime, Local, TimeDelta, TimeZone};

/// The current state of a [`Timer`].
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimerState {
    /// The timer has been created but [`Timer::run`] has not yet been called.
    ///
    /// The inner value is the total duration in seconds.
    Ready(i64),
    /// The timer is actively running.
    ///
    /// The inner value is the number of seconds elapsed.
    InProgress(i64),
    /// The timer has completed.
    ///
    /// The inner value is the total duration in seconds.
    Ended(i64),
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState::Ready(0)
    }
}

impl fmt::Display for TimerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerState::Ready(n) => write!(f, "Ready ({n}s)"),
            TimerState::InProgress(n) => write!(f, "In Progress ({n}s elapsed)"),
            TimerState::Ended(n) => write!(f, "Ended ({n}s)"),
        }
    }
}

/// A snapshot of timer state delivered to the tick callback on each interval.
///
/// `TickData` is constructed by [`Timer::run`] and passed by reference to the
/// callback on each tick.
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TickData {
    /// Seconds elapsed since the timer started.
    pub elapsed: i64,
    /// Seconds remaining until the timer ends.
    pub remaining: i64,
    /// Local timestamp at the moment this tick fired.
    pub now: DateTime<Local>,
}

impl fmt::Display for TickData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "elapsed: {}s, remaining: {}s",
            self.elapsed, self.remaining
        )
    }
}

/// A countdown timer that calls a tick function on each interval.
///
/// Create a timer with [`Timer::new`] and drive it with [`Timer::run`].
///
/// # Examples
///
/// ```no_run
/// use std::time::Duration;
/// use vsleep::core::Timer;
///
/// let mut timer = Timer::new(Duration::from_secs(10), Duration::from_secs(1));
/// timer.run(|tick| {
///     println!("{}s remaining", tick.remaining);
/// });
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Timer {
    state: TimerState,
    start: DateTime<Local>,
    int: i64,
    dur: i64,
    cur: i64,
    end: DateTime<Local>,
}

impl Timer {
    /// Creates a new timer with the given total `duration` and tick `interval`.
    ///
    /// If `interval` is zero it is clamped to one second. Sub-second values
    /// are truncated to whole seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use vsleep::core::Timer;
    ///
    /// let timer = Timer::new(Duration::from_secs(60), Duration::from_secs(1));
    /// assert_eq!(timer.duration(), 60);
    /// assert_eq!(timer.interval(), 1);
    /// ```
    pub fn new(duration: Duration, interval: Duration) -> Timer {
        let dur = duration.as_secs() as i64;
        let int = match interval.as_secs() {
            0 => 1,
            n => n as i64,
        };
        let start = Local::now();
        let end = start + TimeDelta::seconds(dur);

        Timer {
            state: TimerState::Ready(dur),
            start,
            int,
            dur,
            cur: 0,
            end,
        }
    }

    /// Runs the timer, calling `tick_fn` once per interval until the duration elapses.
    ///
    /// The callback receives a [`TickData`] snapshot with elapsed time, remaining
    /// time, and the current local timestamp. This method blocks the calling thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use vsleep::core::Timer;
    ///
    /// let mut timer = Timer::new(Duration::from_secs(5), Duration::from_secs(1));
    /// timer.run(|tick| {
    ///     println!("{}s remaining", tick.remaining);
    /// });
    /// ```
    pub fn run<F>(&mut self, mut tick_fn: F)
    where
        F: FnMut(&TickData),
    {
        let int_duration = Duration::from_secs(self.int as u64);

        self.state = TimerState::InProgress(0);
        let steps = self.dur / self.int;

        for _ in 0..steps {
            let tick = TickData {
                elapsed: self.cur,
                remaining: self.dur - self.cur,
                now: Local::now(),
            };
            tick_fn(&tick);
            thread::sleep(int_duration);
            self.step();
        }
    }

    /// Returns the current [`TimerState`].
    pub fn state(&self) -> TimerState {
        self.state
    }

    /// Returns the local timestamp when the timer was created.
    pub fn start(&self) -> DateTime<Local> {
        self.start
    }

    /// Returns the timer's start time converted to the given timezone.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use chrono::Local;
    /// use vsleep::core::Timer;
    ///
    /// let timer = Timer::new(Duration::from_secs(10), Duration::from_secs(1));
    /// let _ = timer.start_tz(Local);
    /// ```
    pub fn start_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
        self.start.with_timezone(&timezone)
    }

    /// Returns the tick interval in seconds.
    pub fn interval(&self) -> i64 {
        self.int
    }

    /// Returns the tick interval as a [`chrono::TimeDelta`].
    pub fn interval_duration(&self) -> TimeDelta {
        TimeDelta::seconds(self.int)
    }

    /// Returns the total timer duration in seconds.
    pub fn duration(&self) -> i64 {
        self.dur
    }

    /// Returns the number of seconds elapsed since the timer started.
    pub fn elapsed(&self) -> i64 {
        self.cur
    }

    /// Returns the local timestamp at which the timer will end.
    pub fn end(&self) -> DateTime<Local> {
        self.end
    }

    /// Returns the timer's end time converted to the given timezone.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use chrono::Local;
    /// use vsleep::core::Timer;
    ///
    /// let timer = Timer::new(Duration::from_secs(10), Duration::from_secs(1));
    /// let _ = timer.end_tz(Local);
    /// ```
    pub fn end_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
        self.end.with_timezone(&timezone)
    }

    /// Advances the timer by one interval, updating elapsed time and [`TimerState`].
    ///
    /// Called automatically by [`Timer::run`]. Exposed for callers who want to
    /// drive the timer manually.
    pub fn step(&mut self) {
        self.cur += self.int;
        self.state = match self.cur {
            n if n == self.dur => TimerState::Ended(n),
            _ => TimerState::InProgress(self.cur),
        };
    }

    /// Returns the time remaining until the timer ends as a [`chrono::TimeDelta`].
    ///
    /// Computed from the current wall-clock time; may differ slightly from the
    /// `remaining` field in a [`TickData`] snapshot.
    pub fn time_remaining(&self) -> TimeDelta {
        self.end - Local::now()
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TimerState>();
    assert_send::<TickData>();
    assert_send::<Timer>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TimerState>();
    assert_sync::<TickData>();
    assert_sync::<Timer>();
}

#[test]
fn test_timer_new() {
    let timer = Timer::new(Duration::from_secs(60), Duration::from_secs(1));
    assert_eq!(timer.duration(), 60);
    assert_eq!(timer.interval(), 1);
    assert_eq!(timer.state(), TimerState::Ready(60));
    assert!(timer.start().timestamp() > 0);
    assert!(timer.end().timestamp() > 0);
    assert!(timer.end() > timer.start());
}

#[test]
fn test_timer_run() {
    let mut timer = Timer::new(Duration::from_secs(2), Duration::from_secs(1));
    assert_eq!(timer.state(), TimerState::Ready(2));
    timer.run(|_| {});
    assert_eq!(timer.state(), TimerState::Ended(2));
    assert_eq!(timer.cur, 2);
}
