//! 🧩️ The state of a launch the daemon starts in several steps: the services that must be ready first, then
//! the processes of the launch in order, each after the previous one is ready when it declares to be.
//! The supervisor advances a group whenever a session of it changes; the group only remembers what is
//! left to do and what belongs to it.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧩️groups/🥒️.feature

use super::ipc::{GroupStop, SessionCommand};
use std::collections::VecDeque;

/// 👣 One process still to start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub session_id: String,
    pub command: SessionCommand,
    pub service: bool,
}

/// 🪢️ A launch in progress or in operation.
#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub stop: GroupStop,
    pub environment: Vec<(String, String)>,
    pub owner: u64,
    pub queue: VecDeque<Step>,
    pub waiting: Option<String>,
    pub members: Vec<String>,
    pub ending: bool,
}

impl Group {
    /// 🆕 A group whose steps are the services in order, then the members in order.
    pub fn new(id: &str, stop: GroupStop, owner: u64, environment: Vec<(String, String)>, steps: Vec<Step>) -> Self {
        let members = steps.iter().filter(|step| !step.service).map(|step| step.session_id.clone()).collect();
        Self { id: id.to_string(), stop, environment, owner, queue: steps.into(), waiting: None, members, ending: false }
    }

    /// 🔍 Whether the session is one of the processes the group stops together.
    pub fn owns(&self, session_id: &str) -> bool {
        self.members.iter().any(|member| member == session_id)
    }
}
