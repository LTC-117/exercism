use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let expected: DateTime = DateTime::new(start.date(), start.time());

    expected
}
