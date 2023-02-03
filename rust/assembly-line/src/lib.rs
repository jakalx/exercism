fn in_range(lo: u8, hi: u8, x: u8) -> bool {
    x >= lo && x <= hi
}

fn success_rate_at_speed(speed: u8) -> f64 {
    if in_range(1, 4, speed) {
        return 1.0;
    }

    if in_range(5, 8, speed) {
        return 0.9;
    }

    if in_range(9, 10, speed) {
        return 0.77;
    }

    0.0
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (221.0 * f64::from(speed)) * success_rate_at_speed(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
