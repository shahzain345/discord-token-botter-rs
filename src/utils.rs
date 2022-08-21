use rand::{distributions::Alphanumeric, Rng};
use reqwest::StatusCode;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Read};
use std::time::Duration;
#[path = "./headers.rs"]
mod headers;
#[path = "./request_structs.rs"]
mod request_structs;
#[path = "./structs.rs"]
mod structs;
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn loads() -> structs::Config {
    let mut file = File::open("config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let static_str = string_to_static_str(data);
    let config: structs::Config = serde_json::from_str(static_str).unwrap();
    config
}

pub fn read_all_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines_vec = vec![];
    for line in reader.lines() {
        lines_vec.push(line.unwrap());
    }
    lines_vec
}

pub fn random(mut vector: Vec<String>) -> String {
    let index = (rand::random::<f32>() * vector.len() as f32).floor() as usize;
    let value = vector.remove(index);
    value
}

pub fn rand_str() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    s
}

pub fn get_username() -> String {
    let cfg = loads();
    if cfg.username.username_from_file == true {
        let usernames = read_all_lines("input/usernames.txt");
        let username = random(usernames);
        username
    } else {
        let username = format!("{} | {}", cfg.username.username_prefix, rand_str());
        username
    }
}

pub fn get_captcha() -> String {
    let cfg = loads();
    if cfg.captcha.api == "anti-captcha.com" {
        let client = build_client();
        let key = cfg.captcha.key;
        let key_value = string_to_static_str(key.clone());
        let payload = request_structs::CaptchaPayloadOne {
            clientKey: key_value.to_string(),
            task: request_structs::CaptchaTask {
                r#type: "HCaptchaTaskProxyless".to_string(),
                websiteURL: "https://discord.com/".to_string(),
                websiteKey: "4c672d35-0701-42b2-88c3-78380b0db560".to_string(),
                userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.2 Safari/605.1.15".to_string()
            }
        };
        let task = client
            .post("https://api.anti-captcha.com/createTask")
            .json(&payload)
            .send()
            .unwrap();
        let mut response = "";
        match task.status() {
            StatusCode::OK => {
                let task_response: structs::CaptchaResponseOne = task.json().unwrap();
                while response == "" {
                    let response_payload = request_structs::CaptchaPayloadTwo {
                        clientKey: key_value.to_string(),
                        taskId: task_response.taskId,
                    };
                    let task_resp_two = client
                        .post("https://api.anti-captcha.com/getTaskResult")
                        .json(&response_payload)
                        .send()
                        .unwrap();
                    let json_resp: serde_json::Value =
                        serde_json::from_str(&task_resp_two.text().unwrap()).unwrap();
                    if json_resp["status"]
                        .to_string()
                        .replace('"', "")
                        .replace("/", "")
                        == "ready"
                    {
                        response = string_to_static_str(
                            json_resp["solution"]["gRecaptchaResponse"]
                                .to_string()
                                .replace('"', "")
                                .replace("/", ""),
                        );
                    }
                }
            }
            s => {
                println!("Unexpected status code: {}", s);
            }
        }
        response.to_string()
    } else {
        let client = build_client();
        let key = cfg.captcha.key;
        let key_value = string_to_static_str(key.clone());
        let payload = request_structs::CaptchaPayloadOne {
            clientKey: key_value.to_string(),
            task: request_structs::CaptchaTask {
                r#type: "HCaptchaTaskProxyless".to_string(),
                websiteURL: "https://discord.com/".to_string(),
                websiteKey: "4c672d35-0701-42b2-88c3-78380b0db560".to_string(),
                userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.2 Safari/605.1.15".to_string()
            }
        };
        let task = client
            .post("https://api.capmonster.cloud/createTask")
            .json(&payload)
            .send()
            .unwrap();
        let mut response = "";
        match task.status() {
            StatusCode::OK => {
                let task_response: structs::CaptchaResponseOne = task.json().unwrap();
                while response == "" {
                    let response_payload = request_structs::CaptchaPayloadTwo {
                        clientKey: key_value.to_string(),
                        taskId: task_response.taskId,
                    };
                    let task_resp_two = client
                        .post("https://api.capmonster.cloud/getTaskResult")
                        .json(&response_payload)
                        .send()
                        .unwrap();
                    let json_resp: serde_json::Value =
                        serde_json::from_str(&task_resp_two.text().unwrap()).unwrap();
                    if json_resp["status"]
                        .to_string()
                        .replace('"', "")
                        .replace("/", "")
                        == "ready"
                    {
                        response = string_to_static_str(
                            json_resp["solution"]["gRecaptchaResponse"]
                                .to_string()
                                .replace('"', "")
                                .replace("/", ""),
                        );
                    }
                }
            }
            s => {
                println!("Unexpected status code: {}", s);
            }
        }
        response.to_string()
    }
}

pub fn write_to_file(token: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output/tokens.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", &token) {
        println!("Couldn't write to file: {}", e);
    }
}

pub fn build_client() -> reqwest::blocking::Client {
    let cfg = loads();
    if cfg.proxy.proxyless == true {
        let client = reqwest::blocking::Client::new();
        client
    } else {
        let proxies = read_all_lines("input/proxies.txt");
        let proxy_str = random(proxies);
        let proxy = reqwest::Proxy::all(format!("http://{}", proxy_str)).unwrap();
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .cookie_store(true)
            .proxy(proxy)
            .build()
            .unwrap();
        client
    }
}
