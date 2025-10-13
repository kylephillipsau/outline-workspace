use crate::actions::Action;
use ratatui::widgets::ListState;

/// Different types of modal dialogs
#[derive(Debug, Clone, PartialEq)]
pub enum ModalType {
    None,
    ActionMenu,
    Help,
    CommandInput,
    TextInput {
        title: String,
        prompt: String,
        value: String,
        action: Action,
    },
    MultiInput {
        title: String,
        fields: Vec<InputField>,
        current_field: usize,
        action: Action,
    },
    Confirmation {
        title: String,
        message: String,
        action: Action,
    },
    List {
        title: String,
        items: Vec<String>,
        state: ListState,
    },
    Message {
        title: String,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputField {
    pub label: String,
    pub value: String,
    pub placeholder: String,
}

impl InputField {
    pub fn new(label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            placeholder: placeholder.into(),
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

/// Modal state management
pub struct Modal {
    pub modal_type: ModalType,
    pub menu_state: ListState,
}

impl Modal {
    pub fn new() -> Self {
        Self {
            modal_type: ModalType::None,
            menu_state: ListState::default(),
        }
    }

    pub fn is_open(&self) -> bool {
        !matches!(self.modal_type, ModalType::None)
    }

    pub fn close(&mut self) {
        self.modal_type = ModalType::None;
        self.menu_state = ListState::default();
    }

    pub fn show_action_menu(&mut self) {
        self.modal_type = ModalType::ActionMenu;
        let mut state = ListState::default();
        state.select(Some(0));
        self.menu_state = state;
    }

    pub fn show_help(&mut self) {
        self.modal_type = ModalType::Help;
    }

    pub fn show_command_input(&mut self) {
        self.modal_type = ModalType::CommandInput;
    }

    pub fn show_text_input(&mut self, title: String, prompt: String, action: Action) {
        self.modal_type = ModalType::TextInput {
            title,
            prompt,
            value: String::new(),
            action,
        };
    }

    pub fn show_multi_input(&mut self, title: String, fields: Vec<InputField>, action: Action) {
        self.modal_type = ModalType::MultiInput {
            title,
            fields,
            current_field: 0,
            action,
        };
    }

    pub fn show_confirmation(&mut self, title: String, message: String, action: Action) {
        self.modal_type = ModalType::Confirmation {
            title,
            message,
            action,
        };
    }

    pub fn show_list(&mut self, title: String, items: Vec<String>) {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        self.modal_type = ModalType::List {
            title,
            items,
            state,
        };
    }

    pub fn show_message(&mut self, title: String, message: String) {
        self.modal_type = ModalType::Message { title, message };
    }

    /// Handle character input for text fields
    pub fn handle_char(&mut self, c: char) {
        match &mut self.modal_type {
            ModalType::TextInput { value, .. } => {
                value.push(c);
            }
            ModalType::MultiInput { fields, current_field, .. } => {
                if let Some(field) = fields.get_mut(*current_field) {
                    field.value.push(c);
                }
            }
            _ => {}
        }
    }

    /// Handle backspace in text fields
    pub fn handle_backspace(&mut self) {
        match &mut self.modal_type {
            ModalType::TextInput { value, .. } => {
                value.pop();
            }
            ModalType::MultiInput { fields, current_field, .. } => {
                if let Some(field) = fields.get_mut(*current_field) {
                    field.value.pop();
                }
            }
            _ => {}
        }
    }

    /// Move to next field in multi-input
    pub fn next_field(&mut self) {
        if let ModalType::MultiInput { fields, current_field, .. } = &mut self.modal_type {
            if *current_field < fields.len() - 1 {
                *current_field += 1;
            }
        }
    }

    /// Move to previous field in multi-input
    pub fn previous_field(&mut self) {
        if let ModalType::MultiInput { current_field, .. } = &mut self.modal_type {
            if *current_field > 0 {
                *current_field -= 1;
            }
        }
    }

    /// Navigate menu items
    pub fn menu_next(&mut self) {
        let items_count = match &self.modal_type {
            ModalType::ActionMenu => get_action_list().len(),
            ModalType::List { items, .. } => items.len(),
            _ => return,
        };

        if items_count == 0 {
            return;
        }

        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= items_count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    pub fn menu_previous(&mut self) {
        let items_count = match &self.modal_type {
            ModalType::ActionMenu => get_action_list().len(),
            ModalType::List { items, .. } => items.len(),
            _ => return,
        };

        if items_count == 0 {
            return;
        }

        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    items_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    /// Get selected action from menu
    pub fn get_selected_action(&self) -> Option<Action> {
        if let ModalType::ActionMenu = &self.modal_type {
            self.menu_state.selected().map(|i| get_action_list()[i].clone())
        } else {
            None
        }
    }

    /// Get text input value
    pub fn get_text_input(&self) -> Option<String> {
        if let ModalType::TextInput { value, .. } = &self.modal_type {
            Some(value.clone())
        } else {
            None
        }
    }

    /// Get multi-input values
    pub fn get_multi_input(&self) -> Option<Vec<String>> {
        if let ModalType::MultiInput { fields, .. } = &self.modal_type {
            Some(fields.iter().map(|f| f.value.clone()).collect())
        } else {
            None
        }
    }

    /// Get current action for input modals
    pub fn get_pending_action(&self) -> Option<Action> {
        match &self.modal_type {
            ModalType::TextInput { action, .. } => Some(action.clone()),
            ModalType::MultiInput { action, .. } => Some(action.clone()),
            ModalType::Confirmation { action, .. } => Some(action.clone()),
            _ => None,
        }
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

/// Get list of all actions for the menu
pub fn get_action_list() -> Vec<Action> {
    vec![
        // Document operations
        Action::CreateDocument,
        Action::UpdateDocument,
        Action::DeleteDocument,
        Action::SearchDocuments,
        Action::ArchiveDocument,
        Action::UnarchiveDocument,
        Action::StarDocument,
        Action::UnstarDocument,
        Action::ExportDocument,
        Action::MoveDocument,
        Action::ViewDrafts,
        Action::ViewTemplates,
        Action::ViewRecent,

        // Collection operations
        Action::CreateCollection,
        Action::UpdateCollection,
        Action::DeleteCollection,
        Action::ExportCollection,
        Action::ViewCollectionMemberships,

        // User operations
        Action::ViewCurrentUser,
        Action::ListUsers,
        Action::InviteUser,

        // Comment operations
        Action::ViewComments,
        Action::CreateComment,

        // Group operations
        Action::ListGroups,
        Action::CreateGroup,

        // Share operations
        Action::ListShares,
        Action::CreateShare,

        // Attachment operations
        Action::ListAttachments,
        Action::UploadAttachment,

        // Navigation
        Action::Refresh,
        Action::ShowHelp,
    ]
}
