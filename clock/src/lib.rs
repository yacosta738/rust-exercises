use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!(
            "Construct a new Clock from {} hours and {} minutes",
            hours,
            minutes
        );
        let time = Self::calculate_time(hours, minutes);
        Clock {
            hours: time.0,
            minutes: time.1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("Add {} minutes to existing Clock time", minutes);
        let time = Self::calculate_time(self.hours, self.minutes + minutes);
        Clock {
            hours: time.0,
            minutes: time.1,
        }
    }

    fn calculate_time(hours: i32, minutes: i32) -> (i32, i32) {
        let new_minutes = minutes;
        let new_hours = hours;
        if new_minutes < 0 {
            return Self::calculate_time(new_hours - 1, new_minutes + 60);
        }
        if new_hours < 0 {
            return Self::calculate_time(new_hours + 24, new_minutes);
        }
        if new_minutes > 59 {
            return Self::calculate_time(new_hours + 1, new_minutes - 60);
        }
        if new_hours > 23 {
            return Self::calculate_time(new_hours - 24, new_minutes);
        }
        (new_hours, new_minutes)
    }
    // implementation of campares two clocks
    pub fn is_equal(&self, other: &Self) -> bool {
        println!("Compare two clocks");
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}
