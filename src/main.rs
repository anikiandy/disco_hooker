mod hook_manager;
mod payload;
use payload::Payload;

#[tokio::main]
async fn main() {
    let mut hooker2 = Payload::new_from_json("settings.json").unwrap();
    hooker2.set_content("content hello I like boogers I like boogers do you like boogers?");
    hooker2.send_msg().await.expect("FAILED TO DO THE THING");

    let mut hooker1 = Payload::new(
        "https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl",
        "andy",
        "https://i.etsystatic.com/6048333/r/il/3e80f0/2362984696/il_570xN.2362984696_kn27.jpg",
    );

    hooker1.set_content("OOGA BOOGA BOOGER");
    //hooker1.send_msg().await.expect("failed");

    let mut hookManager = hook_manager::HookManager::new(
        "andy",
        "https://i.etsystatic.com/6048333/r/il/3e80f0/2362984696/il_570xN.2362984696_kn27.jpg",
    );

    hookManager.add_hook("https://discordapp.com/api/webhooks/1021825932442472488/DmLKFLggwYPT625-XvttvxA1v4Iqwk7nM0-k75J7UuBqxEqVKX44a-mbsqldgcGJE4Pl");
    hookManager.set_message("testing testing 123");

    let nobody_carres = send_hook(&mut hookManager).await;
    let nobody_cares = send_hook(&mut hooker2).await;
    send_hook(&mut hooker1).await.expect("boogers")
} // END MAIN

// Driver to test the disco_hook trait for send_hook
async fn send_hook<T>(hook: &mut impl disco_hook::DiscoHook<T>) -> Result<(), T> {
    hook.send_hook().await
}

// Module for the trait
mod disco_hook {
    use async_trait::async_trait;
    #[async_trait]
    pub trait DiscoHook<E> {
        async fn send_hook(&mut self) -> Result<(), E>;
    }
}
