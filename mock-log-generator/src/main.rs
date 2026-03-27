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

use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::rng();

    println!("char: '{}'", rng.random::<char>());
    println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);
}
