use std::{fmt, thread};

use chrono::{DateTime, Local, TimeDelta, TimeZone, Utc};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimerState {
    Ready(i64),
    InProgress(i64),
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TickData {
    pub elapsed: i64,
    pub remaining: i64,
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Timer {
    state: TimerState,
    start: DateTime<Utc>,
    int: i64,
    dur: i64,
    cur: i64,
    end: DateTime<Utc>,
}

impl Timer {
    pub fn new(duration: TimeDelta, interval: TimeDelta) -> Timer {
        let dur = duration.num_seconds();
        let int = match interval.num_seconds() {
            0 => 1,
            n => n,
        };
        let start = Utc::now();
        let end = start + duration;

        Timer {
            state: TimerState::Ready(dur),
            start,
            int,
            dur,
            cur: 0,
            end,
        }
    }

    pub fn run<F>(&mut self, mut tick_fn: F)
    where
        F: FnMut(&TickData),
    {
        let int_duration = self
            .interval_duration()
            .to_std()
            .expect("interval must be positive");

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

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn start_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
        self.start.with_timezone(&timezone)
    }

    pub fn interval(&self) -> i64 {
        self.int
    }

    pub fn interval_duration(&self) -> TimeDelta {
        TimeDelta::seconds(self.int)
    }

    pub fn duration(&self) -> i64 {
        self.dur
    }

    pub fn elapsed(&self) -> i64 {
        self.cur
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn end_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
        self.end.with_timezone(&timezone)
    }

    pub fn step(&mut self) {
        self.cur += self.int;
        self.state = match self.cur {
            n if n == self.dur => TimerState::Ended(n),
            _ => TimerState::InProgress(self.cur),
        };
    }

    pub fn time_remaining(&self) -> TimeDelta {
        self.end - Utc::now()
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
