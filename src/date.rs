use std::fmt::Display;

use chrono::prelude::*;
use chrono_tz::Tz;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub struct TimezoneData(pub String, pub String);

impl Display for TimezoneData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tz_str = &self.0.split('/').last().unwrap().to_string();
        let dt_str = &self.1;
        write!(f, "{}: {}", tz_str, dt_str)
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
                            .with_ymd_and_hms(
                                date.year(),
                                date.month(),
                                date.day(),
                                date.hour(),
                                date.minute(),
                                0,
                            )
                            .unwrap()
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

#[cfg(test)]
mod tests {
    use super::EventDate;

    #[test]
    fn get_event_dates_by_timezone() {
        // https://www.timeanddate.com/worldclock/converter.html?iso=20211123T140000&p1=248&p2=37&p3=107
        // Tokyo, Japan    	Tue, 23 Nov 2021 at 23:00 JST
        // Berlin, Germany 	Tue, 23 Nov 2021 at 15:00 CET
        // Istanbul, Turkey	Tue, 23 Nov 2021 at 17:00 TRT
        let event_dates = EventDate::new(
            "2021-11-23 23:00".to_string(),
            Some("Japan".to_string()),
            vec!["Europe/Istanbul".to_string(), "Europe/Berlin".to_string()],
        )
        .unwrap()
        .get_dates_by_timezones();
        assert_eq!(event_dates.len(), 2);
        assert_eq!(event_dates[0].0, "Europe/Istanbul");
        assert_eq!(event_dates[0].1, "2021-11-23 17:00");
        assert_eq!(event_dates[1].0, "Europe/Berlin");
        assert_eq!(event_dates[1].1, "2021-11-23 15:00");
    }
}
