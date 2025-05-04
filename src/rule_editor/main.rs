mod list;

use std::{
    collections::HashSet,
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::Error;
use eframe::egui;
use egui::{Key, Modifiers};
use egui_file_dialog::FileDialog;
use labs::{rules::*, school::Time, sources::html::load_school};
use list::List;

struct RulesApp {
    rules: Rules,
    grades: Vec<String>,
    teachers: Vec<String>,
    subjects: Vec<String>,
    labs: Vec<String>,
    times: Vec<Time>,
    last_saved: Option<Result<Instant, Error>>,
    rules_path: PathBuf,
    file_dialog: FileDialog,
}

const RULES_PATH: &str = "input/rules.json";
const SCHOOL_PATH: &str = "input/school.html";
const LABS_PATH: &str = "input/labs.json";

impl RulesApp {
    fn new() -> Self {
        let (mut grades, mut teachers, mut subjects, mut times, mut labs): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = load_school(SCHOOL_PATH, LABS_PATH)
            .map(|school| {
                (
                    school.grades().map(|grade| grade.name.to_owned()).collect(),
                    school
                        .teachers()
                        .map(|teacher| teacher.name.to_owned())
                        .collect(),
                    school
                        .subjects()
                        .map(|subject| subject.name.to_owned())
                        .collect(),
                    school
                        .slots()
                        .map(|s| s.time)
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect(),
                    school.labs().map(|s| s.name.to_owned()).collect(),
                )
            })
            .unwrap_or_default();
        grades.sort();
        teachers.sort();
        subjects.sort();
        times.sort();
        labs.sort();
        let rules_path = RULES_PATH.into();
        Self {
            rules: load_rules(&rules_path).unwrap_or_default(),
            rules_path,
            grades,
            teachers,
            subjects,
            labs,
            times,
            last_saved: None,
            file_dialog: FileDialog::new(),
        }
    }

    fn load_rules(&mut self, new_path: PathBuf) {
        match load_rules(&new_path) {
            Ok(rules) => {
                self.rules = rules;
                self.last_saved = None;
                self.rules_path = new_path;
            }
            Err(e) => {
                self.last_saved = Some(Err(e));
            }
        }
    }

    fn save_rules(&mut self) {
        self.last_saved = Some(save_rules(&self.rules, RULES_PATH).map(|_| Instant::now()))
    }
}

impl eframe::App for RulesApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.matches_logically(Modifiers::CTRL) && i.key_pressed(Key::S)) {
            self.save_rules();
        }
        egui::TopBottomPanel::top("top_panel")
            .min_height(40.)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(self.rules_path.to_string_lossy());
                    if ui.button("Abrir").clicked() {
                        self.file_dialog.pick_file();
                    }
                    if ui.button("💾 Salvar (Ctrl+S)").clicked() {
                        self.save_rules();
                    }
                    if let Some(last) = &self.last_saved {
                        match last {
                            Ok(last) => {
                                let elapsed = last.elapsed();
                                if elapsed < Duration::from_secs(20) {
                                    ui.label("Salvo agora mesmo");
                                } else if elapsed < Duration::from_secs(60) {
                                    ui.label(format!(
                                        "Salvo há {}s",
                                        (elapsed.as_secs() / 20) * 20
                                    ));
                                } else {
                                    ui.label(format!("Salvo há {}min", elapsed.as_secs() / 60));
                                }
                            }
                            Err(e) => {
                                ui.label(format!("Erro: {e}"));
                            }
                        }
                    }
                    self.file_dialog.update(ctx);
                    if let Some(path) = self.file_dialog.take_picked() {
                        self.load_rules(path)
                    }
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // List::new("Laboratório").wide().show_vec_default(
                //     ui,
                //     &mut self.rules.labs,
                //     |ui, (lab_idx, lab)| {
                //         ui.vertical(|ui| {
                //             ui.horizontal(|ui| {
                //                 ui.label("Lab.");
                //                 ui.text_edit_singleline(&mut lab.name);
                //             });
                //         });
                //         ui.end_row();
                //     },
                // );
                List::new("Matéria").wide().show_vec_default(
                    ui,
                    &mut self.rules.classes,
                    |ui, (class_idx, class)| {
                        ui.horizontal(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Mat.");
                                egui::ComboBox::from_id_salt(format!("subject_{class_idx}"))
                                    .selected_text(class.subject.clone())
                                    .show_ui(ui, |ui| {
                                        for option in &self.subjects {
                                            ui.selectable_value(
                                                &mut class.subject,
                                                option.to_string(),
                                                option,
                                            );
                                        }
                                    });
                            });

                            List::new("Professor").show_vec_default(
                                ui,
                                &mut class.teachers,
                                |ui, (teacher_idx, teacher)| {
                                    ui.horizontal(|ui| {
                                        ui.label("Prof.");
                                        egui::ComboBox::from_id_salt(format!(
                                            "teacher_{class_idx}_{teacher_idx}"
                                        ))
                                        .selected_text(teacher.name.clone())
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                for option in &self.teachers {
                                                    ui.selectable_value(
                                                        &mut teacher.name,
                                                        option.to_string(),
                                                        option,
                                                    );
                                                }
                                            },
                                        );
                                    });

                                    List::new("Turma").show_vec(
                                        ui,
                                        &mut teacher.grades,
                                        || GradeRules {
                                            name: {
                                                self.grades
                                                    .first()
                                                    .map(|s| s.to_owned())
                                                    .unwrap_or_default()
                                            },
                                            ..Default::default()
                                        },
                                        |ui, (i, grade)| {
                                            ui.horizontal(|ui| {
                                                egui::ComboBox::from_id_salt(format!(
                                                    "grades_{class_idx}_{teacher_idx}_{i}"
                                                ))
                                                .selected_text(grade.name.clone())
                                                .show_ui(ui, |ui| {
                                                    for option in &self.grades {
                                                        ui.selectable_value(
                                                            &mut grade.name,
                                                            option.to_string(),
                                                            option,
                                                        );
                                                    }
                                                });
                                                let new = grade
                                                .labs
                                                .last()
                                                .or_else(|| self.labs.first())
                                                .map(|s| s.to_owned())
                                                .unwrap_or_default();
                                                List::new("Laboratório").show_vec(
                                                    ui,
                                                    &mut grade.labs,
                                                    || new.clone(),
                                                    |ui, (lab_idx, lab)| {
                                                        egui::ComboBox::from_id_salt(format!(
                                                            "lab_{class_idx}_{teacher_idx}_{i}_{lab_idx}"
                                                        ))
                                                        .selected_text(lab.clone())
                                                        .show_ui(ui, |ui| {
                                                            for option in &self.labs {
                                                                ui.selectable_value(
                                                                    lab,
                                                                    option.to_string(),
                                                                    option,
                                                                );
                                                            }
                                                        });
                                                    },
                                                );
                                            })
                                        },
                                    );
                                },
                            );
                        })
                    },
                );

                // List::new("Horário Proibido").show_vec(
                //     ui,
                //     &mut lab.forbidden_times,
                //     || self.times.first().map(|t| *t).unwrap_or_default(),
                //     |ui, (time_idx, time)| {
                //         egui::ComboBox::from_id_salt(format!("time_{time_idx}"))
                //             .selected_text(format!("{time}"))
                //             .show_ui(ui, |ui| {
                //                 for option in &self.times {
                //                     ui.selectable_value(time, *option, option.to_string());
                //                 }
                //             });
                //     },
                // );
            })
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Editor de Regras",
        options,
        Box::new(|_cc| Ok(Box::new(RulesApp::new()))),
    )
}
