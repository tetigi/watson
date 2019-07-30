#![feature(proc_macro_hygiene, decl_macro)]

/// Fetches what you have on for today via Google calendar and reminders
/// Start with calendar entries
/// Then hack in a reminders integration too
/// > watson
/// > 0900 - Do homework
/// > 1100 - Leave for lunch
/// > 1800 - Interviews
use chrono::prelude::*;
use std::env;
use structopt::StructOpt;
use time::Duration;
use yup_oauth2::ServiceAccountAccess;

mod auth;

const GAPP_CREDS_LOC: &str = "GOOGLE_APPLICATION_CREDENTIALS";

//type CalendarHub =
//    google_calendar3::CalendarHub<hyper::Client, yup_oauth2::ServiceAccountAccess<hyper::Client>>;

struct GoogleCalendar {
    //hub: CalendarHub,
}

// TODO Add an Into -> Event for a GEvent
impl GoogleCalendar {
    pub fn new() -> GoogleCalendar {
        //let hub = GoogleCalendar::init();
        GoogleCalendar {}
    }

    //fn init() -> CalendarHub {
    //    let key_loc = match env::var(GAPP_CREDS_LOC) {
    //        Ok(val) => val,
    //        Err(e) => panic!("Error getting env var {}: {}", GAPP_CREDS_LOC, e),
    //    };

    //    let secret = yup_oauth2::service_account_key_from_file(&key_loc).unwrap();

    //    let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(
    //        hyper_rustls::TlsClient::new(),
    //    ));

    //    let auth = ServiceAccountAccess::new(secret, client);

    //    CalendarHub::new(
    //        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
    //            hyper_rustls::TlsClient::new(),
    //        )),
    //        auth,
    //    )
    //}
}

impl EventProvider for GoogleCalendar {
    fn get_events(&self, from: &str, until: &str) -> Vec<Event> {
        vec![
            Event::new("Do homework"),
            Event::new("Lunch with friends"),
            Event::new("More homework :("),
            Event::new("Early bedtime"),
        ]
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "watson")]
struct Opt {}

struct Event {
    name: String,
    // TODO Add start date and other metadata
}

impl Event {
    fn new(name: &str) -> Event {
        Event {
            name: name.to_string(),
        }
    }
}

trait EventProvider {
    fn get_events(&self, from: &str, until: &str) -> Vec<Event>;
}

fn fetch<P: EventProvider>(p: P) -> Vec<Event> {
    let (start, end) = todays_range();
    p.get_events(&start, &end)
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

fn print_events(events: Vec<Event>) {
    for event in events {
        println!(" - {}", event.name); // TODO Will want to add start times (and maybe length of time?) here once the API is working
    }
}

fn main() {
    let opt = Opt::from_args();
    let cal_provider = GoogleCalendar::new();
    print_events(fetch(cal_provider));

    println!("{}", auth::create_code_verifier());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("hello", "hello");
    }
}
