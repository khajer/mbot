use std::fs;
use pulldown_cmark::{Parser, Event};

#[tokio::main]
async fn main() {
    let markdown_content = "# Hello\n\nThis is **bold** text.";
    let parser = Parser::new(markdown_content);

    for event in parser {
        match event {
            Event::Start(tag) => println!("Start: {:?}", tag),
            Event::End(tag) => println!("End: {:?}", tag),
            Event::Text(text) => println!("Text: {}", text),
            Event::Code(code) => println!("Code: {}", code),
            Event::SoftBreak => println!("SoftBreak"),
            Event::HardBreak => println!("HardBreak"),
            _ => {}
        }
    }
}

pub fn read_markdown_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read markdown file")
}

pub fn parse_markdown(content: &str) -> Vec<String> {
    let parser = Parser::new(content);
    let mut texts = Vec::new();

    for event in parser {
        if let Event::Text(text) = event {
            texts.push(text.to_string());
        }
    }

    texts
}
