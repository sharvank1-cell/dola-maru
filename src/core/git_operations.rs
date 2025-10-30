use crate::core::repository::{RepositoryInfo, RepoConfig, AuthType};
use git2::Repository;
use anyhow::Result;
use std::path::Path;

pub fn add_all_changes(repo: &Repository) -> Result<()> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn commit_changes(repo: &Repository, message: &str) -> Result<git2::Oid> {
    let signature = repo.signature()?;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let parent_commit = if let Ok(head) = repo.head() {
        head.target().map(|target| repo.find_commit(target)).transpose()?
    } else {
        None
    };
    
    let commit_oid = if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent],
        )?
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[],
        )?
    };
    
    Ok(commit_oid)
}

pub fn push_to_remote(repo: &Repository, repo_info: &RepositoryInfo, branch: &str) -> Result<()> {
    // Try to find existing remote or create new one
    let mut remote = match repo.find_remote(&repo_info.name) {
        Ok(remote) => remote,
        Err(_) => {
            repo.remote(&repo_info.name, &repo_info.url)?
        }
    };
    
    // Configure callbacks for authentication based on auth type
    let mut callbacks = git2::RemoteCallbacks::new();
    
    match &repo_info.auth_type {
        AuthType::SSH => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");
                git2::Cred::ssh_key(
                    username,
                    None,
                    Path::new(&repo_info.ssh_key_path),
                    None,
                )
            });
        },
        AuthType::Token => {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                // For token-based auth, we typically use username/password with token as password
                git2::Cred::userpass_plaintext("token", &repo_info.auth_token)
            });
        },
        AuthType::Default => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    None,
                )
                .or_else(|_| git2::Cred::default())
            });
        }
    }
    
    // Push to remote
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
    remote.push(&[&refspec], Some(&mut push_options))?;
    
    Ok(())
}

pub fn pull_from_remote(repo: &Repository, repo_info: &RepositoryInfo, branch: &str) -> Result<()> {
    // Try to find existing remote or create new one
    let mut remote = match repo.find_remote(&repo_info.name) {
        Ok(remote) => remote,
        Err(_) => {
            repo.remote(&repo_info.name, &repo_info.url)?
        }
    };
    
    // Configure callbacks for authentication based on auth type
    let mut callbacks = git2::RemoteCallbacks::new();
    
    match &repo_info.auth_type {
        AuthType::SSH => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");
                git2::Cred::ssh_key(
                    username,
                    None,
                    Path::new(&repo_info.ssh_key_path),
                    None,
                )
            });
        },
        AuthType::Token => {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext("token", &repo_info.auth_token)
            });
        },
        AuthType::Default => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    None,
                )
                .or_else(|_| git2::Cred::default())
            });
        }
    }
    
    // Fetch from remote
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    remote.fetch(&[branch], Some(&mut fetch_options), None)?;
    
    // Merge fetched changes
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    
    // Perform merge
    let mut merge_options = git2::MergeOptions::new();
    repo.merge(&[&fetch_commit], Some(&mut merge_options), None)?;
    
    // Check for conflicts
    let index = repo.index()?;
    if index.has_conflicts() {
        return Err(anyhow::anyhow!("Merge conflicts detected"));
    }
    
    Ok(())
}

pub fn fetch_from_remote(repo: &Repository, repo_info: &RepositoryInfo, branch: &str) -> Result<()> {
    // Try to find existing remote or create new one
    let mut remote = match repo.find_remote(&repo_info.name) {
        Ok(remote) => remote,
        Err(_) => {
            repo.remote(&repo_info.name, &repo_info.url)?
        }
    };
    
    // Configure callbacks for authentication based on auth type
    let mut callbacks = git2::RemoteCallbacks::new();
    
    match &repo_info.auth_type {
        AuthType::SSH => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");
                git2::Cred::ssh_key(
                    username,
                    None,
                    Path::new(&repo_info.ssh_key_path),
                    None,
                )
            });
        },
        AuthType::Token => {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext("token", &repo_info.auth_token)
            });
        },
        AuthType::Default => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    None,
                )
                .or_else(|_| git2::Cred::default())
            });
        }
    }
    
    // Fetch from remote
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    remote.fetch(&[branch], Some(&mut fetch_options), None)?;
    
    Ok(())
}

pub fn create_and_push_tag(repo: &Repository, repo_info: &RepositoryInfo, tag_name: &str, message: &str) -> Result<()> {
    // Get the current HEAD commit
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    
    // Create annotated tag
    let signature = repo.signature()?;
    repo.tag(tag_name, commit.as_object(), &signature, message, false)?;
    
    // Push tag to remote
    let mut remote = match repo.find_remote(&repo_info.name) {
        Ok(remote) => remote,
        Err(_) => {
            repo.remote(&repo_info.name, &repo_info.url)?
        }
    };
    
    // Configure callbacks for authentication based on auth type
    let mut callbacks = git2::RemoteCallbacks::new();
    
    match &repo_info.auth_type {
        AuthType::SSH => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");
                git2::Cred::ssh_key(
                    username,
                    None,
                    Path::new(&repo_info.ssh_key_path),
                    None,
                )
            });
        },
        AuthType::Token => {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext("token", &repo_info.auth_token)
            });
        },
        AuthType::Default => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    None,
                )
                .or_else(|_| git2::Cred::default())
            });
        }
    }
    
    // Push tag to remote
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    let refspec = format!("refs/tags/{}:refs/tags/{}", tag_name, tag_name);
    remote.push(&[&refspec], Some(&mut push_options))?;
    
    Ok(())
}

pub fn check_merge_conflicts(repo: &Repository) -> Result<bool> {
    // Check if there are any merge conflicts in the index
    let index = repo.index()?;
    
    // Check if index has conflicts
    Ok(index.has_conflicts())
}

pub fn push_to_all_repositories(config: &RepoConfig, commit_message: &str, branch: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get the current repository
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
    
    // Push to all repositories
    for repo_info in &config.repositories {
        match push_to_remote(&repo, repo_info, branch) {
            Ok(_) => {
                results.push((repo_info.name.clone(), "Success".to_string()));
            }
            Err(e) => {
                results.push((repo_info.name.clone(), format!("Failed: {}", e)));
            }
        }
    }
    
    results
}

pub fn pull_from_all_repositories(config: &RepoConfig, branch: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get the current repository
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            results.push(("Repository".to_string(), format!("Failed to open repository: {}", e)));
            return results;
        }
    };
    
    // Pull from all repositories
    for repo_info in &config.repositories {
        match pull_from_remote(&repo, repo_info, branch) {
            Ok(_) => {
                results.push((repo_info.name.clone(), "Success".to_string()));
            }
            Err(e) => {
                results.push((repo_info.name.clone(), format!("Failed: {}", e)));
            }
        }
    }
    
    results
}

pub fn fetch_from_all_repositories(config: &RepoConfig, branch: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    // Get the current repository
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            results.push(("Repository".to_string(), format!("Failed to open repository: {}", e)));
            return results;
        }
    };
    
    // Fetch from all repositories
    for repo_info in &config.repositories {
        match fetch_from_remote(&repo, repo_info, branch) {
            Ok(_) => {
                results.push((repo_info.name.clone(), "Success".to_string()));
            }
            Err(e) => {
                results.push((repo_info.name.clone(), format!("Failed: {}", e)));
            }
        }
    }
    
    results
}

// Repository validation functions
pub fn validate_repository_url(url: &str) -> bool {
    // Basic URL validation
    url.starts_with("https://") || url.starts_with("http://") || url.starts_with("git@")
}

pub fn verify_authentication(repo_info: &RepositoryInfo) -> Result<bool> {
    // This is a simplified authentication verification
    // In a real implementation, you would actually test the credentials
    match &repo_info.auth_type {
        AuthType::SSH => {
            // Check if SSH key file exists
            if !repo_info.ssh_key_path.is_empty() {
                Ok(Path::new(&repo_info.ssh_key_path).exists())
            } else {
                // Check default SSH key location
                let default_key = format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default());
                Ok(Path::new(&default_key).exists())
            }
        },
        AuthType::Token => {
            // Check if token is provided
            Ok(!repo_info.auth_token.is_empty())
        },
        AuthType::Default => {
            // For default, we assume it works
            Ok(true)
        }
    }
}

// New function to clone a repository
pub fn clone_repository(repo_info: &RepositoryInfo, destination_path: &str) -> Result<Repository> {
    // Configure callbacks for authentication based on auth type
    let mut callbacks = git2::RemoteCallbacks::new();
    
    match &repo_info.auth_type {
        AuthType::SSH => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");
                git2::Cred::ssh_key(
                    username,
                    None,
                    Path::new(&repo_info.ssh_key_path),
                    None,
                )
            });
        },
        AuthType::Token => {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                // For GitHub, we can use the token as username with 'x-oauth-basic' as password
                git2::Cred::userpass_plaintext(&repo_info.auth_token, "x-oauth-basic")
            });
        },
        AuthType::Default => {
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    None,
                )
                .or_else(|_| git2::Cred::default())
            });
        }
    }
    
    // Configure clone options
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    
    // Clone the repository
    let repo = builder.clone(&repo_info.url, Path::new(destination_path))?;
    
    // Add remote with the repository name
    repo.remote(&repo_info.name, &repo_info.url)?;
    
    Ok(repo)
}

// New function to clone all repositories in a configuration
pub fn clone_all_repositories(config: &RepoConfig, base_path: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    for repo_info in &config.repositories {
        let destination_path = format!("{}/{}", base_path, repo_info.name);
        
        match clone_repository(repo_info, &destination_path) {
            Ok(_) => {
                results.push((repo_info.name.clone(), "Success".to_string()));
            }
            Err(e) => {
                results.push((repo_info.name.clone(), format!("Failed: {}", e)));
            }
        }
    }
    
    results
}
