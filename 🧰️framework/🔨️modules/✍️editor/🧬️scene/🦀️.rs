//! 🎬️ Typed Editor updates use one declared General intrinsic frame and owned projection.
use super::{ByteRangeJson, DiagnosticJson, EditorError, EditorErrorKind, EditorSettingsJson, PlaceholderJson, SelectableSpanJson, SemanticTokenJson};
use semio_framework_value::{FromValue, NativeDecodeControl};
use semio_framework_value_derive::ToValue;
pub use pack::intrinsic::IntrinsicFormat;
pub use pack::record::DecodeOptions;

/// 🧭️ Presence changes hover explicitly; omission leaves the existing hover unchanged.
#[derive(Clone, Debug, FromValue, ToValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(super) enum Hover { Clear, Range { start: usize, end: usize } }

/// 🔎️ Hover and selection occurrence lists are typed arrays.
#[derive(Clone, Debug, FromValue, ToValue)]
#[value(deny_unknown_fields)]
pub(super) struct Occurrences {
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) hover: Option<Vec<ByteRangeJson>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) selection: Option<Vec<ByteRangeJson>>,
}
/// 📷️ The Editor camera has one scrolling axis.
#[derive(Clone, Debug, FromValue, ToValue)]
#[value(deny_unknown_fields)]
pub(super) struct Camera { pub(super) y: f64 }
/// 🪧️ Present overlay values update their declared visual feature.
#[derive(Clone, Debug, FromValue, ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Overlays {
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) dead_line_y: Option<f64>,
}
/// ✍️ An opaque owned update is constructed only by canonical typed decoding.
#[derive(Clone, Debug, FromValue, ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct EditorScene {
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) buffer: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) selection: Option<ByteRangeJson>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) tokens: Option<Vec<SemanticTokenJson>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) diagnostics: Option<Vec<DiagnosticJson>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) placeholders: Option<Vec<PlaceholderJson>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) occurrences: Option<Occurrences>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) extra_carets: Option<Vec<usize>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) selectable_spans: Option<Vec<SelectableSpanJson>>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) settings: Option<EditorSettingsJson>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) camera: Option<Camera>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) overlays: Option<Overlays>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub(super) hover: Option<Hover>,
}
/// 📦️ Decodes only the caller-declared physical frame under cumulative caller authority.
pub fn decode(bytes: &[u8], format: IntrinsicFormat, options: &DecodeOptions, control: &mut NativeDecodeControl<'_>) -> Result<EditorScene, EditorError> {
 let value=pack::intrinsic::decode(bytes,format,options,control).map_err(|error|EditorError::from_cause(EditorErrorKind::Pack,error))?;
 EditorScene::from_value_controlled(&value,control).map_err(|error|EditorError::from_cause(EditorErrorKind::Scene,error))
}
/// 🧾️ Parses a canonical scene document with duplicate members refused.
pub fn from_json(json: &str, control: &mut NativeDecodeControl<'_>) -> Result<EditorScene, EditorError> {
 semio_framework_pack_json::from_json_str_controlled(json,semio_framework_pack_json::JsonMemberPolicy::Reject,control).map_err(|error|EditorError::from_cause(EditorErrorKind::Scene,error))
}
