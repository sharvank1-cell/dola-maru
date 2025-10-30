use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryInfo {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub auth_type: AuthType,
    #[serde(default)]
    pub auth_token: String,
    #[serde(default)]
    pub ssh_key_path: String,
}

impl RepositoryInfo {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            auth_type: AuthType::default(),
            auth_token: String::new(),
            ssh_key_path: String::new(),
        }
    }
    
    pub fn with_auth(name: String, url: String, auth_type: AuthType) -> Self {
        Self {
            name,
            url,
            auth_type,
            auth_token: String::new(),
            ssh_key_path: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AuthType {
    #[serde(rename = "ssh")]
    SSH,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "default")]
    Default,
}

impl Default for AuthType {
    fn default() -> Self {
        AuthType::Default
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub repositories: Vec<RepositoryInfo>,
    #[serde(default)]
    pub config_name: String,
}

impl RepoConfig {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            config_name: "default".to_string(),
        }
    }
    
    pub fn with_name(name: String) -> Self {
        Self {
            repositories: Vec::new(),
            config_name: name,
        }
    }
    
    pub fn add_repository(&mut self, repo: RepositoryInfo) {
        self.repositories.push(repo);
    }
    
    pub fn remove_repository(&mut self, index: usize) {
        if index < self.repositories.len() {
            self.repositories.remove(index);
        }
    }
    
    pub fn validate_repository_url(url: &str) -> bool {
        // Basic URL validation
        url.starts_with("https://") || url.starts_with("http://") || url.starts_with("git@")
    }
}