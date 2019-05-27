use chrono::{DateTime, Utc,Duration};

const GIGA_SECOND : i64 = 1000000000;
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(GIGA_SECOND)
}
