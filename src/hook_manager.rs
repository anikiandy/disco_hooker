#![allow(dead_code)]
use super::payload;
use crate::disco_hook;
use async_trait::async_trait;

//Hook_manager holds multiple payloads of various enpoints. It also stores common user name, avatar_url, and message to enable sending multiple messages
pub struct HookManager {
    hooks: Vec<payload::Payload>, //vector of payloads
    message: Option<String>,      //  Message
    avatar: Option<String>,       //avatar_url that new hooks will be created with
    user: Option<String>,         // user_name that new hooks will be created with
}

#[async_trait]
impl disco_hook::disco_hook<String> for HookManager {
    async fn send_hook(&mut self) -> Result<(), String> {
        self.send_messages().await
    }
}

impl HookManager {
    // takes a &str for an enpoint and pushes a new payload into the vector using the username and avatar url from struct
    pub fn add_hook(&mut self, end_point: &str) {
        self.hooks.push(payload::Payload::new(
            end_point,
            self.user.as_ref().unwrap(),
            self.avatar.as_ref().unwrap(),
        ));
    }

    //Constructor takes the user name and avatar url to populate hooks added
    pub fn new(user_name: &str, avatar_url: &str) -> HookManager {
        HookManager {
            hooks: Vec::new(),
            user: Some(user_name.to_owned()),
            avatar: Some(avatar_url.to_owned()),
            message: None,
        }
    }

    // Sets the message to be sent
    pub fn set_message(&mut self, message: &str) {
        match self.message {
            None => {
                // If it's None, set it to Some and tell them
                self.message = Some(message.to_owned());
                println!("Set message");
            }
            Some(_) => {
                // If it's Some that means it has been set already but not sent so print that
                self.message = Some(message.to_owned());
                println!("Replaced unsent message");
            }
        }
    }

    // Function will send all the messages if there is a message is not None
    pub async fn send_messages(&mut self) -> Result<(), String> {
        if self.message.is_none() {
            return Err("No message to send".to_owned());
        }
        for hook in self.hooks.iter_mut() {
            hook.set_content(self.message.clone().unwrap().as_str());
        }
        let futures: Vec<_> = self
            .hooks
            .iter()
            .map(|payload| payload.send_msg())
            .collect();
        let results: Vec<_> = futures::future::join_all(futures).await;
        for res in results {
            match res {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => println!("Failed to send a messagee: {}", e),
            }
        }
        //if we sent the message clear the message field so we dont double send
        self.message = None;
        Ok(())
    } // send_massages
}

//--------- TESTS ----------//

#[cfg(test)]
mod tests {
    use super::HookManager;
    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test] // TEst that we can create a new hook_manager struct
    fn hook_new() {
        let manager = HookManager::new(
            "andy",
            "https://i.etsystatic.com/6048333/r/il/3e80f0/2362984696/il_570xN.2362984696_kn27.jpg",
        );
        assert_eq!(manager.hooks.len(), 0);
    } //new hook test

    #[test] // Test that we can push a new hooker into the hooks vec
    fn hook_push() {
        let mut manager = HookManager::new(
            "andy",
            "https://i.etsystatic.com/6048333/r/il/3e80f0/2362984696/il_570xN.2362984696_kn27.jpg",
        );
        //add hook
        manager.add_hook("https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl");
        assert_eq!(manager.hooks.len(), 1);
        //add another hook
        manager.add_hook("https://discordapp.com/api/webhooks/1158622386074173500/h_I4fowNiVDtJmwSV3sqE6M1V6yxZpgsezBuuPQ5df9d3SXrQeSxHtZSG7DWma8hvGYk");
        assert_eq!(manager.hooks.len(), 2);
    } // hook push test

    #[tokio::test]
    async fn send_messages() {
        let mut manager = HookManager::new(
            "andy",
            "https://i.etsystatic.com/6048333/r/il/3e80f0/2362984696/il_570xN.2362984696_kn27.jpg",
        );
        //add hook
        manager.add_hook("https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl");
        //add another hook
        manager.add_hook("https://discordapp.com/api/webhooks/1158622386074173500/h_I4fowNiVDtJmwSV3sqE6M1V6yxZpgsezBuuPQ5df9d3SXrQeSxHtZSG7DWma8hvGYk");

        // message not set should fail
        assert!(manager.send_messages().await.is_err());

        //set message
        manager.set_message("This is a mesage");
        // Send message should be ok
        assert!(manager.send_messages().await.is_ok());

        // attempt to send again should fail because we are supposed to unset the message after its sent
        assert!(manager.send_messages().await.is_err());

        //add another message
        manager.set_message("This is a more different message");
        assert!(manager.send_messages().await.is_ok());
    } // send messages test
}
