use serde::{Deserialize, Serialize};
pub struct SearchRepositoryQuery {
    min_stars: Option<u32>,
    topics: Vec<&'static str>,
}

impl SearchRepositoryQuery {
    pub fn new(min_stars: Option<u32>, topics: Vec<&'static str>) -> Self {
        Self { min_stars, topics }
    }

    pub fn get_raw_query(&self) -> String {
        let topics = format!("{} in:topics", self.topics.join(" "));

        let stars = match self.min_stars {
            Some(min_stars) => format!("stars:>={min_stars}"),
            None => String::new(),
        };

        String::from(format!("{topics} {stars}").trim())
    }
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SearchRepositoriesResult {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<RepositoryItem>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MinimalRepository {
    pub id: i64,
    pub full_name: String,
    pub description: String,
    pub stargazers_count: i64,
    pub url: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct RepositoryItem {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub homepage: String,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: String,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub master_branch: Option<String>,
    pub default_branch: String,
    pub score: f64,
    pub archive_url: Option<String>,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: String,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: String,
    pub license: Option<License>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct License {
    key: String,
    name: String,
    url: String,
    spdx_id: String,
    node_id: String,
    html_url: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Owner {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    owner_type: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    site_admin: bool,
}

impl RepositoryItem {
    pub fn get_minimal(&self) -> MinimalRepository {
        MinimalRepository {
            id: self.id,
            full_name: self.full_name.clone(),
            description: self.description.clone(),
            stargazers_count: self.stargazers_count,
            url: self.url.clone(),
        }
    }
}
