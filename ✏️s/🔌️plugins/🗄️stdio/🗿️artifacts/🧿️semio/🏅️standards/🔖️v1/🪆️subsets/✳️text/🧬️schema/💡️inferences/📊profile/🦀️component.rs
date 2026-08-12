//! 📊 `profile` — one named inference: this semio text's own word/mark census plus the distinct
//! BCP-47 `language` tags actually used. This subset owns runs standalone, not nested inside block
//! structure (this subset's own module doc comment) — there is no heading hierarchy the way
//! `document`'s `DocBlock` tree has, so a flat census is the honest structural summary, not an
//! outline. `languages` excludes the unspecified tag (`""`, "inherits from context" per
//! `SemioTextRun`'s own doc comment) — an empty tag names no language.

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Profile
/// 📊️ Semio text word/mark census.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

    fn populated() -> SemioTextSnapshot {
        SemioTextSnapshot {
            schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(),
            runs: vec![
                SemioTextRun { language: "en".into(), content: "Hello, world".into(), marks: vec![] },
                SemioTextRun { language: "en".into(), content: "again".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }] },
                SemioTextRun { language: "de".into(), content: "semio.tech".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() }] },
                SemioTextRun { language: String::new(), content: "unspecified".into(), marks: vec![] },
            ],
        }
    }

    #[test]
    fn censuses_words_chars_marks_and_distinct_languages() {
        let profile = compute_semio_text_profile(&populated());
        assert_eq!(profile.run_count, 4);
        assert_eq!(profile.word_count, 5); // "Hello, world"(2) + "again"(1) + "semio.tech"(1) + "unspecified"(1)
        assert_eq!(profile.mark_count, 2);
        assert_eq!(profile.languages, vec!["de".to_string(), "en".to_string()], "sorted, distinct, unspecified tag excluded");
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = populated();
        assert_eq!(compute_semio_text_profile(&snapshot), compute_semio_text_profile(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_semio_text_profile(&SemioTextSnapshot::default()), SemioTextProfile::default());
    }
}
//#endregion 🧪️Tests
