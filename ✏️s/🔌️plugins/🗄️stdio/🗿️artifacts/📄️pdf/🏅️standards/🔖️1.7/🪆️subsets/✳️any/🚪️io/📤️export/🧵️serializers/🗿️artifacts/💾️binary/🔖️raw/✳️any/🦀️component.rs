//! Serialize stdio.pdf (1.7) to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

pub async fn register() {}

pub async fn serialize(from: &PdfSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::pdf::standards::v1_7::subsets::any::io::encode_pdf(from).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
