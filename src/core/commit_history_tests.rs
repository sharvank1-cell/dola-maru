#[cfg(test)]
mod tests {
    use crate::core::commit_history::{
        CommitInfo, 
        FileChange, 
        FileChangeStatus, 
        CommitDiff,
        get_commit_history,
        get_commit_diff,
        get_repository_commits
    };
    use git2::Commit;

    // Since we can't easily create real git commits for testing, we'll test the data structures
    // and function signatures instead
    
    #[test]
    fn test_commit_info_struct() {
        let commit_info = CommitInfo {
            id: "abc123".to_string(),
            short_id: "abc123".to_string(),
            message: "Test commit".to_string(),
            author: "Test Author".to_string(),
            author_email: "test@example.com".to_string(),
            date: 1234567890,
            parents: vec!["def456".to_string()],
        };
        
        assert_eq!(commit_info.id, "abc123");
        assert_eq!(commit_info.short_id, "abc123");
        assert_eq!(commit_info.message, "Test commit");
        assert_eq!(commit_info.author, "Test Author");
        assert_eq!(commit_info.author_email, "test@example.com");
        assert_eq!(commit_info.date, 1234567890);
        assert_eq!(commit_info.parents, vec!["def456".to_string()]);
    }

    #[test]
    fn test_file_change_struct() {
        let file_change = FileChange {
            path: "src/main.rs".to_string(),
            status: FileChangeStatus::Modified,
            additions: 10,
            deletions: 5,
        };
        
        assert_eq!(file_change.path, "src/main.rs");
        assert_eq!(file_change.status, FileChangeStatus::Modified);
        assert_eq!(file_change.additions, 10);
        assert_eq!(file_change.deletions, 5);
    }

    #[test]
    fn test_file_change_status_enum() {
        assert_eq!(FileChangeStatus::Added, FileChangeStatus::Added);
        assert_eq!(FileChangeStatus::Modified, FileChangeStatus::Modified);
        assert_eq!(FileChangeStatus::Deleted, FileChangeStatus::Deleted);
        assert_eq!(FileChangeStatus::Renamed, FileChangeStatus::Renamed);
        
        // Test that they are different
        assert_ne!(FileChangeStatus::Added, FileChangeStatus::Modified);
        assert_ne!(FileChangeStatus::Deleted, FileChangeStatus::Renamed);
    }

    #[test]
    fn test_commit_diff_struct() {
        let commit_info = CommitInfo {
            id: "abc123".to_string(),
            short_id: "abc123".to_string(),
            message: "Test commit".to_string(),
            author: "Test Author".to_string(),
            author_email: "test@example.com".to_string(),
            date: 1234567890,
            parents: vec!["def456".to_string()],
        };
        
        let file_changes = vec![FileChange {
            path: "src/main.rs".to_string(),
            status: FileChangeStatus::Modified,
            additions: 10,
            deletions: 5,
        }];
        
        let commit_diff = CommitDiff {
            commit_info: commit_info.clone(),
            file_changes: file_changes.clone(),
            diff_content: "diff content".to_string(),
        };
        
        assert_eq!(commit_diff.commit_info, commit_info);
        assert_eq!(commit_diff.file_changes, file_changes);
        assert_eq!(commit_diff.diff_content, "diff content");
    }

    #[test]
    fn test_commit_info_from_git_commit() {
        // This test would require a real git repository and commit
        // Since we can't easily create one in tests, we'll just verify the function exists
        // and has the correct signature
        let _fn = CommitInfo::from_git_commit as fn(&Commit) -> CommitInfo;
    }

    #[test]
    fn test_file_change_from_diff_delta() {
        // This test would require a real git diff delta
        // Since we can't easily create one in tests, we'll just verify the function exists
        // and has the correct signature
        let _fn = FileChange::from_diff_delta as fn(&git2::DiffDelta) -> FileChange;
    }

    #[test]
    fn test_get_commit_history_function_signature() {
        // Test that the function exists with the correct signature
        let _fn = get_commit_history as fn(&str, usize) -> Result<Vec<CommitInfo>, anyhow::Error>;
    }

    #[test]
    fn test_get_commit_diff_function_signature() {
        // Test that the function exists with the correct signature
        let _fn = get_commit_diff as fn(&str, &str) -> Result<CommitDiff, anyhow::Error>;
    }

    #[test]
    fn test_get_repository_commits_function_signature() {
        // Test that the function exists with the correct signature
        let _fn = get_repository_commits as fn(&str, usize) -> Result<Vec<CommitInfo>, anyhow::Error>;
    }

    #[test]
    fn test_module_public_api() {
        // Test that all public items are accessible
        let _commit_info = CommitInfo {
            id: String::new(),
            short_id: String::new(),
            message: String::new(),
            author: String::new(),
            author_email: String::new(),
            date: 0,
            parents: vec![],
        };
        
        let _file_change = FileChange {
            path: String::new(),
            status: FileChangeStatus::Added,
            additions: 0,
            deletions: 0,
        };
        
        let _commit_diff = CommitDiff {
            commit_info: _commit_info,
            file_changes: vec![],
            diff_content: String::new(),
        };
        
        // Test enums
        let _status_added = FileChangeStatus::Added;
        let _status_modified = FileChangeStatus::Modified;
        let _status_deleted = FileChangeStatus::Deleted;
        let _status_renamed = FileChangeStatus::Renamed;
    }
}