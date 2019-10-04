use std::ops::Range;
use std::str::FromStr;

use clap::{App, Arg};

use chrono::format::{strftime::StrftimeItems, Item};
use chrono::prelude::*;

type ClapResult = Result<(), String>;

fn to_clap<T>(r: Result<T, String>) -> ClapResult {
    r.map(|_| ())
}

fn validate<T>(s: String, msg: &dyn ToString) -> ClapResult
where
    T: FromStr,
{
    s.parse::<T>().map(|_| ()).map_err(|_| msg.to_string())
}

fn validate_int(s: String) -> ClapResult {
    validate::<i32>(s, &"Must be valid integer")
}

/// int must be > 0
fn validate_int_positive(s: String) -> ClapResult {
    let msg = &"Must be valid integer > 0";
    s.parse::<i32>()
        .ok()
        .filter(|&j| j > 0)
        .map(|_| ())
        .ok_or(msg.to_string())
}

fn validate_int_range(r: Range<i32>) -> impl Fn(String) -> ClapResult {
    let msg = format!("Must be a valid integer between {} and {}", r.start, r.end);
    move |s| {
        s.parse::<i32>()
            .map_err(|_| msg.to_string())
            .ok()
            .filter(|&j| r.start < j && j <= r.end)
            .map(|_| ())
            .ok_or(msg.to_string())
    }
}

fn validate_float(s: String) -> ClapResult {
    validate::<f64>(s, &"Must be valid floating point number")
}

fn validate_strftime(s: String) -> ClapResult {
    if StrftimeItems::new(&s).any(|item| match item {
        Item::Error => true,
        _ => false,
    }) {
        Err("Must be a valid format string; see chrono::format::strftime docs".to_string())
    } else {
        Ok(())
    }
}

fn fmt_filename(raw: &str) -> String {
    Utc::now().format(raw).to_string()
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Ray marcher")
        .author("Rebecca Turner <637275@gmail.com>")
        .arg(Arg::from_usage("-r --resolution [WIDTH] [HEIGHT] 'Output resolution in pixels'")
             .validator(validate_int_positive))
        .arg(Arg::from_usage("-a --antialiasing [N] 'Subpixel antialiasing; note that 2 would render 4 samples per pixel'")
             .validator(validate_int_positive)
             .default_value("1"))
        .arg(Arg::from_usage("-o --output [FILENAME] 'PNG output filename; accepts standard date/time formatters'")
             .validator(validate_strftime)
             .default_value("ray-marcher-%FT%H_%M_%S.png"))
        .arg(Arg::from_usage("-i --iterations [N] 'Number of iterations to render with'")
             .validator(validate_int_positive)
             .default_value("64"))
        .arg(Arg::from_usage("-q --quaternion [F] [F] [F] [F] 'Quaternion to render, with the real component first, then i, j, and k components'")
             .validator(validate_float))
}

fn main() {
    let matches = app().get_matches();

    let filename = fmt_filename(matches.value_of("output").unwrap());
    print!("{}", filename);
}
