use crate::constants;

#[derive(Copy, Clone, Debug)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: constants::UNIX_DEFAULT_YEAR,
            month: constants::UNIX_DEFAULT_MONTH,
            day: constants::UNIX_DEFAULT_DAY,
        }
    }
}
