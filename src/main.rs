use csv;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/names.rs"));

fn main() -> Result<(), Box<dyn Error>> {
    use clap::{crate_authors, crate_description, crate_version, value_t, App, Arg};

    let matches = App::new("name_distribution")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("number_buckets")
                .short("n")
                .long("no_buckets")
                .help("Number of buckets of names to create (default 4)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("accuracy")
                .short("p")
                .long("percentage")
                .help("Sets how accurately the buckets should be evenly matched (default 2 = 2%)")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let no_buckets = value_t!(matches, "number_buckets", usize).unwrap_or(4);
    let max_deviation_percentage = value_t!(matches, "accuracy", usize).unwrap_or(2) as f32 / 100.0;
    let ranges = distributed_ranges(no_buckets, max_deviation_percentage);

    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&["start_letter", "end_letter", "percentage"])?;
    for (start, end, percentage) in ranges {
        let p = (percentage * 100.0).to_string();
        let r = vec![start, end, p];
        wtr.write_record(r)?;
    }
    wtr.flush()?;
    Ok(())
}

type Bucket = (String, String, f32);

fn distributed_ranges(no_buckets: usize, max_deviation_percentage: f32) -> Vec<Bucket> {
    let total_count = SURNAME[SURNAME_LEN - 1].0 as f32;
    let max_deviation_count = (total_count as f32 * max_deviation_percentage) as u32;
    let ranges = distributed_name_boundaries(no_buckets, max_deviation_count);
    let mut last_boundary = "A".to_string();
    let mut last_count = 0u32;
    let mut buckets = Vec::new();
    for r in ranges {
        let (count, bucket_end, bucket_start) = r;
        let bucket_percentage = (count - last_count) as f32 / total_count;
        let bucket = (last_boundary, bucket_end, bucket_percentage);
        buckets.push(bucket);
        last_boundary = bucket_start;
        last_count = count;
    }
    let last_bucket = (
        last_boundary,
        "Z".to_string(),
        (total_count - last_count as f32) / total_count,
    );
    buckets.push(last_bucket);
    buckets
}

type NameRange = (u32, String, String);

fn distributed_name_boundaries(no_buckets: usize, max_deviation_count: u32) -> Vec<NameRange> {
    let total_count = SURNAME[SURNAME.len() - 1].0;
    let bucket_size = total_count / no_buckets as u32;
    let boundaries: Vec<NameRange> = (1..no_buckets)
        .map(|i| {
            let count = i as u32 * bucket_size;
            boundary(SURNAME, SURNAME.len(), count, max_deviation_count)
        })
        .collect();
    boundaries
}

type ArrayType = &'static [(FrequencyCount, &'static str)];
type FrequencyCount = u32;

fn boundary(
    array: ArrayType,
    length: usize,
    count: u32,
    max_deviation_count: u32,
) -> (u32, String, String) {
    let max_deviation_per_boundary = max_deviation_count / 2;
    let i = get_entry_index(array, count, 0, length);
    let mut best_width = boundary_width(array, i);
    let mut best_index = i;

    let mut try_index = |i: usize| {
        let width = boundary_width(array, i);
        if width < best_width
            && ((count as i32 - array[i].0 as i32).abs() as u32) < max_deviation_per_boundary
        {
            best_index = i;
            best_width = width;
        }
    };

    for d in 1..2000 {
        if d < i {
            try_index(i - d);
        }
        if i + d < length {
            try_index(i + d);
        }
    }
    let end_boundary = &array[best_index - 1].1[..best_width.min(array[best_index - 1].1.len())];
    // let _start_boundary = &array[best_index].1[..best_width.min(array[best_index].1.len())];
    let fixed_start_boundary = next_boundary(end_boundary);
    (
        array[best_index].0,
        end_boundary.to_string(),
        fixed_start_boundary,
    )
}

fn next_boundary(boundary: &str) -> String {
    let len = boundary.chars().count();
    format!(
        "{}{}",
        boundary.chars().take(len - 1).collect::<String>(),
        (boundary.chars().nth(len - 1).unwrap() as u8 + 1) as char
    )
}

fn boundary_width(array: ArrayType, i: usize) -> usize {
    let x = array[i - 1].1;
    let y = array[i].1;
    x.chars().zip(y.chars()).take_while(|(x, y)| x == y).count() + 1
}

fn get_entry_index(array: ArrayType, count: FrequencyCount, min: usize, max: usize) -> usize {
    let avg = min + (max - min) / 4;
    let (low, high) = match avg {
        0 => (0, array[avg].0),
        _ => (array[avg - 1].0, array[avg].0),
    };
    if count >= low && count < high {
        avg
    } else if count < low {
        get_entry_index(&array, count, min, avg - 1)
    } else {
        get_entry_index(&array, count, avg + 1, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_names_for_four_buckets() {
        let x = distributed_name_boundaries(4, 10000);
        assert_eq!(x[0], (33233, "D".to_string(), "E".to_string()));
        assert_eq!(x[1], (67197, "L".to_string(), "M".to_string()));
        assert_eq!(x[2], (102399, "P".to_string(), "Q".to_string()));
        let s1 = x[0].0;
        let s2 = x[1].0 - s1;
        let s3 = x[2].0 - x[1].0;
        let s4 = SURNAME[SURNAME_LEN - 1].0 - x[2].0;
        assert_eq!(s1, 33233);
        assert_eq!(s2, 33964);
        assert_eq!(s3, 35202);
        assert_eq!(s4, 32424);
    }

    #[test]
    fn test_produces_results_within_reasonable_range() {
        let max_deviation = 0.02;
        let mut has_failed = false;
        for no_buckets in 2..50 {
            let b = distributed_ranges(no_buckets, max_deviation);
            let biggest_deviation = biggest_deviation_for_buckets(&b);
            if biggest_deviation > max_deviation {
                eprintln!(
                    "Maximum deviation was exceeded for {} buckets - with deviation of {}%",
                    no_buckets,
                    biggest_deviation * 100.0
                );
                eprintln!("Buckets were: {:?}", b);
                has_failed = true;
            }
        }
        if has_failed {
            panic!()
        };
    }

    fn biggest_deviation_for_buckets(buckets: &Vec<Bucket>) -> f32 {
        let no_buckets = buckets.len();
        let division = 1.0 / no_buckets as f32;
        buckets
            .iter()
            .fold(0f32, |m, (_, _, p)| m.max((p - division).abs()))
    }
}
