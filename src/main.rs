use std::mem::zeroed;
use clap::{Arg, ArgAction, Command};
use clap::builder::PossibleValue;
use chrono::{DateTime, TimeZone};
use chrono::Local;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };
        let day_of_week = match t.weekday() {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6
        };
        let mut ns = t.nanosecond();
        let mut leap = 0;
        // chrono represents leap seconds by adding an extra second within nanoseconds field.
        let is_leap_second = ns > 1_000_000_000;
        if is_leap_second {
            ns -= 1_000_000_000;
            leap += 1;
        }
        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = day_of_week as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = (leap + t.second()) as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }
}



const ABOUT: &str = "Clonk - Gets and (maybe) Sets the time";
const RFC_2822: &str = "rfc2822";
const RFC_3339: &str = "rfc3339";
const TIMESTAMP: &str = "timestamp";
fn main() {
    let args = Command::new("clonk")
        .version("0.1")
        .about(ABOUT)
        .arg(Arg::new("action")
            .action(ArgAction::Set)
            .required(true)
            .value_parser([
                PossibleValue::new("get"),
                PossibleValue::new("set")
            ])
            .default_value("get")

        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("standard")
                .action(ArgAction::Append)
                .value_parser([
                                  PossibleValue::new(RFC_2822),
                                  PossibleValue::new(RFC_3339),
                                  PossibleValue::new(TIMESTAMP)],
                )
                .default_value(RFC_3339)
        )
        .arg(
            Arg::new("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore")
                .required(false)
                // .last(true)
        ).get_matches();
    let action = args.get_one::<String>("action").unwrap();
    let std = args.get_one::<String>("std").unwrap();

    if action == "set" {
        let t_ = args.get_one::<String>("datetime").unwrap();
        let parser = match std.as_str() {
            RFC_2822 => DateTime::parse_from_rfc2822,
            RFC_3339 => DateTime::parse_from_rfc3339,
            _ => unimplemented!()
        };
        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);

        let maybe_error = std::io::Error::last_os_error();
        let os_error_code = &maybe_error.raw_os_error();

        match os_error_code {
            Some(0) => (),
            // Some(_) => eprintln!("Unable to set the time: {:?}", maybe_error),
            Some(_) => eprintln!("Unable to set the time: {}", maybe_error.to_string()),
            None => (),
        }
    }
    let now = Clock::get();
    match std.as_str() {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => {
            unreachable!()
        }
    }
}
