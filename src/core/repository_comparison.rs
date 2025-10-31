//! Repository comparison functionality for Multi-Repo Pusher
//! 
//! This module provides functionality to compare repositories and generate diffs
//! between different versions or branches.

use crate::core::repository::{RepositoryInfo, RepoConfig};
use anyhow::Result;
use git2::{Repository, DiffOptions, DiffFormat, DiffDelta, DiffHunk, DiffLine, Oid};
// use std::path::Path; // Not currently used

/// Struct to represent differences between repositories
#[derive(Debug, Clone)]
pub struct RepositoryDiff {
    pub repository_name: String,
    pub diff_content: String,
    pub stats: DiffStats,
}

/// Statistics about the diff
#[derive(Debug, Clone)]
pub struct DiffStats {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

impl DiffStats {
    pub fn new() -> Self {
        Self {
            files_changed: 0,
            insertions: 0,
            deletions: 0,
        }
    }
}

impl RepositoryDiff {
    pub fn new(repository_name: String, diff_content: String, stats: DiffStats) -> Self {
        Self {
            repository_name,
            diff_content,
            stats,
        }
    }
}

/// Generate a diff between two specific commits
pub fn generate_commit_diff(
    repo_info: &RepositoryInfo,
    repo_path: &str,
    commit1_oid: &str,
    commit2_oid: &str,
) -> Result<RepositoryDiff> {
    // Open the repository
    let repo = Repository::open(repo_path)?;
    
    // Parse the commit OIDs
    let oid1 = Oid::from_str(commit1_oid)?;
    let oid2 = Oid::from_str(commit2_oid)?;
    
    // Get the commits
    let commit1 = repo.find_commit(oid1)?;
    let commit2 = repo.find_commit(oid2)?;
    
    // Get the trees for the commits
    let tree1 = commit1.tree()?;
    let tree2 = commit2.tree()?;
    
    // Generate the diff
    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(Some(&tree1), Some(&tree2), Some(&mut diff_opts))?;
    
    // Collect diff content and statistics
    let mut diff_content = String::new();
    let mut files_changed = 0;
    let mut insertions = 0;
    let mut deletions = 0;
    
    diff.print(DiffFormat::Patch, |delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine| -> bool {
        // Count files (only count each file once)
        if delta.status() != git2::Delta::Unmodified {
            files_changed += 1;
        }
        
        // Count insertions and deletions
        match line.origin() {
            '+' => insertions += 1,
            '-' => deletions += 1,
            _ => {}
        }
        
        // Add line to diff content
        diff_content.push_str(&format!("{}{}", line.origin(), String::from_utf8_lossy(line.content())));
        true
    })?;
    
    let stats = DiffStats {
        files_changed,
        insertions,
        deletions,
    };
    
    Ok(RepositoryDiff::new(repo_info.name.clone(), diff_content, stats))
}

/// Generate a diff for a repository between two commits or branches
pub fn generate_repository_diff(
    repo_info: &RepositoryInfo,
    repo_path: &str,
    branch1: &str,
    branch2: &str,
) -> Result<RepositoryDiff> {
    // Open the repository
    let repo = Repository::open(repo_path)?;
    
    // Resolve the references for the branches/commits
    let commit1 = repo.revparse_single(branch1)?.peel_to_commit()?;
    let commit2 = repo.revparse_single(branch2)?.peel_to_commit()?;
    
    // Get the trees for the commits
    let tree1 = commit1.tree()?;
    let tree2 = commit2.tree()?;
    
    // Generate the diff
    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(Some(&tree1), Some(&tree2), Some(&mut diff_opts))?;
    
    // Collect diff content and statistics
    let mut diff_content = String::new();
    let mut files_changed = 0;
    let mut insertions = 0;
    let mut deletions = 0;
    
    diff.print(DiffFormat::Patch, |delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine| -> bool {
        // Count files
        if delta.status() != git2::Delta::Unmodified {
            files_changed += 1;
        }
        
        // Count insertions and deletions
        match line.origin() {
            '+' => insertions += 1,
            '-' => deletions += 1,
            _ => {}
        }
        
        // Add line to diff content
        diff_content.push_str(&format!("{}{}", line.origin(), String::from_utf8_lossy(line.content())));
        true
    })?;
    
    let stats = DiffStats {
        files_changed,
        insertions,
        deletions,
    };
    
    Ok(RepositoryDiff::new(repo_info.name.clone(), diff_content, stats))
}

/// Generate a diff for a repository between the working directory and HEAD
pub fn generate_working_directory_diff(
    repo_info: &RepositoryInfo,
    repo_path: &str,
) -> Result<RepositoryDiff> {
    // Open the repository
    let repo = Repository::open(repo_path)?;
    
    // Get the HEAD commit
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let head_tree = head_commit.tree()?;
    
    // Get the working directory diff
    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_workdir_with_index(Some(&head_tree), Some(&mut diff_opts))?;
    
    // Collect diff content and statistics
    let mut diff_content = String::new();
    let mut files_changed = 0;
    let mut insertions = 0;
    let mut deletions = 0;
    
    diff.print(DiffFormat::Patch, |delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine| -> bool {
        // Count files
        if delta.status() != git2::Delta::Unmodified {
            files_changed += 1;
        }
        
        // Count insertions and deletions
        match line.origin() {
            '+' => insertions += 1,
            '-' => deletions += 1,
            _ => {}
        }
        
        // Add line to diff content
        diff_content.push_str(&format!("{}{}", line.origin(), String::from_utf8_lossy(line.content())));
        true
    })?;
    
    let stats = DiffStats {
        files_changed,
        insertions,
        deletions,
    };
    
    Ok(RepositoryDiff::new(repo_info.name.clone(), diff_content, stats))
}

/// Compare two repositories by comparing their latest commits
pub fn compare_repositories(repo1: &RepositoryInfo, repo2: &RepositoryInfo) -> Result<RepositoryDiff> {
    // This is a simplified implementation
    // In a real implementation, you would need to:
    // 1. Clone both repositories locally if they're not already
    // 2. Compare specific branches or commits
    // 3. Generate a meaningful diff
    
    let diff_content = format!(
        "Comparison between {} and {}\n\nRepository comparison functionality would compare the contents of both repositories.",
        repo1.name, repo2.name
    );
    
    let stats = DiffStats {
        files_changed: 0,
        insertions: 0,
        deletions: 0,
    };
    
    Ok(RepositoryDiff::new(repo1.name.clone(), diff_content, stats))
}

/// Compare all repositories in a group
pub fn compare_group_repositories(
    config: &RepoConfig,
    group_name: &str,
) -> Result<Vec<RepositoryDiff>> {
    let repositories = config.get_repositories_in_group(group_name);
    
    if repositories.len() < 2 {
        return Ok(Vec::new());
    }
    
    let mut diffs = Vec::new();
    
    // Compare each pair of repositories in the group
    for i in 0..repositories.len() {
        for j in (i + 1)..repositories.len() {
            let diff = compare_repositories(repositories[i], repositories[j])?;
            diffs.push(diff);
        }
    }
    
    Ok(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::{AuthType, RepositoryGroup};
    
    #[test]
    fn test_diff_stats_creation() {
        let stats = DiffStats::new();
        assert_eq!(stats.files_changed, 0);
        assert_eq!(stats.insertions, 0);
        assert_eq!(stats.deletions, 0);
    }
    
    #[test]
    fn test_repository_diff_creation() {
        let stats = DiffStats::new();
        let diff = RepositoryDiff::new(
            "test_repo".to_string(),
            "diff content".to_string(),
            stats,
        );
        
        assert_eq!(diff.repository_name, "test_repo");
        assert_eq!(diff.diff_content, "diff content");
        assert_eq!(diff.stats.files_changed, 0);
    }
    
    #[test]
    fn test_compare_repositories() {
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        
        let result = compare_repositories(&repo1, &repo2);
        assert!(result.is_ok());
        
        let diff = result.unwrap();
        assert_eq!(diff.repository_name, "repo1");
        assert!(diff.diff_content.contains("Comparison between repo1 and repo2"));
    }
    
    #[test]
    fn test_compare_group_repositories_empty() {
        let mut config = RepoConfig::new();
        config.add_group(RepositoryGroup::new("empty_group".to_string(), "Empty group".to_string()));
        
        let result = compare_group_repositories(&config, "empty_group");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
    
    #[test]
    fn test_compare_group_repositories_single() {
        let mut config = RepoConfig::new();
        let mut group = RepositoryGroup::new("single_group".to_string(), "Single repo group".to_string());
        config.add_group(group.clone());
        
        let repo = RepositoryInfo::new("single_repo".to_string(), "https://github.com/user/repo.git".to_string());
        config.add_repository(repo.clone());
        group.add_repository("single_repo".to_string());
        
        let result = compare_group_repositories(&config, "single_group");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}