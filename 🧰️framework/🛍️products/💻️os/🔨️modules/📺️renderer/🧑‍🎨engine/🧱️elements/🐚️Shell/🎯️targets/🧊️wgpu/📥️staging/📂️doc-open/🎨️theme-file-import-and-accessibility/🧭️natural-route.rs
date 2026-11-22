//! 🧭 Routes one picked or dropped file to the editor that opens it: the formats the mounted editors declare as natural files, resolved by the shared picked-file routing of `🚪️io` (`route_picked_file`), the Rust twin of React's `routePickedFileV1`. Pure, so a native law drives the same routing the browser shell runs.
use super::*;
use protocol::io::{route_picked_file, FormatDescriptor, PackageClaim, PackageDialect, PackageSource, PickedFileCatalog, PickedFileRoute, RevisionSniff};
use std::{cell::RefCell, collections::BTreeMap};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;

/// 🛬 One OS file the page holds for the shell: the page's token for the `File`, its name, byte size and MIME claim.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct BrowserPickedFile {
    pub token: u32,
    pub name: String,
    pub size: u64,
    #[serde(default)]
    pub mime: String,
}

/// 🧭 What the pick and route step answered: nothing picked, a named refusal, or the file with the dialect coordinate whose editor opens it.
pub enum BrowserOpenRouting {
    Cancelled(Option<u32>),
    Refused { name: String, reason: &'static str, token: u32 },
    Routed { file: BrowserPickedFile, artifact_ref: String },
}

impl BrowserOpenRouting {
    /// 🛬 The page's token for the file this routing still holds, so a settled routing can let the page release it.
    pub fn token(&self) -> Option<u32> {
        match self {
            Self::Routed { file, .. } => Some(file.token),
            Self::Refused { token, .. } => Some(*token),
            Self::Cancelled(token) => *token,
        }
    }
}

type NaturalRow = (String, String, String, bool);

fn natural_row(app: &AppDefinition, id: &str) -> Option<NaturalRow> {
    let action = app.actions.iter().chain(app.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == id)?;
    let default = |name: &str| action.args.iter().find(|arg| arg.id == name)?.default.as_ref();
    let extension = default("extension")?.as_str()?.to_owned();
    (extension.len() > 1 && extension.starts_with('.') && extension[1..].bytes().all(|byte| byte.is_ascii_alphanumeric())).then_some(())?;
    Some((default("formatKind")?.as_str()?.to_owned(), extension, default("mediaType")?.as_str()?.to_owned(), default("binary")?.as_bool()?))
}

/// 📋 The natural file format an editor declares: its Open and Save file actions must agree on kind, extension, media type and binary flag (React's `naturalFileFormatV1`); answers `(formatKind, extension, mediaType, binary)`.
pub fn browser_natural_format(app: &AppDefinition) -> Option<NaturalRow> {
    let open = natural_row(app, semio_framework::OPEN_ARTIFACT_FILE_ACTION_ID)?;
    (natural_row(app, semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID)? == open).then_some(open)
}

/// 🧬 One claim row of the host's Open file catalog (`wgpuFormatClaimsJson`): a format with its owner dialect, package claim and revisions.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimRow {
    kind_id: String,
    short_id: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    mimes: Vec<String>,
    #[serde(default)]
    extensions: Vec<String>,
    artifact_ref: Option<String>,
    package: Option<PackageRow>,
    #[serde(default)]
    revisions: Vec<RevisionRow>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RevisionRow {
    kind_id: String,
    artifact_ref: Option<String>,
    package: Option<PackageRow>,
    window: usize,
    contains: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageRow {
    main_part_content_types: Vec<String>,
    dialects: Vec<DialectRow>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DialectRow {
    artifact_ref: String,
    namespace: String,
}

impl PackageRow {
    fn claim(self) -> PackageClaim {
        PackageClaim { main_part_content_types: self.main_part_content_types, dialects: self.dialects.into_iter().map(|dialect| PackageDialect { artifact_ref: dialect.artifact_ref, namespace: dialect.namespace }).collect() }
    }
}

fn descriptor_of(kind_id: String, short_id: String, aliases: Vec<String>, mimes: Vec<String>, extensions: Vec<String>) -> FormatDescriptor {
    let name = short_id.to_ascii_uppercase();
    FormatDescriptor { kind_id, aliases, mimes, extensions, full_name: name.clone(), name, neutral: false, dir_name: short_id.clone(), short_id, is_binary: true }
}

/// 🗂 The formats the shell opens as natural files: each claim, the dialect of its owner editor, every revision's content check and every package claim; the Rust twin of React's `PickedFileFormatCatalogPortV1`.
#[derive(Clone, Debug, Default)]
pub struct BrowserCatalog {
    descriptors: Vec<FormatDescriptor>,
    owners: BTreeMap<String, String>,
    revisions: BTreeMap<String, Vec<RevisionSniff>>,
    packages: BTreeMap<String, PackageClaim>,
}

impl BrowserCatalog {
    /// 🧬 The product's catalog as the page answers it (`format-claims`); `None` when the answer is not a claim document or holds no claim.
    pub fn from_claims_json(json: &str) -> Option<Self> {
        #[derive(Deserialize)]
        struct Document {
            formats: Vec<ClaimRow>,
        }
        let document: Document = serde_json::from_str(json).ok()?;
        let mut catalog = Self::default();
        for row in document.formats {
            if let Some(artifact_ref) = row.artifact_ref { catalog.owners.insert(row.kind_id.clone(), artifact_ref); }
            if let Some(package) = row.package { catalog.packages.insert(row.kind_id.clone(), package.claim()); }
            let mut sniffs = Vec::new();
            for revision in row.revisions {
                if let Some(artifact_ref) = revision.artifact_ref { catalog.owners.insert(revision.kind_id.clone(), artifact_ref); }
                if let Some(package) = revision.package { catalog.packages.insert(revision.kind_id.clone(), package.claim()); }
                sniffs.push(RevisionSniff { kind_id: revision.kind_id, window: revision.window, contains: revision.contains });
            }
            if !sniffs.is_empty() { catalog.revisions.insert(row.kind_id.clone(), sniffs); }
            catalog.descriptors.push(descriptor_of(row.kind_id, row.short_id, row.aliases, row.mimes, row.extensions));
        }
        (!catalog.descriptors.is_empty()).then_some(catalog)
    }

    /// 🗂 The catalog the mounted editors declare when the host supplies none: one row per kind, owned by the dialect of the first editor declaring it, without revision or package claims.
    pub fn from_mounted(plugins: &[ProgramBridgeEntry]) -> Self {
        let mut catalog = Self::default();
        for app in plugins.iter().flat_map(|program| program.manifest.apps.iter()).filter(|app| app.role == semio_framework::manifest::AppRole::Editor) {
            let Some((kind_id, extension, mime, _)) = browser_natural_format(app) else { continue };
            if catalog.owners.contains_key(&kind_id) { continue; }
            catalog.owners.insert(kind_id.clone(), app.dialect.to_coordinate());
            let short_id = extension.trim_start_matches('.').to_ascii_lowercase();
            catalog.descriptors.push(descriptor_of(kind_id, short_id, Vec::new(), vec![mime], vec![extension.to_ascii_lowercase()]));
        }
        catalog
    }

    /// 📂 The picker's `accept` list: every declared extension.
    pub fn accept(&self) -> String {
        self.descriptors.iter().flat_map(|descriptor| descriptor.extensions.iter().map(String::as_str)).collect::<Vec<_>>().join(",")
    }
}

impl PickedFileCatalog for BrowserCatalog {
    fn descriptors(&self) -> Vec<FormatDescriptor> { self.descriptors.clone() }
    fn artifact_ref_of(&self, kind_id: &str) -> Option<String> { self.owners.get(kind_id).cloned() }
    fn revisions_of(&self, kind_id: &str) -> Vec<RevisionSniff> { self.revisions.get(kind_id).cloned().unwrap_or_default() }
    fn package_of(&self, kind_id: &str) -> Option<PackageClaim> { self.packages.get(kind_id).cloned() }
}

/// 📖 The picked file as a package source over the ranges read so far: a range not read yet answers `None` and is recorded, so the caller reads the recorded ranges and routes again until no range is missing; the page's file reads are asynchronous and the shared sniff is synchronous.
pub struct RangeCache {
    size: u64,
    ranges: RefCell<BTreeMap<(u64, u64), Vec<u8>>>,
    misses: RefCell<Vec<(u64, u64)>>,
    cancelled: Box<dyn Fn() -> bool>,
}

impl RangeCache {
    pub fn new(size: u64, cancelled: impl Fn() -> bool + 'static) -> Self {
        Self { size, ranges: RefCell::default(), misses: RefCell::default(), cancelled: Box::new(cancelled) }
    }

    /// 📥 Remembers the bytes of `[start, end)`.
    pub fn insert(&self, start: u64, end: u64, bytes: Vec<u8>) { self.ranges.borrow_mut().insert((start, end), bytes); }

    /// 🔍 Takes the ranges the last route asked for and did not find.
    pub fn take_misses(&self) -> Vec<(u64, u64)> { std::mem::take(&mut *self.misses.borrow_mut()) }

    /// 🔬 The first `bytes` of the file for the revision sniff; empty (and recorded) until read.
    pub fn head(&self, bytes: usize) -> Vec<u8> { self.read(0, bytes as u64).unwrap_or_default() }
}

impl PackageSource for RangeCache {
    fn size(&self) -> u64 { self.size }
    fn read(&self, start: u64, end: u64) -> Option<Vec<u8>> {
        let end = end.min(self.size);
        if start >= end { return Some(Vec::new()); }
        if let Some(bytes) = self.ranges.borrow().get(&(start, end)) { return Some(bytes.clone()); }
        self.misses.borrow_mut().push((start, end));
        None
    }
    fn cancelled(&self) -> bool { (self.cancelled)() }
}

/// 🧭 Routes `file` by name and MIME claim, sniffing its head for a revision and its package for a dialect through `content` and `head` like React's `routePickedFileV1`; the file's token is released by the caller for every routing but a routed one. A cancel settles as no pick, never a refusal.
pub fn route_browser_file(file: BrowserPickedFile, catalog: &BrowserCatalog, cancelled: bool, content: Option<&dyn PackageSource>, head: &dyn Fn(usize) -> Vec<u8>) -> BrowserOpenRouting {
    let mime = (!file.mime.is_empty()).then_some(file.mime.as_str());
    let routed = if cancelled { None } else { route_picked_file(Some(catalog), &file.name, mime, head, content).ok() };
    match routed {
        Some(PickedFileRoute::Routed { artifact_ref, .. }) => BrowserOpenRouting::Routed { file, artifact_ref },
        Some(PickedFileRoute::Refused(reason)) => BrowserOpenRouting::Refused { token: file.token, name: file.name, reason: reason.as_str() },
        None => BrowserOpenRouting::Cancelled(Some(file.token)),
    }
}
