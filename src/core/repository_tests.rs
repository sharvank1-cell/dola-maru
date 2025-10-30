#[cfg(test)]
mod tests {
    use crate::core::repository::{RepositoryInfo, RepoConfig, AuthType};

    #[test]
    fn test_repository_creation() {
        let repo = RepositoryInfo::new("test".to_string(), "https://github.com/user/repo.git".to_string());
        assert_eq!(repo.name, "test");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Default);
    }

    #[test]
    fn test_repository_with_auth() {
        let repo = RepositoryInfo::with_auth(
            "test".to_string(), 
            "https://github.com/user/repo.git".to_string(),
            AuthType::SSH
        );
        assert_eq!(repo.name, "test");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::SSH);
    }

    #[test]
    fn test_repo_config_creation() {
        let config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "default");
    }

    #[test]
    fn test_repo_config_with_name() {
        let config = RepoConfig::with_name("production".to_string());
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "production");
    }

    #[test]
    fn test_add_remove_repository() {
        let mut config = RepoConfig::new();
        let repo = RepositoryInfo::new("test".to_string(), "https://github.com/user/repo.git".to_string());
        
        config.add_repository(repo);
        assert_eq!(config.repositories.len(), 1);
        
        config.remove_repository(0);
        assert_eq!(config.repositories.len(), 0);
    }

    #[test]
    fn test_validate_repository_url() {
        // Valid URLs
        assert!(RepoConfig::validate_repository_url("https://github.com/user/repo.git"));
        assert!(RepoConfig::validate_repository_url("http://github.com/user/repo.git"));
        assert!(RepoConfig::validate_repository_url("git@github.com:user/repo.git"));
        
        // Invalid URLs
        assert!(!RepoConfig::validate_repository_url("github.com/user/repo.git"));
        assert!(!RepoConfig::validate_repository_url("ftp://github.com/user/repo.git"));
        assert!(!RepoConfig::validate_repository_url(""));
    }
}