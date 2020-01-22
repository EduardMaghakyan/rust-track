use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn calculate_hours_and_minutes(mut hours: i32, mut minutes: i32) -> (i32, i32) {
    if minutes <= -60 || minutes >= 60 {
        let delta_hours = minutes / 60;
        minutes -= delta_hours * 60;
        hours += delta_hours;
    }

    // go back in minutes 12:-40 -> 11:20
    if minutes < 0 {
        hours -= 1;
        minutes += 60;
    }

    if hours < 0 {
        // e.g. hours = -91
        // days = hours // 24 = -91 // 24 = -3
        // 24 + (-91) - (days * 24) => -67 - (-3 * 24) => -67 + 72 => 5
        hours = 24 + hours - ((hours / 24) * 24);
    }

    // flip days and ensure 24 -> 0
    if hours >= 24 {
        hours -= (hours / 24) * 24;
    }

    (hours, minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = calculate_hours_and_minutes(hours, minutes);
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{h:>0width$}:{m:>0width$}",
            h = self.hours,
            m = self.minutes,
            width = 2
        )
    }
}
