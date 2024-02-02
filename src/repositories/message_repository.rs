use crate::entities::message::Message;

pub trait MessageRepository {
    fn addMessage(&mut self, message: Message);
}

struct InMemoryMessageRepository {
    messages: Vec<Message>
}

impl MessageRepository for InMemoryMessageRepository {
    fn addMessage(&mut self, message: Message) {
        self.messages.push(message);
    }
}