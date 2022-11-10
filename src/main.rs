use cal::*;

use chrono::{Datelike, Local};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// show only a single month (default)
    #[arg(group = "len", short = '1', long = "one")]
    len_1: bool,

    /// show three months spanning the date
    #[arg(group = "len", short = '3', long = "three")]
    len_3: bool,

    /// show the whole year
    #[arg(group = "len", short = 'y', long = "year")]
    len_y: bool,

    /// show NUM months starting with date's month
    #[arg(group = "len", short = 'n', long = "months", value_name = "NUM")]
    len_n: Option<usize>,

    /// defaults to current year
    year: Option<Year>,

    /// defaults to current month
    month: Option<Month>,
}

fn main() {
    let cli = Cli::parse();

    let now = Local::now();
    let year = cli.year.unwrap_or(now.year() as Year);
    let month = cli.month.unwrap_or(now.month() as Month);

    let (start, len) = match (cli.len_1, cli.len_3, cli.len_y, cli.len_n) {
        (_, true, _, _) => (
            if month == 1 {
                YearMonth::new(year - 1, 12)
            } else {
                YearMonth::new(year, month - 1)
            },
            3,
        ),
        (_, _, true, _) => (YearMonth::new(year, 1), 12),
        (_, _, _, Some(n)) => (YearMonth::new(year, month), n),
        _ => (YearMonth::new(year, month), 1),
    };

    let cal = CalRange::new(start, len);

    println!("{}", cal);
}
