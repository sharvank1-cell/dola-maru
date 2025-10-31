pub mod repository;
pub mod git_operations;
pub mod commit_history;
pub mod error_handler;
pub mod oauth;
pub mod batch_operations;
pub mod repository_stats;
pub mod repository_comparison;

#[cfg(test)]
mod repository_tests;

#[cfg(test)]
mod git_operations_tests;

#[cfg(test)]
mod advanced_git_tests;

#[cfg(test)]
mod account_management_tests;

#[cfg(test)]
mod commit_history_tests;

#[cfg(test)]
mod error_handler_tests;

#[cfg(test)]
mod batch_operations_tests;

#[cfg(test)]
mod repository_stats_tests;

#[cfg(test)]
mod repository_comparison_tests;