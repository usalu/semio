//! 🚨️ Errors of the goals: the goal error with its display and its conversion from the model error. See [`super`].

use super::*;

//#region ❗️Errors
/// ❗️ Everything this crate can refuse, as a class rather than a rendered string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalError {
    /// 🕳️ A required create input was blank. Carries the field name Go names in its message.
    Missing(&'static str),
    /// 🚫️ A slug is outside the allowed vocabulary of `📐️model`.
    NotAllowed(String),
    /// 📛️ A goal already exists at that identifier.
    AlreadyExists(String),
    /// 🔍️ No goal document at that identifier.
    NotFound(String),
    /// 🔁️ The lifecycle transition is a no-operation.
    AlreadyInState(&'static str),
    /// 🔤️ A title is blank or is a slug rather than a titleized phrase.
    Title(&'static str),
    /// 🔢️ A milestone or issue reference carries no number.
    Unparsable(String),
    /// 📄️ The document is not the JSON this codec accepts.
    Document(String),
    /// 🗄️ The store or the management provider refused.
    Port(String),
}

impl std::fmt::Display for GoalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalError::Missing(field) => write!(formatter, "missing {field}"),
            GoalError::NotAllowed(message) => write!(formatter, "{message}"),
            GoalError::AlreadyExists(id) => write!(formatter, "goal with id {id} already exists"),
            GoalError::NotFound(id) => write!(formatter, "goal file not found: {id}"),
            GoalError::AlreadyInState(state) => write!(formatter, "goal is already {state}"),
            GoalError::Title(message) => write!(formatter, "{message}"),
            GoalError::Unparsable(value) => write!(formatter, "could not parse number from {value}"),
            GoalError::Document(message) => write!(formatter, "{message}"),
            GoalError::Port(message) => write!(formatter, "{message}"),
        }
    }
}

impl GoalError {
    /// 🏷️ The stable class of the refusal, for a projection that must not depend on wording.
    pub fn class(&self) -> &'static str {
        match self {
            GoalError::Missing(_) => "missing",
            GoalError::NotAllowed(_) => "not-allowed",
            GoalError::AlreadyExists(_) => "already-exists",
            GoalError::NotFound(_) => "not-found",
            GoalError::AlreadyInState(_) => "already-in-state",
            GoalError::Title(_) => "title",
            GoalError::Unparsable(_) => "unparsable",
            GoalError::Document(_) => "document",
            GoalError::Port(_) => "port",
        }
    }
}

impl From<model::ModelError> for GoalError {
    fn from(error: model::ModelError) -> Self {
        GoalError::NotAllowed(error.to_string())
    }
}
//#endregion ❗️Errors
