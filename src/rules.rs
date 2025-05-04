use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::school::Time;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Rules {
    pub classes: Vec<ClassRules>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ClassRules {
    pub subject: String,
    pub teachers: Vec<TeacherRules>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TeacherRules {
    pub name: String,
    pub grades: Vec<GradeRules>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GradeRules {
    pub name: String,
    pub labs: Vec<String>,
    pub forbidden_times: Vec<Time>,
}

impl Rules {
    pub fn flatten<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str, &'a str)> {
        self.classes.iter().flat_map(|class| {
            class.teachers.iter().flat_map(|teacher| {
                teacher.grades.iter().map(|grade| {
                    (
                        teacher.name.as_ref(),
                        grade.name.as_ref(),
                        class.subject.as_ref(),
                    )
                })
            })
        })
    }
}

pub fn load_rules(p: impl AsRef<Path>) -> anyhow::Result<Rules> {
    let file = File::open(p)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn save_rules(rules: &Rules, p: impl AsRef<Path>) -> anyhow::Result<()> {
    let p = p.as_ref();
    let s = serde_json::to_string_pretty(&rules)?;
    fs::create_dir_all("input")?;
    fs::write(p, s)?;
    Ok(())
}
