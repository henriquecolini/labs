pub struct LabSlottedClass {
    pub lab: usize,
    pub slot: usize,
    pub class: usize,
}

pub enum Error {
    Missing(usize),
    NoLabs(usize),
}

pub enum Warning {
    UndesiredLab {
        class: usize,
        was: usize,
        got: usize,
    },
}

pub struct Solution {
    pub slotted: Vec<LabSlottedClass>,
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>,
}
