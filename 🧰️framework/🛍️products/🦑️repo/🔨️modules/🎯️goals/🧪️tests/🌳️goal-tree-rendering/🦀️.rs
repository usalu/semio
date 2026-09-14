//! 🦀️ Rust side of the goal tree rendering case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

#[cfg(feature = "sut")]
fn seeds(ctx: &Context) -> Result<(Vec<semio_framework_repo_goals::GoalSeed>, Vec<semio_framework_repo_goals::TicketSeed>), String> {
    use semio_framework_repo_goals as goals;
    let file = ctx.fixture_json("shared://🌳️tree-vectors.json")?;
    let goal_seeds = file
        .array("goals")
        .iter()
        .map(|entry| goals::GoalSeed {
            id: entry.str("id"),
            title: entry.str("title"),
            status: entry.str("status"),
            due_date: entry.str("dueDate"),
            created_at: entry.str("createdAt"),
            description: entry.str("description"),
        })
        .collect();
    let ticket_seeds = file
        .array("tickets")
        .iter()
        .map(|entry| goals::TicketSeed {
            id: entry.str("id"),
            slug: entry.str("slug"),
            status: entry.str("status"),
            title: entry.str("title"),
            goal: entry.str("goal"),
            parent: entry.str("parent"),
            prompt: entry.str("prompt"),
            summary: entry.str("summary"),
            created: entry.str("created"),
            finished: entry.str("finished"),
        })
        .collect();
    Ok((goal_seeds, ticket_seeds))
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_forest_nests_and_sorts(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_goals as goals;
    let (goal_seeds, ticket_seeds) = seeds(ctx)?;
    let roots = goals::build_goal_tree(&goal_seeds, &ticket_seeds);
    let counts: Vec<Json> = roots
        .iter()
        .map(|root| Json::String(format!("{}|{}|{}", root.title, goals::count_open_subgoals(root), goals::count_open_tickets(root))))
        .collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("text".to_string(), Json::String(goals::render_goal_tree(&roots, goals::TreeFormat::Text, &goals::PlainTreeLines))),
        ("markdown".to_string(), Json::String(goals::render_goal_tree(&roots, goals::TreeFormat::Markdown, &goals::PlainTreeLines))),
        ("counts".to_string(), Json::Array(counts)),
    ])))
}

#[cfg(feature = "sut")]
fn goal_order_does_not_reach_the_rendering(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_goals as goals;
    let (goal_seeds, ticket_seeds) = seeds(ctx)?;
    let forward = goals::render_goal_tree(&goals::build_goal_tree(&goal_seeds, &ticket_seeds), goals::TreeFormat::Text, &goals::PlainTreeLines);
    let mut reversed = goal_seeds;
    reversed.reverse();
    let backward = goals::render_goal_tree(&goals::build_goal_tree(&reversed, &ticket_seeds), goals::TreeFormat::Text, &goals::PlainTreeLines);
    Ok(Outcome::projection(Json::Object(vec![
        ("stable".to_string(), Json::Bool(forward == backward)),
        ("rendering".to_string(), Json::String(forward)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-forest-nests-and-sorts", the_forest_nests_and_sorts)
        .subject("goal-order-does-not-reach-the-rendering", goal_order_does_not_reach_the_rendering);
    adapter
}

//#endregion 🔖️Registration
