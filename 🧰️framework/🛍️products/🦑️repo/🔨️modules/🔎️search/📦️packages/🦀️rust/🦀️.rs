//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🔎️ Repository search: a deterministic, cancellable, append-replayed text index.
//! Behaviour twin of `github.com/usalu/semio/repo/search` — the same event schema, the same
//! bounds, the same bounded edit distance and the same ranking and truncation rules.

//#endregion 🧲️Header

use std::collections::BTreeMap;

//#region 📜️Schema

/// 🏷️ The event schema every persisted index line carries.
pub const SCHEMA: &str = "semio.search.event/1";

/// 📏️ The largest indexable document, in bytes.
pub const MAX_DOCUMENT_BYTES: usize = 1 << 20;

/// 📏️ The largest number of live documents.
pub const MAX_DOCUMENTS: usize = 250_000;

/// 📏️ The largest number of traversed filesystem nodes.
pub const MAX_TRAVERSAL_NODES: usize = 500_000;

/// 📏️ The largest number of events buffered before a commit.
pub const MAX_PENDING_EVENTS: usize = 500_000;

/// 📏️ The largest persisted index, in bytes.
pub const MAX_INDEX_BYTES: u64 = 256 << 20;

/// 📏️ The largest query, in bytes.
pub const MAX_QUERY_BYTES: usize = 4 << 10;

/// 📏️ The largest number of terms a query may carry.
pub const MAX_QUERY_TERMS: usize = 32;

/// ❌️ Everything that can go wrong inside the index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    /// 🧨️ The persisted event log does not replay.
    Corrupt(String),
    /// 🚧️ A declared bound was exceeded.
    TooLarge(String),
    /// 🚫️ The request itself is not usable.
    Invalid(String),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Corrupt(detail) => write!(formatter, "corrupt index: {detail}"),
            SearchError::TooLarge(detail) => write!(formatter, "index limit exceeded: {detail}"),
            SearchError::Invalid(detail) => write!(formatter, "{detail}"),
        }
    }
}

/// 🔤️ One term of a query together with its edit-distance tolerance.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MatchQuery {
    /// 🔤️ The literal term.
    pub term: String,
    /// 🎚️ The maximum edit distance a word may be away from the term.
    pub fuzziness: usize,
}

impl MatchQuery {
    /// 🆕️ Returns an exact-match term query.
    pub fn new(term: &str) -> Self {
        MatchQuery { term: term.to_string(), fuzziness: 0 }
    }

    /// 🎚️ Returns the same term with an edit-distance tolerance.
    pub fn with_fuzziness(mut self, fuzziness: usize) -> Self {
        self.fuzziness = fuzziness;
        self
    }
}

/// 🧾️ One ranked hit.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchHit {
    /// 🆔️ The document identifier.
    pub id: String,
    /// 🏅️ The accumulated score.
    pub score: f64,
}

/// 📊️ A ranked result page and the number of hits before truncation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    /// 🔢️ The number of matching documents before the size limit is applied.
    pub total: u64,
    /// 🧾️ The retained hits, best first.
    pub hits: Vec<SearchHit>,
}

/// 📨️ One persisted index event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    /// 🏷️ The event schema.
    pub schema: String,
    /// 🔢️ The one-based sequence number.
    pub seq: u64,
    /// 🧭️ Either `indexed` or `deleted`.
    pub kind: String,
    /// 🆔️ The document identifier.
    pub id: String,
    /// 📝️ The indexed text, absent for deletions.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

//#endregion 📜️Schema

//#region 📝️AppendReplay

/// 🗄️ An append-replayed text index.
#[derive(Debug, Clone, Default)]
pub struct Index {
    docs: BTreeMap<String, String>,
    seq: u64,
    pending: Vec<Event>,
    closed: bool,
}

impl Index {
    /// 🆕️ Returns an empty in-memory index.
    pub fn new() -> Self {
        Index::default()
    }

    /// ♻️ Replays a persisted event log, rejecting gaps, duplicates and foreign schemas.
    pub fn replay(log: &str) -> Result<Self, SearchError> {
        let mut index = Index::new();
        let mut expected = 1u64;
        for line in log.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            if line.len() > MAX_DOCUMENT_BYTES * 2 {
                return Err(SearchError::Corrupt(format!("event {expected} too large")));
            }
            let event: Event = serde_json::from_str(line)
                .map_err(|error| SearchError::Corrupt(format!("event {expected}: {error}")))?;
            if event.schema != SCHEMA || event.seq != expected {
                return Err(SearchError::Corrupt(format!("sequence got {}, want {expected}", event.seq)));
            }
            index.apply(&event)?;
            index.seq = event.seq;
            expected += 1;
        }
        Ok(index)
    }

    /// 📝️ Records a document under an identifier.
    pub fn index(&mut self, id: &str, text: &str) -> Result<(), SearchError> {
        let trimmed = text.trim().to_string();
        if trimmed.len() > MAX_DOCUMENT_BYTES {
            return Err(SearchError::TooLarge(format!(
                "document {id:?} has {} bytes, maximum {MAX_DOCUMENT_BYTES}",
                trimmed.len()
            )));
        }
        self.append("indexed", id, &trimmed)
    }

    /// 🗑️ Removes a document.
    pub fn delete(&mut self, id: &str) -> Result<(), SearchError> {
        self.append("deleted", id, "")
    }

    /// ➕️ Applies an event and buffers it for the next commit.
    fn append(&mut self, kind: &str, id: &str, text: &str) -> Result<(), SearchError> {
        if self.closed {
            return Err(SearchError::Invalid("index closed".to_string()));
        }
        if id.is_empty() {
            return Err(SearchError::Invalid("index event has empty id".to_string()));
        }
        if kind == "indexed" && !self.docs.contains_key(id) && self.docs.len() >= MAX_DOCUMENTS {
            return Err(SearchError::TooLarge(format!("documents > {MAX_DOCUMENTS}")));
        }
        if self.pending.len() >= MAX_PENDING_EVENTS {
            return Err(SearchError::TooLarge(format!("pending events > {MAX_PENDING_EVENTS}")));
        }
        self.seq += 1;
        let event = Event {
            schema: SCHEMA.to_string(),
            seq: self.seq,
            kind: kind.to_string(),
            id: id.to_string(),
            text: text.to_string(),
        };
        if let Err(error) = self.apply(&event) {
            self.seq -= 1;
            return Err(error);
        }
        self.pending.push(event);
        Ok(())
    }

    /// 🎬️ Folds one event into the live document set.
    fn apply(&mut self, event: &Event) -> Result<(), SearchError> {
        match event.kind.as_str() {
            "indexed" => {
                if event.id.is_empty() {
                    return Err(SearchError::Invalid("index event has empty id".to_string()));
                }
                if event.text.len() > MAX_DOCUMENT_BYTES {
                    return Err(SearchError::Corrupt(format!(
                        "document {:?} exceeds {MAX_DOCUMENT_BYTES} bytes",
                        event.id
                    )));
                }
                if !self.docs.contains_key(&event.id) && self.docs.len() >= MAX_DOCUMENTS {
                    return Err(SearchError::Corrupt(format!("documents exceed {MAX_DOCUMENTS}")));
                }
                self.docs.insert(event.id.clone(), event.text.clone());
            }
            "deleted" => {
                self.docs.remove(&event.id);
            }
            other => return Err(SearchError::Corrupt(format!("unknown index event kind {other:?}"))),
        }
        Ok(())
    }

    /// 💾️ Renders the buffered events as the JSONL lines a commit would append.
    pub fn pending_log(&self) -> String {
        self.pending
            .iter()
            .map(|event| serde_json::to_string(event).unwrap_or_default() + "\n")
            .collect()
    }

    /// 🔒️ Marks the index closed and drops the buffered events.
    pub fn close(&mut self) {
        self.pending.clear();
        self.closed = true;
    }

    //#endregion 📝️AppendReplay

    //#region 🔎️Query

    /// 🔎️ Ranks every live document against the query and returns at most `size` hits.
    pub fn search(&self, terms: &[MatchQuery], size: i64) -> Result<SearchResult, SearchError> {
        if terms.is_empty() {
            return Err(SearchError::Invalid("search query is required".to_string()));
        }
        if terms.len() > MAX_QUERY_TERMS {
            return Err(SearchError::TooLarge(format!("query terms > {MAX_QUERY_TERMS}")));
        }
        let query_bytes: usize = terms.iter().map(|term| term.term.len()).sum();
        if query_bytes > MAX_QUERY_BYTES {
            return Err(SearchError::TooLarge(format!("query bytes > {MAX_QUERY_BYTES}")));
        }
        let mut hits: Vec<SearchHit> = Vec::new();
        for (id, text) in &self.docs {
            if let Some(score) = score(text, terms) {
                hits.push(SearchHit { id: id.clone(), score });
            }
        }
        hits.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.id.cmp(&right.id))
        });
        let total = hits.len() as u64;
        if size >= 0 && hits.len() > size as usize {
            hits.truncate(size as usize);
        }
        Ok(SearchResult { total, hits })
    }
}

/// 🏅️ Scores one document against every query term, or reports that a term did not match.
pub fn score(text: &str, queries: &[MatchQuery]) -> Option<f64> {
    let lowered = text.to_lowercase();
    let words: Vec<&str> = lowered
        .split(|current: char| {
            !(current.is_ascii_lowercase() || current.is_ascii_digit() || (current as u32) >= 0x80)
        })
        .filter(|word| !word.is_empty())
        .collect();
    let mut total = 0.0f64;
    for query in queries {
        let term = query.term.to_lowercase();
        let mut best = query.fuzziness + 1;
        if lowered.contains(&term) {
            best = 0;
        }
        for word in &words {
            let distance = bounded_distance(&term, word, query.fuzziness);
            if distance < best {
                best = distance;
            }
        }
        if best > query.fuzziness {
            return None;
        }
        total += (query.fuzziness - best + 1) as f64;
    }
    Some(total)
}

/// 📐️ Returns the Levenshtein distance, or `maximum + 1` as soon as it is provably larger.
pub fn bounded_distance(left: &str, right: &str, maximum: usize) -> usize {
    if left == right {
        return 0;
    }
    let left_bytes = left.as_bytes();
    let right_bytes = right.as_bytes();
    if left_bytes.len().abs_diff(right_bytes.len()) > maximum {
        return maximum + 1;
    }
    let mut previous: Vec<usize> = (0..=right_bytes.len()).collect();
    for row in 1..=left_bytes.len() {
        let mut current = vec![0usize; right_bytes.len() + 1];
        current[0] = row;
        let mut minimum = current[0];
        for column in 1..=right_bytes.len() {
            let cost = usize::from(left_bytes[row - 1] != right_bytes[column - 1]);
            current[column] = (previous[column] + 1)
                .min(current[column - 1] + 1)
                .min(previous[column - 1] + cost);
            minimum = minimum.min(current[column]);
        }
        if minimum > maximum {
            return maximum + 1;
        }
        previous = current;
    }
    previous[right_bytes.len()]
}

//#endregion 🔎️Query

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
