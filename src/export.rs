use askama::Template;

use crate::{
    school::{ClassRef, School, SlottedClass, Teacher},
    solver::SolveError,
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
    teachers: Result<Vec<TeacherSchedule>, SolveError>,
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct Tabulated {
    schedules: Vec<Schedule>,
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

fn tabulate(
    school: &School,
    data: Vec<(String, Result<Vec<SlottedClass>, SolveError>)>,
) -> Tabulated {
    let mut schedules = Vec::new();

    for (lab_name, slotted_classes) in data {
        schedules.push(Schedule {
            lab_name,
            teachers: slotted_classes.map(|slotted_classes| {
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

                teachers
            }),
        });
    }

    Tabulated { schedules }
}

pub fn csv(school: &School, organized_labs: Vec<(String, Result<Vec<SlottedClass>, SolveError>)>) {
    let tabulated = tabulate(&school, organized_labs);
    for table in tabulated.schedules {
        println!("Horário {}", table.lab_name);
        print!("Professor,");
        for day in ["Segunda", "Terça", "Quarta", "Quinta", "Sexta"] {
            print!("{day},");
            for _ in 1..6 {
                print!(",")
            }
        }
        println!();
        if let Ok(teachers) = table.teachers {
            for teacher in teachers {
                print!("{}", teacher.teacher_name);
                for class in teacher.grades {
                    match class {
                        Some(c) => print!(",{c}"),
                        None => print!(","),
                    }
                }
                println!()
            }
        }
    }
}

pub fn html(school: &School, organized_labs: Vec<(String, Result<Vec<SlottedClass>, SolveError>)>) {
    let tabulated = tabulate(&school, organized_labs);
    println!("{}", tabulated.render().unwrap());
}
