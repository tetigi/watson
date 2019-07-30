#[macro_use] extern crate rocket;

use curl::easy::Easy;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

const OAUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const CLIENT_ID: &str = "395397085121-j1fvnakm3870a1s1bocuslp47feql2li.apps.googleusercontent.com";
const PORT: u32 = 7878;
const STATE_STUB: &str = "stub";

#[get("/")]
fn index() -> &'static str {
    panic!("OH SHIT")
}

pub fn create_code_verifier() -> String {
    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();

    format!("{}-{}", id1, id2)
}

fn create_state_token() -> String {
    Uuid::new_v4().to_string()
}

fn oauth2_request_params() -> HashMap<String, String> {
    let mut params = HashMap::new();

    params.insert("client_id".to_string(), CLIENT_ID.to_string());
    params.insert(
        "redirect_uri".to_string(),
        format!("http://127.0.0.1:{}", PORT),
    );
    params.insert("response_type".to_string(), "code".to_string());
    params.insert(
        "scope".to_string(),
        "https://www.googleapis.com/auth/calendar.events.readonly".to_string(),
    );

    params.insert("state".to_string(), STATE_STUB.to_string());

    params.insert("code_challenge_method".to_string(), "plain".to_string());
    params.insert("code_challenge".to_string(), create_code_verifier());

    params
}

fn oauth2_workflow() {
    let params = oauth2_request_params();

    let link = Url::parse_with_params(OAUTH_URL, params);

    println!("{:?}", link);
}

mod test {
    use super::*;

    #[test]
    fn test_oauth_params() {
	println!("{:?}", rocket::ignite().mount("/", routes![index]).launch());
        panic!();
    }
}
