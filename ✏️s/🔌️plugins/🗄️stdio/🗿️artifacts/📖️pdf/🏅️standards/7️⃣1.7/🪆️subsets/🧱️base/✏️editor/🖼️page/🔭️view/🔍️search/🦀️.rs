//! 🔍️ PDF page search: case-folding text search over supplied page texts with progress, cancellation, hit navigation and highlight layers.
//! [Unicode case folding](https://www.unicode.org/reports/tr44/#Case_Folding)

use super::*;

//#region 🔍️Search
/// 📝 One searchable text run: the page it sits on, its object id and its text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchText {
    pub page: usize,
    pub object: String,
    pub text: String,
}

/// 🎯️ One occurrence: the page object holding it and the character offset inside that object's text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchHit {
    pub page: usize,
    pub object: String,
    pub offset: usize,
}

/// 🎛️ Caller-owned cancellation and progress; progress receives (pages scanned, pages total).
pub struct SearchControl<'a> {
    pub cancelled: &'a dyn Fn() -> bool,
    pub progress: &'a mut dyn FnMut(usize, usize),
}

/// 🏁️ Every hit found, and whether the scan ran to the end.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchOutcome {
    pub hits: Vec<SearchHit>,
    pub completed: bool,
}

fn fold(text: &str, match_case: bool) -> Vec<char> {
    if match_case { text.chars().collect() } else { text.chars().flat_map(char::to_lowercase).collect() }
}

/// 🔎 Hits in reading order (`texts` ordered by page); an empty query matches nothing. Cancellation is checked before every page.
pub fn find_hits(texts: &[SearchText], page_count: usize, query: &str, match_case: bool, control: &mut SearchControl<'_>) -> SearchOutcome {
    let needle = fold(query, match_case);
    let mut hits = Vec::new();
    if needle.is_empty() {
        (control.progress)(page_count, page_count);
        return SearchOutcome { hits, completed: true };
    }
    for page in 0..page_count {
        if (control.cancelled)() {
            return SearchOutcome { hits, completed: false };
        }
        for run in texts.iter().filter(|run| run.page == page) {
            let haystack = fold(&run.text, match_case);
            hits.extend(haystack.windows(needle.len()).enumerate().filter(|(_, window)| *window == needle.as_slice()).map(|(offset, _)| SearchHit { page, object: run.object.clone(), offset }));
        }
        (control.progress)(page + 1, page_count);
    }
    SearchOutcome { hits, completed: true }
}

/// ⏭️ The hit after (or before) `current`, wrapping around; no current hit starts at the first (or last).
pub fn step_hit(count: usize, current: Option<usize>, forward: bool) -> Option<usize> {
    match (count, current, forward) {
        (0, ..) => None,
        (_, None, true) => Some(0),
        (_, None, false) => Some(count - 1),
        (_, Some(at), true) => Some((at + 1) % count),
        (_, Some(at), false) => Some((at + count - 1) % count),
    }
}

/// 🖍️ Translucent highlight layers for every hit whose object has a rectangle `(x, y, width, height)`; the active hit is stronger.
pub fn highlight_hits(hits: &[SearchHit], active: Option<usize>, rect: &dyn Fn(&str) -> Option<[f64; 4]>) -> Vec<OverlayLayer> {
    hits.iter().enumerate().filter_map(|(index, hit)| {
        let [x, y, width, height] = rect(&hit.object)?;
        let color = if active == Some(index) { [1.0, 0.55, 0.0, 0.55] } else { [1.0, 0.9, 0.1, 0.4] };
        Some(json!({ "id": format!("{}.hit{}", hit.object, hit.offset), "segments": rect_segments(x, y, width.max(1.0), height.max(1.0)), "fill": { "color": color } }))
    }).collect()
}
//#endregion 🔍️Search
