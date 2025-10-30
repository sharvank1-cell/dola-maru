#[cfg(test)]
mod tests {
    use crate::core::repository::{RepoConfig, RepositoryInfo};
    use crate::cli::runner::run_cli;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_cli_module_exists() {
        // This test simply verifies that the CLI module can be compiled
        // and that the run_cli function exists with the correct signature
        let _run_cli_fn = run_cli as fn(
            Arc<Mutex<RepoConfig>>, 
            &str, 
            &str
        ) -> anyhow::Result<()>;
    }

    #[test]
    fn test_repo_config_structure() {
        let config = RepoConfig::new();
        assert_eq!(config.repositories.len(), 0);
        assert_eq!(config.config_name, "default");
    }

    #[test]
    fn test_repository_info_structure() {
        let repo = RepositoryInfo::new(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string()
        );
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://github.com/user/repo.git");
    }
}