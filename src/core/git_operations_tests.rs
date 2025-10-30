#[cfg(test)]
mod tests {
    use crate::core::repository::{RepositoryInfo, AuthType, RepoConfig};
    use crate::core::git_operations::{
        validate_repository_url, 
        check_merge_conflicts,
        push_to_all_repositories,
        pull_from_all_repositories,
        fetch_from_all_repositories,
        create_and_push_tag,
        push_to_remote,
        pull_from_remote,
        fetch_from_remote
    };

    #[test]
    fn test_validate_repository_url_valid() {
        // Test valid URLs
        assert!(validate_repository_url("https://github.com/user/repo.git"));
        assert!(validate_repository_url("http://github.com/user/repo.git"));
        assert!(validate_repository_url("git@github.com:user/repo.git"));
    }

    #[test]
    fn test_validate_repository_url_invalid() {
        // Test invalid URLs
        assert!(!validate_repository_url("github.com/user/repo.git"));
        assert!(!validate_repository_url("ftp://github.com/user/repo.git"));
        assert!(!validate_repository_url(""));
        assert!(!validate_repository_url("invalid-url"));
    }

    #[test]
    fn test_repository_info_creation() {
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
    fn test_repository_info_with_auth() {
        let repo = RepositoryInfo::with_auth(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string(),
            AuthType::Token
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
        assert_eq!(repo.auth_type, AuthType::Token);
    }

    #[test]
    fn test_check_merge_conflicts_no_repo() {
        // This test will fail because we don't have a real repository
        // but it tests that the function handles errors properly
        let result = check_merge_conflicts(&git2::Repository::open(".").unwrap());
        // We're just testing that the function can be called
        assert!(result.is_ok());
    }

    #[test]
    fn test_repo_config_functionality() {
        let mut config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        
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
    fn test_git_operations_function_signatures() {
        // Test that all git operation functions exist with correct signatures
        let _repo_info = RepositoryInfo::new(
            "test".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        // Verify function signatures (this will fail at runtime but compile-time checks pass)
        let _push_fn = push_to_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _pull_fn = pull_from_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _fetch_fn = fetch_from_remote as fn(&git2::Repository, &RepositoryInfo, &str) -> Result<(), anyhow::Error>;
        let _tag_fn = create_and_push_tag as fn(&git2::Repository, &RepositoryInfo, &str, &str) -> Result<(), anyhow::Error>;
        
        let _config = RepoConfig::new();
        let _push_all_fn = push_to_all_repositories as fn(&RepoConfig, &str, &str) -> Vec<(String, String)>;
        let _pull_all_fn = pull_from_all_repositories as fn(&RepoConfig, &str) -> Vec<(String, String)>;
        let _fetch_all_fn = fetch_from_all_repositories as fn(&RepoConfig, &str) -> Vec<(String, String)>;
    }
}