use common::user::UserData;
use std::rc::Rc;
use yewdux::{Reducer, Store};

pub enum AppAction {
    UpdateUser(Rc<Option<UserData>>),
}

#[derive(Default, Clone, Debug, PartialEq, Store)]
pub struct AppState {
    pub user_data: Rc<Option<UserData>>,
}
impl Reducer<AppState> for AppAction {
    fn apply(self, _state: Rc<AppState>) -> Rc<AppState> {
        match self {
            AppAction::UpdateUser(user_data) => AppState { user_data }.into(),
        }
    }
}
