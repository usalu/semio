//! 🧩️ The lifter state, the resource categories, the document entry points and the document-level lanes of the lift.

use super::*;

//#region 🔖️Lifter
/// 🏷️ Resource categories whose ids share one namespace each (`Page` keys pages by index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Category {
    Font,
    XObject,
    ExtGState,
    Shading,
    Pattern,
    ColorSpace,
    Properties,
    EmbeddedFile,
    OptionalContent,
    Page,
}

/// 🧭 One lifting pass over a document.
pub struct Lifter<'s> {
    pub(super) source: &'s mut dyn ObjectSource,
    pub snapshot: PdfSnapshot,
    pub admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
    pub ids: HashMap<(Category, ObjRef), String>,
    pub(super) used: HashMap<Category, HashSet<String>>,
    pub(super) inline_ids: HashMap<(Category, usize, String), String>,
    pub(super) in_progress: HashSet<(Category, ObjRef)>,
    pub page_refs: Vec<ObjRef>,
    pub annotation_refs: HashMap<ObjRef, (u32, u32)>,
    pub(super) scope: usize,
}

impl<'s> Lifter<'s> {
    pub fn new(source: &'s mut dyn ObjectSource) -> Self {
        Self { source, snapshot: PdfSnapshot::default(), admitted_stream_roles: Vec::new(), ids: HashMap::new(), used: HashMap::new(), inline_ids: HashMap::new(), in_progress: HashSet::new(), page_refs: Vec::new(), annotation_refs: HashMap::new(), scope: 0 }
    }

    pub(super) fn text(&mut self, value: Option<&PdfObject>) -> Option<String> {
        let value = self.source.deref(value?);
        match value {
            PdfObject::Text(text) => Some(text),
            PdfObject::Name(name) => Some(name),
            _ => None,
        }
    }

    pub(super) fn date(&mut self, value: Option<&PdfObject>) -> Option<PdfDate> {
        match value.map(|value|self.source.deref(value)) {Some(PdfObject::Date(date))=>Some(date),_=>None}
    }

    /// 🆔 Registers (or looks up) the id of a resource object bound under `name`.
    pub(super) fn claim_id(&mut self, category: Category, name: &str, reference: Option<ObjRef>) -> (String, bool) {
        if let Some(reference) = reference {
            if let Some(id) = self.ids.get(&(category, reference)) {
                return (id.clone(), false);
            }
        } else if let Some(id) = self.inline_ids.get(&(category, self.scope, name.to_string())) {
            return (id.clone(), false);
        }
        let used = self.used.entry(category).or_default();
        let base = if name.is_empty() { "R".to_string() } else { name.to_string() };
        let mut candidate = base.clone();
        let mut counter = 2;
        while used.contains(&candidate) {
            candidate = format!("{base}_{counter}");
            counter += 1;
        }
        used.insert(candidate.clone());
        match reference {
            Some(reference) => {
                self.ids.insert((category, reference), candidate.clone());
            }
            None => {
                self.inline_ids.insert((category, self.scope, name.to_string()), candidate.clone());
            }
        }
        (candidate, true)
    }

    pub(super) fn existing_id(&self, category: Category, value: &PdfObject) -> Option<String> {
        self.ids.get(&(category, value.as_ref()?)).cloned()
    }

    pub(super) fn page_index(&self, value: &PdfObject) -> Option<u32> {
        let reference = value.as_ref()?;
        self.page_refs.iter().position(|candidate| *candidate == reference).map(|index| index as u32)
    }
}
//#endregion 🔖️Lifter

//#region 🔖️Document
/// ⬇️ Lifts a whole document from its trailer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_document(trailer: &[PdfDictEntry], declared_version: &str, source: &mut dyn ObjectSource) -> PdfSnapshot {
    lift_document_with(trailer, declared_version, source).snapshot
}

/// ⬇️ Lifts a whole document and hands back the lifter (its reference → id maps included).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_document_with<'s>(trailer: &[PdfDictEntry], declared_version: &str, source: &'s mut dyn ObjectSource) -> Lifter<'s> {
    let mut lifter = Lifter::new(source);
    lifter.snapshot.declared_version = declared_version.to_string();
    let root = dict_get(trailer, "Root").map(|value| lifter.source.deref(value)).unwrap_or(PdfObject::Null);
    let catalog = root.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
    if let Some(version) = dict_name(&catalog, "Version") {
        if version > declared_version {
            lifter.snapshot.declared_version = version.to_string();
        }
    }
    if let Some(info) = dict_get(trailer, "Info") {
        let info = lifter.source.deref(info);
        if let Some(dict) = info.as_dict() {
            lifter.snapshot.info = lifter.lift_info(dict);
        }
    }
    if let Some(PdfObject::Array(items)) = dict_get(trailer, "ID") {
        if let [PdfObject::Str(a), PdfObject::Str(b)] = items.as_slice() {
            lifter.snapshot.document_id = Some([a.clone(), b.clone()]);
        }
    }
    lifter.lift_optional_content(&catalog);
    lifter.lift_embedded_files(&catalog);
    lifter.lift_pages(&catalog);
    let mut pages = Vec::new();
    for (index, page_ref) in lifter.page_refs.clone().iter().enumerate() {
        pages.push(lifter.lift_page(index as u32, *page_ref));
    }
    lifter.snapshot.pages = pages;
    lifter.lift_catalog(&catalog);
    lifter
}

impl Lifter<'_> {
    pub(super) fn lift_info(&mut self, dict: &[PdfDictEntry]) -> PdfInfo {
        PdfInfo {
            title: self.text(dict_get(dict, "Title")),
            author: self.text(dict_get(dict, "Author")),
            subject: self.text(dict_get(dict, "Subject")),
            keywords: self.text(dict_get(dict, "Keywords")),
            creator: self.text(dict_get(dict, "Creator")),
            producer: self.text(dict_get(dict, "Producer")),
            creation_date: self.date(dict_get(dict, "CreationDate")),
            modification_date: self.date(dict_get(dict, "ModDate")),
            trapped: dict_name(dict, "Trapped").map(str::to_string),
            extra: extra_entries(dict, &["Title", "Author", "Subject", "Keywords", "Creator", "Producer", "CreationDate", "ModDate", "Trapped"]),
        }
    }

    /// 🌳 Walks `/Pages → /Kids` to the leaves, cycle-guarded, recording page references in order.
    pub(super) fn lift_pages(&mut self, catalog: &[PdfDictEntry]) {
        let Some(pages_ref) = dict_get(catalog, "Pages").and_then(PdfObject::as_ref) else { return };
        let mut visited = HashSet::new();
        let mut stack = vec![pages_ref];
        let mut order = Vec::new();
        self.walk_pages(pages_ref, &mut visited, &mut order, 0);
        stack.clear();
        self.page_refs = order;
    }

    pub(super) fn walk_pages(&mut self, node_ref: ObjRef, visited: &mut HashSet<u32>, order: &mut Vec<ObjRef>, depth: usize) {
        if !visited.insert(node_ref.num) || depth > 64 {
            return;
        }
        let Some(node) = self.source.get(node_ref) else { return };
        let is_pages = node.dict_get("Type").and_then(PdfObject::as_name) == Some("Pages") || (node.dict_get("Kids").is_some() && node.dict_get("Contents").is_none());
        if is_pages {
            if let Some(resources) = node.dict_get("Resources").cloned() {
                let saved_scope = self.scope;
                self.scope = node_ref.num as usize;
                self.lift_resources(&resources);
                self.scope = saved_scope;
            }
            let kids: Vec<ObjRef> = node.dict_get("Kids").map(|kids| self.source.deref(kids)).and_then(|kids| kids.as_array().map(|items| items.iter().filter_map(PdfObject::as_ref).collect())).unwrap_or_default();
            for kid in kids {
                self.walk_pages(kid, visited, order, depth + 1);
            }
        } else if node.as_dict().is_some() {
            order.push(node_ref);
        }
    }

    /// 🧬 The inherited attribute `key` of a page: its own entry, else the nearest ancestor's.
    pub(super) fn inherited(&mut self, page: &[PdfDictEntry], key: &str) -> Option<PdfObject> {
        let mut current = page.to_vec();
        let mut depth = 0;
        loop {
            if let Some(value) = dict_get(&current, key) {
                return Some(self.source.deref(value));
            }
            let parent = dict_get(&current, "Parent")?.clone();
            let parent = self.source.deref(&parent);
            current = parent.as_dict()?.to_vec();
            depth += 1;
            if depth > 64 {
                return None;
            }
        }
    }

    pub(super) fn lift_page(&mut self, index: u32, page_ref: ObjRef) -> PdfPage {
        let dict = self.source.get(page_ref).and_then(|page| page.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        let media_box = self.inherited(&dict, "MediaBox").and_then(|v| rect_of(Some(&v))).unwrap_or([0.0, 0.0, 612.0, 792.0]);
        let mut page = PdfPage::new(media_box[2] - media_box[0], media_box[3] - media_box[1]);
        page.media_box = media_box;
        page.crop_box = self.inherited(&dict, "CropBox").and_then(|v| rect_of(Some(&v)));
        page.bleed_box = rect_of(dict_get(&dict, "BleedBox"));
        page.trim_box = rect_of(dict_get(&dict, "TrimBox"));
        page.art_box = rect_of(dict_get(&dict, "ArtBox"));
        page.rotate = self.inherited(&dict, "Rotate").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        page.user_unit = dict_f64(&dict, "UserUnit");
        let resources = self.inherited(&dict, "Resources").unwrap_or(PdfObject::Dict(Vec::new()));
        self.scope = page_ref.num as usize;
        let map = self.lift_resources(&resources);
        let identity = PdfGraphIdentity { owner: page_ref, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] };
        let value = dict_get(&dict, "Contents");
        page.content = if value.is_some() { self.source.semantic_role_at(&identity, PdfStreamRoleKind::Operators).map(|role| match role { PdfStreamRoleValue::Operators { content } => rename_content(content, &map), _ => Vec::new() }).unwrap_or_default() } else { Vec::new() };
        page.group = dict_get(&dict, "Group").map(|g| self.source.deref(g)).and_then(|g| g.as_dict().map(|g| self.lift_group(g)));
        page.thumbnail = dict_get(&dict, "Thumb").cloned().map(|thumb| self.lift_x_object("Thumb", &thumb)).filter(|id| !id.is_empty());
        page.struct_parents = dict_i64(&dict, "StructParents").map(|v| v as u32);
        page.transition = dict_get(&dict, "Trans").map(|t| self.source.deref(t)).and_then(|t| t.as_dict().map(<[PdfDictEntry]>::to_vec));
        page.duration = dict_f64(&dict, "Dur");
        page.metadata = dict_get(&dict, "Metadata").map(|m| self.source.deref(m)).and_then(|m| match m {
            stream @ PdfObject::Stream { .. } => Some(role_text(self.source, &stream)),
            _ => None,
        });
        page.additional_actions = dict_get(&dict, "AA").map(|aa| self.source.deref(aa)).and_then(|aa| aa.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        if let Some(annots) = dict_get(&dict, "Annots").cloned() {
            let annots = self.source.deref(&annots);
            if let PdfObject::Array(items) = annots {
                for (annotation_index, item) in items.iter().enumerate() {
                    if let Some(reference) = item.as_ref() {
                        self.annotation_refs.insert(reference, (index, annotation_index as u32));
                    }
                }
                for item in &items {
                    let resolved = self.source.deref(item);
                    if let Some(annotation) = resolved.as_dict() {
                        let lifted = self.lift_annotation(annotation);
                        page.annotations.push(lifted);
                    }
                }
            }
        }
        page.extra = extra_entries(&dict, &["Type", "Parent", "Kids", "Count", "MediaBox", "CropBox", "BleedBox", "TrimBox", "ArtBox", "Rotate", "UserUnit", "Resources", "Contents", "Group", "Thumb", "StructParents", "Trans", "Dur", "Metadata", "AA", "Annots", "LastModified", "B", "PieceInfo", "Tabs", "PresSteps", "VP", "ID", "PZ", "SeparationInfo", "TemplateInstantiated", "OutputIntents"]);
        page
    }

    pub(super) fn lift_group(&mut self, group: &[PdfDictEntry]) -> PdfTransparencyGroup {
        PdfTransparencyGroup { color_space: dict_get(group, "CS").map(|cs| lift_colour_space(cs, self.source)), isolated: dict_get(group, "I").and_then(PdfObject::as_bool).unwrap_or(false), knockout: dict_get(group, "K").and_then(PdfObject::as_bool).unwrap_or(false) }
    }

    /// 🖋️ Parses a content stream against `map` and rewrites its resource names to ids.
    pub(super) fn parse_and_rename(&mut self, owner: &PdfObject, map: &ResourceMap) -> Vec<PdfOp> {
        let ops = match self.source.semantic_role(owner, PdfStreamRoleKind::Operators) { Some(PdfStreamRoleValue::Operators { content }) => content, _ => Vec::new() };
        rename_content(ops, map)
    }
}

//#endregion 🔖️Document
