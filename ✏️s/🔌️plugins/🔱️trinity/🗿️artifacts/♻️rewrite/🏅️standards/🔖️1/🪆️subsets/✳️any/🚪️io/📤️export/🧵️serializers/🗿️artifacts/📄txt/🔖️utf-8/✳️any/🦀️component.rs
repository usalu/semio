//! rewrite -> txt
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &RewriteSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot {
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        text: <RewriteSnapshot as store::DocumentDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.text.into_bytes())
}
