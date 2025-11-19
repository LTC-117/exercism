use std::fmt::{ Display, Formatter, Result, Debug };
use std::cmp::{ PartialEq };

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

impl Clock {
    pub fn new(hrs: i64, min: i64) -> Clock {
        Clock {
            minutes: (((hrs * HOUR + min) % DAY) + DAY) % DAY
        }
    }

    pub fn add_minutes(&self, min: i64) -> Clock {
        Clock::new(0, self.minutes + min)
    }
}
