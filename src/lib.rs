use axum::http::{HeaderMap, HeaderValue};
use models::SearchRepositoryQuery;
use reqwest::ClientBuilder;
use worker::*;

mod models;
mod services;

#[event(scheduled)]
async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    // let bot_api_key = env
    //     .secret("CHAD_BOT_API_KEY")
    //     .map(|e: Secret| e.to_string())
    //     .expect("Bot APIKEY Secret not found");

    let github_token = env
        .secret("GITHUB_TOKEN")
        .map(|e: Secret| e.to_string())
        .expect("Bot APIKEY Secret not found");

    let mut headers = HeaderMap::new();

    headers.append(
        "Accept",
        HeaderValue::from_str("application/vnd.github+json").unwrap(),
    );
    headers.append(
        "X-GitHub-Api-Version",
        HeaderValue::from_str("2022-11-28").unwrap(),
    );
    headers.append(
        "Authorization",
        HeaderValue::from_str(github_token.as_str()).unwrap(),
    );

    let client = ClientBuilder::default()
        .default_headers(headers)
        .user_agent("Mozilla/5.0 LeetCode API")
        .build()
        .expect("Cannot build client reqwest");

    let query = &SearchRepositoryQuery::new(Some(100), ["nextjs", "typescript"].to_vec());

    let result = services::trending_repositories(&client, query, 10).await;

    if let Ok(repos) = result {
        console_debug!("{:?}", { repos });
    } else {
        console_error!("{:?}", result.err());
    }
}
