// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PRODUCED_CARS_PER_HOUR: i32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let sucess_rate: f64 = if speed <= 4 { 1.0 } else if speed <= 8 { 0.90 } else { 0.77 };
    PRODUCED_CARS_PER_HOUR as f64 * speed as f64 * sucess_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars_produced: f64 = production_rate_per_hour(speed);
    (cars_produced as u32 / 60 as u32).into()
}
