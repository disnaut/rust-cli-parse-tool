/*
Requirements:
This tool must create dummy log entries.
This tool must include the capability to have mal-formed log line.
This tool must support the ability to create a file of a given size in MB

Specifications:
The log files will look like the following:
[DATE/TIME] [TYPE] [SYSTEM] [ACTION] [MISC]

Malformed log entries can be anything really.
 */

use chrono::{DateTime, Duration, Utc};
use rand::Rng;

enum LogType {
    INFO,
    WARN,
    DEBUG,
    ERROR,
}

const SYSTEM_TYPES: [&str; 7] = [
    "auth-service",
    "payment-service",
    "user-service",
    "api-gateway",
    "notification-service",
    "system",
    "cache-service",
];

struct LogMessage {
    date_time: DateTime<Utc>,
    log_type: LogType,
    system: String,
    action: String,
    misc: String,
}

fn main() {
    println!("Hello, world!");
}

fn random_datetime_in_range(
    rng: &mut impl Rng,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> DateTime<Utc> {
    let duration = end - start;
    let random_seconds = rng.gen_range(0..duration.num_seconds());
    start + Duration::seconds(random_seconds)
}

fn random_log_type(rng: &mut impl Rng) -> String {
    let index = rng.gen_range(0..SYSTEM_TYPES.len());
    SYSTEM_TYPES[index]
}
