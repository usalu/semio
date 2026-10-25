//! 🏛️ Goals aggregate: the goal aggregate bound to its ports with its construction, list, read and search. See [`super`].

use super::*;

//#region 🔓️Lifecycle
/// 🎯️ The goal aggregate bound to its ports.
pub struct Goals<'a> {
    pub(super) store: &'a dyn GoalStore,
    pub(super) management: &'a dyn ManagementPort,
    pub(super) emitter: &'a dyn Emitter,
    pub(super) author: String,
}

impl<'a> Goals<'a> {
    /// 🆕️ Binds the aggregate to a store, a management provider, an emitter and an author alias.
    pub fn new(store: &'a dyn GoalStore, management: &'a dyn ManagementPort, emitter: &'a dyn Emitter, author: &str) -> Self {
        Self { store, management, emitter, author: author.to_string() }
    }

    /// 📋️ Every goal, identifier ascending.
    pub fn list(&self) -> Result<Vec<Goal>, GoalError> {
        self.store.ids().iter().map(|id| self.read(id)).collect()
    }

    /// 📖️ One goal by identifier, accepting the compose form.
    pub fn read(&self, id: &str) -> Result<Goal, GoalError> {
        let id = goal_id_for_filesystem(id);
        decode_goal(&id, &self.store.read(&id)?)
    }

    /// 🔎️ Every goal whose identifier, title, description or status contains the term, compared
    /// case-insensitively. A blank term keeps everything.
    pub fn search(&self, term: &str) -> Result<Vec<Goal>, GoalError> {
        let needle = term.to_lowercase();
        Ok(self
            .list()?
            .into_iter()
            .filter(|goal| {
                needle.is_empty() || format!("{} {} {} {}", goal.id, goal.title, goal.description, goal.status.as_str()).to_lowercase().contains(&needle)
            })
            .collect())
    }
}
//#endregion 🔓️Lifecycle
