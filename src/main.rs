mod payload;
use payload::Payload;
use tokio::main;
use reqwest::{Client,Error};


#[tokio::main]
async fn main() -> Result<(),Error>{
    let _endpoint = "https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl";
                                 //        "https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl"
    let user_name = String::from("andy");
    let avatar_url = String::from("default");

    let mut hooker: Payload = Payload::new( user_name, avatar_url);

    hooker.set_content(String::from("boogers"));
    hooker.set_title(String::from("its a title"));

    println!("{:?}", hooker);
    let mut hooker2 = Payload::new_from_json("settings.json");

    println!("hooker2: {:?}", hooker2);

    let content = r#"{"content": "boogers","username": "sandy","avatar": "689161dc90ac261d00f1608694ac6bfd","application_id": "658822586720976555"}"#;
    let client = reqwest::Client::new();
    let res = client.post(_endpoint)
        .header("Content-Type", "application/json")
        .body(content.to_owned())
        .send()
        .await?;
    println!("didntfail {:?} ", res);
    Ok(())
}
