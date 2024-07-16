use llama_cpp::{
    standard_sampler::StandardSampler, CompletionHandle, LlamaContextError, LlamaModel,
    LlamaParams, LlamaSession, SessionParams, TokensToStrings,
};
use log::debug;

pub struct LLM {
    system: String,
    session: LlamaSession,
    temperature: f32,
    top_p: f32,
    max_new_tokens: usize,
}

impl LLM {
    pub fn new(
        path_to_gguf: &str,
        system: &str,
        temperature: f32,
        top_p: f32,
        max_new_tokens: usize,
    ) -> Self {
        let model = LlamaModel::load_from_file(path_to_gguf, LlamaParams::default())
            .expect(&format!("Failed to load model from {}", path_to_gguf));
        let session = model
            .create_session(SessionParams::default())
            .expect("Failed to create a session");

        Self {
            system: system.to_string(),
            session,
            temperature,
            top_p,
            max_new_tokens,
        }
    }

    pub fn build_prompt(&self, message_history: &Vec<(String, String)>) -> String {
        // https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF
        assert!(message_history.len() > 0);
        let sent_start = "<s>";
        let sent_end = "</s>";
        let inst_start = "[INST]";
        let inst_end = "[/INST]";
        let mut text = format!("{sent_start}{inst_start} {}", self.system);
        for (i, (user, assistant)) in message_history.iter().enumerate() {
            text = if i == 0 {
                format!("{text} {user} {inst_end} ")
            } else {
                format!("{text} {inst_start} {user} {inst_end} ")
            };
            if !assistant.is_empty() {
                text = format!("{text}{assistant}{sent_end}");
            }
        }
        debug!("{text}");
        text
    }

    pub fn clear_session(&mut self) -> Result<(), LlamaContextError> {
        // self.session.truncate_context(0); // FIXME: something is wrong here
        self.session.set_context("")
    }

    pub fn prepare_completion_handle(
        &mut self,
        message_history: &Vec<(String, String)>,
    ) -> Result<TokensToStrings<CompletionHandle>, LlamaContextError> {
        let prompt = self.build_prompt(message_history);

        debug!(
            "Generating with temp: {} | top_p: {} | max_new_tokens: {} | for prompt:\n{}",
            self.temperature, self.top_p, self.max_new_tokens, prompt
        );

        self.session.advance_context(prompt)?;
        let completion_handle = self
            .session
            .start_completing_with(StandardSampler::default(), self.max_new_tokens)?
            .into_strings();

        Ok(completion_handle)
    }
}
