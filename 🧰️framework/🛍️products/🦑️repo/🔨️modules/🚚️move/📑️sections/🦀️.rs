//! 📑️ Sections: creating, renaming and deleting sections.

use super::{clean_relative, fail, find_section, language_for_path, parse_sections_with, Change, Message, MoveError, Plan, PlanEvent, Workspace};
use semio_framework_repo_events::SectionPayload;
use semio_framework_repo_languages::{format_section_both, format_section_end, format_section_start};

//#region 📑️Sections

fn last_segment(section_path: &str) -> String {
    section_path.rsplit('#').next().unwrap_or(section_path).to_string()
}

fn require_file<'a>(workspace: &'a Workspace, file_path: &str) -> Result<&'a str, MoveError> {
    match workspace.file(file_path) {
        Some(content) => Ok(content),
        None => fail(format!("File not found: {file_path}")),
    }
}

/// 🏷️ Appends an empty section skeleton to a file.
pub fn plan_section_create(workspace: &Workspace, file_path: &str, section_path: &str) -> Result<Plan, MoveError> {
    let content = require_file(workspace, file_path)?;
    let section_name = last_segment(section_path);
    let Some(lang) = language_for_path(file_path) else {
        return fail("Unsupported file type");
    };
    if !lang.supports_sections() {
        return fail("Unsupported file type");
    }
    let skeleton = format_section_both(lang, &section_name);
    if skeleton.is_empty() {
        return fail("Cannot create section for this file type");
    }
    let mut plan = Plan::empty();
    let payload = SectionPayload { file: file_path.to_string(), name: section_path.to_string(), ..SectionPayload::default() };
    plan.events.push(PlanEvent::new("section.create.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content: format!("{content}{skeleton}") });
    plan.events.push(PlanEvent::new("section.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🏷️Created section \"{section_name}\" in {file_path}")));
    Ok(plan)
}

/// 📕️ Renames a section by rewriting its opening and closing markers in place.
pub fn plan_section_move(workspace: &Workspace, file_path: &str, old_path: &str, new_path: &str) -> Result<Plan, MoveError> {
    let original = require_file(workspace, file_path)?;
    let old_name = last_segment(old_path);
    let new_name = last_segment(new_path);
    let mut content = original.to_string();
    if let Some(lang) = language_for_path(file_path) {
        if lang.supports_sections() {
            let old_start = format_section_start(lang, &old_name);
            let new_start = format_section_start(lang, &new_name);
            if !old_start.is_empty() && !new_start.is_empty() {
                content = content.replace(&old_start, &new_start);
            }
            let old_end = format_section_end(lang, &old_name);
            let new_end = format_section_end(lang, &new_name);
            if !old_end.is_empty() && !new_end.is_empty() {
                content = content.replace(&old_end, &new_end);
            }
            if lang.name == "markdown" {
                content = content.replace(&format!("# {old_name}"), &format!("# {new_name}"));
            }
        }
    }
    let mut plan = Plan::empty();
    let payload = SectionPayload {
        file: file_path.to_string(),
        name: new_path.to_string(),
        old_name: old_path.to_string(),
        ..SectionPayload::default()
    };
    plan.events.push(PlanEvent::new("section.move.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content });
    plan.events.push(PlanEvent::new("section.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🏷️Renamed section \"{old_name}\" to \"{new_name}\" in {file_path}")));
    Ok(plan)
}

/// 🗑️ Removes every line of a section, markers included.
pub fn plan_section_delete(workspace: &Workspace, file_path: &str, section_path: &str) -> Result<Plan, MoveError> {
    let content = require_file(workspace, file_path)?;
    let section_name = last_segment(section_path);
    let Some(lang) = language_for_path(file_path) else {
        return fail(format!("Section not found: {section_name}"));
    };
    let sections = parse_sections_with(lang, content);
    let Some(section) = find_section(&sections, &section_name) else {
        return fail(format!("Section not found: {section_name}"));
    };
    let kept: Vec<&str> = content
        .split('\n')
        .enumerate()
        .filter(|(index, _)| {
            let line_number = (*index as i64) + 1;
            line_number < section.start_line || line_number > section.end_line
        })
        .map(|(_, line)| line)
        .collect();
    let mut plan = Plan::empty();
    let payload = SectionPayload { file: file_path.to_string(), name: section_path.to_string(), ..SectionPayload::default() };
    plan.events.push(PlanEvent::new("section.delete.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content: kept.join("\n") });
    plan.events.push(PlanEvent::new("section.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted section \"{section_name}\" from {file_path}")));
    Ok(plan)
}

//#endregion 📑️Sections
