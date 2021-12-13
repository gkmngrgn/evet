use chrono::prelude::*;
use chrono_tz::Tz;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub struct TimezoneData(pub String, pub String);

impl TimezoneData {
    pub fn to_string(self) -> String {
        let tz_str = self.0.split('/').last().unwrap();
        let dt_str = self.1;
        format!("{}: {}", tz_str, dt_str)
    }
}

pub struct EventDate {
    date: DateTime<Local>,
    timezones: Vec<String>,
}

impl EventDate {
    pub fn new(
        date_str: String,
        local_timezone: Option<String>,
        timezones: Vec<String>,
    ) -> Result<Self, &'static str> {
        let tz = Local::now().timezone();
        match tz.datetime_from_str(&date_str, DATE_FORMAT) {
            Ok(date) => {
                let date = match local_timezone {
                    Some(tz_str) => {
                        let tz: Tz = tz_str.parse().unwrap();
                        let date = tz
                            .ymd(date.year(), date.month(), date.day())
                            .and_hms(date.hour(), date.minute(), 0)
                            .naive_utc();
                        Local.from_utc_datetime(&date)
                    }
                    None => date.with_timezone(&Local),
                };
                Ok(Self { date, timezones })
            }
            Err(_) => Err("Error: The date format should be '%Y-%m-%d %H:%M'."),
        }
    }

    pub fn get_dates_by_timezones(self) -> Vec<TimezoneData> {
        let mut timezone_list = vec![];
        for tz_str in self.timezones {
            let tz: Tz = tz_str.parse().unwrap();
            let dt = self.date.with_timezone(&tz);
            timezone_list.push(TimezoneData(
                dt.timezone().to_string(),
                dt.format(DATE_FORMAT).to_string(),
            ));
        }
        timezone_list
    }
}
