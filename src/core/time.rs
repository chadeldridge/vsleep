use std::thread;

use chrono::{DateTime, Local, TimeDelta, TimeZone, Utc};

pub enum TimerState {
    Ready(i64),
    InProgress(i64),
    Ended(i64),
}

pub struct TickData {
    pub elapsed: i64,
    pub remaining: i64,
    pub now: DateTime<Local>,
}

pub struct Timer {
    state: TimerState,
    start: DateTime<Utc>,
    int: i64,
    dur: i64,
    cur: i64,
    end: DateTime<Utc>,
}

impl Timer {
    pub fn new(duration: i64, interval: i64) -> Timer {
        let int = match interval {
            0 => 1,
            _ => interval,
        };
        let start = Utc::now();
        let end = start + TimeDelta::seconds(duration);

        Timer {
            state: TimerState::Ready(duration),
            start,
            int,
            dur: duration,
            cur: 0,
            end,
        }
    }

    pub fn start<F>(&mut self, mut tick_fn: F)
    where
        F: FnMut(&TickData),
    {
        let int_duration = self.get_interval_duration()
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

    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    pub fn get_start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn get_start_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
        self.start.with_timezone(&timezone)
    }

    pub fn get_interval(&self) -> i64 {
        self.int
    }

    pub fn get_interval_duration(&self) -> TimeDelta {
        TimeDelta::seconds(self.int)
    }

    pub fn get_duration(&self) -> i64 {
        self.dur
    }

    pub fn elapsed(&self) -> i64 {
        self.cur
    }

    pub fn get_end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn get_end_tz<Tz: TimeZone>(&self, timezone: Tz) -> DateTime<Tz> {
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
