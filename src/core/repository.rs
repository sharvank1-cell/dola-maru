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
    #[serde(default)]
    pub group: String, // New field for repository grouping
}

impl RepositoryInfo {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            auth_type: AuthType::default(),
            auth_token: String::new(),
            ssh_key_path: String::new(),
            group: String::new(), // Default to no group
        }
    }
    
    pub fn with_auth(name: String, url: String, auth_type: AuthType) -> Self {
        Self {
            name,
            url,
            auth_type,
            auth_token: String::new(),
            ssh_key_path: String::new(),
            group: String::new(), // Default to no group
        }
    }
    
    // New method to set group for a repository
    pub fn with_group(mut self, group: String) -> Self {
        self.group = group;
        self
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

// New struct for repository groups
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryGroup {
    pub name: String,
    pub description: String,
    pub repository_names: Vec<String>, // Names of repositories in this group
}

impl RepositoryGroup {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            repository_names: Vec::new(),
        }
    }
    
    pub fn add_repository(&mut self, repo_name: String) {
        if !self.repository_names.contains(&repo_name) {
            self.repository_names.push(repo_name);
        }
    }
    
    pub fn remove_repository(&mut self, repo_name: &str) {
        self.repository_names.retain(|name| name != repo_name);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub repositories: Vec<RepositoryInfo>,
    #[serde(default)]
    pub config_name: String,
    #[serde(default)]
    pub groups: Vec<RepositoryGroup>, // New field for repository groups
}

impl RepoConfig {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            config_name: "default".to_string(),
            groups: Vec::new(), // Initialize with empty groups
        }
    }
    
    pub fn with_name(name: String) -> Self {
        Self {
            repositories: Vec::new(),
            config_name: name,
            groups: Vec::new(), // Initialize with empty groups
        }
    }
    
    pub fn add_repository(&mut self, repo: RepositoryInfo) {
        self.repositories.push(repo);
    }
    
    pub fn remove_repository(&mut self, index: usize) {
        if index < self.repositories.len() {
            // Remove this repository from any groups that contain it
            let repo_name = &self.repositories[index].name;
            for group in &mut self.groups {
                group.remove_repository(repo_name);
            }
            self.repositories.remove(index);
        }
    }
    
    // New methods for group management
    pub fn add_group(&mut self, group: RepositoryGroup) {
        self.groups.push(group);
    }
    
    pub fn remove_group(&mut self, group_name: &str) {
        self.groups.retain(|group| group.name != group_name);
    }
    
    pub fn get_group(&self, group_name: &str) -> Option<&RepositoryGroup> {
        self.groups.iter().find(|group| group.name == group_name)
    }
    
    pub fn get_group_mut(&mut self, group_name: &str) -> Option<&mut RepositoryGroup> {
        self.groups.iter_mut().find(|group| group.name == group_name)
    }
    
    // Get repositories belonging to a specific group
    pub fn get_repositories_in_group(&self, group_name: &str) -> Vec<&RepositoryInfo> {
        if let Some(group) = self.get_group(group_name) {
            self.repositories
                .iter()
                .filter(|repo| group.repository_names.contains(&repo.name))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    // Make this function publicly accessible
    pub fn validate_repository_url(url: &str) -> bool {
        // Basic URL validation
        url.starts_with("https://") || url.starts_with("http://") || url.starts_with("git@")
    }
}