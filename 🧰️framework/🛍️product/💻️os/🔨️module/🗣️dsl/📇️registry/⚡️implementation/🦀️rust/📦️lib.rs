//! 📇️ `dsl_registry` — W1 foundation of the DSL registry unification (design ruling B-R3): a real
//! (non-demonstration) `pack_cli::SchemaResolver` fan-in, so `pack_cli`'s CLI functions can resolve
//! real app schemas without `pack_cli` itself ever depending on an app crate (the orphan-rule-shaped
//! reason `SchemaResolver` is a trait defined in `pack_cli`, implemented here instead). This crate is
//! the one place in the workspace allowed to depend on many app `🗣️dsl`/`🔧️op` crates at once —
//! every other crate in the `dsl_*`/`pack_*`/`protocol_*` family stays app-dependency-free by design.
//!
//! 🚧️ W1 scope: proves the mechanism on two apps (writer, note) — their document schema AND their
//! `"<doc-schema>#diff"` diff schema (the first two real `#[derive(dsl::DslDiff)]` types, see
//! `writer_op::WriterDiff`/`note_op::NoteDiff`). Full fan-in across every real app schema (the
//! `🧪️fixture-sweep` crate's dev-dependency list is the template for what that eventually looks
//! like) is deferred to a later wave — tracked as the W8 "dsl_registry completeness assertion" item
//! in `.claude/plans/the-final-goal-for-jolly-spindle.md`. Add one app's `🗣️dsl`/`🔧️op` pair to
//! `Cargo.toml` + [`full_resolver`] per follow-up; nothing else in this crate needs to change shape.

use pack_cli::SchemaResolver;
use std::collections::HashMap;

//#region 🔖️Registry
/// @emoji 📇️ A `SchemaResolver` backed by a fixed table of `(schema id, RecordSpec constructor)`
/// pairs — [`full_resolver`] is the only constructor real callers use; the struct itself stays
/// public so a caller that wants a narrower/custom table (e.g. a test double) can build one by hand.
pub struct FullResolver {
    schemas: HashMap<&'static str, fn() -> dsl_schema::RecordSpec>,
}

impl SchemaResolver for FullResolver {
    fn resolve(&self, schema: &str) -> Option<dsl_schema::RecordSpec> {
        self.schemas.get(schema).map(|spec_fn| spec_fn())
    }

    fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.schemas.keys().map(|s| s.to_string()).collect();
        names.sort_unstable();
        names
    }
}

/// @emoji 🏗️ Builds the real fan-in resolver — the schema ids follow the schema lattice's own
/// convention (`"<doc-schema>"` for a document, `"<doc-schema>#diff"` for its diff, design ruling
/// B-R4) so a future `dsl_registry`-driven `pack diff --schema writer.document#diff` (or similar)
/// resolves the diff's own grammar, not the document's.
pub fn full_resolver() -> FullResolver {
    let mut schemas: HashMap<&'static str, fn() -> dsl_schema::RecordSpec> = HashMap::new();
    schemas.insert(writer::WRITER_DOCUMENT_SCHEMA, writer::WriterProjection::__dsl_spec);
    schemas.insert("writer.document#diff", writer_op::WriterDiff::__dsl_diff_spec);
    schemas.insert(note::NOTE_DOCUMENT_SCHEMA, note::NoteDocument::__dsl_spec);
    schemas.insert("note.document#diff", note_op::NoteDiff::__dsl_diff_spec);
    FullResolver { schemas }
}
//#endregion 🔖️Registry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_resolver_resolves_every_registered_schema() {
        let resolver = full_resolver();
        for name in ["writer.document", "writer.document#diff", "note.document", "note.document#diff"] {
            assert!(resolver.resolve(name).is_some(), "expected '{name}' to resolve");
        }
        assert!(resolver.resolve("never-registered").is_none());
    }

    #[test]
    fn full_resolver_names_are_sorted_and_complete() {
        let resolver = full_resolver();
        assert_eq!(resolver.names(), vec!["note.document", "note.document#diff", "writer.document", "writer.document#diff"]);
    }

    /// 🧬️ The resolved `RecordSpec` must actually match the real derive-generated one — not just
    /// resolve to *some* spec — proven by encoding a real value against the resolved spec and
    /// decoding it back through the type's own `DocumentPack`/`DiffCodec` impl.
    #[test]
    fn resolved_writer_document_spec_matches_the_real_type() {
        let resolver = full_resolver();
        let spec = resolver.resolve("writer.document").expect("writer.document must resolve");
        let document = writer::WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into() };
        let record = document.__dsl_to_record();
        let bytes = pack_cli_encode_for_test(&spec, &record);
        let (decoded_record, _report) = pack::decode_document(&bytes, &spec, &pack::DecodeOptions::default()).expect("decode against resolved spec");
        let decoded = writer::WriterProjection::__dsl_from_record(&decoded_record).expect("__dsl_from_record");
        assert_eq!(decoded, document);
    }

    #[test]
    fn resolved_writer_diff_spec_matches_the_real_diff_codec() {
        use protocol::DiffCodec;
        let resolver = full_resolver();
        let spec = resolver.resolve("writer.document#diff").expect("writer.document#diff must resolve");
        let diff = writer_op::WriterDiff { text: Some("hi".into()), document: None };
        assert_eq!(spec.keyword, writer_op::WriterDiff::__dsl_diff_spec().keyword, "resolved spec must be the real diff spec");
        let bytes = diff.encode_diff().expect("encode_diff");
        let decoded = writer_op::WriterDiff::decode_diff(&bytes).expect("decode_diff");
        assert_eq!(decoded, diff);
    }

    fn pack_cli_encode_for_test(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue) -> Vec<u8> {
        pack::encode_document(spec, record, &pack::EncodeOptions::default()).expect("encode_document")
    }
}
//#endregion 🧪️Tests
