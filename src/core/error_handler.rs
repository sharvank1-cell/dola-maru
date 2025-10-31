use anyhow::Result;
use crate::core::repository::{RepositoryInfo};

#[derive(Debug, Clone)]
pub struct GitOperationError {
    pub operation: String,
    pub repository: String,
    pub error_message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Authentication,
    Network,
    Repository,
    Permission,
    Unknown,
}

impl GitOperationError {
    pub fn new(operation: &str, repository: &str, error_message: &str, error_type: ErrorType) -> Self {
        Self {
            operation: operation.to_string(),
            repository: repository.to_string(),
            error_message: error_message.to_string(),
            error_type,
        }
    }
    
    pub fn format_user_message(&self) -> String {
        match self.error_type {
            ErrorType::Authentication => {
                format!("Authentication failed for repository '{}'. Please check your credentials.", self.repository)
            },
            ErrorType::Network => {
                format!("Network error while {} repository '{}'. Please check your connection.", self.operation, self.repository)
            },
            ErrorType::Repository => {
                format!("Repository error while {} '{}'. The repository may be corrupted or inaccessible.", self.operation, self.repository)
            },
            ErrorType::Permission => {
                format!("Permission denied while {} repository '{}'. Check your access rights.", self.operation, self.repository)
            },
            ErrorType::Unknown => {
                format!("Error while {} repository '{}': {}", self.operation, self.repository, self.error_message)
            },
        }
    }
}

pub fn handle_git_error(operation: &str, repo_info: &RepositoryInfo, error: anyhow::Error) -> GitOperationError {
    let error_message = error.to_string();
    
    // Classify error based on error message content
    let error_type = if error_message.contains("authentication") || 
                     error_message.contains("Authentication") || 
                     error_message.contains("401") || 
                     error_message.contains("403") ||
                     error_message.contains("Unauthorized") {
        ErrorType::Authentication
    } else if error_message.contains("network") || 
              error_message.contains("Network") || 
              error_message.contains("connection") ||
              error_message.contains("Connection") ||
              error_message.contains("timeout") ||
              error_message.contains("Timeout") {
        ErrorType::Network
    } else if error_message.contains("permission") || 
              error_message.contains("Permission") || 
              error_message.contains("access denied") {
        ErrorType::Permission
    } else if error_message.contains("repository") || 
              error_message.contains("Repository") ||
              error_message.contains("not found") ||
              error_message.contains("corrupt") {
        ErrorType::Repository
    } else {
        ErrorType::Unknown
    };
    
    GitOperationError::new(operation, &repo_info.name, &error_message, error_type)
}

pub fn format_error_result(operation: &str, repo_info: &RepositoryInfo, result: Result<()>) -> (String, String) {
    match result {
        Ok(_) => (repo_info.name.clone(), "Success".to_string()),
        Err(e) => {
            let error = handle_git_error(operation, repo_info, e);
            (repo_info.name.clone(), error.format_user_message())
        }
    }
}