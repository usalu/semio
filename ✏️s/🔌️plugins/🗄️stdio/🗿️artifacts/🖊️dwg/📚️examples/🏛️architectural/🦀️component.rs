//! 📚️ Example `🏛️architectural` for artifact `stdio.dwg` — a real, non-trivial fixture (AC1024,
//! ~145KB) for ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.
//! The dwg codec now does real R2004+ D1 (file header decrypt + section/page location) and D2
//! (LZ77-variant decompression) decode -- this leaf exposes the genuinely-decoded section names
//! and decode status, not a fabricated/guessed result (D3+ bitcode/header-variable interpretation
//! is out of scope for this ticket; `sections[].pages[].decoded` carries raw undecoded bytes for
//! anything further than that).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};
use crate::artifacts::dwg::schema::snapshot::decode_dwg;

pub const ID: &str = "architectural";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Architectural Example", "Architekturbeispiel") }
pub const ICON: &str = "file";
pub const FIXTURE_BYTES: &[u8] = include_bytes!("🖼️assets/📄️architectural.dwg");

fn decoded_summary_json() -> String {
    match decode_dwg(FIXTURE_BYTES) {
        Ok(snap) => {
            let names: Vec<String> = snap.section_names.iter().map(|n| format!("\"{n}\"")).collect();
            format!(
                r#"{{"fixture":"architectural.dwg","bytes":{},"version":"{}","decodeStatus":"{:?}","sectionCount":{},"sectionNames":[{}]}}"#,
                FIXTURE_BYTES.len(),
                snap.version,
                snap.decode_status,
                snap.sections.len(),
                names.join(",")
            )
        }
        Err(e) => format!(r#"{{"fixture":"architectural.dwg","bytes":{},"error":"{e}"}}"#, FIXTURE_BYTES.len()),
    }
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), decoded_summary_json(), ICON)
}
