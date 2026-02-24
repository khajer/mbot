use std::fs;
use std::net::SocketAddr;

use axum::{
    Json, Router,
    routing::{get, post},
};
use chrono::Local;
use pulldown_cmark::{Event, Parser};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let sched = JobScheduler::new().await?;

    let job1 = Job::new("0 * * * * *", |_uuid, _l| {
        info!(
            "Cron job runs every minute: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
    })?;
    sched.add(job1).await?;

    let job2 = Job::new_repeated(std::time::Duration::from_secs(30), |_uuid, _l| {
        info!(
            "Repeated job every 30 seconds: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
    })?;
    sched.add(job2).await?;

    sched.start().await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/version", get(get_version))
        .route("/tasks", get(get_tasks))
        .route("/tasks", post(add_task));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "mbot is running"
}

async fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

async fn get_tasks() -> Json<Vec<String>> {
    let markdown_content = read_markdown_file("schedules/schedule.md");
    let tasks = parse_tasks(&markdown_content);
    Json(tasks)
}

async fn add_task(Json(payload): Json<CreateTask>) -> Json<Task> {
    let task = Task {
        id: 1,
        name: payload.name,
    };
    Json(task)
}

#[derive(serde::Deserialize)]
struct CreateTask {
    name: String,
}

#[derive(serde::Serialize)]
struct Task {
    id: u32,
    name: String,
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
            let task_text = line.split("] ").nth(1).unwrap_or("");
            if !task_text.is_empty() {
                tasks.push(task_text.to_string());
            }
        }
    }

    tasks
}
