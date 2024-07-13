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
pub struct RepositoryItem {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner,
    private: bool,
    html_url: String,
    description: String,
    fork: bool,
    url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    homepage: String,
    size: i64,
    stargazers_count: i64,
    watchers_count: i64,
    language: String,
    forks_count: i64,
    open_issues_count: i64,
    master_branch: Option<String>,
    default_branch: String,
    score: f64,
    archive_url: Option<String>,
    assignees_url: String,
    blobs_url: String,
    branches_url: String,
    collaborators_url: String,
    comments_url: String,
    commits_url: String,
    compare_url: String,
    contents_url: String,
    contributors_url: String,
    deployments_url: String,
    downloads_url: String,
    events_url: String,
    forks_url: String,
    git_commits_url: String,
    git_refs_url: String,
    git_tags_url: String,
    git_url: String,
    issue_comment_url: String,
    issue_events_url: String,
    issues_url: String,
    keys_url: String,
    labels_url: String,
    languages_url: String,
    merges_url: String,
    milestones_url: String,
    notifications_url: String,
    pulls_url: String,
    releases_url: String,
    ssh_url: String,
    stargazers_url: String,
    statuses_url: String,
    subscribers_url: String,
    subscription_url: String,
    tags_url: String,
    teams_url: String,
    trees_url: String,
    clone_url: String,
    mirror_url: Option<String>,
    hooks_url: String,
    svn_url: String,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    has_issues: bool,
    has_projects: bool,
    has_pages: bool,
    has_wiki: bool,
    has_downloads: bool,
    archived: bool,
    disabled: bool,
    visibility: String,
    license: Option<License>,
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