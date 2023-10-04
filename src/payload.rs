/*
TODOS:
1. Change to test file instead of main driver
2. Replace Unwraps in new_from_json to handle errors
3. New from json should return a result incase there is a parsing error
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
    pub fn new(end_point: &str, user_name: &str, avatar_url: &str) -> Payload {
        Payload {
            user_name: user_name.to_owned(),
            content: None,
            avatar_url: Some(avatar_url.to_owned()),
            end_point: Some(end_point.to_owned()),
        }
    }

    //fn to take a config file to setup the payload with the endpoint and user and user avatar
    pub fn new_from_json(file_path: &str) -> Result<Payload, &str> {
        let v: Value;

        if let Ok(s) = fs::read_to_string(file_path) {
            v = match serde_json::from_str(&s) {
                Ok(value) => value,
                Err(_) => return Err("Failed to parse JSON"),
            };
        } else {
            return Err("Failed to read json");
        }

        Ok(Payload {
            //TODO: replace with unwrap()s with a default behavior incase that field is not parsed from the json
            user_name: v["user_name"]
                .as_str()
                .ok_or("Failed to find user name in json file")?
                .to_owned(),
            content: None,
            avatar_url: Some(
                v["avatar_url"] //get value from the array parsed
                    .as_str() //returns an option some if there is a string none if not
                    .map(|s| s.to_owned()) // If there is an &str in Some, it will call to_owned to convert to String. If not map keeps it as None
                    .ok_or("Failed to find avatar_url")?, //converts this thing to a result if there is a Some(String) in it it will be Ok(Srting) if None -> Err. ? will unwrap or propagate the error
            ),
            end_point: Some(
                v["end_point"]
                    .as_str()
                    .map(|s| s.to_owned())
                    .ok_or("Could not find Endpoint in json file")?,
            ),
        })
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = Some(content.to_owned());
    } //content setter

/*     pub fn set_endpoint(&mut self, end_point: &str) -> Result<(), &str> {
        if self.end_point.is_none() {
            self.end_point = Some(end_point.to_owned());
        } else {
            return Err("End point already set! create new payload for new endpoint");
        }
        self.end_point = Some(end_point.to_owned());
        Ok(())
    } */

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

/*
----- TESTS ------
*/

#[cfg(test)]
mod tests {
    use super::Payload;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    //Test Payload
    #[test]
    fn new_payload() {
        let mut x = Payload::new("www.oogle.com", "user_name", "avatar_url");
        assert_eq!(&x.user_name, "user_name");
        assert_eq!(&x.avatar_url.clone().unwrap(), "avatar_url");
        assert_eq!(&x.end_point.clone().unwrap(), "www.oogle.com");

        //set endpoint
      //  assert!(x.set_endpoint("www.noogle.com").is_err());

        //content test
        assert!(&x.content.is_none());
        x.set_content("There once was a man from venus");
        assert!(&x.content.is_some());
        assert_eq!(
            &x.content.clone().unwrap(),
            "There once was a man from venus"
        );
    }
}
