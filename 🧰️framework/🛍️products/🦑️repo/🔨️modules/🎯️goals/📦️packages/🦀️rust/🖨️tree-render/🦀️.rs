//! 🖨️ Tree render of the goals: the open counts and the rendering of the goal tree as lines. See [`super`].

use super::*;

//#region 🌳️Tree
/// 🔢️ How many open goals hang below a node, at any depth.
pub fn count_open_subgoals(node: &GoalNode) -> usize {
    node.children
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|child| usize::from(child.status == "open") + count_open_subgoals(child))
        .sum()
}

/// 🔢️ How many open tickets hang below a node, at any depth, through goals and tickets alike.
pub fn count_open_tickets(node: &GoalNode) -> usize {
    fn tickets(nodes: &[TicketNode]) -> usize {
        nodes.iter().map(|node| usize::from(node.status == "open") + tickets(node.children.as_deref().unwrap_or_default())).sum()
    }
    tickets(node.tickets.as_deref().unwrap_or_default())
        + node.children.as_deref().unwrap_or_default().iter().map(count_open_tickets).sum::<usize>()
}

/// 🎨️ Renders a goal forest, goals first and their tickets after them at every level.
pub fn render_goal_tree(roots: &[GoalNode], format: TreeFormat, lines: &dyn TreeLines) -> String {
    let mut out = String::new();
    let last = roots.len().saturating_sub(1);
    for (index, root) in roots.iter().enumerate() {
        render_goal_node(root, "", index == last, true, format, lines, &mut out);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn render_goal_node(node: &GoalNode, prefix: &str, is_last: bool, is_root: bool, format: TreeFormat, lines: &dyn TreeLines, out: &mut String) {
    let content = lines.goal(node);
    let children = node.children.as_deref().unwrap_or_default();
    let tickets = node.tickets.as_deref().unwrap_or_default();
    let total = children.len() + tickets.len();
    let next = write_line(out, prefix, &content, is_last, is_root, format);
    for (index, child) in children.iter().enumerate() {
        render_goal_node(child, &next, index == total - 1, false, format, lines, out);
    }
    for (index, ticket) in tickets.iter().enumerate() {
        render_ticket_node(ticket, &next, children.len() + index == total - 1, format, lines, out);
    }
}

fn render_ticket_node(node: &TicketNode, prefix: &str, is_last: bool, format: TreeFormat, lines: &dyn TreeLines, out: &mut String) {
    let content = lines.ticket(node);
    let children = node.children.as_deref().unwrap_or_default();
    let next = write_line(out, prefix, &content, is_last, false, format);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_ticket_node(child, &next, index == last, format, lines, out);
    }
}

fn write_line(out: &mut String, prefix: &str, content: &str, is_last: bool, is_root: bool, format: TreeFormat) -> String {
    match format {
        TreeFormat::Markdown => {
            out.push_str(prefix);
            out.push_str("- ");
            out.push_str(content);
            out.push('\n');
            format!("{prefix}  ")
        }
        TreeFormat::Text => {
            let connector = if is_root {
                ""
            } else if is_last {
                "└️─️─️ "
            } else {
                "├️─️─️ "
            };
            out.push_str(prefix);
            out.push_str(connector);
            out.push_str(content);
            out.push('\n');
            if is_root {
                prefix.to_string()
            } else if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│️   ")
            }
        }
    }
}
//#endregion 🌳️Tree
