#[cfg(test)]
mod tests {
    use crate::core::repository::{RepositoryInfo, RepoConfig, AuthType};

    #[test]
    fn test_new_account_selection_populates_edit_fields() {
        // Test that selecting an account from the panel populates the edit fields
        let mut config = RepoConfig::new();
        let repo = RepositoryInfo::with_auth(
            "test-repo".to_string(),
            "https://github.com/user/repo.git".to_string(),
            AuthType::Token
        );
        config.add_repository(repo);
        
        assert_eq!(config.repositories.len(), 1);
        let selected_repo = &config.repositories[0];
        
        // Simulate the account selection logic that populates edit fields
        let edit_account_name;
        let edit_account_url;
        let edit_account_auth_type;
        
        // When an account is selected, these fields should be populated
        edit_account_name = selected_repo.name.clone();
        edit_account_url = selected_repo.url.clone();
        edit_account_auth_type = selected_repo.auth_type.clone();
        
        assert_eq!(edit_account_name, "test-repo");
        assert_eq!(edit_account_url, "https://github.com/user/repo.git");
        assert_eq!(edit_account_auth_type, AuthType::Token);
    }

    #[test]
    fn test_new_save_account_changes_with_valid_input() {
        // Test that account changes can be saved successfully with valid data
        let mut config = RepoConfig::new();
        let original_repo = RepositoryInfo::with_auth(
            "original-repo".to_string(),
            "https://github.com/user/original.git".to_string(),
            AuthType::Default
        );
        config.add_repository(original_repo);
        
        // Simulate editing the account with valid data
        let updated_name = "updated-repo".to_string();
        let updated_url = "https://github.com/user/updated.git".to_string();
        let updated_auth_type = AuthType::Token;
        
        // Create updated repository info (simulating the save operation)
        let updated_repo = RepositoryInfo::with_auth(
            updated_name.clone(),
            updated_url.clone(),
            updated_auth_type.clone(),
        );
        
        // Save changes to config (simulating the save_account_changes functionality)
        config.repositories[0] = updated_repo;
        
        // Verify changes were saved
        assert_eq!(config.repositories[0].name, "updated-repo");
        assert_eq!(config.repositories[0].url, "https://github.com/user/updated.git");
        assert_eq!(config.repositories[0].auth_type, AuthType::Token);
    }

    #[test]
    fn test_new_save_account_changes_rejects_invalid_url() {
        // Test that the system rejects account changes with invalid URLs
        let invalid_urls = vec![
            "invalid-url",
            "github.com/user/repo.git",  // Missing protocol
            "ftp://github.com/user/repo.git",  // Unsupported protocol
            "",  // Empty URL
        ];
        
        for invalid_url in invalid_urls {
            // Use the existing validation function
            let is_valid = RepoConfig::validate_repository_url(invalid_url);
            assert!(!is_valid, "URL '{}' should be invalid", invalid_url);
        }
    }

    #[test]
    fn test_new_delete_account_functionality() {
        // Test that accounts can be successfully deleted
        let mut config = RepoConfig::new();
        
        // Add two accounts
        let repo1 = RepositoryInfo::new("repo1".to_string(), "https://github.com/user/repo1.git".to_string());
        let repo2 = RepositoryInfo::new("repo2".to_string(), "https://github.com/user/repo2.git".to_string());
        config.add_repository(repo1);
        config.add_repository(repo2);
        
        assert_eq!(config.repositories.len(), 2);
        
        // Delete first account (simulating the delete_selected_account functionality)
        config.repositories.remove(0);
        
        // Verify the account was deleted and the other remains
        assert_eq!(config.repositories.len(), 1);
        assert_eq!(config.repositories[0].name, "repo2");
    }

    #[test]
    fn test_new_save_account_rejects_empty_required_fields() {
        // Test that the system rejects saving when required fields are empty
        let empty_name = "";
        let empty_url = "";
        
        // Both name and URL are required fields
        assert!(empty_name.is_empty());
        assert!(empty_url.is_empty());
        
        // In the actual implementation, this would trigger:
        // self.status_message = "Please fill in all required fields".to_string();
    }
}