//! 🧲️ Extract: moving a section into its own file.

use super::{clean_relative, ensure_trailing_newline, extract_imports, extract_package, fail, find_section, format_imports, language_for_path, parse_sections_with, split_header, Change, Message, MoveError, Plan, PlanEvent, Workspace};
use semio_framework_repo_events::ExtractPayload;

//#region 🧲️Extract

/// ⛏️ Lifts one section out of a file into a new file that carries the source's header, package
/// declaration and imports, and removes the section from the source.
pub fn plan_extract(workspace: &Workspace, source_file_path: &str, source_section_name: &str, target_file_path: &str) -> Result<Plan, MoveError> {
    let Some(source_content) = workspace.file(source_file_path) else {
        return fail(format!("Source file not found: {source_file_path}"));
    };
    let Some(lang) = language_for_path(source_file_path) else {
        return fail("Source file type does not support sections");
    };
    if !lang.supports_sections() {
        return fail("Source file type does not support sections");
    }
    let sections = parse_sections_with(lang, source_content);
    let Some(section) = find_section(&sections, source_section_name) else {
        return fail(format!("Section not found: {source_section_name}"));
    };
    let lines: Vec<&str> = source_content.split('\n').collect();
    let total = lines.len() as i64;
    if section.start_line > total || section.end_line > total {
        return fail("Section range invalid");
    }

    let extracted_body = if section.end_line > section.start_line {
        let from = section.start_line.max(0) as usize;
        let to = (section.end_line - 1).max(0) as usize;
        lines[from.min(lines.len())..to.min(lines.len())].join("\n")
    } else {
        String::new()
    };

    let (header, source_body) = split_header(source_content, lang);
    let (package_declaration, source_body_no_package) = extract_package(lang, &source_body);
    let (imports, _) = extract_imports(lang, &source_body_no_package);

    let mut target_content = String::new();
    if !header.is_empty() {
        target_content.push_str(&header);
        target_content.push_str("\n\n");
    }
    if !package_declaration.is_empty() {
        target_content.push_str(&package_declaration);
        target_content.push_str("\n\n");
    }
    if !imports.is_empty() {
        let formatted = format_imports(lang, &imports);
        if !formatted.is_empty() {
            target_content.push_str(&formatted);
            target_content.push_str("\n\n");
        }
    }
    target_content.push_str(&extracted_body);
    ensure_trailing_newline(&mut target_content);
    if target_content.is_empty() {
        target_content.push('\n');
    }

    let mut remaining: Vec<&str> = Vec::with_capacity(lines.len());
    if section.start_line > 0 {
        remaining.extend_from_slice(&lines[..((section.start_line - 1).max(0) as usize).min(lines.len())]);
    }
    if section.end_line < total {
        remaining.extend_from_slice(&lines[(section.end_line.max(0) as usize).min(lines.len())..]);
    }

    let mut plan = Plan::empty();
    let payload = ExtractPayload {
        source_file: source_file_path.to_string(),
        source_section: source_section_name.to_string(),
        target_file: target_file_path.to_string(),
        ..ExtractPayload::default()
    };
    plan.events.push(PlanEvent::new("extract.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(target_file_path), content: target_content });
    plan.changes.push(Change::Write { path: clean_relative(source_file_path), content: remaining.join("\n") });
    plan.events.push(PlanEvent::new("extract.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🧩️Extracted {source_section_name} from {source_file_path} to {target_file_path}")));
    Ok(plan)
}

//#endregion 🧲️Extract
