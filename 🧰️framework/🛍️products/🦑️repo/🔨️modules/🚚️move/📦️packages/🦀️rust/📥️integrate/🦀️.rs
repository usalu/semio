//! 📥️ Integrate: moving a file into a section.

use super::{
    clean_relative, ensure_trailing_newline, extract_imports, extract_package, fail, find_section, format_imports, language_for_path, merge_headers, parse_sections_with, split_header, unique_strings, Change, Message, MoveError, Plan, PlanEvent,
    Workspace,
};
use semio_framework_repo_events::IntegratePayload;
use semio_framework_repo_languages::{format_section_end, format_section_start};

//#region 📥️Integrate

/// 🧬️ Folds a whole source file into a named section of a target file, merging the two headers and
/// the two import lists and keeping the target's package declaration.
pub fn plan_integrate(
    workspace: &Workspace,
    source_path: &str,
    target_section_name: &str,
    target_file_path: &str,
    target_parent_section_name: &str,
) -> Result<Plan, MoveError> {
    let Some(source_content) = workspace.file(source_path) else {
        return fail(format!("Source file not found: {source_path}"));
    };
    let Some(target_content) = workspace.file(target_file_path) else {
        return fail(format!("Target file not found: {target_file_path}"));
    };
    let Some(lang) = language_for_path(target_file_path) else {
        return fail("Target file type does not support sections");
    };
    if !lang.supports_sections() {
        return fail("Target file type does not support sections");
    }

    let mut plan = Plan::empty();
    let payload = IntegratePayload {
        source: source_path.to_string(),
        target_file: target_file_path.to_string(),
        target_section: target_section_name.to_string(),
        ..IntegratePayload::default()
    };
    plan.events.push(PlanEvent::new("integrate.starting", &payload));
    plan.messages.push(Message::Info("Splitting headers...".to_string()));

    let (source_header, source_body) = split_header(source_content, lang);
    let (target_header, target_body) = split_header(target_content, lang);
    plan.messages.push(Message::Info(format!("Source Header len: {}, Source Body len: {}", source_header.len(), source_body.len())));

    let (_, source_body_no_package) = extract_package(lang, &source_body);
    let (target_package, target_body_no_package) = extract_package(lang, &target_body);
    let (source_imports, source_code) = extract_imports(lang, &source_body_no_package);
    let (target_imports, target_code) = extract_imports(lang, &target_body_no_package);

    let merged_header = merge_headers(&target_header, &source_header, lang);
    let mut merged_imports = target_imports;
    merged_imports.extend(source_imports);
    let merged_imports = unique_strings(&merged_imports);

    let start_marker = format_section_start(lang, target_section_name);
    let end_marker = format_section_end(lang, target_section_name);
    let mut source_code = source_code;
    ensure_trailing_newline(&mut source_code);
    let section_content = format!("\n{start_marker}\n{source_code}{end_marker}\n");

    let updated_body = if target_parent_section_name.is_empty() {
        let mut body = target_code;
        ensure_trailing_newline(&mut body);
        body.push_str(&section_content);
        body
    } else {
        let sections = parse_sections_with(lang, &target_code);
        let Some(parent) = find_section(&sections, target_parent_section_name) else {
            return fail(format!("Parent section not found: {target_parent_section_name}"));
        };
        if parent.end_line == -1 {
            return fail(format!("Parent section {target_parent_section_name} is not properly closed"));
        }
        let lines: Vec<&str> = target_code.split('\n').collect();
        let cut = (parent.end_line - 1).max(0) as usize;
        let cut = cut.min(lines.len());
        let inserted: Vec<&str> = section_content.trim_matches('\n').split('\n').collect();
        let mut new_lines: Vec<&str> = Vec::with_capacity(lines.len() + inserted.len());
        new_lines.extend_from_slice(&lines[..cut]);
        new_lines.extend_from_slice(&inserted);
        new_lines.extend_from_slice(&lines[cut..]);
        new_lines.join("\n")
    };

    let mut final_content = merged_header;
    if !final_content.is_empty() {
        ensure_trailing_newline(&mut final_content);
        final_content.push('\n');
    }
    if !target_package.is_empty() {
        final_content.push_str(&target_package);
        final_content.push_str("\n\n");
    }
    let formatted_imports = format_imports(lang, &merged_imports);
    if !formatted_imports.is_empty() {
        final_content.push_str(&formatted_imports);
        final_content.push_str("\n\n");
    }
    final_content.push_str(&updated_body);

    plan.changes.push(Change::Write { path: clean_relative(target_file_path), content: final_content });
    plan.events.push(PlanEvent::new("integrate.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🧩️Integrated {source_path} into {target_section_name} section of {target_file_path}")));
    Ok(plan)
}

//#endregion 📥️Integrate
