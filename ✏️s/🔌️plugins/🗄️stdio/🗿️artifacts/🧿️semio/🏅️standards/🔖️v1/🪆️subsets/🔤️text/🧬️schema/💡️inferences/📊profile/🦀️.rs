//! 📊 `profile` — one named inference: this semio text's own word/mark census plus the distinct
//! BCP-47 `language` tags actually used. This subset owns runs standalone, not nested inside block
//! structure (this subset's own module doc comment) — there is no heading hierarchy the way
//! `document`'s `DocBlock` tree has, so a flat census is the honest structural summary, not an
//! outline. `languages` excludes the unspecified tag (`""`, "inherits from context" per
//! `SemioTextRun`'s own doc comment) — an empty tag names no language.

use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Profile
/// 📊️ Semio text word/mark census.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTextProfile {
    pub word_count: u32,
    pub char_count: u32,
    pub run_count: u32,
    pub mark_count: u32,
    /// 🌐️ Distinct non-empty `language` tags, sorted for determinism (source order carries no
    /// meaning for a set of tags).
    pub languages: Vec<String>,
}

/// 📊️ Computes [`SemioTextProfile`] — pure, total, O(runs + marks).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_text_profile(snapshot: &SemioTextSnapshot) -> SemioTextProfile {
    let mut word_count = 0u32;
    let mut char_count = 0u32;
    let mut mark_count = 0u32;
    let mut languages: Vec<String> = Vec::new();
    for run in &snapshot.runs {
        word_count += run.content.split_whitespace().count() as u32;
        char_count += run.content.chars().count() as u32;
        mark_count += run.marks.len() as u32;
        if !run.language.is_empty() && !languages.contains(&run.language) {
            languages.push(run.language.clone());
        }
    }
    languages.sort();
    SemioTextProfile { word_count, char_count, run_count: snapshot.runs.len() as u32, mark_count, languages }
}
//#endregion 🔖️Profile

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
