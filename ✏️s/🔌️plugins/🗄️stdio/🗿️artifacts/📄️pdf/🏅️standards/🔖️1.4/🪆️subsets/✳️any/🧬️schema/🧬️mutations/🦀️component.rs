//! 🧬️ PdfMutation — document mutation dispatch for `stdio.pdf` (1.4).
//!
//! ✍️ **Why `OpText`/`OpBinary` are hand-rolled here.** `SetSnapshot` carries the whole
//! `PdfSnapshot`, and the printed form of that payload is what the sibling facet file
//! `📝️text/📖️component.grammar.semio` has to state production for production. A derived printer
//! chooses that shape for us; a hand-rolled one lets the grammar be written from this file's own
//! `format!` call sites, which is what `ops_grammar_conformance_law` actually checks. The 1.7
//! standard hand-rolls its own for the same reason, and the binary payload is genuine
//! varint/length-prefixed structure reusing the diff facet's `pub(crate)` primitives rather than a
//! second encoding of the same snapshot.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::{dec_snapshot_bin, diff_set_snapshot, enc_snapshot_bin, PdfDiff};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::{Mutation, OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pdf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
}

/// 🦠️ Kebab-case spelling of every `PdfMutation` variant, in declaration order — the ground truth
/// `oracle_mutation_kinds_law` below checks itself against, and what `🧪️oracle/🔣️component.json`'s
/// `mutationCatalogs[].kinds` must equal. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    let outcome = <PdfMutation as Mutation<PdfSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            PdfMutation::NoMutation => PdfDiff::default(),
            PdfMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
        })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        match self {
            PdfMutation::NoMutation => vec![PdfMutation::NoMutation],
            PdfMutation::SetSnapshot { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 📸️ The snapshot payload as lowercase hex over its own binary encoding — one structure, one
/// encoding, shared with the diff facet (`enc_snapshot_bin`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_snapshot(snapshot: &PdfSnapshot) -> String {
    let mut bytes = Vec::new();
    enc_snapshot_bin(snapshot, &mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_snapshot(text: &str) -> Result<PdfSnapshot, String> {
    if text.len() % 2 != 0 {
        return Err(format!("odd hex length: {text:?}"));
    }
    let bytes: Result<Vec<u8>, String> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect();
    let bytes = bytes?;
    let mut reader = store::ByteReader::new(&bytes);
    let snapshot = dec_snapshot_bin(&mut reader)?;
    if reader.remaining() != 0 {
        return Err(format!("snapshot: {} trailing bytes", reader.remaining()));
    }
    Ok(snapshot)
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        match self {
            PdfMutation::NoMutation => "no-mutation".to_string(),
            PdfMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parse = |line: &str| -> Result<Self, String> {
            if line == "no-mutation" {
                return Ok(PdfMutation::NoMutation);
            }
            match line.strip_prefix("set-snapshot snapshot=") {
                Some(rest) => Ok(PdfMutation::SetSnapshot { snapshot: dec_snapshot(rest)? }),
                None => Err(format!("pdf 1.4 mutation: unknown operation line {line:?}")),
            }
        };
        parse(line).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }
}

/// 🧪️ Real binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape. `tag` is
/// the variant ordinal in [`KINDS`] order.
impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            PdfMutation::NoMutation => 0,
            PdfMutation::SetSnapshot { .. } => 1,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        if let PdfMutation::SetSnapshot { snapshot } = self {
            enc_snapshot_bin(snapshot, &mut out);
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|error| malformed("op format", 0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let tag = reader.read_u8().map_err(|error| malformed("op tag", 1, error.to_string()))?;
        let mutation = match tag {
            0 => PdfMutation::NoMutation,
            1 => PdfMutation::SetSnapshot { snapshot: dec_snapshot_bin(&mut reader).map_err(|error| malformed("op snapshot", reader.position(), error))? },
            other => return Err(malformed("op tag", 1, format!("unknown variant tag {other}"))),
        };
        if reader.remaining() != 0 {
            return Err(malformed("op trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(mutation)
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;
    use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(width: f64, height: f64, text: &str) -> PdfSnapshot {
        PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width, height, text: text.into() }] }
    }

    //#region mutation_diff_law
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_matches_apply_pdf_mutation() {
        let base = snap(612.0, 792.0, "base");
        let cases = vec![PdfMutation::NoMutation, PdfMutation::SetSnapshot { snapshot: snap(300.0, 400.0, "next") }];
        for m in cases {
            let mut s = base.clone();
            let returned_diff = apply_pdf_mutation(&mut s, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
            assert_eq!(s, expected_diff.diff().apply(&base).unwrap());
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn mutation_apply_inverse_round_trips_every_variant() {
        let base = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "base".into() }, PageDoc { width: 1.0, height: 2.0, text: "second".into() }] };
        for m in [PdfMutation::NoMutation, PdfMutation::SetSnapshot { snapshot: snap(300.0, 400.0, "next") }] {
            let diff = m.diff(&base);
            let mutated = diff.diff().apply(&base).unwrap();
            let mut restored = mutated;
            for inv in m.inverse(&base) {
                let inv_diff = inv.diff(&restored);
                restored = inv_diff.diff().apply(&restored).unwrap();
            }
            assert_eq!(restored, base, "apply(inverse(m), apply(m, base)) must recover base for {m:?}");
        }
    }
    //#endregion inverse_law

    //#region kinds_law
    /// 🧪️ Wave 7: `KINDS` is the ONLY place the catalog's honesty rests on, since the framework
    /// never parses this enum -- one arm here mirrors every enum variant (kept in sync by hand,
    /// caught by this test the moment they drift), and the other mirrors the manifest's declared
    /// `kinds`, which are byte-identical strings at plan time (verified separately by the contract
    /// gate against `🧪️oracle/🔣️component.json`).
    #[semio_framework_async_macros::async_test]
    async fn oracle_mutation_kinds_law_matches_enum_variants() {
        let variants = [PdfMutation::NoMutation, PdfMutation::SetSnapshot { snapshot: PdfSnapshot::default() }];
        assert_eq!(KINDS.len(), variants.len(), "KINDS must list exactly one kebab-case entry per PdfMutation variant");
        for (kind, variant) in KINDS.iter().zip(variants.iter()) {
            let matches = matches!((*kind, variant), ("no-mutation", PdfMutation::NoMutation) | ("set-snapshot", PdfMutation::SetSnapshot { .. }));
            assert!(matches, "KINDS entry {kind:?} does not correspond to variant {variant:?} in declaration order");
        }
    }

    const MANIFEST_KINDS_JSON: &str = include_str!("../../🧪️oracle/🔣️component.json");

    #[semio_framework_async_macros::async_test]
    async fn oracle_mutation_kinds_law_matches_manifest_catalog() {
        assert!(MANIFEST_KINDS_JSON.contains("\"kinds\""), "manifest must declare a mutationCatalogs[].kinds array");
        for kind in KINDS {
            let needle = format!("\"{kind}\"");
            assert!(MANIFEST_KINDS_JSON.contains(&needle), "🧪️oracle/🔣️component.json must declare kind {kind:?} (KINDS and the manifest must be kept byte-identical)");
        }
    }
    //#endregion kinds_law

    //#region op_text_binary_roundtrip_law
    /// 🧪️ F6: `protocol::OpText`/`OpBinary` LAW, exercised for every variant incl. `SetSnapshot`'s
    /// multi-page payload -- both text (`print_op`/`parse_op`) and binary
    /// (`encode_op`/`decode_op`) sides.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let many = PdfSnapshot {
            schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
            pages: vec![PageDoc { width: 300.5, height: 400.25, text: "hello world".into() }, PageDoc { width: 1.0, height: 2.0, text: String::new() }, PageDoc { width: 0.0, height: 841.89, text: "a (parenthesised, comma'd) line".into() }],
        };
        let cases = vec![PdfMutation::NoMutation, PdfMutation::SetSnapshot { snapshot: snap(300.5, 400.25, "hello world") }, PdfMutation::SetSnapshot { snapshot: many }];
        for m in cases {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must not contain a newline: {printed:?}");
            let parsed = PdfMutation::parse_op(&printed).expect("parse_op must accept its own print_op output");
            assert_eq!(parsed, m, "parse_op(print_op(m)) must equal m for {m:?}");

            let encoded = m.encode_op().expect("encode_op must succeed");
            let decoded = PdfMutation::decode_op(&encoded).expect("decode_op must accept its own encode_op output");
            assert_eq!(decoded, m, "decode_op(encode_op(m)) must equal m for {m:?}");
        }
    }
    //#endregion op_text_binary_roundtrip_law
}
//#endregion Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🦀️component.rs"]
    mod tests_set_snapshot_shrinks_the_page_to_a5_and_rewrites_its_text;
}
//#endregion 🧪️FixtureTests
