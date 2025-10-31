use crate::core::repository::{RepoConfig, RepositoryInfo};
use crate::core::git_operations::{
    push_to_remote, 
    pull_from_remote, 
    fetch_from_remote,
    add_all_changes,
    commit_changes
};
use crate::core::error_handler::format_error_result;
use git2::Repository;
use anyhow::Result;

/// Perform push operation on all repositories in a group
pub fn push_to_group_repositories(
    config: &RepoConfig, 
    group_name: &str, 
    commit_message: &str, 
    branch: &str
) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get repositories in the group
    let repositories = config.get_repositories_in_group(group_name);
    
    if repositories.is_empty() {
        results.push((group_name.to_string(), "No repositories found in group".to_string()));
        return results;
    }
    
    // Open the current repository
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            results.push(("Repository".to_string(), format!("Failed to open repository: {}", e)));
            return results;
        }
    };
    
    // Add all changes
    if let Err(e) = add_all_changes(&repo) {
        results.push(("Repository".to_string(), format!("Failed to add changes: {}", e)));
        return results;
    }
    
    // Commit changes
    if let Err(e) = commit_changes(&repo, commit_message) {
        results.push(("Repository".to_string(), format!("Failed to commit changes: {}", e)));
        return results;
    }
    
    // Push to each repository in the group
    for repo_info in repositories {
        let result = push_to_remote(&repo, repo_info, branch);
        results.push(format_error_result("pushing to", repo_info, result));
    }
    
    results
}

/// Perform pull operation on all repositories in a group
pub fn pull_from_group_repositories(
    config: &RepoConfig, 
    group_name: &str, 
    branch: &str
) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get repositories in the group
    let repositories = config.get_repositories_in_group(group_name);
    
    if repositories.is_empty() {
        results.push((group_name.to_string(), "No repositories found in group".to_string()));
        return results;
    }
    
    // Open the current repository
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            results.push(("Repository".to_string(), format!("Failed to open repository: {}", e)));
            return results;
        }
    };
    
    // Pull from each repository in the group
    for repo_info in repositories {
        let result = pull_from_remote(&repo, repo_info, branch);
        results.push(format_error_result("pulling from", repo_info, result));
    }
    
    results
}

/// Perform fetch operation on all repositories in a group
pub fn fetch_from_group_repositories(
    config: &RepoConfig, 
    group_name: &str, 
    branch: &str
) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get repositories in the group
    let repositories = config.get_repositories_in_group(group_name);
    
    if repositories.is_empty() {
        results.push((group_name.to_string(), "No repositories found in group".to_string()));
        return results;
    }
    
    // Open the current repository
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            results.push(("Repository".to_string(), format!("Failed to open repository: {}", e)));
            return results;
        }
    };
    
    // Fetch from each repository in the group
    for repo_info in repositories {
        let result = fetch_from_remote(&repo, repo_info, branch);
        results.push(format_error_result("fetching from", repo_info, result));
    }
    
    results
}