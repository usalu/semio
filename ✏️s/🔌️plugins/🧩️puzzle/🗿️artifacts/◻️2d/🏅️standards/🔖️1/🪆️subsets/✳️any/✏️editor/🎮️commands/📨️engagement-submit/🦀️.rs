//! 🤝️ `engagement-submit` command.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::{puzzle2d_transform_selection, Puzzle2dActionCtx, Puzzle2dTransform};
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

/// 🤝️ Parses one typed engagement line: `select` / `brush` (utility switch), `fill [<n>]` (activate
/// the fill tool, optionally retargeting its count), `clear` (empty the live selection), the
/// transform verbs `move <dx> <dy>`, `rotate <deg>`, `scale <factor>` over the live selection, and
/// `connect <handle> <handle>` — which takes its two operands from the ORIGINAL line, since handle
/// ids are case-sensitive and the verb match reads the lowercased one.
pub fn engagement_submit(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let line = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_string();
    let value = line.to_lowercase();
    let operands: Vec<&str> = line.split_whitespace().skip(1).collect();
    let mut words = value.split_whitespace();
    let verb = words.next().unwrap_or("");
    let numbers: Vec<f64> = words.filter_map(|word| word.parse::<f64>().ok()).filter(|number| number.is_finite()).collect();
    let selected_ids = ctx.selected_ids();
    let applied = match verb {
        "connect" => {
            let (source, target) = match operands.as_slice() {
                [source, target, ..] => ((*source).to_string(), (*target).to_string()),
                // 🔗️ A bare `connect` takes the two selected handles, the same pair the context-menu row does.
                [] => match selected_ids.as_slice() {
                    [source, target] => (source.clone(), target.clone()),
                    _ => (String::new(), String::new()),
                },
                _ => (String::new(), String::new()),
            };
            if source.is_empty() || target.is_empty() {
                false
            } else {
                crate::editor::puzzle2d::commands::create_edge::create_edge(ctx, Some(&serde_json::json!({ "source": source, "target": target })));
                true
            }
        }
        "select" | "brush" => {
            // 🧰️ Reconcile the engagement text-command utility switch through the host-owned active
            // utility: point the local engine now and let the framework persist the new active utility
            // for the pane via `Effect::SetActiveUtility`.
            ctx.host.borrow_mut().set_active_utility(verb);
            ctx.effects.push(Effect::SetActiveUtility { window_id: ctx.window_id.unwrap_or(ctx.window_kind).to_string(), utility_id: verb.to_string() });
            true
        }
        "fill" => {
            // 🛠️ Fill is a mode-level tool, not a window utility — activate it through
            // `Effect::SetActiveTool`, leaving this window's active utility untouched.
            if let Some(count) = numbers.first().filter(|count| **count >= 0.0 && count.round() <= f64::from(u32::MAX)) {
                ctx.scene.runtime.fill_count = count.round() as u32;
            }
            ctx.effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
            true
        }
        "clear" => {
            ctx.interaction_writes.extend(crate::editor::puzzle2d::puzzle2d_clear_selection_write(&ctx.scene.fixture, &selected_ids));
            true
        }
        "move" if numbers.len() >= 2 => {
            puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Translate { dx: numbers[0], dy: numbers[1] });
            true
        }
        "rotate" if !numbers.is_empty() => {
            puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Rotate { radians: numbers[0].to_radians() });
            true
        }
        "scale" if numbers.first().is_some_and(|factor| *factor > 0.0) => {
            puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Scale { factor: numbers[0] });
            true
        }
        _ => false,
    };
    if applied {
        ctx.scene.runtime.engagement_input_by_pane.insert(ctx.window_kind.to_string(), String::new());
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
