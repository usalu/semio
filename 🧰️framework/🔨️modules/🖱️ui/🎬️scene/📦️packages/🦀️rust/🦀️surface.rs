//! 🗺️ `encode`/`decode` — the bridge between a typed [`crate::SceneDoc`] payload and
//! `ui_contract`'s opaque `SurfaceProps.doc` (`SurfaceDoc.bytes`, a pack-encoded blob the contract
//! crate never parses; see `ui_contract`'s own `🦀️surface.rs` header). This is the ONE place in this
//! crate that names `ui_contract::{SurfaceKind, SurfaceProps, SurfaceDoc}` — everything in
//! `🦀️scenes.rs`/`🦀️math.rs` stays product/renderer-neutral.
//!
//! 🚫️async: E6 sync payload encoding — no `async fn` anywhere in this module.

use crate::pack::{self, PackError};
use crate::SceneDoc;
use ui_contract::{SurfaceDoc, SurfaceKind, SurfaceProps};

//#region 🔖️SurfaceDocError
/// 🚨️ Everything that can go wrong decoding a `SurfaceProps.doc` back into a typed `T: SceneDoc` —
/// a stale/foreign `doc_schema` or a malformed byte payload. Never panics; see this crate's own
/// `pack` module for why the codec itself cannot panic on truncated/invalid input.
#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceDocError {
    /// 🏷️ `doc_schema` on the `SurfaceProps` did not match `T::SCHEMA` — e.g. a `TableScene` decoder
    /// handed a `"world-3d@1"` payload, or a schema version bump the caller hasn't migrated past.
    SchemaMismatch { expected: &'static str, actual: String },
    /// 🧵️ `doc.bytes` did not decode as a well-formed `T` under this crate's pack codec.
    Decode(PackError),
}

impl std::fmt::Display for SurfaceDocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaMismatch { expected, actual } => write!(f, "surface doc schema mismatch: expected {expected:?}, got {actual:?}"),
            Self::Decode(error) => write!(f, "surface doc decode failed: {error}"),
        }
    }
}
impl std::error::Error for SurfaceDocError {}
//#endregion 🔖️SurfaceDocError

//#region 🔖️EncodeDecode
/// 📦️ Packs `doc` and stamps `doc_schema = T::SCHEMA`, leaving every other `SurfaceProps` field
/// (`surface_id`, `controller_id`, `pane_id`, `binding_id`, `domain_id`, `domain_granularity_id`) at
/// its default — the caller (the surface's own owner, which knows those identities) fills them in.
/// Infallible in practice: every `SceneDoc` struct in `🦀️scenes.rs` is plain data this crate's own
/// codec fully covers (`serialize_map`/enum-variant payloads are the only unsupported shapes, and no
/// `SceneDoc` struct uses either) — the `expect` documents that invariant rather than silently
/// swallowing a codec bug a future `SceneDoc` impl might introduce.
pub fn encode<T: SceneDoc>(kind: SurfaceKind, doc: &T) -> SurfaceProps {
    let bytes = pack::to_bytes(doc).expect("SceneDoc payloads are plain data this crate's pack codec fully covers");
    SurfaceProps { kind, doc_schema: T::SCHEMA.into(), doc: SurfaceDoc { bytes }, ..Default::default() }
}

/// 📦️ Verifies `props.doc_schema == T::SCHEMA` before attempting to decode — a stale/foreign schema
/// returns [`SurfaceDocError::SchemaMismatch`] rather than feeding mismatched bytes to `T`'s decoder
/// (which could otherwise misparse rather than cleanly fail).
pub fn decode<T: SceneDoc>(props: &SurfaceProps) -> Result<T, SurfaceDocError> {
    if props.doc_schema != T::SCHEMA {
        return Err(SurfaceDocError::SchemaMismatch { expected: T::SCHEMA, actual: props.doc_schema.clone() });
    }
    pack::from_bytes(&props.doc.bytes).map_err(SurfaceDocError::Decode)
}
//#endregion 🔖️EncodeDecode

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TableScene, World3dScene};

    #[test]
    fn table_scene_round_trips_byte_identical() {
        let scene = TableScene::base("[{\"id\":\"name\"}]", "[]");
        let props = encode(SurfaceKind::Table, &scene);
        assert_eq!(props.doc_schema, "table@1");
        let back: TableScene = decode(&props).expect("decode");
        assert_eq!(scene, back);
    }

    #[test]
    fn wrong_schema_errs_never_panics() {
        let scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        let mut props = encode(SurfaceKind::World3d, &scene);
        props.doc_schema = "world3d@99".into();
        let result = decode::<World3dScene>(&props);
        assert!(matches!(result, Err(SurfaceDocError::SchemaMismatch { .. })));
    }
}
