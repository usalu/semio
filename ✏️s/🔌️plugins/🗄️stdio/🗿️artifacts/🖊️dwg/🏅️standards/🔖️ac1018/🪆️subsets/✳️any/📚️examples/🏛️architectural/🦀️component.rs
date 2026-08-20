//! 📚️ Example `🏛️architectural` for artifact `stdio.dwg` — a real, non-trivial fixture (AC1024,
//! ~145KB) for ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.
//! The DWG codec projects the real fixture into standard logical drawing and metadata concepts.

use crate::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "architectural";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Architectural Example", "Architekturbeispiel")
}
pub const ICON: &str = "file";
pub const FIXTURE_BYTES: &[u8] = include_bytes!("🖼️assets/📄️architectural.dwg");

async fn decoded_summary_json() -> String {
    match decode_dwg(FIXTURE_BYTES).await {
        Ok(snap) => {
            format!(
                r#"{{"fixture":"architectural.dwg","bytes":{},"version":"{}","layerCount":{},"entityCount":{},"classCount":{},"dependencyCount":{}}}"#,
                FIXTURE_BYTES.len(),
                snap.version,
                snap.drawing.layers.len(),
                snap.drawing.entities().await.len(),
                snap.classes.len(),
                snap.dependencies.len()
            )
        }
        Err(e) => format!(r#"{{"fixture":"architectural.dwg","bytes":{},"error":"{e}"}}"#, FIXTURE_BYTES.len()),
    }
}

pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), decoded_summary_json(), ICON).await
}
