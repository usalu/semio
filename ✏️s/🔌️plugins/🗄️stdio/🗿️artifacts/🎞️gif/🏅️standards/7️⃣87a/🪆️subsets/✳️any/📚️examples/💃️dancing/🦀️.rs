//! 📚️ Example "dancing" for stdio.gif — a real animated GIF89a fixture (54 frames, 800×800,
//! per-frame local color tables, NETSCAPE2.0 infinite loop). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: unlike the `🎬️demo`
//! example (a stub hex string that never actually decodes), this one's `artifact_json` comes from
//! ACTUALLY decoding the real fixture bytes via the 89a engine's real LZW/GCE/loop codec and
//! serializing the resulting real snapshot.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "dancing";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Dancing", "Dancing")
}
pub const ICON: &str = "file";

/// 📦️ The name the fixture travels under as a package asset: its repository-relative path, so a host resolves it without a registry.
pub const DANCING_GIF_ASSET_NAME: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif";

/// 📏️ The exact byte length of the fixture, which the host checks before it delivers the asset.
pub const DANCING_GIF_ASSET_SIZE_BYTES: u64 = 4433936;

/// 🔏️ The SHA-256 of the fixture, which the host checks before it delivers the asset; `📦️package-asset-constants` recomputes it independently.
pub const DANCING_GIF_ASSET_SHA256: &str = "494a93e5ca4714b17dad097353727a01efe72aa3d22f8bcb4896d30b898cf866";

/// 🖼️ The real fixture bytes — confirmed via sniffing to be GIF89a, animated, multiple frames — as the host delivered them with the
/// instance open. A host-less test reads the same repository file instead; any other caller without a delivery is a defect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dancing_gif_bytes() -> &'static [u8] {
    match semio_framework_plugin::reactor::package_asset(DANCING_GIF_ASSET_NAME) {
        Some(bytes) => bytes,
        #[cfg(test)]
        None => repository_file(),
        #[cfg(not(test))]
        None => panic!("package asset {DANCING_GIF_ASSET_NAME} was not delivered by the host"),
    }
}

/// 🧪️ Reads the fixture below the repository root once per process, for tests that have no host to deliver the asset.
#[cfg(test)]
fn repository_file() -> &'static [u8] {
    static BYTES: std::sync::OnceLock<&'static [u8]> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|candidate| candidate.join("nx.json").is_file()).expect("the repository root holds nx.json");
        Box::leak(std::fs::read(root.join(DANCING_GIF_ASSET_NAME)).unwrap_or_else(|error| panic!("{DANCING_GIF_ASSET_NAME}: {error}")).into_boxed_slice())
    })
}

/// 📄️ Decodes [`dancing_gif_bytes`] via the real 89a codec. Panics (at example-registration
/// time, not at runtime for end users) if the fixture ever stops decoding — that's a real
/// regression this example exists to catch, not something to paper over with a fallback.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decoded_snapshot() -> crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot {
    crate::standards::v89a::subsets::any::io::decode_gif(dancing_gif_bytes()).expect("dancing.gif fixture must decode via the real GIF89a codec")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    let artifact_json = semio_framework_pack_json::to_json_string(&decoded_snapshot());
    ExampleSource::new(ID, label(), artifact_json, ICON)
}


