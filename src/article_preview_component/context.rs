use std::rc::Rc;
use yew::{Properties, Reducible, UseReducerHandle};

pub enum ModalAction {
    Toggle,
    Hide,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ModalState {
    pub show: bool,
}

impl Default for ModalState {
    fn default() -> Self {
        Self { show: false }
    }
}

impl Reducible for ModalState {
    /// Reducer Action Type
    type Action = ModalAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let show = match action {
            ModalAction::Toggle => !self.show,
            ModalAction::Hide => false,
        };

        Self { show }.into()
    }
}

pub type Context = UseReducerHandle<ModalState>;
