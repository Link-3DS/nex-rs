use chrono::{Timelike, Datelike, Utc};

#[derive(Debug)]
struct DateTime {
    second: i32,
    minute: i32,
    hour: i32,
    day: i32,
    month: i32,
    year: i32,

    timestamp: i32,
}

impl DateTime {
    fn new(current: &chrono::DateTime<Utc>) -> DateTime {
        let second = current.second();
        let minute = current.minute();
        let hour = current.hour();
        let day = current.day();
        let month = current.month() as i32;
        let year = current.year();

        let timestamp = (second
            | (minute << 6)
            | (hour << 12)
            | (day << 17)
            | (month << 22)
            | (year << 26)) as i32;

        DateTime {
            timestamp,
            second: timestamp & 63,
            minute: (timestamp >> 6) & 63,
            hour: (timestamp >> 12) & 31,
            day: (timestamp >> 17) & 31,
            month: (timestamp >> 22) & 15,
            year: timestamp >> 26,
        }
    }
}
