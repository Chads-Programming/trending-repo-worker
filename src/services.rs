use reqwest::{Client, Error};
use worker::{console_debug, console_error, console_warn};

use crate::models::{RepositoryItem, SearchRepositoriesResult, SearchRepositoryQuery};

const GITHUB_HOST: &str = "https://api.github.com";

pub async fn trending_repositories(
    client: &Client,
    query: &SearchRepositoryQuery,
    take: u64,
) -> Result<Vec<RepositoryItem>, Error> {
    let parser_query = query.get_raw_query();
    let uri = format!("{GITHUB_HOST}/search/repositories?q={parser_query}&per_page={take}");

    console_debug!("{uri}");

    let repositories_result = client
        .get(format!(
            "{GITHUB_HOST}/search/repositories?q={parser_query}&per_page={take}"
        ))
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))?
        .json::<SearchRepositoriesResult>()
        .await;

    match repositories_result {
        Ok(search) => Ok(search.items),
        Err(err) => {
            console_error!("Error parsing: {:?}", err.to_string());

            Err(err)
        }
    }
}
