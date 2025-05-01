mod list;

use eframe::egui;
use labs::{rules::*, sources::html::load_school};
use list::List;

struct RulesApp {
    rules: Rules,
    grades: Vec<String>,
}

const RULES_PATH: &str = "input/rules.json";
const SCHOOL_PATH: &str = "input/school.html";

impl RulesApp {
    fn new() -> Self {
        let mut grades: Vec<_> = load_school(SCHOOL_PATH)
            .map(|school| school.grades().map(|grade| grade.name.to_owned()).collect())
            .unwrap_or_default();
        grades.sort();
        Self {
            rules: load_rules(RULES_PATH).unwrap_or_default(),
            grades,
        }
    }

    fn save_rules(&self) {
        let _ = save_rules(&self.rules, RULES_PATH);
    }
}

impl eframe::App for RulesApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                List::new("Laboratórios").show_vec_default(ui, &mut self.rules.labs, |ui, (lab_idx, lab)| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut lab.name);
                        });
    
                        List::new("Matérias").show_vec_default(ui, &mut lab.classes, |ui, (class_idx, class)| {
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut class.subject);
                            });

                            List::new("Professores").show_vec_default(ui, &mut class.teachers, |ui, (teacher_idx, teacher)| {
                                ui.horizontal(|ui| {
                                    ui.text_edit_singleline(&mut teacher.name);
                                });

                                List::new("Turmas").show_vec(
                                    ui,
                                    &mut teacher.grades,
                                    || self.grades.first().map(|s| s.to_owned()).unwrap_or_default(),
                                    |ui, (i, grade)| {
                                        egui::ComboBox::from_id_salt(format!("grades_{lab_idx}_{class_idx}_{teacher_idx}_{i}"))
                                            .selected_text(grade.clone())
                                            .show_ui(ui, |ui| {
                                                for option in &self.grades {
                                                    ui.selectable_value(grade, option.to_string(), option);
                                                }
                                            });
                                    },
                                );

                                // egui::Grid::new(format!("grades_{lab_idx}_{class_idx}_{teacher_idx}")).show(ui, |ui| {
                                //     let mut to_remove = None;
                                //     for (i, grade) in teacher.grades.iter_mut().enumerate() {

                                //         if ui.button("✖").clicked() {
                                //             to_remove = Some(i);
                                //         }
                                //         ui.end_row();
                                //     }
                                //     if let Some(i) = to_remove {
                                //         teacher.grades.remove(i);
                                //     }

                                //     if ui.button("Add Grade").clicked() {
                                //         teacher.grades.push(String::new());
                                //     }
                                // });
                            });
                        })
                    });
                    ui.end_row();
                });
                ui.separator();
                if ui.button("💾 SAVE").clicked() {
                    self.save_rules();
                }
            })            
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Rules JSON Editor", options, Box::new(|_cc| Ok(Box::new(RulesApp::new()))))
}
