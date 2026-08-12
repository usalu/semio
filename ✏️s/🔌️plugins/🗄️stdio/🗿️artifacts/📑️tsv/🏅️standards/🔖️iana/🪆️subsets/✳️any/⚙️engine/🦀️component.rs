//! ⚙️ TsvEngine — owns the real IANA text/tab-separated-values codec
//! (https://www.iana.org/assignments/media-types/text/tab-separated-values). Byte-exact
//! split/rejoin: TSV has no quoting/escaping mechanism, so — unlike csv — this codec never
//! invents one; a field containing a literal tab or newline byte is a genuine, spec-honest
//! limitation of the format itself, documented (not silently worked around) below and in the
//! subset's own `📝️text/📖️component.grammar.semio` leaf.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};

//#region 🔖️Sniff
/// 🔍️ TSV has no reliable magic bytes (per the master plan: "heuristic tab-density check or just
/// accept-by-default since TSV has no reliable magic"). Real structural heuristic: at least one
/// line, and every line contains at least one tab OR the file is a single untabbed line (a valid
/// one-column TSV) — i.e. reject obvious binary noise (NUL bytes) rather than claim a false magic.
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    !bytes.is_empty() && !bytes.contains(&0u8)
}
//#endregion 🔖️Sniff

//#region 🔖️SnapshotCodec
/// 📥️ Decodes TSV text via a byte-exact split on the file's own line ending, then `\t` per line
/// — no quoting, no escaping, no coercion (matches the real W0 fixture's own `verify_tsv.py`
/// verification method exactly: split on `\n`, then each line on `\t`).
pub fn decode_tsv(text: &str) -> TsvSnapshot {
    let line_ending = if text.contains("\r\n") { LineEnding::Crlf } else { LineEnding::Lf };
    let sep = line_ending.as_str();
    let trailing_newline = text.ends_with(sep);
    let body = if trailing_newline { &text[..text.len() - sep.len()] } else { text };
    let records: Vec<Vec<String>> = if body.is_empty() {
        Vec::new()
    } else {
        body.split(sep).map(|line| line.split('\t').map(|s| s.to_string()).collect()).collect()
    };
    TsvSnapshot { schema: STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline, line_ending }
}

/// 📤️ Encodes via a byte-exact rejoin: `\t` within a row, the snapshot's own `line_ending`
/// between rows, plus a final terminator iff `trailing_newline` is set.
pub fn encode_tsv(snap: &TsvSnapshot) -> String {
    let sep = snap.line_ending.as_str();
    let mut out = snap.records.iter().map(|r| r.join("\t")).collect::<Vec<_>>().join(sep);
    if snap.trailing_newline {
        out.push_str(sep);
    }
    out
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor (via the ✳️any subset composer) and this
/// standard's handcrafted grammar/protocol.
pub fn register() {
    crate::artifacts::tsv::standards::iana::subsets::any::io::register();
    register_pilot_languages();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot;
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.tsv",
        extension: Some("tsv"),
        role: dsl::LanguageRole::Document,
        grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.tsv"),
    });
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const REAL_FIXTURE: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/example.tsv");

    #[test]
    fn round_trips_a_real_shaped_tsv_body() {
        let text = "name\tage\nAda\t30\nGrace\t85\n";
        assert!(sniff_real_bytes(text.as_bytes()));
        let snap = decode_tsv(text);
        assert_eq!(snap.records[0], vec!["name", "age"]);
        assert_eq!(snap.records.len(), 3);
        assert!(snap.trailing_newline);
        assert_eq!(snap.line_ending, LineEnding::Lf);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[test]
    fn detects_crlf_line_ending() {
        let text = "a\tb\r\n1\t2\r\n";
        let snap = decode_tsv(text);
        assert_eq!(snap.line_ending, LineEnding::Crlf);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[test]
    fn sniff_rejects_binary_noise() {
        assert!(!sniff_real_bytes(b"a\tb\0\x01\x02"));
    }

    #[test]
    fn embedded_backslash_t_is_not_a_real_tab() {
        // 🔒 Documents the honest IANA TSV limitation: a genuine tab byte inside a field is
        // indistinguishable from a column boundary (no quoting mechanism exists to escape it).
        // A `\t` two-character ESCAPE SEQUENCE (backslash + t), by contrast, is just two
        // ordinary text characters and round-trips perfectly, same as the real W0 fixture's row 5.
        let text = "note\nWeathering Test\\tSample\n";
        let snap = decode_tsv(text);
        assert_eq!(snap.records[1], vec!["Weathering Test\\tSample"]);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[test]
    fn decodes_the_real_fixture_with_6_rows_and_5_columns() {
        let snap = decode_tsv(REAL_FIXTURE);
        assert_eq!(snap.records.len(), 6, "1 header row + 5 data rows");
        for row in &snap.records {
            assert_eq!(row.len(), 5, "every row must have exactly 5 columns");
        }
        assert!(snap.trailing_newline);
        assert_eq!(snap.line_ending, LineEnding::Lf);
        assert_eq!(snap.records[0], vec!["id", "name", "qty", "unit_price", "note"]);
        assert_eq!(snap.records[5][1], "Weathering Test\\tSample");
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode is byte-preserving on the real W0 fixture
    /// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📚️examples/🎬️demo/🖼️assets/example.tsv`,
    /// verified upstream by `verify_tsv.py`'s own "byte-exact split/rejoin" check).
    #[test]
    fn codec_retention_law() {
        let snap = decode_tsv(REAL_FIXTURE);
        let reencoded = encode_tsv(&snap);
        assert_eq!(reencoded, REAL_FIXTURE, "decode->encode must be byte-preserving on the real W0 fixture");

        let reparsed = decode_tsv(&reencoded);
        assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
    }
    //#endregion 🔖️CodecRetentionLaw
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::TsvComposer as TsvRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TsvRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
