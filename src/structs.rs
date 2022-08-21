use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExperimentsResponse {
    pub fingerprint: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ExceptionTokenMissingFields {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProxyStruct {
    pub proxyless: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UsernameStruct {
    pub username_from_file: bool,
    pub username_prefix: String,
}

#[derive(Deserialize, Serialize)]
pub struct CaptchaStruct {
    pub api: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct CaptchaResponseOne {
    pub errorId: u32,
    pub taskId: u32,
}

#[derive(Deserialize)]
pub struct CaptchaResponseTwoStructResponseSolution {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize)]
pub struct CaptchaResponseTwo {
    pub status: String,
    pub solution: CaptchaResponseTwoStructResponseSolution,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub proxy: ProxyStruct,
    pub username: UsernameStruct,
    pub invite: String,
    pub captcha: CaptchaStruct,
}
