use serde::Deserialize;
use std::{fs, f64::consts::E};
use serde_json::{Result, Value};


#[derive(Deserialize, Debug)]
pub struct Payload {
    user_name: String,
    content: Option<String>,
    title: Option<String>,
    avatar_url: String,
    end_point: String,
}

impl Payload {
    pub fn new(end_point: String, user_name: String, avatar_url: String) -> Payload {
        Payload {
            user_name,
            content: None,
            title: None,
            avatar_url,
            end_point,
        }
    }
    
    pub fn new_from_json(file_path: &str) -> Payload {

        let settings = fs::read_to_string(file_path).unwrap();
        let v: Value = serde_json::from_str(&settings).unwrap();
        Payload { 
            user_name: v["user_name"].as_str().unwrap().to_owned(), 
            content: None, 
            title: None, 
            avatar_url: v["avatar_url"].as_str().unwrap().to_owned(), 
            end_point: v["end_point"].as_str().unwrap().to_owned(), 
        }
        
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }
}
