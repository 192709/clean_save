use chrono::{DateTime, Datelike, Duration, Local};
use std::{
    collections::{BTreeSet, HashSet},
    error::Error,
    ffi::OsString,
    fs::{self, DirEntry},
    path::PathBuf,
};

fn main() {
    let now = Local::now();
    println!(
        "Current year {}, month {}, day {}",
        now.year(),
        now.month(),
        now.day()
    );
    let to = now.year();

    // each value once and sorted - dedup for vec needs a sorted vec. Further a set is faster on searching
    let mut to_be_hold = BTreeSet::new();

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

    // dbg!(&to_be_hold);

    let endings = vec!["tar.gz".to_owned(), "tar.xz".to_owned()];
    let (keep, remove) = create_lists("/save2/backup", to_be_hold, endings)
        .expect("could not get lists remove and keep ...");
    println!("keeeeeep:");
    println!("{:#?}", keep);

    println!("remove:");
    println!("{:#?}", remove);
}

//create list of files ...
fn create_lists(
    folder: &str,
    to_be_hold: BTreeSet<&String>,
    endings: Vec<String>,
) -> Result<(BTreeSet<String>, BTreeSet<String>), OsString> {
    let mut keep = BTreeSet::new();
    let mut remove = BTreeSet::new();

    // let dir = fs::read_dir(folder).unwrap();
    // let postfix_with_dot = if POSTFIX.starts_with(".") {
    //     POSTFIX.to_owned()
    // } else {
    //     format!(".{}", POSTFIX)
    // };

    for ending in endings {
        let dir = get_paths_by_ending(folder, &ending);
        for entry in dir.expect("path could not be read") {
            // if let Ok(entry) = entry {
            // Here, `entry` is a `DirEntry`.
            let file_name = entry.file_name().to_str().unwrap().to_owned();
            let file_name_lenght_wo_ending =
                &file_name.chars().count() - &ending.chars().count() - 1;
            let file_name_wo_ending = &file_name[0..file_name_lenght_wo_ending].to_owned();
            let mut found = false;
            for candidate in to_be_hold.iter() {
                // let ending_w_postfix = format!("{}{}", ending, postfix_with_dot);
                // println!(
                // "found: <{}>, shortened to <{}> check against <{}>",
                // file_name, file_name_wo_ending, candidate
                // );
                if file_name_wo_ending.ends_with(*candidate) {
                    found = true;
                    // println!("result is: {}", found);
                    break;
                }
            }
            if found {
                keep.insert(file_name);
            } else {
                remove.insert(file_name);
            }
            // }
        }
    }
    Ok((keep, remove))
}

fn get_paths_by_ending(dir: &str, ending: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        // .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|path| {
            // println!("path in Filtering: {:?}", &path);
            if path
                .file_name()
                // .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                .ends_with(ending)
            {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!(
        "found elements in path for ending {}: {}",
        ending,
        paths.len()
    );
    Ok(paths)
}

//  a value per year.
// REMINDER: always to as today, otherwice the value might be lost in quaterly ... .
fn all_years_from_to(from: i32, to: i32) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        let year_str = format!("{:0>4}-12-01", year);
        res.insert(year_str);
    }
    res
}

// a value per Quater
fn all_quartals_from_to(from: i32, to: i32) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        for quater_month in [1, 4, 7, 10] {
            let year_str = format!("{:0>4}-{:0>2}-01", year, quater_month);
            res.insert(year_str);
        }
    }
    res
}

// a value per month
fn all_months_from_to(from: i32, to: i32) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    let to_plus_one = to + 1;
    for year in from..to_plus_one {
        for month in 1..13 {
            let year_str = format!("{:0>4}-{:0>2}-01", year, month);
            res.insert(year_str);
        }
    }
    res
}

// a value each day for the past 180 days and today
fn all_past_180_days(end_date: &DateTime<Local>) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
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
