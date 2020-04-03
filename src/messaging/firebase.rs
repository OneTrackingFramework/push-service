use std::error::Error;

use lazy_static::lazy_static;

lazy_static! {
    static ref FCM_CLIENT: fcm::Client = fcm::Client::new();
}

pub async fn send_message(
    api_key: &str,
    token: &str,
    title: &str,
    body: &str,
) -> Result<(), Box<dyn Error>> {
    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title(title);
    notification_builder.body(body);

    let notification = notification_builder.finalize();

    let mut message_builder = fcm::MessageBuilder::new(api_key, token);
    message_builder.notification(notification);

    let response = FCM_CLIENT.send(message_builder.finalize()).await?;
    info!("FCM client send: {:?}", response);
    Ok(())
}
