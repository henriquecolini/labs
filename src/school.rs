use crate::table::Table;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

pub const DAYS: [Day; 5] = [
    Day::Monday,
    Day::Tuesday,
    Day::Wednesday,
    Day::Thursday,
    Day::Friday,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Time(pub u8, pub u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Slot {
    pub day: Day,
    pub time: Time,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Teacher {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grade {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
    pub teacher: usize,
    pub grade: usize,
    pub subject: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassRef<'a> {
    pub teacher: &'a Teacher,
    pub grade: &'a Grade,
    pub subject: &'a Subject,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlottedClass {
    pub slot: usize,
    pub class: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlottedClassRef<'a> {
    pub slot: &'a Slot,
    pub class: ClassRef<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct School {
    pub slots: Table<Slot>,
    pub teachers: Table<Teacher>,
    pub grades: Table<Grade>,
    pub subjects: Table<Subject>,
    pub classes: Table<Class>,
    pub slotted_classes: Table<SlottedClass>,
}

#[allow(dead_code)]
impl School {
    pub fn get<'a, T: 'a>(&'a self, id: usize) -> T
    where
        Self: Store<'a, T>,
    {
        self.table().get(id).map(|it| self.flatten(it)).unwrap()
    }
    pub fn iter<'a, T: 'a>(&'a self) -> impl Iterator<Item = T> + use<'a, T>
    where
        Self: Store<'a, T>,
    {
        self.table().iter().map(|(_, v)| self.flatten(v))
    }
    pub fn slots(&self) -> impl Iterator<Item = &Slot> {
        self.iter()
    }
    pub fn teachers(&self) -> impl Iterator<Item = &Teacher> {
        self.iter()
    }
    pub fn grades(&self) -> impl Iterator<Item = &Grade> {
        self.iter()
    }
    pub fn subjects(&self) -> impl Iterator<Item = &Subject> {
        self.iter()
    }
    pub fn classes(&self) -> impl Iterator<Item = ClassRef> {
        self.iter()
    }
    pub fn slotted_classes(&self) -> impl Iterator<Item = SlottedClassRef> {
        self.iter()
    }
    pub fn retain_classes(&mut self, predicate: impl Fn(ClassRef) -> bool) {
        self.classes.retain(|k, v| {
            let cr = ClassRef {
                teacher: self.teachers.get(v.teacher).unwrap(),
                grade: self.grades.get(v.grade).unwrap(),
                subject: self.subjects.get(v.subject).unwrap(),
            };
            let retain = predicate(cr);
            if !retain {
                self.slotted_classes.retain(|_, v| v.class != *k);
            }
            retain
        });
    }
    pub fn retain_slots(&mut self, predicate: impl Fn(Slot) -> bool) {
        self.slots.retain(|k, v| {
            let retain = predicate(v.clone());
            if !retain {
                self.slotted_classes.retain(|_, v| v.slot != *k);
            }
            retain
        });
    }
}

impl FromStr for Time {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = s.split(":").map(|x| u8::from_str_radix(x, 10));
        match (time.next(), time.next(), time.next()) {
            (Some(Ok(h)), Some(Ok(m)), None) if h <= 23 && m <= 59 => Ok(Time(h, m)),
            _ => Err("Bad time format"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Time::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

pub trait Store<'a, T> {
    type Inner;
    fn table(&self) -> &Table<Self::Inner>;
    fn flatten(&'a self, item: &'a Self::Inner) -> T;
}

impl<'a> Store<'a, &'a Slot> for School {
    type Inner = Slot;
    fn table(&self) -> &Table<Self::Inner> {
        &self.slots
    }
    fn flatten(&'a self, item: &'a Self::Inner) -> &'a Slot {
        item
    }
}

impl<'a> Store<'a, &'a Teacher> for School {
    type Inner = Teacher;
    fn table(&self) -> &Table<Self::Inner> {
        &self.teachers
    }
    fn flatten(&'a self, item: &'a Self::Inner) -> &'a Teacher {
        item
    }
}

impl<'a> Store<'a, &'a Grade> for School {
    type Inner = Grade;
    fn table(&self) -> &Table<Self::Inner> {
        &self.grades
    }
    fn flatten(&'a self, item: &'a Self::Inner) -> &'a Grade {
        item
    }
}

impl<'a> Store<'a, &'a Subject> for School {
    type Inner = Subject;
    fn table(&self) -> &Table<Self::Inner> {
        &self.subjects
    }
    fn flatten(&'a self, item: &'a Self::Inner) -> &'a Subject {
        item
    }
}

impl<'a> Store<'a, ClassRef<'a>> for School {
    type Inner = Class;
    fn table(&self) -> &Table<Self::Inner> {
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
    fn table(&self) -> &Table<Self::Inner> {
        &self.slotted_classes
    }
    fn flatten(&'a self, item: &'a Self::Inner) -> SlottedClassRef<'a> {
        SlottedClassRef {
            slot: self.get(item.slot),
            class: self.get(item.class),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.day, self.time)
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

impl<'a> Display for SlottedClassRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.slot, self.class)
    }
}
