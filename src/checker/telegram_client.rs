use frankenstein::Api;
use frankenstein::Error;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct TelegramClient {
    api: Api,
    chat_id: i64,
}

impl TelegramClient {
    pub fn send_notification(&self, message: String) -> Result<(), Error> {
        let send_message_params = SendMessageParams::builder()
            .chat_id(self.chat_id)
            .text(message)
            .build();

        self.api.send_message(&send_message_params)?;

        Ok(())
    }
}
