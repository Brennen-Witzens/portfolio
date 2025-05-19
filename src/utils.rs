use yew::{Hook, Reducible, UseReducerHandle, use_reducer};

#[derive(Debug, PartialEq)]
pub struct MenuState {
    pub current_state: String,
}

#[derive(Clone)]
pub enum MenuAction {
    AboutMe,
    Projects,
    Experience,
}

pub fn use_menu_context() -> impl Hook<Output = UseReducerHandle<MenuState>> + 'static {
    use_reducer(MenuState::default)
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            current_state: "AboutMe".to_string(),
        }
    }
}

impl MenuAction {
    fn as_str(&self) -> &str {
        match self {
            MenuAction::AboutMe => "AboutMe",
            MenuAction::Projects => "Projects",
            MenuAction::Experience => "Experience",
        }
    }
}

impl Into<String> for MenuAction {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl From<String> for MenuAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AboutMe" => MenuAction::AboutMe,
            "Projects" => MenuAction::Projects,
            "Experience" => MenuAction::Experience,
            _ => todo!(),
        }
    }
}

impl Reducible for MenuState {
    type Action = MenuAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Self {
            current_state: action.into(),
        }
        .into()
    }
}
