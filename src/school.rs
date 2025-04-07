use std::fmt::Display;

use crate::item::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time(pub u8, pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
	pub day: Day,
	pub time: Time,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teacher {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grade {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
	pub teacher: usize,
	pub grade: usize,
	pub subject: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassRef<'a> {
	pub teacher: &'a Teacher,
	pub grade: &'a Grade,
	pub subject: &'a Subject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlottedClass {
	pub slot: usize,
	pub class: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlottedClassRef<'a> {
	pub slot: &'a Slot,
	pub class: ClassRef<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct School {
	pub slots: Vec<Slot>,
	pub teachers: Vec<Teacher>,
	pub grades: Vec<Grade>,
	pub subjects: Vec<Subject>,
	pub classes: Vec<Class>,
	pub slotted_classes: Vec<SlottedClass>,
}

impl School {
	pub fn slots(&self) -> impl Iterator<Item = &Slot> {
		<Self as Store<&Slot>>::iter(self)
	}
	pub fn teachers(&self) -> impl Iterator<Item = &Teacher> {
		<Self as Store<&Teacher>>::iter(self)
	}
	pub fn grades(&self) -> impl Iterator<Item = &Grade> {
		<Self as Store<&Grade>>::iter(self)
	}
	pub fn subjects(&self) -> impl Iterator<Item = &Subject> {
		<Self as Store<&Subject>>::iter(self)
	}
	pub fn classes(&self) -> impl Iterator<Item = ClassRef> {
		<Self as Store<ClassRef>>::iter(self)
	}
	pub fn slotted_classes(&self) -> impl Iterator<Item = SlottedClassRef> {
		<Self as Store<SlottedClassRef>>::iter(self)
	}
}

impl<'a> Store<'a, &'a Slot> for School {
	type Inner = Slot;
	fn items(&self) -> &[Self::Inner] {
		&self.slots
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> &'a Self::Inner {
		item
	}
}

impl<'a> Store<'a, &'a Teacher> for School {
	type Inner = Teacher;
	fn items(&self) -> &[Self::Inner] {
		&self.teachers
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> &'a Self::Inner {
		item
	}
}

impl<'a> Store<'a, &'a Grade> for School {
	type Inner = Grade;
	fn items(&self) -> &[Self::Inner] {
		&self.grades
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> &'a Self::Inner {
		item
	}
}

impl<'a> Store<'a, &'a Subject> for School {
	type Inner = Subject;
	fn items(&self) -> &[Self::Inner] {
		&self.subjects
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> &'a Self::Inner {
		item
	}
}

impl<'a> Store<'a, ClassRef<'a>> for School {
	type Inner = Class;
	fn items(&self) -> &[Self::Inner] {
		&self.classes
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> ClassRef<'a> {
		ClassRef {
			teacher: self.get(item.teacher),
			grade: self.get(item.grade),
			subject: self.get(item.subject),
		}
	}
}

impl<'a> Store<'a, SlottedClassRef<'a>> for School {
	type Inner = SlottedClass;
	fn items(&self) -> &[Self::Inner] {
		&self.slotted_classes
	}
	fn flatten(&'a self, item: &'a Self::Inner) -> SlottedClassRef<'a> {
		SlottedClassRef {
			slot: self.get(item.slot),
			class: self.get(item.class),
		}
	}
}

impl Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Display for ClassRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}/{}", self.teacher, self.grade, self.subject)
    }
}