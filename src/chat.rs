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
            message_history: vec![],
            model: model,
        }
    }

    pub fn generate(&self) -> String {
        let mstr = &self.model.to_string();
        let prompt = &self.build_prompt();
        return format!("MODEL: {mstr}\nPROMPT:\n{prompt}");
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
        // hardcoded for llama3 https://llama.meta.com/docs/model-cards-and-prompt-formats/meta-llama-3/
        let system_prefix = "<|begin_of_text|><|start_header_id|>system<|end_header_id|>";
        let user_prefix = "<|start_header_id|>user<|end_header_id|>";
        let assistant_prefix = "<|start_header_id|>assistant<|end_header_id|>";
        let end_tag = "<|eot_id|>";
        let system = format!("{system_prefix}\nYou're a helpful chatbot that is always to the point, doesn't beat around the bush, and admits when it doesn't know a topic well.{end_tag}\n");

        assert!(self.message_history.len() > 0);
        let mut text = system;
        for MessagePair { user, assistant } in
            self.message_history[..self.message_history.len() - 1].iter()
        {
            text = format!(
                "{text}{user_prefix}\n{user}{end_tag}{assistant_prefix}\n{assistant}{end_tag}"
            );
        }
        let MessagePair { user, .. } = self.message_history.last().unwrap();
        text = format!("{text}{user_prefix}\n{user}{end_tag}{assistant_prefix}\n");
        text
    }
}
