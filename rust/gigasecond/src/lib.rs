use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let mut expected: DateTime = DateTime::new(start.date(), start.time());

    let gigaseconds = Duration::new(1_000_000_000, 0);

    expected += gigaseconds;

    expected
}
