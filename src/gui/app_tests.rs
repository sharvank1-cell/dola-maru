#[cfg(test)]
mod tests {
    use crate::core::repository::{RepoConfig, RepositoryInfo, AuthType};

    #[test]
    fn test_repository_info_creation() {
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
    }

    #[test]
    fn test_repo_config_creation() {
        let config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "default");
    }

    #[test]
    fn test_auth_type_variants() {
        // Test that all authentication types are properly defined
        assert_eq!(format!("{:?}", AuthType::Default), "Default");
        assert_eq!(format!("{:?}", AuthType::SSH), "SSH");
        assert_eq!(format!("{:?}", AuthType::Token), "Token");
    }

    #[test]
    fn test_repository_with_all_auth_types() {
        let default_repo = RepositoryInfo::new(
            "default-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        assert_eq!(default_repo.auth_type, AuthType::Default);
        
        let ssh_repo = RepositoryInfo::with_auth(
            "ssh-repo".to_string(),
            "git@github.com:user/repo.git".to_string(),
            AuthType::SSH
        );
        assert_eq!(ssh_repo.auth_type, AuthType::SSH);
        
        let token_repo = RepositoryInfo::with_auth(
            "token-repo".to_string(),
            "https://github.com/user/repo.git".to_string(),
            AuthType::Token
        );
        assert_eq!(token_repo.auth_type, AuthType::Token);
    }

    #[test]
    fn test_repo_config_with_multiple_repositories() {
        let mut config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        
        // Add multiple repositories
        config.add_repository(RepositoryInfo::new(
            "repo1".to_string(),
            "https://github.com/user/repo1.git".to_string()
        ));
        
        config.add_repository(RepositoryInfo::with_auth(
            "repo2".to_string(),
            "git@github.com:user/repo2.git".to_string(),
            AuthType::SSH
        ));
        
        assert_eq!(config.repositories.len(), 2);
        assert_eq!(config.repositories[0].name, "repo1");
        assert_eq!(config.repositories[1].name, "repo2");
    }
}