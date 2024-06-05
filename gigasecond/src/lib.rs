use time::{Duration, PrimitiveDateTime as DateTime};

const GIGA_SEC: i64 = 1_000_000_000;
// const MEGA_SEC: i64 = 1_000_000;
// const KILO_SEC: i64 = 1_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let later_date = start + Duration::seconds(GIGA_SEC);
    later_date
}
