#![allow(unused)]
use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use crate::parser::ParsedCFTime;


impl ParsedCFTime {
    pub(crate) fn add_scaled_duration_integer(&self, delta: i64) -> Self {
        todo!()
    }
    pub(crate) fn add_scaled_duration_integers(&self, delta: &[i64]) -> Vec<Self> {
        delta.iter().copied().map(|delta| self.add_scaled_duration_integer(delta)).collect()
    }
    pub(crate) fn add_scaled_duration_float(&self, delta: f64) -> Self {
        todo!()
    }
    pub(crate) fn add_scaled_duration_floats(&self, delta: &[f64]) -> Vec<Self> {
        delta.iter().copied().map(|delta| self.add_scaled_duration_float(delta)).collect()
    }
}

pub trait AddableDuration<T> {
    type Output;
    fn add_scaled_duration(&self, t: T) -> Self::Output;
}

impl AddableDuration<i64> for ParsedCFTime {
    type Output = Self;
    fn add_scaled_duration(&self, delta: i64) -> Self::Output {
        self.add_scaled_duration_integer(delta)
    }
}
impl<'a> AddableDuration<&'a [i64]> for ParsedCFTime {
    type Output = Vec<Self>;
    fn add_scaled_duration(&self, delta: &'a [i64]) -> Self::Output {
        self.add_scaled_duration_integers(delta)
    }
}

impl AddableDuration<f64> for ParsedCFTime {
    type Output = Self;
    fn add_scaled_duration(&self, delta: f64) -> Self::Output {
        self.add_scaled_duration_float(delta)
    }
}
impl<'a> AddableDuration<&'a [f64]> for ParsedCFTime {
    type Output = Vec<Self>;
    fn add_scaled_duration(&self, delta: &'a [f64]) -> Self::Output {
        self.add_scaled_duration_floats(delta)
    }
}

/*
use crate::{
    calendars::Calendars,
    datetimes::{
        day360::{Date360Day, DateTime360Day},
        factory::CFDatetimes,
    },
    durations::CFDuration,
    parser::cf_parser,
};
pub trait IsLeap {
    fn is_leap(year: i32) -> bool;
}

pub trait CFTimeEncoder {
    fn encode(unit: &str, calendar: Calendars);
}
pub trait CFTimeDecoder {
    fn decode(self, unit: &str, calendar: Option<Calendars>);
}

pub trait DateLike: Debug {
    fn num_days_from_ce(&self) -> i32;
    fn num_hours_from_ce(&self) -> i32;
    fn num_minutes_from_ce(&self) -> i32;
    fn num_seconds_from_ce(&self) -> i32;
    fn num_nanoseconds_from_ce(&self) -> i64;
    fn from_timestamp(seconds: i32) -> Self
    where
        Self: Sized;
}
pub trait DateTimeLike: Debug {
    fn from_hms(hour: u32, minute: u32, second: u32) -> Self
    where
        Self: Sized;
    fn from_ymd(year: i32, month: u32, day: u32) -> Self
    where
        Self: Sized;
    fn from_timestamp(seconds: i32) -> Self
    where
        Self: Sized;
    fn num_hours_from_ce(&self) -> i32;
    fn num_minutes_from_ce(&self) -> i32;
    fn num_seconds_from_ce(&self) -> i32;
    fn num_nanoseconds_from_ce(&self) -> i64;
}
*/
