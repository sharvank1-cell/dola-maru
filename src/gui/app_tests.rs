#[cfg(test)]
mod tests {
    use crate::core::repository::{RepoConfig, RepositoryInfo};

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
}