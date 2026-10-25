//! 🪪️ Id scheme of the goals: the `GOAL/SUBGOAL` identifier depth, parent, root, filesystem and compose id conversions and the trailing number parsers. See [`super`].

use super::*;

//#region 🪪️IdScheme
/// 📏️ How many `/` separated ancestors an identifier carries.
pub fn goal_depth(goal_id: &str) -> usize {
    goal_id.matches('/').count()
}

/// 🌱️ A goal with no ancestor, which is the one that owns a management milestone.
pub fn is_root_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) == 0
}

/// 🔹️ A direct child of a root goal, which is the one whose issue joins the root milestone.
pub fn is_first_gen_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) == 1
}

/// 🔸️ A goal at depth two or deeper, whose issue is a sub-issue of its parent's issue.
pub fn is_deeper_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) >= 2
}

/// 🎯️ The compose identifier of a `GOAL/SUBGOAL` path, one `🎯️`-tagged segment per level.
pub fn goal_path_to_compose_id(goal_path: &str) -> String {
    identity::goal_path_to_compose_id(goal_path)
}

/// 🛤️ The `GOAL/SUBGOAL` path of a compose identifier, empty when it carries no goal segment.
pub fn compose_id_to_goal_path(compose_id: &str) -> String {
    identity::compose_id_to_goal_segments(compose_id).join("/")
}

/// 📂️ The filesystem path form of any identifier, accepting both the path and the compose form.
pub fn goal_id_for_filesystem(goal_id: &str) -> String {
    let goal_id = goal_id.trim();
    if goal_id.is_empty() {
        return String::new();
    }
    if goal_id.contains('/') {
        return goal_id.to_string();
    }
    if goal_id.contains(&identity::emoji_text(identity::entity("goal"))) {
        let path = compose_id_to_goal_path(goal_id);
        if !path.is_empty() && path != goal_id {
            return path;
        }
    }
    goal_id.to_string()
}

/// 🔺️ The outermost ancestor of an identifier, in filesystem path form.
pub fn root_goal_id(goal_id: &str) -> String {
    let goal_id = goal_id_for_filesystem(goal_id);
    match goal_id.find('/') {
        Some(index) => goal_id[..index].to_string(),
        None => goal_id,
    }
}

/// 🔻️ The immediate ancestor of an identifier, empty for a root goal.
pub fn parent_goal_id(goal_id: &str) -> String {
    let goal_id = goal_id_for_filesystem(goal_id);
    match goal_id.rfind('/') {
        Some(index) => goal_id[..index].to_string(),
        None => String::new(),
    }
}

/// 🧬️ The identifier a title takes under a parent: the parent path, `/`, and the title's slug.
pub fn compose_goal_id(parent: &str, title: &str) -> String {
    let slug = identity::slugify(title);
    if parent.is_empty() {
        slug
    } else {
        format!("{parent}/{slug}")
    }
}

/// 🔢️ The trailing number of a milestone reference, accepting a bare number or a URL.
pub fn parse_milestone_number(milestone: &str) -> Result<i64, GoalError> {
    parse_trailing_number(milestone)
}

/// 🎫️ The trailing number of an issue URL.
pub fn parse_issue_number(issue_url: &str) -> Result<i64, GoalError> {
    parse_trailing_number(issue_url)
}

fn parse_trailing_number(value: &str) -> Result<i64, GoalError> {
    if let Ok(number) = value.parse::<i64>() {
        return Ok(number);
    }
    if let Some(last) = value.split('/').next_back() {
        if let Ok(number) = last.parse::<i64>() {
            return Ok(number);
        }
    }
    Err(GoalError::Unparsable(value.to_string()))
}
//#endregion 🪪️IdScheme
