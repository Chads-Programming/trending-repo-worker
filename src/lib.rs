use models::SearchRepositoryQuery;
use reqwest::ClientBuilder;
use worker::*;

mod chad_bot;
mod consts;
mod models;
mod trending_repo;

#[event(scheduled)]
async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    let bot_api_key = env
        .secret("BOT_API_KEY")
        .map(|e: Secret| e.to_string())
        .expect("BOT_API_KEY Secret not found");

    let github_token = env
        .secret("API_GITHUB_TOKEN")
        .map(|e: Secret| e.to_string())
        .expect("API_GITHUB_TOKEN Secret not found");

    let bot_endoint = env
        .secret("BOT_ENDPOINT")
        .map(|e: Secret| e.to_string())
        .expect("BOT_ENDPOINT var not found");

    let client = ClientBuilder::default()
        .user_agent("Mozilla/5.0")
        .build()
        .expect("Cannot build client reqwest");

    let query = SearchRepositoryQuery::new(Some(100), ["nextjs", "typescript", "nestjs"].to_vec());
    let trending_response =
        trending_repo::get_trending_repos(&client, &github_token, &query, 5).await;

    let Ok(trending_repos) = trending_response else {
        let err = trending_response.err().unwrap();
        console_error!("{:?}", err);

        return;
    };

    chad_bot::publish_trending_repos(&client, &bot_endoint, &bot_api_key, trending_repos)
        .await
        .unwrap();
}
