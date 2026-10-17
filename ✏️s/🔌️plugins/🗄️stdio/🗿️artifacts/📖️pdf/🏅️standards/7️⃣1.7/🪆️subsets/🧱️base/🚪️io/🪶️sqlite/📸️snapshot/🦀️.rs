//! 📖️ Explicit PDF 1.7 document, COS, rendering and resource relations.
pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::io::sqlite::snapshot::number;
pub use semio_s_artifact_stdio_pdf_tests::standards::v1_7::subsets::base::io::sqlite::snapshot::reader;

pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::stream_roles;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::artifact_reference;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::cos;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::color;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::font;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::content;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::resource;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::metadata;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::navigation;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::annotation;
pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::form;
pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::sqlite::snapshot::document;
pub use document::{PdfConformanceProfile,validate_conformance_profile};
pub use semio_s_artifact_stdio_pdf_tests::standards::v1_7::subsets::base::io::sqlite::snapshot::admission;

