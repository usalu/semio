//! 🚪️ IO stdio.pdf (1.7/🧱️base) — the codec entry points over the engine modules: `decode_pdf`
//! (sniff → cross-reference → decrypt → retained graph → typed lanes), `encode_pdf` (typed lanes
//! reconciled onto the retained graph when one is carried, lowered afresh otherwise → bytes),
//! [`carry_graph_edit`] (a direct edit of the retained graph carried into the typed lanes it
//! moves), the streaming [`DocumentStream`] a guest can drive one page per step, and the typed
//! builders every consumer starts from ([`text_document`], [`PdfTextLayout`]). Reads PDF 1.0–2.0
//! leniently (`declared_version` records the header verbatim).
//!
//! Laws (proven in `🧪️tests`): `lift(lower(t)) == t` on the typed lanes; `decode(encode(s)) == s`;
//! a write re-states only what a moved lane owns, so every other retained object, dictionary entry
//! and trailer entry — direct COS edits included — survives it untouched; a decoded document is
//! its own fixed point.

use crate::standards::v1_7::subsets::base::modules::content::{content_references, ContentReferences};
use crate::standards::v1_7::subsets::base::modules::encryption::open_standard_security;
use crate::standards::v1_7::subsets::base::modules::fonts::{standard_font, FontCodec};
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::lift::{lift_document, lift_document_with, Category};
use crate::standards::v1_7::subsets::base::modules::lower::{lower_acro_form_standalone, lower_catalog_standalone, lower_document, lower_document_headless, lower_info, lower_page_standalone, LowerOptions, LoweredDocument};
use crate::standards::v1_7::subsets::base::modules::writer::{serialize_document, DocumentTrailer, PdfWriter, WriteOptions, WRITER_TRAILER_KEYS};
use crate::standards::v1_7::subsets::base::modules::xref::{build_xref, startxref_offset, GraphSource, ObjectSource, Resolver};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

pub use crate::standards::v1_7::subsets::base::modules::lexer::PdfEngineError as EngineError;

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_7::subsets::base::schema::PdfAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct PdfComposerComposition;

    impl ArtifactComposition for PdfComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PdfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PdfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PdfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Sniff
/// 🔍️ Real magic + version probe: `%PDF-` header, version digits parsed and reported.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_pdf(bytes: &[u8]) -> Option<String> {
    let start = bytes.windows(5).take(1024).position(|window| window == b"%PDF-")?;
    let rest = &bytes[start + 5..];
    let end = rest.iter().take(8).position(|&b| b == b'\n' || b == b'\r' || b == b' ' || b == b'\t').unwrap_or(rest.len().min(8));
    let version = String::from_utf8_lossy(&rest[..end]).trim().to_string();
    (!version.is_empty() && version.chars().all(|c| c.is_ascii_digit() || c == '.')).then_some(version)
}
//#endregion 🔖️Sniff

//#region 🔖️Decode
/// 📥️ Decodes a file with the empty user password.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf(data: &[u8]) -> PResult<PdfSnapshot> {
    decode_pdf_with_password(data, "")
}

/// 📥️ Decodes a file, opening the standard security handler with `password` when it is
/// encrypted (`Unsupported` when the password does not open it or the handler is not the
/// standard one — never garbage).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf_with_password(data: &[u8], password: &str) -> PResult<PdfSnapshot> {
    let declared_version = sniff_pdf(data).ok_or(PdfEngineError::NotPdf)?;
    let start = data.windows(5).position(|window| window == b"%PDF-").unwrap_or(0);
    let data = &data[start..];
    let xref = build_xref(data, startxref_offset(data));
    let mut resolver = Resolver::new(data, xref.entries.clone());
    let mut encryption = None;
    let mut encrypt_object: Option<u32> = None;
    if let Some(encrypt) = dict_get(&xref.trailer, "Encrypt") {
        encrypt_object = encrypt.as_ref().map(|reference| reference.num);
        let dictionary = match encrypt {
            PdfObject::Ref(reference) => resolver.resolve(reference.num).ok_or_else(|| PdfEngineError::Malformed("/Encrypt reference does not resolve".into()))?,
            other => other.clone(),
        };
        let dictionary = dictionary.as_dict().ok_or_else(|| PdfEngineError::Malformed("/Encrypt is not a dictionary".into()))?.to_vec();
        let document_id = dict_get(&xref.trailer, "ID").and_then(PdfObject::as_array).and_then(|items| items.first()).and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let (decryptor, parameters) = open_standard_security(&dictionary, &document_id, password)?;
        resolver.set_decryptor(Some(decryptor));
        encryption = Some(parameters);
    }
    let mut objects = resolver.resolve_all()?;
    if let Some(number) = encrypt_object {
        objects.retain(|object| object.id.num != number);
    }
    let trailer: Vec<PdfDictEntry> = xref.trailer.iter().filter(|entry| !WRITER_TRAILER_KEYS.contains(&entry.key.as_str())).cloned().collect();
    if !trailer.iter().any(|entry| entry.key == "Root") {
        return Err(PdfEngineError::Malformed("no /Root in any trailer and no /Catalog object to recover one from".into()));
    }
    let mut source = GraphSource::new(&objects);
    let lifter = lift_document_with(&trailer, &declared_version, &mut source);
    let mut snapshot = lifter.snapshot;
    snapshot.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    snapshot.declared_version = declared_version;
    snapshot.encryption = encryption;
    snapshot.objects = objects;
    snapshot.trailer = trailer;
    Ok(snapshot)
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🎛️ Every knob of a write in one place.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EncodeOptions {
    pub lower: LowerOptions,
    pub write: WriteOptions,
}

impl EncodeOptions {
    /// 📦 The export profile consumers want: subset fonts, compressed streams.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export() -> Self {
        Self { lower: LowerOptions { subset_fonts: true, compress: true }, write: WriteOptions::default() }
    }
}

/// 📤️ Writes the snapshot with default options (no font subsetting, classic cross-reference
/// table, the snapshot's own `encryption`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pdf(snapshot: &PdfSnapshot) -> PResult<Vec<u8>> {
    encode_pdf_with(snapshot, &EncodeOptions::default())
}

/// 📤️ Writes the snapshot. A snapshot carrying a retained graph is reconciled onto it
/// ([`reconcile`]); without one — or under an export profile, whose writer choices a retained
/// graph cannot carry — the typed lanes are lowered afresh. Either way every trailer entry the
/// snapshot carries beyond the writer's own bookkeeping is written back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pdf_with(snapshot: &PdfSnapshot, options: &EncodeOptions) -> PResult<Vec<u8>> {
    let version = if snapshot.declared_version.is_empty() { "1.7".to_string() } else { snapshot.declared_version.clone() };
    if !version.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.') {
        return Err(PdfEngineError::Malformed("declared PDF version is not numeric".into()));
    }
    let version = if options.write.xref_stream && version.as_str() < "1.5" { "1.5".to_string() } else { version };
    let mut write = options.write.clone();
    if write.encryption.is_none() {
        write.encryption = snapshot.encryption.clone();
    }
    let retained = dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).is_some() && !snapshot.objects.is_empty() && options.lower == LowerOptions::default();
    let (objects, trailer) = if retained {
        reconcile(snapshot)?
    } else {
        let LoweredDocument { objects, root, info, .. } = lower_document(snapshot, 1, options.lower.clone())?;
        (Cow::Owned(objects), DocumentTrailer { root, info, id: snapshot.document_id.clone(), extra: trailer_extra(&snapshot.trailer) })
    };
    Ok(serialize_document(&version, &objects, &trailer, &write))
}

/// 🧾 Trailer entries beyond the identity the writer states itself (`/Root`, `/Info`, `/ID`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn trailer_extra(trailer: &[PdfDictEntry]) -> Vec<PdfDictEntry> {
    trailer.iter().filter(|entry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID")).cloned().collect()
}

/// 🧾 The trailer a reconciled graph is written under: the graph's own `/Root`, `/Info` and extra
/// entries, and the typed `document_id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn retained_trailer(snapshot: &PdfSnapshot, trailer: &[PdfDictEntry]) -> PResult<DocumentTrailer> {
    let root = dict_get(trailer, "Root").and_then(PdfObject::as_ref).ok_or_else(|| PdfEngineError::Malformed("the retained trailer names no /Root".into()))?;
    Ok(DocumentTrailer { root, info: dict_get(trailer, "Info").and_then(PdfObject::as_ref), id: snapshot.document_id.clone(), extra: trailer_extra(trailer) })
}

/// 🔁 Rewrites references inside a retained object through `map`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rewrite_refs(value: &PdfObject, map: &HashMap<u32, ObjRef>) -> PdfObject {
    match value {
        PdfObject::Ref(reference) => PdfObject::Ref(map.get(&reference.num).copied().unwrap_or(*reference)),
        PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| rewrite_refs(item, map)).collect()),
        PdfObject::Dict(entries) => PdfObject::Dict(entries.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: rewrite_refs(&entry.value, map) }).collect()),
        PdfObject::Stream { dict, data, filters } => PdfObject::Stream { dict: dict.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: rewrite_refs(&entry.value, map) }).collect(), data: data.clone(), filters: filters.clone() },
        other => other.clone(),
    }
}

/// 🔗 Every object number `value` references, skipping the entries named in `skip`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn referenced(value: &PdfObject, skip: &[&str], out: &mut Vec<u32>) {
    match value {
        PdfObject::Ref(reference) => out.push(reference.num),
        PdfObject::Array(items) => items.iter().for_each(|item| referenced(item, skip, out)),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().filter(|entry| !skip.contains(&entry.key.as_str())).for_each(|entry| referenced(&entry.value, skip, out)),
        _ => {}
    }
}

/// 🧭 A lifting source that remembers every object the typed lanes reached.
struct Recording<S> {
    inner: S,
    seen: HashSet<u32>,
}

impl<S: ObjectSource> ObjectSource for Recording<S> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.seen.insert(reference.num);
        self.inner.get(reference)
    }
}

/// 🕸️ A lifting source over a graph being grafted.
struct GraphView<'a>(&'a BTreeMap<u32, PdfIndirectObject>);

impl ObjectSource for GraphView<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.0.get(&reference.num).map(|object| object.value.clone())
    }
}

/// 🔭 What a retained graph spells: the typed lanes lifted from it and how they map back onto its
/// objects.
struct Reading {
    lanes: PdfSnapshot,
    ids: HashMap<(Category, ObjRef), String>,
    refs: HashMap<(Category, String), ObjRef>,
    page_refs: Vec<ObjRef>,
    annotation_refs: HashMap<ObjRef, (u32, u32)>,
    seen: HashSet<u32>,
}

impl Reading {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn of(source: impl ObjectSource, trailer: &[PdfDictEntry], declared_version: &str) -> Self {
        let mut recording = Recording { inner: source, seen: HashSet::new() };
        let (lanes, ids, page_refs, annotation_refs) = {
            let lifter = lift_document_with(trailer, declared_version, &mut recording);
            (lifter.snapshot, lifter.ids, lifter.page_refs, lifter.annotation_refs)
        };
        let refs = ids.iter().map(|((category, reference), id)| ((*category, id.clone()), *reference)).collect();
        Self { lanes, ids, refs, page_refs, annotation_refs, seen: recording.seen }
    }

    /// 🗺️ Where every resource this reading names lives, and every page in `pages` order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn targets(&self, pages: &[ObjRef]) -> HashMap<(Category, String), ObjRef> {
        let mut refs = self.refs.clone();
        refs.extend(pages.iter().enumerate().map(|(index, reference)| ((Category::Page, index.to_string()), *reference)));
        refs
    }
}

/// 🗂️ A catalog lane a graft re-states entry by entry (ISO 32000-1 §7.7.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CatalogLane {
    Outlines,
    NamedDestinations,
    PageLabels,
    OutputIntents,
    PageLayout,
    PageMode,
    ViewerPreferences,
    OpenAction,
    Language,
    MarkInfo,
    Metadata,
    Extra,
}

impl CatalogLane {
    const ALL: [Self; 12] = [Self::Outlines, Self::NamedDestinations, Self::PageLabels, Self::OutputIntents, Self::PageLayout, Self::PageMode, Self::ViewerPreferences, Self::OpenAction, Self::Language, Self::MarkInfo, Self::Metadata, Self::Extra];

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn moved(self, read: &PdfSnapshot, typed: &PdfSnapshot) -> bool {
        match self {
            Self::Outlines => read.outlines != typed.outlines,
            Self::NamedDestinations => read.named_destinations != typed.named_destinations,
            Self::PageLabels => read.page_labels != typed.page_labels,
            Self::OutputIntents => read.output_intents != typed.output_intents,
            Self::PageLayout => read.page_layout != typed.page_layout,
            Self::PageMode => read.page_mode != typed.page_mode,
            Self::ViewerPreferences => read.viewer_preferences != typed.viewer_preferences,
            Self::OpenAction => read.open_action != typed.open_action,
            Self::Language => read.language != typed.language,
            Self::MarkInfo => read.mark_info != typed.mark_info,
            Self::Metadata => read.metadata != typed.metadata,
            Self::Extra => read.catalog_extra != typed.catalog_extra,
        }
    }

    /// 🔑 The catalog key the lane owns (named destinations live in the `/Names` tree, extra entries under their own keys).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn key(self) -> &'static str {
        match self {
            Self::Outlines => "Outlines",
            Self::NamedDestinations => "Dests",
            Self::PageLabels => "PageLabels",
            Self::OutputIntents => "OutputIntents",
            Self::PageLayout => "PageLayout",
            Self::PageMode => "PageMode",
            Self::ViewerPreferences => "ViewerPreferences",
            Self::OpenAction => "OpenAction",
            Self::Language => "Lang",
            Self::MarkInfo => "MarkInfo",
            Self::Metadata => "Metadata",
            Self::Extra => "",
        }
    }
}

/// 🧭 The typed lanes that differ from what a retained graph spells, grouped by how a write
/// honors them: pages, info and the catalog lanes are grafted; the resource collections are kept
/// alive and ordered through the page tree root when their values are the graph's own; the rest
/// only a regeneration can express.
#[derive(PartialEq)]
struct Moved {
    pages: bool,
    info: bool,
    resources: bool,
    catalog: Vec<CatalogLane>,
    regenerate: bool,
}

impl Moved {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(read: &PdfSnapshot, typed: &PdfSnapshot) -> Self {
        Self {
            pages: read.pages != typed.pages,
            info: read.info != typed.info,
            resources: read.fonts != typed.fonts || read.images != typed.images || read.forms != typed.forms || read.ext_g_states != typed.ext_g_states || read.shadings != typed.shadings || read.patterns != typed.patterns || read.color_spaces != typed.color_spaces || read.properties != typed.properties,
            catalog: CatalogLane::ALL.into_iter().filter(|lane| lane.moved(read, typed)).collect(),
            regenerate: read.declared_version != typed.declared_version || read.embedded_files != typed.embedded_files || read.acro_form != typed.acro_form || read.optional_content != typed.optional_content,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        !self.pages && !self.info && !self.resources && self.catalog.is_empty() && !self.regenerate
    }
}

/// 🔁 How many graft-and-read-back rounds a write spends before it regenerates: the first grafts
/// the lanes the typed model moved, the next ones what re-stating them moved in turn (page
/// indices a structural page edit shifts, resource discovery order). A round that grafts a set of
/// lanes and reads back exactly that set still moved has reached the graft's fixed point: those
/// lanes hold what no write can spell (a destination to a page index the document no longer has),
/// and a regeneration lowers them identically, so the graft is written as it stands — unless a
/// resource collection is among them, whose order a regeneration can still restate.
const GRAFT_ROUNDS: usize = 3;

/// 🪡 Reconciles the typed lanes onto the retained graph with incremental-writer semantics. A
/// graph that already spells every lane is written as it stands. A moved lane is grafted: only
/// the objects and entries it owns are re-stated, every other retained object and entry — direct
/// COS edits and trailer entries included — survives untouched, and the grafted graph is read
/// back until it spells the typed lanes. Only a lane no graft can express regenerates the typed
/// objects wholesale ([`regenerate`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reconcile(snapshot: &PdfSnapshot) -> PResult<(Cow<'_, [PdfIndirectObject]>, DocumentTrailer)> {
    let original = Reading::of(GraphSource::new(&snapshot.objects), &snapshot.trailer, &snapshot.declared_version);
    let mut moved = Moved::between(&original.lanes, snapshot);
    if moved.is_empty() {
        return Ok((Cow::Borrowed(snapshot.objects.as_slice()), retained_trailer(snapshot, &snapshot.trailer)?));
    }
    if !moved.regenerate {
        let mut graft = Graft::new(snapshot);
        let mut current: Option<Reading> = None;
        for _ in 0..GRAFT_ROUNDS {
            if !graft.round(current.as_ref().unwrap_or(&original), &original, &moved)? {
                break;
            }
            let reading = graft.read();
            let still = Moved::between(&reading.lanes, snapshot);
            if still.is_empty() || (current.is_some() && still == moved && !still.resources) {
                return graft.finish();
            }
            if still.regenerate {
                break;
            }
            moved = still;
            current = Some(reading);
        }
    }
    regenerate(snapshot, &original)
}

/// ♻️ Regenerates every object the typed lanes own from the typed lanes and keeps every other
/// retained object, references to moved objects rewritten.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn regenerate<'a>(snapshot: &'a PdfSnapshot, original: &Reading) -> PResult<(Cow<'a, [PdfIndirectObject]>, DocumentTrailer)> {
    let retained = retained_trailer(snapshot, &snapshot.trailer)?;
    let owned: HashSet<u32> = original.seen.iter().copied().chain(std::iter::once(retained.root.num)).chain(retained.info.map(|info| info.num)).collect();
    let first_number = snapshot.objects.iter().map(|object| object.id.num).max().unwrap_or(0) + 1;
    let lowered = lower_document(snapshot, first_number, LowerOptions::default())?;
    let mut rewrite: HashMap<u32, ObjRef> = HashMap::new();
    for ((category, old_ref), id) in &original.ids {
        if let Some(new_ref) = lowered.refs.get(&(*category, id.clone())) {
            rewrite.insert(old_ref.num, *new_ref);
        }
    }
    for (index, old_ref) in original.page_refs.iter().enumerate() {
        if let Some(new_ref) = lowered.refs.get(&(Category::Page, index.to_string())) {
            rewrite.insert(old_ref.num, *new_ref);
        }
    }
    for (old_ref, (page, index)) in &original.annotation_refs {
        if let Some(new_ref) = lowered.annotation_refs.get(*page as usize).and_then(|refs| refs.get(*index as usize)) {
            rewrite.insert(old_ref.num, *new_ref);
        }
    }
    rewrite.insert(retained.root.num, lowered.root);
    if let (Some(old), Some(new)) = (retained.info, lowered.info) {
        rewrite.insert(old.num, new);
    }
    let mut objects: Vec<PdfIndirectObject> = snapshot.objects.iter().filter(|object| !owned.contains(&object.id.num)).map(|object| PdfIndirectObject { id: object.id, value: rewrite_refs(&object.value, &rewrite) }).collect();
    objects.extend(lowered.objects);
    objects.sort_by_key(|object| object.id.num);
    let extra = retained.extra.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: rewrite_refs(&entry.value, &rewrite) }).collect();
    Ok((Cow::Owned(objects), DocumentTrailer { root: lowered.root, info: lowered.info, id: snapshot.document_id.clone(), extra }))
}

/// 📐 The page attributes a page inherits from its page tree ancestors (ISO 32000-1 §7.7.3.4).
const INHERITED_PAGE_KEYS: [&str; 4] = ["Resources", "MediaBox", "CropBox", "Rotate"];

/// 🪡 A graft in progress: the retained graph being patched and its trailer, the next free
/// object number, and every value a patch displaced — collected at the end once nothing reaches
/// it, so the objects a lane stopped owning leave the file while unrelated orphans stay.
struct Graft<'a> {
    typed: &'a PdfSnapshot,
    graph: BTreeMap<u32, PdfIndirectObject>,
    trailer: Vec<PdfDictEntry>,
    next: u32,
    displaced: Vec<PdfObject>,
}

impl<'a> Graft<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(typed: &'a PdfSnapshot) -> Self {
        let graph: BTreeMap<u32, PdfIndirectObject> = typed.objects.iter().map(|object| (object.id.num, object.clone())).collect();
        let next = graph.keys().next_back().copied().unwrap_or(0) + 1;
        Self { typed, graph, trailer: typed.trailer.clone(), next, displaced: Vec::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read(&self) -> Reading {
        Reading::of(GraphView(&self.graph), &self.trailer, &self.typed.declared_version)
    }

    /// 🔁 One graft round against `reading` (what the graph spells now); `original` is the
    /// untouched graph's reading, the only source a resource may be retained from. `false` when a
    /// moved lane is beyond what a graft can express.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn round(&mut self, reading: &Reading, original: &Reading, moved: &Moved) -> PResult<bool> {
        let pages = match moved.pages {
            true => match self.graft_pages(reading)? {
                Some(pages) => pages,
                None => return Ok(false),
            },
            false => reading.page_refs.clone(),
        };
        if moved.info {
            self.graft_info();
        }
        if !moved.catalog.is_empty() {
            self.graft_catalog(reading, &pages, &moved.catalog)?;
        }
        Ok(!moved.resources || self.retain_resources(original, &pages))
    }

    /// 🏁 Collects what the grafts displaced and hands back the graph and its trailer.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish(mut self) -> PResult<(Cow<'a, [PdfIndirectObject]>, DocumentTrailer)> {
        self.collect_displaced();
        let trailer = retained_trailer(self.typed, &self.trailer)?;
        Ok((Cow::Owned(self.graph.into_values().collect()), trailer))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn reserve(&mut self) -> ObjRef {
        let reference = ObjRef { num: self.next, gen: 0 };
        self.next += 1;
        reference
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn value(&self, reference: ObjRef) -> Option<&PdfObject> {
        self.graph.get(&reference.num).map(|object| &object.value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve(&self, value: &PdfObject) -> Option<PdfObject> {
        match value {
            PdfObject::Ref(reference) => self.value(*reference).cloned(),
            other => Some(other.clone()),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict(&self, reference: ObjRef) -> Vec<PdfDictEntry> {
        self.value(reference).and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec).unwrap_or_default()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn set_dict(&mut self, reference: ObjRef, entries: Vec<PdfDictEntry>) {
        self.graph.insert(reference.num, PdfIndirectObject { id: reference, value: PdfObject::Dict(entries) });
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn catalog(&self) -> Option<ObjRef> {
        dict_get(&self.trailer, "Root").and_then(PdfObject::as_ref)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page_tree_root(&self) -> Option<ObjRef> {
        dict_get(&self.dict(self.catalog()?), "Pages").and_then(PdfObject::as_ref)
    }

    /// 🧬 Moves the lowered objects `value` reaches out of `scratch` into the graph under fresh
    /// numbers (one number per scratch object across a batch, recorded in `adopted`) and returns
    /// `value` with its references rewritten.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn adopt(&mut self, value: &PdfObject, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) -> PdfObject {
        let mut pending = Vec::new();
        referenced(value, &[], &mut pending);
        let mut fresh = Vec::new();
        while let Some(num) = pending.pop() {
            if adopted.contains_key(&num) {
                continue;
            }
            let Some(inner) = scratch.get(&num) else { continue };
            let target = self.reserve();
            adopted.insert(num, target);
            referenced(inner, &[], &mut pending);
            fresh.push(num);
        }
        for num in fresh {
            let target = adopted[&num];
            self.graph.insert(target.num, PdfIndirectObject { id: target, value: rewrite_refs(&scratch[&num], adopted) });
        }
        rewrite_refs(value, adopted)
    }

    /// 🔧 Re-states `key` in `entries`: `value` (adopted from `scratch`) replaces the entry,
    /// `None` drops it; the value it displaces is collected later.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn put(&mut self, entries: &mut Vec<PdfDictEntry>, key: &str, value: Option<PdfObject>, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) {
        let value = value.map(|value| self.adopt(&value, scratch, adopted));
        match (entries.iter().position(|entry| entry.key == key), value) {
            (Some(index), Some(value)) => {
                let old = std::mem::replace(&mut entries[index].value, value);
                self.displaced.push(old);
            }
            (Some(index), None) => {
                let old = entries.remove(index).value;
                self.displaced.push(old);
            }
            (None, Some(value)) => entries.push(PdfDictEntry::new(key, value)),
            (None, None) => {}
        }
    }

    /// 🧹 Drops every object only displaced values reached that nothing in the grafted document
    /// reaches any more (`/Parent` back-links do not keep an object in the candidate set).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn collect_displaced(&mut self) {
        let mut pending = Vec::new();
        for value in std::mem::take(&mut self.displaced) {
            referenced(&value, &["Parent"], &mut pending);
        }
        let mut candidates = HashSet::new();
        while let Some(num) = pending.pop() {
            if candidates.insert(num) {
                if let Some(object) = self.graph.get(&num) {
                    referenced(&object.value, &["Parent"], &mut pending);
                }
            }
        }
        if candidates.is_empty() {
            return;
        }
        let mut reachable = HashSet::new();
        let mut pending = Vec::new();
        self.trailer.iter().for_each(|entry| referenced(&entry.value, &[], &mut pending));
        while let Some(num) = pending.pop() {
            if reachable.insert(num) {
                if let Some(object) = self.graph.get(&num) {
                    referenced(&object.value, &[], &mut pending);
                }
            }
        }
        self.graph.retain(|num, _| !candidates.contains(num) || reachable.contains(num));
    }

    /// 📄 Grafts the page lane. A page the graph already spells keeps its object; a page whose
    /// fields moved is patched in place (only the moved entries re-stated); a new page is lowered
    /// fresh; and the page tree is re-stated flat under its root only when the page sequence
    /// itself changed. Returns the page references in typed order, `None` when a page carries
    /// what no graft can express.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn graft_pages(&mut self, reading: &Reading) -> PResult<Option<Vec<ObjRef>>> {
        let typed = self.typed;
        let read = &reading.lanes.pages;
        let Some(root) = self.page_tree_root() else { return Ok(None) };
        let mut source: Vec<Option<usize>> = (0..typed.pages.len()).map(|index| (index < read.len() && typed.pages[index] == read[index]).then_some(index)).collect();
        let mut used = vec![false; read.len()];
        source.iter().flatten().for_each(|slot| used[*slot] = true);
        for index in 0..typed.pages.len() {
            if source[index].is_none() {
                if let Some(slot) = (0..read.len()).find(|slot| !used[*slot] && read[*slot] == typed.pages[index]) {
                    used[slot] = true;
                    source[index] = Some(slot);
                }
            }
        }
        let mut patched = Vec::new();
        for index in 0..typed.pages.len() {
            if source[index].is_none() && index < read.len() && !used[index] {
                used[index] = true;
                source[index] = Some(index);
                patched.push(index);
            }
        }
        let mut pages = Vec::with_capacity(typed.pages.len());
        let mut inserted = Vec::new();
        for (index, slot) in source.iter().enumerate() {
            match slot {
                Some(slot) => pages.push(reading.page_refs[*slot]),
                None => {
                    inserted.push(index);
                    pages.push(self.reserve());
                }
            }
        }
        let removed: Vec<ObjRef> = (0..read.len()).filter(|slot| !used[*slot]).map(|slot| reading.page_refs[slot]).collect();
        let targets = reading.targets(&pages);
        for index in patched {
            if !self.patch_page(index, &read[index], pages[index], &targets)? {
                return Ok(None);
            }
        }
        for index in inserted {
            if !self.insert_page(index, pages[index], root, &targets)? {
                return Ok(None);
            }
        }
        if pages != reading.page_refs {
            self.restate_page_tree(root, &pages, &removed);
        }
        Ok(Some(pages))
    }

    /// ✏️ Patches the retained page at `reference` so it spells typed page `index`: the entries of
    /// every moved field are re-stated from the page lowered afresh, every other entry (content
    /// stream, resources, annotations, keys no field owns) stays as the graph holds it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn patch_page(&mut self, index: usize, read: &PdfPage, reference: ObjRef, targets: &HashMap<(Category, String), ObjRef>) -> PResult<bool> {
        let snapshot = self.typed;
        let typed = &snapshot.pages[index];
        let content = typed.content != read.content;
        let annotations = typed.annotations != read.annotations;
        if annotations && (carries_widget(typed) || carries_widget(read)) {
            return Ok(false);
        }
        let mut probe = typed.clone();
        if !content {
            probe.content.clear();
        }
        if !annotations {
            probe.annotations.clear();
        }
        let (objects, _) = lower_page_standalone(snapshot, &probe, index, reference, reference, self.next, LowerOptions::default(), targets, &HashMap::new())?;
        let mut scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let lowered = scratch.remove(&reference.num).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        let fields: [(bool, &str); 16] = [
            (typed.media_box != read.media_box, "MediaBox"),
            (typed.crop_box != read.crop_box, "CropBox"),
            (typed.bleed_box != read.bleed_box, "BleedBox"),
            (typed.trim_box != read.trim_box, "TrimBox"),
            (typed.art_box != read.art_box, "ArtBox"),
            (typed.rotate != read.rotate, "Rotate"),
            (typed.user_unit != read.user_unit, "UserUnit"),
            (content, "Contents"),
            (annotations, "Annots"),
            (typed.group != read.group, "Group"),
            (typed.thumbnail != read.thumbnail, "Thumb"),
            (typed.struct_parents != read.struct_parents, "StructParents"),
            (typed.transition != read.transition, "Trans"),
            (typed.duration != read.duration, "Dur"),
            (typed.metadata != read.metadata, "Metadata"),
            (typed.additional_actions != read.additional_actions, "AA"),
        ];
        let mut keys: Vec<&str> = fields.iter().filter(|(moved, _)| *moved).map(|(_, key)| *key).collect();
        if typed.extra != read.extra {
            keys.extend(read.extra.iter().chain(&typed.extra).map(|entry| entry.key.as_str()));
        }
        let mut seen = HashSet::new();
        keys.retain(|key| seen.insert(*key));
        let mut dict = self.dict(reference);
        let mut adopted = HashMap::new();
        for key in keys {
            let value = match (key, dict_get(&lowered, key)) {
                ("Rotate", None) => Some(PdfObject::Int(typed.rotate as i64)),
                (_, value) => value.cloned(),
            };
            self.put(&mut dict, key, value, &scratch, &mut adopted);
        }
        if content && !self.bind_resources(&mut dict, &typed.content, targets) {
            return Ok(false);
        }
        self.set_dict(reference, dict);
        Ok(true)
    }

    /// ➕ Lowers typed page `index` fresh at the reserved `reference` under `parent`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn insert_page(&mut self, index: usize, reference: ObjRef, parent: ObjRef, targets: &HashMap<(Category, String), ObjRef>) -> PResult<bool> {
        let snapshot = self.typed;
        let page = &snapshot.pages[index];
        let references = content_references(&page.content);
        if carries_widget(page) || resource_sections(&references).iter().any(|(_, category, ids)| ids.iter().any(|id| !targets.contains_key(&(*category, id.clone())) && self.defines(*category, id))) {
            return Ok(false);
        }
        let (objects, _) = lower_page_standalone(snapshot, page, index, reference, parent, self.next, LowerOptions::default(), targets, &HashMap::new())?;
        let mut scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let lowered = scratch.remove(&reference.num).unwrap_or(PdfObject::Dict(Vec::new()));
        let value = self.adopt(&lowered, &scratch, &mut HashMap::new());
        self.graph.insert(reference.num, PdfIndirectObject { id: reference, value });
        Ok(true)
    }

    /// 🆔 Whether a typed resource collection defines `id` in `category`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn defines(&self, category: Category, id: &str) -> bool {
        let typed = self.typed;
        match category {
            Category::Font => typed.font(id).is_some(),
            Category::XObject => typed.image(id).is_some() || typed.form(id).is_some(),
            Category::ExtGState => typed.ext_g_state(id).is_some(),
            Category::Shading => typed.shading(id).is_some(),
            Category::Pattern => typed.pattern(id).is_some(),
            Category::ColorSpace => typed.color_spaces.iter().any(|space| space.name == id),
            Category::Properties => typed.properties.iter().any(|properties| properties.name == id),
            _ => false,
        }
    }

    /// 📚 Makes the page's resources bind every id its new content names: the resources it
    /// already sees (its own or inherited) are kept whole, and an id they do not bind to the
    /// graph's object for it is bound on a page-own copy. `false` when the content names a typed
    /// resource the graph holds no object for.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bind_resources(&mut self, dict: &mut Vec<PdfDictEntry>, ops: &[PdfOp], targets: &HashMap<(Category, String), ObjRef>) -> bool {
        let effective = self.effective_resources(dict);
        let mut resources = effective.clone();
        for (key, category, ids) in resource_sections(&content_references(ops)) {
            let mut section = dict_get(&resources, key).and_then(|value| self.resolve(value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
            let mut bound = false;
            for id in ids {
                match targets.get(&(category, id.clone())) {
                    Some(target) if dict_get(&section, &id) != Some(&PdfObject::Ref(*target)) => {
                        section.retain(|entry| entry.key != id);
                        section.push(PdfDictEntry::new(id, PdfObject::Ref(*target)));
                        bound = true;
                    }
                    None if self.defines(category, &id) && dict_get(&section, &id).is_none() => return false,
                    _ => {}
                }
            }
            if bound {
                resources.retain(|entry| entry.key != key);
                resources.push(PdfDictEntry::new(key, PdfObject::Dict(section)));
            }
        }
        if resources != effective {
            let own = PdfObject::Dict(resources);
            match dict.iter_mut().find(|entry| entry.key == "Resources") {
                Some(entry) => self.displaced.push(std::mem::replace(&mut entry.value, own)),
                None => dict.push(PdfDictEntry::new("Resources", own)),
            }
        }
        true
    }

    /// 📚 The resource dictionary a page sees: its own, else the nearest ancestor's.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn effective_resources(&self, dict: &[PdfDictEntry]) -> Vec<PdfDictEntry> {
        self.inherited(dict, "Resources", None).and_then(|value| self.resolve(&value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default()
    }

    /// 🧬 The inherited attribute `key` of a page dictionary: its own entry, else the nearest
    /// ancestor's below `stop`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inherited(&self, dict: &[PdfDictEntry], key: &str, stop: Option<ObjRef>) -> Option<PdfObject> {
        if let Some(value) = dict_get(dict, key) {
            return Some(value.clone());
        }
        let mut parent = dict_get(dict, "Parent").and_then(PdfObject::as_ref);
        for _ in 0..64 {
            let node = parent.filter(|node| Some(node.num) != stop.map(|stop| stop.num))?;
            let entries = self.dict(node);
            if let Some(value) = dict_get(&entries, key) {
                return Some(value.clone());
            }
            parent = dict_get(&entries, "Parent").and_then(PdfObject::as_ref);
        }
        None
    }

    /// 🌳 Re-states the page tree flat under `root` (§7.7.3.2): every page a direct kid in typed
    /// order, carrying the attributes it inherited from the intermediate nodes it leaves; the
    /// intermediate nodes and the removed pages are displaced.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn restate_page_tree(&mut self, root: ObjRef, pages: &[ObjRef], removed: &[ObjRef]) {
        for page in pages {
            let mut dict = self.dict(*page);
            for key in INHERITED_PAGE_KEYS {
                if dict_get(&dict, key).is_none() {
                    if let Some(value) = self.inherited(&dict, key, Some(root)) {
                        dict.push(PdfDictEntry::new(key, value));
                    }
                }
            }
            dict.retain(|entry| entry.key != "Parent");
            dict.push(PdfDictEntry::new("Parent", PdfObject::Ref(root)));
            self.set_dict(*page, dict);
        }
        let mut tree = self.dict(root);
        let kids = PdfObject::Array(pages.iter().map(|page| PdfObject::Ref(*page)).collect());
        match tree.iter_mut().find(|entry| entry.key == "Kids") {
            Some(entry) => self.displaced.push(std::mem::replace(&mut entry.value, kids)),
            None => tree.push(PdfDictEntry::new("Kids", kids)),
        }
        tree.retain(|entry| entry.key != "Count");
        tree.push(PdfDictEntry::new("Count", PdfObject::Int(pages.len() as i64)));
        self.set_dict(root, tree);
        self.displaced.extend(removed.iter().map(|page| PdfObject::Ref(*page)));
    }

    /// ℹ️ Grafts the information dictionary: re-stated in place when the graph holds one, added
    /// when it does not, dropped from the trailer when the typed record is empty.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn graft_info(&mut self) {
        let snapshot = self.typed;
        let info = &snapshot.info;
        let current = dict_get(&self.trailer, "Info").and_then(PdfObject::as_ref).filter(|reference| self.graph.contains_key(&reference.num));
        if info.is_empty() {
            if let Some(index) = self.trailer.iter().position(|entry| entry.key == "Info") {
                let old = self.trailer.remove(index).value;
                self.displaced.push(old);
            }
            return;
        }
        let reference = current.unwrap_or_else(|| self.reserve());
        if let Some(old) = self.graph.insert(reference.num, PdfIndirectObject { id: reference, value: PdfObject::Dict(lower_info(info)) }) {
            self.displaced.push(old.value);
        }
        self.trailer.retain(|entry| entry.key != "Info");
        self.trailer.push(PdfDictEntry::new("Info", PdfObject::Ref(reference)));
    }

    /// 🗂️ Grafts the moved catalog lanes: the catalog is lowered afresh against the graph (its
    /// destinations resolve to the graph's pages) and only the entries the moved lanes own are
    /// re-stated in the retained catalog; named destinations re-state the `/Names` tree's `/Dests`
    /// (superseding a PDF 1.1 catalog `/Dests`), extra entries re-state exactly the keys they carry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn graft_catalog(&mut self, reading: &Reading, pages: &[ObjRef], lanes: &[CatalogLane]) -> PResult<()> {
        let (Some(catalog_ref), Some(root)) = (self.catalog(), self.page_tree_root()) else { return Ok(()) };
        let (lowered, objects) = lower_catalog_standalone(self.typed, self.next, LowerOptions::default(), &reading.targets(pages), root)?;
        let scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let mut catalog = self.dict(catalog_ref);
        let mut adopted = HashMap::new();
        for lane in lanes {
            match lane {
                CatalogLane::NamedDestinations => {
                    self.put(&mut catalog, "Dests", None, &scratch, &mut adopted);
                    let dests = dict_get(&lowered, "Names").and_then(|names| names.dict_get("Dests")).cloned();
                    self.put_named_destinations(&mut catalog, dests, &scratch, &mut adopted);
                }
                CatalogLane::Extra => {
                    for entry in &reading.lanes.catalog_extra {
                        self.put(&mut catalog, &entry.key, None, &scratch, &mut adopted);
                    }
                    catalog.extend(self.typed.catalog_extra.iter().cloned());
                }
                lane => self.put(&mut catalog, lane.key(), dict_get(&lowered, lane.key()).cloned(), &scratch, &mut adopted),
            }
        }
        self.set_dict(catalog_ref, catalog);
        Ok(())
    }

    /// 🎯 Re-states `/Names /Dests`, editing the `/Names` dictionary where the graph keeps it and
    /// dropping it once it holds nothing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn put_named_destinations(&mut self, catalog: &mut Vec<PdfDictEntry>, dests: Option<PdfObject>, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) {
        let held = dict_get(catalog, "Names").cloned();
        let mut names = held.as_ref().and_then(|value| self.resolve(value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        self.put(&mut names, "Dests", dests, scratch, adopted);
        match (held.as_ref().and_then(PdfObject::as_ref), names.is_empty()) {
            (Some(reference), false) if self.graph.contains_key(&reference.num) => self.set_dict(reference, names),
            (_, true) => self.put(catalog, "Names", None, scratch, adopted),
            (_, false) => self.put(catalog, "Names", Some(PdfObject::Dict(names)), scratch, adopted),
        }
    }

    /// 🗃️ Keeps every typed resource alive and in typed order through the page tree root's
    /// `/Resources` (§7.7.3.4) — the only way a graft can restate a resource collection whose
    /// values are the graph's own when a page edit changed which pages reach them, or in what
    /// order. Pages that inherited the root's resources get them as their own first. `false` when
    /// a typed resource is not the original graph's object.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn retain_resources(&mut self, original: &Reading, pages: &[ObjRef]) -> bool {
        fn section<T: PartialEq>(key: &'static str, category: Category, typed: &[T], read: &[T], id: impl Fn(&T) -> &str, refs: &HashMap<(Category, String), ObjRef>) -> Option<(&'static str, Vec<PdfDictEntry>)> {
            let entries = typed.iter().map(|item| read.contains(item).then(|| refs.get(&(category, id(item).to_string())).map(|reference| PdfDictEntry::new(id(item), PdfObject::Ref(*reference)))).flatten()).collect::<Option<Vec<_>>>()?;
            Some((key, entries))
        }
        let (typed, read, refs) = (self.typed, &original.lanes, &original.refs);
        let x_objects: Option<Vec<PdfDictEntry>> = section("XObject", Category::XObject, &typed.images, &read.images, |image| image.id.as_str(), refs).zip(section("XObject", Category::XObject, &typed.forms, &read.forms, |form| form.id.as_str(), refs)).map(|((_, images), (_, forms))| images.into_iter().chain(forms).collect());
        let sections = [
            section("Font", Category::Font, &typed.fonts, &read.fonts, |font| font.id.as_str(), refs),
            x_objects.map(|entries| ("XObject", entries)),
            section("ExtGState", Category::ExtGState, &typed.ext_g_states, &read.ext_g_states, |state| state.id.as_str(), refs),
            section("Shading", Category::Shading, &typed.shadings, &read.shadings, |shading| shading.id.as_str(), refs),
            section("Pattern", Category::Pattern, &typed.patterns, &read.patterns, |pattern| pattern.id.as_str(), refs),
            section("ColorSpace", Category::ColorSpace, &typed.color_spaces, &read.color_spaces, |space| space.name.as_str(), refs),
            section("Properties", Category::Properties, &typed.properties, &read.properties, |properties| properties.name.as_str(), refs),
        ];
        let Some(sections) = sections.into_iter().collect::<Option<Vec<_>>>() else { return false };
        let Some(root) = self.page_tree_root() else { return false };
        let resources = PdfObject::Dict(sections.into_iter().filter(|(_, entries)| !entries.is_empty()).map(|(key, entries)| PdfDictEntry::new(key, PdfObject::Dict(entries))).collect());
        let mut tree = self.dict(root);
        let inherited = dict_get(&tree, "Resources").cloned().unwrap_or(PdfObject::Dict(Vec::new()));
        for page in pages {
            let mut dict = self.dict(*page);
            if self.inherited(&dict, "Resources", Some(root)).is_none() {
                dict.push(PdfDictEntry::new("Resources", inherited.clone()));
                self.set_dict(*page, dict);
            }
        }
        tree.retain(|entry| entry.key != "Resources");
        tree.push(PdfDictEntry::new("Resources", resources));
        self.set_dict(root, tree);
        true
    }
}

/// 📚 A content stream's resource names by resource dictionary section (§7.8.3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resource_sections(references: &ContentReferences) -> [(&'static str, Category, Vec<String>); 7] {
    [
        ("Font", Category::Font, references.fonts.clone()),
        ("XObject", Category::XObject, references.x_objects.clone()),
        ("ExtGState", Category::ExtGState, references.ext_g_states.clone()),
        ("Shading", Category::Shading, references.shadings.clone()),
        ("Pattern", Category::Pattern, references.patterns.clone()),
        ("ColorSpace", Category::ColorSpace, references.color_spaces.clone()),
        ("Properties", Category::Properties, references.properties.clone()),
    ]
}

/// 🧷 Whether a page carries an interactive-form widget, whose field back-links only a whole
/// interactive-form write can re-state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn carries_widget(page: &PdfPage) -> bool {
    page.annotations.iter().any(|annotation| matches!(annotation.kind, PdfAnnotationKind::Widget { .. }))
}
//#endregion 🔖️Encode

//#region 🔖️GraphEdit
/// 🪢 Carries a retained-graph edit into the typed lanes: `next` is `base` with its COS graph
/// (`objects`, `trailer`) edited, and every lane whose reading the edit moved takes the edited
/// graph's reading while every other lane keeps what `base` holds — so the next write never
/// undoes a direct graph edit with a stale typed lane, and a typed edit pending in another lane
/// survives it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn carry_graph_edit(base: &PdfSnapshot, next: &mut PdfSnapshot) {
    if base.objects == next.objects && base.trailer == next.trailer {
        return;
    }
    let before = lift_document(&base.trailer, &base.declared_version, &mut GraphSource::new(&base.objects));
    let after = lift_document(&next.trailer, &base.declared_version, &mut GraphSource::new(&next.objects));
    macro_rules! carry {
        ($($lane:ident),* $(,)?) => {
            $(if before.$lane != after.$lane {
                next.$lane = after.$lane;
            })*
        };
    }
    carry!(declared_version, pages, fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, outlines, named_destinations, page_labels, embedded_files, output_intents, acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, info, catalog_extra);
}
//#endregion 🔖️GraphEdit

//#region 🔖️Streaming
/// 🌊 A page-at-a-time document writer for guests with per-step budgets: the document-level
/// lanes are lowered up front, then every `page` call yields that page's bytes, and `finish`
/// closes the file. Pages appended this way reference the snapshot's fonts, images, forms and
/// graphics states by id exactly like [`PdfPage::content`] does.
pub struct DocumentStream {
    writer: PdfWriter,
    lowered: LoweredDocument,
    pages_ref: ObjRef,
    page_refs: Vec<ObjRef>,
    written_pages: usize,
    next_number: u32,
    snapshot: PdfSnapshot,
    options: LowerOptions,
    root_resources: PdfObject,
    annotation_refs: Vec<Vec<ObjRef>>,
}

impl DocumentStream {
    /// 🏁 Lowers the document-level lanes of `snapshot` (its `pages` are ignored — they arrive
    /// through [`DocumentStream::page`]) for `expected_pages` pages and returns the header +
    /// resource bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn begin(snapshot: &PdfSnapshot, expected_pages: usize, options: EncodeOptions) -> PResult<(Self, Vec<u8>)> {
        let (lowered, pages_ref, page_refs, root_resources) = lower_document_headless(snapshot, expected_pages, options.lower.clone())?;
        let mut write = options.write.clone();
        if write.encryption.is_none() {
            write.encryption = snapshot.encryption.clone();
        }
        let version = if snapshot.declared_version.is_empty() { "1.7".to_string() } else { snapshot.declared_version.clone() };
        let seed = snapshot.document_id.as_ref().map(|id| id[0].clone()).unwrap_or_default();
        let (mut writer, mut out) = PdfWriter::begin(&version, &write, &seed, &seed);
        for object in lowered.objects.iter().filter(|object| object.id != pages_ref && object.id != lowered.root) {
            out.extend_from_slice(&writer.object(object));
        }
        let next_number = lowered.objects.iter().map(|object| object.id.num).max().unwrap_or(0).max(page_refs.iter().map(|r| r.num).max().unwrap_or(0)) + 1;
        let mut headless = snapshot.clone();
        headless.pages.clear();
        headless.objects.clear();
        headless.trailer.clear();
        Ok((Self { writer, lowered, pages_ref, page_refs, written_pages: 0, next_number, snapshot: headless, options: options.lower, root_resources, annotation_refs: Vec::new() }, out))
    }

    /// 📄 Appends the next page; returns its bytes (page object, content stream, annotations).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn page(&mut self, page: &PdfPage) -> PResult<Vec<u8>> {
        let index = self.written_pages;
        let page_ref = match self.page_refs.get(index) {
            Some(reference) => *reference,
            None => {
                let reference = ObjRef { num: self.next_number, gen: 0 };
                self.next_number += 1;
                self.page_refs.push(reference);
                reference
            }
        };
        let (objects, annotation_refs) = lower_page_standalone(&self.snapshot, page, index, page_ref, self.pages_ref, self.next_number, self.options.clone(), &self.lowered.refs, &self.lowered.widget_parents)?;
        self.annotation_refs.push(annotation_refs);
        let mut out = Vec::new();
        for object in &objects {
            self.next_number = self.next_number.max(object.id.num + 1);
            out.extend_from_slice(&self.writer.object(object));
        }
        self.written_pages += 1;
        Ok(out)
    }

    /// 🏁 Writes the page tree, catalog and cross-reference section; returns the closing bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn finish(mut self) -> PResult<Vec<u8>> {
        let mut out = Vec::new();
        let kids: Vec<PdfObject> = self.page_refs.iter().take(self.written_pages).map(|r| PdfObject::Ref(*r)).collect();
        let pages = PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("Pages")), PdfDictEntry::new("Kids", PdfObject::Array(kids)), PdfDictEntry::new("Count", PdfObject::Int(self.written_pages as i64)), PdfDictEntry::new("Resources", self.root_resources.clone())]);
        out.extend_from_slice(&self.writer.object(&PdfIndirectObject { id: self.pages_ref, value: pages }));
        let mut acro_form: Option<ObjRef> = None;
        if let Some((objects, reference)) = lower_acro_form_standalone(&self.snapshot, self.next_number, self.options.clone(), &self.lowered.refs, std::mem::take(&mut self.annotation_refs), self.lowered.field_refs.clone())? {
            for object in &objects {
                self.next_number = self.next_number.max(object.id.num + 1);
                out.extend_from_slice(&self.writer.object(object));
            }
            acro_form = Some(reference);
        }
        if let Some(mut catalog) = self.lowered.objects.iter().find(|object| object.id == self.lowered.root).cloned() {
            if let (Some(reference), PdfObject::Dict(entries)) = (acro_form, &mut catalog.value) {
                entries.push(PdfDictEntry::new("AcroForm", PdfObject::Ref(reference)));
            }
            out.extend_from_slice(&self.writer.object(&catalog));
        }
        let trailer = DocumentTrailer { root: self.lowered.root, info: self.lowered.info, id: self.snapshot.document_id.clone(), extra: Vec::new() };
        out.extend_from_slice(&self.writer.finish(&trailer));
        Ok(out)
    }
}
//#endregion 🔖️Streaming

//#region 🔖️Text
impl PdfSnapshot {
    /// 🔤 The Unicode text page `index` shows, raw codes decoded through the page's fonts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn page_text(&self, index: usize) -> String {
        let Some(page) = self.pages.get(index) else { return String::new() };
        let codecs: HashMap<String, FontCodec> = self.fonts.iter().map(|font| (font.id.clone(), FontCodec::new(font))).collect();
        crate::standards::v1_7::subsets::base::modules::content::extract_text(&page.content, &codecs)
    }
}
//#endregion 🔖️Text

//#region 🔖️Builders
/// 📐 Simple text layout over a font: line breaking on words by real advance widths, returning
/// the operators that show the lines top-down from `(x, top)`.
pub struct PdfTextLayout<'a> {
    pub font: &'a PdfFont,
    pub size: f64,
    pub leading: f64,
    codec: FontCodec,
}

impl<'a> PdfTextLayout<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(font: &'a PdfFont, size: f64) -> Self {
        Self { font, size, leading: size * 1.2, codec: FontCodec::new(font) }
    }

    /// 📏 Width of `text` at this size in user space (`None` when the font cannot show it).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self, text: &str) -> Option<f64> {
        self.codec.text_width(text).map(|width| width * self.size / 1000.0)
    }

    /// 📏 Ascent of the face at this size (AFM/descriptor metrics, else 0.8 em).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ascent(&self) -> f64 {
        let ascent = match &self.font.kind {
            PdfFontKind::Type1 { descriptor: Some(d), .. } | PdfFontKind::TrueType { descriptor: Some(d), .. } => d.ascent,
            PdfFontKind::Type0 { descendant, .. } => descendant.descriptor.ascent,
            _ => standard_font(self.font.base_font()).map(|metrics| metrics.ascender).unwrap_or(800.0),
        };
        (if ascent == 0.0 { 800.0 } else { ascent }) * self.size / 1000.0
    }

    /// 🔤 Breaks `text` into lines no wider than `max_width` (paragraphs split on `\n`; a word
    /// wider than the line is split by characters).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn wrap(&self, text: &str, max_width: f64) -> Vec<String> {
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split(' ') {
                let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
                if self.width(&candidate).unwrap_or(0.0) <= max_width || line.is_empty() && self.width(word).unwrap_or(0.0) <= max_width {
                    line = candidate;
                    continue;
                }
                if !line.is_empty() {
                    lines.push(std::mem::take(&mut line));
                }
                let mut piece = String::new();
                for character in word.chars() {
                    let next = format!("{piece}{character}");
                    if self.width(&next).unwrap_or(0.0) > max_width && !piece.is_empty() {
                        lines.push(std::mem::take(&mut piece));
                    }
                    piece.push(character);
                }
                line = piece;
            }
            lines.push(line);
        }
        lines
    }

    /// 🖋️ Operators showing `lines` from `(x, top)` downwards (the first baseline sits one ascent
    /// below `top`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn show_lines(&self, lines: &[String], x: f64, top: f64) -> Vec<PdfOp> {
        let mut ops = vec![PdfOp::BeginText, PdfOp::SetFont { name: self.font.id.clone(), size: self.size }, PdfOp::SetLeading { leading: self.leading }, PdfOp::MoveText { tx: x, ty: top - self.ascent() }];
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                ops.push(PdfOp::NextLine);
            }
            if !line.is_empty() {
                ops.push(PdfOp::ShowText { text: PdfTextString::text(line.clone()) });
            }
        }
        ops.push(PdfOp::EndText);
        ops
    }

    /// 🖋️ Wraps and shows `text` inside the rectangle `[x, top - height, x + width, top]`,
    /// dropping lines that do not fit; returns the operators and how many lines fit.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn show_paragraph(&self, text: &str, x: f64, top: f64, width: f64, height: f64) -> (Vec<PdfOp>, usize) {
        let lines = self.wrap(text, width);
        let fitting = ((height - self.ascent()) / self.leading).floor().max(0.0) as usize + 1;
        let shown: Vec<String> = lines.into_iter().take(fitting).collect();
        let count = shown.len();
        (self.show_lines(&shown, x, top), count)
    }
}

/// 📄️ A document of text pages: one page per `(width, height, text)` in Helvetica 12 pt with
/// 72 pt margins, wrapped by real metrics — the shape every text-only exporter shares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn text_document(pages: &[(f64, f64, &str)]) -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    let font = PdfFont::standard("F1", "Helvetica");
    let layout = PdfTextLayout::new(&font, 12.0);
    for (width, height, text) in pages {
        let mut page = PdfPage::new(*width, *height);
        if !text.is_empty() {
            let margin = (width.min(*height) * 0.1).min(72.0);
            let (ops, _) = layout.show_paragraph(text, margin, height - margin, width - 2.0 * margin, height - 2.0 * margin);
            page.content = ops;
        }
        snapshot.pages.push(page);
    }
    if snapshot.pages.iter().any(|page| !page.content.is_empty()) {
        snapshot.fonts.push(font);
    }
    snapshot
}

/// 🔤 An embedded TrueType font as a Type 0 / CIDFontType2 (Identity-H, glyph ids as CIDs) with
/// widths and a ToUnicode map for every character of `text` (the whole cmap when `None`) — the
/// font shape any consumer with a `.ttf` in hand wants.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn embedded_true_type_font(id: &str, program: &[u8], base_font: &str, text: Option<&str>) -> Result<PdfFont, String> {
    use crate::standards::v1_7::subsets::base::modules::fonts::TrueTypeFont;
    let font = TrueTypeFont::parse(program)?;
    let characters: Vec<char> = match text {
        Some(text) => {
            let mut set: Vec<char> = text.chars().collect();
            set.sort_unstable();
            set.dedup();
            set
        }
        None => font.unicode_map.keys().filter_map(|code| char::from_u32(*code)).collect(),
    };
    let mut widths: BTreeMap<u32, f64> = BTreeMap::new();
    let mut mappings = Vec::new();
    for character in characters {
        if let Some(gid) = font.glyph_for_char(character) {
            widths.insert(gid as u32, font.advance_1000(gid));
            mappings.push(PdfToUnicodeMapping::Char { code: gid as u32, text: character.to_string() });
        }
    }
    let mut runs: Vec<PdfCidWidthRun> = Vec::new();
    for (gid, width) in widths {
        match runs.last_mut() {
            Some(run) if run.start_cid + run.widths.len() as u32 == gid => run.widths.push(width),
            _ => runs.push(PdfCidWidthRun { start_cid: gid, widths: vec![width] }),
        }
    }
    let flags = if font.fixed_pitch { 1 } else { 0 } | 32 | if font.italic_angle != 0.0 { 64 } else { 0 } | if font.weight_class.unwrap_or(400) >= 600 { 1 << 18 } else { 0 };
    let descriptor = PdfFontDescriptor {
        font_name: base_font.to_string(),
        flags,
        font_bbox: [font.scale_1000(font.bbox[0]), font.scale_1000(font.bbox[1]), font.scale_1000(font.bbox[2]), font.scale_1000(font.bbox[3])],
        italic_angle: font.italic_angle,
        ascent: font.scale_1000(font.ascender),
        descent: font.scale_1000(font.descender),
        cap_height: font.cap_height.map(|v| font.scale_1000(v)).unwrap_or_else(|| font.scale_1000(font.ascender)),
        stem_v: 80.0,
        x_height: font.x_height.map(|v| font.scale_1000(v)),
        ..PdfFontDescriptor::default()
    };
    Ok(PdfFont {
        id: id.to_string(),
        kind: PdfFontKind::Type0 {
            base_font: base_font.to_string(),
            cmap: PdfCMap::identity_h(),
            descendant: PdfCidFont { true_type: true, base_font: base_font.to_string(), system_info: PdfCidSystemInfo::default(), descriptor, default_width: 1000.0, widths: runs, default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: Some(PdfCidToGid::Identity), program: Some(PdfFontProgram::TrueType { data: program.to_vec() }), extra: Vec::new() },
        },
        to_unicode: Some(PdfToUnicode { byte_width: 2, mappings }),
        extra: Vec::new(),
    })
}

/// 🧾 Ids of every resource `ops` reference that the snapshot does not define — what a consumer
/// asserts is empty before writing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unresolved_resources(snapshot: &PdfSnapshot, ops: &[PdfOp]) -> Vec<String> {
    let references = content_references(ops);
    let mut missing = Vec::new();
    missing.extend(references.fonts.iter().filter(|id| snapshot.font(id).is_none()).map(|id| format!("font {id}")));
    missing.extend(references.x_objects.iter().filter(|id| snapshot.image(id).is_none() && snapshot.form(id).is_none()).map(|id| format!("xobject {id}")));
    missing.extend(references.ext_g_states.iter().filter(|id| snapshot.ext_g_state(id).is_none()).map(|id| format!("extgstate {id}")));
    missing.extend(references.shadings.iter().filter(|id| snapshot.shading(id).is_none()).map(|id| format!("shading {id}")));
    missing.extend(references.patterns.iter().filter(|id| snapshot.pattern(id).is_none()).map(|id| format!("pattern {id}")));
    missing.extend(references.color_spaces.iter().filter(|id| !snapshot.color_spaces.iter().any(|space| &space.name == *id)).map(|id| format!("colorspace {id}")));
    missing
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_7::subsets::a::schema::PdfAComposer;
    use crate::standards::v1_7::subsets::base::schema::PdfComposer as PdfRawAnyComposer;
    use crate::standards::v1_7::subsets::e::schema::PdfEComposer;
    use crate::standards::v1_7::subsets::h::schema::PdfHComposer;
    use crate::standards::v1_7::subsets::ua::schema::PdfUaComposer;
    use crate::standards::v1_7::subsets::vt::schema::PdfVtComposer;
    use crate::standards::v1_7::subsets::x::schema::PdfXComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<PdfRawAnyComposer>(),
                    composer_entry_of::<PdfAComposer>(),
                    composer_entry_of::<PdfXComposer>(),
                    composer_entry_of::<PdfEComposer>(),
                    composer_entry_of::<PdfUaComposer>(),
                    composer_entry_of::<PdfVtComposer>(),
                    composer_entry_of::<PdfHComposer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
