//! ⬇️ Lift: the COS object graph → the typed document lanes (ISO 32000-1 §7.7 document
//! structure, §7.7.3 page tree with inheritance, §7.8.3 resources, §9 fonts, §8.9–8.10
//! XObjects, §8.4.5 graphics states, §8.7 shadings and patterns, §12 interactive features,
//! §14.3 metadata). Resource names are canonicalized into document-level ids: the first name a
//! resource object is bound under becomes its id, and every content stream that binds the same
//! object under another name is rewritten to the id — so `lift ∘ lower` is the identity on the
//! typed lanes and content operators can name resources directly.

use super::colour::{array_n, extra_entries, lift_colour_space, lift_ext_g_state, lift_function, lift_shading, matrix_of, numbers_of, rect_of};
use super::content::{content_references, parse_content, FontTable};
use super::fonts::{base_encoding_from_name, cmap, decode_text_string, FontCodec};
use super::images::lift_image;
use super::lexer::{dict_f64, dict_get, dict_i64, dict_name};
use super::xref::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{HashMap, HashSet};

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
    source: &'s mut dyn ObjectSource,
    pub snapshot: PdfSnapshot,
    pub ids: HashMap<(Category, ObjRef), String>,
    used: HashMap<Category, HashSet<String>>,
    inline_ids: HashMap<(Category, usize, String), String>,
    in_progress: HashSet<(Category, ObjRef)>,
    pub page_refs: Vec<ObjRef>,
    pub annotation_refs: HashMap<ObjRef, (u32, u32)>,
    font_codecs: HashMap<String, FontCodec>,
    scope: usize,
}

/// 📚 A resource dictionary's name → id maps per category (what a content stream's names
/// resolve to).
#[derive(Clone, Debug, Default)]
pub struct ResourceMap {
    pub fonts: HashMap<String, String>,
    pub x_objects: HashMap<String, String>,
    pub ext_g_states: HashMap<String, String>,
    pub shadings: HashMap<String, String>,
    pub patterns: HashMap<String, String>,
    pub color_spaces: HashMap<String, String>,
    pub properties: HashMap<String, String>,
}

struct MappedFonts<'a> {
    map: &'a HashMap<String, String>,
    codecs: &'a HashMap<String, FontCodec>,
}

impl FontTable for MappedFonts<'_> {
    fn font(&self, name: &str) -> Option<&FontCodec> {
        self.codecs.get(self.map.get(name)?)
    }
}

impl<'s> Lifter<'s> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(source: &'s mut dyn ObjectSource) -> Self {
        Self { source, snapshot: PdfSnapshot::default(), ids: HashMap::new(), used: HashMap::new(), inline_ids: HashMap::new(), in_progress: HashSet::new(), page_refs: Vec::new(), annotation_refs: HashMap::new(), font_codecs: HashMap::new(), scope: 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn text(&mut self, value: Option<&PdfObject>) -> Option<String> {
        let value = self.source.deref(value?);
        match value {
            PdfObject::Str(bytes) => Some(decode_text_string(&bytes)),
            PdfObject::Name(name) => Some(name),
            _ => None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn date(&mut self, value: Option<&PdfObject>) -> Option<PdfDate> {
        self.text(value).and_then(|text| PdfDate::parse(&text))
    }

    /// 🆔 Registers (or looks up) the id of a resource object bound under `name`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn claim_id(&mut self, category: Category, name: &str, reference: Option<ObjRef>) -> (String, bool) {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn existing_id(&self, category: Category, value: &PdfObject) -> Option<String> {
        self.ids.get(&(category, value.as_ref()?)).cloned()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page_index(&self, value: &PdfObject) -> Option<u32> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_info(&mut self, dict: &[PdfDictEntry]) -> PdfInfo {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_pages(&mut self, catalog: &[PdfDictEntry]) {
        let Some(pages_ref) = dict_get(catalog, "Pages").and_then(PdfObject::as_ref) else { return };
        let mut visited = HashSet::new();
        let mut stack = vec![pages_ref];
        let mut order = Vec::new();
        self.walk_pages(pages_ref, &mut visited, &mut order, 0);
        stack.clear();
        self.page_refs = order;
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn walk_pages(&mut self, node_ref: ObjRef, visited: &mut HashSet<u32>, order: &mut Vec<ObjRef>, depth: usize) {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inherited(&mut self, page: &[PdfDictEntry], key: &str) -> Option<PdfObject> {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_page(&mut self, index: u32, page_ref: ObjRef) -> PdfPage {
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
        let mut content = Vec::new();
        if let Some(contents) = dict_get(&dict, "Contents").cloned() {
            let contents = self.source.deref(&contents);
            let parts: Vec<PdfObject> = match contents {
                PdfObject::Array(items) => items.iter().map(|item| self.source.deref(item)).collect(),
                other => vec![other],
            };
            for part in parts {
                if let PdfObject::Stream { data, .. } = part {
                    if !content.is_empty() {
                        content.push(b'\n');
                    }
                    content.extend_from_slice(&data);
                }
            }
        }
        page.content = self.parse_and_rename(&content, &map);
        page.group = dict_get(&dict, "Group").map(|g| self.source.deref(g)).and_then(|g| g.as_dict().map(|g| self.lift_group(g)));
        page.thumbnail = dict_get(&dict, "Thumb").cloned().map(|thumb| self.lift_x_object("Thumb", &thumb)).filter(|id| !id.is_empty());
        page.struct_parents = dict_i64(&dict, "StructParents").map(|v| v as u32);
        page.transition = dict_get(&dict, "Trans").map(|t| self.source.deref(t)).and_then(|t| t.as_dict().map(<[PdfDictEntry]>::to_vec));
        page.duration = dict_f64(&dict, "Dur");
        page.metadata = dict_get(&dict, "Metadata").map(|m| self.source.deref(m)).and_then(|m| match m {
            PdfObject::Stream { data, .. } => Some(String::from_utf8_lossy(&data).into_owned()),
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_group(&mut self, group: &[PdfDictEntry]) -> PdfTransparencyGroup {
        PdfTransparencyGroup { color_space: dict_get(group, "CS").map(|cs| lift_colour_space(cs, self.source)), isolated: dict_get(group, "I").and_then(PdfObject::as_bool).unwrap_or(false), knockout: dict_get(group, "K").and_then(PdfObject::as_bool).unwrap_or(false) }
    }

    /// 🖋️ Parses a content stream against `map` and rewrites its resource names to ids.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_and_rename(&mut self, content: &[u8], map: &ResourceMap) -> Vec<PdfOp> {
        let ops = parse_content(content, &MappedFonts { map: &map.fonts, codecs: &self.font_codecs });
        rename_content(ops, map)
    }
}

/// 🔁 Rewrites every resource name in `ops` through `map` (names the map does not know stay).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rename_content(ops: Vec<PdfOp>, map: &ResourceMap) -> Vec<PdfOp> {
    let rename = |table: &HashMap<String, String>, name: &mut String| {
        if let Some(id) = table.get(name.as_str()) {
            *name = id.clone();
        }
    };
    ops.into_iter()
        .map(|mut op| {
            match &mut op {
                PdfOp::SetFont { name, .. } => rename(&map.fonts, name),
                PdfOp::PaintXObject { name } => rename(&map.x_objects, name),
                PdfOp::SetExtGState { name } => rename(&map.ext_g_states, name),
                PdfOp::PaintShading { name } => rename(&map.shadings, name),
                PdfOp::SetStrokeColorN { pattern: Some(name), .. } | PdfOp::SetFillColorN { pattern: Some(name), .. } => rename(&map.patterns, name),
                PdfOp::SetStrokeColorSpace { name } | PdfOp::SetFillColorSpace { name } => rename(&map.color_spaces, name),
                PdfOp::MarkedContentPointWithProperties { properties: PdfPropertyList::Named { name }, .. } | PdfOp::BeginMarkedContentWithProperties { properties: PdfPropertyList::Named { name }, .. } => rename(&map.properties, name),
                _ => {}
            }
            op
        })
        .collect()
}
//#endregion 🔖️Document

//#region 🔖️Resources
impl Lifter<'_> {
    /// 📚 Lifts every entry of a resource dictionary into the document collections and returns
    /// the name → id map its content streams are rewritten with.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn lift_resources(&mut self, resources: &PdfObject) -> ResourceMap {
        let resources = self.source.deref(resources);
        let mut map = ResourceMap::default();
        let sub = |lifter: &mut Self, key: &str| -> Vec<PdfDictEntry> { resources.dict_get(key).map(|v| lifter.source.deref(v)).and_then(|v| v.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default() };
        for entry in sub(self, "Font") {
            let id = self.lift_font(&entry.key, &entry.value);
            map.fonts.insert(entry.key, id);
        }
        for entry in sub(self, "XObject") {
            let id = self.lift_x_object(&entry.key, &entry.value);
            if !id.is_empty() {
                map.x_objects.insert(entry.key, id);
            }
        }
        for entry in sub(self, "ExtGState") {
            let id = self.lift_ext_g_state_entry(&entry.key, &entry.value);
            map.ext_g_states.insert(entry.key, id);
        }
        for entry in sub(self, "ColorSpace") {
            let (id, fresh) = self.claim_id(Category::ColorSpace, &entry.key, entry.value.as_ref());
            if fresh {
                let color_space = lift_colour_space(&entry.value, self.source);
                self.snapshot.color_spaces.push(PdfNamedColorSpace { name: id.clone(), color_space });
            }
            map.color_spaces.insert(entry.key, id);
        }
        for entry in sub(self, "Shading") {
            let id = self.lift_shading_entry(&entry.key, &entry.value);
            if !id.is_empty() {
                map.shadings.insert(entry.key, id);
            }
        }
        for entry in sub(self, "Pattern") {
            let id = self.lift_pattern(&entry.key, &entry.value);
            if !id.is_empty() {
                map.patterns.insert(entry.key, id);
            }
        }
        for entry in sub(self, "Properties") {
            let (id, fresh) = self.claim_id(Category::Properties, &entry.key, entry.value.as_ref());
            if fresh {
                let resolved = self.source.deref(&entry.value);
                let mut entries = resolved.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
                if let Some(group) = self.existing_id(Category::OptionalContent, &entry.value) {
                    entries = vec![PdfDictEntry::new("OCG", PdfObject::name(group))];
                }
                self.snapshot.properties.push(PdfNamedProperties { name: id.clone(), entries });
            }
            map.properties.insert(entry.key, id);
        }
        map
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_ext_g_state_entry(&mut self, name: &str, value: &PdfObject) -> String {
        let (id, fresh) = self.claim_id(Category::ExtGState, name, value.as_ref());
        if fresh {
            let resolved = self.source.deref(value);
            let dict = resolved.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
            let mut form_id_of = |group: &PdfObject, _source: &mut dyn ObjectSource| -> Option<String> { self_form_id(group) };
            let mut font_id_of = |font: &PdfObject, _source: &mut dyn ObjectSource| -> Option<String> { self_font_id(font) };
            // 🧷 Soft-mask groups and fonts are lifted first so the closures above only look up.
            if let Some(mask) = dict_get(&dict, "SMask").map(|m| self.source.deref(m)) {
                if let Some(group) = mask.dict_get("G").cloned() {
                    self.lift_x_object("SMaskGroup", &group);
                }
            }
            if let Some(PdfObject::Array(font)) = dict_get(&dict, "Font") {
                if let Some(font_ref) = font.first().cloned() {
                    self.lift_font("GSFont", &font_ref);
                }
            }
            let ids = self.ids.clone();
            let mut form_lookup = |group: &PdfObject, source: &mut dyn ObjectSource| -> Option<String> { form_id_of(group, source).or_else(|| ids.get(&(Category::XObject, group.as_ref()?)).cloned()) };
            let mut font_lookup = |font: &PdfObject, source: &mut dyn ObjectSource| -> Option<String> { font_id_of(font, source).or_else(|| ids.get(&(Category::Font, font.as_ref()?)).cloned()) };
            let state = lift_ext_g_state(&id, &dict, self.source, &mut form_lookup, &mut font_lookup);
            self.snapshot.ext_g_states.push(state);
        }
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_shading_entry(&mut self, name: &str, value: &PdfObject) -> String {
        let (id, fresh) = self.claim_id(Category::Shading, name, value.as_ref());
        if fresh {
            match lift_shading(&id, value, self.source) {
                Some(shading) => self.snapshot.shadings.push(shading),
                None => return String::new(),
            }
        }
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_pattern(&mut self, name: &str, value: &PdfObject) -> String {
        let (id, fresh) = self.claim_id(Category::Pattern, name, value.as_ref());
        if !fresh {
            return id;
        }
        let resolved = self.source.deref(value);
        let Some(dict) = resolved.as_dict().map(<[PdfDictEntry]>::to_vec) else { return String::new() };
        let matrix = matrix_of(dict_get(&dict, "Matrix")).unwrap_or(PDF_IDENTITY_MATRIX);
        let kind = match dict_i64(&dict, "PatternType") {
            Some(2) => {
                let shading = dict_get(&dict, "Shading").cloned().map(|s| self.lift_shading_entry(&format!("{id}Shading"), &s)).unwrap_or_default();
                let ext_g_state = dict_get(&dict, "ExtGState").cloned().map(|g| self.lift_ext_g_state_entry(&format!("{id}State"), &g));
                PdfPatternKind::Shading { shading, ext_g_state }
            }
            _ => {
                let PdfObject::Stream { data, .. } = &resolved else { return String::new() };
                let resources = dict_get(&dict, "Resources").cloned().unwrap_or(PdfObject::Dict(Vec::new()));
                let saved_scope = self.scope;
                self.scope = value.as_ref().map_or(self.scope.wrapping_mul(31).wrapping_add(7), |r| r.num as usize);
                let map = self.lift_resources(&resources);
                let content = self.parse_and_rename(data, &map);
                self.scope = saved_scope;
                PdfPatternKind::Tiling { paint_type: dict_i64(&dict, "PaintType").unwrap_or(1) as u32, tiling_type: dict_i64(&dict, "TilingType").unwrap_or(1) as u32, bbox: rect_of(dict_get(&dict, "BBox")).unwrap_or([0.0, 0.0, 1.0, 1.0]), x_step: dict_f64(&dict, "XStep").unwrap_or(1.0), y_step: dict_f64(&dict, "YStep").unwrap_or(1.0), content }
            }
        };
        self.snapshot.patterns.push(PdfPattern { id: id.clone(), matrix, kind, extra: extra_entries(&dict, &["Type", "PatternType", "Matrix", "Shading", "ExtGState", "PaintType", "TilingType", "BBox", "XStep", "YStep", "Resources", "Length"]) });
        id
    }

    /// 🖼️ Lifts an XObject (image or form) and returns its id (`""` when it is neither).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn lift_x_object(&mut self, name: &str, value: &PdfObject) -> String {
        if let Some(reference) = value.as_ref() {
            if self.in_progress.contains(&(Category::XObject, reference)) {
                return self.ids.get(&(Category::XObject, reference)).cloned().unwrap_or_default();
            }
        }
        let (id, fresh) = self.claim_id(Category::XObject, name, value.as_ref());
        if !fresh {
            return id;
        }
        self.lift_x_object_claimed(&id, value)
    }

    /// 🖼️ Lifts an XObject under an already-claimed id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_x_object_claimed(&mut self, id: &str, value: &PdfObject) -> String {
        let id = id.to_string();
        let resolved = self.source.deref(value);
        let PdfObject::Stream { dict, data, filters } = &resolved else {
            return String::new();
        };
        if let Some(reference) = value.as_ref() {
            self.in_progress.insert((Category::XObject, reference));
        }
        match dict_name(dict, "Subtype") {
            Some("Image") => {
                let mut pending_masks: Vec<(String, PdfObject)> = Vec::new();
                let mut mask_ids: HashMap<u32, String> = HashMap::new();
                for (key, mask_name) in [("SMask", format!("{id}SMask")), ("Mask", format!("{id}Mask"))] {
                    let Some(mask) = dict_get(dict, key).cloned() else { continue };
                    let Some(mask_ref) = mask.as_ref() else { continue };
                    if !matches!(self.source.deref(&mask), PdfObject::Stream { .. }) {
                        continue;
                    }
                    let (mask_id, fresh) = self.claim_id(Category::XObject, &mask_name, Some(mask_ref));
                    mask_ids.insert(mask_ref.num, mask_id.clone());
                    if fresh {
                        pending_masks.push((mask_id, mask));
                    }
                }
                let oc_ids = self.ids.clone();
                let mut id_of = |reference: &PdfObject| -> Option<String> { mask_ids.get(&reference.as_ref()?.num).cloned() };
                let mut oc_id_of = |reference: &PdfObject| -> Option<String> { oc_ids.get(&(Category::OptionalContent, reference.as_ref()?)).cloned() };
                let image = lift_image(&id, dict, data, filters, self.source, &mut id_of, &mut oc_id_of);
                self.snapshot.images.push(image);
                for (mask_id, mask) in pending_masks {
                    self.lift_x_object_claimed(&mask_id, &mask);
                }
            }
            Some("Form") | Some("PS") | None => {
                let resources = dict_get(dict, "Resources").cloned().unwrap_or(PdfObject::Dict(Vec::new()));
                let saved_scope = self.scope;
                self.scope = value.as_ref().map_or(self.scope.wrapping_mul(31).wrapping_add(11), |r| r.num as usize);
                let map = self.lift_resources(&resources);
                let content = self.parse_and_rename(data, &map);
                self.scope = saved_scope;
                let group = dict_get(dict, "Group").map(|g| self.source.deref(g)).and_then(|g| g.as_dict().map(|g| self.lift_group(g)));
                let optional_content = dict_get(dict, "OC").and_then(|oc| self.existing_id(Category::OptionalContent, oc));
                self.snapshot.forms.push(PdfFormXObject {
                    id: id.clone(),
                    bbox: rect_of(dict_get(dict, "BBox")).unwrap_or([0.0, 0.0, 0.0, 0.0]),
                    matrix: matrix_of(dict_get(dict, "Matrix")).unwrap_or(PDF_IDENTITY_MATRIX),
                    content,
                    group,
                    optional_content,
                    struct_parent: dict_i64(dict, "StructParent").map(|v| v as u32),
                    extra: extra_entries(dict, &["Type", "Subtype", "FormType", "BBox", "Matrix", "Resources", "Group", "OC", "StructParent", "Length", "Filter", "DecodeParms"]),
                });
            }
            _ => {
                if let Some(reference) = value.as_ref() {
                    self.in_progress.remove(&(Category::XObject, reference));
                }
                return String::new();
            }
        }
        if let Some(reference) = value.as_ref() {
            self.in_progress.remove(&(Category::XObject, reference));
        }
        id
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn self_form_id(_group: &PdfObject) -> Option<String> {
    None
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn self_font_id(_font: &PdfObject) -> Option<String> {
    None
}
//#endregion 🔖️Resources

//#region 🔖️Fonts
impl Lifter<'_> {
    /// 🔤 Lifts a font dictionary and returns its id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn lift_font(&mut self, name: &str, value: &PdfObject) -> String {
        let (id, fresh) = self.claim_id(Category::Font, name, value.as_ref());
        if !fresh {
            return id;
        }
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
        let subtype = dict_name(&dict, "Subtype").unwrap_or("Type1").to_string();
        let base_font = dict_name(&dict, "BaseFont").unwrap_or("").to_string();
        let to_unicode = dict_get(&dict, "ToUnicode").map(|t| self.source.deref(t)).and_then(|t| match t {
            PdfObject::Stream { data, .. } => Some(cmap::parse_to_unicode(&data)),
            _ => None,
        });
        let descriptor = dict_get(&dict, "FontDescriptor").map(|d| self.source.deref(d)).and_then(|d| d.as_dict().map(|d| self.lift_descriptor(d)));
        let (descriptor, program) = match descriptor {
            Some((descriptor, program)) => (Some(descriptor), program),
            None => (None, None),
        };
        let encoding = self.lift_simple_encoding(dict_get(&dict, "Encoding"));
        let first_char = dict_i64(&dict, "FirstChar").unwrap_or(0).max(0) as u32;
        let widths = dict_get(&dict, "Widths").map(|w| self.source.deref(w)).map(|w| numbers_of(Some(&w))).unwrap_or_default();
        let kind = match subtype.as_str() {
            "Type0" => {
                let cmap = match dict_get(&dict, "Encoding").map(|e| self.source.deref(e)) {
                    Some(PdfObject::Name(name)) => PdfCMap::Predefined { name },
                    Some(PdfObject::Stream { dict: cmap_dict, data, .. }) => {
                        let mut parsed = cmap::parse_cmap(&data, dict_name(&cmap_dict, "CMapName").unwrap_or("Embedded"));
                        if let Some(PdfObject::Name(parent)) = dict_get(&cmap_dict, "UseCMap") {
                            parsed.use_cmap = Some(parent.clone());
                        }
                        PdfCMap::Embedded { cmap: parsed }
                    }
                    _ => PdfCMap::identity_h(),
                };
                let descendant = dict_get(&dict, "DescendantFonts").map(|d| self.source.deref(d)).and_then(|d| d.as_array().and_then(|items| items.first().cloned())).map(|d| self.source.deref(&d)).and_then(|d| d.as_dict().map(|d| self.lift_cid_font(d)));
                PdfFontKind::Type0 { base_font, cmap, descendant: descendant.unwrap_or_else(|| PdfCidFont { true_type: true, base_font: String::new(), system_info: PdfCidSystemInfo::default(), descriptor: PdfFontDescriptor::default(), default_width: 1000.0, widths: Vec::new(), default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: None, program: None, extra: Vec::new() }) }
            }
            "TrueType" => PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program },
            "Type3" => {
                let resources = dict_get(&dict, "Resources").cloned().unwrap_or(PdfObject::Dict(Vec::new()));
                let saved_scope = self.scope;
                self.scope = value.as_ref().map_or(self.scope.wrapping_mul(31).wrapping_add(13), |r| r.num as usize);
                let map = self.lift_resources(&resources);
                let procs = dict_get(&dict, "CharProcs").map(|p| self.source.deref(p)).and_then(|p| p.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
                let mut char_procs = Vec::new();
                for entry in procs {
                    if let PdfObject::Stream { data, .. } = self.source.deref(&entry.value) {
                        let content = self.parse_and_rename(&data, &map);
                        char_procs.push(PdfCharProc { name: entry.key, content });
                    }
                }
                self.scope = saved_scope;
                PdfFontKind::Type3 { font_matrix: matrix_of(dict_get(&dict, "FontMatrix")).unwrap_or([0.001, 0.0, 0.0, 0.001, 0.0, 0.0]), font_bbox: rect_of(dict_get(&dict, "FontBBox")).unwrap_or([0.0; 4]), encoding, first_char, widths, char_procs, descriptor }
            }
            _ => PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program },
        };
        let font = PdfFont { id: id.clone(), kind, to_unicode, extra: extra_entries(&dict, &["Type", "Subtype", "BaseFont", "Name", "FirstChar", "LastChar", "Widths", "FontDescriptor", "Encoding", "ToUnicode", "DescendantFonts", "FontMatrix", "FontBBox", "CharProcs", "Resources"]) };
        self.font_codecs.insert(id.clone(), FontCodec::new(&font));
        self.snapshot.fonts.push(font);
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_simple_encoding(&mut self, value: Option<&PdfObject>) -> PdfSimpleEncoding {
        let Some(value) = value else { return PdfSimpleEncoding::default() };
        match self.source.deref(value) {
            PdfObject::Name(name) => PdfSimpleEncoding { base: base_encoding_from_name(&name), differences: Vec::new() },
            PdfObject::Dict(dict) => {
                let base = dict_name(&dict, "BaseEncoding").and_then(base_encoding_from_name);
                let mut differences = Vec::new();
                if let Some(items) = dict_get(&dict, "Differences").map(|d| self.source.deref(d)).and_then(|d| d.as_array().map(<[PdfObject]>::to_vec)) {
                    let mut code = 0u32;
                    for item in items {
                        match item {
                            PdfObject::Int(value) => code = value.max(0) as u32,
                            PdfObject::Real(value) => code = value.to_f64().unwrap_or(0.0).max(0.0) as u32,
                            PdfObject::Name(glyph) => {
                                differences.push(PdfEncodingDifference { code, glyph });
                                code += 1;
                            }
                            _ => {}
                        }
                    }
                }
                PdfSimpleEncoding { base, differences }
            }
            _ => PdfSimpleEncoding::default(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_descriptor(&mut self, dict: &[PdfDictEntry]) -> (PdfFontDescriptor, Option<PdfFontProgram>) {
        let program = ["FontFile", "FontFile2", "FontFile3"].iter().find_map(|key| {
            let stream = self.source.deref(dict_get(dict, key)?);
            let PdfObject::Stream { dict: stream_dict, data, .. } = stream else { return None };
            Some(match *key {
                "FontFile" => PdfFontProgram::Type1 { data, length1: dict_i64(&stream_dict, "Length1").unwrap_or(0) as u32, length2: dict_i64(&stream_dict, "Length2").unwrap_or(0) as u32, length3: dict_i64(&stream_dict, "Length3").unwrap_or(0) as u32 },
                "FontFile2" => PdfFontProgram::TrueType { data },
                _ => match dict_name(&stream_dict, "Subtype") {
                    Some("CIDFontType0C") => PdfFontProgram::CidCff { data },
                    Some("OpenType") => PdfFontProgram::OpenType { data },
                    _ => PdfFontProgram::Cff { data },
                },
            })
        });
        let descriptor = PdfFontDescriptor {
            font_name: dict_name(dict, "FontName").unwrap_or("").to_string(),
            flags: dict_i64(dict, "Flags").unwrap_or(0).max(0) as u32,
            font_bbox: rect_of(dict_get(dict, "FontBBox")).unwrap_or([0.0; 4]),
            italic_angle: dict_f64(dict, "ItalicAngle").unwrap_or(0.0),
            ascent: dict_f64(dict, "Ascent").unwrap_or(0.0),
            descent: dict_f64(dict, "Descent").unwrap_or(0.0),
            cap_height: dict_f64(dict, "CapHeight").unwrap_or(0.0),
            stem_v: dict_f64(dict, "StemV").unwrap_or(0.0),
            stem_h: dict_f64(dict, "StemH"),
            x_height: dict_f64(dict, "XHeight"),
            leading: dict_f64(dict, "Leading"),
            avg_width: dict_f64(dict, "AvgWidth"),
            max_width: dict_f64(dict, "MaxWidth"),
            missing_width: dict_f64(dict, "MissingWidth"),
            font_family: self.text(dict_get(dict, "FontFamily")),
            font_stretch: dict_name(dict, "FontStretch").map(str::to_string),
            font_weight: dict_f64(dict, "FontWeight"),
            char_set: self.text(dict_get(dict, "CharSet")),
            extra: extra_entries(dict, &["Type", "FontName", "Flags", "FontBBox", "ItalicAngle", "Ascent", "Descent", "CapHeight", "StemV", "StemH", "XHeight", "Leading", "AvgWidth", "MaxWidth", "MissingWidth", "FontFamily", "FontStretch", "FontWeight", "CharSet", "FontFile", "FontFile2", "FontFile3"]),
        };
        (descriptor, program)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_cid_font(&mut self, dict: &[PdfDictEntry]) -> PdfCidFont {
        let (descriptor, program) = dict_get(dict, "FontDescriptor").map(|d| self.source.deref(d)).and_then(|d| d.as_dict().map(|d| self.lift_descriptor(d))).unwrap_or_default();
        let system_info = dict_get(dict, "CIDSystemInfo").map(|s| self.source.deref(s)).and_then(|s| s.as_dict().map(<[PdfDictEntry]>::to_vec)).map(|s| PdfCidSystemInfo { registry: dict_get(&s, "Registry").and_then(PdfObject::as_str_bytes).map(|b| String::from_utf8_lossy(b).into_owned()).unwrap_or_else(|| "Adobe".into()), ordering: dict_get(&s, "Ordering").and_then(PdfObject::as_str_bytes).map(|b| String::from_utf8_lossy(b).into_owned()).unwrap_or_else(|| "Identity".into()), supplement: dict_i64(&s, "Supplement").unwrap_or(0).max(0) as u32 }).unwrap_or_default();
        let widths = dict_get(dict, "W").map(|w| self.source.deref(w)).and_then(|w| w.as_array().map(<[PdfObject]>::to_vec)).map(|items| self.lift_cid_widths(&items)).unwrap_or_default();
        let vertical_metrics = dict_get(dict, "W2").map(|w| self.source.deref(w)).and_then(|w| w.as_array().map(<[PdfObject]>::to_vec)).map(|items| self.lift_cid_vertical(&items)).unwrap_or_default();
        let cid_to_gid = match dict_get(dict, "CIDToGIDMap").map(|m| self.source.deref(m)) {
            Some(PdfObject::Name(name)) if name == "Identity" => Some(PdfCidToGid::Identity),
            Some(PdfObject::Stream { data, .. }) => Some(PdfCidToGid::Map { data }),
            _ => None,
        };
        PdfCidFont {
            true_type: dict_name(dict, "Subtype") == Some("CIDFontType2"),
            base_font: dict_name(dict, "BaseFont").unwrap_or("").to_string(),
            system_info,
            descriptor,
            default_width: dict_f64(dict, "DW").unwrap_or(1000.0),
            widths,
            default_vertical: array_n::<2>(dict_get(dict, "DW2")),
            vertical_metrics,
            cid_to_gid,
            program,
            extra: extra_entries(dict, &["Type", "Subtype", "BaseFont", "CIDSystemInfo", "FontDescriptor", "DW", "W", "DW2", "W2", "CIDToGIDMap"]),
        }
    }

    /// 📏 `/W` array: `c [w1 w2 …]` runs and `cfirst clast w` ranges (§9.7.4.3).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_cid_widths(&mut self, items: &[PdfObject]) -> Vec<PdfCidWidthRun> {
        let mut runs = Vec::new();
        let mut index = 0;
        while index < items.len() {
            let Some(start) = items[index].as_f64() else {
                index += 1;
                continue;
            };
            match items.get(index + 1).map(|v| self.source.deref(v)) {
                Some(PdfObject::Array(widths)) => {
                    runs.push(PdfCidWidthRun { start_cid: start as u32, widths: widths.iter().filter_map(PdfObject::as_f64).collect() });
                    index += 2;
                }
                Some(last) => {
                    let last = last.as_f64().unwrap_or(start);
                    let width = items.get(index + 2).and_then(PdfObject::as_f64).unwrap_or(0.0);
                    let count = ((last - start).max(0.0) as usize + 1).min(65536);
                    runs.push(PdfCidWidthRun { start_cid: start as u32, widths: vec![width; count] });
                    index += 3;
                }
                None => break,
            }
        }
        runs
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_cid_vertical(&mut self, items: &[PdfObject]) -> Vec<PdfCidVerticalRun> {
        let mut runs = Vec::new();
        let mut index = 0;
        while index < items.len() {
            let Some(start) = items[index].as_f64() else {
                index += 1;
                continue;
            };
            match items.get(index + 1).map(|v| self.source.deref(v)) {
                Some(PdfObject::Array(values)) => {
                    let numbers: Vec<f64> = values.iter().filter_map(PdfObject::as_f64).collect();
                    runs.push(PdfCidVerticalRun { start_cid: start as u32, metrics: numbers.chunks(3).filter(|c| c.len() == 3).map(|c| [c[0], c[1], c[2]]).collect() });
                    index += 2;
                }
                Some(last) => {
                    let last = last.as_f64().unwrap_or(start);
                    let metric = [items.get(index + 2).and_then(PdfObject::as_f64).unwrap_or(0.0), items.get(index + 3).and_then(PdfObject::as_f64).unwrap_or(0.0), items.get(index + 4).and_then(PdfObject::as_f64).unwrap_or(0.0)];
                    let count = ((last - start).max(0.0) as usize + 1).min(65536);
                    runs.push(PdfCidVerticalRun { start_cid: start as u32, metrics: vec![metric; count] });
                    index += 5;
                }
                None => break,
            }
        }
        runs
    }
}
//#endregion 🔖️Fonts

//#region 🔖️Navigation
impl Lifter<'_> {
    /// 🎯 Lifts a destination: an explicit array, a name/string, or a dictionary with `/D`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_destination(&mut self, value: &PdfObject) -> Option<PdfDestination> {
        let resolved = self.source.deref(value);
        match resolved {
            PdfObject::Name(name) => Some(PdfDestination::Named { name }),
            PdfObject::Str(bytes) => Some(PdfDestination::Named { name: decode_text_string(&bytes) }),
            PdfObject::Dict(dict) => {
                let inner = dict_get(&dict, "D")?.clone();
                self.lift_destination(&inner)
            }
            PdfObject::Array(items) => {
                let page_value = items.first()?;
                let fit_name = items.get(1).and_then(PdfObject::as_name).unwrap_or("Fit");
                let number = |index: usize| items.get(index).and_then(PdfObject::as_f64);
                let fit = match fit_name {
                    "XYZ" => PdfDestinationFit::Xyz { left: number(2), top: number(3), zoom: number(4).filter(|z| *z != 0.0) },
                    "FitH" => PdfDestinationFit::FitHorizontal { top: number(2) },
                    "FitV" => PdfDestinationFit::FitVertical { left: number(2) },
                    "FitR" => PdfDestinationFit::FitRectangle { rect: [number(2).unwrap_or(0.0), number(3).unwrap_or(0.0), number(4).unwrap_or(0.0), number(5).unwrap_or(0.0)] },
                    "FitB" => PdfDestinationFit::FitBoundingBox,
                    "FitBH" => PdfDestinationFit::FitBoundingBoxHorizontal { top: number(2) },
                    "FitBV" => PdfDestinationFit::FitBoundingBoxVertical { left: number(2) },
                    _ => PdfDestinationFit::Fit,
                };
                match page_value {
                    PdfObject::Int(number) => Some(PdfDestination::RemotePage { page: (*number).max(0) as u32, fit }),
                    reference => Some(PdfDestination::Page { page: self.page_index(reference)?, fit }),
                }
            }
            _ => None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_file_specification(&mut self, value: &PdfObject) -> Option<PdfFileSpecification> {
        let resolved = self.source.deref(value);
        match resolved {
            PdfObject::Str(bytes) => Some(PdfFileSpecification::Path { path: decode_text_string(&bytes) }),
            PdfObject::Dict(dict) => {
                if let Some(id) = self.existing_id(Category::EmbeddedFile, value).or_else(|| dict_get(&dict, "EF").and_then(|ef| ef.dict_get("F").or_else(|| ef.dict_get("UF"))).and_then(|f| self.existing_id(Category::EmbeddedFile, f))) {
                    return Some(PdfFileSpecification::Embedded { file: id });
                }
                if dict_get(&dict, "EF").is_some() {
                    let name = self.text(dict_get(&dict, "UF")).or_else(|| self.text(dict_get(&dict, "F"))).unwrap_or_else(|| "Attachment".into());
                    let id = self.lift_embedded_file(&name, value, false);
                    if let Some(id) = id {
                        return Some(PdfFileSpecification::Embedded { file: id });
                    }
                }
                let path = self.text(dict_get(&dict, "UF")).or_else(|| self.text(dict_get(&dict, "F"))).or_else(|| self.text(dict_get(&dict, "DOS"))).or_else(|| self.text(dict_get(&dict, "Unix")))?;
                Some(PdfFileSpecification::Path { path })
            }
            _ => None,
        }
    }

    /// 📎 Lifts a file specification with an embedded file stream, returning its id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_embedded_file(&mut self, name: &str, value: &PdfObject, listed: bool) -> Option<String> {
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict()?.to_vec();
        let ef = dict_get(&dict, "EF").map(|ef| self.source.deref(ef))?;
        let stream_ref = ef.dict_get("F").or_else(|| ef.dict_get("UF")).cloned()?;
        let (id, fresh) = self.claim_id(Category::EmbeddedFile, name, value.as_ref().or(stream_ref.as_ref()));
        if !fresh {
            if listed {
                if let Some(file) = self.snapshot.embedded_files.iter_mut().find(|file| file.id == id) {
                    file.listed = true;
                }
            }
            return Some(id);
        }
        let PdfObject::Stream { dict: stream_dict, data, .. } = self.source.deref(&stream_ref) else { return None };
        let params = dict_get(&stream_dict, "Params").map(|p| self.source.deref(p)).and_then(|p| p.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        let file_name = self.text(dict_get(&dict, "UF")).or_else(|| self.text(dict_get(&dict, "F"))).unwrap_or_else(|| name.to_string());
        let file = PdfEmbeddedFile {
            id: id.clone(),
            file_name,
            description: self.text(dict_get(&dict, "Desc")),
            mime_type: dict_name(&stream_dict, "Subtype").map(str::to_string),
            data,
            creation_date: self.date(dict_get(&params, "CreationDate")),
            modification_date: self.date(dict_get(&params, "ModDate")),
            relationship: dict_name(&dict, "AFRelationship").map(str::to_string),
            listed,
        };
        self.snapshot.embedded_files.push(file);
        Some(id)
    }

    /// 🎬 Lifts an action dictionary (with its `/Next` chain).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_action(&mut self, value: &PdfObject) -> Option<PdfAction> {
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict()?.to_vec();
        let subtype = dict_name(&dict, "S").unwrap_or("").to_string();
        let new_window = dict_get(&dict, "NewWindow").and_then(PdfObject::as_bool);
        let names = |lifter: &mut Self, key: &str| -> Vec<String> {
            dict_get(&dict, key).map(|v| lifter.source.deref(v)).map(|v| match v {
                PdfObject::Array(items) => items.iter().filter_map(|item| match item {
                    PdfObject::Str(bytes) => Some(decode_text_string(bytes)),
                    PdfObject::Name(name) => Some(name.clone()),
                    _ => None,
                }).collect(),
                PdfObject::Str(bytes) => vec![decode_text_string(&bytes)],
                _ => Vec::new(),
            }).unwrap_or_default()
        };
        let kind = match subtype.as_str() {
            "GoTo" => PdfActionKind::GoTo { destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d))? },
            "GoToR" => PdfActionKind::GoToRemote { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f))?, destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d)).unwrap_or(PdfDestination::RemotePage { page: 0, fit: PdfDestinationFit::Fit }), new_window },
            "GoToE" => PdfActionKind::GoToEmbedded { destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d)).unwrap_or(PdfDestination::RemotePage { page: 0, fit: PdfDestinationFit::Fit }), new_window },
            "Launch" => PdfActionKind::Launch { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)).unwrap_or(PdfFileSpecification::Path { path: String::new() }), new_window },
            "Thread" => PdfActionKind::Thread { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)), thread: dict_i64(&dict, "D").unwrap_or(0).max(0) as u32 },
            "URI" => PdfActionKind::Uri { uri: dict_get(&dict, "URI").and_then(PdfObject::as_str_bytes).map(|b| String::from_utf8_lossy(b).into_owned()).unwrap_or_default(), is_map: dict_get(&dict, "IsMap").and_then(PdfObject::as_bool).unwrap_or(false) },
            "Sound" => PdfActionKind::Sound { sound: String::new(), volume: dict_f64(&dict, "Volume"), synchronous: dict_get(&dict, "Synchronous").and_then(PdfObject::as_bool).unwrap_or(false), repeat: dict_get(&dict, "Repeat").and_then(PdfObject::as_bool).unwrap_or(false), mix: dict_get(&dict, "Mix").and_then(PdfObject::as_bool).unwrap_or(false) },
            "Movie" => PdfActionKind::Movie { annotation: self.text(dict_get(&dict, "T")), operation: dict_name(&dict, "Operation").map(str::to_string) },
            "Hide" => PdfActionKind::Hide { annotations: names(self, "T"), hide: dict_get(&dict, "H").and_then(PdfObject::as_bool).unwrap_or(true) },
            "Named" => PdfActionKind::Named { name: dict_name(&dict, "N").unwrap_or("").to_string() },
            "SubmitForm" => PdfActionKind::SubmitForm { url: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)).map(|f| match f {
                PdfFileSpecification::Path { path } => path,
                PdfFileSpecification::Embedded { file } => file,
            }).unwrap_or_default(), fields: names(self, "Fields"), flags: dict_i64(&dict, "Flags").unwrap_or(0).max(0) as u32 },
            "ResetForm" => PdfActionKind::ResetForm { fields: names(self, "Fields"), flags: dict_i64(&dict, "Flags").unwrap_or(0).max(0) as u32 },
            "ImportData" => PdfActionKind::ImportData { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)).unwrap_or(PdfFileSpecification::Path { path: String::new() }) },
            "JavaScript" => PdfActionKind::JavaScript { script: match dict_get(&dict, "JS").map(|js| self.source.deref(js)) {
                Some(PdfObject::Str(bytes)) => decode_text_string(&bytes),
                Some(PdfObject::Stream { data, .. }) => String::from_utf8_lossy(&data).into_owned(),
                _ => String::new(),
            } },
            "SetOCGState" => PdfActionKind::SetOptionalContentState { states: extra_entries(&dict, &["Type", "S", "Next"]), preserve_radio_buttons: dict_get(&dict, "PreserveRB").and_then(PdfObject::as_bool).unwrap_or(true) },
            "Rendition" => PdfActionKind::Rendition { entries: extra_entries(&dict, &["Type", "S", "Next"]) },
            "Trans" => PdfActionKind::Transition { entries: extra_entries(&dict, &["Type", "S", "Next"]) },
            "GoTo3DView" => PdfActionKind::GoTo3dView { entries: extra_entries(&dict, &["Type", "S", "Next"]) },
            other => PdfActionKind::Unknown { subtype: other.to_string(), entries: extra_entries(&dict, &["Type", "S", "Next"]) },
        };
        let next = match dict_get(&dict, "Next").map(|n| self.source.deref(n)) {
            Some(PdfObject::Array(items)) => items.iter().filter_map(|item| self.lift_action(item)).collect(),
            Some(single) => self.lift_action(&single).into_iter().collect(),
            None => Vec::new(),
        };
        Some(PdfAction { kind, next })
    }

    /// 📑 Lifts the outline tree.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_outlines(&mut self, root: &PdfObject) -> Vec<PdfOutlineItem> {
        let root = self.source.deref(root);
        let Some(first) = root.dict_get("First").cloned() else { return Vec::new() };
        let mut visited = HashSet::new();
        self.lift_outline_siblings(&first, &mut visited, 0)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_outline_siblings(&mut self, first: &PdfObject, visited: &mut HashSet<u32>, depth: usize) -> Vec<PdfOutlineItem> {
        let mut items = Vec::new();
        let mut cursor = Some(first.clone());
        while let Some(current) = cursor {
            let Some(reference) = current.as_ref() else { break };
            if !visited.insert(reference.num) || depth > 32 {
                break;
            }
            let Some(dict) = self.source.get(reference).and_then(|item| item.as_dict().map(<[PdfDictEntry]>::to_vec)) else { break };
            let children = match dict_get(&dict, "First").cloned() {
                Some(child) => self.lift_outline_siblings(&child, visited, depth + 1),
                None => Vec::new(),
            };
            let flags = dict_i64(&dict, "F").unwrap_or(0);
            items.push(PdfOutlineItem {
                title: self.text(dict_get(&dict, "Title")).unwrap_or_default(),
                destination: dict_get(&dict, "Dest").cloned().and_then(|d| self.lift_destination(&d)),
                action: dict_get(&dict, "A").cloned().and_then(|a| self.lift_action(&a)),
                color: array_n::<3>(dict_get(&dict, "C")),
                italic: flags & 1 != 0,
                bold: flags & 2 != 0,
                open: dict_i64(&dict, "Count").unwrap_or(0) > 0,
                children,
                extra: extra_entries(&dict, &["Title", "Parent", "Prev", "Next", "First", "Last", "Count", "Dest", "A", "SE", "C", "F"]),
            });
            cursor = dict_get(&dict, "Next").cloned();
        }
        items
    }

    /// 🌳 Flattens a name tree (§7.9.6) into (key, value) pairs in tree order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn name_tree(&mut self, node: &PdfObject, out: &mut Vec<(Vec<u8>, PdfObject)>, visited: &mut HashSet<u32>) {
        if let Some(reference) = node.as_ref() {
            if !visited.insert(reference.num) {
                return;
            }
        }
        let node = self.source.deref(node);
        if let Some(kids) = node.dict_get("Kids").map(|k| self.source.deref(k)) {
            if let PdfObject::Array(items) = kids {
                for kid in items {
                    self.name_tree(&kid, out, visited);
                }
            }
        }
        if let Some(names) = node.dict_get("Names").map(|n| self.source.deref(n)) {
            if let PdfObject::Array(items) = names {
                for pair in items.chunks(2) {
                    if let [PdfObject::Str(key), value] = pair {
                        out.push((key.clone(), value.clone()));
                    }
                }
            }
        }
    }

    /// 🔢 Flattens a number tree (§7.9.7) into (key, value) pairs.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn number_tree(&mut self, node: &PdfObject, out: &mut Vec<(i64, PdfObject)>, visited: &mut HashSet<u32>) {
        if let Some(reference) = node.as_ref() {
            if !visited.insert(reference.num) {
                return;
            }
        }
        let node = self.source.deref(node);
        if let Some(PdfObject::Array(items)) = node.dict_get("Kids").map(|k| self.source.deref(k)) {
            for kid in items {
                self.number_tree(&kid, out, visited);
            }
        }
        if let Some(PdfObject::Array(items)) = node.dict_get("Nums").map(|n| self.source.deref(n)) {
            for pair in items.chunks(2) {
                if let [PdfObject::Int(key), value] = pair {
                    out.push((*key, value.clone()));
                }
            }
        }
    }
}
//#endregion 🔖️Navigation

//#region 🔖️Annotations
impl Lifter<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_appearance_entry(&mut self, id: &str, value: &PdfObject) -> Option<PdfAppearanceEntry> {
        let resolved = self.source.deref(value);
        match resolved {
            PdfObject::Stream { .. } => {
                let form = self.lift_x_object(id, value);
                (!form.is_empty()).then_some(PdfAppearanceEntry::Single { form })
            }
            PdfObject::Dict(states) => {
                let mut out = Vec::new();
                for state in states {
                    let form = self.lift_x_object(&format!("{id}{}", state.key), &state.value);
                    if !form.is_empty() {
                        out.push(PdfAppearanceState { state: state.key, form });
                    }
                }
                Some(PdfAppearanceEntry::States { states: out })
            }
            _ => None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_annotation(&mut self, dict: &[PdfDictEntry]) -> PdfAnnotation {
        let subtype = dict_name(dict, "Subtype").unwrap_or("").to_string();
        let rect = rect_of(dict_get(dict, "Rect")).unwrap_or([0.0; 4]);
        let quad_points = |lifter: &mut Self| lifter.source.deref(dict_get(dict, "QuadPoints").unwrap_or(&PdfObject::Null)).as_array().map(|items| items.iter().filter_map(PdfObject::as_f64).collect()).unwrap_or_default();
        let interior = |lifter: &mut Self| dict_get(dict, "IC").map(|ic| numbers_of(Some(&lifter.source.deref(ic))));
        let line_endings = |lifter: &mut Self| lifter.source.deref(dict_get(dict, "LE").unwrap_or(&PdfObject::Null)).as_array().and_then(|items| Some([items.first()?.as_name()?.to_string(), items.get(1)?.as_name()?.to_string()]));
        let vertices = |lifter: &mut Self| numbers_of(Some(&lifter.source.deref(dict_get(dict, "Vertices").unwrap_or(&PdfObject::Null))));
        let kind = match subtype.as_str() {
            "Text" => PdfAnnotationKind::Text { open: dict_get(dict, "Open").and_then(PdfObject::as_bool).unwrap_or(false), icon: dict_name(dict, "Name").map(str::to_string), state: self.text(dict_get(dict, "State")), state_model: self.text(dict_get(dict, "StateModel")) },
            "Link" => PdfAnnotationKind::Link { action: dict_get(dict, "A").cloned().and_then(|a| self.lift_action(&a)), destination: dict_get(dict, "Dest").cloned().and_then(|d| self.lift_destination(&d)), highlight: dict_name(dict, "H").map(str::to_string), quad_points: quad_points(self) },
            "FreeText" => PdfAnnotationKind::FreeText { default_appearance: self.text(dict_get(dict, "DA")).unwrap_or_default(), quadding: dict_i64(dict, "Q").unwrap_or(0).max(0) as u32, callout: dict_get(dict, "CL").map(|cl| numbers_of(Some(cl))), line_ending: dict_name(dict, "LE").map(str::to_string), rich_text: self.text(dict_get(dict, "RC")) },
            "Line" => PdfAnnotationKind::Line { points: array_n::<4>(dict_get(dict, "L")).unwrap_or([0.0; 4]), line_endings: line_endings(self), interior_color: interior(self), leader_length: dict_f64(dict, "LL"), caption: dict_get(dict, "Cap").and_then(PdfObject::as_bool).unwrap_or(false) },
            "Square" => PdfAnnotationKind::Square { interior_color: interior(self), rect_differences: rect_of(dict_get(dict, "RD")) },
            "Circle" => PdfAnnotationKind::Circle { interior_color: interior(self), rect_differences: rect_of(dict_get(dict, "RD")) },
            "Polygon" => PdfAnnotationKind::Polygon { vertices: vertices(self), interior_color: interior(self) },
            "PolyLine" => PdfAnnotationKind::PolyLine { vertices: vertices(self), line_endings: line_endings(self), interior_color: interior(self) },
            "Highlight" => PdfAnnotationKind::Highlight { quad_points: quad_points(self) },
            "Underline" => PdfAnnotationKind::Underline { quad_points: quad_points(self) },
            "Squiggly" => PdfAnnotationKind::Squiggly { quad_points: quad_points(self) },
            "StrikeOut" => PdfAnnotationKind::StrikeOut { quad_points: quad_points(self) },
            "Stamp" => PdfAnnotationKind::Stamp { icon: dict_name(dict, "Name").map(str::to_string) },
            "Caret" => PdfAnnotationKind::Caret { rect_differences: rect_of(dict_get(dict, "RD")), symbol: dict_name(dict, "Sy").map(str::to_string) },
            "Ink" => PdfAnnotationKind::Ink { paths: self.source.deref(dict_get(dict, "InkList").unwrap_or(&PdfObject::Null)).as_array().map(|paths| paths.iter().map(|path| numbers_of(Some(path))).collect()).unwrap_or_default() },
            "Popup" => PdfAnnotationKind::Popup { parent: dict_get(dict, "Parent").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| *index as usize)), open: dict_get(dict, "Open").and_then(PdfObject::as_bool).unwrap_or(false) },
            "FileAttachment" => PdfAnnotationKind::FileAttachment { file: dict_get(dict, "FS").cloned().and_then(|f| self.lift_file_specification(&f)).unwrap_or(PdfFileSpecification::Path { path: String::new() }), icon: dict_name(dict, "Name").map(str::to_string) },
            "Sound" => PdfAnnotationKind::Sound { sound: dict_get(dict, "Sound").map(|s| self.source.deref(s)).and_then(|s| s.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default(), icon: dict_name(dict, "Name").map(str::to_string) },
            "Movie" => PdfAnnotationKind::Movie { title: self.text(dict_get(dict, "T")), movie: dict_get(dict, "Movie").map(|m| self.source.deref(m)).and_then(|m| m.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default(), activation: dict_get(dict, "A").map(|a| self.source.deref(a)).and_then(|a| a.as_dict().map(<[PdfDictEntry]>::to_vec)) },
            "Widget" => PdfAnnotationKind::Widget { field: dict_get(dict, "T").is_some().then(|| self.text(dict_get(dict, "T"))).flatten().or_else(|| dict_get(dict, "Parent").map(|p| self.source.deref(p)).and_then(|p| p.dict_get("T").cloned()).map(|t| self.text(Some(&t))).flatten()), highlight: dict_name(dict, "H").map(str::to_string), characteristics: dict_get(dict, "MK").map(|mk| self.source.deref(mk)).and_then(|mk| mk.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default(), action: dict_get(dict, "A").cloned().and_then(|a| self.lift_action(&a)), additional_actions: dict_get(dict, "AA").map(|aa| self.source.deref(aa)).and_then(|aa| aa.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default() },
            "Screen" => PdfAnnotationKind::Screen { title: self.text(dict_get(dict, "T")), characteristics: dict_get(dict, "MK").map(|mk| self.source.deref(mk)).and_then(|mk| mk.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default(), action: dict_get(dict, "A").cloned().and_then(|a| self.lift_action(&a)), additional_actions: dict_get(dict, "AA").map(|aa| self.source.deref(aa)).and_then(|aa| aa.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default() },
            "PrinterMark" => PdfAnnotationKind::PrinterMark { mark_style: dict_name(dict, "MN").map(str::to_string), colorants: Vec::new() },
            "TrapNet" => PdfAnnotationKind::TrapNet { entries: extra_entries(dict, ANNOTATION_KEYS) },
            "Watermark" => PdfAnnotationKind::Watermark { fixed_print: dict_get(dict, "FixedPrint").map(|f| self.source.deref(f)).and_then(|f| f.as_dict().map(<[PdfDictEntry]>::to_vec)) },
            "3D" => PdfAnnotationKind::ThreeD { entries: extra_entries(dict, ANNOTATION_KEYS) },
            "Redact" => PdfAnnotationKind::Redact { quad_points: quad_points(self), interior_color: interior(self), overlay_text: self.text(dict_get(dict, "OverlayText")), repeat: dict_get(dict, "Repeat").and_then(PdfObject::as_bool).unwrap_or(false), default_appearance: self.text(dict_get(dict, "DA")), quadding: dict_i64(dict, "Q").unwrap_or(0).max(0) as u32 },
            other => PdfAnnotationKind::Unknown { subtype: other.to_string(), entries: extra_entries(dict, ANNOTATION_KEYS) },
        };
        let appearance = dict_get(dict, "AP").map(|ap| self.source.deref(ap)).and_then(|ap| {
            let normal = ap.dict_get("N").cloned()?;
            let normal = self.lift_appearance_entry("AP", &normal)?;
            let rollover = ap.dict_get("R").cloned().and_then(|r| self.lift_appearance_entry("APR", &r));
            let down = ap.dict_get("D").cloned().and_then(|d| self.lift_appearance_entry("APD", &d));
            Some(PdfAppearance { normal, rollover, down })
        });
        let border = if let Some(bs) = dict_get(dict, "BS").map(|bs| self.source.deref(bs)).filter(|bs| bs.as_dict().is_some()) {
            let bs = bs.as_dict().expect("checked").to_vec();
            Some(PdfBorderStyle { width: dict_f64(&bs, "W").unwrap_or(1.0), style: dict_name(&bs, "S").map(str::to_string), dash: dict_get(&bs, "D").map(|d| numbers_of(Some(d))), radii: None })
        } else {
            dict_get(dict, "Border").and_then(|b| b.as_array()).map(|items| PdfBorderStyle { width: items.get(2).and_then(PdfObject::as_f64).unwrap_or(1.0), style: None, dash: items.get(3).and_then(|d| d.as_array()).map(|d| d.iter().filter_map(PdfObject::as_f64).collect()), radii: Some([items.first().and_then(PdfObject::as_f64).unwrap_or(0.0), items.get(1).and_then(PdfObject::as_f64).unwrap_or(0.0)]) })
        };
        let markup = if MARKUP_SUBTYPES.contains(&subtype.as_str()) && ["T", "Popup", "CA", "RC", "CreationDate", "IRT", "Subj", "RT", "IT"].iter().any(|key| dict_get(dict, key).is_some()) {
            Some(PdfMarkupAnnotation {
                title: self.text(dict_get(dict, "T")),
                popup: dict_get(dict, "Popup").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| *index as usize)),
                opacity: dict_f64(dict, "CA"),
                rich_contents: self.text(dict_get(dict, "RC")),
                creation_date: self.date(dict_get(dict, "CreationDate")),
                in_reply_to: dict_get(dict, "IRT").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| *index as usize)),
                subject: self.text(dict_get(dict, "Subj")),
                reply_type: dict_name(dict, "RT").map(str::to_string),
                intent: dict_name(dict, "IT").map(str::to_string),
            })
        } else {
            None
        };
        PdfAnnotation {
            rect,
            kind,
            contents: self.text(dict_get(dict, "Contents")),
            name: self.text(dict_get(dict, "NM")),
            modified: self.text(dict_get(dict, "M")),
            flags: dict_i64(dict, "F").unwrap_or(0).max(0) as u32,
            border,
            color: numbers_of(dict_get(dict, "C")),
            appearance,
            appearance_state: dict_name(dict, "AS").map(str::to_string),
            markup,
            optional_content: dict_get(dict, "OC").and_then(|oc| self.existing_id(Category::OptionalContent, oc)),
            struct_parent: dict_i64(dict, "StructParent").map(|v| v as u32),
            extra: extra_entries(dict, ANNOTATION_KEYS),
        }
    }
}

const MARKUP_SUBTYPES: &[&str] = &["Text", "FreeText", "Line", "Square", "Circle", "Polygon", "PolyLine", "Highlight", "Underline", "Squiggly", "StrikeOut", "Stamp", "Caret", "Ink", "FileAttachment", "Sound", "Redact"];

const ANNOTATION_KEYS: &[&str] = &[
    "Type", "Subtype", "Rect", "Contents", "P", "NM", "M", "F", "AP", "AS", "Border", "BS", "C", "StructParent", "OC", "T", "Popup", "CA", "RC", "CreationDate", "IRT", "Subj", "RT", "IT", "ExData", "Open", "Name", "State", "StateModel", "A", "Dest", "H", "PA", "QuadPoints", "DA", "Q", "DS", "CL", "LE", "BE", "RD", "L", "LL", "LLE", "Cap", "LLO", "CP", "IC", "Vertices", "InkList", "Parent", "FS", "Sound", "Movie", "MK", "AA", "MN", "FixedPrint", "OverlayText", "Repeat", "Sy", "BM", "ca",
];
//#endregion 🔖️Annotations

//#region 🔖️Catalog
impl Lifter<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_optional_content(&mut self, catalog: &[PdfDictEntry]) {
        let Some(properties) = dict_get(catalog, "OCProperties").map(|p| self.source.deref(p)) else { return };
        let Some(properties) = properties.as_dict().map(<[PdfDictEntry]>::to_vec) else { return };
        let mut content = PdfOptionalContent::default();
        if let Some(PdfObject::Array(groups)) = dict_get(&properties, "OCGs").map(|g| self.source.deref(g)) {
            for group_ref in groups {
                let group = self.source.deref(&group_ref);
                let Some(group) = group.as_dict().map(<[PdfDictEntry]>::to_vec) else { continue };
                let name = self.text(dict_get(&group, "Name")).unwrap_or_default();
                let (id, fresh) = self.claim_id(Category::OptionalContent, &format!("OCG{}", content.groups.len() + 1), group_ref.as_ref());
                if fresh {
                    content.groups.push(PdfOptionalContentGroup { id, name, intent: match dict_get(&group, "Intent") {
                        Some(PdfObject::Name(intent)) => vec![intent.clone()],
                        Some(PdfObject::Array(items)) => items.iter().filter_map(PdfObject::as_name).map(str::to_string).collect(),
                        _ => Vec::new(),
                    }, usage: dict_get(&group, "Usage").map(|u| self.source.deref(u)).and_then(|u| u.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default() });
                }
            }
        }
        if let Some(config) = dict_get(&properties, "D").map(|d| self.source.deref(d)).and_then(|d| d.as_dict().map(<[PdfDictEntry]>::to_vec)) {
            content.name = self.text(dict_get(&config, "Name"));
            content.base_state_off = dict_name(&config, "BaseState") == Some("OFF");
            let ids = |lifter: &mut Self, key: &str| -> Vec<String> { lifter.source.deref(dict_get(&config, key).unwrap_or(&PdfObject::Null)).as_array().map(|items| items.iter().filter_map(|item| lifter.existing_id(Category::OptionalContent, item)).collect()).unwrap_or_default() };
            content.on = ids(self, "ON");
            content.off = ids(self, "OFF");
            content.order = self.source.deref(dict_get(&config, "Order").unwrap_or(&PdfObject::Null)).as_array().map(|items| items.iter().map(|item| self.map_oc_refs(item)).collect()).unwrap_or_default();
            content.extra = extra_entries(&config, &["Type", "Name", "BaseState", "ON", "OFF", "Order"]);
        }
        self.snapshot.optional_content = Some(content);
    }

    /// 🔁 Replaces optional-content-group references in an `/Order` entry by their ids.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn map_oc_refs(&mut self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Ref(_) => self.existing_id(Category::OptionalContent, value).map(PdfObject::Name).unwrap_or(PdfObject::Null),
            PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| self.map_oc_refs(item)).collect()),
            other => other.clone(),
        }
    }

    /// 📎 Lifts the `/EmbeddedFiles` name tree first so its keys become the file ids.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_embedded_files(&mut self, catalog: &[PdfDictEntry]) {
        let Some(names) = dict_get(catalog, "Names").map(|n| self.source.deref(n)) else { return };
        let Some(files) = names.dict_get("EmbeddedFiles").cloned() else { return };
        let mut pairs = Vec::new();
        self.name_tree(&files, &mut pairs, &mut HashSet::new());
        for (key, value) in pairs {
            let name = decode_text_string(&key);
            self.lift_embedded_file(&name, &value, true);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_catalog(&mut self, catalog: &[PdfDictEntry]) {
        if let Some(outlines) = dict_get(catalog, "Outlines") {
            self.snapshot.outlines = self.lift_outlines(outlines);
        }
        if let Some(names) = dict_get(catalog, "Names").map(|n| self.source.deref(n)) {
            if let Some(dests) = names.dict_get("Dests").cloned() {
                let mut pairs = Vec::new();
                self.name_tree(&dests, &mut pairs, &mut HashSet::new());
                for (key, value) in pairs {
                    if let Some(destination) = self.lift_destination(&value) {
                        self.snapshot.named_destinations.push(PdfNamedDestination { name: decode_text_string(&key), destination });
                    }
                }
            }
        }
        if let Some(dests) = dict_get(catalog, "Dests").map(|d| self.source.deref(d)).and_then(|d| d.as_dict().map(<[PdfDictEntry]>::to_vec)) {
            for entry in dests {
                if let Some(destination) = self.lift_destination(&entry.value) {
                    self.snapshot.named_destinations.push(PdfNamedDestination { name: entry.key, destination });
                }
            }
        }
        if let Some(labels) = dict_get(catalog, "PageLabels").cloned() {
            let mut pairs = Vec::new();
            self.number_tree(&labels, &mut pairs, &mut HashSet::new());
            for (index, value) in pairs {
                let label = self.source.deref(&value);
                let dict = label.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
                let prefix = self.text(dict_get(&dict, "P"));
                self.snapshot.page_labels.push(PdfPageLabelRange {
                    start_index: index.max(0) as u32,
                    style: dict_name(&dict, "S").and_then(|s| match s {
                        "D" => Some(PdfPageLabelStyle::Decimal),
                        "R" => Some(PdfPageLabelStyle::RomanUpper),
                        "r" => Some(PdfPageLabelStyle::RomanLower),
                        "A" => Some(PdfPageLabelStyle::LettersUpper),
                        "a" => Some(PdfPageLabelStyle::LettersLower),
                        _ => None,
                    }),
                    prefix,
                    start: dict_i64(&dict, "St").unwrap_or(1).max(1) as u32,
                });
            }
        }
        if let Some(PdfObject::Array(intents)) = dict_get(catalog, "OutputIntents").map(|o| self.source.deref(o)) {
            for intent in intents {
                let intent = self.source.deref(&intent);
                let Some(dict) = intent.as_dict().map(<[PdfDictEntry]>::to_vec) else { continue };
                let condition_identifier = self.text(dict_get(&dict, "OutputConditionIdentifier")).unwrap_or_default();
                let condition = self.text(dict_get(&dict, "OutputCondition"));
                let registry_name = self.text(dict_get(&dict, "RegistryName"));
                let info = self.text(dict_get(&dict, "Info"));
                let profile = dict_get(&dict, "DestOutputProfile").map(|p| self.source.deref(p)).and_then(|p| match p {
                    PdfObject::Stream { data, .. } => Some(data),
                    _ => None,
                });
                self.snapshot.output_intents.push(PdfOutputIntent { subtype: dict_name(&dict, "S").unwrap_or("GTS_PDFA1").to_string(), condition_identifier, condition, registry_name, info, profile });
            }
        }
        if let Some(form) = dict_get(catalog, "AcroForm").map(|f| self.source.deref(f)).and_then(|f| f.as_dict().map(<[PdfDictEntry]>::to_vec)) {
            self.snapshot.acro_form = Some(self.lift_acro_form(&form));
        }
        self.snapshot.page_layout = dict_name(catalog, "PageLayout").and_then(|layout| match layout {
            "SinglePage" => Some(PdfPageLayout::SinglePage),
            "OneColumn" => Some(PdfPageLayout::OneColumn),
            "TwoColumnLeft" => Some(PdfPageLayout::TwoColumnLeft),
            "TwoColumnRight" => Some(PdfPageLayout::TwoColumnRight),
            "TwoPageLeft" => Some(PdfPageLayout::TwoPageLeft),
            "TwoPageRight" => Some(PdfPageLayout::TwoPageRight),
            _ => None,
        });
        self.snapshot.page_mode = dict_name(catalog, "PageMode").and_then(page_mode_from_name);
        if let Some(preferences) = dict_get(catalog, "ViewerPreferences").map(|v| self.source.deref(v)).and_then(|v| v.as_dict().map(<[PdfDictEntry]>::to_vec)) {
            let flag = |key: &str| dict_get(&preferences, key).and_then(PdfObject::as_bool).unwrap_or(false);
            self.snapshot.viewer_preferences = Some(PdfViewerPreferences {
                hide_toolbar: flag("HideToolbar"),
                hide_menubar: flag("HideMenubar"),
                hide_window_ui: flag("HideWindowUI"),
                fit_window: flag("FitWindow"),
                center_window: flag("CenterWindow"),
                display_doc_title: flag("DisplayDocTitle"),
                non_full_screen_page_mode: dict_name(&preferences, "NonFullScreenPageMode").and_then(page_mode_from_name),
                direction: dict_name(&preferences, "Direction").map(str::to_string),
                view_area: dict_name(&preferences, "ViewArea").map(str::to_string),
                view_clip: dict_name(&preferences, "ViewClip").map(str::to_string),
                print_area: dict_name(&preferences, "PrintArea").map(str::to_string),
                print_clip: dict_name(&preferences, "PrintClip").map(str::to_string),
                print_scaling: dict_name(&preferences, "PrintScaling").map(str::to_string),
                duplex: dict_name(&preferences, "Duplex").map(str::to_string),
                pick_tray_by_pdf_size: flag("PickTrayByPDFSize"),
                print_page_range: numbers_of(dict_get(&preferences, "PrintPageRange")).iter().map(|v| *v as u32).collect(),
                num_copies: dict_i64(&preferences, "NumCopies").map(|v| v as u32),
                extra: extra_entries(&preferences, &["HideToolbar", "HideMenubar", "HideWindowUI", "FitWindow", "CenterWindow", "DisplayDocTitle", "NonFullScreenPageMode", "Direction", "ViewArea", "ViewClip", "PrintArea", "PrintClip", "PrintScaling", "Duplex", "PickTrayByPDFSize", "PrintPageRange", "NumCopies"]),
            });
        }
        if let Some(open) = dict_get(catalog, "OpenAction").cloned() {
            let resolved = self.source.deref(&open);
            self.snapshot.open_action = match resolved {
                PdfObject::Array(_) => self.lift_destination(&open).map(|destination| PdfOpenAction::Destination { destination }),
                PdfObject::Dict(ref dict) if dict_get(dict, "S").is_some() => self.lift_action(&open).map(|action| PdfOpenAction::Action { action }),
                _ => self.lift_destination(&open).map(|destination| PdfOpenAction::Destination { destination }),
            };
        }
        self.snapshot.language = self.text(dict_get(catalog, "Lang"));
        if let Some(mark) = dict_get(catalog, "MarkInfo").map(|m| self.source.deref(m)).and_then(|m| m.as_dict().map(<[PdfDictEntry]>::to_vec)) {
            let flag = |key: &str| dict_get(&mark, key).and_then(PdfObject::as_bool).unwrap_or(false);
            self.snapshot.mark_info = Some(PdfMarkInfo { marked: flag("Marked"), user_properties: flag("UserProperties"), suspects: flag("Suspects") });
        }
        self.snapshot.metadata = dict_get(catalog, "Metadata").map(|m| self.source.deref(m)).and_then(|m| match m {
            PdfObject::Stream { data, .. } => Some(String::from_utf8_lossy(&data).into_owned()),
            _ => None,
        });
        self.snapshot.catalog_extra = extra_entries(catalog, &["Type", "Version", "Pages", "Outlines", "Names", "Dests", "PageLabels", "OutputIntents", "AcroForm", "PageLayout", "PageMode", "ViewerPreferences", "OpenAction", "Lang", "MarkInfo", "Metadata", "OCProperties"]);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_acro_form(&mut self, form: &[PdfDictEntry]) -> PdfAcroForm {
        let mut default_fonts = Vec::new();
        if let Some(resources) = dict_get(form, "DR").cloned() {
            let map = self.lift_resources(&resources);
            let mut ids: Vec<String> = map.fonts.into_values().collect();
            ids.sort();
            default_fonts = ids;
        }
        let fields = self.source.deref(dict_get(form, "Fields").unwrap_or(&PdfObject::Null)).as_array().map(|items| items.iter().filter_map(|item| self.lift_form_field(item, None, 0)).collect()).unwrap_or_default();
        PdfAcroForm {
            fields,
            need_appearances: dict_get(form, "NeedAppearances").and_then(PdfObject::as_bool).unwrap_or(false),
            signature_flags: dict_i64(form, "SigFlags").unwrap_or(0).max(0) as u32,
            default_appearance: self.text(dict_get(form, "DA")),
            quadding: dict_i64(form, "Q").map(|v| v.max(0) as u32),
            default_fonts,
            extra: extra_entries(form, &["Fields", "NeedAppearances", "SigFlags", "DA", "Q", "DR", "CO"]),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lift_form_field(&mut self, value: &PdfObject, inherited_type: Option<&str>, depth: usize) -> Option<PdfFormField> {
        if depth > 32 {
            return None;
        }
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict()?.to_vec();
        let field_type = dict_name(&dict, "FT").map(str::to_string).or_else(|| inherited_type.map(str::to_string));
        let text_value = |lifter: &mut Self, key: &str| -> Option<String> {
            match dict_get(&dict, key).map(|v| lifter.source.deref(v)) {
                Some(PdfObject::Str(bytes)) => Some(decode_text_string(&bytes)),
                Some(PdfObject::Name(name)) => Some(name),
                Some(PdfObject::Stream { data, .. }) => Some(String::from_utf8_lossy(&data).into_owned()),
                _ => None,
            }
        };
        let text_values = |lifter: &mut Self, key: &str| -> Vec<String> {
            match dict_get(&dict, key).map(|v| lifter.source.deref(v)) {
                Some(PdfObject::Array(items)) => items.iter().filter_map(|item| match item {
                    PdfObject::Str(bytes) => Some(decode_text_string(bytes)),
                    PdfObject::Name(name) => Some(name.clone()),
                    _ => None,
                }).collect(),
                Some(PdfObject::Str(bytes)) => vec![decode_text_string(&bytes)],
                Some(PdfObject::Name(name)) => vec![name],
                _ => Vec::new(),
            }
        };
        let kids: Vec<PdfObject> = self.source.deref(dict_get(&dict, "Kids").unwrap_or(&PdfObject::Null)).as_array().map(<[PdfObject]>::to_vec).unwrap_or_default();
        let mut widgets = Vec::new();
        if let Some(reference) = value.as_ref() {
            if let Some((page, index)) = self.annotation_refs.get(&reference) {
                widgets.push([*page, *index]);
            }
        }
        let mut children = Vec::new();
        for kid in &kids {
            let kid_dict = self.source.deref(kid);
            let is_widget_only = kid_dict.dict_get("T").is_none() && kid_dict.dict_get("Subtype").and_then(PdfObject::as_name) == Some("Widget");
            if is_widget_only {
                if let Some((page, index)) = kid.as_ref().and_then(|r| self.annotation_refs.get(&r)) {
                    widgets.push([*page, *index]);
                }
            } else if let Some(child) = self.lift_form_field(kid, field_type.as_deref(), depth + 1) {
                children.push(child);
            }
        }
        let kind = match field_type.as_deref() {
            Some("Btn") => PdfFormFieldKind::Button { value: text_value(self, "V"), default_value: text_value(self, "DV"), options: text_values(self, "Opt") },
            Some("Tx") => PdfFormFieldKind::Text { value: text_value(self, "V"), default_value: text_value(self, "DV"), max_length: dict_i64(&dict, "MaxLen").map(|v| v.max(0) as u32), rich_value: text_value(self, "RV") },
            Some("Ch") => PdfFormFieldKind::Choice {
                values: text_values(self, "V"),
                default_values: text_values(self, "DV"),
                options: self.source.deref(dict_get(&dict, "Opt").unwrap_or(&PdfObject::Null)).as_array().map(|items| items.iter().map(|item| match item {
                    PdfObject::Array(pair) => (pair.first().and_then(PdfObject::as_str_bytes).map(decode_text_string).unwrap_or_default(), pair.get(1).and_then(PdfObject::as_str_bytes).map(decode_text_string).unwrap_or_default()),
                    PdfObject::Str(bytes) => (decode_text_string(bytes), decode_text_string(bytes)),
                    _ => (String::new(), String::new()),
                }).collect()).unwrap_or_default(),
                top_index: dict_i64(&dict, "TI").map(|v| v.max(0) as u32),
            },
            Some("Sig") => PdfFormFieldKind::Signature { value: dict_get(&dict, "V").map(|v| self.source.deref(v)).and_then(|v| v.as_dict().map(<[PdfDictEntry]>::to_vec)) },
            _ => PdfFormFieldKind::Container,
        };
        Some(PdfFormField {
            name: text_value(self, "T").unwrap_or_default(),
            kind,
            flags: dict_i64(&dict, "Ff").unwrap_or(0).max(0) as u32,
            alternate_name: text_value(self, "TU"),
            mapping_name: text_value(self, "TM"),
            default_appearance: text_value(self, "DA"),
            quadding: dict_i64(&dict, "Q").map(|v| v.max(0) as u32),
            widgets,
            children,
            additional_actions: dict_get(&dict, "AA").map(|aa| self.source.deref(aa)).and_then(|aa| aa.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default(),
            extra: extra_entries(&dict, &["T", "FT", "Ff", "V", "DV", "Kids", "Parent", "Opt", "MaxLen", "TI", "DA", "Q", "TU", "TM", "AA", "RV", "I", "Type", "Subtype", "Rect", "P", "F", "AP", "AS", "MK", "BS", "Border", "H", "A", "NM", "M", "OC", "StructParent", "Contents", "C"]),
        })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_mode_from_name(name: &str) -> Option<PdfPageMode> {
    match name {
        "UseNone" => Some(PdfPageMode::UseNone),
        "UseOutlines" => Some(PdfPageMode::UseOutlines),
        "UseThumbs" => Some(PdfPageMode::UseThumbs),
        "FullScreen" => Some(PdfPageMode::FullScreen),
        "UseOC" => Some(PdfPageMode::UseOc),
        "UseAttachments" => Some(PdfPageMode::UseAttachments),
        _ => None,
    }
}
//#endregion 🔖️Catalog

#[allow(unused_imports)]
use lift_function as _lift_function_is_used_by_colour;
