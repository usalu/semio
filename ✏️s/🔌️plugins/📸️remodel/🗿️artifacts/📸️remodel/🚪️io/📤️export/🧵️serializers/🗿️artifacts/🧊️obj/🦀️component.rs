//! remodel -> obj
use crate::artifacts::remodel::WatertightReportSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &WatertightReportSnapshot) -> Result<ObjSnapshot, store::TextError> {
    let bytes = <WatertightReportSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<WatertightReportSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::obj::engine::decode_obj(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &WatertightReportSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::obj::engine::encode_obj(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
