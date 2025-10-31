#[cfg(test)]
mod tests {
    use crate::core::repository::{RepositoryInfo, RepoConfig, RepositoryGroup};
    use crate::core::repository_comparison::*;

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