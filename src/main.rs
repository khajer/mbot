use std::fs;
use pulldown_cmark::{Parser, Event};

#[tokio::main]
async fn main() {
    let markdown_content = read_markdown_file("schedules/schedule.md");
    let tasks = parse_tasks(&markdown_content);
    
    for task in tasks {
        println!("{}", task);
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

pub fn parse_tasks(content: &str) -> Vec<String> {
    let mut tasks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines {
        if line.starts_with("- [") {
            let task_text = line.split("] ")
                .nth(1)
                .unwrap_or("");
            if !task_text.is_empty() {
                tasks.push(task_text.to_string());
            }
        }
    }
    
    tasks
}
