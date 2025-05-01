mod backtrack;

use crate::{
    rules::Rules,
    school::{School, SlottedClass},
};
use backtrack::State;
use std::collections::BTreeMap;

struct Mapping<'a> {
    map: BTreeMap<usize, usize>,
    remaining: &'a [usize],
}

struct Context<'a> {
    slots: &'a [usize],
}

impl<'a> State<'a> for Mapping<'a> {
    type Context = Context<'a>;

    fn successors(&self, ctx: &Self::Context) -> impl Iterator<Item = Self> {
        match self.remaining {
            [head, remaining @ ..] => Some(
                ctx.slots
                    .iter()
                    .filter(|lab_slot| !self.map.contains_key(lab_slot))
                    .map(|lab_slot| {
                        let mut map = self.map.clone();
                        map.insert(lab_slot.clone(), *head);
                        Mapping {
                            map,
                            remaining,
                            ..*self
                        }
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

pub fn organize(school: &School) -> Option<Vec<SlottedClass>> {
    let classes = school.classes.keys().cloned().collect::<Vec<_>>();
    let slots = school.slots.keys().map(|slot| *slot).collect::<Vec<_>>();
    if classes.len() > slots.len() {
        return None;
    }
    backtrack::solve(
        &Context { slots: &slots },
        Mapping {
            map: BTreeMap::new(),
            remaining: &classes,
        },
    )
    .map(|s| {
        s.map
            .into_iter()
            .map(|(slot, class)| SlottedClass { slot, class })
            .collect::<Vec<_>>()
    })
}

pub fn solve(rules: &Rules, school: &School) -> Vec<(String, Vec<SlottedClass>)> {
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
        let organized = organize(&school).expect("No solution");
        organized_labs.push((lab.name.to_owned(), organized));
    }
    organized_labs
}
