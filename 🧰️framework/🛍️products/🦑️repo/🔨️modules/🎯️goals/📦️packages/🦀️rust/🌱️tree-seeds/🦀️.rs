//! 🌱️ Tree seeds of the goals: the flat goal and ticket records the tree is assembled from, the tree format and the plain tree lines. See [`super`].

use super::*;

//#region 🌳️Tree
/// 🌱️ The flat goal record the tree is assembled from.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GoalSeed {
    /// 🪪️ The `GOAL/SUBGOAL` identifier.
    #[serde(default)]
    pub id: String,
    /// 🔤️ The human title.
    #[serde(default)]
    pub title: String,
    /// 🚦️ `open` or `closed`.
    #[serde(default)]
    pub status: String,
    /// 📅️ The due date the tree sorts by.
    #[serde(rename = "dueDate", default)]
    pub due_date: String,
    /// 🕰️ When the goal was created.
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    /// 📝️ The description.
    #[serde(default)]
    pub description: String,
}

/// 🎫️ The flat ticket record the tree is assembled from.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TicketSeed {
    /// 🪪️ The ticket identifier.
    #[serde(default)]
    pub id: String,
    /// 🔤️ The ticket slug.
    #[serde(default)]
    pub slug: String,
    /// 🚦️ `open` or `closed`.
    #[serde(default)]
    pub status: String,
    /// 🔤️ The human title.
    #[serde(default)]
    pub title: String,
    /// 🎯️ The goal the ticket hangs under, empty for none.
    #[serde(default)]
    pub goal: String,
    /// 🧬️ The parent ticket, empty for a root ticket.
    #[serde(default)]
    pub parent: String,
    /// 📝️ The prompt the ticket was opened with.
    #[serde(default)]
    pub prompt: String,
    /// 📄️ The closing summary.
    #[serde(default)]
    pub summary: String,
    /// 🕰️ When the ticket was created.
    #[serde(default)]
    pub created: String,
    /// 🏁️ When the ticket was finished.
    #[serde(default)]
    pub finished: String,
}

/// 🖨️ How a rendered tree lays its lines out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeFormat {
    /// 🌲️ Box-drawing connectors, one indentation step per level.
    Text,
    /// 📝️ Markdown list items, two spaces per level.
    Markdown,
}

/// 🖋️ Renders the content of one tree line. Entity rendering is `🪪️identity`'s concern, so the
/// tree takes the line renderer as a port instead of owning a presentation vocabulary.
pub trait TreeLines {
    /// 🎯️ The line of a goal node.
    fn goal(&self, node: &GoalNode) -> String;
    /// 🎫️ The line of a ticket node.
    fn ticket(&self, node: &TicketNode) -> String;
}

/// 🔤️ The renderer that states the identifying members and nothing else, so a rendering scenario
/// judges the shape of the tree rather than a presentation vocabulary.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlainTreeLines;

impl TreeLines for PlainTreeLines {
    fn goal(&self, node: &GoalNode) -> String {
        format!("goal {} {} [{}] {}", node.id, node.title, node.status, node.due_date)
    }

    fn ticket(&self, node: &TicketNode) -> String {
        format!("ticket {} {} [{}] {}", node.slug, node.title, node.status, node.uri)
    }
}
//#endregion 🌳️Tree
