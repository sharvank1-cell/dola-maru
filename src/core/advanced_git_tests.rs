#[cfg(test)]
mod tests {
    use crate::core::repository::{RepositoryInfo, AuthType, RepoConfig};
    use crate::core::git_operations::{
        validate_repository_url,
        create_and_push_tag,
        check_merge_conflicts,
        push_to_remote,
        pull_from_remote,
        fetch_from_remote,
        push_to_all_repositories,
        pull_from_all_repositories,
        fetch_from_all_repositories
    };

    #[test]
    fn test_validate_repository_url_comprehensive() {
        // Valid HTTPS URLs
        assert!(validate_repository_url("https://github.com/user/repo.git"));
        assert!(validate_repository_url("https://gitlab.com/group/project.git"));
        assert!(validate_repository_url("https://bitbucket.org/user/repo.git"));
        
        // Valid HTTP URLs
        assert!(validate_repository_url("http://github.com/user/repo.git"));
        
        // Valid SSH URLs
        assert!(validate_repository_url("git@github.com:user/repo.git"));
        assert!(validate_repository_url("git@gitlab.com:group/project.git"));
        
        // Invalid URLs
        assert!(!validate_repository_url("github.com/user/repo.git"));
        assert!(!validate_repository_url("ftp://github.com/user/repo.git"));
        assert!(!validate_repository_url(""));
        assert!(!validate_repository_url("invalid-url"));
        assert!(!validate_repository_url("/local/path/to/repo"));
    }

    #[test]
    fn test_repository_info_default_auth() {
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Default);
        assert_eq!(repo.auth_token, "");
        assert_eq!(repo.ssh_key_path, "");
    }

    #[test]
    fn test_repository_info_ssh_auth() {
        let repo = RepositoryInfo::with_auth(
            "ssh-repo".to_string(),
            "git@github.com:user/repo.git".to_string(),
            AuthType::SSH
        );
        
        assert_eq!(repo.name, "ssh-repo");
        assert_eq!(repo.url, "git@github.com:user/repo.git");
        assert_eq!(repo.auth_type, AuthType::SSH);
        assert_eq!(repo.auth_token, "");
    }

    #[test]
    fn test_repository_info_token_auth() {
        let repo = RepositoryInfo::with_auth(
            "token-repo".to_string(),
            "https://github.com/user/repo.git".to_string(),
            AuthType::Token
        );
        
        assert_eq!(repo.name, "token-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Token);
        assert_eq!(repo.ssh_key_path, "");
    }

    #[test]
    fn test_repo_config_management() {
        let mut config = RepoConfig::new();
        assert_eq!(config.config_name, "default");
        assert_eq!(config.repositories.len(), 0);
        
        // Add repositories
        let repo1 = RepositoryInfo::new(
            "repo1".to_string(),
            "https://github.com/user/repo1.git".to_string()
        );
        let repo2 = RepositoryInfo::with_auth(
            "repo2".to_string(),
            "git@github.com:user/repo2.git".to_string(),
            AuthType::SSH
        );
        
        config.add_repository(repo1);
        config.add_repository(repo2);
        assert_eq!(config.repositories.len(), 2);
        
        // Remove repository
        config.remove_repository(0);
        assert_eq!(config.repositories.len(), 1);
        assert_eq!(config.repositories[0].name, "repo2");
    }

    #[test]
    fn test_tag_name_validation() {
        // This tests the tag creation functionality indirectly
        // by ensuring repository URLs are valid for tagging operations
        let valid_urls = vec![
            "https://github.com/user/repo.git",
            "git@github.com:user/repo.git",
            "http://gitlab.com/group/project.git"
        ];
        
        for url in valid_urls {
            assert!(validate_repository_url(url), "URL should be valid: {}", url);
        }
    }

    #[test]
    fn test_merge_conflict_detection_function_exists() {
        // Test that the function exists and can be called
        // Note: We can't test actual conflict detection without a real repo
        assert!(check_merge_conflicts(&git2::Repository::open(".").unwrap()).is_ok());
    }

    #[test]
    fn test_remote_operations_function_signatures() {
        // Test that all remote operation functions exist with correct signatures
        // Note: We can't test actual functionality without real repositories
        
        // These tests just verify the functions can be compiled and called
        // with the correct parameter types
        let _repo_info = RepositoryInfo::new(
            "test".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        // Verify function signatures (this will fail at runtime but compile-time checks pass)
        let _push_fn = push_to_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _pull_fn = pull_from_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _fetch_fn = fetch_from_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _tag_fn = create_and_push_tag as fn(&git2::Repository, &RepositoryInfo, &str, &str) -> Result<(), anyhow::Error>;
    }

    #[test]
    fn test_bulk_operations_function_signatures() {
        // Test that all bulk operation functions exist with correct signatures
        let _config = RepoConfig::new();
        
        // Verify function signatures
        let _push_all_fn = push_to_all_repositories as fn(&RepoConfig, &str, &str) -> Vec<(String, String)>;
        let _pull_all_fn = pull_from_all_repositories as fn(&RepoConfig, &str) -> Vec<(String, String)>;
        let _fetch_all_fn = fetch_from_all_repositories as fn(&RepoConfig, &str) -> Vec<(String, String)>;
    }

    #[test]
    fn test_auth_type_variants() {
        // Test that all authentication types are properly defined
        assert_eq!(format!("{:?}", AuthType::Default), "Default");
        assert_eq!(format!("{:?}", AuthType::SSH), "SSH");
        assert_eq!(format!("{:?}", AuthType::Token), "Token");
    }
}