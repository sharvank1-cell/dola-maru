use crate::core::repository::RepoConfig;
use crate::core::git_operations::{add_all_changes, commit_changes, push_to_remote};
use anyhow::Result;
use git2::Repository;
use std::sync::{Arc, Mutex};

pub fn run_cli(config: Arc<Mutex<RepoConfig>>, message: &str, branch: &str) -> Result<()> {
    println!("Multi-Repo Pusher");
    println!("=================");
    
    let config_guard = config.lock().unwrap();
    
    // Get the current repository
    let repo = Repository::open(".")?;
    
    // Add all changes
    add_all_changes(&repo)?;
    
    // Commit changes
    commit_changes(&repo, message)?;
    
    // Push to all configured repositories
    for repo_info in &config_guard.repositories {
        println!("\nPushing to {} ({})...", repo_info.name, repo_info.url);
        match push_to_remote(&repo, repo_info, branch) {
            Ok(_) => println!("✓ Successfully pushed to {}", repo_info.name),
            Err(e) => println!("✗ Failed to push to {}: {}", repo_info.name, e),
        }
    }
    
    Ok(())
}