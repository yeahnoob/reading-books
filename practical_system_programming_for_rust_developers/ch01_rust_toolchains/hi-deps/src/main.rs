use std::time::Duration;
use humantime::parse_duration;

fn main() {
    // println!("Hi, time duration is {} seconds", parse_duration("3min").unwrap().as_secs());
    println!(".. Target duration : {:?}", Duration::new(180, 0));
    let target_duration = parse_duration("3m");
    match target_duration {
        Ok(v) => println!("Hi, time duration is {} seconds", v.as_secs()),
        Err(e) => println!("error parsing time duration: {:?}", e),
    }
}
