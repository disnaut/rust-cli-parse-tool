/*
This program grabs random strings from the .txt files located in files/

This assumes that each string is around 80 bytes give or take.
 */

use chrono::{DateTime, Days, Duration, Utc};
use rand::seq::IndexedRandom;
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

fn main() {
    //iteration_one();
    //iteration_two();
    iteration_three();
}

// 29.2 secs
// Creates a single large string
// and writes it to the file.
fn iteration_one() {
    let mut rng = rand::rng();

    let wd = std::env::current_dir();
    println!("The working directory is '{:?}'", wd);

    let api_lines = read_lines("./src/files/api-gateway.txt");
    let auth_lines = read_lines("./src/files/auth-service.txt");
    let cache_lines = read_lines("./src/files/cache-service.txt");
    let inventory_lines = read_lines("./src/files/inventory-service.txt");
    let malformed_lines = read_lines("./src/files/malformed.txt");
    let notification_lines = read_lines("./src/files/notification-service.txt");
    let payment_lines = read_lines("./src/files/payment-service.txt");
    let system_lines = read_lines("./src/files/system.txt");

    let mut final_string: String = String::from("");

    let path = Path::new("example.log");
    let display = path.display();

    let current_time = Utc::now();
    let range_time = current_time.checked_add_days(Days::new(1)).unwrap();

    // not real systems numbers but good enough
    // kb 1000
    // mb 1000000
    // gb 1000000000

    // Lines for 1 gb:  1,073,741,824 / 81 ≈ 13,256,071 lines

    for _ in 1..13256071 {
        // Iteration 1, approximately 30 sec
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

        // Problem is that the malformed shouldn't have the time_stamp
        // and should instead be just the line.
        match rand::random_range(0..8) {
            0 => new_str.push_str(api_lines.choose(&mut rng).unwrap()),
            1 => new_str.push_str(auth_lines.choose(&mut rng).unwrap()),
            2 => new_str.push_str(cache_lines.choose(&mut rng).unwrap()),
            3 => new_str.push_str(inventory_lines.choose(&mut rng).unwrap()),
            4 => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
            5 => new_str.push_str(notification_lines.choose(&mut rng).unwrap()),
            6 => new_str.push_str(payment_lines.choose(&mut rng).unwrap()),
            7 => new_str.push_str(system_lines.choose(&mut rng).unwrap()),
            _ => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
        }

        new_str.push('\n');

        final_string.push_str(&new_str);
    }

    // Write to the file
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&final_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

// 1 min 40 secs
// Appends a string to the end of the file.
// Bad because it opens and closes the file multiple times.
fn iteration_two() {
    let mut rng = rand::rng();

    let wd = std::env::current_dir();
    println!("The working directory is '{:?}'", wd);

    let api_lines = read_lines("./src/files/api-gateway.txt");
    let auth_lines = read_lines("./src/files/auth-service.txt");
    let cache_lines = read_lines("./src/files/cache-service.txt");
    let inventory_lines = read_lines("./src/files/inventory-service.txt");
    let malformed_lines = read_lines("./src/files/malformed.txt");
    let notification_lines = read_lines("./src/files/notification-service.txt");
    let payment_lines = read_lines("./src/files/payment-service.txt");
    let system_lines = read_lines("./src/files/system.txt");

    let path = Path::new("example.log");

    let current_time = Utc::now();
    let range_time = current_time.checked_add_days(Days::new(1)).unwrap();

    // not real systems numbers but good enough
    // kb 1000
    // mb 1000000
    // gb 1000000000

    // Lines for 1 gb:  1,073,741,824 / 81 ≈ 13,256,071 lines

    // Write to the file
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    for _ in 1..13256071 {
        // Iteration 1, approximately 30 sec
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

        // Problem is that the malformed shouldn't have the time_stamp
        // and should instead be just the line.
        match rand::random_range(0..8) {
            0 => new_str.push_str(api_lines.choose(&mut rng).unwrap()),
            1 => new_str.push_str(auth_lines.choose(&mut rng).unwrap()),
            2 => new_str.push_str(cache_lines.choose(&mut rng).unwrap()),
            3 => new_str.push_str(inventory_lines.choose(&mut rng).unwrap()),
            4 => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
            5 => new_str.push_str(notification_lines.choose(&mut rng).unwrap()),
            6 => new_str.push_str(payment_lines.choose(&mut rng).unwrap()),
            7 => new_str.push_str(system_lines.choose(&mut rng).unwrap()),
            _ => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
        }

        writeln!(&mut file, "{new_str}");
    }

    println!("Completed iteration two.");
}

// Using BufWriter to try and see if buffering makes it better?
// Probably crashes because BufWriter only has so much space in the buffer
// so in theory I need to flush it more often I think?

// Took 29.9 seconds, still slower than iteration one.
fn iteration_three() {
    let mut rng = rand::rng();

    let wd = std::env::current_dir();
    println!("The working directory is '{:?}'", wd);

    let api_lines = read_lines("./src/files/api-gateway.txt");
    let auth_lines = read_lines("./src/files/auth-service.txt");
    let cache_lines = read_lines("./src/files/cache-service.txt");
    let inventory_lines = read_lines("./src/files/inventory-service.txt");
    let malformed_lines = read_lines("./src/files/malformed.txt");
    let notification_lines = read_lines("./src/files/notification-service.txt");
    let payment_lines = read_lines("./src/files/payment-service.txt");
    let system_lines = read_lines("./src/files/system.txt");

    let path = Path::new("example.log");

    let current_time = Utc::now();
    let range_time = current_time.checked_add_days(Days::new(1)).unwrap();

    // not real systems numbers but good enough
    // kb 1000
    // mb 1000000
    // gb 1000000000

    // Lines for 1 gb:  1,073,741,824 / 81 ≈ 13,256,071 lines

    // Write to the file
    let file = File::create(path).unwrap();

    let mut buf_writer = BufWriter::new(file);

    for _ in 1..13256071 {
        // Iteration 1, approximately 30 sec
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

        // Problem is that the malformed shouldn't have the time_stamp
        // and should instead be just the line.
        match rand::random_range(0..8) {
            0 => new_str.push_str(api_lines.choose(&mut rng).unwrap()),
            1 => new_str.push_str(auth_lines.choose(&mut rng).unwrap()),
            2 => new_str.push_str(cache_lines.choose(&mut rng).unwrap()),
            3 => new_str.push_str(inventory_lines.choose(&mut rng).unwrap()),
            4 => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
            5 => new_str.push_str(notification_lines.choose(&mut rng).unwrap()),
            6 => new_str.push_str(payment_lines.choose(&mut rng).unwrap()),
            7 => new_str.push_str(system_lines.choose(&mut rng).unwrap()),
            _ => new_str = malformed_lines.choose(&mut rng).unwrap().to_string(),
        }

        writeln!(buf_writer, "{new_str}");
    }

    buf_writer.flush();

    println!("Completed iteration two.");
}

fn random_datetime_in_range(start: DateTime<Utc>, end: DateTime<Utc>) -> DateTime<Utc> {
    let duration = end - start;
    let random_seconds = rand::random_range(0..duration.num_seconds());
    start + Duration::seconds(random_seconds)
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
