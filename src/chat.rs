//pub train AR_LLM;

pub struct MessagePair {
    user: String,
    assistant: String,
}

pub struct Chat {
    message_history: Vec<MessagePair>,
    model: u8,
}

impl Chat {
    pub fn new(model: u8) -> Chat {
        Chat {
            message_history: vec![MessagePair {
                user: "user1".to_string(),
                assistant: "ass1".to_string(),
            }],
            model: model,
        }
    }

    pub fn generate(&self) -> String {
        return format!("MODEL: {} PROMPT: {}", self.model, &self.build_prompt());
    }

    pub fn add_user_msg(&mut self, msg: &str) {
        self.message_history.push(MessagePair {
            user: msg.to_string(),
            assistant: "".to_string(),
        });
    }

    pub fn add_assistant_msg(&mut self, msg: &str) {
        let pair = self.message_history.last_mut().unwrap();
        pair.assistant = msg.to_string();
    }

    fn build_prompt(&self) -> String {
        let mut text = "".to_string();
        for MessagePair { user, assistant } in self.message_history.iter() {
            text = format!("{}\nUser: {}\nAssistant: {}", text, user, assistant);
        }
        text
    }
}
