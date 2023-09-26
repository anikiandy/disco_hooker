mod payload;
use payload::Payload;

#[tokio::main]
async fn main() {
    let mut hooker2 = Payload::new_from_json("settings.json").unwrap();
    hooker2.set_content(String::from(
        "content hello I like boogers I like boogers do you like boogers?",
    ));
    hooker2.send_msg().await.expect("FAILED TO DO THE THING");
}
