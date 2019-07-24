/// Fetches what you have on for today via Google calendar and reminders
/// Start with calendar entries
/// Then hack in a reminders integration too
/// > watson
/// > 0900 - Do homework
/// > 1100 - Leave for lunch
/// > 1800 - Interviews
use chrono::prelude::*;
use structopt::StructOpt;
use time::Duration;

#[derive(StructOpt, Debug)]
#[structopt(name = "watson")]
struct Opt {}

struct Event {
    name: String,
}

trait EventProvider {
    fn get_events(&self, from: u32, until: u32) -> Vec<Event>;
}

fn fetch<P: EventProvider>(p: P) -> Vec<Event> {
    p.get_events(1, 2)
}

/// Returns a Google cal formatted date range of (NOW, EOD)
fn todays_range() -> (String, String) {
    let now: DateTime<Local> = Local::now();
    let tomorrow = now.checked_add_signed(Duration::days(1)).unwrap();
    let eod = Local
        .ymd(tomorrow.year(), tomorrow.month(), tomorrow.day())
        .and_hms(0, 0, 0);

    (now.to_rfc3339(), eod.to_rfc3339())
}

fn main() {
    println!("Hello, world!");
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    println!("{:?}", todays_range());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        panic!()
    }
}
