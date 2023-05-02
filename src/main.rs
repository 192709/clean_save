use chrono::{format, Datelike, Duration, Local, NaiveDate};

const POSTFIX: &str = "tar.xz";
fn main() {
    let now = Local::now();
    println!(
        "Current year {}, month {}, day {}",
        now.year(),
        now.month(),
        now.day()
    );
    let mut to_be_hold_endings = Vec::new();
    for i in 1..30 {
        let given_date = now.checked_sub_days(chrono::Days::new(i)).unwrap();
        // println!("{ending}");
        to_be_hold_endings.push(get_ending(given_date));
    }

    for i in 2..10 {
        let given_date = now.checked_sub_months(chrono::Months::new(i)).unwrap();
        let fst_date = NaiveDate::from_ymd_opt(given_date.year(), given_date.month(), 1).unwrap();
        to_be_hold_endings.push(get_ending(fst_date));
        let snd_date = NaiveDate::from_ymd_opt(given_date.year(), given_date.month(), 10).unwrap();
        to_be_hold_endings.push(get_ending(snd_date));
        let trd_date = NaiveDate::from_ymd_opt(given_date.year(), given_date.month(), 20).unwrap();
        to_be_hold_endings.push(get_ending(trd_date));
    }

    for i in 12..24 {
        let given_date = now.checked_sub_months(chrono::Months::new(i)).unwrap();
        let fst_date = NaiveDate::from_ymd_opt(given_date.year(), given_date.month(), 1).unwrap();
        to_be_hold_endings.push(get_ending(fst_date));
    }

    let given_date = now.checked_sub_months(chrono::Months::new(1)).unwrap();
    for i in 2..6 {
        let fst_date = NaiveDate::from_ymd_opt(given_date.year() - i, 6, 30).unwrap();
        to_be_hold_endings.push(get_ending(fst_date));
        let snd_date = NaiveDate::from_ymd_opt(given_date.year() - i, 12, 31).unwrap();
        to_be_hold_endings.push(get_ending(snd_date));
    }

    for i in 6..12 {
        let fst_date = NaiveDate::from_ymd_opt(given_date.year() - i, 12, 31).unwrap();
        to_be_hold_endings.push(get_ending(fst_date));
    }

    dbg!(to_be_hold_endings);
}

fn get_ending(given_date: impl Datelike) -> String {
    format!(
        "{:04}-{:02}-{:02}.{}",
        given_date.year(),
        given_date.month(),
        given_date.day(),
        POSTFIX
    )
}
