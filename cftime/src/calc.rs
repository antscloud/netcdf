use crate::calendars::Calendar;
use crate::datetimes::Datetime;
use crate::durations::DurationUnit;
use crate::parser::ParsedCFTime;

fn add_gregorian_integer(dt: Datetime, duration: DurationUnit, delta: i64) -> Datetime {
    match duration {
        d => todo!("{d:?}"),
    }
}

fn add_360day_integer(dt: Datetime, duration: DurationUnit, delta: i64) -> Datetime {
    todo!()
}

impl ParsedCFTime {
    pub(crate) fn add_scaled_duration_integer(&self, delta: i64) -> Datetime {
        let calendar = self.from.calendar.unwrap_or_default();
        match calendar {
            Calendar::Gregorian | Calendar::Standard => {
                add_gregorian_integer(self.from, self.duration, delta)
            }
            Calendar::ProlepticGregorian => todo!(),
            Calendar::NoLeap => todo!(),
            Calendar::Day365 => todo!(),
            Calendar::AllLeap => todo!(),
            Calendar::Day366 => todo!(),
            Calendar::Julian => todo!(),
            Calendar::Day360 => todo!(),
        }
    }
    pub(crate) fn add_scaled_duration_integers(&self, delta: &[i64]) -> Vec<Datetime> {
        delta
            .iter()
            .copied()
            .map(|delta| self.add_scaled_duration_integer(delta))
            .collect()
    }
    pub(crate) fn add_scaled_duration_float(&self, delta: f64) -> Datetime {
        let calendar = self.from.calendar.unwrap_or_default();
        match calendar {
            Calendar::Gregorian => todo!(),
            Calendar::Standard => todo!(),
            Calendar::ProlepticGregorian => todo!(),
            Calendar::NoLeap => todo!(),
            Calendar::Day365 => todo!(),
            Calendar::AllLeap => todo!(),
            Calendar::Day366 => todo!(),
            Calendar::Julian => todo!(),
            Calendar::Day360 => todo!(),
        }
    }
    pub(crate) fn add_scaled_duration_floats(&self, delta: &[f64]) -> Vec<Datetime> {
        delta
            .iter()
            .copied()
            .map(|delta| self.add_scaled_duration_float(delta))
            .collect()
    }
}
