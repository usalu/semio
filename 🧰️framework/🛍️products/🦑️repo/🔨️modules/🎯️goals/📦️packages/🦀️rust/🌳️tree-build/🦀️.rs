//! 🌳️ Tree build of the goals: the goal tree assembly, ticket nesting and node ordering. See [`super`].

use super::*;

//#region 🌳️Tree
/// 🌳️ Assembles the goal forest: every goal hangs under the goal its identifier prefix names, the
/// siblings sort by due date and then identifier, every ticket hangs under its goal and then under
/// its parent ticket, and the tickets that reference no known goal end up under a trailing
/// `No Goal` node.
pub fn build_goal_tree(goals: &[GoalSeed], tickets: &[TicketSeed]) -> Vec<GoalNode> {
    let mut nodes: BTreeMap<String, GoalNode> = BTreeMap::new();
    for seed in goals {
        nodes.insert(
            seed.id.clone(),
            GoalNode {
                id: seed.id.clone(),
                title: seed.title.clone(),
                status: seed.status.clone(),
                due_date: seed.due_date.clone(),
                created_at: seed.created_at.clone(),
                description: seed.description.clone(),
                children: None,
                tickets: None,
            },
        );
    }

    let mut ticket_nodes: Vec<TicketNode> = Vec::new();
    for seed in tickets {
        ticket_nodes.push(TicketNode {
            id: seed.id.clone(),
            slug: seed.slug.clone(),
            status: seed.status.clone(),
            title: seed.title.clone(),
            uri: format!("repo://ticket/{}{}", identity::emoji_text(identity::entity("ticket")), identity::flat(&seed.slug)),
            goal_id: seed.goal.clone(),
            parent_id: seed.parent.clone(),
            children: None,
            created: seed.created.clone(),
            finished: seed.finished.clone(),
            description: seed.prompt.clone(),
            summary: seed.summary.clone(),
        });
    }

    let known: Vec<String> = nodes.keys().cloned().collect();
    let mut per_goal: BTreeMap<String, Vec<TicketNode>> = BTreeMap::new();
    let mut orphans: Vec<TicketNode> = Vec::new();
    for node in ticket_nodes {
        if !node.goal_id.is_empty() && known.contains(&node.goal_id) {
            per_goal.entry(node.goal_id.clone()).or_default().push(node);
        } else {
            orphans.push(node);
        }
    }
    for (id, owned) in per_goal {
        if let Some(node) = nodes.get_mut(&id) {
            node.tickets = Some(nest_tickets(&owned));
        }
    }

    let mut child_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    for id in &known {
        let parent = match id.rfind('/') {
            Some(index) => id[..index].to_string(),
            None => String::new(),
        };
        if !parent.is_empty() && &parent != id && known.contains(&parent) {
            child_ids.entry(parent).or_default().push(id.clone());
        } else {
            root_ids.push(id.clone());
        }
    }
    let mut roots: Vec<GoalNode> = root_ids.iter().filter_map(|id| assemble_goal_node(id, &nodes, &child_ids)).collect();
    sort_goal_nodes(&mut roots);

    if !orphans.is_empty() {
        roots.push(GoalNode {
            id: String::new(),
            title: "No Goal".to_string(),
            status: String::new(),
            due_date: String::new(),
            created_at: String::new(),
            description: String::new(),
            children: None,
            tickets: Some(nest_tickets(&orphans)),
        });
    }
    roots
}

fn assemble_goal_node(id: &str, nodes: &BTreeMap<String, GoalNode>, child_ids: &BTreeMap<String, Vec<String>>) -> Option<GoalNode> {
    let mut node = nodes.get(id)?.clone();
    let children: Vec<GoalNode> = child_ids
        .get(id)
        .map(|ids| ids.iter().filter_map(|child| assemble_goal_node(child, nodes, child_ids)).collect())
        .unwrap_or_default();
    node.children = if children.is_empty() { None } else { Some(children) };
    Some(node)
}

fn nest_tickets(nodes: &[TicketNode]) -> Vec<TicketNode> {
    let ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let by_id: BTreeMap<String, TicketNode> = nodes.iter().map(|node| (node.id.clone(), node.clone())).collect();
    let mut child_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    for node in nodes {
        if !node.parent_id.is_empty() && ids.contains(&node.parent_id) && node.parent_id != node.id {
            child_ids.entry(node.parent_id.clone()).or_default().push(node.id.clone());
        } else {
            root_ids.push(node.id.clone());
        }
    }
    root_ids.iter().filter_map(|id| assemble_ticket_node(id, &by_id, &child_ids)).collect()
}

fn assemble_ticket_node(id: &str, nodes: &BTreeMap<String, TicketNode>, child_ids: &BTreeMap<String, Vec<String>>) -> Option<TicketNode> {
    let mut node = nodes.get(id)?.clone();
    let children: Vec<TicketNode> = child_ids
        .get(id)
        .map(|ids| ids.iter().filter_map(|child| assemble_ticket_node(child, nodes, child_ids)).collect())
        .unwrap_or_default();
    node.children = if children.is_empty() { None } else { Some(children) };
    Some(node)
}

fn sort_goal_nodes(nodes: &mut [GoalNode]) {
    nodes.sort_by(|left, right| {
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
    for node in nodes.iter_mut() {
        if let Some(children) = node.children.as_mut() {
            sort_goal_nodes(children);
        }
    }
}
//#endregion 🌳️Tree
