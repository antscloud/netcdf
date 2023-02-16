use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Timezone {
    pub hour: i8,
    pub minute: u8,
}

impl Timezone {
    pub const fn utc() -> Self {
        Self { hour: 0, minute: 0 }
    }
}

impl fmt::Display for Timezone {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "+{:02}:{:02}", self.hour, self.minute)
    }
}
