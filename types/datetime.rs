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

        let timestamp = ((second as u32)
            | ((minute as u32) << 6)
            | ((hour as u32) << 12)
            | ((day as u32) << 17)
            | ((month as u32) << 22)
            | ((year as u32) << 26)) as i32;

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
