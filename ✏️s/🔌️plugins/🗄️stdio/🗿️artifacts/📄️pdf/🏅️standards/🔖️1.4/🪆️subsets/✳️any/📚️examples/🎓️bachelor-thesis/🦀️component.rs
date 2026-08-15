//! 📚️ Example `🎓️bachelor-thesis` for artifact `stdio.pdf` — a real, non-trivial fixture (PDF
//! **1.5** per its own `%PDF-` header, ~6.3MB) decoded via the `🔖️1.7` standard's real engine
//! (1.7 reads 1.0-1.7 leniently, Decision #5 — `declared_version` below reports the fixture's
//! actual "1.5", not the reader standard). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "bachelor-thesis";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Bachelor Thesis", "Bachelorarbeit")
}
pub const ICON: &str = "file";
pub const FIXTURE_BYTES: &[u8] = include_bytes!("🖼️assets/📄️bachelor-thesis.pdf");

fn decoded_summary_json() -> String {
    match crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(FIXTURE_BYTES) {
        Ok(snap) => format!(r#"{{"fixture":"bachelor-thesis.pdf","bytes":{},"declaredVersion":"{}","pageCount":{},"objectCount":{}}}"#, FIXTURE_BYTES.len(), snap.declared_version, snap.pages.len(), snap.objects.len(),),
        Err(e) => format!(r#"{{"fixture":"bachelor-thesis.pdf","bytes":{},"decodeError":"{}"}}"#, FIXTURE_BYTES.len(), e),
    }
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), decoded_summary_json(), ICON)
}
