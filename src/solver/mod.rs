mod backtrack;
pub mod solution;

use crate::{
    rules::Rules,
    school::{Class, ClassRef, Grade, Laboratory, School, Subject, Teacher},
};
use solution::{Error, LabSlottedClass, Solution, Warning};
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LabSlotId {
    lab: usize,
    slot: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ClassId(usize);

struct State<'a> {
    decided: BTreeMap<LabSlotId, ClassId>,
    remaining: &'a [ClassId],
}

struct Context {
    slots_of: BTreeMap<ClassId, Vec<LabSlotId>>,
}

impl<'a> backtrack::State<Context> for State<'a> {
    fn successors(&self, ctx: &Context) -> impl Iterator<Item = Self> {
        match self.remaining {
            [head, remaining @ ..] => Some(
                ctx.slots_of[head]
                    .iter()
                    .filter(|slot| !self.decided.contains_key(slot))
                    .map(|slot| {
                        let mut map = self.decided.clone();
                        map.insert(slot.clone(), *head);
                        State {
                            decided: map,
                            remaining,
                        }
                    }),
            )
            .into_iter()
            .flatten(),
            _ => None.into_iter().flatten(),
        }
    }

    fn is_goal(&self, _ctx: &Context) -> bool {
        self.remaining.is_empty()
    }
}

#[derive(Debug)]
pub struct SolveError;

fn solve_for(school: &School, mut classes: Vec<(usize, Vec<usize>)>) -> Solution {
    let mut solution = Solution {
        slotted: vec![],
        errors: vec![],
        warnings: vec![],
    };
    let max_relax = classes.iter().map(|c| c.1.len()).max().unwrap_or_default();
    let mut relax = 1;
    for class in classes.iter().filter(|(_, l)| l.is_empty()) {
        solution.errors.push(Error::NoLabs(class.0))
    }
    classes.retain(|c| !c.1.is_empty());
    while !classes.is_empty() {
        let mut slots_of = BTreeMap::new();
        let mut remaining = Vec::new();
        for (class, labs) in classes.iter() {
            let slots: Vec<_> = school
                .slots_of(*class)
                .flat_map(|slot| {
                    labs.iter()
                        .take(relax)
                        .map(move |&lab| LabSlotId { lab, slot })
                })
                .collect();
            if slots.len() > 0 {
                slots_of.insert(ClassId(*class), slots);
                remaining.push(ClassId(*class));
            }
        }
        let final_state = backtrack::solve(
            &Context { slots_of },
            State {
                decided: BTreeMap::new(),
                remaining: &remaining,
            },
        );
        if let Some(s) = final_state {
            solution.slotted = s
                .decided
                .into_iter()
                .map(
                    |(LabSlotId { slot, lab }, ClassId(class))| LabSlottedClass {
                        lab,
                        slot,
                        class,
                    },
                )
                .collect::<Vec<_>>();
            break;
        }
        if relax < max_relax {
            eprintln!("Failed to use {relax}-th lab choices, relaxing");
            relax += 1;
        } else if remaining.len() > 0 {
            let first_considered = remaining[0].0;
            let class: ClassRef = school.get(first_considered);
            eprintln!("Failed to solve with max relaxation, dropping class {class}");
            solution.errors.push(Error::Missing(first_considered));
            classes.retain(|(c, _)| *c != first_considered);
        } else {
            break;
        }
    }
    for slotted in solution.slotted.iter() {
        let first_choice = classes
            .iter()
            .find(|c| c.0 == slotted.class)
            .map(|c| c.1.get(0))
            .flatten();
        if let Some(first_choice) = first_choice {
            if *first_choice != slotted.lab {
                solution.warnings.push(Warning::UndesiredLab {
                    class: (slotted.class),
                    was: *first_choice,
                    got: slotted.lab,
                })
            }
        }
    }
    solution
}

pub fn solve(school: &School, rules: &Rules) -> Solution {
    let mut classes = vec![];
    for class in &rules.classes {
        let Some(subject_id) = school.subjects.find_key(&Subject {
            name: class.subject.clone(),
        }) else {
            eprintln!("Subject does not exist: {}", class.subject);
            continue;
        };
        for teacher in &class.teachers {
            let Some(teacher_id) = school.teachers.find_key(&Teacher {
                name: teacher.name.clone(),
            }) else {
                eprintln!("Teacher does not exist: {}", teacher.name);
                continue;
            };
            for grade in &teacher.grades {
                let Some(grade_id) = school.grades.find_key(&Grade {
                    name: grade.name.clone(),
                }) else {
                    eprintln!("Grade does not exist: {}", grade.name);
                    continue;
                };
                let Some(class_id) = school.classes.find_key(&Class {
                    teacher: teacher_id,
                    grade: grade_id,
                    subject: subject_id,
                }) else {
                    eprintln!(
                        "Class does not exist: {}, {}, {}",
                        teacher.name, class.subject, grade.name
                    );
                    continue;
                };
                let mut labs = vec![];
                for lab in &grade.labs {
                    let Some(lab_id) = school.labs.find_key(&Laboratory { name: lab.clone() })
                    else {
                        eprintln!("Lab does not exist: {}", lab);
                        continue;
                    };
                    labs.push(lab_id);
                }
                classes.push((class_id, labs));
            }
        }
    }
    solve_for(school, classes)
}
