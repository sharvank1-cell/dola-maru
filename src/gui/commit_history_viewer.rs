use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::core::commit_history::{CommitInfo, CommitDiff, get_repository_commits, get_commit_diff};
use crate::core::repository::RepoConfig;

pub struct CommitHistoryViewer {
    config: Arc<Mutex<RepoConfig>>,
    selected_repo_index: Option<usize>,
    commits: Vec<CommitInfo>,
    selected_commit: Option<CommitDiff>,
    loading: bool,
    error_message: Option<String>,
    show_commit_details: bool,
}

impl CommitHistoryViewer {
    pub fn new(config: Arc<Mutex<RepoConfig>>) -> Self {
        Self {
            config,
            selected_repo_index: None,
            commits: Vec::new(),
            selected_commit: None,
            loading: false,
            error_message: None,
            show_commit_details: false,
        }
    }

    pub fn load_commit_history(&mut self) {
        if let Some(_index) = self.selected_repo_index {
            self.loading = true;
            self.error_message = None;
            
            // Get repository path - in a real implementation, you would get the actual path
            // For now, we'll use the current directory as a placeholder
            let repo_path = ".";
            
            let result = get_repository_commits(repo_path, 50);
            
            self.loading = false;
            
            match result {
                Ok(commits) => {
                    self.commits = commits;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to load commit history: {}", e));
                }
            }
        }
    }

    pub fn show_commit_details(&mut self, commit_id: &str) {
        self.loading = true;
        self.error_message = None;
        
        // Get repository path - in a real implementation, you would get the actual path
        // For now, we'll use the current directory as a placeholder
        let repo_path = ".";
        
        let result = get_commit_diff(repo_path, commit_id);
        
        self.loading = false;
        
        match result {
            Ok(commit_diff) => {
                self.selected_commit = Some(commit_diff);
                self.show_commit_details = true;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load commit details: {}", e));
            }
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("📜 Commit History Viewer");
        ui.separator();
        
        // Repository selection
        self.render_repository_selection(ui);
        
        // Loading indicator
        if self.loading {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add(egui::Spinner::new());
                ui.label("Loading commit history...");
            });
            ui.add_space(10.0);
            return;
        }
        
        // Error message
        if let Some(error) = &self.error_message {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⚠").color(egui::Color32::YELLOW));
                    ui.label(egui::RichText::new(error).color(egui::Color32::YELLOW));
                });
            });
            ui.add_space(10.0);
        }
        
        // Commit list
        if !self.commits.is_empty() {
            self.render_commit_list(ui);
        } else if self.selected_repo_index.is_some() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(egui::RichText::new("No commits found").weak());
                ui.add_space(30.0);
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(egui::RichText::new("Select a repository to view commit history").weak());
                ui.add_space(30.0);
            });
        }
        
        // Commit details modal
        self.render_commit_details_modal(ui.ctx());
    }

    fn render_repository_selection(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Repository:").strong());
                
                let repo_names: Vec<String> = {
                    let config = self.config.lock().unwrap();
                    let repos = &config.repositories;
                    repos.iter().map(|repo| repo.name.clone()).collect()
                };
                
                if repo_names.is_empty() {
                    ui.label(egui::RichText::new("No repositories configured").weak());
                } else {
                    egui::ComboBox::from_id_source("repo_selection")
                        .selected_text(
                            if let Some(index) = self.selected_repo_index {
                                if index < repo_names.len() {
                                    &repo_names[index]
                                } else {
                                    "Select repository"
                                }
                            } else {
                                "Select repository"
                            }
                        )
                        .show_ui(ui, |ui| {
                            for (i, repo_name) in repo_names.iter().enumerate() {
                                if ui.selectable_label(
                                    self.selected_repo_index == Some(i),
                                    repo_name
                                ).clicked() {
                                    self.selected_repo_index = Some(i);
                                    self.load_commit_history();
                                }
                            }
                        });
                    
                    if ui.button("🔄 Refresh").clicked() {
                        self.load_commit_history();
                    }
                }
            });
        });
        
        ui.add_space(10.0);
    }

    fn render_commit_list(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Recent Commits");
            ui.add_space(10.0);
            
            // Clone the commits to avoid borrowing issues
            let commits = self.commits.clone();
            
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    for commit in &commits {
                        let commit_id = commit.id.clone();
                        let short_id = commit.short_id.clone();
                        let message = commit.message.clone();
                        let author = commit.author.clone();
                        let author_email = commit.author_email.clone();
                        let date = commit.date;
                        
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&short_id)
                                        .monospace()
                                        .color(egui::Color32::from_rgb(100, 150, 200)));
                                    
                                    ui.label(egui::RichText::new(&message)
                                        .size(14.0)
                                        .strong());
                                    
                                    ui.label(egui::RichText::new(format!("{} <{}>", author, author_email))
                                        .weak()
                                        .size(12.0));
                                        
                                    let datetime = chrono::DateTime::from_timestamp(date, 0)
                                        .unwrap_or_else(|| chrono::Utc::now());
                                    let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                                    ui.label(egui::RichText::new(formatted_date)
                                        .weak()
                                        .size(11.0));
                                });
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("🔍 View Details").clicked() {
                                        self.show_commit_details(&commit_id);
                                    }
                                });
                            });
                        });
                        
                        ui.add_space(5.0);
                    }
                });
        });
    }

    fn render_commit_details_modal(&mut self, ctx: &egui::Context) {
        if self.show_commit_details {
            let mut open = self.show_commit_details;
            egui::Window::new("Commit Details")
                .open(&mut open)
                .resizable(true)
                .default_width(600.0)
                .default_height(500.0)
                .show(ctx, |ui| {
                    if let Some(commit_diff) = &self.selected_commit {
                        ui.group(|ui| {
                            ui.heading(egui::RichText::new(&commit_diff.commit_info.short_id).monospace());
                            ui.label(egui::RichText::new(&commit_diff.commit_info.message).size(14.0).strong());
                            
                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Author:").strong());
                                ui.label(format!("{} <{}>", 
                                    commit_diff.commit_info.author, 
                                    commit_diff.commit_info.author_email));
                            });
                            
                            let datetime = chrono::DateTime::from_timestamp(commit_diff.commit_info.date, 0)
                                .unwrap_or_else(|| chrono::Utc::now());
                            let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Date:").strong());
                                ui.label(formatted_date);
                            });
                            
                            if !commit_diff.commit_info.parents.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Parent:").strong());
                                    ui.label(egui::RichText::new(&commit_diff.commit_info.parents[0][..7]).monospace());
                                });
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        // File changes summary
                        if !commit_diff.file_changes.is_empty() {
                            ui.group(|ui| {
                                ui.heading("File Changes");
                                egui::ScrollArea::vertical()
                                    .max_height(150.0)
                                    .show(ui, |ui| {
                                        for file_change in &commit_diff.file_changes {
                                            ui.horizontal(|ui| {
                                                let status_text = match file_change.status {
                                                    crate::core::commit_history::FileChangeStatus::Added => "A",
                                                    crate::core::commit_history::FileChangeStatus::Modified => "M",
                                                    crate::core::commit_history::FileChangeStatus::Deleted => "D",
                                                    crate::core::commit_history::FileChangeStatus::Renamed => "R",
                                                };
                                                
                                                ui.label(egui::RichText::new(status_text)
                                                    .monospace()
                                                    .color(match file_change.status {
                                                        crate::core::commit_history::FileChangeStatus::Added => egui::Color32::GREEN,
                                                        crate::core::commit_history::FileChangeStatus::Modified => egui::Color32::BLUE,
                                                        crate::core::commit_history::FileChangeStatus::Deleted => egui::Color32::RED,
                                                        crate::core::commit_history::FileChangeStatus::Renamed => egui::Color32::YELLOW,
                                                    }));
                                                
                                                ui.label(&file_change.path);
                                            });
                                        }
                                    });
                            });
                            
                            ui.add_space(10.0);
                        }
                        
                        // Diff content
                        ui.group(|ui| {
                            ui.heading("Diff");
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    ui.add(egui::TextEdit::multiline(&mut commit_diff.diff_content.clone())
                                        .font(egui::TextStyle::Monospace)
                                        .desired_width(f32::INFINITY)
                                        .interactive(false));
                                });
                        });
                    } else {
                        ui.label("No commit details available");
                    }
                });
            self.show_commit_details = open;
        }
    }
}