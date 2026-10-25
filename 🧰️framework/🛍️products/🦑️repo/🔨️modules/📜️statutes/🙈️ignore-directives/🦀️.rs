//! 🙈️ Ignore directives: parsing `// compose-ignore-` lines and filtering the breaches they suppress.

use super::{Breach, SourceSet, Statute};
use std::collections::BTreeMap;

//#region 🙈️Ignore Directives

const IGNORE_PREFIX: &str = "// compose-ignore-";
const IGNORE_WINDOW: i64 = 100;

/// 📖️ The statute prefixes each `// compose-ignore-` line suppresses, keyed by one-based line.
pub fn parse_ignore_directives(content: &str) -> BTreeMap<i64, Vec<String>> {
    let mut result: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (index, line) in content.split('\n').enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(IGNORE_PREFIX) {
            for part in rest.split(',') {
                let pattern = part.trim();
                if !pattern.is_empty() {
                    result.entry(index as i64 + 1).or_default().push(pattern.to_string());
                }
            }
        }
    }
    result
}

/// 🚫️ Whether a breach on `breach_line` is suppressed by a directive of the file.
pub fn is_ignored(directives: &BTreeMap<i64, Vec<String>>, breach_line: i64, kind: &Statute) -> bool {
    directives.iter().any(|(ignore_line, patterns)| {
        breach_line > *ignore_line
            && breach_line <= ignore_line + IGNORE_WINDOW
            && patterns.iter().any(|pattern| kind.0.starts_with(pattern))
    })
}

/// 📄️ The file part of a breach scope, dropping the section and definition suffixes.
pub fn extract_file_from_scope(scope: &str) -> String {
    let mut value = scope;
    if let Some(index) = value.find('#') {
        value = &value[..index];
    }
    if let Some(index) = value.find("::") {
        value = &value[..index];
    }
    value.to_string()
}

/// 🧹️ The breaches that survive the ignore directives of the files they point at.
pub fn filter_ignored(breachs: Vec<Breach>, sources: &SourceSet) -> Vec<Breach> {
    breachs
        .into_iter()
        .filter(|breach| {
            let file = extract_file_from_scope(&breach.scope);
            match sources.directives(&file) {
                Some(directives) => !is_ignored(directives, breach.line, &breach.kind),
                None => true,
            }
        })
        .collect()
}

//#endregion 🙈️Ignore Directives
