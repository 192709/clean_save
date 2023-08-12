use chrono::{DateTime, Datelike, Duration, Local};
use clap::{command, Parser};
use std::{collections::BTreeSet, error::Error, ffi::OsString, fs::DirEntry};

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'e', value_delimiter = ' ', num_args = 1.., default_values = ["tar.gz", "tar.xz"])]
    endings: Vec<String>,

    #[arg(short = 'f',value_delimiter = ' ', num_args = 1.., default_values = ["/save/backup", "/save2/backup"])]
    folders: Vec<String>,
    #[arg(short, long)]
    dryrun: bool,
}

fn main() {
    let args = Args::parse();
    println!("Endings {:?}!", args.endings);
    println!("Folders {:?}!", args.folders);
    println!("Dryrun {:?}!", args.dryrun);

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

    let last_1180_days = past_120_days(&now);
    to_be_hold.extend(&last_1180_days);

    // dbg!(&to_be_hold);
    let endings = args.endings;
    for folder in args.folders.iter() {
        let (keep, remove) = create_lists(folder, &to_be_hold, &endings)
            .expect("could not get lists remove and keep ...");
        println!("keeeeeep:");
        println!("{:#?}", keep);

        println!("remove:");
        println!("{:#?}", remove);

        if !args.dryrun {
            delete_files(folder, &remove);
        }
    }
}

//create list of files ...
fn create_lists(
    folder: &str,
    to_be_hold: &BTreeSet<&String>,
    endings: &Vec<String>,
) -> Result<(BTreeSet<String>, BTreeSet<String>), OsString> {
    let mut keep = BTreeSet::new();
    let mut remove = BTreeSet::new();

    for ending in endings {
        let dir = get_paths_by_ending(folder, ending);
        for entry in dir.expect("path could not be read") {
            let file_name = entry.file_name().to_str().unwrap().to_owned();

            let file_name_lenght_wo_ending = file_name.chars().count() - ending.chars().count() - 1;

            let file_name_wo_ending = &file_name[0..file_name_lenght_wo_ending].to_owned();

            let mut found = false;
            for candidate in to_be_hold.iter() {
                if file_name_wo_ending.ends_with(*candidate) {
                    found = true;
                    break;
                }
            }

            if found {
                keep.insert(file_name);
            } else {
                remove.insert(file_name);
            }
        }
    }
    Ok((keep, remove))
}

fn get_paths_by_ending(dir: &str, ending: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
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

fn delete_files(folder: &str, to_be_removed: &BTreeSet<String>) {}

//  a value per year.
// REMINDER: always to as today, otherwice the value might be lost in quaterly, which does not know about 12-01 ... .
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

// a value each day for the past 120 days and today
fn past_120_days(end_date: &DateTime<Local>) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    for i in 0..120 {
        let rel_date = *end_date - Duration::days(i);
        let year = rel_date.year();
        let month = rel_date.month();
        let day = rel_date.day();
        let year_str = format!("{:0>4}-{:0>2}-{:0>2}", year, month, day);
        res.insert(year_str);
    }
    res
}
