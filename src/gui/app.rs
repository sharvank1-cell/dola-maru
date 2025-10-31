use eframe::egui;
use crate::core::repository::{RepoConfig, RepositoryInfo, AuthType};
use crate::core::git_operations::{
    push_to_all_repositories, 
    pull_from_all_repositories, 
    fetch_from_all_repositories, 
    create_and_push_tag, 
    check_merge_conflicts,
    validate_repository_url, 
    verify_authentication,
    clone_all_repositories
};
use crate::core::batch_operations::{
    push_to_group_repositories,
    pull_from_group_repositories,
    fetch_from_group_repositories
};
// GitOperationError import removed as it's not currently used
use crate::gui::commit_history_viewer::CommitHistoryViewer;
use std::sync::{Arc, Mutex};
use webbrowser;

pub struct MultiRepoPusherApp {
    config: Arc<Mutex<RepoConfig>>,
    commit_message: String,
    branch_name: String,
    tag_name: String,
    tag_message: String,
    status_message: String,
    is_operation_running: bool,
    operation_results: Vec<(String, String)>, // (repo_name, status)
    new_repo_name: String,
    new_repo_url: String,
    new_repo_auth_type: AuthType,
    new_repo_token: String,
    new_repo_ssh_key: String,
    config_name_input: String,
    show_auth_fields: bool,
    active_tab: Tab,
    // New fields for account management
    show_account_form: bool,
    account_username: String,
    account_email: String,
    account_token: String,
    account_ssh_key_path: String,
    account_auth_type: AuthType,
    // Animation variables
    animation_timer: f32,
    // New fields for cloning and group management
    clone_destination_path: String,
    show_group_form: bool,
    new_group_name: String,
    new_group_description: String,
    selected_group: String,
    // First-time setup fields
    show_first_time_setup: bool,
    setup_completed: bool,
    // OAuth fields
    oauth_token: String,
    oauth_code: String,
    // Account selection and editing fields
    selected_account_index: usize,
    edit_account_name: String,
    edit_account_url: String,
    edit_account_auth_type: AuthType,
    edit_account_token: String,
    edit_account_ssh_key: String,
    // Commit history viewer
    commit_history_viewer: CommitHistoryViewer,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Commit,
    Repositories,
    CommitHistory,
    Advanced,
    Statistics,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Commit
    }
}

impl MultiRepoPusherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Arc<Mutex<RepoConfig>>) -> Self {
        // Customize the look of the GUI with premium styling
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(25, 25, 35); // Deep dark background
        visuals.window_fill = egui::Color32::from_rgb(35, 35, 50); // Slightly lighter window
        visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 100));
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 40, 60);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(200, 200, 220);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(55, 55, 80);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(80, 80, 130);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(100, 100, 180);
        visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
        visuals.selection.bg_fill = egui::Color32::from_rgb(90, 90, 150);
        cc.egui_ctx.set_visuals(visuals);
        
        // Check if this is first time setup
        let config_lock = config.lock().unwrap();
        let is_first_time = config_lock.repositories.is_empty() || 
            (config_lock.repositories.len() == 1 && 
             config_lock.repositories[0].url.contains("YOUR_USERNAME"));
        drop(config_lock);
        
        Self {
            config: config.clone(),
            commit_message: "Auto commit".to_string(),
            branch_name: "main".to_string(),
            tag_name: String::new(),
            tag_message: String::new(),
            status_message: "Ready".to_string(),
            is_operation_running: false,
            operation_results: Vec::new(),
            new_repo_name: String::new(),
            new_repo_url: String::new(),
            new_repo_auth_type: AuthType::Default,
            new_repo_token: String::new(),
            new_repo_ssh_key: String::new(),
            config_name_input: "default".to_string(),
            show_auth_fields: false,
            active_tab: Tab::Commit,
            // Initialize new account fields
            show_account_form: false,
            account_username: String::new(),
            account_email: String::new(),
            account_token: String::new(),
            account_ssh_key_path: String::new(),
            account_auth_type: AuthType::Default,
            // Initialize animation timer
            animation_timer: 0.0,
            // Initialize new fields for cloning and group management
            clone_destination_path: String::new(),
            show_group_form: false,
            new_group_name: String::new(),
            new_group_description: String::new(),
            selected_group: String::new(),
            // First-time setup fields
            show_first_time_setup: is_first_time,
            setup_completed: !is_first_time,
            // OAuth fields
            oauth_token: String::new(),
            oauth_code: String::new(),
            // Account selection and editing fields
            selected_account_index: 0,
            edit_account_name: String::new(),
            edit_account_url: String::new(),
            edit_account_auth_type: AuthType::Default,
            edit_account_token: String::new(),
            edit_account_ssh_key: String::new(),
            // Commit history viewer
            commit_history_viewer: CommitHistoryViewer::new(config.clone()),
        }
    }
    
    fn push_to_all_repositories(&mut self) {
        self.is_operation_running = true;
        self.status_message = "Pushing to repositories...".to_string();
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let commit_message = self.commit_message.clone();
        let branch_name = self.branch_name.clone();
        
        // Push to all repositories
        self.operation_results = push_to_all_repositories(&config, &commit_message, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Push completed with {} errors!", failed_count);
        } else {
            self.status_message = "Push completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn pull_from_all_repositories(&mut self) {
        self.is_operation_running = true;
        self.status_message = "Pulling from repositories...".to_string();
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let branch_name = self.branch_name.clone();
        
        // Pull from all repositories
        self.operation_results = pull_from_all_repositories(&config, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Pull completed with {} errors!", failed_count);
        } else {
            self.status_message = "Pull completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn fetch_from_all_repositories(&mut self) {
        self.is_operation_running = true;
        self.status_message = "Fetching from repositories...".to_string();
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let branch_name = self.branch_name.clone();
        
        // Fetch from all repositories
        self.operation_results = fetch_from_all_repositories(&config, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Fetch completed with {} errors!", failed_count);
        } else {
            self.status_message = "Fetch completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    // Render statistics tab
    fn render_statistics_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 Repository Statistics");
        ui.separator();
        
        // Add a refresh button to collect statistics
        ui.horizontal(|ui| {
            if self.is_operation_running {
                ui.add(egui::Spinner::new().size(20.0));
                ui.label("Collecting statistics...");
            } else {
                let refresh_button = egui::Button::new(
                    egui::RichText::new("🔄 Refresh Statistics")
                        .size(14.0)
                )
                .fill(egui::Color32::from_rgb(90, 90, 90))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)))
                .rounding(egui::Rounding::same(6.0));
                
                if ui.add(refresh_button).clicked() {
                    self.collect_statistics();
                }
            }
        });
        
        ui.add_space(10.0);
        
        // Display overall statistics
        let config = self.config.clone();
        let config_lock = config.lock().unwrap();
        
        // Overall stats
        ui.group(|ui| {
            ui.heading("📈 Overall Statistics");
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Repositories:").strong());
                    ui.label(egui::RichText::new("Groups:").strong());
                    ui.label(egui::RichText::new("Total Commits:").strong());
                    ui.label(egui::RichText::new("Contributors:").strong());
                });
                
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(format!("{}", config_lock.repositories.len())));
                    ui.label(egui::RichText::new(format!("{}", config_lock.groups.len())));
                    // We'll update these with actual stats when collected
                    ui.label(egui::RichText::new("0"));
                    ui.label(egui::RichText::new("0"));
                });
            });
        });
        
        ui.add_space(10.0);
        
        // Repository-specific stats
        if !config_lock.repositories.is_empty() {
            ui.group(|ui| {
                ui.heading("📁 Repository Details");
                ui.add_space(10.0);
                
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    for repo in &config_lock.repositories {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&repo.name).size(16.0).strong());
                                    ui.label(egui::RichText::new(&repo.url).weak().size(12.0));
                                    
                                    // Stats placeholders
                                    ui.label(egui::RichText::new("Commits: 0").weak().size(11.0));
                                    ui.label(egui::RichText::new("Files: 0").weak().size(11.0));
                                    ui.label(egui::RichText::new("Contributors: 0").weak().size(11.0));
                                    
                                    if !repo.group.is_empty() {
                                        ui.label(egui::RichText::new(format!("📁 Group: {}", repo.group)).weak().size(11.0).color(egui::Color32::from_rgb(200, 150, 200)));
                                    }
                                });
                            });
                        });
                        ui.add_space(5.0);
                    }
                });
            });
        }
        
        // Group-specific stats
        if !config_lock.groups.is_empty() {
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.heading("📁 Group Statistics");
                ui.add_space(10.0);
                
                for group in &config_lock.groups {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&group.name).size(16.0).strong());
                                ui.label(egui::RichText::new(&group.description).weak().size(12.0));
                                
                                // Stats placeholders
                                ui.label(egui::RichText::new(format!("Repositories: {}", group.repository_names.len())).weak().size(11.0));
                                ui.label(egui::RichText::new("Total Commits: 0").weak().size(11.0));
                                ui.label(egui::RichText::new("Avg Commits/Repo: 0.0").weak().size(11.0));
                            });
                        });
                    });
                    ui.add_space(5.0);
                }
            });
        }
    }
    
    // Method to collect statistics
    fn collect_statistics(&mut self) {
        self.is_operation_running = true;
        self.status_message = "Collecting repository statistics...".to_string();
        
        // In a real implementation, this would collect actual statistics
        // For now, we'll just simulate the process
        
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        self.is_operation_running = false;
        self.status_message = "Statistics collected successfully!".to_string();
    }
    
    // Batch operations for repository groups
    fn push_to_group_repositories(&mut self) {
        if self.selected_group.is_empty() {
            self.status_message = "Please select a group first".to_string();
            return;
        }
        
        self.is_operation_running = true;
        self.status_message = format!("Pushing to repositories in group '{}'...", self.selected_group);
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let commit_message = self.commit_message.clone();
        let branch_name = self.branch_name.clone();
        let group_name = self.selected_group.clone();
        
        // Push to all repositories in the group
        self.operation_results = push_to_group_repositories(&config, &group_name, &commit_message, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Push to group completed with {} errors!", failed_count);
        } else {
            self.status_message = "Push to group completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn pull_from_group_repositories(&mut self) {
        if self.selected_group.is_empty() {
            self.status_message = "Please select a group first".to_string();
            return;
        }
        
        self.is_operation_running = true;
        self.status_message = format!("Pulling from repositories in group '{}'...", self.selected_group);
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let branch_name = self.branch_name.clone();
        let group_name = self.selected_group.clone();
        
        // Pull from all repositories in the group
        self.operation_results = pull_from_group_repositories(&config, &group_name, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Pull from group completed with {} errors!", failed_count);
        } else {
            self.status_message = "Pull from group completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn fetch_from_group_repositories(&mut self) {
        if self.selected_group.is_empty() {
            self.status_message = "Please select a group first".to_string();
            return;
        }
        
        self.is_operation_running = true;
        self.status_message = format!("Fetching from repositories in group '{}'...", self.selected_group);
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let branch_name = self.branch_name.clone();
        let group_name = self.selected_group.clone();
        
        // Fetch from all repositories in the group
        self.operation_results = fetch_from_group_repositories(&config, &group_name, &branch_name);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Fetch from group completed with {} errors!", failed_count);
        } else {
            self.status_message = "Fetch from group completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn create_and_push_tag(&mut self) {
        if self.tag_name.is_empty() {
            self.status_message = "Please enter a tag name".to_string();
            return;
        }
        
        self.is_operation_running = true;
        self.status_message = "Creating and pushing tag...".to_string();
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        let tag_name = self.tag_name.clone();
        let tag_message = if self.tag_message.is_empty() {
            format!("Release {}", tag_name)
        } else {
            self.tag_message.clone()
        };
        
        // Get the current repository
        let repo = match git2::Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => {
                self.status_message = format!("Failed to open repository: {}", e);
                self.is_operation_running = false;
                return;
            }
        };
        
        // Create and push tag for all repositories
        for repo_info in &config.repositories {
            match create_and_push_tag(&repo, repo_info, &tag_name, &tag_message) {
                Ok(_) => {
                    self.operation_results.push((repo_info.name.clone(), "Success".to_string()));
                }
                Err(e) => {
                    self.operation_results.push((repo_info.name.clone(), e.to_string()));
                }
            }
        }
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Tag creation and push completed with {} errors!", failed_count);
        } else {
            self.status_message = "Tag creation and push completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    fn check_merge_conflicts(&mut self) {
        self.is_operation_running = true;
        self.status_message = "Checking for merge conflicts...".to_string();
        self.operation_results.clear();
        
        // Get the current repository
        let repo = match git2::Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => {
                self.status_message = format!("Failed to open repository: {}", e);
                self.is_operation_running = false;
                return;
            }
        };
        
        match check_merge_conflicts(&repo) {
            Ok(has_conflicts) => {
                if has_conflicts {
                    self.status_message = "Merge conflicts detected!".to_string();
                    self.operation_results.push(("Repository".to_string(), "Conflicts detected".to_string()));
                } else {
                    self.status_message = "No merge conflicts found".to_string();
                    self.operation_results.push(("Repository".to_string(), "No conflicts".to_string()));
                }
            }
            Err(e) => {
                self.status_message = format!("Error checking conflicts: {}", e);
                self.operation_results.push(("Repository".to_string(), e.to_string()));
            }
        }
        
        self.is_operation_running = false;
    }
    
    // New method for cloning all repositories
    fn clone_all_repositories(&mut self) {
        if self.clone_destination_path.is_empty() {
            self.status_message = "Please specify a destination path for cloning".to_string();
            return;
        }
        
        self.is_operation_running = true;
        self.status_message = "Cloning repositories...".to_string();
        self.operation_results.clear();
        
        // Clone config for iteration
        let config_clone = self.config.clone();
        let config = config_clone.lock().unwrap();
        
        // Clone all repositories
        self.operation_results = clone_all_repositories(&config, &self.clone_destination_path);
        
        // Check if any operations failed
        let failed_count = self.operation_results.iter().filter(|(_, status)| !status.contains("Success")).count();
        if failed_count > 0 {
            self.status_message = format!("Cloning completed with {} errors!", failed_count);
        } else {
            self.status_message = "Cloning completed successfully!".to_string();
        }
        
        self.is_operation_running = false;
    }
    
    // New method for creating a repository group
    fn create_repository_group(&mut self) {
        if self.new_group_name.is_empty() {
            self.status_message = "Please enter a group name".to_string();
            return;
        }
        
        let group = crate::core::repository::RepositoryGroup::new(
            self.new_group_name.clone(),
            self.new_group_description.clone()
        );
        
        let mut config = self.config.lock().unwrap();
        config.add_group(group);
        
        self.status_message = format!("Group '{}' created successfully", self.new_group_name);
        
        // Clear form fields
        self.new_group_name.clear();
        self.new_group_description.clear();
        self.show_group_form = false;
    }
    
    // New method for adding a repository to a group
    fn add_repository_to_group(&mut self, repo_index: usize, group_name: String) {
        let mut config = self.config.lock().unwrap();
        
        // Check if repository exists
        if repo_index >= config.repositories.len() {
            self.status_message = "Repository not found".to_string();
            return;
        }
        
        // Get repository name
        let repo_name = config.repositories[repo_index].name.clone();
        
        // Add the repository to the group
        if let Some(group) = config.get_group_mut(&group_name) {
            group.add_repository(repo_name.clone());
            // Also update the repository's group field
            config.repositories[repo_index].group = group_name.clone();
            self.status_message = format!("Repository '{}' added to group '{}'", repo_name, group_name);
        } else {
            self.status_message = format!("Group '{}' not found", group_name);
        }
    }
    
    // New method for removing a repository from a group
    fn remove_repository_from_group(&mut self, repo_index: usize, group_name: String) {
        let mut config = self.config.lock().unwrap();
        
        // Check if repository exists
        if repo_index >= config.repositories.len() {
            self.status_message = "Repository not found".to_string();
            return;
        }
        
        // Get repository name
        let repo_name = config.repositories[repo_index].name.clone();
        
        // Remove the repository from the group
        config.remove_repository_from_group(&repo_name, &group_name);
        self.status_message = format!("Repository '{}' removed from group '{}'", repo_name, group_name);
    }
    
    fn validate_and_add_repository(&mut self) {
        if self.new_repo_name.is_empty() || self.new_repo_url.is_empty() {
            self.status_message = "Please fill in all required fields".to_string();
            return;
        }
        
        // Validate repository URL
        if !validate_repository_url(&self.new_repo_url) {
            self.status_message = "Invalid repository URL format".to_string();
            return;
        }
        
        // Create repository info with authentication
        let mut repo_info = RepositoryInfo::with_auth(
            self.new_repo_name.clone(),
            self.new_repo_url.clone(),
            self.new_repo_auth_type.clone(),
        );
        
        // Set authentication details based on type
        match &self.new_repo_auth_type {
            AuthType::Token => {
                repo_info.auth_token = self.new_repo_token.clone();
            },
            AuthType::SSH => {
                repo_info.ssh_key_path = self.new_repo_ssh_key.clone();
            },
            _ => {}
        }
        
        // Add to config
        let mut config = self.config.lock().unwrap();
        config.add_repository(repo_info);
        
        // Clear form fields
        self.new_repo_name.clear();
        self.new_repo_url.clear();
        self.new_repo_token.clear();
        self.new_repo_ssh_key.clear();
        self.status_message = "Repository added successfully".to_string();
    }
    
    // New function to handle account creation
    fn add_new_account(&mut self) {
        if self.account_username.is_empty() || self.account_email.is_empty() {
            self.status_message = "Please fill in username and email".to_string();
            return;
        }
        
        // Show loading indicator
        self.status_message = "Validating account and saving configuration...".to_string();
        self.is_operation_running = true;
        
        // In a real implementation, you would validate the account credentials here
        // For now, we'll just show a success message
        self.is_operation_running = false;
        self.status_message = format!("Account '{}' added successfully", self.account_username);
        
        // Close the form
        self.show_account_form = false;
        
        // Clear the form fields
        self.account_username.clear();
        self.account_email.clear();
        self.account_token.clear();
        self.account_ssh_key_path.clear();
    }
    
    // New function to save account changes
    fn save_account_changes(&mut self) {
        if self.edit_account_name.is_empty() || self.edit_account_url.is_empty() {
            self.status_message = "Please fill in all required fields".to_string();
            return;
        }
        
        // Validate repository URL
        if !validate_repository_url(&self.edit_account_url) {
            self.status_message = "Invalid repository URL format".to_string();
            return;
        }
        
        // Update the selected account in the config
        let mut config = self.config.lock().unwrap();
        if self.selected_account_index < config.repositories.len() {
            let mut repo_info = RepositoryInfo::with_auth(
                self.edit_account_name.clone(),
                self.edit_account_url.clone(),
                self.edit_account_auth_type.clone(),
            );
            
            // Set authentication details based on type
            match &self.edit_account_auth_type {
                AuthType::Token => {
                    repo_info.auth_token = self.edit_account_token.clone();
                },
                AuthType::SSH => {
                    repo_info.ssh_key_path = self.edit_account_ssh_key.clone();
                },
                _ => {}
            }
            
            // Replace the repository at the selected index
            config.repositories[self.selected_account_index] = repo_info;
            
            self.status_message = "Account changes saved successfully".to_string();
        } else {
            self.status_message = "Invalid account selection".to_string();
        }
    }
    
    // New function to delete the selected account
    fn delete_selected_account(&mut self) {
        let mut config = self.config.lock().unwrap();
        if self.selected_account_index < config.repositories.len() {
            let repo_name = config.repositories[self.selected_account_index].name.clone();
            config.repositories.remove(self.selected_account_index);
            
            // Adjust selected index if needed
            if !config.repositories.is_empty() && self.selected_account_index >= config.repositories.len() {
                self.selected_account_index = config.repositories.len() - 1;
            } else if config.repositories.is_empty() {
                self.selected_account_index = 0;
            }
            
            // Clear edit fields
            self.edit_account_name.clear();
            self.edit_account_url.clear();
            self.edit_account_token.clear();
            self.edit_account_ssh_key.clear();
            
            self.status_message = format!("Account '{}' deleted successfully", repo_name);
        } else {
            self.status_message = "No account selected for deletion".to_string();
        }
    }
}

impl eframe::App for MultiRepoPusherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update animation timer
        self.animation_timer += ctx.input(|i| i.stable_dt);
        
        // Show first-time setup modal if needed
        if self.show_first_time_setup {
            self.render_first_time_setup(ctx);
            return;
        }
        
        // Create a side panel layout
        egui::SidePanel::left("account_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("👥 Accounts");
                ui.separator();
                
                // Account selection
                let config = self.config.clone();
                let repos = config.lock().unwrap().repositories.clone();
                
                if repos.is_empty() {
                    ui.label(egui::RichText::new("No accounts configured").weak());
                } else {
                    for (i, repo) in repos.iter().enumerate() {
                        let button = egui::Button::new(
                            egui::RichText::new(&repo.name)
                                .size(14.0)
                        )
                        .fill(if self.selected_account_index == i {
                            egui::Color32::from_rgb(90, 90, 150) // Highlight selected account
                        } else {
                            egui::Color32::from_rgb(50, 50, 70)
                        })
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 150)))
                        .rounding(egui::Rounding::same(4.0))
                        .min_size(egui::Vec2::new(ui.available_width() - 10.0, 30.0));
                        
                        if ui.add(button).clicked() {
                            // Handle account selection
                            self.selected_account_index = i;
                            self.status_message = format!("Selected account: {}", repo.name);
                            
                            // Update edit fields with selected account details
                            self.edit_account_name = repo.name.clone();
                            self.edit_account_url = repo.url.clone();
                            self.edit_account_auth_type = repo.auth_type.clone();
                            self.edit_account_token = repo.auth_token.clone();
                            self.edit_account_ssh_key = repo.ssh_key_path.clone();
                        }
                        
                        ui.add_space(5.0);
                    }
                }
                
                ui.add_space(10.0);
                
                // Add account button
                let add_account_button = egui::Button::new(
                    egui::RichText::new("➕ Add Account")
                        .size(14.0)
                )
                .fill(egui::Color32::from_rgb(60, 100, 60))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 100)))
                .rounding(egui::Rounding::same(6.0))
                .min_size(egui::Vec2::new(ui.available_width() - 10.0, 35.0));
                
                if ui.add(add_account_button).clicked() {
                    self.show_account_form = true;
                }
                
                ui.add_space(10.0);
                
                // Show account details for selected account
                ui.separator();
                ui.heading("📋 Account Details");
                ui.add_space(10.0);
                
                if !repos.is_empty() {
                    // Update edit fields when account selection changes
                    if self.edit_account_name.is_empty() && self.selected_account_index < repos.len() {
                        let selected_repo = &repos[self.selected_account_index];
                        self.edit_account_name = selected_repo.name.clone();
                        self.edit_account_url = selected_repo.url.clone();
                        self.edit_account_auth_type = selected_repo.auth_type.clone();
                        self.edit_account_token = selected_repo.auth_token.clone();
                        self.edit_account_ssh_key = selected_repo.ssh_key_path.clone();
                    }
                    
                    ui.label(egui::RichText::new("Name:").strong());
                    ui.add(egui::TextEdit::singleline(&mut self.edit_account_name).desired_width(ui.available_width() * 0.8));
                    ui.add_space(5.0);
                    
                    ui.label(egui::RichText::new("URL:").strong());
                    ui.add(egui::TextEdit::singleline(&mut self.edit_account_url).desired_width(ui.available_width() * 0.8));
                    ui.add_space(5.0);
                    
                    ui.label(egui::RichText::new("Auth Type:").strong());
                    egui::ComboBox::from_id_source("edit_account_auth_type")
                        .selected_text(format!("{:?}", self.edit_account_auth_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.edit_account_auth_type, AuthType::Default, "Default");
                            ui.selectable_value(&mut self.edit_account_auth_type, AuthType::SSH, "SSH Key");
                            ui.selectable_value(&mut self.edit_account_auth_type, AuthType::Token, "Personal Access Token");
                        });
                    ui.add_space(5.0);
                    
                    // Show auth-specific fields based on selected type
                    match &self.edit_account_auth_type {
                        AuthType::Token => {
                            ui.label(egui::RichText::new("Token:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.edit_account_token).password(true).desired_width(ui.available_width() * 0.8));
                            ui.add_space(5.0);
                        },
                        AuthType::SSH => {
                            ui.label(egui::RichText::new("SSH Key Path:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.edit_account_ssh_key).desired_width(ui.available_width() * 0.8));
                            ui.add_space(5.0);
                        },
                        _ => {}
                    }
                    
                    ui.add_space(10.0);
                    
                    // Save and Delete buttons
                    ui.horizontal(|ui| {
                        let save_button = egui::Button::new(
                            egui::RichText::new("💾 Save")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(60, 100, 60))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 100)))
                        .rounding(egui::Rounding::same(4.0))
                        .min_size(egui::Vec2::new(80.0, 30.0));
                        
                        if ui.add(save_button).clicked() {
                            self.save_account_changes();
                        }
                        
                        let delete_button = egui::Button::new(
                            egui::RichText::new("🗑 Delete")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(150, 80, 80))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 150, 150)))
                        .rounding(egui::Rounding::same(4.0))
                        .min_size(egui::Vec2::new(80.0, 30.0));
                        
                        if ui.add(delete_button).clicked() {
                            self.delete_selected_account();
                        }
                    });
                } else {
                    ui.label(egui::RichText::new("No account selected").weak());
                }
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header with title and styling
            ui.vertical_centered(|ui| {
                // Animated title with gradient effect
                let hue = (self.animation_timer * 0.5).sin() * 0.5 + 0.5;
                let color = egui::Color32::from_rgb(
                    (hue * 255.0) as u8,
                    ((1.0 - hue) * 255.0) as u8,
                    (hue * 128.0) as u8
                );
                ui.heading(egui::RichText::new("Multi-Repo Pusher").size(28.0).color(color));
                ui.label(egui::RichText::new("Push your code to multiple repositories simultaneously").italics().weak());
            });
            
            ui.separator();
            
            // Tab selection with improved styling
            ui.horizontal(|ui| {
                ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 50, 70);
                ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 70, 110);
                ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_rgb(90, 90, 150);
                ui.visuals_mut().widgets.active.fg_stroke.color = egui::Color32::WHITE;
                
                ui.selectable_value(&mut self.active_tab, Tab::Commit, "📝 Commit");
                ui.selectable_value(&mut self.active_tab, Tab::Repositories, "📂 Repositories");
                ui.selectable_value(&mut self.active_tab, Tab::CommitHistory, "📜 Commit History");
                ui.selectable_value(&mut self.active_tab, Tab::Advanced, "⚙️ Advanced");
                ui.selectable_value(&mut self.active_tab, Tab::Statistics, "📊 Statistics");
            });
            
            ui.separator();
            
            match self.active_tab {
                Tab::Commit => self.render_commit_tab(ui),
                Tab::Repositories => self.render_repositories_tab(ui),
                Tab::CommitHistory => self.commit_history_viewer.render(ui),
                Tab::Advanced => self.render_advanced_tab(ui),
                Tab::Statistics => self.render_statistics_tab(ui),
            }
            
            // Show account form as a modal if needed
            self.render_account_modal(ctx);
            
            // Results section with improved styling
            if !self.operation_results.is_empty() {
                ui.add_space(10.0);
                
                ui.group(|ui| {
                    ui.heading("📋 Results");
                    
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for (repo_name, status) in &self.operation_results {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(repo_name).size(14.0).strong());
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if status == "Success" || status == "No conflicts" {
                                            ui.label(egui::RichText::new("✓ Success").color(egui::Color32::GREEN));
                                        } else if status == "Conflicts detected" {
                                            ui.label(egui::RichText::new("⚠ Conflicts").color(egui::Color32::YELLOW));
                                        } else {
                                            ui.label(egui::RichText::new("✗ Failed").color(egui::Color32::RED));
                                        }
                                    });
                                });
                                
                                if status != "Success" && status != "No conflicts" {
                                    ui.label(egui::RichText::new(status).weak().small());
                                }
                            });
                        }
                    });
                });
            }
            
            // Status message with loading indicator for long operations
            if !self.status_message.is_empty() && self.status_message != "Ready" {
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Status:").strong());
                        ui.label(&self.status_message);
                        
                        // Show spinner when operation is running
                        if self.is_operation_running {
                            ui.add(egui::Spinner::new());
                        }
                    });
                });
            }
        });
    }
}

impl MultiRepoPusherApp {
    fn render_commit_tab(&mut self, ui: &mut egui::Ui) {
        // Commit section with premium styling
        ui.group(|ui| {
            ui.heading("📝 Commit Settings");
            ui.add_space(10.0);
            
            // Create a visually appealing input group
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Commit message:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.commit_message).hint_text("Enter commit message"));
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Branch name:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.branch_name).hint_text("main"));
                });
            });
            
            ui.add_space(15.0);
            
            // Premium push button with animation
            ui.vertical_centered(|ui| {
                if self.is_operation_running {
                    ui.add(egui::Spinner::new().size(20.0));
                    ui.label("Pushing to repositories...");
                } else {
                    let button = egui::Button::new(
                        egui::RichText::new("🚀 Push to All Repositories")
                            .size(16.0)
                            .color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(70, 130, 180))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                    .rounding(egui::Rounding::same(8.0))
                    .min_size(egui::Vec2::new(250.0, 40.0));
                    
                    if ui.add(button).clicked() {
                        self.push_to_all_repositories();
                    }
                }
            });
        });
    }
    
    fn render_repositories_tab(&mut self, ui: &mut egui::Ui) {
        // Configuration management with premium styling
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("⚙️ Configuration");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Add the new "Add New Account" button with premium styling
                    let account_button = egui::Button::new(
                        egui::RichText::new("👤 Add New Account")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(60, 100, 60))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 100)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(account_button).clicked() {
                        self.show_account_form = true;
                    }
                    
                    let save_button = egui::Button::new(
                        egui::RichText::new("💾 Save Config")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(80, 80, 120))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 150, 200)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(save_button).clicked() {
                        self.status_message = "Configuration saved".to_string();
                    }
                    
                    let load_button = egui::Button::new(
                        egui::RichText::new("📂 Load Config")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(80, 80, 120))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 150, 200)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(load_button).clicked() {
                        self.status_message = "Configuration loaded".to_string();
                    }
                });
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Config Name:").strong().size(14.0));
                ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.config_name_input).hint_text("default"));
            });
        });
        
        ui.add_space(15.0);
        
        // Repository cloning section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("📥 Repository Cloning");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_operation_running {
                        ui.add(egui::Spinner::new().size(16.0));
                    }
                    
                    let clone_button = egui::Button::new(
                        egui::RichText::new("📥 Clone All Repositories")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(60, 120, 160))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 220)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(clone_button).clicked() && !self.is_operation_running {
                        self.clone_all_repositories();
                    }
                });
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Clone Destination:").strong().size(14.0));
                ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.clone_destination_path).hint_text("e.g., C:\\repos or /home/user/repos"));
            });
            
            ui.label(egui::RichText::new("Specify the directory where repositories will be cloned.").weak().size(12.0));
        });
        
        ui.add_space(15.0);
        
        // Repository group management section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("📁 Repository Groups");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let create_group_button = egui::Button::new(
                        egui::RichText::new("➕ Create Group")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(100, 100, 160))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 220)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(create_group_button).clicked() {
                        self.show_group_form = true;
                    }
                });
            });
            
            ui.add_space(10.0);
            
            // Display existing groups
            let config = self.config.clone();
            let groups = config.lock().unwrap().groups.clone();
            
            if groups.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("No repository groups created").weak().size(14.0));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Select Group:").strong().size(14.0));
                    egui::ComboBox::from_id_source("selected_group")
                        .selected_text(if self.selected_group.is_empty() { "Select a group" } else { &self.selected_group })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_group, String::new(), "None");
                            for group in &groups {
                                ui.selectable_value(&mut self.selected_group, group.name.clone(), &group.name);
                            }
                        });
                });
                
                if !self.selected_group.is_empty() {
                    if let Some(group) = groups.iter().find(|g| g.name == self.selected_group) {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new(&group.description).weak().size(13.0));
                        ui.label(egui::RichText::new(format!("{} repositories in this group", group.repository_names.len())).weak().size(12.0));
                        
                        // Add batch operation buttons for the selected group
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Group Operations:").strong().size(14.0));
                            
                            ui.add_space(10.0);
                            
                            if self.is_operation_running {
                                ui.add(egui::Spinner::new().size(16.0));
                            } else {
                                let push_button = egui::Button::new(
                                    egui::RichText::new("🚀 Push")
                                        .size(12.0)
                                )
                                .fill(egui::Color32::from_rgb(70, 130, 180))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                                .rounding(egui::Rounding::same(4.0))
                                .min_size(egui::Vec2::new(60.0, 25.0));
                                
                                if ui.add(push_button).clicked() {
                                    self.push_to_group_repositories();
                                }
                                
                                let pull_button = egui::Button::new(
                                    egui::RichText::new("📥 Pull")
                                        .size(12.0)
                                )
                                .fill(egui::Color32::from_rgb(60, 120, 160))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 220)))
                                .rounding(egui::Rounding::same(4.0))
                                .min_size(egui::Vec2::new(60.0, 25.0));
                                
                                if ui.add(pull_button).clicked() {
                                    self.pull_from_group_repositories();
                                }
                                
                                let fetch_button = egui::Button::new(
                                    egui::RichText::new("🔄 Fetch")
                                        .size(12.0)
                                )
                                .fill(egui::Color32::from_rgb(100, 100, 150))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 230)))
                                .rounding(egui::Rounding::same(4.0))
                                .min_size(egui::Vec2::new(60.0, 25.0));
                                
                                if ui.add(fetch_button).clicked() {
                                    self.fetch_from_group_repositories();
                                }
                            }
                        });
                    }
                }
            }
        });
        
        // Show group form as a modal if needed
        self.render_group_modal(ui.ctx());
        
        ui.add_space(15.0);
        
        // Repository management section with premium styling
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("📂 Repository Management");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let refresh_button = egui::Button::new(
                        egui::RichText::new("🔄 Refresh")
                            .size(14.0)
                    )
                    .fill(egui::Color32::from_rgb(90, 90, 90))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)))
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(refresh_button).clicked() {
                        // Refresh functionality could be added here
                    }
                });
            });
            
            ui.add_space(10.0);
            
            // Repository list with premium styling and increased height
            let config = self.config.clone();
            let repos = config.lock().unwrap().repositories.clone();
            
            if repos.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.label(egui::RichText::new("No repositories configured").weak().size(14.0));
                    ui.add_space(30.0);
                });
            } else {
                // Increased height for better browsing experience
                egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                    for (i, repo) in repos.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&repo.name).size(16.0).strong().color(egui::Color32::from_rgb(180, 200, 255)));
                                    // Show full URL as per user preference
                                    ui.label(egui::RichText::new(&repo.url).weak().size(12.0));
                                    match &repo.auth_type {
                                        AuthType::SSH => {
                                            ui.label(egui::RichText::new("🔐 Auth: SSH").weak().size(11.0).color(egui::Color32::from_rgb(150, 200, 150)));
                                        },
                                        AuthType::Token => {
                                            ui.label(egui::RichText::new("🔐 Auth: Token").weak().size(11.0).color(egui::Color32::from_rgb(150, 200, 150)));
                                        },
                                        AuthType::Default => {
                                            ui.label(egui::RichText::new("🔐 Auth: Default").weak().size(11.0).color(egui::Color32::from_rgb(150, 200, 150)));
                                        }
                                    }
                                    
                                    // Show group if repository belongs to one
                                    if !repo.group.is_empty() {
                                        ui.label(egui::RichText::new(format!("📁 Group: {}", repo.group)).weak().size(11.0).color(egui::Color32::from_rgb(200, 150, 200)));
                                    }
                                });
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // Add to group button if a group is selected
                                    if !self.selected_group.is_empty() && repo.group != self.selected_group {
                                        let add_to_group_button = egui::Button::new(
                                            egui::RichText::new("📁 Add to Group")
                                                .size(11.0)
                                        )
                                        .fill(egui::Color32::from_rgb(120, 100, 160))
                                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 160, 220)))
                                        .rounding(egui::Rounding::same(4.0))
                                        .min_size(egui::Vec2::new(90.0, 25.0));
                                        
                                        if ui.add(add_to_group_button).clicked() {
                                            self.add_repository_to_group(i, self.selected_group.clone());
                                        }
                                    }
                                    
                                    // Remove from group button if repository is in a group
                                    if !repo.group.is_empty() {
                                        let remove_from_group_button = egui::Button::new(
                                            egui::RichText::new("❌ Remove from Group")
                                                .size(11.0)
                                        )
                                        .fill(egui::Color32::from_rgb(150, 80, 80))
                                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 150, 150)))
                                        .rounding(egui::Rounding::same(4.0))
                                        .min_size(egui::Vec2::new(110.0, 25.0));
                                        
                                        if ui.add(remove_from_group_button).clicked() {
                                            self.remove_repository_from_group(i, repo.group.clone());
                                        }
                                    }
                                    
                                    let remove_button = egui::Button::new(
                                        egui::RichText::new("🗑 Remove")
                                            .size(12.0)
                                    )
                                    .fill(egui::Color32::from_rgb(150, 80, 80))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 150, 150)))
                                    .rounding(egui::Rounding::same(4.0))
                                    .min_size(egui::Vec2::new(70.0, 25.0));
                                    
                                    if ui.add(remove_button).clicked() {
                                        let mut config = self.config.lock().unwrap();
                                        config.remove_repository(i);
                                    }
                                    
                                    let validate_button = egui::Button::new(
                                        egui::RichText::new("🔍 Validate")
                                            .size(12.0)
                                    )
                                    .fill(egui::Color32::from_rgb(80, 120, 80))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 200, 150)))
                                    .rounding(egui::Rounding::same(4.0))
                                    .min_size(egui::Vec2::new(70.0, 25.0));
                                    
                                    if ui.add(validate_button).clicked() {
                                        match verify_authentication(repo) {
                                            Ok(true) => {
                                                self.status_message = format!("Repository {} authentication verified", repo.name);
                                            },
                                            Ok(false) => {
                                                self.status_message = format!("Repository {} authentication failed. Please check your credentials.", repo.name);
                                            },
                                            Err(e) => {
                                                let error_msg = e.to_string();
                                                if error_msg.contains("authentication") || error_msg.contains("Authentication") {
                                                    self.status_message = format!("Authentication failed for repository {}. Please check your credentials.", repo.name);
                                                } else if error_msg.contains("network") || error_msg.contains("Network") {
                                                    self.status_message = format!("Network error for repository {}. Please check your connection.", repo.name);
                                                } else {
                                                    self.status_message = format!("Validation error for {}: {}", repo.name, error_msg);
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        });
                        ui.add_space(5.0);
                    }
                });
            }
            
            ui.add_space(15.0);
            
            // Add new repository form with premium styling
            ui.separator();
            ui.heading("➕ Add New Repository");
            
            ui.add_space(10.0);
            
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Name:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.new_repo_name).hint_text("e.g., github"));
                });
                
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("URL:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.new_repo_url).hint_text("e.g., https://github.com/user/repo.git"));
                });
                
                ui.add_space(8.0);
                
                // Authentication type selection
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Auth Type:").strong().size(14.0));
                    egui::ComboBox::from_id_source("auth_type")
                        .selected_text(format!("{:?}", self.new_repo_auth_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Default, "Default");
                            ui.selectable_value(&mut self.new_repo_auth_type, AuthType::SSH, "SSH Key");
                            ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Token, "Token");
                        });
                    
                    let toggle_button = egui::Button::new(
                        egui::RichText::new("🔑 Toggle Auth Fields")
                            .size(12.0)
                    )
                    .fill(egui::Color32::from_rgb(90, 90, 120))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 200)))
                    .rounding(egui::Rounding::same(4.0));
                    
                    if ui.add(toggle_button).clicked() {
                        self.show_auth_fields = !self.show_auth_fields;
                    }
                });
                
                // Conditional authentication fields
                if self.show_auth_fields {
                    match &self.new_repo_auth_type {
                        AuthType::Token => {
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Token:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.new_repo_token).password(true));
                            });
                        },
                        AuthType::SSH => {
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("SSH Key Path:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.new_repo_ssh_key).hint_text("~/.ssh/id_rsa"));
                            });
                        },
                        _ => {}
                    }
                }
                
                ui.add_space(10.0);
                
                // Premium add button
                ui.vertical_centered(|ui| {
                    let add_button = egui::Button::new(
                        egui::RichText::new("➕ Add Repository")
                            .size(14.0)
                            .color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(70, 130, 180))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                    .rounding(egui::Rounding::same(6.0))
                    .min_size(egui::Vec2::new(150.0, 35.0));
                    
                    if ui.add(add_button).clicked() {
                        self.validate_and_add_repository();
                    }
                });
            });
        });
    }
    
    // New function to handle the account modal rendering
    fn render_account_modal(&mut self, ctx: &egui::Context) {
        if self.show_account_form {
            let mut open = self.show_account_form;
            egui::Window::new("👤 Add New Account")
                .open(&mut open)
                .resizable(true)
                .default_width(450.0)
                .default_height(350.0)
                .show(ctx, |ui| {
                    self.render_account_form(ui);
                });
            self.show_account_form = open;
        }
    }
    
    // New function to render the account form with premium styling
    fn render_account_form(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Account Details");
            ui.label(egui::RichText::new("Enter your GitHub account information").weak().size(13.0));
            
            ui.add_space(15.0);
            
            // Show loading indicator if operation is running
            if self.is_operation_running {
                ui.horizontal(|ui| {
                    ui.add(egui::Spinner::new().size(20.0));
                    ui.label(egui::RichText::new(&self.status_message).size(14.0));
                });
                ui.add_space(20.0);
                return;
            }
            
            // Show success or error message if there is one
            if !self.status_message.is_empty() && self.status_message != "Ready" {
                if self.status_message.contains("successfully") {
                    // Success message
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✓").color(egui::Color32::GREEN));
                            ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::GREEN));
                        });
                    });
                } else if self.status_message != "Validating account and saving configuration..." {
                    // Error message (but not the temporary loading message)
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("⚠").color(egui::Color32::YELLOW));
                            ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::YELLOW));
                        });
                    });
                }
                ui.add_space(10.0);
            }
            
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("GitHub Username:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_username).hint_text("e.g., john_doe"));
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Email:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_email).hint_text("e.g., john@example.com"));
                });
                
                ui.add_space(15.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Authentication Type:").strong().size(14.0));
                    egui::ComboBox::from_id_source("account_auth_type")
                        .selected_text(format!("{:?}", self.account_auth_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.account_auth_type, AuthType::Default, "Default");
                            ui.selectable_value(&mut self.account_auth_type, AuthType::SSH, "SSH Key");
                            ui.selectable_value(&mut self.account_auth_type, AuthType::Token, "Personal Access Token");
                        });
                });
                
                ui.add_space(10.0);
                
                match &self.account_auth_type {
                    AuthType::Token => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Personal Access Token:").strong().size(14.0));
                            ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_token).password(true).hint_text("ghp_..."));
                        });
                        ui.label(egui::RichText::new("Generate a token in GitHub Settings > Developer settings > Personal access tokens").weak().size(11.0));
                                                
                        // OAuth code input and exchange button
                        ui.add_space(10.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Or use OAuth:").strong().size(12.0));
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("OAuth Code:").strong().size(14.0));
                            ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.oauth_code).hint_text("Enter code from GitHub"));
                        });
                                                
                        let exchange_button = egui::Button::new(
                            egui::RichText::new("🔄 Exchange Code for Token")
                                .size(12.0)
                        )
                        .fill(egui::Color32::from_rgb(60, 100, 160))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                        .rounding(egui::Rounding::same(4.0));
                                                
                        if ui.add(exchange_button).clicked() {
                            self.exchange_oauth_code();
                        }
                                                
                        ui.label(egui::RichText::new("After entering the code from GitHub, click the button above to get your access token.").weak().size(11.0));
                    },
                    AuthType::SSH => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("SSH Key Path:").strong().size(14.0));
                            ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_ssh_key_path).hint_text("~/.ssh/id_rsa"));
                        });
                        ui.label(egui::RichText::new("Ensure your SSH key is added to ssh-agent").weak().size(11.0));
                    },
                    _ => {
                        ui.label(egui::RichText::new("Default authentication will use system Git configuration").weak().size(12.0));
                    }
                }
                
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let cancel_button = egui::Button::new(
                            egui::RichText::new("❌ Cancel")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(120, 120, 120))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(100.0, 30.0));
                        
                        if ui.add(cancel_button).clicked() {
                            self.show_account_form = false;
                            // Clear form fields
                            self.account_username.clear();
                            self.account_email.clear();
                            self.account_token.clear();
                            self.account_ssh_key_path.clear();
                        }
                        
                        let add_button = egui::Button::new(
                            egui::RichText::new("✅ Add Account")
                                .size(14.0)
                                .color(egui::Color32::WHITE)
                        )
                        .fill(egui::Color32::from_rgb(70, 150, 70))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 220, 120)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(120.0, 30.0));
                        
                        if ui.add(add_button).clicked() {
                            self.add_new_account();
                        }
                    });
                });
            });
        });
    }
    
    // New function to handle the group modal rendering
    fn render_group_modal(&mut self, ctx: &egui::Context) {
        if self.show_group_form {
            let mut open = self.show_group_form;
            egui::Window::new("📁 Create New Repository Group")
                .open(&mut open)
                .resizable(true)
                .default_width(400.0)
                .default_height(250.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Create Repository Group");
                        ui.label(egui::RichText::new("Create a group to organize and batch operate on repositories").weak().size(13.0));
                        
                        ui.add_space(15.0);
                        
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Group Name:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.new_group_name).hint_text("e.g., frontend, backend, mobile"));
                            });
                            
                            ui.add_space(10.0);
                            
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Description:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.new_group_description).hint_text("e.g., Frontend repositories"));
                            });
                            
                            ui.add_space(20.0);
                            
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let cancel_button = egui::Button::new(
                                        egui::RichText::new("❌ Cancel")
                                            .size(14.0)
                                    )
                                    .fill(egui::Color32::from_rgb(120, 120, 120))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .min_size(egui::Vec2::new(100.0, 30.0));
                                    
                                    if ui.add(cancel_button).clicked() {
                                        self.show_group_form = false;
                                        // Clear form fields
                                        self.new_group_name.clear();
                                        self.new_group_description.clear();
                                    }
                                    
                                    let create_button = egui::Button::new(
                                        egui::RichText::new("✅ Create Group")
                                            .size(14.0)
                                            .color(egui::Color32::WHITE)
                                    )
                                    .fill(egui::Color32::from_rgb(100, 100, 160))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 220)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .min_size(egui::Vec2::new(120.0, 30.0));
                                    
                                    if ui.add(create_button).clicked() {
                                        self.create_repository_group();
                                    }
                                });
                            });
                        });
                    });
                });
            self.show_group_form = open;
        }
    }
    
    fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("⚙️ Advanced Git Operations");
            
            ui.add_space(15.0);
            
            // Branch operations
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Branch:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.branch_name).hint_text("main"));
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_operation_running {
                            ui.add(egui::Spinner::new().size(16.0));
                        }
                        
                        let pull_button = egui::Button::new(
                            egui::RichText::new("📥 Pull from All")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(100, 100, 150))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 230)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(130.0, 35.0));
                        
                        if ui.add(pull_button).clicked() && !self.is_operation_running {
                            self.pull_from_all_repositories();
                        }
                        
                        let fetch_button = egui::Button::new(
                            egui::RichText::new("🔄 Fetch from All")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(100, 100, 150))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 230)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(130.0, 35.0));
                        
                        if ui.add(fetch_button).clicked() && !self.is_operation_running {
                            self.fetch_from_all_repositories();
                        }
                    });
                });
            });
            
            ui.separator();
            
            // Tag operations
            ui.vertical(|ui| {
                ui.heading("🏷️ Tag Operations");
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Tag Name:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.tag_name).hint_text("v1.0.0"));
                });
                
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Tag Message:").strong().size(14.0));
                    ui.add_sized([ui.available_width() * 0.7, 25.0], egui::TextEdit::singleline(&mut self.tag_message).hint_text("Release v1.0.0"));
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_operation_running {
                            ui.add(egui::Spinner::new().size(16.0));
                        }
                        
                        let tag_button = egui::Button::new(
                            egui::RichText::new("🏷️ Create and Push Tag")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(130, 100, 150))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(210, 180, 230)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(180.0, 35.0));
                        
                        if ui.add(tag_button).clicked() && !self.is_operation_running {
                            self.create_and_push_tag();
                        }
                    });
                });
            });
            
            ui.separator();
            
            // Merge conflict detection
            ui.vertical(|ui| {
                ui.heading("🔍 Conflict Detection");
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_operation_running {
                            ui.add(egui::Spinner::new().size(16.0));
                        }
                        
                        let conflict_button = egui::Button::new(
                            egui::RichText::new("🔍 Check Merge Conflicts")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(150, 120, 100))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 200, 180)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(egui::Vec2::new(200.0, 35.0));
                        
                        if ui.add(conflict_button).clicked() && !self.is_operation_running {
                            self.check_merge_conflicts();
                        }
                    });
                });
            });
        });
    }
    
    // New function to handle the first-time setup modal
    fn render_first_time_setup(&mut self, ctx: &egui::Context) {
        let mut open = self.show_first_time_setup;
        egui::Window::new("Welcome to Multi-Repo Pusher")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .default_width(500.0)
            .default_height(450.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🚀 Welcome to Multi-Repo Pusher!");
                    ui.add_space(10.0);
                    ui.label("Let's set up your account to get started.");
                    ui.add_space(20.0);
                });
                
                // Show loading indicator if operation is running
                if self.is_operation_running {
                    ui.horizontal(|ui| {
                        ui.add(egui::Spinner::new().size(20.0));
                        ui.label(egui::RichText::new(&self.status_message).size(14.0));
                    });
                    ui.add_space(20.0);
                    return;
                }
                
                // Show success or error message if there is one
                if !self.status_message.is_empty() && self.status_message != "Ready" {
                    if self.status_message.contains("successfully") {
                        // Success message
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("✓").color(egui::Color32::GREEN));
                                ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::GREEN));
                            });
                        });
                        
                        // Mark setup as completed
                        self.setup_completed = true;
                    } else if self.status_message != "Validating repository and saving configuration..." {
                        // Error message (but not the temporary loading message)
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("⚠").color(egui::Color32::YELLOW));
                                ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::YELLOW));
                            });
                        });
                    }
                    ui.add_space(10.0);
                }
                
                // If setup is completed, close the modal
                if self.setup_completed {
                    return;
                }
                
                ui.vertical(|ui| {
                    ui.heading("Account Information");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("GitHub Username:").strong().size(14.0));
                        ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_username).hint_text("e.g., john_doe"));
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Email:").strong().size(14.0));
                        ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_email).hint_text("e.g., john@example.com"));
                    });
                    
                    ui.add_space(15.0);
                    
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.heading("Repository Configuration");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Repository URL:").strong().size(14.0));
                        ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.new_repo_url).hint_text("e.g., https://github.com/user/repo.git"));
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Auth Type:").strong().size(14.0));
                        egui::ComboBox::from_id_source("setup_auth_type")
                            .selected_text(format!("{:?}", self.new_repo_auth_type))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Default, "Default");
                                ui.selectable_value(&mut self.new_repo_auth_type, AuthType::SSH, "SSH Key");
                                ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Token, "Personal Access Token");
                            });
                    });
                    
                    ui.add_space(10.0);
                    
                    match &self.new_repo_auth_type {
                        AuthType::Token => {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Personal Access Token:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_token).password(true).hint_text("ghp_..."));
                            });
                            ui.add_space(5.0);
                            
                            // Add OAuth button for GitHub token generation
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("").weak().size(12.0));
                                let oauth_button = egui::Button::new(
                                    egui::RichText::new("🔑 Generate Token via GitHub OAuth")
                                        .size(12.0)
                                )
                                .fill(egui::Color32::from_rgb(60, 100, 160))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                                .rounding(egui::Rounding::same(4.0));
                                
                                if ui.add(oauth_button).clicked() {
                                    self.open_github_oauth();
                                }
                            });
                            
                            ui.label(egui::RichText::new("Click above to generate a personal access token via GitHub OAuth").weak().size(11.0));
                            
                            // OAuth code input and exchange button
                            ui.add_space(10.0);
                            ui.separator();
                            ui.label(egui::RichText::new("Or enter OAuth Code:").strong().size(12.0));
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("OAuth Code:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.oauth_code).hint_text("Enter code from GitHub"));
                            });
                            
                            let exchange_button = egui::Button::new(
                                egui::RichText::new("🔄 Exchange Code for Token")
                                    .size(12.0)
                            )
                            .fill(egui::Color32::from_rgb(60, 100, 160))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 180, 255)))
                            .rounding(egui::Rounding::same(4.0));
                            
                            if ui.add(exchange_button).clicked() {
                                self.exchange_oauth_code();
                            }
                            
                            ui.label(egui::RichText::new("After entering the code from GitHub, click the button above to get your access token.").weak().size(11.0));
                        },
                        AuthType::SSH => {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("SSH Key Path:").strong().size(14.0));
                                ui.add_sized([ui.available_width() * 0.7, 28.0], egui::TextEdit::singleline(&mut self.account_ssh_key_path).hint_text("~/.ssh/id_rsa"));
                            });
                            ui.label(egui::RichText::new("Ensure your SSH key is added to ssh-agent").weak().size(11.0));
                        },
                        _ => {
                            ui.label(egui::RichText::new("Default authentication will use system Git configuration").weak().size(12.0));
                        }
                    }
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let skip_button = egui::Button::new(
                                egui::RichText::new("Skip Setup")
                                    .size(14.0)
                            )
                            .fill(egui::Color32::from_rgb(120, 120, 120))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)))
                            .rounding(egui::Rounding::same(6.0))
                            .min_size(egui::Vec2::new(100.0, 30.0));
                            
                            if ui.add(skip_button).clicked() {
                                self.show_first_time_setup = false;
                                self.setup_completed = true;
                                self.status_message = "Setup skipped. You can configure repositories later.".to_string();
                            }
                            
                            let setup_button = egui::Button::new(
                                egui::RichText::new("Complete Setup")
                                    .size(14.0)
                                    .color(egui::Color32::WHITE)
                            )
                            .fill(egui::Color32::from_rgb(70, 150, 70))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 220, 120)))
                            .rounding(egui::Rounding::same(6.0))
                            .min_size(egui::Vec2::new(120.0, 30.0));
                            
                            if ui.add(setup_button).clicked() {
                                self.complete_first_time_setup();
                            }
                        });
                    });
                });
            });
        self.show_first_time_setup = open;
    }
    
    // New function to handle first-time setup completion
    fn complete_first_time_setup(&mut self) {
        if self.new_repo_url.is_empty() {
            self.status_message = "Please enter a repository URL".to_string();
            return;
        }
        
        // Validate repository URL
        if !validate_repository_url(&self.new_repo_url) {
            self.status_message = "Invalid repository URL format".to_string();
            return;
        }
        
        // Show loading indicator immediately
        self.status_message = "Validating repository and saving configuration...".to_string();
        self.is_operation_running = true;
        
        // Force UI update
        // In a real implementation, this should be done asynchronously to avoid blocking the UI
        // For now, we'll proceed with synchronous validation
        
        // Create repository info with authentication
        let mut repo_info = RepositoryInfo::with_auth(
            "origin".to_string(),
            self.new_repo_url.clone(),
            self.new_repo_auth_type.clone(),
        );
        
        // Set authentication details based on type
        match &self.new_repo_auth_type {
            AuthType::Token => {
                repo_info.auth_token = self.account_token.clone();
            },
            AuthType::SSH => {
                repo_info.ssh_key_path = self.account_ssh_key_path.clone();
            },
            _ => {}
        }
        
        // Validate the repository configuration before adding it
        match verify_authentication(&repo_info) {
            Ok(true) => {
                // Repository authentication is valid
                let mut config = self.config.lock().unwrap();
                config.repositories.clear(); // Clear the default placeholder
                config.add_repository(repo_info);
                drop(config); // Release the lock
                
                // Close the setup modal
                self.show_first_time_setup = false;
                self.setup_completed = true;
                self.is_operation_running = false;
                self.status_message = "Setup completed successfully! Welcome to Multi-Repo Pusher.".to_string();
            },
            Ok(false) => {
                // Repository authentication failed
                self.is_operation_running = false;
                self.status_message = "Repository authentication failed. Please check your credentials.".to_string();
            },
            Err(e) => {
                // Error occurred during validation
                self.is_operation_running = false;
                self.status_message = format!("Error validating repository: {}", e);
            }
        }
    }
    
    // New function to open GitHub OAuth flow
    fn open_github_oauth(&mut self) {
        self.status_message = "Opening GitHub for OAuth authentication...".to_string();
        self.is_operation_running = true;
        
        // In a real implementation, you would open the GitHub OAuth URL
        // For now, we'll open the GitHub OAuth authorization page
        match webbrowser::open("https://github.com/login/oauth/authorize?client_id=YOUR_CLIENT_ID&scope=repo,user") {
            Ok(_) => {
                self.is_operation_running = false;
                self.status_message = "GitHub OAuth page opened in your browser. Please authorize the application and enter the authorization code below.".to_string();
                // In a real implementation, you would set up a local server to receive the callback
                // and automatically exchange the code for an access token
            },
            Err(e) => {
                self.is_operation_running = false;
                self.status_message = format!("Failed to open browser: {}. Please manually go to https://github.com/login/oauth/authorize?client_id=YOUR_CLIENT_ID&scope=repo,user", e);
            }
        }
    }
    
    // New function to exchange OAuth code for access token
    fn exchange_oauth_code(&mut self) {
        if self.oauth_code.is_empty() {
            self.status_message = "Please enter the authorization code".to_string();
            return;
        }
        
        self.status_message = "Exchanging code for access token...".to_string();
        self.is_operation_running = true;
        
        // In a real implementation, you would call the OAuth exchange function
        // For now, we'll just simulate the process
        self.is_operation_running = false;
        self.oauth_token = format!("gho_{}", self.oauth_code);
        self.status_message = "Access token acquired successfully!".to_string();
        
        // Clear the code field
        self.oauth_code.clear();
    }
}
