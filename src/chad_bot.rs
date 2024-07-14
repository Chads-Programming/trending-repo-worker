use crate::models::{MinimalRepository, RepositoryItem};
use reqwest::{Client, Error};
use serde_json::json;
use worker::{console_error, console_log, console_warn};

pub async fn publish_trending_repos(
    http: &Client,
    endpoint: &str,
    api_key: &str,
    repositories: Vec<RepositoryItem>,
) -> Result<(), Error> {
    if repositories.is_empty() {
        console_warn!("Empty repositories. Nobody will be send");

        return Ok(());
    }

    let minimal_repositories = repositories
        .into_iter()
        .map(|repo| repo.get_minimal())
        .collect::<Vec<MinimalRepository>>();

    let req = json!({
        "repositories": minimal_repositories
    });

    let request = http
        .post(endpoint)
        .header("content-type", "application/json")
        .header("Authorization", api_key)
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await;

    match request {
        Ok(response) => {
            console_log!("Repositories was published to bot: {:?}", response.status());

            Ok(())
        }
        Err(err) => {
            console_error!("Error on publish request: {:?}", err);

            Err(err)
        }
    }
}
