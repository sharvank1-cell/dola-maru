#[cfg(test)]
mod tests {
    use crate::core::error_handler::{GitOperationError, ErrorType, handle_git_error, format_error_result};
    use crate::core::repository::RepositoryInfo;
    use anyhow::anyhow;

    #[test]
    fn test_git_operation_error_creation() {
        let error = GitOperationError::new(
            "pushing to",
            "test-repo",
            "Authentication failed",
            ErrorType::Authentication
        );
        
        assert_eq!(error.operation, "pushing to");
        assert_eq!(error.repository, "test-repo");
        assert_eq!(error.error_message, "Authentication failed");
        assert!(matches!(error.error_type, ErrorType::Authentication));
    }

    #[test]
    fn test_git_operation_error_formatting() {
        let auth_error = GitOperationError::new(
            "pushing to",
            "test-repo",
            "Authentication failed",
            ErrorType::Authentication
        );
        assert_eq!(
            auth_error.format_user_message(),
            "Authentication failed for repository 'test-repo'. Please check your credentials."
        );

        let network_error = GitOperationError::new(
            "fetching from",
            "test-repo",
            "Network timeout",
            ErrorType::Network
        );
        assert_eq!(
            network_error.format_user_message(),
            "Network error while fetching from repository 'test-repo'. Please check your connection."
        );

        let repo_error = GitOperationError::new(
            "cloning",
            "test-repo",
            "Repository not found",
            ErrorType::Repository
        );
        assert_eq!(
            repo_error.format_user_message(),
            "Repository error while cloning 'test-repo'. The repository may be corrupted or inaccessible."
        );

        let permission_error = GitOperationError::new(
            "pushing to",
            "test-repo",
            "Permission denied",
            ErrorType::Permission
        );
        assert_eq!(
            permission_error.format_user_message(),
            "Permission denied while pushing to repository 'test-repo'. Check your access rights."
        );

        let unknown_error = GitOperationError::new(
            "fetching from",
            "test-repo",
            "Unknown error occurred",
            ErrorType::Unknown
        );
        assert_eq!(
            unknown_error.format_user_message(),
            "Error while fetching from repository 'test-repo': Unknown error occurred"
        );
    }

    #[test]
    fn test_handle_git_error_classification() {
        let repo_info = RepositoryInfo::new("test-repo".to_string(), "https://github.com/user/repo.git".to_string());

        // Test authentication error classification
        let auth_error = handle_git_error("pushing to", &repo_info, anyhow!("Authentication failed"));
        assert!(matches!(auth_error.error_type, ErrorType::Authentication));

        let auth_error2 = handle_git_error("pushing to", &repo_info, anyhow!("401 Unauthorized"));
        assert!(matches!(auth_error2.error_type, ErrorType::Authentication));

        let auth_error3 = handle_git_error("pushing to", &repo_info, anyhow!("403 Forbidden"));
        assert!(matches!(auth_error3.error_type, ErrorType::Authentication));

        // Test network error classification
        let network_error = handle_git_error("fetching from", &repo_info, anyhow!("Network timeout"));
        assert!(matches!(network_error.error_type, ErrorType::Network));

        let network_error2 = handle_git_error("fetching from", &repo_info, anyhow!("Connection refused"));
        assert!(matches!(network_error2.error_type, ErrorType::Network));

        // Test permission error classification
        let permission_error = handle_git_error("pushing to", &repo_info, anyhow!("Permission denied"));
        assert!(matches!(permission_error.error_type, ErrorType::Permission));

        let permission_error2 = handle_git_error("pushing to", &repo_info, anyhow!("access denied"));
        assert!(matches!(permission_error2.error_type, ErrorType::Permission));

        // Test repository error classification
        let repo_error = handle_git_error("cloning", &repo_info, anyhow!("Repository not found"));
        assert!(matches!(repo_error.error_type, ErrorType::Repository));

        let repo_error2 = handle_git_error("cloning", &repo_info, anyhow!("Corrupt repository"));
        assert!(matches!(repo_error2.error_type, ErrorType::Repository));

        // Test unknown error classification
        let unknown_error = handle_git_error("fetching from", &repo_info, anyhow!("Some random error"));
        assert!(matches!(unknown_error.error_type, ErrorType::Unknown));
    }

    #[test]
    fn test_format_error_result() {
        let repo_info = RepositoryInfo::new("test-repo".to_string(), "https://github.com/user/repo.git".to_string());

        // Test successful result
        let success_result = Ok(());
        let (name, status) = format_error_result("pushing to", &repo_info, success_result);
        assert_eq!(name, "test-repo");
        assert_eq!(status, "Success");

        // Test error result
        let error_result = Err(anyhow!("Authentication failed"));
        let (name, status) = format_error_result("pushing to", &repo_info, error_result);
        assert_eq!(name, "test-repo");
        assert_eq!(status, "Authentication failed for repository 'test-repo'. Please check your credentials.");
    }
}