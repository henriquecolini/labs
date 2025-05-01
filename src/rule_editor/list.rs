use eframe::egui::{Layout, Ui, WidgetText};
use egui::{Button, UiBuilder};

pub struct List {
    text: WidgetText,
    wide: bool,
}

pub trait ApplyVecAction {
    fn apply<T>(self, v: &mut Vec<T>, new_item: impl Fn() -> T);
    fn apply_default<T>(self, v: &mut Vec<T>)
    where
        T: Default,
        Self: Sized,
    {
        self.apply(v, Default::default);
    }
}

pub enum VecAction {
    Push,
    Remove(usize),
}

impl List {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        let text = text.into();
        Self { text, wide: false }
    }
    pub fn wide(mut self) -> Self {
        self.wide = true;
        self
    }
    pub fn show<T, R>(
        self,
        ui: &mut Ui,
        iter: impl Iterator<Item = T>,
        mut add_body: impl FnMut(&mut Ui, T) -> R,
    ) -> Option<VecAction> {
        let mut action = None;
        ui.group(|ui| {
            ui.vertical(|ui| {
                for (i, value) in iter.enumerate() {
                    ui.scope_builder(UiBuilder::new(), |ui| {
                        ui.horizontal_top(|ui| {
                            ui.scope_builder(UiBuilder::new().layout(Layout::default()), |ui| {
                                add_body(ui, value);
                            });
                            if ui.button("➖").clicked() {
                                action = Some(VecAction::Remove(i))
                            }
                        });
                    });
                }
                ui.horizontal(|ui| {
                    let btn = Button::new(format!("Adicionar {} ➕", self.text.text()));
                    if (if self.wide {
                        ui.add_sized(ui.available_size(), btn)
                    } else {
                        ui.add(btn)
                    })
                    .clicked()
                    {
                        action = Some(VecAction::Push)
                    }
                });
            });
        });
        action
    }
    pub fn show_vec<T, R>(
        self,
        ui: &mut Ui,
        vec: &mut Vec<T>,
        new_item: impl Fn() -> T,
        add_body: impl FnMut(&mut Ui, (usize, &mut T)) -> R,
    ) {
        self.show(ui, vec.iter_mut().enumerate(), add_body)
            .apply(vec, new_item);
    }
    pub fn show_vec_default<T: Default, R>(
        self,
        ui: &mut Ui,
        vec: &mut Vec<T>,
        add_body: impl FnMut(&mut Ui, (usize, &mut T)) -> R,
    ) {
        self.show(ui, vec.iter_mut().enumerate(), add_body)
            .apply_default(vec);
    }
}

impl ApplyVecAction for VecAction {
    fn apply<T>(self, v: &mut Vec<T>, new_item: impl Fn() -> T) {
        match self {
            VecAction::Push => v.push(new_item()),
            VecAction::Remove(index) => {
                v.remove(index);
            }
        }
    }
}

impl<Inner> ApplyVecAction for Option<Inner>
where
    Inner: ApplyVecAction,
{
    fn apply<T>(self, v: &mut Vec<T>, new_item: impl Fn() -> T) {
        match self {
            Some(action) => action.apply(v, new_item),
            None => (),
        }
    }
}
