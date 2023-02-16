#![allow(unused)]
use crate::calendars::Calendars;
use crate::constants;
use crate::durations::CFDuration;
use crate::macros::{impl_date_display, impl_dt_display, impl_getter};
use crate::time::Time;
use crate::traits::{DateLike, DateTimeLike, IsLeap};
use crate::tz::Tz;
use chrono::Datelike;
use num_integer::div_mod_floor;
use std::{
    fmt,
    ops::{Add, Sub},
};

#[derive(Debug, Copy, Clone)]
pub struct DateAllLeap {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateAllLeap {
    const DAYS_PER_MONTH_LEAP: [u32; 12] = constants::DAYS_PER_MONTH_LEAP;
    const CUM_DAYS_PER_MONTH_LEAP: [u32; 13] = constants::CUM_DAYS_PER_MONTH_LEAP;
    const CALENDAR: Calendars = Calendars::AllLeap;
}
#[allow(unused_variables)]
impl IsLeap for DateAllLeap {
    fn is_leap(year: i32) -> bool {
        true
    }
}

impl DateAllLeap {
    pub fn new(year: i32, month: u32, day: u32) -> DateAllLeap {
        if (month > 12) | (month < 1) {
            panic!("Month should be between 1 and 12. Found {month}")
        }
        let max_day = DateAllLeap::DAYS_PER_MONTH_LEAP[(month - 1) as usize];

        if day > max_day {
            panic!(
                "Day can not exceed {max_day} for {} of the year {year} and {}",
                constants::MONTHS[(month - 1) as usize],
                DateAllLeap::CALENDAR
            )
        }
        Self {
            year: year,
            month: month,
            day: day,
        }
    }
}
impl Default for DateAllLeap {
    fn default() -> Self {
        Self {
            year: constants::UNIX_DEFAULT_YEAR,
            month: constants::UNIX_DEFAULT_MONTH,
            day: constants::UNIX_DEFAULT_DAY,
        }
    }
}
impl DateLike for DateAllLeap {
    fn num_days_from_ce(&self) -> i32 {
        let mut days: i32 = 0;
        days += (self.year - constants::UNIX_DEFAULT_YEAR) * 366;
        days += constants::CUM_DAYS_PER_MONTH_LEAP[(self.month - 1) as usize] as i32;
        days += (self.day - 1) as i32;
        days
    }
    fn num_hours_from_ce(&self) -> i32 {
        self.num_days_from_ce() * 24
    }
    fn num_minutes_from_ce(&self) -> i32 {
        self.num_hours_from_ce() * 60
    }
    fn num_seconds_from_ce(&self) -> i32 {
        self.num_minutes_from_ce() * 60
    }
    fn num_nanoseconds_from_ce(&self) -> i64 {
        ((self.num_seconds_from_ce() as f64) * 1e6) as i64
    }

    fn from_timestamp(seconds: i32) -> DateAllLeap {
        let (nb_days, _) = div_mod_floor(seconds, constants::SECS_PER_DAY as i32);
        let (nb_year, mut remaining_days) = div_mod_floor(nb_days, 366);

        remaining_days += constants::UNIX_DEFAULT_DAY as i32;
        let mut month: u32 = constants::UNIX_DEFAULT_MONTH;
        for v in DateAllLeap::DAYS_PER_MONTH_LEAP.iter() {
            if remaining_days - (*v as i32) >= 0 && ((month + 1) <= 12) {
                remaining_days -= *v as i32;
                month += 1
            } else {
                break;
            }
        }
        DateAllLeap::new(
            constants::UNIX_DEFAULT_YEAR + nb_year,
            month,
            (remaining_days) as u32,
        )
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct DateTimeAllLeap {
    pub date: DateAllLeap,
    pub time: Time,
    pub tz: Tz,
}

impl DateTimeAllLeap {
    pub fn new(date: DateAllLeap, time: Time, tz: Tz) -> DateTimeAllLeap {
        DateTimeAllLeap {
            date: date,
            time: time,
            tz: tz,
        }
    }
}
impl DateTimeLike for DateTimeAllLeap {
    fn from_hms(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            date: DateAllLeap::new(
                constants::UNIX_DEFAULT_YEAR,
                constants::UNIX_DEFAULT_MONTH,
                constants::UNIX_DEFAULT_DAY,
            ),
            time: Time::new(hour, minute, second, 0),
            tz: Tz { hour: 0, minute: 0 },
        }
    }
    fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self {
            date: DateAllLeap::new(year, month, day),
            time: Time::new(0, 0, 0, 0),
            tz: Tz { hour: 0, minute: 0 },
        }
    }
    fn from_timestamp(seconds: i32) -> Self {
        Self {
            date: DateAllLeap::from_timestamp(seconds),
            time: Time::from_timestamp(seconds),
            tz: Tz { hour: 0, minute: 0 },
        }
    }
    fn num_hours_from_ce(&self) -> i32 {
        self.date.num_hours_from_ce() + (self.time.num_hours() as i32)
    }
    fn num_minutes_from_ce(&self) -> i32 {
        self.date.num_minutes_from_ce() + (self.time.num_minutes() as i32)
    }
    fn num_seconds_from_ce(&self) -> i32 {
        self.date.num_seconds_from_ce() + (self.time.num_seconds() as i32)
    }
    fn num_nanoseconds_from_ce(&self) -> i64 {
        self.date.num_nanoseconds_from_ce() + (self.time.num_nanoseconds() as i64)
    }
}

/// As nanos field is private this is a solution to emulate it
fn _get_real_nano_field(duration: CFDuration) -> i64 {
    let chrono_time = chrono::Duration::seconds(duration.num_seconds());
    let ns = (duration
        - CFDuration {
            duration: chrono_time,
            calendar: duration.calendar,
        })
    .num_nanoseconds()
    .unwrap();
    ns
}

impl Add<CFDuration> for DateTimeAllLeap {
    type Output = Self;
    fn add(self, other: CFDuration) -> Self {
        let ns = _get_real_nano_field(other);
        let mut dt = DateTimeAllLeap::from_timestamp(
            self.num_seconds_from_ce() + other.num_seconds() as i32,
        );
        dt.time.nanosecond = ns as u64;
        dt
    }
}

impl Sub<CFDuration> for DateTimeAllLeap {
    type Output = Self;
    fn sub(self, other: CFDuration) -> Self {
        let ns = _get_real_nano_field(other);
        let mut timestamp = self.num_seconds_from_ce() - other.num_seconds() as i32;
        if ns > 0 {
            timestamp -= 1
        }
        let mut dt = DateTimeAllLeap::from_timestamp(timestamp);
        if ns > 0 {
            dt.time.nanosecond = (constants::MAX_NS - other.num_nanoseconds().unwrap()) as u64;
        }
        dt
    }
}

impl_getter!(DateAllLeap);
impl_date_display!(DateAllLeap);
impl_dt_display!(DateTimeAllLeap);

#[cfg(test)]
mod test {
    use super::*;
    use crate::durations::CFDuration;
    #[test]
    fn test_add_duration_to_datetime() {
        let dt = DateTimeAllLeap::from_timestamp(0);
        let dur = CFDuration::days(1, Calendars::ProlepticGregorian);
        let new_dt = dt + dur;
        assert_eq!(new_dt.date.year, 1970);
        assert_eq!(new_dt.date.month, 01);
        assert_eq!(new_dt.date.day, 02);
        assert_eq!(new_dt.time.hour, 00);
        assert_eq!(new_dt.time.minute, 00);
        assert_eq!(new_dt.time.second, 00);
        let dt = DateTimeAllLeap::from_timestamp(0);
        let dur = CFDuration::milliseconds(1, Calendars::ProlepticGregorian);
        let new_dt = dt + dur;
        assert_eq!(new_dt.date.year, 1970);
        assert_eq!(new_dt.date.month, 01);
        assert_eq!(new_dt.date.day, 01);
        assert_eq!(new_dt.time.hour, 00);
        assert_eq!(new_dt.time.minute, 00);
        assert_eq!(new_dt.time.second, 00);
        assert_eq!(new_dt.time.nanosecond, 1000000);
    }
    #[test]
    fn test_sub_duration_to_datetime() {
        let dt = DateTimeAllLeap::from_timestamp(0);
        let dur = CFDuration::days(1, Calendars::ProlepticGregorian);
        let new_dt = dt - dur;
        println!("{new_dt}");
        assert_eq!(new_dt.date.year, 1969);
        assert_eq!(new_dt.date.month, 12);
        assert_eq!(new_dt.date.day, 31);
        assert_eq!(new_dt.time.hour, 00);
        assert_eq!(new_dt.time.minute, 00);
        assert_eq!(new_dt.time.second, 00);
        let dt = DateTimeAllLeap::from_timestamp(0);
        let dur = CFDuration::milliseconds(1, Calendars::ProlepticGregorian);
        let new_dt = dt - dur;
        assert_eq!(new_dt.date.year, 1969);
        assert_eq!(new_dt.date.month, 12);
        assert_eq!(new_dt.date.day, 31);
        assert_eq!(new_dt.time.hour, 23);
        assert_eq!(new_dt.time.minute, 59);
        assert_eq!(new_dt.time.second, 59);
        assert_eq!(new_dt.time.nanosecond, 999000000);
    }

    #[test]
    fn test_from_timestam() {
        let dt = DateTimeAllLeap::from_timestamp(0);
        println!("{dt}");
        assert_eq!(dt.date.year, 1970);
        assert_eq!(dt.date.month, 01);
        assert_eq!(dt.date.day, 01);
        assert_eq!(dt.time.hour, 00);
        assert_eq!(dt.time.minute, 00);
        assert_eq!(dt.time.second, 00);
        // Bug found for this value
        let dt = DateTimeAllLeap::from_timestamp(-86400);
        println!("{dt}");
        assert_eq!(dt.date.year, 1969);
        assert_eq!(dt.date.month, 12);
        assert_eq!(dt.date.day, 31);
        assert_eq!(dt.time.hour, 00);
        assert_eq!(dt.time.minute, 00);
        assert_eq!(dt.time.second, 00);
        let dt = DateTimeAllLeap::from_timestamp(-1);
        assert_eq!(dt.date.year, 1969);
        assert_eq!(dt.date.month, 12);
        assert_eq!(dt.date.day, 31);
        assert_eq!(dt.time.hour, 23);
        assert_eq!(dt.time.minute, 59);
        assert_eq!(dt.time.second, 59);
        let dt = DateTimeAllLeap::from_timestamp(1000000);
        println!("{dt}");
        assert_eq!(dt.date.year, 1970);
        assert_eq!(dt.date.month, 01);
        assert_eq!(dt.date.day, 12);
        assert_eq!(dt.time.hour, 13);
        assert_eq!(dt.time.minute, 46);
        assert_eq!(dt.time.second, 40);
        let dt = DateTimeAllLeap::from_timestamp(1658876523);
        println!("{dt}");
        assert_eq!(dt.date.year, 2022);
        assert_eq!(dt.date.month, 06);
        assert_eq!(dt.date.day, 16);
        assert_eq!(dt.time.hour, 23);
        assert_eq!(dt.time.minute, 02);
        assert_eq!(dt.time.second, 03);
        let dt = DateTimeAllLeap::from_timestamp(-1658876523);
        println!("{dt}");
        assert_eq!(dt.date.year, 1917);
        assert_eq!(dt.date.month, 07);
        assert_eq!(dt.date.day, 17);
        assert_eq!(dt.time.hour, 00);
        assert_eq!(dt.time.minute, 57);
        assert_eq!(dt.time.second, 57);
    }
}