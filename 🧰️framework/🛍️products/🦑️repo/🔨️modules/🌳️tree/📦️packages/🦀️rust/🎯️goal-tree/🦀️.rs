//! 🎯️ Goal tree: the goal and ticket tree with its open-work counters and text rendering.
//!
//! Holds the `🎯️GoalTree` region of the crate root; the root re-exports its items.

use super::*;

//#region 🎯️GoalTree

/// 🎯️ Builds the goal/ticket tree: subgoals nested by id path, tickets nested by parent.
pub fn build_goal_tree(goals: &[GoalRecord], tickets: &[TicketRecord]) -> Vec<GoalNode> {
    let mut nodes: BTreeMap<String, GoalNode> = BTreeMap::new();
    for goal in goals {
        nodes.insert(
            goal.id.clone(),
            GoalNode {
                id: goal.id.clone(),
                title: goal.title.clone(),
                status: goal.status.clone(),
                due_date: goal.due_date.clone(),
                created_at: goal.created_at.clone(),
                description: goal.description.clone(),
                children: None,
                tickets: None,
            },
        );
    }

    let mut orphans: Vec<TicketNode> = Vec::new();
    for ticket in tickets {
        let node = TicketNode {
            id: ticket.id.clone(),
            slug: ticket.slug.clone(),
            status: ticket.status.clone(),
            title: ticket.title.clone(),
            uri: ticket_uri(ticket),
            goal_id: ticket.goal.clone(),
            parent_id: ticket.parent.clone(),
            children: None,
            created: ticket.started.clone(),
            finished: ticket.finished.clone(),
            description: ticket.description.clone(),
            summary: ticket.summary.clone(),
        };
        match nodes.get_mut(&ticket.goal) {
            Some(goal) if !ticket.goal.is_empty() => goal.tickets.get_or_insert_with(Vec::new).push(node),
            _ => orphans.push(node),
        }
    }

    for node in nodes.values_mut() {
        let nested = nest_tickets(node.tickets.take().unwrap_or_default());
        node.tickets = if nested.is_empty() { None } else { Some(nested) };
    }

    let items: Vec<(String, String, GoalNode)> = goals
        .iter()
        .filter_map(|goal| nodes.remove(&goal.id).map(|node| (goal.id.clone(), goal_parent_id(&goal.id), node)))
        .collect();
    let mut result = nest_ordered(items, &|parent: &mut GoalNode, child: GoalNode| parent.children.get_or_insert_with(Vec::new).push(child));
    sort_goal_nodes(&mut result);
    if !orphans.is_empty() {
        let nested = nest_tickets(orphans);
        result.push(GoalNode {
            id: String::new(),
            title: NO_GOAL_LABEL.to_string(),
            status: String::new(),
            due_date: String::new(),
            created_at: String::new(),
            description: String::new(),
            children: None,
            tickets: Some(nested),
        });
    }
    result
}

/// 🔗️ Builds the `repo://` uri of a ticket, matching the Go builder.
pub fn ticket_uri(ticket: &TicketRecord) -> String {
    if !ticket.uri.is_empty() {
        return ticket.uri.clone();
    }
    let emoji = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("ticket"));
    format!("repo://ticket/{emoji}{}", semio_framework_repo_identity::flat(&ticket.slug))
}

/// 🪜️ Nests tickets under their parent ticket, returning the ones with no parent in the set.
fn nest_tickets(tickets: Vec<TicketNode>) -> Vec<TicketNode> {
    let items: Vec<(String, String, TicketNode)> = tickets.into_iter().map(|ticket| (ticket.id.clone(), ticket.parent_id.clone(), ticket)).collect();
    nest_ordered(items, &|parent: &mut TicketNode, child: TicketNode| parent.children.get_or_insert_with(Vec::new).push(child))
}

/// 📶️ Sorts goals by due date, blank dates last, ties broken by id, recursively.
pub fn sort_goal_nodes(goals: &mut [GoalNode]) {
    goals.sort_by(|left, right| {
        if left.due_date != right.due_date {
            if left.due_date.is_empty() {
                return std::cmp::Ordering::Greater;
            }
            if right.due_date.is_empty() {
                return std::cmp::Ordering::Less;
            }
            return left.due_date.cmp(&right.due_date);
        }
        left.id.cmp(&right.id)
    });
    for goal in goals.iter_mut() {
        if let Some(children) = goal.children.as_mut() {
            sort_goal_nodes(children);
        }
    }
}

/// 🧮️ Counts the open subgoals below a goal.
pub fn count_open_subgoals(goal: &GoalNode) -> usize {
    let mut count = 0;
    for child in goal.children.as_deref().unwrap_or(&[]) {
        if child.status == "open" {
            count += 1;
        }
        count += count_open_subgoals(child);
    }
    count
}

/// 🧮️ Counts the open tickets below a goal, including the tickets of every subgoal.
pub fn count_open_tickets(goal: &GoalNode) -> usize {
    fn walk(tickets: &[TicketNode], count: &mut usize) {
        for ticket in tickets {
            if ticket.status == "open" {
                *count += 1;
            }
            walk(ticket.children.as_deref().unwrap_or(&[]), count);
        }
    }
    let mut count = 0;
    walk(goal.tickets.as_deref().unwrap_or(&[]), &mut count);
    for child in goal.children.as_deref().unwrap_or(&[]) {
        count += count_open_tickets(child);
    }
    count
}

/// 💿️ The data map a goal line renders from.
pub fn goal_node_data(goal: &GoalNode) -> BTreeMap<String, Value> {
    data_of(vec![
        ("id", text(&goal.id)),
        ("title", text(&goal.title)),
        ("status", text(&goal.status)),
        ("dueDate", text(&goal.due_date)),
        ("createdAt", text(&goal.created_at)),
        ("description", text(&goal.description)),
    ])
}

/// 🌿️ The data map a ticket line renders from.
pub fn ticket_node_data(ticket: &TicketNode) -> BTreeMap<String, Value> {
    data_of(vec![
        ("slug", text(&ticket.slug)),
        ("title", text(&ticket.title)),
        ("status", text(&ticket.status)),
        ("started", text(&ticket.created)),
        ("finished", text(&ticket.finished)),
        ("prompt", text(&ticket.description)),
        ("summary", text(&ticket.summary)),
    ])
}

/// 🖨️ The output format of a goal tree rendering.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeRenderFormat {
    /// 🖥️ The indented text form with box-drawing connectors.
    #[serde(rename = "text")]
    Text,
    /// 📰️ The markdown list form.
    #[serde(rename = "md")]
    Markdown,
}

/// ⛳️ Renders a goal tree, one line per goal and ticket.
pub fn render_goal_tree_nodes(roots: &[GoalNode], format: TreeRenderFormat, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    let last = roots.len().saturating_sub(1);
    for (index, root) in roots.iter().enumerate() {
        render_goal_node(&mut out, root, "", index == last, true, format, renderer);
    }
    out
}

/// 🎯️ Renders one goal and everything below it.
fn render_goal_node(out: &mut String, goal: &GoalNode, prefix: &str, is_last: bool, is_root: bool, format: TreeRenderFormat, renderer: &dyn EntityRenderer) {
    let data = goal_node_data(goal);
    let line = match format {
        TreeRenderFormat::Markdown => renderer.markdown_link("goal", &data),
        TreeRenderFormat::Text => renderer.human("goal", &data),
    };
    let children = goal.children.as_deref().unwrap_or(&[]);
    let tickets = goal.tickets.as_deref().unwrap_or(&[]);
    let total = children.len() + tickets.len();
    let new_prefix = write_tree_line(out, prefix, &line, is_last, is_root, format);
    for (index, child) in children.iter().enumerate() {
        render_goal_node(out, child, &new_prefix, index == total - 1, false, format, renderer);
    }
    for (index, ticket) in tickets.iter().enumerate() {
        render_ticket_node(out, ticket, &new_prefix, children.len() + index == total - 1, false, format, renderer);
    }
}

/// 🎫️ Renders one ticket and everything below it.
fn render_ticket_node(out: &mut String, ticket: &TicketNode, prefix: &str, is_last: bool, is_root: bool, format: TreeRenderFormat, renderer: &dyn EntityRenderer) {
    let data = ticket_node_data(ticket);
    let line = match format {
        TreeRenderFormat::Markdown => renderer.markdown_link("ticket", &data),
        TreeRenderFormat::Text => renderer.human("ticket", &data),
    };
    let children = ticket.children.as_deref().unwrap_or(&[]);
    let new_prefix = write_tree_line(out, prefix, &line, is_last, is_root, format);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_ticket_node(out, child, &new_prefix, index == last, false, format, renderer);
    }
}

/// ✏️ Writes one rendered line and returns the prefix its children carry.
fn write_tree_line(out: &mut String, prefix: &str, line: &str, is_last: bool, is_root: bool, format: TreeRenderFormat) -> String {
    match format {
        TreeRenderFormat::Text => {
            let connector = if is_root {
                ""
            } else if is_last {
                TEXT_LAST_CONNECTOR
            } else {
                TEXT_BRANCH_CONNECTOR
            };
            out.push_str(prefix);
            out.push_str(connector);
            out.push_str(line);
            out.push('\n');
            if is_root {
                prefix.to_string()
            } else if is_last {
                format!("{prefix}{TEXT_BLANK_INDENT}")
            } else {
                format!("{prefix}{TEXT_PIPE_INDENT}")
            }
        }
        TreeRenderFormat::Markdown => {
            out.push_str(prefix);
            out.push_str("- ");
            out.push_str(line);
            out.push('\n');
            format!("{prefix}  ")
        }
    }
}

//#endregion 🎯️GoalTree
