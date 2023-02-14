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

#[derive(Copy, Clone, Debug, Default)]
pub struct Datetime {
    pub date: Date,
    pub time: Option<Time>,
    pub tz: Option<Timezone>,
    pub calendar: Option<Calendar>,
}
