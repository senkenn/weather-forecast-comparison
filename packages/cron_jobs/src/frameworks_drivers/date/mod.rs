pub mod date {
    #[derive(Clone)]
    pub struct DateWrapper {
        pub date: chrono::DateTime<chrono::Local>,
    }
}

use anyhow::Result;
use chrono::{Datelike, Local, TimeZone};

use date::DateWrapper;

impl date::DateWrapper {
    pub fn new(year: i32, month: u32, day: u32) -> Result<DateWrapper> {
        let date = match Local.with_ymd_and_hms(year, month, day, 0, 0, 0) {
            chrono::LocalResult::Single(date) => date,
            _ => return Err(anyhow::anyhow!("Invalid date")),
        };

        Ok(DateWrapper { date })
    }

    pub fn now() -> DateWrapper {
        DateWrapper { date: Local::now() }
    }

    pub fn get_yesterday(&self) -> DateWrapper {
        DateWrapper {
            date: self.date - chrono::Duration::days(1),
        }
    }
    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_day(&self) -> u32 {
        self.date.day()
    }
}
