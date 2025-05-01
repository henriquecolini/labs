use crate::{school::*, table::Table};

use std::{fmt::Debug, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ClassAggregate {
    horario: String,
    segunda: String,
    terca: String,
    quarta: String,
    quinta: String,
    sexta: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TeacherAggregate {
    nome: String,
    aulas: Vec<ClassAggregate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchoolAggregate {
    professores: Vec<TeacherAggregate>,
    laboratorios: Vec<String>,
}

impl From<SchoolAggregate> for School {
    fn from(value: SchoolAggregate) -> Self {
        fn extract_grade_subject(ss: String) -> Option<(String, String)> {
            let mut split = ss.split("/");
            match (split.next(), split.next()) {
                (Some(s), Some(c)) => Some((s.into(), c.into())),
                _ => None,
            }
        }
        let mut teachers = Table::new();
        let mut grades = Table::new();
        let mut subjects = Table::new();
        let mut classes = Table::new();
        let mut slots = Table::new();
        let mut slotted_classes = Table::new();
        for professor in value.professores {
            let teacher_id = teachers.insert(Teacher {
                name: professor.nome,
            });
            for aula in professor.aulas {
                let time = aula.horario.parse().unwrap();
                let mut handle_day = |day: Day, ss: String| {
                    let Some((s, c)) = extract_grade_subject(ss) else {
                        return;
                    };
                    let slot_id = slots.insert_unique(Slot { day, time });
                    let grade_id = grades.insert_unique(Grade { name: s });
                    let subject_id = subjects.insert_unique(Subject { name: c });
                    let class_id = classes.insert_unique(Class {
                        teacher: teacher_id,
                        grade: grade_id,
                        subject: subject_id,
                    });
                    slotted_classes.insert_unique(SlottedClass {
                        slot: slot_id,
                        class: class_id,
                    });
                };
                handle_day(Day::Monday, aula.segunda);
                handle_day(Day::Tuesday, aula.terca);
                handle_day(Day::Wednesday, aula.quarta);
                handle_day(Day::Thursday, aula.quinta);
                handle_day(Day::Friday, aula.sexta);
            }
        }
        Self {
            teachers,
            grades,
            subjects,
            classes,
            slots,
            slotted_classes,
        }
    }
}

#[allow(unused)]
pub fn load_school(p: impl AsRef<Path>) -> School {
    let file = File::open(p).unwrap();
    let reader = BufReader::new(file);
    let ta: SchoolAggregate = serde_json::from_reader(reader).unwrap();
    ta.into()
}
