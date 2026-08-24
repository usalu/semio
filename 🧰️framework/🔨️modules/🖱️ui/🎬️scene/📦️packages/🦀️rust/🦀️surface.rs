//! 🗺️ `encode`/`decode` — the bridge between a typed [`crate::SceneDoc`] payload and
//! `ui_contract`'s opaque `SurfaceProps.doc` (`SurfaceDoc.bytes`, a pack-encoded blob the contract
//! crate never parses; see `ui_contract`'s own `🦀️surface.rs` header). This is the ONE place in this
//! crate that names `ui_contract::{SurfaceKind, SurfaceProps, SurfaceDoc}` — everything in
//! `🦀️scenes.rs`/`🦀️math.rs` stays product/renderer-neutral.
//!
//! 🚫️async: E6 sync payload encoding — no `async fn` anywhere in this module.

use crate::pack::PackError;
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

//#region 🔖️SurfaceEncodeError
#[derive(Debug)]
pub enum SurfaceEncodeError {
    Pack(PackError),
    Payload(Vec<u8>),
    Schema(&'static str),
}

impl std::fmt::Display for SurfaceEncodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(error) => write!(formatter, "surface payload encode failed: {error}"),
            Self::Payload(bytes) => write!(formatter, "surface payload exceeds fixed capacity with {} bytes", bytes.len()),
            Self::Schema(schema) => write!(formatter, "surface schema exceeds fixed capacity: {schema}"),
        }
    }
}

impl std::error::Error for SurfaceEncodeError {}
//#endregion 🔖️SurfaceEncodeError

//#region 🔖️EncodeDecode
/// 📦️ Packs `doc` and stamps `doc_schema = T::SCHEMA`, leaving every other `SurfaceProps` field
/// (`surface_id`, `controller_id`, `pane_id`, `binding_id`, `domain_id`, `domain_granularity_id`) at
/// its default — the caller (the surface's own owner, which knows those identities) fills them in.
/// Admission is fallible: codec failures and fixed schema/payload capacity failures return their
/// rejected owner instead of aborting or truncating.
pub fn encode<T: SceneDoc>(kind: SurfaceKind, doc: &T) -> Result<SurfaceProps, SurfaceEncodeError> {
    let bytes = doc.encode_pack().map_err(SurfaceEncodeError::Pack)?;
    let doc_schema = ui_contract::UiText::try_from_str(T::SCHEMA).ok_or(SurfaceEncodeError::Schema(T::SCHEMA))?;
    let bytes = ui_contract::UiFixedBytes::try_from_vec(bytes).map_err(SurfaceEncodeError::Payload)?;
    Ok(SurfaceProps { kind, doc_schema, doc: SurfaceDoc { bytes }, ..Default::default() })
}

/// 📦️ Verifies `props.doc_schema == T::SCHEMA` before attempting to decode — a stale/foreign schema
/// returns [`SurfaceDocError::SchemaMismatch`] rather than feeding mismatched bytes to `T`'s decoder
/// (which could otherwise misparse rather than cleanly fail).
pub fn decode<T: SceneDoc>(props: &SurfaceProps) -> Result<T, SurfaceDocError> {
    if props.doc_schema.as_str() != T::SCHEMA {
        return Err(SurfaceDocError::SchemaMismatch { expected: T::SCHEMA, actual: props.doc_schema.to_string() });
    }
    T::decode_pack(props.doc.bytes.as_slice()).map_err(SurfaceDocError::Decode)
}
//#endregion 🔖️EncodeDecode

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Board2dScene, TableScene, World3dScene};

    #[test]
    fn table_scene_round_trips_byte_identical() {
        let scene = TableScene::base("[{\"id\":\"name\"}]", "[]");
        let props = encode(SurfaceKind::Table, &scene).expect("bounded fixture");
        assert_eq!(props.doc_schema.as_str(), "table@1");
        let back: TableScene = decode(&props).expect("decode");
        assert_eq!(scene, back);
    }

    #[test]
    fn board_scene_round_trips_with_absent_optional_fields() {
        let scene = Board2dScene::base("{}".into(), "{}".into(), true);
        let props = encode(SurfaceKind::Board2d, &scene).expect("bounded fixture");
        let back: Board2dScene = decode(&props).expect("decode");
        assert_eq!(scene, back);
    }

    #[test]
    fn world_scene_round_trips_with_absent_optional_fields() {
        let scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        let props = encode(SurfaceKind::World3d, &scene).expect("bounded fixture");
        let back: World3dScene = decode(&props).expect("decode");
        assert_eq!(scene, back);
    }

    #[test]
    fn wrong_schema_errs_never_panics() {
        let scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        let mut props = encode(SurfaceKind::World3d, &scene).expect("bounded fixture");
        props.doc_schema = ui_contract::UiText::try_from_str("world3d@99").expect("bounded fixture");
        let result = decode::<World3dScene>(&props);
        assert!(matches!(result, Err(SurfaceDocError::SchemaMismatch { .. })));
    }
}
