//! 🧬️ PdfMutation — document mutation dispatch.
//!
//! 🧪️ F6 (real `OpText`/`OpBinary`): `PdfMutation`'s only enum-shaped payload is `SetSnapshot`'s
//! whole `PdfSnapshot`, and `PdfSnapshot`'s tree has zero data-carrying enums, so per
//! f6-recon-report.md §3 `#[derive(dsl::DslOps)]` derives clean (verified via `cargo check`,
//! zero pdf-scoped errors) giving `dsl::DslVariants` for free. Per P6
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:914-915`) `DslOps`
//! never emits `OpText`/`OpBinary` itself -- those are the §2 handcrafted boilerplate wrapper
//! below (identical shape to `BinaryMutation`/`GifMutation`), replacing the prior
//! `serde_json`-based stubs.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::{diff_set_snapshot, PdfDiff};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pdf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
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
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on `PdfMutation` derives clean (no data-carrying enum
/// anywhere in `PdfSnapshot`'s tree) and gives `dsl::DslVariants` for free, but per P6
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:914-915`) `DslOps`
/// NEVER emits `OpText`/`OpBinary` — those are always handcrafted. This is the exact
/// boilerplate wrapper from f6-recon-report.md §2 (verbatim shape as `BinaryMutation`/
/// `GifMutation`), replacing the prior `serde_json`-based stubs.
impl protocol::OpText for PdfMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
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
        PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width, height, text: text.into() } }
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
        let base = snap(612.0, 792.0, "base");
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
            let matches = match (*kind, variant) {
                ("no-mutation", PdfMutation::NoMutation) => true,
                ("set-snapshot", PdfMutation::SetSnapshot { .. }) => true,
                _ => false,
            };
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
    /// nested-struct payload -- both text (`print_op`/`parse_op`) and binary
    /// (`encode_op`/`decode_op`) sides.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};
        let cases = vec![PdfMutation::NoMutation, PdfMutation::SetSnapshot { snapshot: snap(300.5, 400.25, "hello world") }];
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
