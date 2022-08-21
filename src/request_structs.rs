use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CaptchaTask {
    pub r#type: String,
    pub websiteURL: String,
    pub websiteKey: String,
    pub userAgent: String
}

#[derive(Deserialize, Serialize)]
pub struct CaptchaPayloadOne {
    pub clientKey: String,
    pub task: CaptchaTask
}

#[derive(Deserialize, Serialize)]
pub struct CaptchaPayloadTwo {
    pub clientKey: String,
    pub taskId: u32
}