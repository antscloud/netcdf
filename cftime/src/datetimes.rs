//pub mod all_leap;
//pub mod day360;
//pub mod factory;
//pub mod julian;
//pub mod no_leap;
//pub mod prolecpticgregorian;
use crate::calendars::Calendar;

#[derive(Copy, Clone, Debug, Default)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}
#[derive(Copy, Clone, Debug, Default)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: Option<u32>,
    pub nanosecond: Option<u64>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Timezone {
    pub hour: i8,
    pub minute: u8,
}

impl Timezone {
    pub const fn utc() -> Self {
        Self { hour: 0, minute: 0 }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Datetime {
    pub date: Date,
    pub time: Option<Time>,
    pub tz: Option<Timezone>,
    pub calendar: Option<Calendar>,
}
