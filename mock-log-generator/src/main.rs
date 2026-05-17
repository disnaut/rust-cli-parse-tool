/*
This program grabs random strings from the .txt files located in files/

This assumes that each string is around 80 bytes give or take.
 */

use chrono::{DateTime, Days, Duration, Utc};
use rand::seq::IndexedRandom;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::string::String;

fn main() {
    const BYTES_IN_GIG: i32 = 1073741824;
    let number_of_gigs: i32 = 1;
    //iteration_one();
    //iteration_two();
    //iteration_three();
    //iteration_four();
    //iteration_five();
    iteration_six(number_of_gigs * BYTES_IN_GIG);
}

/// This iteration was my first attempt and just creates a really large string.
/// It first creates an empty string.
/// It will then loop 13,256,071 times creating a mock log string, resulting in an approx 1GB sized string.
/// Each string is appended to the empty string.
/// Once the giga-string is created, it will then write that entire string to a file.
/// Approximate run time: 29.3 Seconds
/// Math: 1,073,741,824 bytes / 81 bytes ≈ 13,256,071 lines
/// Assumed that each string was approximately 80 bytes,
/// 81 in calculation because need an extra byte for the carriage return.
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

    for _ in 1..13256071 {
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

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

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&final_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

/// This iteration was my attempt to optimize the first iteration.
/// What happens in this one is I create a mock log string, open the file, write to the file, then close the file.
/// As you can imagine that didn't work very well, at all.
/// My analysis is the following:
/// - We open the file, meaning that we have to read in all of the data
/// - We append to the file.
/// - We close the file.
/// And as we saw in iteration one, we're doing that 13 million times.
/// That is a lot of I/O, and as a result, it took this function 1 minute and 40 seconds to finish creating a 1GB file.
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

    let mut file = File::options()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    for _ in 1..13256071 {
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

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

/// This is where I started looking into other modules in the io crate.
/// When reading about the BufReader, it clicked that buffering writes to the file would
/// more than likely be more efficient for both memory and time. So I went and read the BufWriter docs.
/// This iteration is effectively iteration one, but using a BufWriter, flushing at the end, so writing a giga-string to a file.
/// Took 29.9 seconds. I'm assuming slower than iteration one because we have effectively a middleman and that
/// adds time to these operations, no matter how miniscule.
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

    let file = File::create(path).unwrap();

    let mut buf_writer = BufWriter::new(file);

    for _ in 1..13256071 {
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

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

    println!("Completed iteration three.");
}

/// This is iteration two but BufWriter, so flushing after each string.
/// Needless to say, it wasn't cash money.
/// Took 1 minute and 7 seconds.
/// Not great.
fn iteration_four() {
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

    let file = File::create(path).unwrap();

    let mut buf_writer = BufWriter::new(file);

    for _ in 1..13256071 {
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

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
        buf_writer.flush();
    }

    println!("Completed iteration four.");
}

/// This is where it clicked and the wrinkle in my brain formed.
/// I saw that the default buffersize for the BufWriter was 8KiB, so I used
/// SI notation (I know, i'm lazy), and the 80 byte assumption to have the buffer
/// flush every 100 writes to the buffer.
/// Pretty good, finished in 30.0 seconds.
fn iteration_five() {
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

    let file = File::create(path).unwrap();

    let mut buf_writer = BufWriter::new(file);
    let mut counter = 0;

    for _ in 1..13256071 {
        counter = counter + 1;

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

        // 1KiB is 1024 bytes
        // 8KiB is 8192 bytes
        // So since I'm lazy and am going to use SI notation for this iteration,
        // 8000 / 80 = 100.
        if counter == 100 {
            buf_writer.flush();
            counter = 0;
        }
    }

    println!("Completed iteration five.");
}

/// This iteration is going to utilize more tools from the BufWriter to
/// be more percise with using the capacity method from the BufWriter and
/// the buffer().len() to flush if the generated string goes over the capacity of the default buffer.
/// This primarily is to check if doing this would make things faster since I don't have to worry about
/// incrementing a counter, and instead am doing a boolean operation.
fn iteration_six(total_bytes: i32) {
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

    let file = File::create(path).unwrap();

    let mut buf_writer = BufWriter::new(file);

    for _ in 1..total_bytes {
        let time_stamp = random_datetime_in_range(current_time, range_time);
        let mut new_str: String = time_stamp.to_string() + " ";

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

        if buf_writer.buffer().len() + new_str.len() > buf_writer.capacity() {
            buf_writer.flush();
        }

        writeln!(buf_writer, "{new_str}");
    }

    if buf_writer.buffer().len() > 0 {
        buf_writer.flush();
    }

    println!("Completed iteration six.");
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
