use futures::stream::StreamExt;
use teloxide::{
    dispatching::{
        chat::{ChatUpdate, ChatUpdateKind, Dispatcher},
        update_listeners::polling_default,
        SessionState,
    },
    requests::Request,
    Bot,
};

#[tokio::main]
async fn main() {
    let bot = &Bot::new("1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0");
    let mut updater = Box::pin(polling_default(bot));
    let handler = |_, upd: ChatUpdate| async move {
        if let ChatUpdateKind::Message(m) = upd.kind {
            let msg = bot.send_message(m.chat.id, "pong");
            msg.send().await.unwrap();
        }
        SessionState::Continue(())
    };
    let mut dp = Dispatcher::<'_, (), _>::new(handler);
    println!("Starting the message handler.");
    loop {
        let u = updater.next().await.unwrap();
        match u {
            Err(e) => eprintln!("Error: {}", e),
            Ok(u) => {
                let _ = dp.dispatch(u).await;
            }
        }
    }
}
