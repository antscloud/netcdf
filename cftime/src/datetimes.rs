//pub mod all_leap;
pub mod day360;
//pub mod factory;
//pub mod julian;
//pub mod no_leap;
//pub mod prolecpticgregorian;

use crate::calendars::Calendar;
use crate::date::Date;
use crate::time::Time;
use crate::tz::Timezone;
use crate::durations::Duration;
use crate::traits::DurationAddable;

#[derive(Copy, Clone, Debug, Default)]
pub struct Datetime {
    pub date: Date,
    pub time: Option<Time>,
    pub tz: Option<Timezone>,
    pub calendar: Option<Calendar>,
}

impl Datetime {
    pub fn add_duration(&self, duration: Duration) -> Result<Self, ()> {
        match self.calendar.unwrap_or_default() {
            Calendar::Day360 => {
                let dt: day360::Datetime360Day = (*self).try_into()?;
                let next_dt = dt.add_duration(duration);
                return Ok(next_dt.into());
            },
            Calendar::Gregorian => {
                todo!();
            },
            Calendar::Standard => {
                todo!();
            },
            Calendar::ProlepticGregorian => {
                todo!();
            },
            Calendar::NoLeap => {
                todo!();
            },
            Calendar::Day365 => {
                todo!();
            },
            Calendar::AllLeap => {
                todo!();
            },
            Calendar::Day366 => {
                todo!();
            },
            Calendar::Julian => {
                todo!();
            },
        }
    }
}
