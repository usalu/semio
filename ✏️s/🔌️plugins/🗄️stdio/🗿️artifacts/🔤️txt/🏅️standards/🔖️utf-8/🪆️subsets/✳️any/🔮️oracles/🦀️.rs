//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: none — recorded no-oracle decision `txt-utf-8-line-structure`. Line splitting,
//! line-ending policy and trailing-newline handling are exactly what THIS subset defines; no
//! third-party crate is authoritative over them the way `lopdf` is authoritative over PDF. What
//! stands in for a reference implementation instead:
//! 1. [`independent_split`]/[`independent_render`] — a hand-written re-derivation of the
//!    subset's own documented Lf/CrLf-only spec, compiled into THIS crate
//!    (`semio_s_plugin_stdio_test_oracle`), which never depends on the subject crate
//!    (`semio_s_plugin_stdio`) and therefore never calls `TxtSnapshot::from_body`/`to_body`.
//! 2. [`csv_independent_line_count`] — the `csv` crate's own record reader (already linked for
//!    the tabular subsets) as a genuinely independent, third-party cross-check of WHERE the line
//!    boundaries fall, on the real fixture and on every spec vector. It cannot referee the
//!    LF-vs-CRLF/trailing-newline questions themselves (its terminator collapses CR, LF and CRLF
//!    into one undifferentiated boundary and never reports which one it saw), so it discharges
//!    only part of the line-splitting half — see the rationale on the manifest's
//!    `noOracleDecisions` entry for the full accounting.
//! 3. Specification vectors and the inverse law as a metamorphic property, both exercised in
//!    `../🧪️tests/📝️mutate-txt-utf-8/`.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🔣️oracle.json — the mutation catalog and no-oracle decision this module is
//! measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`TxtMutation`,
//! `KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️IndependentReader
/// 🧩️ Splits a raw UTF-8 body into `(lines, trailing_newline, is_crlf)`, hand-derived directly
/// from the subset's own documented rule ("a text file is a sequence of lines"; the format
/// declares exactly `Lf`/`CrLf`, never a mixed per-line style; the whole document is CrLf iff it
/// contains at least one literal `\r\n`) — never by calling `TxtSnapshot::from_body`, which this
/// crate cannot even see (it does not depend on the subject crate). An empty body is zero lines,
/// not one empty line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn independent_split(body: &str) -> (Vec<String>, bool, bool) {
    if body.is_empty() {
        return (Vec::new(), false, false);
    }
    let is_crlf = body.as_bytes().windows(2).any(|pair| pair == b"\r\n");
    let sep = if is_crlf { "\r\n" } else { "\n" };
    let trailing_newline = body.ends_with(sep);
    let core = if trailing_newline { &body[..body.len() - sep.len()] } else { body };
    let lines: Vec<String> = core.split(sep).map(str::to_string).collect();
    (lines, trailing_newline, is_crlf)
}

/// 🧩️ Inverse of [`independent_split`]: joins `lines` by the chosen separator, appending a
/// trailing terminator iff `trailing_newline`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn independent_render(lines: &[String], trailing_newline: bool, is_crlf: bool) -> String {
    let sep = if is_crlf { "\r\n" } else { "\n" };
    let mut out = lines.join(sep);
    if trailing_newline {
        out.push_str(sep);
    }
    out
}

/// 🔒️ Why `(lines, trailing_newline)` is not the canonical decomposition of the body it renders, or
/// `None` when it is — re-derived here from the rendering rule itself, never by calling the
/// subject's `TxtMutation`/`TxtSnapshot`, which this crate cannot see. [`independent_render`] is a
/// join plus an optional terminator, so `(L, true)` and `(L ++ [""], false)` emit identical bytes,
/// as do `(vec![], true)` and `(vec![""], true)`; [`independent_split`] resolves both ties in favour
/// of the terminated reading. Exactly those two shapes therefore lie outside its image, and a
/// mutation that lands on one has silently lost a line the way back cannot recover.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn non_canonical_reason(lines: &[String], trailing_newline: bool) -> Option<String> {
    if trailing_newline && lines.is_empty() {
        return Some("a document with no lines cannot carry a trailing terminator — that pair renders the very bytes the one-empty-line document renders, and reading them back returns the latter".to_string());
    }
    if !trailing_newline && lines.last().is_some_and(|line| line.is_empty()) {
        return Some(
            "a document whose last line is empty cannot drop its trailing terminator — that pair renders the very bytes the same document one line shorter renders, and reading them back returns the latter, losing the empty line".to_string(),
        );
    }
    None
}
//#endregion 🔖️IndependentReader

//#region 🔖️Projection
/// 🔎️ The `exact-bytes-v1` projection: the whole re-serialized document AS TEXT, so the profile's
/// opaque-byte-string comparison catches any difference at all — a carrier format has nothing a
/// looser profile is entitled to ignore.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn project_txt(bytes: &[u8]) -> Result<Json, String> {
    String::from_utf8(bytes.to_vec()).map(Json::String).map_err(|error| format!("output is not UTF-8: {error}"))
}
//#endregion 🔖️Projection

//#region 🔖️SpecHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_usize(params: &Json, key: &str) -> Result<usize, String> {
    match params.get(key) {
        Some(Json::Number(value)) => Ok(*value as usize),
        _ => Err(format!("mutation spec is missing numeric `{key}`")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_bool(params: &Json, key: &str) -> bool {
    matches!(params.get(key), Some(Json::Bool(true)))
}

//#endregion 🔖️SpecHelpers

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test. Every arm is hand-rolled against [`independent_split`]/
/// [`independent_render`] alone, mirroring the clamping/no-op rules the subset's own
/// `TxtMutation::diff`/`TxtLinesDiff::apply` document (`InsertLine` clamps to `min(index, len)`;
/// an out-of-range `RemoveLine`/`SetLine` is a no-op) — those are the FORMAT's rules, not this
/// crate's implementation detail, so a genuinely independent reader has to agree with them too.
///
/// 🔒️ The same holds for representability: a result outside [`independent_split`]'s image is a
/// document this encoding cannot write down, so it is REFUSED here rather than rendered into bytes
/// that read back as something else. That refusal is a property of the encoding, arrived at from
/// the join rule alone (see [`non_canonical_reason`]), and the subset's own vocabulary refuses the
/// same states under `stdio.txt.mutation-not-representable` — two independent statements of one
/// format rule, not one implementation consulted twice.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let body = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
    let (mut lines, mut trailing_newline, mut is_crlf) = independent_split(body);
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    match spec.str("kind").as_str() {
        "" => return Err("mutation spec carries no `kind`".to_string()),
        "set-trailing-newline" => trailing_newline = json_bool(&params, "value"),
        "set-line-ending" => is_crlf = params.str("value") == "crLf",
        "insert-line" => {
            let index = json_usize(&params, "index")?;
            let at = index.min(lines.len());
            lines.insert(at, params.str("text"));
        }
        "remove-line" => {
            let index = json_usize(&params, "index")?;
            if index < lines.len() {
                lines.remove(index);
            }
        }
        "set-line" => {
            let index = json_usize(&params, "index")?;
            if let Some(slot) = lines.get_mut(index) {
                *slot = params.str("text");
            }
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
    if let Some(reason) = non_canonical_reason(&lines, trailing_newline) {
        return Err(format!("{} is not representable on this document — {reason}", spec.str("kind")));
    }
    Ok(independent_render(&lines, trailing_newline, is_crlf).into_bytes())
}

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the SAME independent reader [`oracle_apply_mutation`] mutates
/// with — never by calling this repository's own `TxtMutation::inverse`, which would defeat the
/// point of an independently-computed reference. Mirrors that method's documented rule exactly
/// (index-aware, reading the pre-state it needs from the ORIGINAL document; `insert-line`'s inverse
/// lands at `min(index, len)`, matching the clamped position it actually inserted at).
///
/// 🏠️ Lives HERE, in the reference module, rather than in the case adapter, because it is reference
/// SEMANTICS and not test plumbing: the adapter drives it, and this module's own unit tests below
/// exercise it against the real committed fixture — which is the only place the inverse law for
/// this subset is checked at all today, the case being a recorded no-oracle one whose scenarios the
/// runner never dispatches in the oracle phase.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let body = std::str::from_utf8(original).map_err(|error| format!("input is not UTF-8: {error}"))?;
    let (lines, trailing_newline, is_crlf) = independent_split(body);
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let index = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value as usize),
        _ => None,
    };
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let spec = |kind: &str, params: Json| Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)]);
    let ending = if is_crlf { "crLf" } else { "lf" };
    match forward.str("kind").as_str() {
        "set-trailing-newline" => Ok(spec("set-trailing-newline", object(vec![("value", Json::Bool(trailing_newline))]))),
        "set-line-ending" => Ok(spec("set-line-ending", object(vec![("value", Json::String(ending.to_string()))]))),
        "insert-line" => {
            let requested = index("index").ok_or("insert-line inverse: missing `index`")?;
            Ok(spec("remove-line", object(vec![("index", Json::Number(requested.min(lines.len()) as f64))])))
        }
        "remove-line" => {
            let requested = index("index").ok_or("remove-line inverse: missing `index`")?;
            match lines.get(requested) {
                Some(text) => Ok(spec("insert-line", object(vec![("index", Json::Number(requested as f64)), ("text", Json::String(text.clone()))]))),
                None => Err("remove-line inverse has no operation for an absent line".to_string()),
            }
        }
        "set-line" => {
            let requested = index("index").ok_or("set-line inverse: missing `index`")?;
            match lines.get(requested) {
                Some(text) => Ok(spec("set-line", object(vec![("index", Json::Number(requested as f64)), ("text", Json::String(text.clone()))]))),
                None => Err("set-line inverse has no operation for an absent line".to_string()),
            }
        }
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_original: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️CsvCrossCheck
/// 🧮️ Independent LINE-BOUNDARY cross-check via the `csv` crate's own record reader (`csv-core`
/// 0.1's `ReaderBuilder`, default `Terminator::CRLF`, which "parses `\r`, `\n` or `\r\n` as a
/// single record terminator" per its own doc comment): quoting disabled, `flexible` records, and
/// a delimiter byte (`0x1F`, unit separator) that never occurs in real text, so every "record" IS
/// one physical line, verbatim. This can confirm `independent_split`'s LINE COUNT and per-line
/// CONTENT independently of this subset's own splitting rule, but it genuinely cannot referee
/// which terminator style was used (CR/LF/CRLF collapse to one undifferentiated boundary) or
/// whether the input ends with one — see the manifest's `noOracleDecisions` rationale for why
/// this is a partial, not a full, substitute.
#[cfg(feature = "oracles")]
pub fn csv_independent_line_count(body: &str) -> Result<usize, String> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).quoting(false).flexible(true).delimiter(0x1F).terminator(csv::Terminator::CRLF).from_reader(body.as_bytes());
    let mut count = 0usize;
    for record in reader.records() {
        record.map_err(|error| format!("csv cross-check reader error: {error}"))?;
        count += 1;
    }
    Ok(count)
}
//#endregion 🔖️CsvCrossCheck

//#region 🧪️Tests
#[cfg(test)]
#[cfg(feature = "oracles")]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
