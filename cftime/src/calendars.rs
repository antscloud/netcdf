use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Calendar {
    Gregorian, // alias of Standard
    Standard,
    ProlepticGregorian,
    NoLeap, // 365 days
    Day365,
    AllLeap, // 366 days
    Day366,
    Julian,
    Day360,
}

const DEFAULT_CAL: Calendar = Calendar::ProlepticGregorian;

impl Default for Calendar {
    fn default() -> Self {
        DEFAULT_CAL
    }
}

impl fmt::Display for Calendar {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let name = match *self {
            Self::Gregorian => "Gregorian",
            Self::Standard => "Standard",
            Self::ProlepticGregorian => "Proleptic Gregorian",
            Self::NoLeap | Self::Day365 => "No Leap",
            Self::AllLeap | Self::Day366 => "All Leap",
            Self::Julian => "Julian",
            Self::Day360 => "360 Day",
        };
        write!(f, "{name}")
    }
}
