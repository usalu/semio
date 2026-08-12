//! ⚙️ CsvEngine — owns a real `CsvArtifact` + the real RFC 4180 codec.

use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use crate::artifacts::csv::{CsvArtifact, CsvDiff, CsvMutation, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_csv_snapshot() -> CsvSnapshot {
    CsvSnapshot::default()
}

/// 📄️ The `demo` example, parsed once from `examples::demo::PRIMARY_TEXT` — the single source
/// of truth `🗣️example.dsl.semio` is genuinely `print_dsl` of (P2-P1 `fixture_honesty_law`),
/// same pattern as `note::semio_example_snapshot`.
pub fn demo_csv_snapshot() -> CsvSnapshot {
    <CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::csv::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_csv_snapshot())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Codec
//#region 🔖️Tokenizer
/// 📥 RFC 4180 record tokenizer over the WHOLE text (not line-by-line, so a quoted
/// field's embedded `\n`/`\r\n` is consumed as data, not a record boundary), handling
/// escaped `""` quotes, both CRLF and bare-LF line endings, and tracking per-field
/// whether the source actually wrapped it in quotes (real, losslessly-retained
/// information per RFC 4180 §2 rule 5 — quoting is optional).
fn parse_csv_records(text: &str) -> Vec<CsvRecord> {
    let mut records = Vec::new();
    let mut fields: Vec<CsvField> = Vec::new();
    let mut cur = String::new();
    let mut cur_quoted = false;
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    fn take_field(cur: &mut String, cur_quoted: &mut bool) -> CsvField {
        CsvField { value: std::mem::take(cur), quoted: std::mem::take(cur_quoted) }
    }
    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(ch);
            }
            continue;
        }
        match ch {
            '"' => {
                in_quotes = true;
                cur_quoted = true;
            }
            ',' => fields.push(take_field(&mut cur, &mut cur_quoted)),
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                fields.push(take_field(&mut cur, &mut cur_quoted));
                records.push(CsvRecord { fields: std::mem::take(&mut fields) });
            }
            '\n' => {
                fields.push(take_field(&mut cur, &mut cur_quoted));
                records.push(CsvRecord { fields: std::mem::take(&mut fields) });
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() || cur_quoted || !fields.is_empty() {
        fields.push(take_field(&mut cur, &mut cur_quoted));
        records.push(CsvRecord { fields });
    }
    records
}

/// 📤 Quotes a field when the source quoted it OR when RFC 4180 §2 rule 6 REQUIRES
/// quoting (the value itself contains a comma, quote, or line break).
fn escape_field(field: &CsvField) -> String {
    let needs_quote = field.quoted
        || field.value.contains(',')
        || field.value.contains('"')
        || field.value.contains('\n')
        || field.value.contains('\r');
    if needs_quote {
        format!("\"{}\"", field.value.replace('"', "\"\""))
    } else {
        field.value.clone()
    }
}

fn write_csv_records(records: &[CsvRecord], line_ending: &str) -> String {
    let mut out = String::new();
    for record in records {
        out.push_str(&record.fields.iter().map(escape_field).collect::<Vec<_>>().join(","));
        out.push_str(line_ending);
    }
    out
}
//#endregion 🔖️Tokenizer

//#region 🔖️SnapshotCodec
/// 📥 Decodes RFC 4180 text into a snapshot. `has_header` is pure metadata about whether
/// `records[0]` should be read as a header row — RFC 4180 draws no structural distinction
/// between a header record and a data record on the wire, so decoding never drops or
/// relocates the first record.
pub fn decode_csv_with(text: &str, has_header: bool) -> CsvSnapshot {
    let records = parse_csv_records(text);
    CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header, records }
}

/// 📥 Decodes assuming a header row is present (the pre-existing default behavior).
pub fn decode_csv(text: &str) -> Result<CsvSnapshot, String> {
    Ok(decode_csv_with(text, true))
}

/// 📤 Encodes with LF line endings.
pub fn encode_csv(snap: &CsvSnapshot) -> String {
    encode_csv_with(snap, "\n")
}

/// 📤 Encodes with a caller-chosen line ending (`"\n"` or `"\r\n"`).
pub fn encode_csv_with(snap: &CsvSnapshot, line_ending: &str) -> String {
    if snap.records.is_empty() {
        return String::new();
    }
    write_csv_records(&snap.records, line_ending)
}
//#endregion 🔖️SnapshotCodec
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::csv::io_registry::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<CsvSnapshot, CsvMutation>(STDIO_CSV_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) — 5-role
/// `LanguageSpec` set (Document/Ops/Diff/Pack/Spr), following `note`'s exemplar pattern exactly
/// (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv",
        extension: Some("csv"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.csv`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::csv::schema::csv_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.csv` artifact engine.
pub struct CsvEngine {
    artifact_state: CsvArtifact,
    snapshot_state: CsvSnapshot,
}

impl CsvEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: CsvSnapshot) -> Self {
        let artifact_state = CsvArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_csv_snapshot();
        assert_eq!(snapshot.schema, STDIO_CSV_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_csv_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <CsvSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    fn field_values(record: &CsvRecord) -> Vec<String> {
        record.fields.iter().map(|f| f.value.clone()).collect()
    }

    #[test]
    fn quoted_field_with_embedded_comma_and_escaped_quote() {
        let text = "name,note\n\"Doe, John\",\"He said \"\"hi\"\"\"\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(field_values(&snap.records[0]), vec!["name", "note"]);
        assert_eq!(field_values(&snap.records[1]), vec!["Doe, John".to_string(), "He said \"hi\"".to_string()]);
        assert!(snap.records[1].fields[0].quoted, "comma-containing field must be recorded as quoted");
        assert!(snap.records[1].fields[1].quoted);
        assert!(!snap.records[0].fields[0].quoted, "unquoted header field stays unquoted");
    }

    #[test]
    fn quoted_field_with_embedded_newline_spans_records() {
        let text = "a,b\n\"line1\nline2\",2\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(field_values(&snap.records[1]), vec!["line1\nline2".to_string(), "2".to_string()]);
    }

    #[test]
    fn crlf_and_lf_both_parse_to_the_same_records() {
        let lf = "a,b\n1,2\n3,4\n";
        let crlf = "a,b\r\n1,2\r\n3,4\r\n";
        assert_eq!(decode_csv_with(lf, true), decode_csv_with(crlf, true));
    }

    #[test]
    fn header_row_option_is_pure_metadata_first_record_always_decoded() {
        let text = "1,2\n3,4\n";
        let with_header = decode_csv_with(text, true);
        assert!(with_header.has_header);
        assert_eq!(with_header.records.len(), 2);
        assert_eq!(field_values(&with_header.records[0]), vec!["1", "2"]);
        assert_eq!(field_values(&with_header.records[1]), vec!["3", "4"]);

        let without_header = decode_csv_with(text, false);
        assert!(!without_header.has_header);
        assert_eq!(without_header.records, with_header.records);
    }

    #[test]
    fn quoted_flag_round_trips_even_when_not_structurally_required() {
        // 🔒 A field that didn't NEED quoting but WAS quoted in the source must re-encode
        // quoted (lossless retention, not a lossy structural-minimum normal form).
        let snap = CsvSnapshot {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: false,
            records: vec![CsvRecord { fields: vec![CsvField { value: "plain".into(), quoted: true }] }],
        };
        let text = encode_csv(&snap);
        assert_eq!(text, "\"plain\"\n");
        let reparsed = decode_csv_with(&text, false);
        assert_eq!(reparsed.records, snap.records);
    }

    #[test]
    fn encode_with_crlf_round_trips() {
        let snap = decode_csv_with("a,b\n1,2\n", true);
        let crlf_text = encode_csv_with(&snap, "\r\n");
        assert!(crlf_text.contains("\r\n"));
        let reparsed = decode_csv_with(&crlf_text, true);
        assert_eq!(reparsed, snap);
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode is byte-preserving on a fixture exercising every retention-sensitive
    /// case at once: unquoted fields, a field quoted only because it's structurally required
    /// (embedded comma), a field quoted despite NOT being structurally required (pure
    /// retention), an empty field, and an embedded-newline field spanning lines.
    #[test]
    fn codec_retention_law() {
        let fixture = "name,note,tag,blank\n\"Doe, John\",\"He said \"\"hi\"\"\",\"kept-quoted\",\n\"multi\nline\",x,y,z\n";
        let snap = decode_csv_with(fixture, true);
        let reencoded = encode_csv(&snap);
        assert_eq!(reencoded, fixture, "decode->encode must be byte-preserving on this fixture");

        let reparsed = decode_csv_with(&reencoded, true);
        assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️P2P1GrammarProtocolFixtureLaws
    /// 🧪️ P2-P1: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against the
    /// REAL fixture body — the snapshot text facet's own real RFC 4180 grammar recognizes the
    /// genuine `print_dsl` output (envelope-id-normalized, matching how
    /// `dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dsl_body_from_fixture` feeds the
    /// Recognizer, mirrored here so this law does not depend on the framework's own harness).
    #[test]
    fn grammar_conformance_law() {
        let grammar_text = crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse snapshot grammar");
        assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar);
        let recognizer = dsl::Recognizer::compile(&grammar);
        let fixture = crate::artifacts::csv::examples::demo::PRIMARY_TEXT;
        let (envelope, body) = store::semio_format::split_text_preamble(fixture).expect("real preamble");
        let normalized = format!("{}\n{body}", envelope.envelope_id());
        let ok = recognizer.recognize(&normalized).expect("recognize should not error");
        assert!(ok, "snapshot grammar must recognize the real demo fixture body");
    }

    /// 🧪️ P2-P1: `dsl::parse_protocol` + `dsl::walk_protocol` against REAL bytes for all three
    /// binary facets (Pack/Spr/Diff), asserting `consumed == bytes.len()` exactly (the walker's
    /// own law) — snapshot's Pack facet walks the post-`unwrap_binary` payload of a genuine
    /// `encode_pack` call; mutations' Spr facet walks a genuine `encode_op` frame; diff's own
    /// protocol facet walks a genuine `encode_diff` frame.
    #[test]
    fn protocol_walk_law() {
        // Pack (snapshot binary facet).
        let snap = demo_csv_snapshot();
        let pack_bytes = <CsvSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let (_, payload) = store::semio_format::unwrap_binary(&pack_bytes).expect("unwrap_binary");
        let pack_protocol = dsl::parse_protocol(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let trace = dsl::walk_protocol(&pack_protocol, &payload).expect("walk snapshot protocol");
        assert_eq!(trace.consumed, payload.len(), "snapshot protocol must consume the whole post-envelope payload");

        // Spr (mutations binary facet) — a real, non-trivial mutation.
        let mutation = CsvMutation::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } };
        let op_bytes = <CsvMutation as protocol::OpBinary>::encode_op(&mutation).expect("encode_op");
        let spr_protocol = dsl::parse_protocol(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        let trace = dsl::walk_protocol(&spr_protocol, &op_bytes).expect("walk mutations protocol");
        assert_eq!(trace.consumed, op_bytes.len(), "mutations protocol must consume the whole op frame");

        // Diff binary facet.
        let mut before = snap.clone();
        let diff = crate::artifacts::csv::schema::mutations::apply_csv_mutation(&mut before, &mutation);
        let diff_bytes = <CsvDiff as protocol::DiffCodec>::encode_diff(&diff).expect("encode_diff");
        let diff_protocol = dsl::parse_protocol(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        let trace = dsl::walk_protocol(&diff_protocol, &diff_bytes).expect("walk diff protocol");
        assert_eq!(trace.consumed, diff_bytes.len(), "diff protocol must consume the whole diff frame");
    }

    /// 🧪️ P2-P1 item 5: fixture honesty — the committed `.dsl.semio`/`.pack.semio` fixtures are
    /// genuinely `print_dsl`/`encode_pack` output of the SAME demo snapshot, round-tripping both
    /// ways (never allowed to silently drift again).
    #[test]
    fn fixture_honesty_law() {
        let demo = demo_csv_snapshot();
        assert_eq!(<CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::csv::examples::demo::PRIMARY_TEXT).unwrap(), demo);
        assert_eq!(<CsvSnapshot as store::ArtifactDsl>::print_dsl(&demo), crate::artifacts::csv::examples::demo::PRIMARY_TEXT);

        assert_eq!(<CsvSnapshot as store::ArtifactPack>::decode_pack(crate::artifacts::csv::examples::demo::PACK_BYTES).unwrap(), demo);
        assert_eq!(<CsvSnapshot as store::ArtifactPack>::encode_pack(&demo), crate::artifacts::csv::examples::demo::PACK_BYTES.to_vec());
    }

    /// 🧪️ P2-P1 item 6: every committed grammar/protocol file for this standard genuinely
    /// parses under `dsl::parse_grammar`/`dsl::parse_protocol` — this artifact's own early
    /// warning, independent of the eventual repo-wide policy gate.
    #[test]
    fn committed_grammar_and_protocol_files_parse() {
        let g1 = dsl::parse_grammar(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g1.is_ok(), "snapshot grammar must parse: {g1:?}");
        let g2 = dsl::parse_grammar(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g2.is_ok(), "mutations grammar must parse: {g2:?}");
        let g3 = dsl::parse_grammar(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g3.is_ok(), "diff grammar must parse: {g3:?}");
        let p1 = dsl::parse_protocol(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p1.is_ok(), "snapshot protocol must parse: {p1:?}");
        let p2 = dsl::parse_protocol(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p2.is_ok(), "mutations protocol must parse: {p2:?}");
        let p3 = dsl::parse_protocol(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p3.is_ok(), "diff protocol must parse: {p3:?}");
    }
    //#endregion 🔖️P2P1GrammarProtocolFixtureLaws

    //#region 🔖️ScratchFixtureGen
    /// 🧪️[DEBUG] one-shot scratch generator — writes real `encode_pack`/`encode_op` bytes to the
    /// committed fixture paths. Run once via `--ignored`, then this region is deleted (never a
    /// permanent side-effecting test; CLAUDE.md bans migration scripts left behind).
    #[test]
    #[ignore]
    fn zzz_generate_p2p1_fixtures() {
        let repo_root = {
            let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            loop {
                if dir.join("nx.json").is_file() { break dir; }
                assert!(dir.pop(), "could not find repo root");
            }
        };
        let assets = repo_root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📚️examples/🎬️demo/🖼️assets");
        let demo = demo_csv_snapshot();
        let pack_bytes = <CsvSnapshot as store::ArtifactPack>::encode_pack(&demo);
        std::fs::write(assets.join("🎒️example.pack.semio"), &pack_bytes).unwrap();
        let mutation = CsvMutation::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } };
        let op_bytes = <CsvMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();
        std::fs::write(assets.join("📡️example.spr.semio"), &op_bytes).unwrap();
        eprintln!("[DEBUG] wrote {} pack bytes, {} spr bytes", pack_bytes.len(), op_bytes.len());
    }
    //#endregion 🔖️ScratchFixtureGen
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::CsvComposer as CsvRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<CsvRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
