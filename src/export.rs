use askama::Template;

use crate::school::{ClassRef, School, SlottedClass, Teacher};
use std::{cmp::Ordering, collections::BTreeMap};

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
}

fn group_by<D, K: Eq + std::hash::Hash + Ord, O>(data: Vec<D>, get_key: impl Fn(&D) -> K, data_out: impl Fn(D) -> O) -> Vec<(K, Vec<O>)> {
    let mut groups: BTreeMap<K, Vec<_>> = BTreeMap::new();
    for item in data {
        let k = get_key(&item);
        if groups.contains_key(&k) {
            groups.get_mut(&k).unwrap().push(data_out(item));
        } else {
            groups.insert(k, vec![]);
        }
    }
    let mut groups = groups.into_iter().collect::<Vec<_>>();
    groups.sort_by(|a, b| a.0.cmp(&b.0));
    groups
}

fn sorted_by<T>(mut v: Vec<T>, ordering: impl FnMut(&T, &T) -> Ordering) -> Vec<T> {
    v.sort_by(ordering);
    v
}

fn tabulate(school: &School, data: Vec<(String, Vec<SlottedClass>)>) -> Tabulated {
    let schedules = data
        .into_iter()
        .map(|(lab_name, sc)| Schedule {
            lab_name,
            teachers: group_by(sc, |d| school.classes.get(d.class).unwrap().teacher, |d| d)
                .into_iter()
                .filter(|t| t.1.len() > 0)
                .map(|(id, sl_cl)| TeacherSchedule {
                    teacher_name: school.get::<&Teacher>(id).name.clone(),
                    grades: sorted_by(school.slots.iter().collect::<Vec<_>>(), |a, b| a.1.cmp(b.1))
                        .into_iter()
                        .map(|(slot_id, _)| slot_id)
                        .map(|slot_id| {
                            sl_cl.iter().find_map(|sl_cl| {
                                if sl_cl.slot == *slot_id {
                                    Some(school.get::<ClassRef>(sl_cl.class).grade.name.clone())
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect();
    Tabulated { schedules }
}

pub fn csv(school: &School, organized_labs: Vec<(String, Vec<SlottedClass>)>) {
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
        for teacher in table.teachers {
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

pub fn html(school: &School, organized_labs: Vec<(String, Vec<SlottedClass>)>) {
    let tabulated = tabulate(&school, organized_labs);
    println!("{}", tabulated.render().unwrap());
}
