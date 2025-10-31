#[cfg(test)]
mod tests {
    use crate::core::repository::{RepoConfig, RepositoryInfo, AuthType, RepositoryGroup};

    #[test]
    fn test_repository_info_creation() {
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Default);
        assert_eq!(repo.group, "");
    }

    #[test]
    fn test_repository_info_with_auth() {
        let repo = RepositoryInfo::with_auth(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string(),
            AuthType::SSH
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::SSH);
        assert_eq!(repo.group, "");
    }

    #[test]
    fn test_repository_info_with_group() {
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        ).with_group("frontend".to_string());
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Default);
        assert_eq!(repo.group, "frontend");
    }

    #[test]
    fn test_config_creation() {
        let config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "default");
        assert_eq!(config.groups.len(), 0);
    }

    #[test]
    fn test_config_with_name() {
        let config = RepoConfig::with_name("production".to_string());
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "production");
        assert_eq!(config.groups.len(), 0);
    }

    #[test]
    fn test_add_remove_repository() {
        let mut config = RepoConfig::new();
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        config.add_repository(repo);
        assert_eq!(config.repositories.len(), 1);
        
        config.remove_repository(0);
        assert_eq!(config.repositories.len(), 0);
    }

    #[test]
    fn test_validate_repository_url() {
        // Test valid URLs
        assert!(RepoConfig::validate_repository_url("https://github.com/user/repo.git"));
        assert!(RepoConfig::validate_repository_url("http://github.com/user/repo.git"));
        assert!(RepoConfig::validate_repository_url("git@github.com:user/repo.git"));
        
        // Test invalid URLs
        assert!(!RepoConfig::validate_repository_url("github.com/user/repo.git"));
        assert!(!RepoConfig::validate_repository_url("ftp://github.com/user/repo.git"));
        assert!(!RepoConfig::validate_repository_url(""));
    }

    #[test]
    fn test_repository_grouping() {
        let mut config = RepoConfig::new();
        
        // Add repositories
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        config.add_repository(repo1);
        config.add_repository(repo2);
        
        // Create groups
        let frontend_group = RepositoryGroup::new("frontend".to_string(), "Frontend repositories".to_string());
        let backend_group = RepositoryGroup::new("backend".to_string(), "Backend repositories".to_string());
        config.add_group(frontend_group);
        config.add_group(backend_group);
        
        // Add repositories to groups
        config.get_group_mut("frontend").unwrap().add_repository("repo1".to_string());
        config.get_group_mut("backend").unwrap().add_repository("repo2".to_string());
        
        // Check that repositories are in the correct groups
        let frontend_repos = config.get_repositories_in_group("frontend");
        let backend_repos = config.get_repositories_in_group("backend");
        assert_eq!(frontend_repos.len(), 1);
        assert_eq!(backend_repos.len(), 1);
        assert_eq!(frontend_repos[0].name, "repo1");
        assert_eq!(backend_repos[0].name, "repo2");
        
        // Remove a group
        config.remove_group("frontend");
        assert_eq!(config.groups.len(), 1);
        assert_eq!(config.groups[0].name, "backend");
    }
}