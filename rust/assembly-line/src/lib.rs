// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
const PRODUCTION_RATE_BASE: i32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut success_rate: f64 = 0.0;
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else if speed >= 9 && speed <= 10 {
        success_rate = 0.77;
    }
    return success_rate * speed as f64 * PRODUCTION_RATE_BASE as f64;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0).floor() as u32;
}
