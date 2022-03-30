mod cli;

use {
    log::{error,info},
    crate::cli::cli_matches,
    rdkafka::{message::BorrowedMessage,Message},
    sbkafka::{subscribe,publisher}
};

fn process_msg(message:&BorrowedMessage) {
    let payload = message
    .payload_view::<str>()
    .unwrap_or(Ok(""))
    .unwrap_or_else(|e| {
        error!("Error while deserializing payload: {:?}", e);
        ""
    });
    info!("{:?}",payload);
}

#[tokio::main]
async fn main() {
    let args = cli_matches();
    let consume = subscribe;
    let _producer = publisher(&args);
    consume(&args,&process_msg).await;
}
