use askama::Template;

use crate::{
    school::{ClassRef, Laboratory, School, Teacher},
    solver::solution::{Error, Solution, Warning},
};
use std::collections::BTreeMap;

#[derive(Debug)]
struct TeacherSchedule {
    teacher_name: String,
    grades: Vec<Option<String>>,
}

#[derive(Debug)]
struct Schedule {
    lab_name: String,
    teachers: Vec<TeacherSchedule>,
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct Tabulated {
    schedules: Vec<Schedule>,
    warnings: Vec<String>,
    errors: Vec<String>,
}

fn group_by<D, K: Ord>(data: Vec<D>, get_key: impl Fn(&D) -> K) -> Vec<(K, Vec<D>)> {
    let mut groups: BTreeMap<K, Vec<_>> = BTreeMap::new();
    for item in data {
        let k = get_key(&item);
        if groups.contains_key(&k) {
            groups.get_mut(&k).unwrap().push(item);
        } else {
            groups.insert(k, vec![item]);
        }
    }
    let mut groups = groups.into_iter().collect::<Vec<_>>();
    groups.sort_by(|a, b| a.0.cmp(&b.0));
    groups
}

fn tabulate(school: &School, solution: Solution) -> Tabulated {
    let mut schedules = Vec::new();

    let labs = group_by(solution.slotted, |s| s.lab);
    for (lab_id, slotted_classes) in labs {
        let lab: &Laboratory = school.get(lab_id);
        let lab_name = lab.name.clone();
        let mut teachers = Vec::new();
        let grouped = group_by(slotted_classes, |slotted| {
            school.classes.get(slotted.class).unwrap().teacher
        });

        for (teacher_id, slotted_classes) in grouped {
            if slotted_classes.len() == 0 {
                continue;
            }

            // Create a sorted list of slots
            let mut slots: Vec<_> = school.slots.iter().collect();
            slots.sort_by(|a, b| a.1.cmp(b.1));

            let mut grades = Vec::new();
            for (slot_id, _) in slots {
                let mut found = None;
                for sl in &slotted_classes {
                    if sl.slot == *slot_id {
                        found = Some(school.get::<ClassRef>(sl.class).grade.name.clone());
                        break;
                    }
                }
                grades.push(found);
            }

            teachers.push(TeacherSchedule {
                teacher_name: school.get::<&Teacher>(teacher_id).name.clone(),
                grades,
            });
        }
        schedules.push(Schedule { lab_name, teachers });
    }

    let mut warnings = vec![];
    let mut errors = vec![];

    for warning in solution.warnings {
        match warning {
            Warning::UndesiredLab { class, was, got } => {
                let class: ClassRef = school.get(class);
                let was: &Laboratory = school.get(was);
                let got: &Laboratory = school.get(got);
                warnings.push(format!(
                    "A aula {class} não recebeu sua primeira opção de laboratório (era {was}, recebeu {got})."
                ));
            }
        }
    }

    for error in solution.errors {
        match error {
            Error::Missing(i) => {
                let class: ClassRef = school.get(i);
                errors.push(format!("O sistema não foi capaz de alocar a aula {class} devido a regras restritas demais."))
            }
            Error::NoLabs(i) => {
                let class: ClassRef = school.get(i);
                errors.push(format!("O sistema não alocou a aula {class} pois nenhum laboratório foi escolhido nas regras."))
            }
        }
    }

    Tabulated {
        schedules,
        warnings,
        errors,
    }
}

// pub fn csv(school: &School, organized_labs: Vec<(String, Result<Vec<SlottedClass>, SolveError>)>) {
//     let tabulated = tabulate(&school, organized_labs);
//     for table in tabulated.schedules {
//         println!("Horário {}", table.lab_name);
//         print!("Professor,");
//         for day in ["Segunda", "Terça", "Quarta", "Quinta", "Sexta"] {
//             print!("{day},");
//             for _ in 1..6 {
//                 print!(",")
//             }
//         }
//         println!();
//         if let Ok(teachers) = table.teachers {
//             for teacher in teachers {
//                 print!("{}", teacher.teacher_name);
//                 for class in teacher.grades {
//                     match class {
//                         Some(c) => print!(",{c}"),
//                         None => print!(","),
//                     }
//                 }
//                 println!()
//             }
//         }
//     }
// }

pub fn html(school: &School, solution: Solution) {
    let tabulated = tabulate(&school, solution);
    println!("{}", tabulated.render().unwrap());
}
