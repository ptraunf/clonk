use clap::{Arg, ArgAction, Command};
use clap::builder::PossibleValue;
// use clap::parser::ValueSource::DefaultValue;
use chrono::DateTime;
use chrono::Local;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }
    fn set() -> ! {
        unimplemented!()
    }
}

const ABOUT: &str = "Clonk - Gets and (maybe) Sets the time";

fn main() {
    let args = Command::new("clonk")
        .version("0.1")
        .about(ABOUT)
        .arg(Arg::new("action")
            .action(ArgAction::Append)
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
                                  PossibleValue::new("rfc2822"),
                                  PossibleValue::new("rfc3339"),
                                  PossibleValue::new("timestamp")],
                )
                .default_value("rfc3339")
        )
        .arg(
            Arg::new("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore")
                .required(false)
                .last(true)
        ).get_matches();
   let std = args.get_one::<String>("std").unwrap();
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
