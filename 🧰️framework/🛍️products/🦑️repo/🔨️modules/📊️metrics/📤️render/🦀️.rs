//! 📤️ Render: the Markdown and text table renderers of a report.

use super::{contributor_emoji_id, display_history_branch, history_checkpoint_label, sorted_row_keys, sorted_row_keys_churn, use_full_tree_table, LocLangStats, LocReport};
use std::collections::BTreeMap;

//#region 📤️Render

/// 🏓️ A GitHub-flavoured pipe table; `since_prev` adds the Δ % column of a history row.
pub fn markdown_table(title: &str, rows: &BTreeMap<String, LocLangStats>, full_tree: bool, since_prev: bool) -> String {
    let names = if full_tree { sorted_row_keys(rows) } else { sorted_row_keys_churn(rows) };
    let mut out = String::new();
    if !title.is_empty() {
        out.push_str(&format!("### {title}\n\n"));
    }
    if full_tree && since_prev {
        out.push_str("| Category | loc | Δ% | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    } else if full_tree {
        out.push_str("| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    } else {
        out.push_str("| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: |\n");
    }
    for name in names {
        let row = rows.get(&name).cloned().unwrap_or_default();
        if full_tree && since_prev {
            let delta = row.since_prev_loc_percent.map_or_else(|| "—".to_string(), |value| format!("{value:+.2}%"));
            out.push_str(&format!("| {name} | {} | {delta} | {:.2}% | {:.2}% | {} | {} | {} |\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else if full_tree {
            out.push_str(&format!("| {name} | {} | {:.2}% | {:.2}% | {} | {} | {} |\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else {
            out.push_str(&format!("| {name} | {:.2}% | {} | {} | {} |\n", row.wip_percent, row.edited, row.added, row.removed));
        }
    }
    out
}

/// 🐙️ The whole report as GitHub-flavoured markdown.
pub fn render_markdown(report: &LocReport, with_history: bool, by_contributor: bool) -> String {
    let mut out = String::from("## LOC\n");
    out.push_str(&markdown_table("Snapshot", &report.snapshot, use_full_tree_table(&report.snapshot), false));
    out.push('\n');
    if by_contributor {
        if let Some(rows) = report.by_contributors.as_ref().filter(|rows| !rows.is_empty()) {
            out.push_str("\n## By contributor\n");
            for (alias, table) in rows {
                out.push_str(&format!("\n### {}\n\n", contributor_emoji_id(alias)));
                out.push_str(&markdown_table("", table, use_full_tree_table(table), false));
                out.push('\n');
            }
        }
    }
    if with_history {
        if let Some(history) = report.history.as_ref().filter(|entries| !entries.is_empty()) {
            out.push_str(&format!("\n## History ( {} )\n", display_history_branch(&report.branch)));
            for entry in history {
                if by_contributor {
                    out.push_str(&format!("\n#### {}  {}  {}\n", history_checkpoint_label(&entry.sha), entry.date, contributor_emoji_id(&entry.author)));
                    for (alias, table) in entry.by_contributors.iter().flatten() {
                        out.push_str(&format!("\n- **{}**\n\n", contributor_emoji_id(alias)));
                        out.push_str(&markdown_table("", table, use_full_tree_table(table), true));
                        out.push('\n');
                    }
                } else if let Some(table) = entry.languages.as_ref() {
                    out.push_str(&format!("\n#### {}  {}\n\n", history_checkpoint_label(&entry.sha), entry.date));
                    out.push_str(&markdown_table("", table, use_full_tree_table(table), true));
                    out.push('\n');
                }
            }
        }
    }
    out
}

/// 📖️ A fixed-width plain-text table; `since_prev` adds the Δ % column of a history row.
pub fn text_table(title: &str, rows: &BTreeMap<String, LocLangStats>, full_tree: bool, since_prev: bool) -> String {
    let names = if full_tree { sorted_row_keys(rows) } else { sorted_row_keys_churn(rows) };
    let mut out = String::new();
    if !title.is_empty() && !title.starts_with("  ") {
        out.push_str(&format!("{title}\n"));
    }
    if names.is_empty() {
        return out;
    }
    if full_tree && since_prev {
        out.push_str(&format!("{:<14}{:>8}{:>8}{:>7}{:>7}{:>8}{:>8}{:>8}\n", "Category", "loc", "Δ%", "%", "wip", "edited", "added", "removed"));
    } else if full_tree {
        out.push_str(&format!("{:<14}{:>8}{:>7}{:>7}{:>8}{:>8}{:>8}\n", "Category", "loc", "%", "wip", "edited", "added", "removed"));
    } else {
        out.push_str(&format!("{:<14}{:>7}{:>8}{:>8}{:>8}\n", "Category", "wip", "edited", "added", "removed"));
    }
    for name in names {
        let row = rows.get(&name).cloned().unwrap_or_default();
        if full_tree && since_prev {
            let delta = row.since_prev_loc_percent.map_or_else(|| "    —".to_string(), |value| format!("{value:+6.1}%"));
            out.push_str(&format!("{name:<14}{:>8}{delta:>8}{:>6.1}%{:>6.1}%{:>8}{:>8}{:>8}\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else if full_tree {
            out.push_str(&format!("{name:<14}{:>8}{:>6.1}%{:>6.1}%{:>8}{:>8}{:>8}\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else {
            out.push_str(&format!("{name:<14}{:>6.1}%{:>8}{:>8}{:>8}\n", row.wip_percent, row.edited, row.added, row.removed));
        }
    }
    out
}

/// 🎨️ The whole report as plain text, without any terminal colour.
pub fn render_text(report: &LocReport, with_history: bool, by_contributor: bool) -> String {
    let mut out = text_table("Snapshot", &report.snapshot, use_full_tree_table(&report.snapshot), false);
    if by_contributor {
        for (alias, table) in report.by_contributors.iter().flatten() {
            out.push_str(&format!("\nContributor: {}\n", contributor_emoji_id(alias)));
            out.push_str(&text_table("", table, use_full_tree_table(table), false));
        }
    }
    if with_history {
        if let Some(history) = report.history.as_ref().filter(|entries| !entries.is_empty()) {
            out.push_str(&format!("\nHistory: {}\n", display_history_branch(&report.branch)));
            for entry in history {
                if by_contributor {
                    out.push_str(&format!("{}  {} {}\n", history_checkpoint_label(&entry.sha), entry.date, contributor_emoji_id(&entry.author)));
                    for (alias, table) in entry.by_contributors.iter().flatten() {
                        out.push_str(&format!("  {}\n", contributor_emoji_id(alias)));
                        out.push_str(&text_table("  ", table, use_full_tree_table(table), true));
                    }
                } else if let Some(table) = entry.languages.as_ref() {
                    out.push_str(&format!("{}  {}\n", history_checkpoint_label(&entry.sha), entry.date));
                    out.push_str(&text_table("", table, use_full_tree_table(table), true));
                }
            }
        }
    }
    out
}

//#endregion 📤️Render
