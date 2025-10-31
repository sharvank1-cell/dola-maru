#[cfg(test)]
mod tests {
    use crate::core::batch_operations::*;
    use crate::core::repository::{RepoConfig, RepositoryInfo, RepositoryGroup};
    
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
        assert_eq!(results.len(), 3); // 2 repositories + 1 note about no local repo
        
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
    
    #[test]
    fn test_push_to_group_with_multiple_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("large_group".to_string(), "Large test group".to_string());
        
        // Add multiple repositories to the group
        for i in 1..=5 {
            let repo = RepositoryInfo::new(
                format!("repo{}", i), 
                format!("https://github.com/user/repo{}.git", i)
            );
            group.add_repository(format!("repo{}", i));
            config.add_repository(repo);
        }
        
        config.add_group(group);
        
        // Test push to group with multiple repositories
        let results = push_to_group_repositories(&config, "large_group", "Test commit", "main");
        
        // We expect results for each repository plus one note
        assert_eq!(results.len(), 6); // 5 repositories + 1 note about no local repo
        
        // Check that each result has the repository name
        for i in 1..=5 {
            let repo_name = format!("repo{}", i);
            let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
            assert!(repo_names.contains(&repo_name.as_str()));
        }
    }
    
    #[test]
    fn test_pull_from_group_with_multiple_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("large_group".to_string(), "Large test group".to_string());
        
        // Add multiple repositories to the group
        for i in 1..=5 {
            let repo = RepositoryInfo::new(
                format!("repo{}", i), 
                format!("https://github.com/user/repo{}.git", i)
            );
            group.add_repository(format!("repo{}", i));
            config.add_repository(repo);
        }
        
        config.add_group(group);
        
        // Test pull from group with multiple repositories
        let results = pull_from_group_repositories(&config, "large_group", "main");
        
        // We expect results for each repository
        assert_eq!(results.len(), 5);
        
        // Check that each result has the repository name
        for i in 1..=5 {
            let repo_name = format!("repo{}", i);
            let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
            assert!(repo_names.contains(&repo_name.as_str()));
        }
    }
    
    #[test]
    fn test_fetch_from_group_with_multiple_repositories() {
        let mut config = RepoConfig::new();
        
        // Create a group
        let mut group = RepositoryGroup::new("large_group".to_string(), "Large test group".to_string());
        
        // Add multiple repositories to the group
        for i in 1..=5 {
            let repo = RepositoryInfo::new(
                format!("repo{}", i), 
                format!("https://github.com/user/repo{}.git", i)
            );
            group.add_repository(format!("repo{}", i));
            config.add_repository(repo);
        }
        
        config.add_group(group);
        
        // Test fetch from group with multiple repositories
        let results = fetch_from_group_repositories(&config, "large_group", "main");
        
        // We expect results for each repository
        assert_eq!(results.len(), 5);
        
        // Check that each result has the repository name
        for i in 1..=5 {
            let repo_name = format!("repo{}", i);
            let repo_names: Vec<&str> = results.iter().map(|(name, _)| name.as_str()).collect();
            assert!(repo_names.contains(&repo_name.as_str()));
        }
    }
    
    #[test]
    fn test_group_operations_with_special_characters() {
        let mut config = RepoConfig::new();
        
        // Create a group with special characters
        let mut group = RepositoryGroup::new("test-group_1.2".to_string(), "Test group with special chars".to_string());
        
        // Add repository with special characters
        let repo = RepositoryInfo::new("repo-with-dashes".to_string(), "https://github.com/user/repo-with-dashes.git".to_string());
        
        group.add_repository("repo-with-dashes".to_string());
        config.add_repository(repo);
        config.add_group(group);
        
        // Test all operations with special characters
        let push_results = push_to_group_repositories(&config, "test-group_1.2", "Test commit", "main");
        assert_eq!(push_results.len(), 2); // 1 repository + 1 note
        
        let pull_results = pull_from_group_repositories(&config, "test-group_1.2", "main");
        assert_eq!(pull_results.len(), 1);
        
        let fetch_results = fetch_from_group_repositories(&config, "test-group_1.2", "main");
        assert_eq!(fetch_results.len(), 1);
    }
    
    #[test]
    fn test_nonexistent_group_operations() {
        let mut config = RepoConfig::new();
        
        // Add a repository but no groups
        let repo = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        config.add_repository(repo);
        
        // Test operations on a group that doesn't exist
        let push_results = push_to_group_repositories(&config, "nonexistent", "Test commit", "main");
        assert_eq!(push_results.len(), 1);
        assert_eq!(push_results[0].1, "No repositories found in group");
        
        let pull_results = pull_from_group_repositories(&config, "nonexistent", "main");
        assert_eq!(pull_results.len(), 1);
        assert_eq!(pull_results[0].1, "No repositories found in group");
        
        let fetch_results = fetch_from_group_repositories(&config, "nonexistent", "main");
        assert_eq!(fetch_results.len(), 1);
        assert_eq!(fetch_results[0].1, "No repositories found in group");
    }
    
    #[test]
    fn test_empty_config_operations() {
        let config = RepoConfig::new();
        
        // Test operations on completely empty config
        let push_results = push_to_group_repositories(&config, "any_group", "Test commit", "main");
        assert_eq!(push_results.len(), 1);
        assert_eq!(push_results[0].1, "No repositories found in group");
        
        let pull_results = pull_from_group_repositories(&config, "any_group", "main");
        assert_eq!(pull_results.len(), 1);
        assert_eq!(pull_results[0].1, "No repositories found in group");
        
        let fetch_results = fetch_from_group_repositories(&config, "any_group", "main");
        assert_eq!(fetch_results.len(), 1);
        assert_eq!(fetch_results[0].1, "No repositories found in group");
    }
    
    #[test]
    fn test_group_with_no_repositories() {
        let mut config = RepoConfig::new();
        
        // Create an empty group
        let group = RepositoryGroup::new("empty_group".to_string(), "Empty test group".to_string());
        config.add_group(group);
        
        // Test operations on group with no repositories
        let push_results = push_to_group_repositories(&config, "empty_group", "Test commit", "main");
        assert_eq!(push_results.len(), 1);
        assert_eq!(push_results[0].1, "No repositories found in group");
        
        let pull_results = pull_from_group_repositories(&config, "empty_group", "main");
        assert_eq!(pull_results.len(), 1);
        assert_eq!(pull_results[0].1, "No repositories found in group");
        
        let fetch_results = fetch_from_group_repositories(&config, "empty_group", "main");
        assert_eq!(fetch_results.len(), 1);
        assert_eq!(fetch_results[0].1, "No repositories found in group");
    }
    
    #[test]
    fn test_case_sensitive_group_names() {
        let mut config = RepoConfig::new();
        
        // Create groups with similar but case-different names
        let mut group1 = RepositoryGroup::new("TestGroup".to_string(), "Test group uppercase".to_string());
        let mut group2 = RepositoryGroup::new("testgroup".to_string(), "Test group lowercase".to_string());
        
        // Add repositories to each group
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        
        group1.add_repository("repo1".to_string());
        group2.add_repository("repo2".to_string());
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        config.add_group(group1);
        config.add_group(group2);
        
        // Test that operations are case-sensitive
        let push_results1 = push_to_group_repositories(&config, "TestGroup", "Test commit", "main");
        assert_eq!(push_results1.len(), 3); // 1 repository + 1 note + 1 simulated result
        
        let push_results2 = push_to_group_repositories(&config, "testgroup", "Test commit", "main");
        assert_eq!(push_results2.len(), 3); // 1 repository + 1 note + 1 simulated result
        
        // Verify they return different results
        let repo_names1: Vec<&str> = push_results1.iter().map(|(name, _)| name.as_str()).collect();
        let repo_names2: Vec<&str> = push_results2.iter().map(|(name, _)| name.as_str()).collect();
        
        assert!(repo_names1.contains(&"repo1"));
        assert!(repo_names2.contains(&"repo2"));
        assert!(!repo_names1.contains(&"repo2"));
        assert!(!repo_names2.contains(&"repo1"));
    }
}