

use std::fmt; 


#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            hours: (hours + (minutes as f32/60 as f32).floor() as i32).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { 
            minutes: (self.minutes + minutes).rem_euclid(60) , 
            hours: (self.hours  + ((self.minutes + minutes) as f32 / 60 as f32).floor() as i32).rem_euclid(24) }
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}