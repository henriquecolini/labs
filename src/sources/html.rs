use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use encoding_rs::WINDOWS_1252;
use scraper::{Html, Selector};

use crate::{
    school::{Class, Grade, Laboratory, School, Slot, SlottedClass, Subject, Teacher, DAYS},
    table::Table,
};

pub fn load_school(
    school_path: impl AsRef<Path>,
    labs_path: impl AsRef<Path>,
) -> anyhow::Result<School> {
    let mut teachers = Table::new();
    let mut grades = Table::new();
    let mut subjects = Table::new();
    let mut classes = Table::new();
    let mut slots = Table::new();
    let mut slotted_classes = Table::new();
    let mut labs = Table::new();
    let bytes = fs::read(school_path)?;
    let (cow, _, had_errors) = WINDOWS_1252.decode(&bytes);
    if had_errors {
        eprintln!("Warning: some characters could not be decoded correctly.");
    }
    let sel_name = Selector::parse("p").unwrap();
    let sel_table = Selector::parse("table").unwrap();
    let sel_row = Selector::parse("tr").unwrap();
    let sel_cell = Selector::parse("td").unwrap();

    let doc = Html::parse_document(&cow);
    let names = doc.select(&sel_name).map(|s| s.text());
    let tables = doc.select(&sel_table).map(|s| {
        s.select(&sel_row)
            .map(|s| s.select(&sel_cell).map(|s| s.text()))
    });
    for (name, table) in names.zip(tables) {
        let name: String = name.collect();
        let name = name
            .strip_prefix("Turma ")
            .map(|n| n.to_string())
            .unwrap_or(name);
        let grade_id = grades.insert_unique(Grade { name });
        for mut row in table.skip(1) {
            let time = row
                .next()
                .expect("row must have at least 1 column")
                .next()
                .expect("time column must have at least one line")
                .parse()
                .unwrap();
            for (mut cell, &day) in row.zip(DAYS.iter()) {
                let slot_id = slots.insert_unique(Slot { day, time });
                match (cell.next(), cell.next()) {
                    (Some(subject), Some(teacher)) => {
                        let subject_id = subjects.insert_unique(Subject {
                            name: subject.replace('\u{00A0}', " "),
                        });
                        let teacher_id = teachers.insert_unique(Teacher {
                            name: teacher.replace('\u{00A0}', " "),
                        });
                        let class_id = classes.insert_unique(Class {
                            teacher: teacher_id,
                            grade: grade_id,
                            subject: subject_id,
                        });
                        slotted_classes.insert_unique(SlottedClass {
                            slot: slot_id,
                            class: class_id,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    let lab_names: Vec<String> = serde_json::from_reader(BufReader::new(File::open(labs_path)?))?;
    for name in lab_names {
        labs.insert_unique(Laboratory { name });
    }
    Ok(School {
        teachers,
        grades,
        subjects,
        classes,
        slots,
        slotted_classes,
        labs,
    })
}
