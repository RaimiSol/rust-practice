use chrono::{DateTime, Duration, Utc};
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasec = Duration::seconds(1000000000);
    start + gigasec
}
