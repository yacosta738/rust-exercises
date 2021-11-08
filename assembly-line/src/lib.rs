// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    println!("calculate hourly production rate at speed: {}", speed);

    const CARS_PER_HOUR: u16 = 221;
    let mut speed_cars_per_hour: f64 = (speed as u16 * CARS_PER_HOUR) as f64;
    match speed {
        1..=4 => speed_cars_per_hour *= 1.0,
        5..=8 => speed_cars_per_hour *= 0.9,
        9..=10 => speed_cars_per_hour *= 0.77,
        _ => speed_cars_per_hour *= 0.0,
    }
    speed_cars_per_hour
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    println!("calculate the amount of working items at speed: {}", speed);
    (production_rate_per_hour(speed) / 60.0) as u32
}
