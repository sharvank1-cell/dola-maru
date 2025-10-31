#[cfg(test)]
mod tests {
    use crate::core::repository::{RepoConfig, RepositoryInfo, RepositoryGroup};
    use crate::core::repository_stats::{collect_overall_stats, collect_repository_stats, OverallStats, RepositoryStats, GroupStats};
    
    #[test]
    fn test_repository_stats_creation() {
        let stats = RepositoryStats::new("test_repo".to_string());
        
        assert_eq!(stats.name, "test_repo");
        assert_eq!(stats.total_commits, 0);
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_lines, 0);
        assert_eq!(stats.contributors.len(), 0);
        assert_eq!(stats.branches.len(), 0);
        assert_eq!(stats.tags.len(), 0);
    }
    
    #[test]
    fn test_group_stats_creation() {
        let stats = GroupStats::new("test_group".to_string());
        
        assert_eq!(stats.name, "test_group");
        assert_eq!(stats.total_repositories, 0);
        assert_eq!(stats.total_commits, 0);
        assert_eq!(stats.avg_commits_per_repo, 0.0);
        assert_eq!(stats.total_contributors, 0);
    }
    
    #[test]
    fn test_overall_stats_creation() {
        let stats = OverallStats::new();
        
        assert_eq!(stats.total_repositories, 0);
        assert_eq!(stats.total_groups, 0);
        assert_eq!(stats.total_commits, 0);
        assert_eq!(stats.total_contributors, 0);
        assert_eq!(stats.repository_stats.len(), 0);
        assert_eq!(stats.group_stats.len(), 0);
    }
    
    #[test]
    fn test_collect_repository_stats_with_invalid_path() {
        let repo_info = RepositoryInfo::new(
            "test_repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        // Test with a path that doesn't exist
        let result = collect_repository_stats(&repo_info, "/invalid/path");
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        assert_eq!(stats.name, "test_repo");
        assert_eq!(stats.total_commits, 0);
    }
    
    #[test]
    fn test_collect_overall_stats_empty_config() {
        let config = RepoConfig::new();
        let result = collect_overall_stats(&config);
        
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        assert_eq!(stats.total_repositories, 0);
        assert_eq!(stats.total_groups, 0);
        assert_eq!(stats.total_commits, 0);
        assert_eq!(stats.total_contributors, 0);
        assert_eq!(stats.repository_stats.len(), 0);
        assert_eq!(stats.group_stats.len(), 0);
    }
    
    #[test]
    fn test_collect_overall_stats_with_repositories() {
        let mut config = RepoConfig::new();
        
        // Add repositories
        let repo1 = RepositoryInfo::new(
            "repo1".to_string(),
            "https://github.com/user/repo1.git".to_string()
        );
        let repo2 = RepositoryInfo::new(
            "repo2".to_string(),
            "https://github.com/user/repo2.git".to_string()
        );
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        
        // Add a group
        let mut group = RepositoryGroup::new(
            "test_group".to_string(),
            "Test group".to_string()
        );
        group.add_repository("repo1".to_string());
        config.add_group(group);
        
        let result = collect_overall_stats(&config);
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        assert_eq!(stats.total_repositories, 2);
        assert_eq!(stats.total_groups, 1);
        assert_eq!(stats.repository_stats.len(), 2);
        assert_eq!(stats.group_stats.len(), 1);
    }
    
    #[test]
    fn test_group_stats_calculation() {
        let mut config = RepoConfig::new();
        
        // Add repositories
        let repo1 = RepositoryInfo::new(
            "repo1".to_string(),
            "https://github.com/user/repo1.git".to_string()
        );
        let repo2 = RepositoryInfo::new(
            "repo2".to_string(),
            "https://github.com/user/repo2.git".to_string()
        );
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        
        // Add a group with both repositories
        let mut group = RepositoryGroup::new(
            "test_group".to_string(),
            "Test group".to_string()
        );
        group.add_repository("repo1".to_string());
        group.add_repository("repo2".to_string());
        config.add_group(group);
        
        let result = collect_overall_stats(&config);
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        assert_eq!(stats.group_stats.len(), 1);
        
        let group_stats = &stats.group_stats[0];
        assert_eq!(group_stats.name, "test_group");
        assert_eq!(group_stats.total_repositories, 2);
    }
}