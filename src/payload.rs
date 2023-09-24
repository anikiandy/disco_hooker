use serde::Deserialize;
use serde_json::{Result as JsonResult, Value};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Payload {
    user_name: String,
    content: Option<String>,
    title: Option<String>,
    avatar_url: Option<String>,
    end_point: Option<String>,
}

impl Payload {
    pub fn new(user_name: String, avatar_url: String) -> Payload {
        Payload {
            user_name,
            content: None,
            title: None,
            avatar_url: Some(avatar_url),
            end_point: None,
        }
    }

    //fn to take a config file to setup the payload with the endpoint and user and user avatar
    pub fn new_from_json(file_path: &str) -> Payload {
        let settings = fs::read_to_string(file_path).unwrap();
        let v: Value = serde_json::from_str(&settings).unwrap();
        Payload {
            //TODO: replace with unwarp() with a default behavior incase that fiels is not parsed from the json
            user_name: v["user_name"].as_str().unwrap().to_owned(),
            content: None,
            title: None,
            avatar_url: Some(v["avatar_url"].as_str().unwrap().to_owned()),
            end_point: Some(v["end_point"].as_str().unwrap().to_owned()),
        }
    }

    pub fn set_endpoint(&mut self, end_point: String) -> Result<(), String> {
        match self.end_point {
            None => {
                self.end_point = Some(end_point);
                Ok(())
            }
            Some(_) => Err(String::from("Endpoint already set!")),
        }
    } // End set_endpoint

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }
}
