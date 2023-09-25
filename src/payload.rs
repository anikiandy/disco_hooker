/*
TODOS:
1. Change to test file instead of main driver
2. Replace Unwraps in new_from_json to handle errors
*/

use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Payload {
    user_name: String,
    content: Option<String>,
    // title: Option<String>,
    avatar_url: Option<String>,
    end_point: Option<String>,
}

impl Payload {
    // Constructor that takes user name and avatar url
    #[allow(dead_code)]
    pub fn new(user_name: String, avatar_url: String) -> Payload {
        Payload {
            user_name,
            content: None,
            avatar_url: Some(avatar_url),
            end_point: None,
        }
    }

    //fn to take a config file to setup the payload with the endpoint and user and user avatar
    pub fn new_from_json(file_path: &str) -> Payload {
        let settings = fs::read_to_string(file_path).unwrap();
        let v: Value = serde_json::from_str(&settings).unwrap();
        Payload {
            //TODO: replace with unwrap()s with a default behavior incase that field is not parsed from the json
            user_name: v["user_name"].as_str().unwrap().to_owned(),
            content: None,
            avatar_url: Some(v["avatar_url"].as_str().unwrap().to_owned()),
            end_point: Some(v["end_point"].as_str().unwrap().to_owned()),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    //send message to discord should return an Error if there is no message
    pub async fn send_msg(&self) -> Result<(), &str> {
        //return err if there is no content
        if self.content.is_none() {
            return Err("No message content set");
        } else if self.end_point.is_none() {
            // Check if there is an endpoint
            return Err("No endpoint set");
        }

        // Send with reqwest
        let client = reqwest::Client::new();
        let res = client
            .post(self.end_point.as_ref().unwrap()) //safe to unwrap because checked already
            .header("Content-Type", "application/json")
            .body(self.fmt_payload())
            .send()
            .await;
        if res.is_err() {
            return Err("Recieved request error");
        }

        Ok(())
    } // send_msg fn

    fn fmt_payload(&self) -> String {
        let content = self.content.as_ref().unwrap();
        let avatar = self.avatar_url.as_ref().unwrap();
        format!(
            r#"{{"title": "TEST", "content":"{}", "username": "{}", "avatar_url": "{}"}}"#,
            content, self.user_name, avatar
        )
    }
}
