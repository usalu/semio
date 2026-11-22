//! 📥 Submits a complete, bounded natural file to the fresh native program instance.
use super::NativeMediaDeclaration;
use super::super::{KernelClient, wasm_program_exchange::{app_frame_error_message, exchange, next_seq}};
use protocol::{AppCommand, AppFrame};
use semio_framework::NATURAL_FILE_PORT_ID;
use semio_framework_value::ToValue;
use semio_plugin::app::{MediaArtifactDescriptor, MediaWireFormat};

use semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

pub(crate) async fn import_media(
    client: &KernelClient, instance: u32, declaration: &NativeMediaDeclaration, bytes: Vec<u8>,
) -> Result<(), String> {
    let sequence = next_seq();
    let command = import_command(sequence, declaration, bytes)?;
    let outcome = exchange(client, instance, vec![command]).await.map_err(|error| error.to_string())?;
    match super::driver::one_save_reply(outcome.frames, sequence, app_frame_error_message)? {
        AppFrame::Done { in_reply_to } if in_reply_to == sequence => Ok(()),
        _ => Err("Open File did not receive the exact import completion".into()),
    }
}

impl NativeMediaDeclaration {
    /// 📐 Reserves the exact descriptor and largest sequence before any file bytes are read.
    pub async fn natural_file_payload_limit(&self) -> Result<usize, String> {
        let empty = protocol::os_spr::encode_app_command(&natural_file_command(u64::MAX, self, Vec::new())).await
            .map_err(|fault| fault.describe())?;
        let length_bytes = (usize::BITS - COMMAND_MAXIMUM_BYTES.leading_zeros()).div_ceil(7) as usize;
        Ok(COMMAND_MAXIMUM_BYTES.saturating_sub(empty.byte_len() + length_bytes - 1))
    }
}

/// 🧾 Addresses a bounded natural file using its manifest-owned binary descriptor.
fn import_command(sequence: u64, declaration: &NativeMediaDeclaration, bytes: Vec<u8>) -> Result<AppCommand, String> {
    if !declaration.reference || bytes.is_empty() || bytes.len() > COMMAND_MAXIMUM_BYTES {
        return Err("Open File requires the declared natural binary format within its byte limit".into());
    }
    Ok(natural_file_command(sequence, declaration, bytes))
}

fn natural_file_command(sequence: u64, declaration: &NativeMediaDeclaration, bytes: Vec<u8>) -> AppCommand {
    let port = NATURAL_FILE_PORT_ID;
    let descriptor = MediaArtifactDescriptor {
        edge_id: None, port_id: Some(port.into()), kind_id: Some(declaration.format_kind.clone()),
        media_type: None, wire: MediaWireFormat::Binary { format_kind: declaration.format_kind.clone() }, blob_hash: None,
    };
    let descriptor = protocol::pack_rt::encode_wire_value(&descriptor.to_value());
    AppCommand::MediaIn { seq: sequence, port: port.into(), descriptor, data: bytes }
}
