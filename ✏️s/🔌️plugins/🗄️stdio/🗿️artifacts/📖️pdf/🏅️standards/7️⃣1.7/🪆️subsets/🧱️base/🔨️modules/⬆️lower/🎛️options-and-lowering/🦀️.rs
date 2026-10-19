//! 🎛️ The lowering options, the `Lowering` pass state with its object table and reference maps, `LoweredDocument` and the document-level lowering entry points.

use crate::standards::v1_7::subsets::base::modules::colour::{entry, raw_stream, stream};
use crate::standards::v1_7::subsets::base::modules::content::{content_references, print_content};
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use crate::standards::v1_7::subsets::base::io::error::PResult;
use std::collections::{BTreeSet, HashMap};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::modules::lower::*;

//#region 🔖️Options
/// 🎛️ Writer choices that do not change the typed model (so the default keeps `lift ∘ lower`
/// the identity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LowerOptions {
    /// ✂️ Subset embedded TrueType programs to the glyphs the document shows.
    pub subset_fonts: bool,
    /// 🗜️ Flate-compress content streams and packed image samples.
    pub compress: bool,
}

impl Default for LowerOptions {
    fn default() -> Self {
        Self { subset_fonts: false, compress: true }
    }
}
//#endregion 🔖️Options

//#region 🔖️Lowering
pub use crate::standards::v1_7::subsets::base::modules::lift::Category;

/// 🧱 One lowering pass: the objects produced so far and the references each typed id maps to.
pub struct Lowering<'a> {
    pub(crate) snapshot: &'a PdfSnapshot,
    pub(crate) options: LowerOptions,
    pub objects: Vec<PdfIndirectObject>,
    pub(crate) next: u32,
    pub refs: HashMap<(Category, String), ObjRef>,
    pub annotation_refs: Vec<Vec<ObjRef>>,
    pub field_refs: Vec<ObjRef>,
    pub widget_parents: HashMap<[u32; 2], ObjRef>,
    pub(crate) codecs: HashMap<String, FontCodec>,
    pub(crate) artifacts:NativePdfArtifactResources<'a>,
    pub(crate) used_glyphs: HashMap<String, BTreeSet<u16>>,
}

impl ObjectSink for Lowering<'_> {
    fn resolve_artifact(&self,reference:&semio_framework_artifact_reference::ArtifactRef)->PResult<PdfObject> {self.artifacts.resolve(reference)}
    fn add(&mut self, value: PdfObject) -> ObjRef {
        let reference = self.reserve();
        self.objects.push(PdfIndirectObject { id: reference, value });
        reference
    }
}

impl FontTable for Lowering<'_> {
    fn content_artifact(&self,reference:&semio_framework_artifact_reference::ArtifactRef)->PResult<PdfObject>{self.artifacts.resolve(reference)}
    fn font(&self, name: &str) -> Option<&FontCodec> {
        self.codecs.get(name)
    }
}

/// 📦 What lowering a document yields: its objects, the catalog reference and the info
/// reference (both already inside `objects`).
pub struct LoweredDocument {
    pub objects: Vec<PdfIndirectObject>,
    pub root: ObjRef,
    pub info: Option<ObjRef>,
    pub refs: HashMap<(Category, String), ObjRef>,
    pub annotation_refs: Vec<Vec<ObjRef>>,
    pub field_refs: Vec<ObjRef>,
    pub widget_parents: HashMap<[u32; 2], ObjRef>,
}

impl<'a> Lowering<'a> {
    pub fn new(snapshot: &'a PdfSnapshot, first_number: u32, options: LowerOptions) -> PResult<Self> {
        let artifacts=NativePdfArtifactResources::from_objects(&snapshot.objects);
        let codecs = snapshot.fonts.iter().map(|font| Ok((font.id.clone(), FontCodec::new(font,&artifacts)?))).collect::<PResult<_>>()?;
        Ok(Self { snapshot, options, objects: Vec::new(), next: first_number.max(1), refs: HashMap::new(), annotation_refs: Vec::new(), field_refs: Vec::new(), widget_parents: HashMap::new(), codecs, artifacts, used_glyphs: HashMap::new() })
    }

    pub(crate) fn reserve(&mut self) -> ObjRef {
        let reference = ObjRef { num: self.next, gen: 0 };
        self.next += 1;
        reference
    }

    pub(crate) fn set(&mut self, reference: ObjRef, value: PdfObject) {
        match self.objects.iter_mut().find(|object| object.id == reference) {
            Some(object) => object.value = value,
            None => self.objects.push(PdfIndirectObject { id: reference, value }),
        }
    }

    pub(crate) fn reference(&self, category: Category, id: &str) -> Option<PdfObject> {
        self.refs.get(&(category, id.to_string())).map(|reference| PdfObject::Ref(*reference))
    }

    /// 🗜️ A content stream object for `ops`.
    pub(crate) fn content_stream(&mut self, ops: &[PdfOp]) -> PResult<PdfObject> {
        let bytes = print_content(ops, self)?;
        Ok(if self.options.compress { stream(Vec::new(), bytes) } else { raw_stream(Vec::new(), bytes) })
    }

    /// 📚 The `/Resources` dictionary a content stream needs.
    pub(crate) fn resources_for(&mut self, ops: &[PdfOp]) -> PdfObject {
        let references = content_references(ops);
        let mut dict = Vec::new();
        let mut sub = |lowering: &mut Self, key: &str, category: Category, names: &[String]| {
            let entries: Vec<PdfDictEntry> = names.iter().filter_map(|name| lowering.reference(category, name).map(|reference| PdfDictEntry::new(name, reference))).collect();
            if !entries.is_empty() {
                dict.push(entry(key, PdfObject::Dict(entries)));
            }
        };
        sub(self, "Font", Category::Font, &references.fonts);
        sub(self, "XObject", Category::XObject, &references.x_objects);
        sub(self, "ExtGState", Category::ExtGState, &references.ext_g_states);
        sub(self, "Shading", Category::Shading, &references.shadings);
        sub(self, "Pattern", Category::Pattern, &references.patterns);
        sub(self, "ColorSpace", Category::ColorSpace, &references.color_spaces);
        sub(self, "Properties", Category::Properties, &references.properties);
        dict.push(entry("ProcSet", PdfObject::Array(["PDF", "Text", "ImageB", "ImageC", "ImageI"].iter().map(|name| PdfObject::name(*name)).collect())));
        PdfObject::Dict(dict)
    }

    /// 🔢 Records the glyphs `ops` show per font (for subsetting).
    fn record_glyphs(&mut self, ops: &[PdfOp]) {
        let mut current: Option<String> = None;
        let mut stack: Vec<Option<String>> = Vec::new();
        let use_text = |lowering: &mut Self, font: &Option<String>, text: &PdfTextString| {
            let Some(font) = font else { return };
            let Some(codec) = lowering.codecs.get(font) else { return };
            let decoded = match text {
                PdfTextString::Codes { codes } => codec.decode_codes(codes),
                PdfTextString::Text { text } => codec.decode(&codec.encode(text).unwrap_or_default()),
            };
            let glyphs: Vec<u16> = decoded.iter().filter_map(|glyph| glyph.glyph_id).collect();
            lowering.used_glyphs.entry(font.clone()).or_default().extend(glyphs);
        };
        for op in ops {
            match op {
                PdfOp::SetFont { name, .. } => current = Some(name.clone()),
                PdfOp::Save => stack.push(current.clone()),
                PdfOp::Restore => {
                    if let Some(saved) = stack.pop() {
                        current = saved;
                    }
                }
                PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => use_text(self, &current, text),
                PdfOp::ShowTextArray { items } => {
                    for item in items {
                        match item {
                            PdfTextArrayItem::Text { text } => use_text(self, &current, &PdfTextString::Text { text: text.clone() }),
                            PdfTextArrayItem::Codes { codes } => use_text(self, &current, &PdfTextString::Codes { codes: codes.clone() }),
                            PdfTextArrayItem::Adjust { .. } => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// ⬆️ Lowers one page against an already-lowered document: `refs` are the resource references
/// of a previous [`lower_document`] (or [`lower_document_headless`]) pass, `pages_ref` the page
/// tree root, `page_ref` the reserved reference of this page. Returns the page's own objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_page_standalone(snapshot: &PdfSnapshot, page: &PdfPage, page_index: usize, page_ref: ObjRef, pages_ref: ObjRef, first_number: u32, options: LowerOptions, refs: &HashMap<(Category, String), ObjRef>, widget_parents: &HashMap<[u32; 2], ObjRef>) -> PResult<(Vec<PdfIndirectObject>, Vec<ObjRef>)> {
    let mut lowering = Lowering::new(snapshot, first_number, options)?;
    lowering.refs = refs.clone();
    lowering.widget_parents = widget_parents.clone();
    lowering.refs.insert((Category::Page, page_index.to_string()), page_ref);
    lowering.annotation_refs = vec![Vec::new(); page_index];
    let annotation_refs: Vec<ObjRef> = page.annotations.iter().map(|_| lowering.reserve()).collect();
    lowering.annotation_refs.push(annotation_refs.clone());
    lowering.lower_page(page_index, page, page_ref, pages_ref)?;
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok((lowering.objects, annotation_refs))
}

/// ⬆️ Lowers the interactive form after every page was written (its fields reference the
/// pages' widget annotations); returns the objects and the `/AcroForm` reference.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_acro_form_standalone(snapshot: &PdfSnapshot, first_number: u32, options: LowerOptions, refs: &HashMap<(Category, String), ObjRef>, annotation_refs: Vec<Vec<ObjRef>>, field_refs: Vec<ObjRef>) -> PResult<Option<(Vec<PdfIndirectObject>, ObjRef)>> {
    let Some(form) = &snapshot.acro_form else { return Ok(None) };
    let mut lowering = Lowering::new(snapshot, first_number, options)?;
    lowering.refs = refs.clone();
    lowering.annotation_refs = annotation_refs;
    lowering.field_refs = field_refs;
    let value = lowering.lower_acro_form(form);
    let reference = lowering.add(PdfObject::Dict(value));
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok(Some((lowering.objects, reference)))
}

/// ⬆️ Lowers the catalog against an existing graph: `refs` resolve every page and resource the
/// catalog lanes name (so destinations and actions point into that graph), `pages_ref` is its page
/// tree root. Returns the catalog entries and the objects they introduce, numbered from
/// `first_number` — the raw material a reconciling write grafts the moved catalog lanes from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_catalog_standalone(snapshot: &PdfSnapshot, first_number: u32, options: LowerOptions, refs: &HashMap<(Category, String), ObjRef>, pages_ref: ObjRef) -> PResult<(Vec<PdfDictEntry>, Vec<PdfIndirectObject>)> {
    let mut lowering = Lowering::new(snapshot, first_number, options)?;
    lowering.refs = refs.clone();
    let catalog = lowering.lower_catalog(pages_ref, false)?;
    Ok((catalog, lowering.objects))
}

/// ⬆️ Lowers everything but the pages, reserving `expected_pages` page references (so
/// destinations and outlines resolve) — the first half of a streamed write.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_document_headless(snapshot: &PdfSnapshot, expected_pages: usize, options: LowerOptions) -> PResult<(LoweredDocument, ObjRef, Vec<ObjRef>, PdfObject)> {
    let mut headless = snapshot.clone();
    headless.pages.clear();
    let mut lowering = Lowering::new(&headless, 1, options)?;
    let catalog_ref = lowering.reserve();
    let pages_ref = lowering.reserve();
    let info_ref = if headless.info.is_empty() { None } else { Some(lowering.reserve()) };
    let page_refs: Vec<ObjRef> = (0..expected_pages).map(|_| lowering.reserve()).collect();
    for (index, reference) in page_refs.iter().enumerate() {
        lowering.refs.insert((Category::Page, index.to_string()), *reference);
    }
    lowering.reserve_resources();
    lowering.reserve_fields();
    lowering.lower_optional_content_groups();
    lowering.lower_fonts()?;
    lowering.lower_images()?;
    lowering.lower_forms()?;
    lowering.lower_ext_g_states()?;
    lowering.lower_shadings()?;
    lowering.lower_patterns()?;
    lowering.lower_named_resources()?;
    lowering.lower_embedded_files();
    let catalog = lowering.lower_catalog(pages_ref, false)?;
    lowering.set(catalog_ref, PdfObject::Dict(catalog));
    if let Some(info_ref) = info_ref {
        let info = lower_info(&snapshot.info);
        lowering.set(info_ref, PdfObject::Dict(info));
    }
    let root_resources = lowering.all_resources();
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok((LoweredDocument { objects: lowering.objects, root: catalog_ref, info: info_ref, refs: lowering.refs, annotation_refs: Vec::new(), field_refs: lowering.field_refs, widget_parents: lowering.widget_parents }, pages_ref, page_refs, root_resources))
}

impl Lowering<'_> {
    /// 🆔 Reserves one reference per interactive-form field (depth-first) and notes which field
    /// every widget annotation belongs to.
    fn reserve_fields(&mut self) {
        fn walk(lowering: &mut Lowering<'_>, fields: &[PdfFormField]) {
            for field in fields {
                let reference = lowering.reserve();
                lowering.field_refs.push(reference);
                for widget in &field.widgets {
                    lowering.widget_parents.insert(*widget, reference);
                }
                walk(lowering, &field.children);
            }
        }
        if let Some(form) = &self.snapshot.acro_form {
            walk(self, &form.fields);
        }
    }

    /// 📚 The page-tree root's `/Resources`: every document-level resource, in snapshot order —
    /// what keeps unreferenced resources (and their order) alive across a write.
    pub fn all_resources(&self) -> PdfObject {
        let snapshot = self.snapshot;
        let mut dict = Vec::new();
        let mut sub = |key: &str, category: Category, ids: Vec<String>| {
            let entries: Vec<PdfDictEntry> = ids.iter().filter_map(|id| self.reference(category, id).map(|reference| PdfDictEntry::new(id, reference))).collect();
            if !entries.is_empty() {
                dict.push(entry(key, PdfObject::Dict(entries)));
            }
        };
        sub("Font", Category::Font, snapshot.fonts.iter().map(|f| f.id.clone()).collect());
        sub("XObject", Category::XObject, snapshot.images.iter().map(|i| i.id.clone()).chain(snapshot.forms.iter().map(|f| f.id.clone())).collect());
        sub("ExtGState", Category::ExtGState, snapshot.ext_g_states.iter().map(|s| s.id.clone()).collect());
        sub("Shading", Category::Shading, snapshot.shadings.iter().map(|s| s.id.clone()).collect());
        sub("Pattern", Category::Pattern, snapshot.patterns.iter().map(|p| p.id.clone()).collect());
        sub("ColorSpace", Category::ColorSpace, snapshot.color_spaces.iter().map(|c| c.name.clone()).collect());
        sub("Properties", Category::Properties, snapshot.properties.iter().map(|p| p.name.clone()).collect());
        PdfObject::Dict(dict)
    }

    /// 🆔 Reserves one reference per document-level resource id.
    fn reserve_resources(&mut self) {
        let snapshot = self.snapshot;
        for (category, ids) in [
            (Category::Font, snapshot.fonts.iter().map(|f| f.id.clone()).collect::<Vec<_>>()),
            (Category::XObject, snapshot.images.iter().map(|i| i.id.clone()).chain(snapshot.forms.iter().map(|f| f.id.clone())).collect()),
            (Category::ExtGState, snapshot.ext_g_states.iter().map(|s| s.id.clone()).collect()),
            (Category::Shading, snapshot.shadings.iter().map(|s| s.id.clone()).collect()),
            (Category::Pattern, snapshot.patterns.iter().map(|p| p.id.clone()).collect()),
            (Category::ColorSpace, snapshot.color_spaces.iter().map(|c| c.name.clone()).collect()),
            (Category::Properties, snapshot.properties.iter().map(|p| p.name.clone()).collect()),
            (Category::EmbeddedFile, snapshot.embedded_files.iter().map(|f| f.id.clone()).collect()),
            (Category::OptionalContent, snapshot.optional_content.iter().flat_map(|oc| oc.groups.iter().map(|g| g.id.clone())).collect()),
        ] {
            for id in ids {
                let reference = self.reserve();
                self.refs.insert((category, id), reference);
            }
        }
    }
}

/// ⬆️ Lowers the whole typed document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_document(snapshot: &PdfSnapshot, first_number: u32, options: LowerOptions) -> PResult<LoweredDocument> {
    let mut lowering = Lowering::new(snapshot, first_number, options)?;
    let catalog_ref = lowering.reserve();
    let pages_ref = lowering.reserve();
    let info_ref = if snapshot.info.is_empty() { None } else { Some(lowering.reserve()) };
    let page_refs: Vec<ObjRef> = snapshot.pages.iter().map(|_| lowering.reserve()).collect();
    for (index, reference) in page_refs.iter().enumerate() {
        lowering.refs.insert((Category::Page, index.to_string()), *reference);
    }
    lowering.annotation_refs = snapshot.pages.iter().map(|page| page.annotations.iter().map(|_| lowering.reserve()).collect()).collect();
    lowering.reserve_resources();
    lowering.reserve_fields();
    if lowering.options.subset_fonts {
        for page in &snapshot.pages {
            lowering.record_glyphs(&page.content);
        }
        for form in &snapshot.forms {
            lowering.record_glyphs(&form.content);
        }
        for pattern in &snapshot.patterns {
            if let PdfPatternKind::Tiling { content, .. } = &pattern.kind {
                lowering.record_glyphs(content);
            }
        }
    }
    lowering.lower_optional_content_groups();
    lowering.lower_fonts()?;
    lowering.lower_images()?;
    lowering.lower_forms()?;
    lowering.lower_ext_g_states()?;
    lowering.lower_shadings()?;
    lowering.lower_patterns()?;
    lowering.lower_named_resources()?;
    lowering.lower_embedded_files();
    for (index, page) in snapshot.pages.iter().enumerate() {
        lowering.lower_page(index, page, page_refs[index], pages_ref)?;
    }
    let root_resources = lowering.all_resources();
    lowering.set(pages_ref, PdfObject::Dict(vec![entry("Type", PdfObject::name("Pages")), entry("Kids", PdfObject::Array(page_refs.iter().map(|r| PdfObject::Ref(*r)).collect())), entry("Count", PdfObject::Int(page_refs.len() as i64)), entry("Resources", root_resources)]));
    let catalog = lowering.lower_catalog(pages_ref, true)?;
    lowering.set(catalog_ref, PdfObject::Dict(catalog));
    if let Some(info_ref) = info_ref {
        let info = lower_info(&snapshot.info);
        lowering.set(info_ref, PdfObject::Dict(info));
    }
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok(LoweredDocument { objects: lowering.objects, root: catalog_ref, info: info_ref, refs: lowering.refs, annotation_refs: lowering.annotation_refs, field_refs: lowering.field_refs, widget_parents: lowering.widget_parents })
}
//#endregion 🔖️Lowering
