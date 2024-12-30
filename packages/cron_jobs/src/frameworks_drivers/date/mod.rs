pub mod date {
    use chrono::{DateTime, Datelike, Local};

    #[derive(Clone)]
    pub struct DateWrapper {
        pub date: DateTime<Local>,
    }

    impl DateWrapper {
        pub fn new() -> DateWrapper {
            let date = Local::now();
            DateWrapper { date }
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
}
