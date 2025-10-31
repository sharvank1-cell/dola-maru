use anyhow::Result;
use serde::{Deserialize, Serialize};

// GitHub OAuth configuration
const GITHUB_CLIENT_ID: &str = "YOUR_GITHUB_CLIENT_ID";
const GITHUB_CLIENT_SECRET: &str = "YOUR_GITHUB_CLIENT_SECRET";
const GITHUB_REDIRECT_URI: &str = "http://localhost:8080/callback";

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAccessTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub email: Option<String>,
    pub name: Option<String>,
}

/// Get the GitHub OAuth authorization URL
pub fn get_github_auth_url() -> String {
    format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=repo,user",
        GITHUB_CLIENT_ID, GITHUB_REDIRECT_URI
    )
}

/// Exchange authorization code for access token
pub async fn exchange_code_for_token(code: &str) -> Result<GitHubAccessTokenResponse> {
    let client = reqwest::Client::new();
    
    let params = GitHubAccessTokenRequest {
        client_id: GITHUB_CLIENT_ID.to_string(),
        client_secret: GITHUB_CLIENT_SECRET.to_string(),
        code: code.to_string(),
        redirect_uri: GITHUB_REDIRECT_URI.to_string(),
    };
    
    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .await?;
        
    let token_response: GitHubAccessTokenResponse = response.json().await?;
    Ok(token_response)
}

/// Get user information using access token
pub async fn get_github_user(access_token: &str) -> Result<GitHubUser> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "Multi-Repo-Pusher")
        .send()
        .await?;
        
    let user: GitHubUser = response.json().await?;
    Ok(user)
}

/// Test if a GitHub access token is valid
pub async fn test_github_token(access_token: &str) -> Result<bool> {
    match get_github_user(access_token).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}