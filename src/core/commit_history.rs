use git2::{Repository, Oid, Commit, DiffOptions, DiffDelta};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommitInfo {
    pub id: String,
    pub short_id: String,
    pub message: String,
    pub author: String,
    pub author_email: String,
    pub date: i64,
    pub parents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileChange {
    pub path: String,
    pub status: FileChangeStatus,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FileChangeStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitDiff {
    pub commit_info: CommitInfo,
    pub file_changes: Vec<FileChange>,
    pub diff_content: String,
}

impl CommitInfo {
    pub fn from_git_commit(commit: &Commit) -> Self {
        let id = commit.id().to_string();
        let short_id = id[..7].to_string();
        let message = commit.message().unwrap_or("").to_string();
        let author = commit.author().name().unwrap_or("").to_string();
        let author_email = commit.author().email().unwrap_or("").to_string();
        let date = commit.author().when().seconds();
        let parents = commit.parent_ids().map(|id| id.to_string()).collect();
        
        Self {
            id,
            short_id,
            message,
            author,
            author_email,
            date,
            parents,
        }
    }
}

impl FileChange {
    pub fn from_diff_delta(delta: &DiffDelta) -> Self {
        let path = delta.new_file().path().map_or("unknown".to_string(), |p| p.to_string_lossy().to_string());
        let status = match delta.status() {
            git2::Delta::Added => FileChangeStatus::Added,
            git2::Delta::Deleted => FileChangeStatus::Deleted,
            git2::Delta::Modified => FileChangeStatus::Modified,
            git2::Delta::Renamed => FileChangeStatus::Renamed,
            _ => FileChangeStatus::Modified,
        };
        
        Self {
            path,
            status,
            additions: 0,
            deletions: 0,
        }
    }
}

pub fn get_commit_history(repo_path: &str, limit: usize) -> Result<Vec<CommitInfo>> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    let mut commits = Vec::new();
    
    for oid in revwalk.take(limit) {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        commits.push(CommitInfo::from_git_commit(&commit));
    }
    
    Ok(commits)
}

pub fn get_commit_diff(repo_path: &str, commit_id: &str) -> Result<CommitDiff> {
    let repo = Repository::open(repo_path)?;
    let oid = Oid::from_str(commit_id)?;
    let commit = repo.find_commit(oid)?;
    
    let commit_info = CommitInfo::from_git_commit(&commit);
    
    // Get the diff for this commit
    let mut diff_options = DiffOptions::new();
    let diff = if commit.parent_count() > 0 {
        let parent = commit.parent(0)?;
        repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), Some(&mut diff_options))?
    } else {
        repo.diff_tree_to_tree(None, Some(&commit.tree()?), Some(&mut diff_options))?
    };
    
    let mut file_changes = Vec::new();
    let mut diff_content = String::new();
    
    // Process the diff to extract file changes
    diff.foreach(&mut |delta, _| {
        let file_change = FileChange::from_diff_delta(&delta);
        file_changes.push(file_change);
        true
    }, None, None, None)?;
    
    // Format the diff content
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_content.push_str(&format!("{}", String::from_utf8_lossy(line.content())));
        true
    })?;
    
    Ok(CommitDiff {
        commit_info,
        file_changes,
        diff_content,
    })
}

pub fn get_repository_commits(repo_path: &str, limit: usize) -> Result<Vec<CommitInfo>> {
    get_commit_history(repo_path, limit)
}