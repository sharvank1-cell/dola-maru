use crate::core::repository::{RepoConfig, RepositoryInfo};
use git2::Repository;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryStats {
    pub name: String,
    pub total_commits: usize,
    pub total_files: usize,
    pub total_lines: usize,
    pub last_commit_date: Option<i64>,
    pub contributors: Vec<String>,
    pub branches: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupStats {
    pub name: String,
    pub total_repositories: usize,
    pub total_commits: usize,
    pub avg_commits_per_repo: f64,
    pub total_contributors: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OverallStats {
    pub total_repositories: usize,
    pub total_groups: usize,
    pub total_commits: usize,
    pub total_contributors: usize,
    pub repository_stats: Vec<RepositoryStats>,
    pub group_stats: Vec<GroupStats>,
}

impl RepositoryStats {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_commits: 0,
            total_files: 0,
            total_lines: 0,
            last_commit_date: None,
            contributors: Vec::new(),
            branches: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl GroupStats {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_repositories: 0,
            total_commits: 0,
            avg_commits_per_repo: 0.0,
            total_contributors: 0,
        }
    }
}

impl OverallStats {
    pub fn new() -> Self {
        Self {
            total_repositories: 0,
            total_groups: 0,
            total_commits: 0,
            total_contributors: 0,
            repository_stats: Vec::new(),
            group_stats: Vec::new(),
        }
    }
}

/// Collect statistics for a single repository
pub fn collect_repository_stats(repo_info: &RepositoryInfo, repo_path: &str) -> Result<RepositoryStats> {
    let mut stats = RepositoryStats::new(repo_info.name.clone());
    
    // Try to open the repository
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(_) => {
            // If we can't open the repository, return default stats
            return Ok(stats);
        }
    };
    
    // Collect commit history
    if let Ok(mut revwalk) = repo.revwalk() {
        revwalk.push_head().ok();
        
        let mut commit_count = 0;
        let mut contributors = std::collections::HashSet::new();
        let mut last_commit_date: Option<i64> = None;
        
        for oid in revwalk {
            if let Ok(oid) = oid {
                if let Ok(commit) = repo.find_commit(oid) {
                    commit_count += 1;
                    
                    // Collect contributor information
                    if let Some(author) = commit.author().name() {
                        contributors.insert(author.to_string());
                    }
                    
                    // Track last commit date
                    let commit_date = commit.author().when().seconds();
                    if last_commit_date.is_none() || Some(commit_date) > last_commit_date {
                        last_commit_date = Some(commit_date);
                    }
                }
            }
            
            // Limit to prevent long processing times
            if commit_count >= 1000 {
                break;
            }
        }
        
        stats.total_commits = commit_count;
        stats.contributors = contributors.into_iter().collect();
        stats.last_commit_date = last_commit_date;
    }
    
    // Collect branch information
    if let Ok(branches) = repo.branches(None) {
        stats.branches = branches
            .filter_map(|branch_result| {
                if let Ok((branch, _)) = branch_result {
                    if let Ok(name) = branch.name() {
                        name.map(|n| n.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
    }
    
    // Collect tag information
    if let Ok(tags) = repo.tag_names(None) {
        stats.tags = tags
            .iter()
            .filter_map(|tag| tag.map(|t| t.to_string()))
            .collect();
    }
    
    // Try to count files (this is a simplified approach)
    if let Ok(head) = repo.head() {
        if let Ok(head_commit) = head.peel_to_commit() {
            if let Ok(tree) = head_commit.tree() {
                let mut file_count = 0;
                tree.walk(git2::TreeWalkMode::PreOrder, |_root, entry| {
                    // Count only files, not directories
                    if entry.kind() == Some(git2::ObjectType::Blob) {
                        file_count += 1;
                    }
                    git2::TreeWalkResult::Ok
                }).ok();
                
                stats.total_files = file_count;
            }
        }
    }
    
    Ok(stats)
}

/// Collect statistics for all repositories in a configuration
pub fn collect_overall_stats(config: &RepoConfig) -> Result<OverallStats> {
    let mut overall_stats = OverallStats::new();
    
    // Collect stats for each repository
    for repo_info in &config.repositories {
        // For now, we'll assume repositories are in the current directory
        // In a real implementation, you might want to store repository paths
        let repo_path = ".";
        if let Ok(repo_stats) = collect_repository_stats(repo_info, repo_path) {
            overall_stats.total_commits += repo_stats.total_commits;
            overall_stats.repository_stats.push(repo_stats);
        }
    }
    
    overall_stats.total_repositories = config.repositories.len();
    overall_stats.total_groups = config.groups.len();
    
    // Collect unique contributors across all repositories
    let mut all_contributors = std::collections::HashSet::new();
    for repo_stats in &overall_stats.repository_stats {
        for contributor in &repo_stats.contributors {
            all_contributors.insert(contributor.clone());
        }
    }
    overall_stats.total_contributors = all_contributors.len();
    
    // Collect group statistics
    for group in &config.groups {
        let mut group_stats = GroupStats::new(group.name.clone());
        group_stats.total_repositories = group.repository_names.len();
        
        // Calculate total commits for repositories in this group
        let mut group_commits = 0;
        let mut repo_count = 0;
        
        for repo_name in &group.repository_names {
            if let Some(repo_stats) = overall_stats.repository_stats.iter().find(|r| &r.name == repo_name) {
                group_commits += repo_stats.total_commits;
                repo_count += 1;
            }
        }
        
        group_stats.total_commits = group_commits;
        if repo_count > 0 {
            group_stats.avg_commits_per_repo = group_commits as f64 / repo_count as f64;
        }
        
        overall_stats.group_stats.push(group_stats);
    }
    
    Ok(overall_stats)
}