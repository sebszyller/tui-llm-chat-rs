use llama_cpp::{standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams};
use log::debug;
use std::io::{self, Write};

pub struct LLM {
    model: LlamaModel,
    temperature: f32,
    top_p: f32,
    max_new_tokens: usize,
}

impl LLM {
    pub fn new(path_to_gguf: &str, temperature: f32, top_p: f32, max_new_tokens: usize) -> Self {
        let model = LlamaModel::load_from_file(path_to_gguf, LlamaParams::default())
            .expect("Could not load model");
        Self {
            model,
            temperature,
            top_p,
            max_new_tokens,
        }
    }

    pub fn generate_stream(&self, prompt: &str) -> String {
        debug!(
            "Generating with temp: {} | top_p: {} | max_new_tokens: {} | for prompt:\n{}",
            self.temperature, self.top_p, self.max_new_tokens, prompt
        );
        let mut accum = "".to_owned();
        let mut decoded_tokens = 0;

        let mut ctx = self
            .model
            .create_session(SessionParams::default())
            .expect("Failed to create session");

        ctx.advance_context(prompt).unwrap();

        let mut completions = ctx
            .start_completing_with(StandardSampler::default(), self.max_new_tokens)
            .unwrap()
            .into_strings();

        for completion in completions {
            print!("{completion}");
            let _ = io::stdout().flush();

            decoded_tokens += 1;
            accum = format!("{accum}{completion}");
            if decoded_tokens > self.max_new_tokens {
                break;
            }
        }
        accum.to_string()
    }
}
