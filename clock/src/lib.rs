use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { minutes: (self.minutes + minutes) % 60, hours: (self.hours + minutes / 60) as i32}
    }

    pub fn to_string(&self) -> String{
        format!("{:00$}{:01$}", self.hours as usize, self.minutes as usize)
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}