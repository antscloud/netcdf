#![allow(unused)]
use crate::calendars::Calendar;
//use crate::datetimes::factory::{CFDateFactory, CFDateTimeFactory, CFDates, CFDatetimes};
use crate::datetimes::Datetime;
use crate::datetimes::Timezone;
use crate::durations::{CFDuration, DurationUnit};
// use crate::time::Time;
use crate::datetimes::Time;
use crate::tz::Tz;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, i8, one_of, space1, u32, u8},
    combinator::{all_consuming, map, opt, peek, value},
    number::complete::double,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

/// Parsing error
#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

fn duration<'a>(input: &'a str) -> IResult<&'a str, DurationUnit> {
    #[rustfmt::skip]
    let years = value(
        DurationUnit::Years,
        alt((
            tag("years"),
            tag("common_years"),
            tag("common_year")
        )),
    );
    #[rustfmt::skip]
    let months = value(
        DurationUnit::Months,
        alt((
            tag("months"),
            tag("month")
        ))
    );
    #[rustfmt::skip]
    let days = value(
        DurationUnit::Days,
        alt((
            tag("days"),
            tag("day"),
            tag("d")
        ))
    );
    #[rustfmt::skip]
    let hours = value(
        DurationUnit::Hours,
        alt((
            tag("hours"),
            tag("hour"),
            tag("hrs"),
            tag("hr"),
            tag("h")
        )),
    );
    #[rustfmt::skip]
    let minutes = value(
        DurationUnit::Minutes,
        alt((
            tag("minutes"),
            tag("minute"),
            tag("mins"),
            tag("min")
        )),
    );
    let seconds = value(
        DurationUnit::Seconds,
        alt((
            tag("seconds"),
            tag("second"),
            tag("secs"),
            tag("sec"),
            tag("s"),
        )),
    );
    let milliseconds = value(
        DurationUnit::Milliseconds,
        alt((
            tag("milliseconds"),
            tag("millisecond"),
            tag("millisecs"),
            tag("millisec"),
            tag("msecs"),
            tag("msec"),
            tag("ms"),
        )),
    );
    let microseconds = value(
        DurationUnit::Microseconds,
        alt((
            tag("microseconds"),
            tag("microsecond"),
            tag("microsecs"),
            tag("microsec"),
        )),
    );

    alt((
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
        milliseconds,
        microseconds,
    ))(input)
}

fn date<'a>(input: &'a str, calendar: Option<Calendar>) -> IResult<&'a str, crate::datetimes::Date> {
    map(
        tuple((i32, tag("-"), u32, tag("-"), u32)),
        |(year, _, month, _, day)| crate::datetimes::Date { year, month, day },
    )(input)
}
fn time(input: &str) -> IResult<&str, Time> {
    let hms = map(
        tuple((i32, tag(":"), u32, tag(":"), double)),
        |(hour, _, minute, _, second)| {
            let (second, rest) = (second.trunc(), second.fract());
            let nanosecond = rest * 1e9;

            Time {
                hour: hour as u32,
                minute: minute as u32,
                second: Some(second as u32),
                nanosecond: Some(nanosecond as u64),
            }
        },
    );

    let hm = map(separated_pair(i32, tag(":"), u32), |(hour, minute)| Time {
        hour: hour as u32,
        minute: minute,
        ..Default::default()
    });

    alt((hms, hm))(input)
}

fn timezone(input: &str) -> IResult<&str, crate::datetimes::Timezone> {
    println!("{input}");
    let hm = map(
        preceded(opt(tag("+")), separated_pair(i8, tag(":"), u8)),
        |(hour, minute)| crate::datetimes::Timezone { hour, minute },
    );
    let z = value(Timezone::utc(), tag("Z"));
    let utc = value(Timezone::utc(), tag("UTC"));
    alt((hm, z, utc))(input)
}

fn datetime<'a>(input: &'a str, calendar: Option<Calendar>) -> IResult<&'a str, Datetime> {
    fn space1_or_t(input: &str) -> IResult<&str, ()> {
        alt((value((), space1), value((), tag("T"))))(input)
    }
    let tz = map(
        separated_pair(
            separated_pair(|x| date(x, calendar), space1_or_t, time),
            space1,
            timezone,
        ),
        |((date, time), tz)| Datetime {
            date,
            time: Some(time),
            tz: Some(tz),
            calendar,
        },
    );

    let no_tz = map(
        separated_pair(|x| date(x, calendar), space1_or_t, time),
        |(date, time)| Datetime {
            date,
            time: Some(time),
            calendar,
            ..Default::default()
        },
    );

    let date_with_tz = map(
        separated_pair(|x| date(x, calendar), space1, timezone),
        |(date, tz)| Datetime {
            date,
            tz: Some(tz),
            calendar,
            ..Default::default()
        },
    );

    let date_time_no_space_tz = map(
        separated_pair(
            separated_pair(|x| date(x, calendar), space1_or_t, time),
            peek(one_of("+-Z")),
            timezone,
        ),
        |((date, time), tz)| Datetime {
            date,
            time: Some(time),
            tz: Some(tz),
            calendar,
        },
    );

    let only_date = map(
        |x| date(x, calendar),
        |date| Datetime {
            date,
            calendar,
            ..Default::default()
        },
    );

    alt((tz, date_time_no_space_tz, no_tz, date_with_tz, only_date))(input)
}

#[derive(Debug)]
pub struct ParsedCFTime {
    pub duration: DurationUnit,
    pub from: Datetime,
}

/// Parse a CF compatible string into two components
pub fn cf_parser(input: &str, calendar: Option<Calendar>) -> Result<ParsedCFTime, ParseError> {
    let since = tuple((space1, tag("since"), space1));
    let x = all_consuming(separated_pair(
        duration,
        since,
        |x| datetime(x, calendar),
    ))(input)
    .map(|(_, (duration, from))| ParsedCFTime { duration, from })
    .map_err(|e| ParseError(format!("{}", e)));
    x
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(input: &str) -> ParsedCFTime {
        cf_parser(input, None).unwrap()
    }

    #[test]
    fn cf_conventions_document() {
        parse("days since 1990-1-1 0:0:0");
        parse("seconds since 1992-10-8 15:15:42.5 -6:00");
        parse("days since 1-7-15 0:0:0");
        parse("days since 1-1-1 0:0:0");
    }

    #[test]
    fn cftime_py_setup() {
        parse("hours since 0001-01-01 00:00:00");
        parse("hours since 0001-01-01 00:00:00");
        parse("hours since 0001-01-01 00:00:00 -06:00");
        parse("seconds since 0001-01-01 00:00:00");
        parse("days since 1600-02-28 00:00:00");
        parse("days since 1600-02-29 00:00:00");
        // parse("days since 1600-02-30 00:00:00");
        parse("hours since 1000-01-01 00:00:00");
        parse("seconds since 1970-01-01T00:00:00Z");
        parse("days since  850-01-01 00:00:00");
        parse("hours since 0001-01-01 00:00:00");
        parse("days since 1600-02-28 00:00:00");
    }

    #[test]
    fn cftime_py_tz_naive() {
        let d_check = ["1582-10-15 00:00:00", "1582-10-15 12:00:00"];
        for d in d_check {
            parse(&format!("day since {}", d));
        }
    }

    #[test]
    fn cftime_py() {
        parse("days since 1000-01-01");
        parse("seconds since 1970-01-01T00:00:00Z");
        parse("hours since 2013-12-12T12:00:00");
        parse("hours since 1682-10-15 -07:00");
        parse("hours since 1682-10-15 -07:00:12");
        parse("hours since 1682-10-15T-07:00:12");
        parse("hours since 1682-10-15 -07:00 UTC");
        parse("hours since 2000-01-01 22:30+04:00");
        parse("hours since 2000-01-01 11:30-07:00");
        parse("hours since 2000-01-01 15:00-03:30");
    }

    #[test]
    fn etc() {
        parse("seconds since 1992-10-8 15:15:42.5Z");
        parse("seconds since 1992-10-8 15:15:42Z");
    }

    #[test]
    fn add_some() {
        let d = parse("hours since 2000-01-01 11:30-07:00");

        let _d = d.add_scaled_duration_integer(10);
        let _d = d.add_scaled_duration_integers(&[10, 20, 40]);

        let _d = d.add_scaled_duration_float(10.0);
        let _d = d.add_scaled_duration_floats(&[10.0, 20.0]);
    }

    #[test]
    fn add_some_using_trait() {
        let d = parse("hours since 2000-01-01 11:30-07:00");
        use crate::traits::AddableDuration;

        let _d = d.add_scaled_duration(10);
        let _d = d.add_scaled_duration(&[10, 20, 40][..]);

        let _d = d.add_scaled_duration(10.0);
        let _d = d.add_scaled_duration(&[10.0, 20.0][..]);
    }
}
