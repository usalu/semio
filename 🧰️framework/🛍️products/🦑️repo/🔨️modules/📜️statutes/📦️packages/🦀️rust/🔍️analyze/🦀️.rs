//! 🔍️ Analysis: header, section, orphan, definition and requirement policies over a source set.

use super::{contributor_line, create_breach, filter_ignored, has_implementation_syntax, is_spec_text, is_test_or_benchmark_file, policy_section_end_match, policy_section_start_match, requires_definition_requirements, Breach, SourceSet, Statute};
use semio_framework_repo_languages as languages;
use std::collections::BTreeMap;

//#region 🔍️Analyze

const AGPL_MARKERS: [&str; 3] = ["GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"];

/// 🔍️ Every breach the ported policies detect over the given sources, ignore directives applied.
pub fn analyze(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    breachs.extend(header_policy(sources));
    breachs.extend(section_policy(sources));
    breachs.extend(requirements_policy(sources));
    breachs
}

fn find_header(sections: &[languages::Section]) -> Option<&languages::Section> {
    sections.iter().find(|s| s.name.to_lowercase() == "header")
}

/// 🧲️ The header-region breaches of every source: missing region, contributors, license and summary.
pub fn header_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_headers {
            continue;
        }
        let sections = languages::parse_sections(content, file);
        let Some(header) = find_header(&sections) else {
            breachs.push(create_breach(&format!("Missing header region in {file}"), Statute::from("code/file/missing-header-region"), file, 0, 0, ""));
            continue;
        };
        let header_content = &content[header.start_index as usize..header.end_index as usize];
        let header_lines: Vec<&str> = header_content.split('\n').collect();
        let has_contributors = header_lines.iter().any(|line| contributor_line(line));
        if !has_contributors {
            breachs.push(create_breach(
                &format!("Missing contributors in header of {file}"),
                Statute::from("code/file/missing-contributors"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        }
        let has_license = AGPL_MARKERS.iter().any(|marker| header_content.contains(marker));
        if !has_license {
            breachs.push(create_breach(
                &format!("Missing license in header of {file}"),
                Statute::from("code/file/missing-license"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        } else {
            let mut wrong = ["MIT", "Apache", "BSD"].iter().any(|name| header_content.contains(name));
            if header_content.contains("GPL") && !header_content.contains("AGPL") && !header_content.contains("LGPL") {
                wrong = true;
            }
            if wrong {
                breachs.push(create_breach(
                    &format!("Wrong license in header of {file}"),
                    Statute::from("code/file/wrong-license"),
                    &format!("{file}#Header"),
                    header.start_line,
                    0,
                    "",
                ));
            }
        }
        let prefix = language.comment_prefix.as_str();
        let mut has_summary = false;
        let mut seen_license = false;
        let mut license_end = false;
        for line in &header_lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if seen_license && !license_end {
                    license_end = true;
                }
                continue;
            }
            if trimmed.starts_with(&format!("{prefix} #region")) || trimmed.starts_with(&format!("{prefix} #endregion")) {
                continue;
            }
            let Some(rest) = trimmed.strip_prefix(prefix) else { continue };
            let comment_text = rest.trim();
            if comment_text.is_empty() {
                if seen_license && !license_end {
                    license_end = true;
                }
                continue;
            }
            if contributor_line(line) {
                continue;
            }
            let is_license_line = AGPL_MARKERS.iter().any(|marker| line.contains(marker));
            if is_license_line {
                seen_license = true;
                continue;
            }
            let license_body = ["without", "program", "License", "http", "version", "terms", "WARRANTY", "PURPOSE", "General Public", "redistribute", "published", "Foundation", "received"];
            if !license_end && seen_license && license_body.iter().any(|needle| comment_text.contains(needle)) {
                continue;
            }
            if is_spec_text(comment_text) {
                continue;
            }
            if comment_text.starts_with("TODO:") {
                continue;
            }
            has_summary = true;
            break;
        }
        if !has_summary && !is_test_or_benchmark_file(file) {
            breachs.push(create_breach(
                &format!("Missing summary in header of {file}"),
                Statute::from("code/file/missing-summary"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        }
    }
    filter_ignored(breachs, sources)
}

struct SectionFrame {
    name: String,
    line: i64,
}

/// 🧩️ The section-structure breaches of every source: markers, emptiness, summaries and orphans.
pub fn section_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_sections() {
            continue;
        }
        let lines: Vec<&str> = content.split('\n').collect();
        let mut stack: Vec<SectionFrame> = Vec::new();
        for (index, raw) in lines.iter().enumerate() {
            let line_number = index as i64 + 1;
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if let Some(name) = policy_section_start_match(language, line) {
                if name.is_empty() {
                    breachs.push(create_breach(
                        &format!("Missing section name at {file}:{line_number}"),
                        Statute::from("code/section/missing-start-name"),
                        file,
                        line_number,
                        0,
                        line.trim(),
                    ));
                }
                stack.push(SectionFrame { name, line: line_number });
                continue;
            }
            if let Some(end_name) = policy_section_end_match(language, line) {
                if let Some(open) = stack.pop() {
                    if !open.name.is_empty() {
                        if end_name.is_empty() {
                            breachs.push(create_breach(
                                &format!("Missing end section name at {file}:{line_number}"),
                                Statute::from("code/section/missing-end-name"),
                                file,
                                line_number,
                                0,
                                line.trim(),
                            ));
                        } else if end_name != open.name {
                            breachs.push(create_breach(
                                &format!("Section name mismatch at {file}:{line_number}"),
                                Statute::from("code/section/name-mismatch"),
                                file,
                                line_number,
                                0,
                                &format!("Start: \"{}\" at line {}, End: \"{}\"", open.name, open.line, end_name),
                            ));
                        }
                    }
                }
            }
        }
        let sections = languages::parse_sections(content, file);
        let prefix = language.comment_prefix.as_str();
        let mut section_breachs = Vec::new();
        for section in &sections {
            check_section(section, "", file, content, &lines, language, prefix, &mut section_breachs);
        }
        breachs.append(&mut section_breachs);
        breachs.extend(orphan_breachs(file, &lines, &sections, language, prefix));
        breachs.extend(definition_breachs(file, content, &lines, language, prefix));
    }
    filter_ignored(breachs, sources)
}

#[allow(clippy::too_many_arguments)]
fn check_section(
    section: &languages::Section,
    parent_path: &str,
    file: &str,
    content: &str,
    lines: &[&str],
    language: &languages::Language,
    prefix: &str,
    breachs: &mut Vec<Breach>,
) {
    let section_path = if parent_path.is_empty() { section.name.clone() } else { format!("{parent_path}#{}", section.name) };
    let section_content = &content[section.start_index as usize..section.end_index as usize];
    let section_lines: Vec<&str> = section_content.split('\n').collect();
    let mut non_empty = 0;
    if section_lines.len() >= 2 {
        for line in &section_lines[1..section_lines.len() - 1] {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with('#') {
                non_empty += 1;
            }
        }
    }
    let is_exempt = section.name == "Header";
    if non_empty == 0 && section.children.is_empty() && !is_exempt {
        breachs.push(create_breach(
            &format!("Empty section \"{}\" in {file}", section.name),
            Statute::from("code/section/empty"),
            &format!("{file}#{section_path}"),
            section.start_line,
            0,
            "",
        ));
    }
    if (section.start_line as usize) < lines.len() && lines[section.start_line as usize].trim().is_empty() {
        breachs.push(create_breach(
            &format!("Blank line after region start marker in section \"{}\" in {file}:{}", section.name, section.start_line + 1),
            Statute::from("code/section/wrong-format/newline-after-region"),
            &format!("{file}#{section_path}"),
            section.start_line + 1,
            0,
            "",
        ));
    }
    if !is_exempt && !section.name.is_empty() && !is_test_or_benchmark_file(file) {
        let mut has_summary = false;
        for line in section_lines.iter().take(section_lines.len().saturating_sub(1)).skip(1) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if policy_section_start_match(language, trimmed).is_some() || policy_section_end_match(language, trimmed).is_some() {
                break;
            }
            let Some(rest) = trimmed.strip_prefix(prefix) else { break };
            if rest.trim().is_empty() {
                continue;
            }
            has_summary = true;
        }
        if !has_summary {
            breachs.push(create_breach(
                &format!("Section \"{}\" is missing a summary comment in {file}", section.name),
                Statute::from("code/section/missing-summary"),
                &format!("{file}#{}", section.name),
                section.start_line,
                0,
                "",
            ));
        }
    }
    for child in &section.children {
        check_section(child, &section_path, file, content, lines, language, prefix, breachs);
    }
}

fn mark_covered(section: &languages::Section, covered: &mut [bool]) {
    let start = section.start_line.max(1);
    let mut end = section.end_line.max(start);
    if end > covered.len() as i64 {
        end = covered.len() as i64;
    }
    for line in start..=end {
        if line >= 1 {
            covered[line as usize - 1] = true;
        }
    }
    for child in &section.children {
        mark_covered(child, covered);
    }
}

struct OrphanRange {
    start: i64,
    end: i64,
    first_line: String,
    is_comment_block: bool,
}

fn orphan_breachs(file: &str, lines: &[&str], sections: &[languages::Section], language: &languages::Language, prefix: &str) -> Vec<Breach> {
    let mut breachs = Vec::new();
    if is_test_or_benchmark_file(file) {
        return breachs;
    }
    let mut covered = vec![false; lines.len()];
    for section in sections {
        mark_covered(section, &mut covered);
    }
    let mut orphan_lines = vec![false; lines.len()];
    for (index, raw) in lines.iter().enumerate() {
        if covered[index] {
            continue;
        }
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if line.trim().is_empty() {
            continue;
        }
        if index == 0 && line.trim().starts_with("#!") {
            continue;
        }
        if policy_section_start_match(language, line).is_some() || policy_section_end_match(language, line).is_some() {
            continue;
        }
        orphan_lines[index] = true;
    }
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut in_orphan = false;
    let mut start_line = 0i64;
    for (index, flag) in orphan_lines.iter().enumerate() {
        if *flag {
            if !in_orphan {
                in_orphan = true;
                start_line = index as i64 + 1;
            }
        } else if in_orphan {
            ranges.push((start_line, index as i64));
            in_orphan = false;
        }
    }
    if in_orphan {
        ranges.push((start_line, lines.len() as i64));
    }
    let mut def_ranges: Vec<(String, i64, i64)> = Vec::new();
    let mut excerpts: BTreeMap<String, String> = BTreeMap::new();
    if language.supports_definitions() {
        for def in languages::parse_definition_ranges(language, lines) {
            def_ranges.push((def.name.clone(), def.start, def.end));
            excerpts.insert(def.name, def.excerpt);
        }
    }
    for def in languages::extra_orphan_definitions(language, lines) {
        def_ranges.push((def.name.clone(), def.start, def.end));
        excerpts.insert(def.name, def.excerpt);
    }
    let mut infos = Vec::new();
    for (start, end) in ranges {
        let mut first_line = String::new();
        let mut is_comment_block = true;
        for line_index in start..=end {
            let raw = lines[line_index as usize - 1];
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if line.trim().is_empty() {
                continue;
            }
            if first_line.is_empty() {
                first_line = line.trim().to_string();
            }
            if !line.trim().starts_with(prefix) {
                is_comment_block = false;
            }
        }
        if is_comment_block {
            let name = format!("comment-block-{start}");
            def_ranges.push((name.clone(), start, end));
            excerpts.insert(name, first_line.clone());
        }
        infos.push(OrphanRange { start, end, first_line, is_comment_block });
    }
    let mut reported: BTreeMap<String, bool> = BTreeMap::new();
    for info in &infos {
        let _ = info.is_comment_block;
        let mut matched = false;
        for (name, start, end) in &def_ranges {
            if info.start <= *end && info.end >= *start {
                if !reported.contains_key(name) {
                    reported.insert(name.clone(), true);
                    let excerpt = excerpts.get(name).filter(|v| !v.is_empty()).cloned().unwrap_or_else(|| name.clone());
                    breachs.push(create_breach(
                        &format!("Orphan definition outside sections at {file}:{start}"),
                        Statute::from("code/section/orphan-definition"),
                        &format!("{file}::{name}"),
                        *start,
                        0,
                        &excerpt,
                    ));
                }
                matched = true;
            }
        }
        if matched {
            continue;
        }
        let name = format!("orphan-block-{}", info.start);
        breachs.push(create_breach(
            &format!("Orphan definition outside sections at {file}:{}", info.start),
            Statute::from("code/section/orphan-definition"),
            &format!("{file}::{name}"),
            info.start,
            0,
            &info.first_line,
        ));
    }
    breachs
}

struct DocState {
    has_summary: bool,
    has_requirements: bool,
    is_native: bool,
}

fn scan_jsdoc(lines: &[&str], prev_index: i64) -> Option<DocState> {
    let prev = lines[prev_index as usize].trim();
    if !(prev.ends_with("**/") || prev.ends_with("*/")) {
        return None;
    }
    let mut state = DocState { has_summary: false, has_requirements: false, is_native: true };
    let mut scan = prev_index;
    while scan >= 0 {
        let line = lines[scan as usize].trim();
        let is_open = line.starts_with("/**");
        let mut text = line.to_string();
        if is_open {
            text = text.trim_start_matches("/**").to_string();
        } else if let Some(rest) = text.strip_prefix("* ") {
            text = rest.to_string();
        } else if text == "*" || text == "*/" || text == "**/" {
            scan -= 1;
            continue;
        }
        let mut text = text.trim().to_string();
        if text.ends_with("**/") || text.ends_with("*/") {
            text = text.trim_end_matches("**/").trim_end_matches("*/").trim().to_string();
        }
        if text.is_empty() {
            if is_open {
                break;
            }
            scan -= 1;
            continue;
        }
        if let Some(rest) = text.strip_prefix("* ") {
            text = rest.trim().to_string();
        }
        if is_spec_text(&text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        if is_open {
            break;
        }
        scan -= 1;
    }
    Some(state)
}

fn scan_triple_slash(lines: &[&str], prev_index: i64) -> Option<DocState> {
    if !lines[prev_index as usize].trim().starts_with("///") {
        return None;
    }
    let mut state = DocState { has_summary: false, has_requirements: false, is_native: true };
    let mut scan = prev_index;
    while scan >= 0 {
        let line = lines[scan as usize].trim();
        let Some(rest) = line.strip_prefix("///") else { break };
        let mut text = rest.trim().to_string();
        if text.is_empty() {
            scan -= 1;
            continue;
        }
        text = text.trim_start_matches("<summary>").trim_end_matches("</summary>").trim().to_string();
        if text == "<remarks>" || text == "</remarks>" || text.is_empty() {
            scan -= 1;
            continue;
        }
        if is_spec_text(&text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        scan -= 1;
    }
    Some(state)
}

fn scan_leading_comments(lines: &[&str], start: i64, prefix: &str, state: &mut DocState) {
    let mut index = start - 2;
    while index >= 0 {
        let line = lines[index as usize].trim();
        if line.is_empty() {
            break;
        }
        let Some(rest) = line.strip_prefix(prefix) else { break };
        let text = rest.trim();
        if text.is_empty() {
            index -= 1;
            continue;
        }
        if is_spec_text(text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        index -= 1;
    }
}

fn definition_breachs(file: &str, content: &str, lines: &[&str], language: &languages::Language, prefix: &str) -> Vec<Breach> {
    let mut breachs = Vec::new();
    if is_test_or_benchmark_file(file) || !language.supports_definitions() {
        return breachs;
    }
    let _ = content;
    let lang_name = language.name.as_str();
    for def in languages::parse_definition_ranges(language, lines) {
        let def_line = if def.start >= 1 && (def.start as usize) <= lines.len() { lines[def.start as usize - 1] } else { "" };
        let prev_index = def.start - 2;
        let mut state = DocState { has_summary: false, has_requirements: false, is_native: false };
        if prev_index >= 0 {
            let scanned = match lang_name {
                "typescript" => scan_jsdoc(lines, prev_index),
                "csharp" | "rust" => scan_triple_slash(lines, prev_index),
                _ => None,
            };
            if let Some(scanned) = scanned {
                state = scanned;
            }
        }
        if !state.is_native && lang_name == "python" {
            scan_python_docstring(lines, def.start, &mut state);
            if state.is_native {
                let mut index = def.start - 2;
                while index >= 0 {
                    let line = lines[index as usize].trim();
                    if line.is_empty() {
                        break;
                    }
                    let Some(rest) = line.strip_prefix(prefix) else { break };
                    if !rest.trim().is_empty() {
                        state.is_native = false;
                        break;
                    }
                    index -= 1;
                }
            }
        }
        if !state.is_native {
            if lang_name == "go" {
                state.is_native = true;
            }
            scan_leading_comments(lines, def.start, prefix, &mut state);
        }
        if !state.is_native && (state.has_summary || state.has_requirements) {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is not using native docstring format in {file}:{}", def.name, def.start),
                Statute::from("code/definition/wrong-format/not-native-docstring"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
        if !state.has_summary {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is missing a summary comment in {file}:{}", def.name, def.start),
                Statute::from("code/definition/missing-summary"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
        if !state.has_requirements && requires_definition_requirements(def_line, lang_name) {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is missing spec comments in {file}:{}", def.name, def.start),
                Statute::from("code/definition/missing-requirements"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
    }
    breachs
}

fn scan_python_docstring(lines: &[&str], start: i64, state: &mut DocState) {
    let raw = lines[start as usize - 1];
    let mut depth = 0i64;
    for ch in raw.chars() {
        if ch == '(' {
            depth += 1;
        }
        if ch == ')' {
            depth -= 1;
        }
    }
    let mut body_start = start;
    if depth > 0 {
        let mut scan = start;
        while scan < lines.len() as i64 && scan < start + 15 {
            for ch in lines[scan as usize].chars() {
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }
            if depth <= 0 {
                body_start = scan + 1;
                break;
            }
            scan += 1;
        }
    }
    let mut body = body_start;
    while body < lines.len() as i64 && body < body_start + 5 {
        let trimmed = lines[body as usize].trim();
        if trimmed.is_empty() {
            body += 1;
            continue;
        }
        if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
            state.is_native = true;
            let quote = if trimmed.starts_with("'''") { "'''" } else { "\"\"\"" };
            let after_open = trimmed.strip_prefix(quote).unwrap_or(trimmed);
            if let Some(close) = after_open.find(quote) {
                let text = after_open[..close].trim();
                if !text.is_empty() {
                    if is_spec_text(text) {
                        state.has_requirements = true;
                    } else {
                        state.has_summary = true;
                    }
                }
            } else {
                let first = after_open.trim();
                if !first.is_empty() {
                    if is_spec_text(first) {
                        state.has_requirements = true;
                    } else {
                        state.has_summary = true;
                    }
                }
                let mut scan = body + 1;
                while scan < lines.len() as i64 {
                    let line = lines[scan as usize].trim();
                    if line == quote {
                        break;
                    }
                    if line.ends_with(quote) {
                        let text = line.trim_end_matches(quote).trim();
                        if !text.is_empty() {
                            if is_spec_text(text) {
                                state.has_requirements = true;
                            } else {
                                state.has_summary = true;
                            }
                        }
                        break;
                    }
                    if !line.is_empty() {
                        if is_spec_text(line) {
                            state.has_requirements = true;
                        } else {
                            state.has_summary = true;
                        }
                    }
                    scan += 1;
                }
            }
        }
        break;
    }
}

/// 📐️ The implementation-syntax breaches of every specification comment of every source.
pub fn requirements_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_headers {
            continue;
        }
        let lines: Vec<&str> = content.split('\n').collect();
        let sections = languages::parse_sections(content, file);
        let prefix = language.comment_prefix.as_str();
        if let Some(header) = find_header(&sections) {
            let requirements = header.children.iter().find(|c| c.name.to_lowercase() == "requirements");
            match requirements {
                Some(child) => {
                    let mut index = child.start_line + 1;
                    while index < child.end_line && index <= lines.len() as i64 {
                        let line = lines[index as usize - 1].trim();
                        if line.is_empty() {
                            index += 1;
                            continue;
                        }
                        let text = line.strip_prefix(prefix).unwrap_or(line).trim();
                        if text.is_empty() {
                            index += 1;
                            continue;
                        }
                        if let Some(reason) = has_implementation_syntax(text) {
                            breachs.push(create_breach(
                                &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                                Statute::from("code/requirements/implementation-syntax"),
                                &format!("{file}#Header/Requirements"),
                                index,
                                0,
                                text,
                            ));
                        }
                        index += 1;
                    }
                }
                None => {
                    let mut index = header.start_line + 1;
                    while index < header.end_line && index <= lines.len() as i64 {
                        let line = lines[index as usize - 1].trim();
                        if line.is_empty() {
                            index += 1;
                            continue;
                        }
                        if let Some(rest) = line.strip_prefix(prefix) {
                            let text = rest.trim();
                            if !text.is_empty() && is_spec_text(text) {
                                if let Some(reason) = has_implementation_syntax(text) {
                                    breachs.push(create_breach(
                                        &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                                        Statute::from("code/requirements/implementation-syntax"),
                                        &format!("{file}#Header"),
                                        index,
                                        0,
                                        text,
                                    ));
                                }
                            }
                        }
                        index += 1;
                    }
                }
            }
        }
        for section in &sections {
            check_section_requirements(section, file, &lines, prefix, &mut breachs);
        }
    }
    filter_ignored(breachs, sources)
}

fn check_section_requirements(section: &languages::Section, file: &str, lines: &[&str], prefix: &str, breachs: &mut Vec<Breach>) {
    if section.name.to_lowercase() == "header" {
        return;
    }
    let mut index = section.start_line + 1;
    while index < section.end_line && index <= lines.len() as i64 {
        let line = lines[index as usize - 1].trim();
        if line.is_empty() {
            index += 1;
            continue;
        }
        let Some(rest) = line.strip_prefix(prefix) else { break };
        let text = rest.trim();
        if !is_spec_text(text) {
            break;
        }
        if let Some(reason) = has_implementation_syntax(text) {
            breachs.push(create_breach(
                &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                Statute::from("code/requirements/implementation-syntax"),
                &format!("{file}#{}", section.name),
                index,
                0,
                text,
            ));
        }
        index += 1;
    }
    for child in &section.children {
        check_section_requirements(child, file, lines, prefix, breachs);
    }
}

//#endregion 🔍️Analyze
