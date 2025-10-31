#[cfg(test)]
mod tests {
    use crate::core::batch_operations::*;
    use crate::core::repository::{RepoConfig, RepositoryInfo, RepositoryGroup, AuthType};
    
    #[test]
    fn test_push_to_group_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("test_group".to_string(), "Test group".to_string());
        
        // Add repositories to the group
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        
        group.add_repository("repo1".to_string());
        group.add_repository("repo2".to_string());
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        config.add_group(group);
        
        // Test push to group repositories
        let results = push_to_group_repositories(&config, "test_group", "Test commit", "main");
        
        // We expect results for each repository
        assert_eq!(results.len(), 2);
        
        // Check that each result has the repository name
        let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
        assert!(repo_names.contains(&"repo1"));
        assert!(repo_names.contains(&"repo2"));
    }
    
    #[test]
    fn test_pull_from_group_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("test_group".to_string(), "Test group".to_string());
        
        // Add repositories to the group
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        
        group.add_repository("repo1".to_string());
        group.add_repository("repo2".to_string());
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        config.add_group(group);
        
        // Test pull from group repositories
        let results = pull_from_group_repositories(&config, "test_group", "main");
        
        // We expect results for each repository
        assert_eq!(results.len(), 2);
        
        // Check that each result has the repository name
        let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
        assert!(repo_names.contains(&"repo1"));
        assert!(repo_names.contains(&"repo2"));
    }
    
    #[test]
    fn test_fetch_from_group_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("test_group".to_string(), "Test group".to_string());
        
        // Add repositories to the group
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        
        group.add_repository("repo1".to_string());
        group.add_repository("repo2".to_string());
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        config.add_group(group);
        
        // Test fetch from group repositories
        let results = fetch_from_group_repositories(&config, "test_group", "main");
        
        // We expect results for each repository
        assert_eq!(results.len(), 2);
        
        // Check that each result has the repository name
        let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
        assert!(repo_names.contains(&"repo1"));
        assert!(repo_names.contains(&"repo2"));
    }
    
    #[test]
    fn test_operations_on_empty_group() {
        let config = RepoConfig::new();
        
        // Test push to empty group
        let push_results = push_to_group_repositories(&config, "nonexistent_group", "Test commit", "main");
        assert_eq!(push_results.len(), 1);
        assert_eq!(push_results[0].1, "No repositories found in group");
        
        // Test pull from empty group
        let pull_results = pull_from_group_repositories(&config, "nonexistent_group", "main");
        assert_eq!(pull_results.len(), 1);
        assert_eq!(pull_results[0].1, "No repositories found in group");
        
        // Test fetch from empty group
        let fetch_results = fetch_from_group_repositories(&config, "nonexistent_group", "main");
        assert_eq!(fetch_results.len(), 1);
        assert_eq!(fetch_results[0].1, "No repositories found in group");
    }
}