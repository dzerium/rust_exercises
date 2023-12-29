pub mod gigasec {
    use unicode_segmentation::UnicodeSegmentation;
    pub use time::PrimitiveDateTime as DateTime;

    const GIGASECOND: i64 = 1_000_000_000;

    // Returns a DateTime one billion seconds after start.
    pub fn after(start: DateTime) -> DateTime {
       start + time::Duration::seconds(GIGASECOND)
    }

    pub fn reverse(input: &str) -> String {
        //input.chars().rev().collect::<String>()
        input.graphemes(true).rev().collect::<String>()
    }
}

pub fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> gigasec::DateTime {
    use time::{Date, Time};
    gigasec::DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
#[test]
fn date() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    assert_eq!(gigasec::after(start_date), dt(2043, 1, 1, 1, 46, 40));
}