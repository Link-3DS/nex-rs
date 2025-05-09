use chrono::{Datelike, Timelike, Utc};

#[derive(Debug)]
pub struct DateTime {
    timestamp: i32,
}

impl DateTime {
    pub fn new(current: &chrono::DateTime<Utc>) -> DateTime {
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

        DateTime { timestamp }
    }
}