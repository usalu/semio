//! 🧩️ Lift file: 3 item(s), about 45 lines, of `📜️declaration/🧩️app/🦀️.rs` that belong to crate `document-media` (WP3).
//!
//! WP2 owns the source file and cuts the items here (MediaArtifact, whole_document_media, impl semio_framework_job::Orig); WP3 wires this file with one `mod` line in the crate root. Until `document-media` exists, the source module re-exports the items with `pub use`.
use semio_framework_plugin_decl_app::media_projection_error;
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework_ui_contract::*;
use semio_framework::MediaWireFormat;
use semio_framework::Media;
use semio_framework::MediaPayload;
use semio_framework::MediaType;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_plugin_decl_app::MEDIA_SCHEMA_MISMATCH_CODE;
use semio_framework_plugin_decl_app::MediaArtifactDescriptor;
use semio_framework_plugin_decl_app::MediaArtifactError;

/// 🎞️ Rust mirror of WIT's `media-artifact` record (`framework/wit/📜️world.wit`): `descriptor` is the
/// parsed `descriptor-json`, `data` the sibling raw-bytes field. Deliberately separate from
/// `mesh::Media` (which pairs a `MediaType` with a `MediaPayload` for the declared-port workflow
/// via `export_media`/`import_media`) — `consume-media`/`produce-media` operate at the whole-document
/// level, not a specific `MediaPortSpec`, so `PluginApp::{consume_media, produce_media}` below default
/// to a document passthrough rather than requiring any `media_ports()` declaration at all.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct MediaArtifact {
    pub descriptor: MediaArtifactDescriptor,
    pub data: Vec<u8>,
}

/// 🎞️ A whole document on a media edge (`artifact:out`): the recursive document archive (parent pack + `.spr` + owned members) as
/// `pk:` base64 text on the `Document{schema}` wire — the `Structured{schema, json}` convention of `export_media`, which every host
/// carries losslessly as text.
pub async fn whole_document_media(port: &str, schema: String, archive: &protocol::DocumentArchivePack) -> Result<MediaArtifact, MediaArtifactError> {
    let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
    Ok(MediaArtifact {
        descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema }, blob_hash: None },
        data: store::pack_rt::pack_value_to_base64(&bytes).into_bytes(),
    })
}

impl MediaArtifact {
    /// 🎞️ Owns exact intrinsic bytes and descriptor text from one borrowed media owner.
    pub fn encode_intrinsic(port:&str,media_type:MediaType,schema:&str,value:&DslValue,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_ ,'_>)->Result<Self,MediaArtifactError>{
        let (port,schema,data)=encode_intrinsic_parts(port,schema,value,owner)?;
        Ok(Self{descriptor:MediaArtifactDescriptor{edge_id:None,port_id:Some(port),kind_id:None,media_type:Some(media_type),wire:MediaWireFormat::Intrinsic{schema},blob_hash:None},data})
    }

    /// 🎞️ Reads one full inline intrinsic payload under the caller's import control.
    pub fn decode_intrinsic(&self,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Media,MediaArtifactError>{
        let MediaWireFormat::Intrinsic{schema}=&self.descriptor.wire else{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"media wire is not intrinsic").into())};
        if self.descriptor.blob_hash.is_some(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"intrinsic media requires inline bytes").into())}
        let media_type=self.descriptor.media_type.ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"intrinsic media requires a declared media type"))?;
        let schema=control.copy_text(schema)?;let value=store::pack_rt::decode_wire_value_controlled(&self.data,&store::PackDecodeOptions::default(),control).map_err(|error|MediaArtifactError::Value(error.into_value_error()))?;
        Ok(Media{media_type,payload:MediaPayload::Intrinsic{schema,value}})
    }
}

include!("../../../🎞️media/🛫️intrinsic/🦀️.rs");

include!("../../../🎞️media/💥️fault/🦀️.rs");
