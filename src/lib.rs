use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

#[derive(Deserialize)]
struct Msg{
    password: String,
    username: String
}

fn send_failure(who: &str, why: &str){
    let resp_headers = vec![(String::from("content-type"), String::from("text/html"))];
    send_response(200, resp_headers, format!("Your {} is in valid. Reason: \n{}\n", who, why).as_bytes().to_vec());
}

async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // let msg = qry.get("msg").unwrap();
    let body_string = String::from_utf8(body.clone());
    if body_string.is_err() || serde_json::from_str::<Msg>(body_string.unwrap().as_str()).is_err(){
        send_response(
            500, 
            vec![(String::from("content-type"), String::from("text/html"))],
            String::from("Cannot resolve request body.\n").as_bytes().to_vec()
        );
        return;
    }
    let msg = serde_json::from_str::<Msg>(String::from_utf8(body).unwrap().as_str()).unwrap();
    if msg.password.len() < 5 {
        send_failure("password", "Too short.");
        return;
    }else if !msg.password.chars().all(|c|c.is_digit(10)||c.is_lowercase()||c.is_uppercase()){
        send_failure("password", "Password can only contain A-Z, a-z or 0-9.");
        return;
    }else if msg.password.chars().all(|c|!c.is_digit(10)) 
    || msg.password.chars().all(|c|!c.is_uppercase()) 
    || msg.password.chars().all(|c|!c.is_lowercase()){
        send_failure("password", "Password must contain A-Z, a-z, and 0-9 to be complex enough.");
        return
    }else if msg.password.find(&msg.username).is_some(){
        send_failure("password and username", "Username cannot be part of password.");
        return;
    }

    send_response(
        200, 
        vec![(String::from("content-type"), String::from("text/html"))], 
        String::from("vlidation success.\n").as_bytes().to_vec()
    );
}
