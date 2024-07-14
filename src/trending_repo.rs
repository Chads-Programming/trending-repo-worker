use reqwest::{Client, Error};
use worker::{console_error, console_warn};

use crate::consts::GITHUB_HOST;
use crate::models::{RepositoryItem, SearchRepositoriesResult, SearchRepositoryQuery};

pub async fn get_trending_repos(
    client: &Client,
    github_api_key: &str,
    query: &SearchRepositoryQuery,
    take: u64,
) -> Result<Vec<RepositoryItem>, Error> {
    let parser_query = query.get_raw_query();

    let repositories_result = client
        .get(format!(
            "{GITHUB_HOST}/search/repositories?q={parser_query}&per_page={take}"
        ))
        .header("Authorization", format!("Bearer {github_api_key}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await;

    let parsing_result = repositories_result
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))?
        .json::<SearchRepositoriesResult>()
        .await;

    match parsing_result {
        Ok(search) => Ok(search.items),
        Err(err) => {
            console_error!("Error parsing: {:?}", err.to_string());

            Err(err)
        }
    }
}
