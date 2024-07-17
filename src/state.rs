use llama_cpp::{CompletionHandle, TokensToStrings};

pub enum ModelState {
    Waiting,
    ProcessingContext,
    Generating,
}

pub struct State {
    pub message_history: Vec<(String, String)>,
    pub model_state: ModelState,
    pub completion_handle: Option<TokensToStrings<CompletionHandle>>,
}

impl State {
    pub fn new() -> State {
        Self {
            message_history: vec![],
            model_state: ModelState::Waiting,
            completion_handle: None,
        }
    }

    pub fn clear(&mut self) {
        self.message_history = vec![];
        self.model_state = ModelState::Waiting;
        self.completion_handle = None;
    }

    pub fn add_user_msg(&mut self, msg: &str) {
        self.message_history.push((msg.to_owned(), "".to_string()));
    }

    pub fn add_assistant_msg(&mut self, msg: &str) {
        let pair = self.message_history.last_mut().unwrap();
        pair.1 = format!("{}{}", pair.1, msg);
    }

    pub fn set_completion(&mut self, completions: Option<TokensToStrings<CompletionHandle>>) {
        self.completion_handle = completions;
    }

    pub fn now_generating(&mut self) {
        self.model_state = ModelState::Generating;
    }

    pub fn now_processing(&mut self) {
        self.model_state = ModelState::ProcessingContext;
    }

    pub fn now_waiting(&mut self) {
        self.model_state = ModelState::Waiting;
    }
}
