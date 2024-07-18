# tui-llm-chat-rs

Capstone project for my blog series on [Learning Rust with LLMs](https://sebszyller.com/blog/2024/rustwithllmsstart) (and the [series repo](https://github.com/sebszyller/rust-with-llms)).

<img width="1602" alt="image" src="https://github.com/user-attachments/assets/3f12c024-46a7-436d-9ff4-3f4c0cd320b2">


It's hardcoded for:
- Mistral Instruct,
- Termion terminals,
- Apple Metal.

¯\\\_(ツ)\_/¯

You can download the model from [hugginface](https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF).
Provide the path via CLI args, or just put it in the project root.

`cargo run` should be enough.

Run with `-h` for help:
```
Learning Rust with LLMs Capstone Project: TUI LLM Chat

Usage: tui-llm-chat-rs [OPTIONS]

Options:
      --path_to_model <PATH_TO_MODEL>    Path to gguf file
      --temperature <TEMPERATURE>        Sampling temperature
      --top_p <TOP_P>                    Use top_p tokens
      --max_new_tokens <MAX_NEW_TOKENS>  Generate at most max_new_tokens
      --inline_lines <INLINE_LINES>      Number of lines for the TUI
  -h, --help                             Print help
  -V, --version                          Print version
```
