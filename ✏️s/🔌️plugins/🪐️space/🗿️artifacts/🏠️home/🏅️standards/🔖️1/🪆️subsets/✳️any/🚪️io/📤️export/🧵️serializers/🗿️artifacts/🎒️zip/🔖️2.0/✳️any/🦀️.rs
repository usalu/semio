//! home → zip — the shared document archive (`encode_document_archive`): this artifact's DSL as the
//! authoritative member plus its rfc8259 rendition, both lossless (`IoFidelity::Exact`).
use crate::SHomeSnapshot;
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_document_archive;

pub fn register() {}

pub fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_document_archive(snapshot).map_err(|error| { let message=format!("home→zip: {error}");semio_framework_diagnostic::TextError::new(error.into_value_error().kind,message,semio_framework_diagnostic::TextSpan::at(1,1)) })
}
