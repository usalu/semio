//! 📝️ Text grammar of the history codec: the field identifiers and record specs of the doc, edit and transition lines and the ops text parser and printer. See [`super`].

use super::*;

//#region 🔖️TextGrammar
// Own twin of crate::os_store::command_format::OpsHeaderLine's grammar, built directly against `dsl_schema` (never `vcs`,
// never `dsl_derive` — this crate has no path dep on either). Field declaration order below
// mirrors vcs's struct field order exactly: `crate::os_dsl::schema::print_record` reorders keyed fields
// scalar-before-composite (stable sort, ties broken by declaration order), so matching vcs's
// declared order here reproduces vcs's exact printed field order for free.

const F_DOC_ID: u16 = 0;

const F_DOC_SCHEMA: u16 = 1;

const F_EDIT_ID: u16 = 0;

const F_EDIT_STARTED: u16 = 1;

const F_EDIT_ACTOR: u16 = 2;

const F_EDIT_FINISHED: u16 = 3;

const F_EDIT_VERB: u16 = 5;

const F_EDIT_LINE: u16 = 6;

const F_TRANSITION_ID: u16 = 0;

const F_TRANSITION_ACTOR: u16 = 1;

const F_TRANSITION_HLC: u16 = 2;

const F_TRANSITION_DEPENDENCIES: u16 = 3;

const F_TRANSITION_PAYLOAD: u16 = 4;

const F_TRANSITION_OBSERVED: u16 = 5;

fn doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(F_DOC_ID, "", Shape::Text).positional(0), FieldSpec::new(F_DOC_SCHEMA, "schema", Shape::Text)])
}

fn edit_spec() -> RecordSpec {
    RecordSpec::new(
        Some("edit"),
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            FieldSpec::new(F_EDIT_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_EDIT_STARTED, "started", Shape::Text),
            FieldSpec::new(F_EDIT_ACTOR, "actor", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_FINISHED, "finished", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_VERB, "verb", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_LINE, "line", Shape::List(Box::new(Shape::Text))),
        ],
    )
}

/// 🔀️ `transition <id> actor=<actor> hlc=<actor>,<physical_ms>,<logical> dependencies=[...]
/// payload=<base64> [observed=<id>]` — one [`HistoryTransitionRecord`].
fn transition_spec() -> RecordSpec {
    RecordSpec::new(
        Some("transition"),
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            FieldSpec::new(F_TRANSITION_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_TRANSITION_ACTOR, "actor", Shape::Text),
            FieldSpec::new(F_TRANSITION_HLC, "hlc", Shape::Tuple(Box::new(Shape::UInt), Some(3))),
            FieldSpec::new(F_TRANSITION_DEPENDENCIES, "dependencies", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_TRANSITION_PAYLOAD, "payload", Shape::Bytes64),
            FieldSpec::new(F_TRANSITION_OBSERVED, "observed", Shape::Text).optional(),
        ],
    )
}

fn record_with(fields: Vec<(u16, FieldValue)>) -> RecordValue {
    RecordValue { fields: fields.into_iter().collect() }
}

fn field_text(record: &RecordValue, id: u16) -> Option<String> {
    match record.get(id) {
        Some(FieldValue::Text(s)) => Some(s.clone()),
        _ => None,
    }
}

fn required_text(record: &RecordValue, id: u16, what: &'static str) -> Result<String, ProtocolError> {
    field_text(record, id).ok_or_else(|| ProtocolError::Malformed { what, offset: 0, detail: "missing required field in ops text".to_string() })
}

fn field_text_list(record: &RecordValue, id: u16) -> Vec<String> {
    match record.get(id) {
        Some(FieldValue::List(items)) => items.iter().filter_map(|v| if let FieldValue::Text(s) = v { Some(s.clone()) } else { None }).collect(),
        _ => Vec::new(),
    }
}

fn field_hlc(record: &RecordValue, id: u16) -> Result<(u64, u64, u64), ProtocolError> {
    match record.get(id) {
        Some(FieldValue::Tuple(items)) => match items.as_slice() {
            [FieldValue::UInt(actor), FieldValue::UInt(physical_ms), FieldValue::UInt(logical)] => Ok((*actor, *physical_ms, *logical)),
            _ => Err(ProtocolError::Malformed { what: "transition hlc", offset: 0, detail: "expected three unsigned integers".to_string() }),
        },
        _ => Err(ProtocolError::Malformed { what: "transition hlc", offset: 0, detail: "missing required field in ops text".to_string() }),
    }
}

fn field_bytes(record: &RecordValue, id: u16, what: &'static str) -> Result<Vec<u8>, ProtocolError> {
    match record.get(id) {
        Some(FieldValue::Bytes64(bytes)) => Ok(bytes.clone()),
        _ => Err(ProtocolError::Malformed { what, offset: 0, detail: "missing required field in ops text".to_string() }),
    }
}

// 🚫️async: R9 pure accessor — only consumer is `.map_err(text_error_to_protocol)`, and
// `Result::map_err` requires a sync `FnOnce`; no suspension point exists in the body either.
fn text_error_to_protocol(err: semio_framework_diagnostic::TextError) -> ProtocolError {
    ProtocolError::Malformed { what: "ops text", offset: err.span.line as u64, detail: err.message }
}

/// 📥️ Parses the full `.ops` text into a `HistoryLog`. Blank lines and `#`-comments
/// normalize away; a two-space-indented line under a pending `edit` header is an opaque forward
/// op line (never interpreted). Unlike `crate::os_store::text_format::replay_ops`, this never replays operation semantics
/// (ops are opaque here) — `HistoryEdit::meta`/inverse are simply never populated from text.
pub fn parse_ops_text(ops: &str) -> Result<HistoryLog, ProtocolError> {
    struct PendingEdit {
        id: String,
        actor: Option<semio_framework_value::SharedUtf8>,
        started_at: String,
        finished_at: Option<String>,
        verb: Option<String>,
        line: Option<String>,
    }

    let mut log = HistoryLog::default();
    let mut pending: Option<PendingEdit> = None;
    let mut forwards: Vec<OpPayload> = Vec::new();

    fn flush(pending: &mut Option<PendingEdit>, forwards: &mut Vec<OpPayload>, edits: &mut HistoryEdits) {
        if let Some(header) = pending.take() {
            edits.push(HistoryEdit {
                id: header.id,
                actor: header.actor,
                started_at: header.started_at,
                finished_at: header.finished_at, verb: header.verb, line: header.line,
                ops: std::mem::take(forwards),
                inverse: Vec::new(),
                meta: None, lane: None,
            });
        }
    }

    for raw_line in ops.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending.is_some() {
            forwards.push(OpPayload { text: Some(trimmed.to_string()), binary: None });
            continue;
        }
        flush(&mut pending, &mut forwards, &mut log.edits);

        let opts = semio_framework_dsl_record::ParseOptions::default();
        let keyword = trimmed.split_whitespace().next().unwrap_or("");
        match keyword {
            "doc" => {
                let record = semio_framework_dsl_record::parse(trimmed, &doc_spec(), &opts).map_err(text_error_to_protocol)?;
                log.doc_id = required_text(&record, F_DOC_ID, "doc id")?;
                log.schema = required_text(&record, F_DOC_SCHEMA, "doc schema")?;
            }
            "edit" => {
                let record = semio_framework_dsl_record::parse(trimmed, &edit_spec(), &opts).map_err(text_error_to_protocol)?;
                pending = Some(PendingEdit {
                    id: required_text(&record, F_EDIT_ID, "edit id")?,
                    started_at: required_text(&record, F_EDIT_STARTED, "edit started")?,
                    actor: field_text(&record, F_EDIT_ACTOR).map(Into::into),
                    finished_at: field_text(&record, F_EDIT_FINISHED),
                    verb: field_text(&record, F_EDIT_VERB),
                    line: match record.get(F_EDIT_LINE) {
                        Some(FieldValue::List(values)) if values.len() <= 1 => values.first().map(|value| match value { FieldValue::Text(line) => Ok(line.clone()), _ => Err(ProtocolError::Malformed { what: "edit line", offset: 0, detail: "expected text".into() }) }).transpose()?,
                        _ => return Err(ProtocolError::Malformed { what: "edit line", offset: 0, detail: "expected explicit zero-or-one text list".into() }),
                    },
                });
                forwards = Vec::new();
            }
            "transition" => {
                let record = semio_framework_dsl_record::parse(trimmed, &transition_spec(), &opts).map_err(text_error_to_protocol)?;
                log.transitions.push(HistoryTransitionRecord {
                    id: required_text(&record, F_TRANSITION_ID, "transition id")?,
                    actor: required_text(&record, F_TRANSITION_ACTOR, "transition actor")?.into(),
                    hlt: field_hlc(&record, F_TRANSITION_HLC)?,
                    dependencies: field_text_list(&record, F_TRANSITION_DEPENDENCIES),
                    observed: field_text(&record, F_TRANSITION_OBSERVED),
                    payload: field_bytes(&record, F_TRANSITION_PAYLOAD, "transition payload")?,
                });
            }
            other => return Err(ProtocolError::Malformed { what: "ops text line", offset: 0, detail: format!("unknown line keyword '{other}'") }),
        }
    }
    flush(&mut pending, &mut forwards, &mut log.edits);
    Ok(log)
}

/// 📤️ Prints a `HistoryLog` back to `.ops` text: `doc`, every edit (header + two-space indented forward op lines), then one
/// `transition` line per [`HistoryTransitionRecord`] — the shared log only: the viewer head stays `.spr`-local
/// (`REC_VIEWER`). Errors if any op payload carries no text
/// (the binary-only `.spr` convention): this crate is schema-agnostic and cannot recover text
/// from an opaque binary payload — printing `.ops` for a real app document goes through the
/// concrete `Mutation::print_op` path instead (`crate::os_store::text_format::print_document_pack`'s `.ops` mirror).
pub fn print_ops_text(log: &HistoryLog) -> Result<String, ProtocolError> {
    let mut out = String::new();

    let doc_record = record_with(vec![(F_DOC_ID, FieldValue::Text(log.doc_id.clone())), (F_DOC_SCHEMA, FieldValue::Text(log.schema.clone()))]);
    out.push_str(&semio_framework_dsl_record::print(&doc_record, &doc_spec(), semio_framework_dsl_record::JoinMode::Inline));
    out.push('\n');

    for edit in &log.edits {
        let mut fields = vec![(F_EDIT_ID, FieldValue::Text(edit.id.clone())), (F_EDIT_STARTED, FieldValue::Text(edit.started_at.clone())), (F_EDIT_LINE, FieldValue::List(edit.line.iter().cloned().map(FieldValue::Text).collect()))];
        if let Some(actor) = &edit.actor {
            fields.push((F_EDIT_ACTOR, FieldValue::Text(actor.as_str().to_owned())));
        }
        if let Some(finished) = &edit.finished_at {
            fields.push((F_EDIT_FINISHED, FieldValue::Text(finished.clone())));
        }
        if let Some(verb) = &edit.verb {
            fields.push((F_EDIT_VERB, FieldValue::Text(verb.clone())));
        }
        out.push_str(&semio_framework_dsl_record::print(&record_with(fields), &edit_spec(), semio_framework_dsl_record::JoinMode::Inline));
        out.push('\n');
        for op in &edit.ops {
            let Some(text) = &op.text else {
                return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "no text face — cannot print .ops for a binary-only op".to_string() });
            };
            out.push_str("  ");
            out.push_str(text);
            out.push('\n');
        }
    }

    for transition in &log.transitions {
        let mut fields = vec![
            (F_TRANSITION_ID, FieldValue::Text(transition.id.clone())),
            (F_TRANSITION_ACTOR, FieldValue::Text(transition.actor.as_str().to_owned())),
            (F_TRANSITION_HLC, FieldValue::Tuple(vec![FieldValue::UInt(transition.hlt.0), FieldValue::UInt(transition.hlt.1), FieldValue::UInt(transition.hlt.2)])),
            (F_TRANSITION_DEPENDENCIES, FieldValue::List(transition.dependencies.iter().map(|s| FieldValue::Text(s.clone())).collect())),
            (F_TRANSITION_PAYLOAD, FieldValue::Bytes64(transition.payload.clone())),
        ];
        if let Some(observed) = &transition.observed {
            fields.push((F_TRANSITION_OBSERVED, FieldValue::Text(observed.clone())));
        }
        out.push_str(&semio_framework_dsl_record::print(&record_with(fields), &transition_spec(), semio_framework_dsl_record::JoinMode::Inline));
        out.push('\n');
    }

    Ok(out)
}
//#endregion 🔖️TextGrammar
