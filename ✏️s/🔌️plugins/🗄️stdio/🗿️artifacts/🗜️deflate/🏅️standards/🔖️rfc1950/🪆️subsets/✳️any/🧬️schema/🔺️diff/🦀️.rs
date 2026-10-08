//! 🔺️ DeflateDiff — sparse per-field RFC1950 container diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `DeflateSnapshot` has
//! no keyed/indexed collections (it is five scalar/weak fields), so there is no `XsDiff` triple
//! here -- every field is `Option<T>` (nullable `dict_id` is the tri-state `Option<Option<u32>>`)
//! and absorb is plain last-write-wins per field, exactly as the recipe's "Scalars: LWW" rule
//! prescribes for artifacts with no strong entities.

use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::MutationDiff;
// 🧭️ `DiffAlgebra` lives at `command::DiffAlgebra` (not re-exported bare at the `protocol` crate
// root the way `MutationDiff` is) -- see `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`.
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.deflate`. No `snapshot: Option<DeflateSnapshot>` full-replace slot --
/// every mutation leaf names exactly the fields it changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.diff")]
pub struct DeflateDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub compression_method: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window_bits: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub compression_level_hint: Option<DeflateLevelHint>,
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = dictionary cleared, `Some(Some(id))` =
    /// dictionary set/changed to `id`.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<Option<u32>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
}

impl MutationDiff<DeflateSnapshot> for DeflateDiff {
    fn apply(&self, base: &DeflateSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DeflateSnapshot> {
        let mut next = base.clone();
        if let Some(v) = self.compression_method {
            next.compression_method = v;
        }
        if let Some(v) = self.window_bits {
            next.window_bits = v;
        }
        if let Some(v) = self.compression_level_hint {
            next.compression_level_hint = v;
        }
        if let Some(v) = self.dict_id {
            next.dict_id = v;
        }
        if let Some(v) = &self.payload {
            next.payload = v.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.compression_method.is_some() {
            self.compression_method = other.compression_method;
        }
        if other.window_bits.is_some() {
            self.window_bits = other.window_bits;
        }
        if other.compression_level_hint.is_some() {
            self.compression_level_hint = other.compression_level_hint;
        }
        if other.dict_id.is_some() {
            self.dict_id = other.dict_id;
        }
        if other.payload.is_some() {
            self.payload = other.payload;
        }
    }
}

impl DiffAlgebra<DeflateSnapshot> for DeflateDiff {
    fn inverse(&self, base: &DeflateSnapshot) -> Self {
        DeflateDiff {
            compression_method: self.compression_method.map(|_| base.compression_method),
            window_bits: self.window_bits.map(|_| base.window_bits),
            compression_level_hint: self.compression_level_hint.map(|_| base.compression_level_hint),
            dict_id: self.dict_id.map(|_| base.dict_id),
            payload: self.payload.as_ref().map(|_| base.payload.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.compression_method.is_none() && self.window_bits.is_none() && self.compression_level_hint.is_none() && self.dict_id.is_none() && self.payload.is_none()
    }
}

/// 🧩 Builds a set-compression-params diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_compression_params(method: u8, window_bits: u8, level_hint: DeflateLevelHint) -> DeflateDiff {
    DeflateDiff { compression_method: Some(method), window_bits: Some(window_bits), compression_level_hint: Some(level_hint), ..Default::default() }
}
/// 🧩 Builds a set-preset-dictionary diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_preset_dictionary(dict_id: Option<u32>) -> DeflateDiff {
    DeflateDiff { dict_id: Some(dict_id), ..Default::default() }
}
/// 🧩 Builds a set-payload diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_payload(payload: Vec<u8>) -> DeflateDiff {
    DeflateDiff { payload: Some(payload), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️DemoCases
/// 🧪️ Representative `DeflateDiff` cases built declaratively: the empty diff, every field (incl. the `dict_id` tri-state both ways), and the
/// payload and preset-dictionary builders.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DeflateDiff> {
    vec![
        DeflateDiff::default(),
        DeflateDiff { compression_method: Some(9), window_bits: Some(6), compression_level_hint: Some(DeflateLevelHint::Maximum), dict_id: Some(Some(0xDEAD_BEEF)), payload: Some(b"demo-cases-b-different-longer-payload".to_vec()) },
        DeflateDiff { compression_level_hint: Some(DeflateLevelHint::Fastest), dict_id: Some(None), ..Default::default() },
        diff_set_preset_dictionary(None),
        diff_set_payload(Vec::new()),
    ]
}
//#endregion 🔖️DemoCases

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DeflateDiff` — the derive path
/// (`#[derive(dsl::DslDiff)]`) is NOT usable here: `dict_id: Option<Option<u32>>` is a tri-state
/// field (`f6-recon-report.md` §3b — `dsl_derive::classify_field` peels exactly one `Option<..>`
/// layer before binding, and there is no `impl<T: DslField> DslField for Option<T>` anywhere in
/// the `dsl` crate, so the REMAINING `Option<u32>` after that one peel is structurally
/// unbindable). Confirmed via real `cargo check`:
/// ```text
/// error[E0277]: the trait bound `std::option::Option<u32>: DslField` is not satisfied
///    --> …/🔺️diff/🦀️.rs:37:17   (pub dict_id: Option<Option<u32>>)
/// ```
/// This is the SAME hand-rolled path `GifDiff` uses for its own tri-state fields (`gct`,
/// `loop_count`, `GifFrameDiff`'s `lct`/`transparent_index`/`plain_text`) — the primitive set
/// below (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/`encode_option`/
/// `decode_option`) is copied verbatim from that pilot's grammar template
/// (`f6-recon-report.md` §5), since `DeflateDiff` needs no enum-tag or collection-triple
/// machinery (no data-carrying enum, no keyed collection anywhere in this artifact).
///
/// **Grammar**: one space-separated `name=value` token per changed top-level field (a field
/// absent from the line = unchanged). Bytes (`payload`) are lowercase hex — same local idiom
/// `DeflateSnapshot`'s own `ArtifactDsl` impl above already uses, and the same reason `GifDiff`
/// gives (no external base64 dep, no escaping needed at this grammar layer). `compression_level_hint`
/// uses a single-letter tag (`f`/`a`/`d`/`m`, mirroring `GifDisposal`'s `enc_disposal` pattern).
/// The tri-state `dict_id` and the plain-optional `payload` both use the uniform
/// `[0]`=unchanged-inner-None / `[1,<T>]`=inner-Some(T) tag via `encode_option`/`decode_option`
/// (note: `payload`'s own `Option<Vec<u8>>` is the DIFF's "field changed at all" wrapper, not a
/// second tri-state — `DeflateSnapshot::payload` itself is a bare, never-nullable `Vec<u8>`, so
/// `payload`'s token is only present when the field changed, and its value is always hex, never
/// itself optional).
///
/// Worked example: `compression-method=9 window-bits=6 level=m dict-id=[1,3735928559] payload=` (empty
/// payload prints as a zero-length hex string after `=`).
//#region 🔖️Primitives









//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs


//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
