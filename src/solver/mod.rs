mod backtrack;

use crate::{
    rules::Rules,
    school::{School, SlottedClass},
};
use backtrack::State;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SlotId(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ClassId(usize);

struct Mapping<'a> {
    map: BTreeMap<SlotId, ClassId>,
    remaining: &'a [ClassId],
}

struct Context<'a> {
    school: &'a School,
    slots: &'a [SlotId],
}

impl<'a> State<'a> for Mapping<'a> {
    type Context = Context<'a>;

    fn successors(&self, ctx: &Self::Context) -> impl Iterator<Item = Self> {
        match self.remaining {
            [head, remaining @ ..] => Some(
                ctx.slots
                    .iter()
                    .filter(|slot| !self.map.contains_key(slot))
                    .filter(|&&SlotId(slot)| {
                        ctx.school.slotted_classes.contains(&SlottedClass {
                            slot,
                            class: head.0,
                        })
                    })
                    .map(|slot| {
                        let mut map = self.map.clone();
                        map.insert(slot.clone(), *head);
                        Mapping { map, remaining }
                    }),
            )
            .into_iter()
            .flatten(),
            _ => None.into_iter().flatten(),
        }
    }

    fn is_goal(&self, _ctx: &Self::Context) -> bool {
        self.remaining.is_empty()
    }
}

#[derive(Debug)]
pub struct SolveError;

pub fn organize(school: &School) -> Result<Vec<SlottedClass>, SolveError> {
    let classes: Vec<_> = school.classes.keys().map(|class| ClassId(*class)).collect();
    let slots: Vec<_> = school.slots.keys().map(|slot| SlotId(*slot)).collect();
    if classes.len() > slots.len() {
        return Err(SolveError);
    }
    backtrack::solve(
        &Context {
            school,
            slots: &slots,
        },
        Mapping {
            map: BTreeMap::new(),
            remaining: &classes,
        },
    )
    .map(|s| {
        s.map
            .into_iter()
            .map(|(SlotId(slot), ClassId(class))| SlottedClass { slot, class })
            .collect::<Vec<_>>()
    })
    .ok_or(SolveError)
}

pub fn solve(
    rules: &Rules,
    school: &School,
) -> Vec<(String, Result<Vec<SlottedClass>, SolveError>)> {
    let mut organized_labs = vec![];
    for lab in rules.labs.iter() {
        let mut school = school.clone();
        school.retain_classes(|c| {
            lab.flatten().any(|rule| {
                rule == (
                    c.teacher.name.as_ref(),
                    c.grade.name.as_ref(),
                    c.subject.name.as_ref(),
                )
            })
        });
        school.retain_slots(|s| !lab.forbidden_times.contains(&s.time));
        let organized = organize(&school);
        organized_labs.push((lab.name.to_owned(), organized));
    }
    organized_labs
}
