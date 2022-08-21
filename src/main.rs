// Author: Shahzain
// Please ignore my shit rust skills 😁
// This software contains open source software, `serde`, `serde_json`, `reqwest`, `colored` and `rand`
use colored::*;
use reqwest::StatusCode;
use serde_json::*;
use std::io::stdin;
use std::thread;

mod headers;
mod request_structs;
mod structs;
mod utils;

fn main() {
    println!("{}", "
███████╗██╗  ██╗ █████╗ ██╗  ██╗███████╗ █████╗ ██╗███╗   ██╗    ██████╗  ██████╗ ████████╗████████╗███████╗██████╗ 
██╔════╝██║  ██║██╔══██╗██║  ██║╚══███╔╝██╔══██╗██║████╗  ██║    ██╔══██╗██╔═══██╗╚══██╔══╝╚══██╔══╝██╔════╝██╔══██╗
███████╗███████║███████║███████║  ███╔╝ ███████║██║██╔██╗ ██║    ██████╔╝██║   ██║   ██║      ██║   █████╗  ██████╔╝
╚════██║██╔══██║██╔══██║██╔══██║ ███╔╝  ██╔══██║██║██║╚██╗██║    ██╔══██╗██║   ██║   ██║      ██║   ██╔══╝  ██╔══██╗
███████║██║  ██║██║  ██║██║  ██║███████╗██║  ██║██║██║ ╚████║    ██████╔╝╚██████╔╝   ██║      ██║   ███████╗██║  ██║
╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝    ╚═════╝  ╚═════╝    ╚═╝      ╚═╝   ╚══════╝╚═╝  ╚═╝".red().bold());
    println!("{}", "Rust - By Shahzain".green().bold());
    println!("[{}] {}", "-".red(), "Enter thread count: ".green().bold());
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut thread_vector: Vec<thread::JoinHandle<()>> = vec![];
    match trimmed.parse::<u32>() {
        Ok(i) => {
            let mut index = 0;
            while index < i {
                index = index + 1;
                let _trd = thread::spawn(|| {
                    if let Err(err) = concurent() {
                        concurent();
                    }
                });
                thread_vector.push(_trd);
            }
            for x in thread_vector {
                let _ = x.join();
            }
        }
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            let _ = concurent();
        }
    };
}

fn concurent() -> Result<()> {
    let mut concurent_count: u64 = 0;
    let cfg = utils::loads();
    let client = utils::build_client();
    loop {
        concurent_count = concurent_count + 1;
        let resp: structs::ExperimentsResponse = client
            .get("https://discord.com/api/v9/experiments")
            .headers(headers::construct_headers_experiments())
            .send()
            .unwrap()
            .json()
            .unwrap();
        let _cookies = client.get("https://discord.com").send().unwrap();
        let fingerprint = utils::string_to_static_str(resp.fingerprint);
        let username = utils::get_username();
        let inv = utils::string_to_static_str(cfg.invite.clone());
        let captcha = utils::get_captcha();
        let payload_json = json!({
            "consent": true,
            "fingerprint": fingerprint,
            "username": username,
            "invite": inv,
            "captcha_key": captcha
        });
        let response_token = client
            .post("https://discord.com/api/v9/auth/register")
            .headers(headers::construct_headers_register(fingerprint))
            .json(&payload_json)
            .send()
            .unwrap();
        match response_token.status() {
            StatusCode::OK => {
                println!("Unexpected response code");
            }
            s => {
                if s == 400 {
                    println!("[{}] {}", "+".green(), "Missing fields".red().bold());
                } else if s == 429 {
                    println!(
                        "[{}] {}Resource ratelimited",
                        "-".red().bold(),
                        "(ERROR) ".yellow().bold()
                    )
                } else if s == 201 {
                    let response_json: structs::TokenResponse = response_token.json().unwrap();
                    println!(
                        "[{}] Generated: {}",
                        "+".blue().bold(),
                        response_json.token.blue().bold()
                    );
                    utils::write_to_file(&response_json.token);
                }
            }
        }
    }
}
