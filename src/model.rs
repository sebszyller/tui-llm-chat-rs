pub trait AR_LLM {
    pub fn generate_stream(prommpt: &str) -> String;
}

pub struct Model {
    pub model: u8,
}

impl AR_LLM for Model {
    fn generate_stream(prommpt: &str) -> String {
        return "caw caw".to_owned();
    }
}
