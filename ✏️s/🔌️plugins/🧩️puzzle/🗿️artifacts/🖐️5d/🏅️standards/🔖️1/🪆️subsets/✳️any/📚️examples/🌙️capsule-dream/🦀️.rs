//! 📚️ Example `🌙️capsule-dream` for artifact `puzzle5d`.

use std::sync::LazyLock;

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "capsule-dream";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Capsule Dream", "Kapseltraum")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🔏️ The digest sidecar of the authored capsule-dream DSL (3 035 200 B of `.puzzle5d` text): its repository-relative name, exact byte length and SHA-256, one per line. The fixture itself is no longer compiled in; the descriptor declares the example by this sidecar and the host verifies the delivered file against the same three values.
pub const DSL_DIGEST: &[u8] = include_bytes!("🖼️assets/🌙️dream/🔏️.digest");

/// 📦️ What the digest sidecar of the authored capsule-dream DSL (3 035 200 B of `.puzzle5d` text) says: the package asset name, its exact size and its SHA-256.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dsl_bytes_digest() -> (&'static str, u64, &'static str) {
    let text = std::str::from_utf8(DSL_DIGEST).expect("the digest sidecar is UTF-8");
    let mut lines = text.lines();
    let name = lines.next().expect("the digest sidecar names the asset");
    let size_bytes = lines.next().and_then(|line| line.parse().ok()).expect("the digest sidecar sizes the asset");
    let sha256 = lines.next().expect("the digest sidecar hashes the asset");
    (name, size_bytes, sha256)
}

/// 🖼️ The authored capsule-dream DSL (3 035 200 B of `.puzzle5d` text) as the host delivered it with the instance open. A host-less test of this crate reads the same repository file instead; any other caller without a delivery is a defect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dsl_bytes() -> &'static [u8] {
    let (name, ..) = dsl_bytes_digest();
    match semio_framework_plugin::reactor::package_asset(name) {
        Some(bytes) => bytes,
        #[cfg(test)]
        None => repository_file(name),
        #[cfg(not(test))]
        None => panic!("package asset {name} was not delivered by the host"),
    }
}

/// 🧪️ Reads the authored capsule-dream DSL (3 035 200 B of `.puzzle5d` text) below the repository root once per process, for tests that have no host to deliver the asset.
#[cfg(test)]
fn repository_file(name: &str) -> &'static [u8] {
    static BYTES: std::sync::OnceLock<&'static [u8]> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|candidate| candidate.join("nx.json").is_file()).expect("the repository root holds nx.json");
        Box::leak(std::fs::read(root.join(name)).unwrap_or_else(|error| panic!("{name}: {error}")).into_boxed_slice())
    })
}

/// 🗣️ The authored DSL as text, from the delivered package asset.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dsl_text() -> &'static str {
    std::str::from_utf8(dsl_bytes()).unwrap_or_else(|error| panic!("{ID} example dsl is UTF-8: {error}"))
}

/// 📦️ The package asset declaration of the authored DSL: the repository-relative file the puzzle plugin declares so a host delivers it with the instance open, its exact size and its SHA-256.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dsl_asset_declaration() -> semio_framework::AssetDeclaration {
    let (name, size_bytes, sha256) = dsl_bytes_digest();
    semio_framework::AssetDeclaration { name: name.to_string(), media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Text, form: semio_framework::MediaForm::Document }, size_bytes, sha256: sha256.to_string() }
}

/// 🏭️ Derives the document body from [`dsl_text`]. 3 035 200 B of DSL in, ~3.5 MB of JSON out —
/// this is the producer `ExampleSource::deferred` holds, and NOTHING but an actual request for the
/// document runs it.
fn document_json() -> String {
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(dsl_text()).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    semio_framework_pack_json::to_json_string(&projection)
}

/// 📚️ Canonical example source for `App::example_source` — DEFERRED, because eagerly parsing
/// [`dsl_text`] here ran while the bundle was assembled and made this package the one plugin in the
/// tree whose `describe` could not finish inside its 1 800 s guest epoch
/// (`📓️a3-descriptor-regeneration.md` §3, `📓️a3b-descriptor-sweep.md` §3). The descriptor declares
/// this example by the digest sidecar of the authored DSL (its name, size and SHA-256), which needs neither a parse nor the 3 MB of DSL in the compiled source.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::deferred(ID, label(), ICON, ".dsl.semio.digest", DSL_DIGEST, document_json));
