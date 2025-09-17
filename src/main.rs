use eframe::egui;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct JobApplication {
    company: String,
    position: String,
    status: String,
    interview_dates: Vec<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
struct AppData {
    jobs: Vec<JobApplication>,
}

impl Default for AppData {
    fn default() -> Self {
        Self { jobs: Vec::new() }
    }
}

struct JobTrackerApp {
    data: AppData,
    new_company: String,
    new_position: String,
    new_status: String,
    selected_job_index: Option<usize>,
    new_interview_date: String,
    statuses: Vec<String>,
}

impl Default for JobTrackerApp {
    fn default() -> Self {
        Self {
            data: AppData::default(),
            new_company: String::new(),
            new_position: String::new(),
            new_status: String::new(),
            selected_job_index: None,
            new_interview_date: String::new(),
            statuses: vec![
                "Applied".to_string(),
                "Phone Screen".to_string(),
                "Interview Scheduled".to_string(),
                "Interviewed".to_string(),
                "Offer Received".to_string(),
                "Rejected".to_string(),
                "Withdrawn".to_string(),
            ],
        }
    }
}

impl JobTrackerApp {
    fn load_data(&mut self) {
        if let Ok(content) = fs::read_to_string("job_data.json") {
            if let Ok(data) = serde_json::from_str(&content) {
                self.data = data;
            }
        }
    }

    fn save_data(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.data) {
            let _ = fs::write("job_data.json", json);
        }
    }

    fn add_job(&mut self) {
        if !self.new_company.is_empty() && !self.new_position.is_empty() && !self.new_status.is_empty() {
            self.data.jobs.push(JobApplication {
                company: self.new_company.clone(),
                position: self.new_position.clone(),
                status: self.new_status.clone(),
                interview_dates: Vec::new(),
            });
            self.new_company.clear();
            self.new_position.clear();
            self.new_status.clear();
            self.save_data();
        }
    }

    fn add_interview_date(&mut self, job_index: usize) {
        if let Ok(date) = NaiveDate::parse_from_str(&self.new_interview_date, "%Y-%m-%d") {
            if let Some(job) = self.data.jobs.get_mut(job_index) {
                if !job.interview_dates.contains(&date) {
                    job.interview_dates.push(date);
                    job.interview_dates.sort();
                    self.save_data();
                    self.new_interview_date.clear();
                }
            }
        }
    }
}

impl eframe::App for JobTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark mode toggle state
        static mut DARK_MODE: bool = true;

        // Clone current style and modify based on dark mode
        let mut style = (*ctx.style()).clone();

        unsafe {
            if DARK_MODE {
                style.visuals = egui::Visuals::dark();
            } else {
                style.visuals = egui::Visuals::light();
            }
        }

        ctx.set_style(style);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unsafe {
                        if ui.button(if DARK_MODE { "Light Mode" } else { "Dark Mode" }).clicked() {
                            DARK_MODE = !DARK_MODE;
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("nextMove").size(36.0).strong());
                ui.label(egui::RichText::new("No ads. No trackers. Just your applications.").size(16.0));
            });
            ui.add_space(40.0);

            // Add Job Section
            let add_job_frame = egui::Frame::group(ui.style())
                .rounding(12.0)
                .shadow(egui::epaint::Shadow::small_dark());
            add_job_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Add New Application").size(20.0).strong());
                });
                ui.add_space(20.0);

                egui::Grid::new("add_job_grid").num_columns(2).spacing([20.0, 15.0]).show(ui, |ui| {
                    ui.label("Company");
                    ui.add(egui::TextEdit::singleline(&mut self.new_company).desired_width(250.0));
                    ui.end_row();

                    ui.label("Position");
                    ui.add(egui::TextEdit::singleline(&mut self.new_position).desired_width(250.0));
                    ui.end_row();

                    ui.label("Status");
                    egui::ComboBox::from_id_source("status_combo").selected_text(&self.new_status).show_ui(ui, |ui| {
                        for status in &self.statuses {
                            ui.selectable_value(&mut self.new_status, status.clone(), status);
                        }
                    });
                    ui.end_row();
                });

                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    let add_button = ui.add(egui::Button::new("Add Application").fill(egui::Color32::from_rgb(100, 150, 255)));
                    if add_button.clicked() {
                        self.add_job();
                    }
                });
                ui.add_space(20.0);
            });

            ui.add_space(40.0);

            // Job Applications Section
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Your Applications").size(22.0).strong());
            });
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..self.data.jobs.len() {
                    let company = self.data.jobs[i].company.clone();
                    let position = self.data.jobs[i].position.clone();
                    let status = self.data.jobs[i].status.clone();
                    let interview_dates = self.data.jobs[i].interview_dates.clone();

                    let job_frame = egui::Frame::group(ui.style())
                        .rounding(10.0)
                        .shadow(egui::epaint::Shadow::small_dark());
                    job_frame.show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.collapsing(format!("{} - {} - {}", company, position, status), |ui| {
                            ui.add_space(15.0);
                            if let Some(job) = self.data.jobs.get_mut(i) {
                                ui.horizontal(|ui| {
                                    ui.label("Status");
                                    egui::ComboBox::from_id_source(format!("status_combo_{}", i)).selected_text(&job.status).show_ui(ui, |ui| {
                                        for status_option in &self.statuses {
                                            ui.selectable_value(&mut job.status, status_option.clone(), status_option);
                                        }
                                    });
                                });
                                self.save_data();
                                ui.add_space(15.0);
                            }
                            ui.label(format!("Interview Dates ({})", interview_dates.len()));
                            if !interview_dates.is_empty() {
                                ui.add_space(10.0);
                                for date in &interview_dates {
                                    ui.label(format!("• {}", date.format("%B %d, %Y")));
                                }
                            }
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                ui.label("New Date (YYYY-MM-DD)");
                                ui.add(egui::TextEdit::singleline(&mut self.new_interview_date).desired_width(150.0));
                                if ui.add(egui::Button::new("Add").fill(egui::Color32::from_rgb(120, 180, 120))).clicked() {
                                    self.add_interview_date(i);
                                }
                            });
                        });
                    });
                    ui.add_space(15.0);
                }
                if self.data.jobs.is_empty() {
                    ui.add_space(60.0);
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("No applications yet.\nStart by adding your first job application above.").text_style(egui::TextStyle::Heading).weak());
                    });
                    ui.add_space(60.0);
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_data();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let mut app = JobTrackerApp::default();
    app.load_data();
eframe::run_native(
    "nextMove",
    options,
    Box::new(|_cc| Box::new(app)),
)
}
