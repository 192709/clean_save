use chrono::{DateTime, Datelike, Duration, Local};
use std::{
    collections::HashSet,
    fs::{self, DirEntry},
};

const POSTFIX: &str = "tar.xz";

fn main() {
    let now = Local::now();
    println!(
        "Current year {}, month {}, day {}",
        now.year(),
        now.month(),
        now.day()
    );
    let to = now.year();

    let mut to_be_hold = HashSet::new();

    let from = now.year() - 12;

    let year_strings = all_years_from_to(from, to);
    to_be_hold.extend(&year_strings);

    let from = now.year() - 4;
    let quater_strings = all_quartals_from_to(from, to);
    to_be_hold.extend(&quater_strings);

    let from = now.year() - 2;
    let month_strings = all_months_from_to(from, to);
    to_be_hold.extend(&month_strings);

    let last_1180_days = all_past_180_days(&now);
    to_be_hold.extend(&last_1180_days);

    dbg!(&to_be_hold);

    // let (keep, remove) = create_lists(to_be_hold_endings);
    println!("keeeeeep:");
}

// fn create_lists(endings: Vec<String>) -> Result<(Vec<DirEntry>, Vec<DirEntry>), ()> {
//     let mut keep = vec![];
//     let mut remove = vec![];

//     let paths = fs::read_dir("./").unwrap();

//     for path in paths {
//         let file = path.unwrap();
//         println!("Name: {}", &file.path().display());
//         let filename = file.path().as_os_str().to_str().ok_or_else()?;
//         let mut found = false;
//         for ending in endings.iter() {
//             if filename.ends_with(ending.as_str()) {
//                 found = true;
//                 break;
//             }
//         }
//         if found {
//             keep.push(file)
//         } else {
//             remove.push(file);
//         }
//     }

//     Ok((keep, remove))
// }

//  Up to today
fn all_years_from_to(from: i32, to: i32) -> HashSet<String> {
    let mut res = HashSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        let year_str = format!("{:0>4}-12-01", year);
        res.insert(year_str);
    }
    res
}

fn all_quartals_from_to(from: i32, to: i32) -> HashSet<String> {
    let mut res = HashSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        for quater_month in [1, 4, 7, 10] {
            let year_str = format!("{:0>4}-{:0>2}-01", year, quater_month);
            res.insert(year_str);
        }
    }
    res
}
fn all_months_from_to(from: i32, to: i32) -> HashSet<String> {
    let mut res = HashSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        for month in 1..13 {
            let year_str = format!("{:0>4}-{:0>2}-01", year, month);
            res.insert(year_str);
        }
    }
    res
}

fn all_past_180_days(end_date: &DateTime<Local>) -> HashSet<String> {
    let mut res = HashSet::new();
    for i in 0..180 {
        let rel_date = end_date.clone() - Duration::days(i);
        let year = rel_date.year();
        let month = rel_date.month();
        let day = rel_date.day();
        let year_str = format!("{:0>4}-{:0>2}-{:0>2}", year, month, day);
        res.insert(year_str);
    }
    res
}
