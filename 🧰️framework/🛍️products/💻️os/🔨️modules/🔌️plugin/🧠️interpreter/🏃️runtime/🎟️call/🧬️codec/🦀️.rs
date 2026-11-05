//! 🧬️ Current pure codec envelopes retain one actual Source-backed input before crossing the real component ABI.
use crate::{OwnedCodecInput,OwnedSnapshotInput,OwnedSemioExport,OwnedRuntime};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
impl OwnedRuntime{
 /// 📥️ Uses the component's current document codec shape without creating an unowned temporary buffer.
 pub fn encode_original_codec_input_into(export:OwnedSemioExport,input:&OwnedCodecInput<'_>,control:&mut NativeEncodeControl<'_>,output:&mut Option<Vec<u8>>)->Result<(),ValueError>{if !matches!(export,OwnedSemioExport::PackSchemaHash|OwnedSemioExport::Genesis|OwnedSemioExport::PrintMirror|OwnedSemioExport::ApplyOps|OwnedSemioExport::ReplayEnvelopes){return Err(invalid())}crate::operation::input::encode_json_input_into(input,control,output)}
 /// 🪶️ Keeps semantic snapshot limits and logical native caps as data in the exact current ABI envelope.
 pub fn encode_original_snapshot_input_into(export:OwnedSemioExport,input:&OwnedSnapshotInput<'_>,control:&mut NativeEncodeControl<'_>,output:&mut Option<Vec<u8>>)->Result<(),ValueError>{if !matches!(export,OwnedSemioExport::SqliteSchema|OwnedSemioExport::SqliteExport|OwnedSemioExport::SqliteImport){return Err(invalid())}crate::operation::input::encode_json_input_into(input,control,output)}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original codec input requires its declared pure codec export")}
