pub struct State {
    pub message_history: Vec<(String, String)>,
}

impl State {
    pub fn new() -> State {
        Self {
            message_history: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.message_history = vec![];
    }

    pub fn add_user_msg(&mut self, msg: &str) {
        self.message_history.push((msg.to_owned(), "".to_string()));
    }

    pub fn add_assistant_msg(&mut self, msg: &str) {
        let pair = self.message_history.last_mut().unwrap();
        pair.1 = format!("{}{}", pair.1, msg);
    }
}
