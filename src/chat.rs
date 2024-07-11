use crate::model;
use log::debug;
use std::fmt;

#[derive(Debug)]
pub struct MessagePair {
    user: String,
    maybe_assistant: Option<String>,
}

impl fmt::Display for MessagePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user: {}, maybe_assistant: {}",
            self.user,
            self.maybe_assistant
                .clone()
                .unwrap_or("__NONE__".to_string())
        )
    }
}

pub struct Chat {
    system: String,
    message_history: Vec<MessagePair>,
    model: model::LLM,
}

impl Chat {
    pub fn new(model: model::LLM, system: &str) -> Chat {
        Self {
            system: system.to_owned(),
            message_history: vec![],
            model,
        }
    }

    pub fn clear(&mut self) {
        self.message_history = vec![];
    }

    pub fn generate(&mut self) -> String {
        let prompt = self.build_prompt();
        match self.model.generate_stream(&prompt) {
            Ok(response) => response,
            Err(_) => "Cannot generate anymore; clear!".to_string(),
        }
    }

    pub fn add_user_msg(&mut self, msg: &str) {
        self.message_history.push(MessagePair {
            user: msg.to_owned(),
            maybe_assistant: None,
        });
    }

    pub fn add_assistant_msg(&mut self, msg: &str) {
        let pair = self.message_history.last_mut().unwrap();
        pair.maybe_assistant = Some(msg.to_owned());
    }

    fn build_prompt(&self) -> String {
        // https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF
        assert!(self.message_history.len() > 0);
        let sent_start = "<s>";
        let sent_end = "</s>";
        let inst_start = "[INST]";
        let inst_end = "[/INST]";
        let mut text = format!("{sent_start}{inst_start} {}", self.system);
        for (
            i,
            MessagePair {
                user,
                maybe_assistant,
            },
        ) in self.message_history.iter().enumerate()
        {
            text = if i == 0 {
                format!("{text} {user} {inst_end} ")
            } else {
                format!("{text} {inst_start} {user} {inst_end} ")
            };
            if let Some(assistant) = maybe_assistant {
                text = format!("{text}{assistant}{sent_end}");
            }
        }
        debug!("{text}");
        text
    }
}
