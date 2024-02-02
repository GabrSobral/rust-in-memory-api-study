use crate::{
    entities::message::Message, 
    repositories::message_repository::MessageRepository
};

pub struct MessageService {
    messageRepository: Box<dyn MessageRepository>
}

impl MessageService {
    pub fn saveMessage(&mut self, message: Message) -> Result<Message, &'static str> {
        self.messageRepository.addMessage(message.clone());

        Ok(message)
    }
}