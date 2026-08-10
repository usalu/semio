//! 🎹️ BinaryComposer (final, artifact-level) — the union over every standard's composer entries.
//! `compose` picks the one entry whose `writes` matches the requested target dialect: "read
//! artifacts of different standards/subsets, write one specific standard+subset." `register`
//! feeds every entry into the OS-wide typed io registry; called once from `🔌️plugin/🔧️setup`.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::binary::standards::v_raw::composer as v_raw;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

/// 🎹️ Every composer entry this artifact can serve, across all its standards.
pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v_raw::entries().iter().collect()).as_slice()
}

/// 🎯️ Compose into exactly one target dialect from a set of (possibly foreign-dialect) sources.
pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("BinaryComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

/// 📌️ Registers every entry into the OS-wide typed io registry. Called once from `🔌️plugin/🔧️setup`.
pub fn register() {
    register_composer_entries(v_raw::entries());
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{StandardId, SubsetId, IoPayload, IoDirection, IoKey, io_resolve};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    #[test]
    fn compose_direct_round_trips_a_native_binary_payload() {
        let snapshot = crate::artifacts::binary::standards::v_raw::engine::empty_binary_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let sources = [ErasedComposeSource { dialect: DIALECT, payload: IoPayload::Binary(bytes) }];
        let composed = compose(DIALECT, &sources).expect("compose");
        assert_eq!(composed.dialect, DIALECT);
        assert!(matches!(composed.payload, IoPayload::Binary(_)));
    }

    #[test]
    fn register_then_resolve_through_the_typed_registry_finds_this_composer() {
        register();
        let key = IoKey {
            artifact_kind: "s.stdio.binary".into(),
            standard: "raw".into(),
            subset: "*".into(),
            direction: IoDirection::Import,
            format_kind: "s.stdio.binary".into(),
            format_standard: "raw".into(),
            format_subset: "*".into(),
        };
        let entry = io_resolve(&key).expect("resolve");
        assert_eq!(entry.writes, DIALECT);
    }
}
//#endregion 🧪️Tests
