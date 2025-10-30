use eframe::egui;
use crate::core::repository::{RepoConfig, RepositoryInfo, AuthType};
use crate::core::git_operations::{
    push_to_all_repositories, 
    pull_from_all_repositories, 
    fetch_from_all_repositories, 
    create_and_push_tag, 
    check_merge_conflicts,
    validate_repository_url, 
    verify_authentication
};
use std::sync::{Arc, Mutex};

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
}

#[derive(PartialEq)]
enum Tab {
    Commit,
    Repositories,
    Advanced,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Commit
    }
}

impl MultiRepoPusherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Arc<Mutex<RepoConfig>>) -> Self {
        // Customize the look of the GUI
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        
        Self {
            config,
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
        
        self.status_message = "Push completed!".to_string();
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
        
        self.status_message = "Pull completed!".to_string();
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
        
        self.status_message = "Fetch completed!".to_string();
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
                    self.operation_results.push((repo_info.name.clone(), format!("Failed: {}", e)));
                }
            }
        }
        
        self.status_message = "Tag creation and push completed!".to_string();
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
                self.operation_results.push(("Repository".to_string(), format!("Error: {}", e)));
            }
        }
        
        self.is_operation_running = false;
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
}

impl eframe::App for MultiRepoPusherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header with title and styling
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Multi-Repo Pusher").size(24.0).color(egui::Color32::from_rgb(100, 150, 255)));
                ui.label(egui::RichText::new("Push your code to multiple repositories simultaneously").italics());
            });
            
            ui.separator();
            
            // Tab selection
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Commit, "-commit-");
                ui.selectable_value(&mut self.active_tab, Tab::Repositories, " Repositories ");
                ui.selectable_value(&mut self.active_tab, Tab::Advanced, " Advanced ");
            });
            
            ui.separator();
            
            match self.active_tab {
                Tab::Commit => self.render_commit_tab(ui),
                Tab::Repositories => self.render_repositories_tab(ui),
                Tab::Advanced => self.render_advanced_tab(ui),
            }
            
            // Results section
            if !self.operation_results.is_empty() {
                ui.add_space(10.0);
                
                ui.group(|ui| {
                    ui.heading("Results");
                    
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for (repo_name, status) in &self.operation_results {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(repo_name).strong());
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
            
            // Status message
            if !self.status_message.is_empty() && self.status_message != "Ready" {
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Status:").strong());
                        ui.label(&self.status_message);
                    });
                });
            }
        });
    }
}

impl MultiRepoPusherApp {
    fn render_commit_tab(&mut self, ui: &mut egui::Ui) {
        // Commit section with better styling
        ui.group(|ui| {
            ui.heading("Commit Settings");
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Commit message:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.commit_message).hint_text("Enter commit message"));
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Branch name:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.branch_name).hint_text("main"));
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_operation_running {
                        ui.add(egui::Spinner::new());
                        ui.label("Pushing...");
                    }
                    
                    if ui.button("🚀 Push to All Repositories").clicked() && !self.is_operation_running {
                        self.push_to_all_repositories();
                    }
                });
            });
        });
    }
    
    fn render_repositories_tab(&mut self, ui: &mut egui::Ui) {
        // Configuration management
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Configuration");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("💾 Save Config").clicked() {
                        self.status_message = "Configuration saved".to_string();
                    }
                    if ui.button("📂 Load Config").clicked() {
                        self.status_message = "Configuration loaded".to_string();
                    }
                });
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Config Name:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.config_name_input).hint_text("default"));
            });
        });
        
        ui.add_space(10.0);
        
        // Repository management section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Repository Management");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔄 Refresh").clicked() {
                        // Refresh functionality could be added here
                    }
                });
            });
            
            ui.add_space(5.0);
            
            // Repository list with better styling
            let config = self.config.clone();
            let repos = config.lock().unwrap().repositories.clone();
            
            if repos.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("No repositories configured").weak());
                    ui.add_space(20.0);
                });
            } else {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for (i, repo) in repos.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&repo.name).size(14.0).strong());
                                    ui.label(egui::RichText::new(&repo.url).weak().small());
                                    match &repo.auth_type {
                                        AuthType::SSH => {
                                            ui.label(egui::RichText::new("Auth: SSH").weak().small());
                                        },
                                        AuthType::Token => {
                                            ui.label(egui::RichText::new("Auth: Token").weak().small());
                                        },
                                        AuthType::Default => {
                                            ui.label(egui::RichText::new("Auth: Default").weak().small());
                                        }
                                    }
                                });
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("🗑 Remove").clicked() {
                                        let mut config = self.config.lock().unwrap();
                                        config.remove_repository(i);
                                    }
                                    if ui.button("🔍 Validate").clicked() {
                                        match verify_authentication(repo) {
                                            Ok(true) => {
                                                self.status_message = format!("Repository {} authentication verified", repo.name);
                                            },
                                            Ok(false) => {
                                                self.status_message = format!("Repository {} authentication failed", repo.name);
                                            },
                                            Err(e) => {
                                                self.status_message = format!("Validation error for {}: {}", repo.name, e);
                                            }
                                        }
                                    }
                                });
                            });
                        });
                    }
                });
            }
            
            ui.add_space(10.0);
            
            // Add new repository form
            ui.separator();
            ui.heading("Add New Repository");
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Name:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.new_repo_name).hint_text("e.g., github"));
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("URL:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.new_repo_url).hint_text("e.g., https://github.com/user/repo.git"));
            });
            
            ui.add_space(5.0);
            
            // Authentication type selection
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Auth Type:").strong());
                egui::ComboBox::from_id_source("auth_type")
                    .selected_text(format!("{:?}", self.new_repo_auth_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Default, "Default");
                        ui.selectable_value(&mut self.new_repo_auth_type, AuthType::SSH, "SSH Key");
                        ui.selectable_value(&mut self.new_repo_auth_type, AuthType::Token, "Token");
                    });
                
                if ui.button("🔑 Toggle Auth Fields").clicked() {
                    self.show_auth_fields = !self.show_auth_fields;
                }
            });
            
            // Conditional authentication fields
            if self.show_auth_fields {
                match &self.new_repo_auth_type {
                    AuthType::Token => {
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Token:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.new_repo_token).password(true));
                        });
                    },
                    AuthType::SSH => {
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("SSH Key Path:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.new_repo_ssh_key).hint_text("~/.ssh/id_rsa"));
                        });
                    },
                    _ => {}
                }
            }
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("➕ Add Repository").clicked() {
                        self.validate_and_add_repository();
                    }
                });
            });
        });
    }
    
    fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Advanced Git Operations");
            
            ui.add_space(10.0);
            
            // Branch operations
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Branch:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.branch_name).hint_text("main"));
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_operation_running {
                        ui.add(egui::Spinner::new());
                    }
                    
                    if ui.button("📥 Pull from All").clicked() && !self.is_operation_running {
                        self.pull_from_all_repositories();
                    }
                    
                    if ui.button("🔄 Fetch from All").clicked() && !self.is_operation_running {
                        self.fetch_from_all_repositories();
                    }
                });
            });
            
            ui.separator();
            
            // Tag operations
            ui.heading("Tag Operations");
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Tag Name:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.tag_name).hint_text("v1.0.0"));
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Tag Message:").strong());
                ui.add(egui::TextEdit::singleline(&mut self.tag_message).hint_text("Release v1.0.0"));
            });
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_operation_running {
                        ui.add(egui::Spinner::new());
                    }
                    
                    if ui.button("🏷️ Create and Push Tag").clicked() && !self.is_operation_running {
                        self.create_and_push_tag();
                    }
                });
            });
            
            ui.separator();
            
            // Merge conflict detection
            ui.heading("Conflict Detection");
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_operation_running {
                        ui.add(egui::Spinner::new());
                    }
                    
                    if ui.button("🔍 Check Merge Conflicts").clicked() && !self.is_operation_running {
                        self.check_merge_conflicts();
                    }
                });
            });
        });
    }
}