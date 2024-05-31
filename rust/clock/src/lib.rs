use std::fmt::{self, Display};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Clock {
    m: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.m / 60, self.m % 60)
    }
}

fn clamp_to_day(minutes: i32) -> i32 {
    minutes.rem_euclid(24 * 60)
}

impl Clock {
    pub const ZERO: Clock = Clock { m: 0 };
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            m: clamp_to_day(clamp_to_day(hours) * 60 + clamp_to_day(minutes)),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.m + minutes)
    }
}
