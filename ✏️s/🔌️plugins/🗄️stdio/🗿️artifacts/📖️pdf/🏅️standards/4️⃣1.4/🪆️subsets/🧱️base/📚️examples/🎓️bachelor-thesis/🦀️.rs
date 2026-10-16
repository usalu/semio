//! 📚️ Example `🎓️bachelor-thesis` for artifact `stdio.pdf` — a real, non-trivial fixture (PDF
//! **1.5** per its own `%PDF-` header, ~6.3MB) decoded via the `7️⃣1.7` standard's real engine
//! (1.7 reads 1.0-1.7 leniently, Decision #5 — `declared_version` below reports the fixture's
//! actual "1.5", not the reader standard). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "bachelor-thesis";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Bachelor Thesis", "Bachelorarbeit")
}
pub const ICON: &str = "file";
/// 📦️ The name the thesis fixture travels under as a package asset: its repository-relative path, so a host resolves it without a registry.
pub const BACHELOR_THESIS_ASSET_NAME: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf";

/// 📏️ The exact byte length of the thesis fixture, which the host checks before it delivers the asset.
pub const BACHELOR_THESIS_ASSET_SIZE_BYTES: u64 = 6346331;

/// 🔏️ The SHA-256 of the thesis fixture, which the host checks before it delivers the asset; `📦️package-asset-constants` recomputes it independently.
pub const BACHELOR_THESIS_ASSET_SHA256: &str = "83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3";

/// 🖼️ The thesis fixture as the host delivered it with the instance open. A host-less test reads the same repository file instead; any other caller without a delivery is a defect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn document_bytes() -> &'static [u8] {
    match semio_framework_plugin::reactor::package_asset(BACHELOR_THESIS_ASSET_NAME) {
        Some(bytes) => bytes,
        #[cfg(test)]
        None => repository_file(),
        #[cfg(not(test))]
        None => panic!("package asset {BACHELOR_THESIS_ASSET_NAME} was not delivered by the host"),
    }
}

/// 🧪️ Reads the thesis fixture below the repository root once per process, for tests that have no host to deliver the asset.
#[cfg(test)]
fn repository_file() -> &'static [u8] {
    static BYTES: std::sync::OnceLock<&'static [u8]> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|candidate| candidate.join("nx.json").is_file()).expect("the repository root holds nx.json");
        Box::leak(std::fs::read(root.join(BACHELOR_THESIS_ASSET_NAME)).unwrap_or_else(|error| panic!("{}: {error}", BACHELOR_THESIS_ASSET_NAME)).into_boxed_slice())
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decoded_summary_json() -> String {
    match crate::standards::v1_7::subsets::base::io::import::decode_pdf(document_bytes()) {
        Ok(snap) => format!(r#"{{"documentName":"bachelor-thesis.pdf","bytes":{},"declaredVersion":"{}","pageCount":{},"objectCount":{}}}"#, document_bytes().len(), snap.declared_version, snap.pages.len(), snap.objects.len(),),
        Err(e) => format!(r#"{{"documentName":"bachelor-thesis.pdf","bytes":{},"decodeError":"{}"}}"#, document_bytes().len(), e),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), decoded_summary_json(), ICON)
}
