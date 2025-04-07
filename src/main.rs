mod item;
mod school;

use school::*;
use std::{fmt::Debug, fs::File, io::BufReader};

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

trait VecExt<T: PartialEq> {
	fn push_unique(&mut self, t: T) -> usize;
}

impl<T: PartialEq> VecExt<T> for Vec<T> {
	fn push_unique(&mut self, t: T) -> usize {
		match self.iter().position(|r| r == &t) {
			Some(i) => i,
			None => {
				let i = self.len();
				self.push(t);
				i
			}
		}
	}
}

impl From<Vec<TeacherAggregate>> for School {
	fn from(value: Vec<TeacherAggregate>) -> Self {
		fn extract_grade_subject(ss: String) -> Option<(String, String)> {
			let mut split = ss.split("/");
			match (split.next(), split.next()) {
				(Some(s), Some(c)) => Some((s.into(), c.into())),
				_ => None,
			}
		}
		let mut teachers = vec![];
		let mut grades = vec![];
		let mut subjects = vec![];
		let mut classes = vec![];
		let mut slots = vec![];
		let mut slotted_classes = vec![];
		for value in value {
			let teacher_id = teachers.push_unique(Teacher { name: value.nome });
			for aula in value.aulas {
				let horario: [u8; 2] = aula
					.horario
					.split(":")
					.map(|x| u8::from_str_radix(x, 10))
					.filter_map(|x| x.ok())
					.collect::<Vec<_>>()
					.try_into()
					.unwrap();
				let time = Time(horario[0], horario[1]);
				let mut handle_day = |day: Day, ss: String| {
					let Some((s, c)) = extract_grade_subject(ss) else {
						return;
					};
					let slot_id = slots.push_unique(Slot { day, time });
					let grade_id = grades.push_unique(Grade { name: s });
					let subject_id = subjects.push_unique(Subject { name: c });
					let class_id = classes.push_unique(Class {
						teacher: teacher_id,
						grade: grade_id,
						subject: subject_id,
					});
					slotted_classes.push_unique(SlottedClass {
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

fn main() {
	let ta: Vec<TeacherAggregate> =
		serde_json::from_reader(BufReader::new(File::open("sample.json").unwrap())).unwrap();
	let school: School = ta.into();
	let mut grades: Vec<_> = school.grades().collect();
	grades.sort_by_key(|g| &g.name);
	for x in grades {
		println!("{}", x);
	}
}
