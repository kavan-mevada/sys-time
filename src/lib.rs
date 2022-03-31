#[derive(Debug, Default)]
pub struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    weekday: u8,
    month: u8,
    year: u64,
    timestamp: u64,
}

use std::time::SystemTime;

impl DateTime {
    /// Create a new `DateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use sys_time::DateTime;
    /// assert!(DateTime::now_utc().year() >= 2022);
    /// ```
    #[inline(always)]
    pub fn now_utc() -> Self {
        SystemTime::now().into()
    }

    #[inline(always)]
    pub fn hour(&self) -> u8 {
        self.hours
    }

    #[inline(always)]
    pub fn minute(&self) -> u8 {
        self.minutes
    }

    #[inline(always)]
    pub fn second(&self) -> u8 {
        self.seconds
    }
    #[inline(always)]
    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn year(&self) -> u64 {
        self.year
    }

    pub fn unix_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn unix_timestamp_millis(&self) -> u128 {
        self.timestamp as u128 * 1000
    }

    pub fn unix_timestamp_nanos(&self) -> u128 {
        self.timestamp as u128 * 1000000000
    }

    pub fn month(&self) -> Month {
        match self.month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Month is not valid"),
        }
    }

    #[inline(always)]
    pub fn weekday(&self) -> Weekday {
        match self.weekday {
            0 => panic!("Week is not valid"),
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            7 => Weekday::Sunday,
            _ => panic!("Week is not valid"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Weekday {
    Sunday = 1,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Weekday::Sunday => write!(f, "Sunday"),
            Weekday::Monday => write!(f, "Monday"),
            Weekday::Tuesday => write!(f, "Tuesday"),
            Weekday::Wednesday => write!(f, "Wednesday"),
            Weekday::Thursday => write!(f, "Thrusday"),
            Weekday::Friday => write!(f, "Friday"),
            Weekday::Saturday => write!(f, "Saturday"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Month::January => write!(f, "January"),
            Month::February => write!(f, "February"),
            Month::March => write!(f, "March"),
            Month::April => write!(f, "April"),
            Month::May => write!(f, "May"),
            Month::June => write!(f, "June"),
            Month::July => write!(f, "July"),
            Month::August => write!(f, "August"),
            Month::September => write!(f, "September"),
            Month::October => write!(f, "October"),
            Month::November => write!(f, "November"),
            Month::December => write!(f, "December"),
        }
    }
}

impl From<SystemTime> for DateTime {
    // There is definitely some way to have this conversion be infallible, but
    // it won't be an issue for over 500 years.
    #[inline(always)]
    fn from(system_time: SystemTime) -> Self {
        let mut datetime = DateTime::default();

        let duration = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(err) => err.duration(),
        };

        datetime.timestamp = duration.as_secs();

        //Retrieve hours, minutes and seconds
        datetime.seconds = (datetime.timestamp % 60) as u8;
        datetime.timestamp /= 60;
        datetime.minutes = (datetime.timestamp % 60) as u8;
        datetime.timestamp /= 60;
        datetime.hours = (datetime.timestamp % 24) as u8;
        datetime.timestamp /= 24;

        //Convert Unix time to date
        let a = (4 * datetime.timestamp + 102032) / 146097 + 15;
        let b = datetime.timestamp + 2442113 + a - (a / 4);
        let mut c = (20 * b - 2442) / 7305;
        let d = (b - 365 * c - (c / 4)) as u16;
        let mut e = (d as u32 * 1000 / 30601) as u8;
        let f = (d - e as u16 * 30 - e as u16 * 601 / 1000) as u8;

        //January and February are counted as months 13 and 14 of the previous year
        if e <= 13 {
            c -= 4716;
            e -= 1;
        } else {
            c -= 4715;
            e -= 13;
        }

        //Retrieve year, month and day
        datetime.year = c;
        datetime.month = e;
        datetime.day = f;

        datetime.weekday = compute_day_of_week(c, e, f);

        datetime
    }
}

fn compute_day_of_week(mut y: u64, mut m: u8, d: u8) -> u8 {
    //January and February are counted as months 13 and 14 of the previous year
    if m <= 2 {
        m += 12;
        y -= 1;
    }

    let j = y / 100;
    //K the year of the century
    let k = y % 100;

    //Compute H using Zeller's congruence
    let h: u64 = d as u64 + (26 * (m as u64 + 1) / 10) + k + (k / 4) + (5 * j) + (j / 4);

    //Return the day of the week
    (((h + 5) % 7) as u8) + 1
}
