mod payload;
use payload::Payload;

fn main() {
    let endpoint = String::from("https://discord.com/api/bhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl");
    let user_name = String::from("andy");
    let avatar_url = String::from("default");

    let mut hooker: Payload = Payload::new( user_name, avatar_url);

    hooker.set_content(String::from("boogers"));
    hooker.set_title(String::from("its a title"));

    println!("{:?}", hooker);
    let mut hooker2 = Payload::new_from_json("settings.json");

    println!("hooker2: {:?}", hooker2);
}
