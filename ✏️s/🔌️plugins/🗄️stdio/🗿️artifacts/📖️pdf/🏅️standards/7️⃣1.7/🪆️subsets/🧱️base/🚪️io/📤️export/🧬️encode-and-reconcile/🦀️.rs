//! 🧬️ The write options, `encode_pdf` and its recording, reading and catalog-lane helpers, and the reconcile and regenerate passes.

use super::*;

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
pub fn retained_trailer(snapshot: &PdfSnapshot, trailer: &[PdfDictEntry]) -> PResult<DocumentTrailer> {
    let root = dict_get(trailer, "Root").and_then(PdfObject::as_ref).ok_or_else(|| PdfEngineError::Malformed("the retained trailer names no /Root".into()))?;
    Ok(DocumentTrailer { root, info: dict_get(trailer, "Info").and_then(PdfObject::as_ref), id: snapshot.document_id.clone(), extra: trailer_extra(trailer) })
}
/// 🔁 Rewrites references inside a retained object through `map`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rewrite_refs(value: &PdfObject, map: &HashMap<u32, ObjRef>) -> PdfObject {
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
pub fn referenced(value: &PdfObject, skip: &[&str], out: &mut Vec<u32>) {
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
pub struct GraphView<'a>(pub &'a BTreeMap<u32, PdfIndirectObject>);
impl ObjectSource for GraphView<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.0.get(&reference.num).map(|object| object.value.clone())
    }
}
/// 🔭 What a retained graph spells: the typed lanes lifted from it and how they map back onto its
/// objects.
pub struct Reading {
    pub lanes: PdfSnapshot,
    pub ids: HashMap<(Category, ObjRef), String>,
    pub refs: HashMap<(Category, String), ObjRef>,
    pub page_refs: Vec<ObjRef>,
    pub annotation_refs: HashMap<ObjRef, (u32, u32)>,
    seen: HashSet<u32>,
}
impl Reading {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn of(source: impl ObjectSource, trailer: &[PdfDictEntry], declared_version: &str) -> PResult<Self> {
        let mut recording = Recording { inner: source, seen: HashSet::new() };
        let (lanes, ids, page_refs, annotation_refs) = {
            let lifter = lift_document_with(trailer, declared_version, &mut recording);
            if let Some(error)=lifter.admission_error {return Err(error);}
            (lifter.snapshot, lifter.ids, lifter.page_refs, lifter.annotation_refs)
        };
        let refs = ids.iter().map(|((category, reference), id)| ((*category, id.clone()), *reference)).collect();
        Ok(Self { lanes, ids, refs, page_refs, annotation_refs, seen: recording.seen })
    }

    /// 🗺️ Where every resource this reading names lives, and every page in `pages` order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn targets(&self, pages: &[ObjRef]) -> HashMap<(Category, String), ObjRef> {
        let mut refs = self.refs.clone();
        refs.extend(pages.iter().enumerate().map(|(index, reference)| ((Category::Page, index.to_string()), *reference)));
        refs
    }
}
/// 🗂️ A catalog lane a graft re-states entry by entry (ISO 32000-1 §7.7.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatalogLane {
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
    pub fn key(self) -> &'static str {
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
pub struct Moved {
    pub pages: bool,
    pub info: bool,
    pub resources: bool,
    pub catalog: Vec<CatalogLane>,
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
    pub fn is_empty(&self) -> bool {
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
    let original = Reading::of(GraphSource::new(&snapshot.objects), &snapshot.trailer, &snapshot.declared_version)?;
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
            let reading = graft.read()?;
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
pub const INHERITED_PAGE_KEYS: [&str; 4] = ["Resources", "MediaBox", "CropBox", "Rotate"];
//#endregion 🔖️Encode
