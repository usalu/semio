//! 🔌️ Canvas runtime asset port. Canvas embeds no font and no icon: the host owns the delivery (file system,
//! fetch, packaged asset) of the seven assets named by [`RuntimeAsset`] and hands their bytes in once, through
//! [`install_runtime_assets`], before the first icon or label resolves. Until then math, emoji, text and
//! shortcode icons resolve to nothing and labels fall back to no glyphs. The same port serves every
//! consumer: the compiler's font set (math, emoji and text icons), the label face (map and editor labels)
//! and the icon pack (catalog, metabolism and shortcode SVG).
//!
//! The assets keep living in the directories of the modules that own them ([`RuntimeAsset::repository_directory`]);
//! [`RuntimeAssets::from_root`] reads them from a repository-shaped root, and the browser doors fetch them from the
//! routes `🟦️.ts` declares. A `wasm32-wasip2` guest has no file system or fetch: its plugin declares the three assets
//! it uses ([`RuntimeAsset::GUEST`]) as package assets ([`guest_asset_declarations`]), registers the guest's package
//! asset reader ([`register_guest_asset_reader`]) and the port installs them lazily on first use, once the host has
//! delivered them with the instance open. The compiler still estimates text sizes without glyphs there.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::{fs, io, path::Path};

pub use compiler::world::{FontRole, FontSet};
use compiler::CompilerFonts;

// #region 🏷️IconAssets
/// 🏷️ One runtime asset: a compiler font, the map-label face or the icon pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RuntimeAsset {
    CompilerFont(FontRole),
    LabelFont,
    IconPack,
}

impl RuntimeAsset {
    /// 📋️ Every asset in the order hosts deliver them: the five compiler fonts in [`FontRole::ALL`] order, the label face, the icon pack.
    pub const ALL: [RuntimeAsset; 7] = [
        RuntimeAsset::CompilerFont(FontRole::Math),
        RuntimeAsset::CompilerFont(FontRole::Serif),
        RuntimeAsset::CompilerFont(FontRole::SerifItalic),
        RuntimeAsset::CompilerFont(FontRole::Mono),
        RuntimeAsset::CompilerFont(FontRole::Emoji),
        RuntimeAsset::LabelFont,
        RuntimeAsset::IconPack,
    ];

    /// 🎁️ The assets a `wasm32-wasip2` guest uses: the emoji face its SVG icons parse `<text>` glyphs with, the label face and
    /// the icon pack. The compiler's other fonts stay unused there because the compiler estimates sizes without glyphs.
    pub const GUEST: [RuntimeAsset; 3] = [RuntimeAsset::CompilerFont(FontRole::Emoji), RuntimeAsset::LabelFont, RuntimeAsset::IconPack];

    /// 📄️ The asset's file name inside [`RuntimeAsset::repository_directory`] — the identity a host resolves through its own channel.
    pub const fn file_name(self) -> &'static str {
        match self {
            RuntimeAsset::CompilerFont(role) => role.asset_file_name(),
            RuntimeAsset::LabelFont => "🔤️MapLabelSans.ttf",
            RuntimeAsset::IconPack => "🔣️.json",
        }
    }

    /// 🚏️ The channel the asset is served and staged under: `/renderer-modules/<channel>/<file>` in a bundle and `<channel>/<file>` inside a staged runtime asset folder.
    pub const fn channel(self) -> &'static str {
        match self {
            RuntimeAsset::CompilerFont(_) => "compiler-fonts",
            RuntimeAsset::LabelFont => "canvas-fonts",
            RuntimeAsset::IconPack => "icon-pack",
        }
    }

    /// 📂️ The directory that owns the asset, relative to the repository root.
    pub const fn repository_directory(self) -> &'static str {
        match self {
            RuntimeAsset::CompilerFont(_) => "🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts",
            RuntimeAsset::LabelFont => "🧰️framework/🔨️modules/🖼️canvas/🖼️assets",
            RuntimeAsset::IconPack => "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes",
        }
    }
}

/// 🖥️ The bytes of every runtime asset, borrowed from the host.
#[derive(Clone, Copy, Debug)]
pub struct RuntimeAssets<'a> {
    pub fonts: FontSet<'a>,
    pub label_font: &'a [u8],
    pub icon_pack: &'a [u8],
}

impl<'a> RuntimeAssets<'a> {
    pub fn get(&self, asset: RuntimeAsset) -> &'a [u8] {
        match asset {
            RuntimeAsset::CompilerFont(role) => self.fonts.get(role),
            RuntimeAsset::LabelFont => self.label_font,
            RuntimeAsset::IconPack => self.icon_pack,
        }
    }

    /// 🔨️ Builds the set by asking `load` for every asset in [`RuntimeAsset::ALL`] order and stops at the first failure.
    pub fn try_from_loader<E>(mut load: impl FnMut(RuntimeAsset) -> Result<&'a [u8], E>) -> Result<Self, E> {
        let fonts = FontSet::try_from_loader(|role| load(RuntimeAsset::CompilerFont(role)))?;
        let label_font = load(RuntimeAsset::LabelFont)?;
        let icon_pack = load(RuntimeAsset::IconPack)?;
        Ok(Self { fonts, label_font, icon_pack })
    }

    /// ✅️ Parses every asset exactly as [`install_runtime_assets`] would, without installing anything.
    pub fn validate(&self) -> Result<(), RuntimeAssetsInstallError> {
        parse_runtime_assets(self).map(|_| ())
    }
}

impl RuntimeAssets<'static> {
    /// 🏠️ Reads every asset from `<root>/<repository_directory>/<file_name>` and leaks the buffers, so the set lives
    /// for the process. Call it once per process.
    pub fn from_root(root: &Path) -> io::Result<Self> {
        Self::try_from_loader(|asset| read_leaked(&root.join(asset.repository_directory()).join(asset.file_name())))
    }

    /// 📁️ Reads every asset from a staged runtime asset folder, `<root>/<channel>/<file_name>` (what `stage-runtime-assets` writes
    /// and a packaged install ships next to its executable), and leaks the buffers. Call it once per process.
    pub fn from_staged_root(root: &Path) -> io::Result<Self> {
        Self::try_from_loader(|asset| read_leaked(&root.join(asset.channel()).join(asset.file_name())))
    }
}

fn read_leaked(path: &Path) -> io::Result<&'static [u8]> {
    let bytes = fs::read(path).map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))?;
    Ok(&*Box::leak(bytes.into_boxed_slice()))
}

/// 🚫️ Why [`install_runtime_assets`] refused the host's assets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeAssetsInstallError {
    UnparseableFont { role: FontRole },
    UnparseableIconPack { cause: String },
    AlreadyInstalled,
}

impl std::fmt::Display for RuntimeAssetsInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnparseableFont { role } => write!(f, "compiler font {role:?} ({}) does not parse", role.asset_file_name()),
            Self::UnparseableIconPack { cause } => write!(f, "icon pack ({}) does not parse: {cause}", RuntimeAsset::IconPack.file_name()),
            Self::AlreadyInstalled => f.write_str("runtime assets are already installed"),
        }
    }
}

impl std::error::Error for RuntimeAssetsInstallError {}
// #endregion 🏷️IconAssets

//#region 🔖️GuestDelivery
impl RuntimeAsset {
    /// 📦️ The name the asset travels under as a package asset: its repository-relative path, so a host resolves it without a registry.
    pub fn package_asset_name(self) -> String {
        format!("{}/{}", self.repository_directory(), self.file_name())
    }
}

/// 📑️ One guest asset as a plugin declares it: the exact name, size and SHA-256 hex the host verifies before delivery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeAssetDeclaration {
    pub name: String,
    pub size_bytes: u64,
    pub sha256: String,
}

const GUEST_FONT_DIGESTS: [(RuntimeAsset, u64, &str); 2] =
    [(RuntimeAsset::CompilerFont(FontRole::Emoji), 152308, "ec69d141e4276776e952ecc10a5bd02f9282c96928de1787c25012fb2a18fb02"), (RuntimeAsset::LabelFont, 26772, "6e9112853217f39bc753c94ab0f16fd7dc57fa56003cfb981b0bb7b0c514487f")];

#[path = "../../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🔏️digest/🦀️.rs"]
mod icon_pack_digest;

/// 🔃️ The declarations of [`RuntimeAsset::GUEST`] in that order. The pack's digest is written by the assets builder next to
/// the pack, so it cannot drift from the pack a host serves.
pub fn guest_asset_declarations() -> Vec<RuntimeAssetDeclaration> {
    RuntimeAsset::GUEST
        .iter()
        .map(|asset| {
            let (size_bytes, sha256) = match asset {
                RuntimeAsset::IconPack => (icon_pack_digest::ICON_PACK_SIZE_BYTES, icon_pack_digest::ICON_PACK_SHA256.to_string()),
                other => GUEST_FONT_DIGESTS.iter().find(|(candidate, _, _)| candidate == other).map(|(_, size, sha)| (*size, (*sha).to_string())).expect("every guest font has a digest"),
            };
            RuntimeAssetDeclaration { name: asset.package_asset_name(), size_bytes, sha256 }
        })
        .collect()
}

/// 📥️ Answers a declared package asset name with the bytes the host delivered, `None` when it delivered none
/// (`semio_framework_plugin::reactor::package_asset` in a guest).
pub type PackageAssetReader = fn(&str) -> Option<&'static [u8]>;

static GUEST_READER: OnceLock<PackageAssetReader> = OnceLock::new();
static GUEST_REFUSED: OnceLock<()> = OnceLock::new();

/// 📖️ Registers the guest's package asset reader, once per process (the first registration stays). Every accessor then
/// installs the [`RuntimeAsset::GUEST`] assets on first use after the host delivered them; before that they answer `None`,
/// exactly as on a host that delivers nothing. The compiler fonts stay estimate-only.
pub fn register_guest_asset_reader(read: PackageAssetReader) {
    let _ = GUEST_READER.set(read);
}

fn install_from_guest_reader() -> Option<&'static Installed> {
    let read = GUEST_READER.get()?;
    if GUEST_REFUSED.get().is_some() {
        return None;
    }
    let (emoji_font, label_font, pack) = (read(&RuntimeAsset::GUEST[0].package_asset_name())?, read(&RuntimeAsset::GUEST[1].package_asset_name())?, read(&RuntimeAsset::GUEST[2].package_asset_name())?);
    match IconPack::parse(pack) {
        Ok(icon_pack) => {
            let _ = INSTALLED.set(Installed { fonts: None, emoji_font, label_font, icon_pack });
            INSTALLED.get()
        }
        Err(error) => {
            let _ = GUEST_REFUSED.set(());
            eprintln!("[ERROR] guest icon pack ({}) refused: {error}", RuntimeAsset::IconPack.file_name());
            None
        }
    }
}
//#endregion 🔖️GuestDelivery

//#region 🔖️IconPack
/// 🧳️ Why an icon pack document was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconPackError {
    Malformed(String),
    Version(u32),
}

impl std::fmt::Display for IconPackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(cause) => f.write_str(cause),
            Self::Version(found) => write!(f, "icon pack version {found} is not {}", IconPack::VERSION),
        }
    }
}

impl std::error::Error for IconPackError {}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct IconPackDocument {
    version: u32,
    themed: HashMap<String, String>,
    plain: HashMap<String, String>,
}

/// 🎒️ Every vendored SVG source by shortcode id: `themed` (metabolism, recolored by the paint theme) and `plain`
/// (catalog), exactly the files the assets builder reads.
#[derive(Debug)]
pub struct IconPack {
    themed: HashMap<String, String>,
    plain: HashMap<String, String>,
}

impl IconPack {
    /// 🔖️ The wire version of the pack document this reader admits.
    pub const VERSION: u32 = 1;

    pub fn parse(bytes: &[u8]) -> Result<Self, IconPackError> {
        let document: IconPackDocument = serde_json::from_slice(bytes).map_err(|error| IconPackError::Malformed(error.to_string()))?;
        if document.version != Self::VERSION {
            return Err(IconPackError::Version(document.version));
        }
        Ok(Self { themed: document.themed, plain: document.plain })
    }

    pub fn themed(&self, id: &str) -> Option<&str> {
        self.themed.get(id).map(String::as_str)
    }

    pub fn plain(&self, id: &str) -> Option<&str> {
        self.plain.get(id).map(String::as_str)
    }

    /// 🔢️ The catalog (plain) icon ids in byte order, the order the assets builder lists them in, so an atlas packed from them is deterministic.
    pub fn plain_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.plain.keys().map(String::as_str).collect();
        ids.sort_unstable();
        ids
    }

    pub fn themed_len(&self) -> usize {
        self.themed.len()
    }

    pub fn plain_len(&self) -> usize {
        self.plain.len()
    }
}
//#endregion 🔖️IconPack

//#region 🔖️Installation
struct Installed {
    fonts: Option<CompilerFonts<'static>>,
    emoji_font: &'static [u8],
    label_font: &'static [u8],
    icon_pack: IconPack,
}

static INSTALLED: OnceLock<Installed> = OnceLock::new();

fn installed() -> Option<&'static Installed> {
    INSTALLED.get().or_else(install_from_guest_reader)
}

fn parse_runtime_assets<'a>(assets: &RuntimeAssets<'a>) -> Result<(CompilerFonts<'a>, IconPack), RuntimeAssetsInstallError> {
    let fonts = CompilerFonts::parse(&assets.fonts).map_err(|failure| RuntimeAssetsInstallError::UnparseableFont { role: failure.role })?;
    let icon_pack = IconPack::parse(assets.icon_pack).map_err(|error| RuntimeAssetsInstallError::UnparseableIconPack { cause: error.to_string() })?;
    Ok((fonts, icon_pack))
}

/// ⚙️ The host's runtime asset port. Install once per process, before the first icon or label resolves. Everything
/// is parsed first, so a refused set does not consume the slot.
pub fn install_runtime_assets(assets: RuntimeAssets<'static>) -> Result<(), RuntimeAssetsInstallError> {
    if INSTALLED.get().is_some() {
        return Err(RuntimeAssetsInstallError::AlreadyInstalled);
    }
    let (fonts, icon_pack) = parse_runtime_assets(&assets)?;
    INSTALLED.set(Installed { fonts: Some(fonts), emoji_font: assets.fonts.emoji, label_font: assets.label_font, icon_pack }).map_err(|_| RuntimeAssetsInstallError::AlreadyInstalled)
}

/// 📩️ [`install_runtime_assets`] for a host that received the seven files by value, in [`RuntimeAsset::ALL`] order
/// (a browser fetch handed across a wasm-bindgen door); the buffers live for the process.
pub fn install_runtime_asset_bytes(math: Vec<u8>, serif: Vec<u8>, serif_italic: Vec<u8>, mono: Vec<u8>, emoji: Vec<u8>, label_font: Vec<u8>, icon_pack: Vec<u8>) -> Result<(), RuntimeAssetsInstallError> {
    let leaked = |bytes: Vec<u8>| -> &'static [u8] { Box::leak(bytes.into_boxed_slice()) };
    install_runtime_assets(RuntimeAssets { fonts: FontSet { math: leaked(math), serif: leaked(serif), serif_italic: leaked(serif_italic), mono: leaked(mono), emoji: leaked(emoji) }, label_font: leaked(label_font), icon_pack: leaked(icon_pack) })
}

/// 🌐️ The browser door every wasm module that links Canvas exports under the same name: the host passes the seven
/// fetched files in [`RuntimeAsset::ALL`] order. A module booting twice keeps its first install.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioSetRuntimeAssets)]
pub fn semio_set_runtime_assets(math: Vec<u8>, serif: Vec<u8>, serif_italic: Vec<u8>, mono: Vec<u8>, emoji: Vec<u8>, label_font: Vec<u8>, icon_pack: Vec<u8>) -> Result<(), wasm_bindgen::JsValue> {
    match install_runtime_asset_bytes(math, serif, serif_italic, mono, emoji, label_font, icon_pack) {
        Ok(()) | Err(RuntimeAssetsInstallError::AlreadyInstalled) => Ok(()),
        Err(error) => Err(wasm_bindgen::JsValue::from_str(&error.to_string())),
    }
}

/// 🔤️ The installed compiler fonts behind math, emoji and text icons; `None` until a host installs them.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn compiler_fonts() -> Option<&'static CompilerFonts<'static>> {
    installed().and_then(|installed| installed.fonts.as_ref())
}

/// 🔠️ `wasm32-wasip2` arm: the compiler estimates sizes without glyphs, so nothing needs installing.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn compiler_fonts() -> Option<&'static CompilerFonts<'static>> {
    static ESTIMATE_ONLY: CompilerFonts<'static> = CompilerFonts::estimate_only();
    Some(&ESTIMATE_ONLY)
}

/// 😀️ The installed emoji face the icon SVG parser resolves `<text>` glyphs with; `None` until a host installs it.
pub fn emoji_font() -> Option<&'static [u8]> {
    installed().map(|installed| installed.emoji_font)
}

/// ✏️ The installed label face place-name and editor labels are shaped with; `None` until a host installs it.
pub fn label_font() -> Option<&'static [u8]> {
    installed().map(|installed| installed.label_font)
}

/// 📚️ The installed icon pack behind catalog, metabolism and shortcode SVG icons; `None` until a host installs it.
pub fn icon_pack() -> Option<&'static IconPack> {
    installed().map(|installed| &installed.icon_pack)
}
//#endregion 🔖️Installation

//#region 🧪️Tests
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
