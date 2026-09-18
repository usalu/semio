//! ⬆️ Lower: the typed document lanes → a COS object graph (the inverse of `⬇️lift`). Every
//! document collection becomes indirect objects; every page and form regenerates its
//! `/Resources` from the names its operators use; the catalog, info dictionary and trailer
//! entries are assembled last. Object numbers are allocated densely from `first_number`, so a
//! reconciling write can keep a retained graph's own numbers untouched.

use super::colour::{entry, lower_colour_space, lower_ext_g_state, lower_function, lower_shading, push_opt, raw_stream, stream};
use super::content::{content_references, print_content, FontTable};
use super::fonts::{base_encoding_name, cmap, encode_text_string, FontCodec};
use super::images::lower_image;
use super::lexer::{PResult, PdfEngineError};
use super::xref::ObjectSink;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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
pub use super::lift::Category;

/// 🧱 One lowering pass: the objects produced so far and the references each typed id maps to.
pub struct Lowering<'a> {
    snapshot: &'a PdfSnapshot,
    options: LowerOptions,
    pub objects: Vec<PdfIndirectObject>,
    next: u32,
    pub refs: HashMap<(Category, String), ObjRef>,
    pub annotation_refs: Vec<Vec<ObjRef>>,
    pub field_refs: Vec<ObjRef>,
    pub widget_parents: HashMap<[u32; 2], ObjRef>,
    codecs: HashMap<String, FontCodec>,
    used_glyphs: HashMap<String, BTreeSet<u16>>,
}

impl ObjectSink for Lowering<'_> {
    fn add(&mut self, value: PdfObject) -> ObjRef {
        let reference = self.reserve();
        self.objects.push(PdfIndirectObject { id: reference, value });
        reference
    }
}

impl FontTable for Lowering<'_> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(snapshot: &'a PdfSnapshot, first_number: u32, options: LowerOptions) -> Self {
        let codecs = snapshot.fonts.iter().map(|font| (font.id.clone(), FontCodec::new(font))).collect();
        Self { snapshot, options, objects: Vec::new(), next: first_number.max(1), refs: HashMap::new(), annotation_refs: Vec::new(), field_refs: Vec::new(), widget_parents: HashMap::new(), codecs, used_glyphs: HashMap::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn reserve(&mut self) -> ObjRef {
        let reference = ObjRef { num: self.next, gen: 0 };
        self.next += 1;
        reference
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn set(&mut self, reference: ObjRef, value: PdfObject) {
        match self.objects.iter_mut().find(|object| object.id == reference) {
            Some(object) => object.value = value,
            None => self.objects.push(PdfIndirectObject { id: reference, value }),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn reference(&self, category: Category, id: &str) -> Option<PdfObject> {
        self.refs.get(&(category, id.to_string())).map(|reference| PdfObject::Ref(*reference))
    }

    /// 🗜️ A content stream object for `ops`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn content_stream(&mut self, ops: &[PdfOp]) -> PResult<PdfObject> {
        let bytes = print_content(ops, self)?;
        Ok(if self.options.compress { stream(Vec::new(), bytes) } else { raw_stream(Vec::new(), bytes) })
    }

    /// 📚 The `/Resources` dictionary a content stream needs.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resources_for(&mut self, ops: &[PdfOp]) -> PdfObject {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn record_glyphs(&mut self, ops: &[PdfOp]) {
        let mut current: Option<String> = None;
        let mut stack: Vec<Option<String>> = Vec::new();
        let mut use_text = |lowering: &mut Self, font: &Option<String>, text: &PdfTextString| {
            let Some(font) = font else { return };
            let Some(codec) = lowering.codecs.get(font) else { return };
            let bytes = match text {
                PdfTextString::Codes { bytes } => bytes.clone(),
                PdfTextString::Text { text } => codec.encode(text).unwrap_or_default(),
            };
            let glyphs: Vec<u16> = codec.decode(&bytes).iter().filter_map(|glyph| glyph.glyph_id).collect();
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
                            PdfTextArrayItem::Codes { bytes } => use_text(self, &current, &PdfTextString::Codes { bytes: bytes.clone() }),
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
    let mut lowering = Lowering::new(snapshot, first_number, options);
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
    let mut lowering = Lowering::new(snapshot, first_number, options);
    lowering.refs = refs.clone();
    lowering.annotation_refs = annotation_refs;
    lowering.field_refs = field_refs;
    let value = lowering.lower_acro_form(form);
    let reference = lowering.add(PdfObject::Dict(value));
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok(Some((lowering.objects, reference)))
}

/// ⬆️ Lowers everything but the pages, reserving `expected_pages` page references (so
/// destinations and outlines resolve) — the first half of a streamed write.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_document_headless(snapshot: &PdfSnapshot, expected_pages: usize, options: LowerOptions) -> PResult<(LoweredDocument, ObjRef, Vec<ObjRef>, PdfObject)> {
    let mut headless = snapshot.clone();
    headless.pages.clear();
    let mut lowering = Lowering::new(&headless, 1, options);
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
    lowering.lower_images();
    lowering.lower_forms()?;
    lowering.lower_ext_g_states();
    lowering.lower_shadings();
    lowering.lower_patterns()?;
    lowering.lower_named_resources();
    lowering.lower_embedded_files();
    let catalog = lowering.lower_catalog(pages_ref, false)?;
    lowering.set(catalog_ref, PdfObject::Dict(catalog));
    if let Some(info_ref) = info_ref {
        let info = lowering.lower_info();
        lowering.set(info_ref, PdfObject::Dict(info));
    }
    let root_resources = lowering.all_resources();
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok((LoweredDocument { objects: lowering.objects, root: catalog_ref, info: info_ref, refs: lowering.refs, annotation_refs: Vec::new(), field_refs: lowering.field_refs, widget_parents: lowering.widget_parents }, pages_ref, page_refs, root_resources))
}

impl Lowering<'_> {
    /// 🆔 Reserves one reference per interactive-form field (depth-first) and notes which field
    /// every widget annotation belongs to.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    let mut lowering = Lowering::new(snapshot, first_number, options);
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
    lowering.lower_images();
    lowering.lower_forms()?;
    lowering.lower_ext_g_states();
    lowering.lower_shadings();
    lowering.lower_patterns()?;
    lowering.lower_named_resources();
    lowering.lower_embedded_files();
    for (index, page) in snapshot.pages.iter().enumerate() {
        lowering.lower_page(index, page, page_refs[index], pages_ref)?;
    }
    let root_resources = lowering.all_resources();
    lowering.set(pages_ref, PdfObject::Dict(vec![entry("Type", PdfObject::name("Pages")), entry("Kids", PdfObject::Array(page_refs.iter().map(|r| PdfObject::Ref(*r)).collect())), entry("Count", PdfObject::Int(page_refs.len() as i64)), entry("Resources", root_resources)]));
    let catalog = lowering.lower_catalog(pages_ref, true)?;
    lowering.set(catalog_ref, PdfObject::Dict(catalog));
    if let Some(info_ref) = info_ref {
        let info = lowering.lower_info();
        lowering.set(info_ref, PdfObject::Dict(info));
    }
    lowering.objects.sort_by_key(|object| object.id.num);
    Ok(LoweredDocument { objects: lowering.objects, root: catalog_ref, info: info_ref, refs: lowering.refs, annotation_refs: lowering.annotation_refs, field_refs: lowering.field_refs, widget_parents: lowering.widget_parents })
}
//#endregion 🔖️Lowering

//#region 🔖️Fonts
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_fonts(&mut self) -> PResult<()> {
        for font in &self.snapshot.fonts {
            let reference = self.refs[&(Category::Font, font.id.clone())];
            let dict = self.lower_font(font)?;
            self.set(reference, PdfObject::Dict(dict));
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_simple_encoding(encoding: &PdfSimpleEncoding) -> Option<PdfObject> {
        if encoding.differences.is_empty() {
            return encoding.base.map(|base| PdfObject::name(base_encoding_name(base)));
        }
        let mut dict = vec![entry("Type", PdfObject::name("Encoding"))];
        push_opt(&mut dict, "BaseEncoding", encoding.base.map(|base| PdfObject::name(base_encoding_name(base))));
        let mut differences = Vec::new();
        let mut expected: Option<u32> = None;
        for difference in &encoding.differences {
            if expected != Some(difference.code) {
                differences.push(PdfObject::Int(difference.code as i64));
            }
            differences.push(PdfObject::name(&difference.glyph));
            expected = Some(difference.code + 1);
        }
        dict.push(entry("Differences", PdfObject::Array(differences)));
        Some(PdfObject::Dict(dict))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_program(&mut self, program: &PdfFontProgram, glyphs: Option<&BTreeSet<u16>>) -> (&'static str, PdfObject) {
        match program {
            PdfFontProgram::Type1 { data, length1, length2, length3 } => ("FontFile", PdfObject::Ref(self.add(stream(vec![entry("Length1", PdfObject::Int(*length1 as i64)), entry("Length2", PdfObject::Int(*length2 as i64)), entry("Length3", PdfObject::Int(*length3 as i64))], data.clone())))),
            PdfFontProgram::TrueType { data } => {
                let data = match (glyphs, self.options.subset_fonts) {
                    (Some(glyphs), true) => super::fonts::TrueTypeFont::parse(data).map(|font| font.subset(glyphs)).unwrap_or_else(|_| data.clone()),
                    _ => data.clone(),
                };
                let length = data.len() as i64;
                ("FontFile2", PdfObject::Ref(self.add(stream(vec![entry("Length1", PdfObject::Int(length))], data))))
            }
            PdfFontProgram::Cff { data } => ("FontFile3", PdfObject::Ref(self.add(stream(vec![entry("Subtype", PdfObject::name("Type1C"))], data.clone())))),
            PdfFontProgram::CidCff { data } => ("FontFile3", PdfObject::Ref(self.add(stream(vec![entry("Subtype", PdfObject::name("CIDFontType0C"))], data.clone())))),
            PdfFontProgram::OpenType { data } => ("FontFile3", PdfObject::Ref(self.add(stream(vec![entry("Subtype", PdfObject::name("OpenType"))], data.clone())))),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_descriptor(&mut self, descriptor: &PdfFontDescriptor, program: Option<&PdfFontProgram>, glyphs: Option<&BTreeSet<u16>>) -> PdfObject {
        let mut dict = vec![
            entry("Type", PdfObject::name("FontDescriptor")),
            entry("FontName", PdfObject::name(&descriptor.font_name)),
            entry("Flags", PdfObject::Int(descriptor.flags as i64)),
            entry("FontBBox", PdfObject::numbers(&descriptor.font_bbox)),
            entry("ItalicAngle", PdfObject::number(descriptor.italic_angle)),
            entry("Ascent", PdfObject::number(descriptor.ascent)),
            entry("Descent", PdfObject::number(descriptor.descent)),
            entry("CapHeight", PdfObject::number(descriptor.cap_height)),
            entry("StemV", PdfObject::number(descriptor.stem_v)),
        ];
        push_opt(&mut dict, "StemH", descriptor.stem_h.map(PdfObject::number));
        push_opt(&mut dict, "XHeight", descriptor.x_height.map(PdfObject::number));
        push_opt(&mut dict, "Leading", descriptor.leading.map(PdfObject::number));
        push_opt(&mut dict, "AvgWidth", descriptor.avg_width.map(PdfObject::number));
        push_opt(&mut dict, "MaxWidth", descriptor.max_width.map(PdfObject::number));
        push_opt(&mut dict, "MissingWidth", descriptor.missing_width.map(PdfObject::number));
        push_opt(&mut dict, "FontFamily", descriptor.font_family.as_ref().map(|v| PdfObject::Str(encode_text_string(v))));
        push_opt(&mut dict, "FontStretch", descriptor.font_stretch.as_ref().map(PdfObject::name));
        push_opt(&mut dict, "FontWeight", descriptor.font_weight.map(PdfObject::number));
        push_opt(&mut dict, "CharSet", descriptor.char_set.as_ref().map(|v| PdfObject::Str(encode_text_string(v))));
        if let Some(program) = program {
            let (key, value) = self.lower_program(program, glyphs);
            dict.push(entry(key, value));
        }
        dict.extend(descriptor.extra.iter().cloned());
        PdfObject::Ref(self.add(PdfObject::Dict(dict)))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_font(&mut self, font: &PdfFont) -> PResult<Vec<PdfDictEntry>> {
        let glyphs = self.used_glyphs.get(&font.id).cloned();
        let mut dict = vec![entry("Type", PdfObject::name("Font"))];
        match &font.kind {
            PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program } | PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program } => {
                dict.push(entry("Subtype", PdfObject::name(if matches!(font.kind, PdfFontKind::TrueType { .. }) { "TrueType" } else { "Type1" })));
                dict.push(entry("BaseFont", PdfObject::name(base_font)));
                if !widths.is_empty() {
                    dict.push(entry("FirstChar", PdfObject::Int(*first_char as i64)));
                    dict.push(entry("LastChar", PdfObject::Int(*first_char as i64 + widths.len() as i64 - 1)));
                    dict.push(entry("Widths", PdfObject::numbers(widths)));
                }
                push_opt(&mut dict, "Encoding", Self::lower_simple_encoding(encoding));
                if let Some(descriptor) = descriptor {
                    let value = self.lower_descriptor(descriptor, program.as_ref(), glyphs.as_ref());
                    dict.push(entry("FontDescriptor", value));
                }
            }
            PdfFontKind::Type3 { font_matrix, font_bbox, encoding, first_char, widths, char_procs, descriptor } => {
                dict.push(entry("Subtype", PdfObject::name("Type3")));
                dict.push(entry("FontBBox", PdfObject::numbers(font_bbox)));
                dict.push(entry("FontMatrix", PdfObject::numbers(font_matrix)));
                let mut procs = Vec::new();
                let mut all_ops: Vec<PdfOp> = Vec::new();
                for proc_entry in char_procs {
                    let stream_object = self.content_stream(&proc_entry.content)?;
                    let reference = self.add(stream_object);
                    procs.push(PdfDictEntry::new(&proc_entry.name, PdfObject::Ref(reference)));
                    all_ops.extend(proc_entry.content.iter().cloned());
                }
                dict.push(entry("CharProcs", PdfObject::Dict(procs)));
                push_opt(&mut dict, "Encoding", Self::lower_simple_encoding(encoding).or(Some(PdfObject::Dict(vec![entry("Type", PdfObject::name("Encoding")), entry("Differences", PdfObject::Array(Vec::new()))]))));
                dict.push(entry("FirstChar", PdfObject::Int(*first_char as i64)));
                dict.push(entry("LastChar", PdfObject::Int(*first_char as i64 + widths.len().max(1) as i64 - 1)));
                dict.push(entry("Widths", PdfObject::numbers(widths)));
                let resources = self.resources_for(&all_ops);
                dict.push(entry("Resources", resources));
                if let Some(descriptor) = descriptor {
                    let value = self.lower_descriptor(descriptor, None, None);
                    dict.push(entry("FontDescriptor", value));
                }
            }
            PdfFontKind::Type0 { base_font, cmap: encoding, descendant } => {
                dict.push(entry("Subtype", PdfObject::name("Type0")));
                dict.push(entry("BaseFont", PdfObject::name(base_font)));
                let encoding_value = match encoding {
                    PdfCMap::Predefined { name } => PdfObject::name(name),
                    PdfCMap::Embedded { cmap: embedded } => {
                        let mut cmap_dict = vec![entry("Type", PdfObject::name("CMap")), entry("CMapName", PdfObject::name(&embedded.name)), entry("CIDSystemInfo", cid_system_info(&descendant.system_info))];
                        if embedded.vertical {
                            cmap_dict.push(entry("WMode", PdfObject::Int(1)));
                        }
                        push_opt(&mut cmap_dict, "UseCMap", embedded.use_cmap.as_ref().map(PdfObject::name));
                        PdfObject::Ref(self.add(stream(cmap_dict, cmap::print_cmap(embedded))))
                    }
                };
                dict.push(entry("Encoding", encoding_value));
                let descendant_ref = self.lower_cid_font(descendant, glyphs.as_ref());
                dict.push(entry("DescendantFonts", PdfObject::Array(vec![PdfObject::Ref(descendant_ref)])));
            }
        }
        if let Some(to_unicode) = &font.to_unicode {
            let reference = self.add(stream(Vec::new(), cmap::print_to_unicode(to_unicode)));
            dict.push(entry("ToUnicode", PdfObject::Ref(reference)));
        }
        dict.extend(font.extra.iter().cloned());
        Ok(dict)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_cid_font(&mut self, cid: &PdfCidFont, glyphs: Option<&BTreeSet<u16>>) -> ObjRef {
        let mut dict = vec![entry("Type", PdfObject::name("Font")), entry("Subtype", PdfObject::name(if cid.true_type { "CIDFontType2" } else { "CIDFontType0" })), entry("BaseFont", PdfObject::name(&cid.base_font)), entry("CIDSystemInfo", cid_system_info(&cid.system_info))];
        let descriptor = self.lower_descriptor(&cid.descriptor, cid.program.as_ref(), glyphs);
        dict.push(entry("FontDescriptor", descriptor));
        if cid.default_width != 1000.0 {
            dict.push(entry("DW", PdfObject::number(cid.default_width)));
        }
        if !cid.widths.is_empty() {
            let mut w = Vec::new();
            for run in &cid.widths {
                w.push(PdfObject::Int(run.start_cid as i64));
                w.push(PdfObject::numbers(&run.widths));
            }
            dict.push(entry("W", PdfObject::Array(w)));
        }
        push_opt(&mut dict, "DW2", cid.default_vertical.as_ref().map(|v| PdfObject::numbers(v)));
        if !cid.vertical_metrics.is_empty() {
            let mut w2 = Vec::new();
            for run in &cid.vertical_metrics {
                w2.push(PdfObject::Int(run.start_cid as i64));
                w2.push(PdfObject::Array(run.metrics.iter().flat_map(|m| m.iter().map(|v| PdfObject::number(*v))).collect()));
            }
            dict.push(entry("W2", PdfObject::Array(w2)));
        }
        match &cid.cid_to_gid {
            Some(PdfCidToGid::Identity) => dict.push(entry("CIDToGIDMap", PdfObject::name("Identity"))),
            Some(PdfCidToGid::Map { data }) => {
                let reference = self.add(stream(Vec::new(), data.clone()));
                dict.push(entry("CIDToGIDMap", PdfObject::Ref(reference)));
            }
            None => {}
        }
        dict.extend(cid.extra.iter().cloned());
        self.add(PdfObject::Dict(dict))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cid_system_info(info: &PdfCidSystemInfo) -> PdfObject {
    PdfObject::Dict(vec![entry("Registry", PdfObject::Str(info.registry.as_bytes().to_vec())), entry("Ordering", PdfObject::Str(info.ordering.as_bytes().to_vec())), entry("Supplement", PdfObject::Int(info.supplement as i64))])
}
//#endregion 🔖️Fonts

//#region 🔖️Resources
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_optional_content_groups(&mut self) {
        let Some(optional_content) = &self.snapshot.optional_content else { return };
        for group in &optional_content.groups {
            let reference = self.refs[&(Category::OptionalContent, group.id.clone())];
            let mut dict = vec![entry("Type", PdfObject::name("OCG")), entry("Name", PdfObject::Str(encode_text_string(&group.name)))];
            if !group.intent.is_empty() {
                dict.push(entry("Intent", if group.intent.len() == 1 { PdfObject::name(&group.intent[0]) } else { PdfObject::Array(group.intent.iter().map(PdfObject::name).collect()) }));
            }
            if !group.usage.is_empty() {
                dict.push(entry("Usage", PdfObject::Dict(group.usage.clone())));
            }
            self.set(reference, PdfObject::Dict(dict));
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_images(&mut self) {
        for image in &self.snapshot.images {
            let reference = self.refs[&(Category::XObject, image.id.clone())];
            let refs = self.refs.clone();
            let mut ref_of = |id: &str| refs.get(&(Category::XObject, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut oc_ref_of = |id: &str| refs.get(&(Category::OptionalContent, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut object = lower_image(image, self, &mut ref_of, &mut oc_ref_of);
            if !self.options.compress {
                if let PdfObject::Stream { filters, .. } = &mut object {
                    filters.retain(|filter| filter.is_image_codec());
                }
            }
            self.set(reference, object);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_forms(&mut self) -> PResult<()> {
        for form in &self.snapshot.forms {
            let reference = self.refs[&(Category::XObject, form.id.clone())];
            let mut dict = vec![entry("Type", PdfObject::name("XObject")), entry("Subtype", PdfObject::name("Form")), entry("FormType", PdfObject::Int(1)), entry("BBox", PdfObject::numbers(&form.bbox))];
            if form.matrix != PDF_IDENTITY_MATRIX {
                dict.push(entry("Matrix", PdfObject::numbers(&form.matrix)));
            }
            let resources = self.resources_for(&form.content);
            dict.push(entry("Resources", resources));
            push_opt(&mut dict, "Group", form.group.as_ref().map(|group| self.lower_group(group)));
            push_opt(&mut dict, "OC", form.optional_content.as_deref().and_then(|id| self.reference(Category::OptionalContent, id)));
            push_opt(&mut dict, "StructParent", form.struct_parent.map(|v| PdfObject::Int(v as i64)));
            dict.extend(form.extra.iter().cloned());
            let mut object = self.content_stream(&form.content)?;
            if let PdfObject::Stream { dict: slot, .. } = &mut object {
                *slot = dict;
            }
            self.set(reference, object);
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_group(&mut self, group: &PdfTransparencyGroup) -> PdfObject {
        let mut dict = vec![entry("Type", PdfObject::name("Group")), entry("S", PdfObject::name("Transparency"))];
        push_opt(&mut dict, "CS", group.color_space.as_ref().map(|cs| lower_colour_space(cs, self)));
        if group.isolated {
            dict.push(entry("I", PdfObject::Bool(true)));
        }
        if group.knockout {
            dict.push(entry("K", PdfObject::Bool(true)));
        }
        PdfObject::Dict(dict)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_ext_g_states(&mut self) {
        for state in &self.snapshot.ext_g_states {
            let reference = self.refs[&(Category::ExtGState, state.id.clone())];
            let refs = self.refs.clone();
            let mut form_ref_of = |id: &str| refs.get(&(Category::XObject, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut font_ref_of = |id: &str| refs.get(&(Category::Font, id.to_string())).map(|r| PdfObject::Ref(*r));
            let object = lower_ext_g_state(state, self, &mut form_ref_of, &mut font_ref_of);
            self.set(reference, object);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_shadings(&mut self) {
        for shading in &self.snapshot.shadings {
            let reference = self.refs[&(Category::Shading, shading.id.clone())];
            let lowered = lower_shading(shading, self);
            if let PdfObject::Ref(temporary) = lowered {
                let value = self.objects.iter().position(|object| object.id == temporary).map(|index| self.objects.remove(index).value).unwrap_or(PdfObject::Null);
                self.set(reference, value);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_patterns(&mut self) -> PResult<()> {
        for pattern in &self.snapshot.patterns {
            let reference = self.refs[&(Category::Pattern, pattern.id.clone())];
            let object = match &pattern.kind {
                PdfPatternKind::Shading { shading, ext_g_state } => {
                    let mut dict = vec![entry("Type", PdfObject::name("Pattern")), entry("PatternType", PdfObject::Int(2))];
                    if pattern.matrix != PDF_IDENTITY_MATRIX {
                        dict.push(entry("Matrix", PdfObject::numbers(&pattern.matrix)));
                    }
                    push_opt(&mut dict, "Shading", self.reference(Category::Shading, shading));
                    push_opt(&mut dict, "ExtGState", ext_g_state.as_deref().and_then(|id| self.reference(Category::ExtGState, id)));
                    dict.extend(pattern.extra.iter().cloned());
                    PdfObject::Dict(dict)
                }
                PdfPatternKind::Tiling { paint_type, tiling_type, bbox, x_step, y_step, content } => {
                    let mut dict = vec![entry("Type", PdfObject::name("Pattern")), entry("PatternType", PdfObject::Int(1)), entry("PaintType", PdfObject::Int(*paint_type as i64)), entry("TilingType", PdfObject::Int(*tiling_type as i64)), entry("BBox", PdfObject::numbers(bbox)), entry("XStep", PdfObject::number(*x_step)), entry("YStep", PdfObject::number(*y_step))];
                    if pattern.matrix != PDF_IDENTITY_MATRIX {
                        dict.push(entry("Matrix", PdfObject::numbers(&pattern.matrix)));
                    }
                    let resources = self.resources_for(content);
                    dict.push(entry("Resources", resources));
                    dict.extend(pattern.extra.iter().cloned());
                    let mut object = self.content_stream(content)?;
                    if let PdfObject::Stream { dict: slot, .. } = &mut object {
                        *slot = dict;
                    }
                    object
                }
            };
            self.set(reference, object);
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_named_resources(&mut self) {
        for space in &self.snapshot.color_spaces {
            let reference = self.refs[&(Category::ColorSpace, space.name.clone())];
            let value = lower_colour_space(&space.color_space, self);
            self.set(reference, value);
        }
        for properties in &self.snapshot.properties {
            let reference = self.refs[&(Category::Properties, properties.name.clone())];
            let value = match properties.entries.as_slice() {
                [PdfDictEntry { key, value: PdfObject::Name(group) }] if key == "OCG" => self.reference(Category::OptionalContent, group).unwrap_or(PdfObject::Dict(properties.entries.clone())),
                entries => PdfObject::Dict(entries.to_vec()),
            };
            self.set(reference, value);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_embedded_files(&mut self) {
        for file in &self.snapshot.embedded_files {
            let reference = self.refs[&(Category::EmbeddedFile, file.id.clone())];
            let mut params = vec![entry("Size", PdfObject::Int(file.data.len() as i64))];
            push_opt(&mut params, "CreationDate", file.creation_date.as_ref().map(|d| PdfObject::Str(d.to_string().into_bytes())));
            push_opt(&mut params, "ModDate", file.modification_date.as_ref().map(|d| PdfObject::Str(d.to_string().into_bytes())));
            let mut stream_dict = vec![entry("Type", PdfObject::name("EmbeddedFile"))];
            push_opt(&mut stream_dict, "Subtype", file.mime_type.as_ref().map(PdfObject::name));
            stream_dict.push(entry("Params", PdfObject::Dict(params)));
            let stream_ref = self.add(stream(stream_dict, file.data.clone()));
            let mut dict = vec![entry("Type", PdfObject::name("Filespec")), entry("F", PdfObject::Str(encode_text_string(&file.file_name))), entry("UF", PdfObject::Str(encode_text_string(&file.file_name))), entry("EF", PdfObject::Dict(vec![entry("F", PdfObject::Ref(stream_ref)), entry("UF", PdfObject::Ref(stream_ref))]))];
            push_opt(&mut dict, "Desc", file.description.as_ref().map(|d| PdfObject::Str(encode_text_string(d))));
            push_opt(&mut dict, "AFRelationship", file.relationship.as_ref().map(PdfObject::name));
            self.set(reference, PdfObject::Dict(dict));
        }
    }
}
//#endregion 🔖️Resources

//#region 🔖️Pages
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_page(&mut self, index: usize, page: &PdfPage, reference: ObjRef, parent: ObjRef) -> PResult<()> {
        let mut dict = vec![entry("Type", PdfObject::name("Page")), entry("Parent", PdfObject::Ref(parent)), entry("MediaBox", PdfObject::numbers(&page.media_box))];
        push_opt(&mut dict, "CropBox", page.crop_box.as_ref().map(|b| PdfObject::numbers(b)));
        push_opt(&mut dict, "BleedBox", page.bleed_box.as_ref().map(|b| PdfObject::numbers(b)));
        push_opt(&mut dict, "TrimBox", page.trim_box.as_ref().map(|b| PdfObject::numbers(b)));
        push_opt(&mut dict, "ArtBox", page.art_box.as_ref().map(|b| PdfObject::numbers(b)));
        if page.rotate != 0 {
            dict.push(entry("Rotate", PdfObject::Int(page.rotate as i64)));
        }
        push_opt(&mut dict, "UserUnit", page.user_unit.map(PdfObject::number));
        let resources = self.resources_for(&page.content);
        dict.push(entry("Resources", resources));
        let content = self.content_stream(&page.content)?;
        let content_ref = self.add(content);
        dict.push(entry("Contents", PdfObject::Ref(content_ref)));
        push_opt(&mut dict, "Group", page.group.as_ref().map(|group| self.lower_group(group)));
        push_opt(&mut dict, "Thumb", page.thumbnail.as_deref().and_then(|id| self.reference(Category::XObject, id)));
        push_opt(&mut dict, "StructParents", page.struct_parents.map(|v| PdfObject::Int(v as i64)));
        push_opt(&mut dict, "Trans", page.transition.as_ref().map(|t| PdfObject::Dict(t.clone())));
        push_opt(&mut dict, "Dur", page.duration.map(PdfObject::number));
        if let Some(metadata) = &page.metadata {
            let reference = self.add(raw_stream(vec![entry("Type", PdfObject::name("Metadata")), entry("Subtype", PdfObject::name("XML"))], metadata.as_bytes().to_vec()));
            dict.push(entry("Metadata", PdfObject::Ref(reference)));
        }
        if !page.additional_actions.is_empty() {
            dict.push(entry("AA", PdfObject::Dict(page.additional_actions.clone())));
        }
        if !page.annotations.is_empty() {
            let refs = self.annotation_refs[index].clone();
            for (annotation_index, annotation) in page.annotations.iter().enumerate() {
                let object = self.lower_annotation(annotation, index, annotation_index, reference);
                self.set(refs[annotation_index], PdfObject::Dict(object));
            }
            dict.push(entry("Annots", PdfObject::Array(refs.iter().map(|r| PdfObject::Ref(*r)).collect())));
        }
        dict.extend(page.extra.iter().cloned());
        self.set(reference, PdfObject::Dict(dict));
        Ok(())
    }
}
//#endregion 🔖️Pages

//#region 🔖️Navigation
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_destination(&self, destination: &PdfDestination) -> PdfObject {
        let fit_items = |fit: &PdfDestinationFit| -> Vec<PdfObject> {
            let opt = |value: Option<f64>| value.map(PdfObject::number).unwrap_or(PdfObject::Null);
            match fit {
                PdfDestinationFit::Xyz { left, top, zoom } => vec![PdfObject::name("XYZ"), opt(*left), opt(*top), opt(*zoom)],
                PdfDestinationFit::Fit => vec![PdfObject::name("Fit")],
                PdfDestinationFit::FitHorizontal { top } => vec![PdfObject::name("FitH"), opt(*top)],
                PdfDestinationFit::FitVertical { left } => vec![PdfObject::name("FitV"), opt(*left)],
                PdfDestinationFit::FitRectangle { rect } => vec![PdfObject::name("FitR"), PdfObject::number(rect[0]), PdfObject::number(rect[1]), PdfObject::number(rect[2]), PdfObject::number(rect[3])],
                PdfDestinationFit::FitBoundingBox => vec![PdfObject::name("FitB")],
                PdfDestinationFit::FitBoundingBoxHorizontal { top } => vec![PdfObject::name("FitBH"), opt(*top)],
                PdfDestinationFit::FitBoundingBoxVertical { left } => vec![PdfObject::name("FitBV"), opt(*left)],
            }
        };
        match destination {
            PdfDestination::Page { page, fit } => {
                let mut items = vec![self.reference(Category::Page, &page.to_string()).unwrap_or(PdfObject::Null)];
                items.extend(fit_items(fit));
                PdfObject::Array(items)
            }
            PdfDestination::RemotePage { page, fit } => {
                let mut items = vec![PdfObject::Int(*page as i64)];
                items.extend(fit_items(fit));
                PdfObject::Array(items)
            }
            PdfDestination::Named { name } => PdfObject::Str(encode_text_string(name)),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_file_specification(&self, file: &PdfFileSpecification) -> PdfObject {
        match file {
            PdfFileSpecification::Path { path } => PdfObject::Str(encode_text_string(path)),
            PdfFileSpecification::Embedded { file } => self.reference(Category::EmbeddedFile, file).unwrap_or(PdfObject::Null),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_action(&mut self, action: &PdfAction) -> PdfObject {
        let mut dict = vec![entry("Type", PdfObject::name("Action"))];
        let text = |value: &str| PdfObject::Str(encode_text_string(value));
        let names = |values: &[String]| PdfObject::Array(values.iter().map(|value| text(value)).collect());
        match &action.kind {
            PdfActionKind::GoTo { destination } => {
                dict.push(entry("S", PdfObject::name("GoTo")));
                dict.push(entry("D", self.lower_destination(destination)));
            }
            PdfActionKind::GoToRemote { file, destination, new_window } => {
                dict.push(entry("S", PdfObject::name("GoToR")));
                dict.push(entry("F", self.lower_file_specification(file)));
                dict.push(entry("D", self.lower_destination(destination)));
                push_opt(&mut dict, "NewWindow", new_window.map(PdfObject::Bool));
            }
            PdfActionKind::GoToEmbedded { destination, new_window } => {
                dict.push(entry("S", PdfObject::name("GoToE")));
                dict.push(entry("D", self.lower_destination(destination)));
                push_opt(&mut dict, "NewWindow", new_window.map(PdfObject::Bool));
            }
            PdfActionKind::Launch { file, new_window } => {
                dict.push(entry("S", PdfObject::name("Launch")));
                dict.push(entry("F", self.lower_file_specification(file)));
                push_opt(&mut dict, "NewWindow", new_window.map(PdfObject::Bool));
            }
            PdfActionKind::Thread { file, thread } => {
                dict.push(entry("S", PdfObject::name("Thread")));
                push_opt(&mut dict, "F", file.as_ref().map(|f| self.lower_file_specification(f)));
                dict.push(entry("D", PdfObject::Int(*thread as i64)));
            }
            PdfActionKind::Uri { uri, is_map } => {
                dict.push(entry("S", PdfObject::name("URI")));
                dict.push(entry("URI", PdfObject::Str(uri.as_bytes().to_vec())));
                if *is_map {
                    dict.push(entry("IsMap", PdfObject::Bool(true)));
                }
            }
            PdfActionKind::Sound { volume, synchronous, repeat, mix, .. } => {
                dict.push(entry("S", PdfObject::name("Sound")));
                push_opt(&mut dict, "Volume", volume.map(PdfObject::number));
                if *synchronous {
                    dict.push(entry("Synchronous", PdfObject::Bool(true)));
                }
                if *repeat {
                    dict.push(entry("Repeat", PdfObject::Bool(true)));
                }
                if *mix {
                    dict.push(entry("Mix", PdfObject::Bool(true)));
                }
            }
            PdfActionKind::Movie { annotation, operation } => {
                dict.push(entry("S", PdfObject::name("Movie")));
                push_opt(&mut dict, "T", annotation.as_ref().map(|t| text(t)));
                push_opt(&mut dict, "Operation", operation.as_ref().map(PdfObject::name));
            }
            PdfActionKind::Hide { annotations, hide } => {
                dict.push(entry("S", PdfObject::name("Hide")));
                dict.push(entry("T", names(annotations)));
                if !hide {
                    dict.push(entry("H", PdfObject::Bool(false)));
                }
            }
            PdfActionKind::Named { name } => {
                dict.push(entry("S", PdfObject::name("Named")));
                dict.push(entry("N", PdfObject::name(name)));
            }
            PdfActionKind::SubmitForm { url, fields, flags } => {
                dict.push(entry("S", PdfObject::name("SubmitForm")));
                dict.push(entry("F", PdfObject::Dict(vec![entry("FS", PdfObject::name("URL")), entry("F", PdfObject::Str(url.as_bytes().to_vec()))])));
                if !fields.is_empty() {
                    dict.push(entry("Fields", names(fields)));
                }
                if *flags != 0 {
                    dict.push(entry("Flags", PdfObject::Int(*flags as i64)));
                }
            }
            PdfActionKind::ResetForm { fields, flags } => {
                dict.push(entry("S", PdfObject::name("ResetForm")));
                if !fields.is_empty() {
                    dict.push(entry("Fields", names(fields)));
                }
                if *flags != 0 {
                    dict.push(entry("Flags", PdfObject::Int(*flags as i64)));
                }
            }
            PdfActionKind::ImportData { file } => {
                dict.push(entry("S", PdfObject::name("ImportData")));
                dict.push(entry("F", self.lower_file_specification(file)));
            }
            PdfActionKind::JavaScript { script } => {
                dict.push(entry("S", PdfObject::name("JavaScript")));
                dict.push(entry("JS", text(script)));
            }
            PdfActionKind::SetOptionalContentState { states, preserve_radio_buttons } => {
                dict.push(entry("S", PdfObject::name("SetOCGState")));
                dict.extend(states.iter().cloned());
                if !preserve_radio_buttons {
                    dict.push(entry("PreserveRB", PdfObject::Bool(false)));
                }
            }
            PdfActionKind::Rendition { entries } => {
                dict.push(entry("S", PdfObject::name("Rendition")));
                dict.extend(entries.iter().cloned());
            }
            PdfActionKind::Transition { entries } => {
                dict.push(entry("S", PdfObject::name("Trans")));
                dict.extend(entries.iter().cloned());
            }
            PdfActionKind::GoTo3dView { entries } => {
                dict.push(entry("S", PdfObject::name("GoTo3DView")));
                dict.extend(entries.iter().cloned());
            }
            PdfActionKind::Unknown { subtype, entries } => {
                dict.push(entry("S", PdfObject::name(subtype)));
                dict.extend(entries.iter().cloned());
            }
        }
        if !action.next.is_empty() {
            let next: Vec<PdfObject> = action.next.iter().map(|next| self.lower_action(next)).collect();
            dict.push(entry("Next", if next.len() == 1 { next.into_iter().next().expect("one") } else { PdfObject::Array(next) }));
        }
        PdfObject::Dict(dict)
    }

    /// 📑 Writes an outline level as a doubly linked list and returns (first, last, count).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_outline_items(&mut self, items: &[PdfOutlineItem], parent: ObjRef) -> Option<(ObjRef, ObjRef, i64)> {
        if items.is_empty() {
            return None;
        }
        let refs: Vec<ObjRef> = items.iter().map(|_| self.reserve()).collect();
        let mut total = 0i64;
        for (index, item) in items.iter().enumerate() {
            let mut dict = vec![entry("Title", PdfObject::Str(encode_text_string(&item.title))), entry("Parent", PdfObject::Ref(parent))];
            if index > 0 {
                dict.push(entry("Prev", PdfObject::Ref(refs[index - 1])));
            }
            if index + 1 < refs.len() {
                dict.push(entry("Next", PdfObject::Ref(refs[index + 1])));
            }
            let mut descendants = 0i64;
            if let Some((first, last, count)) = self.lower_outline_items(&item.children, refs[index]) {
                dict.push(entry("First", PdfObject::Ref(first)));
                dict.push(entry("Last", PdfObject::Ref(last)));
                descendants = count;
                dict.push(entry("Count", PdfObject::Int(if item.open { count } else { -count })));
            }
            push_opt(&mut dict, "Dest", item.destination.as_ref().map(|d| self.lower_destination(d)));
            push_opt(&mut dict, "A", item.action.as_ref().map(|a| self.lower_action(a)));
            push_opt(&mut dict, "C", item.color.as_ref().map(|c| PdfObject::numbers(c)));
            let flags = (item.italic as i64) | ((item.bold as i64) << 1);
            if flags != 0 {
                dict.push(entry("F", PdfObject::Int(flags)));
            }
            dict.extend(item.extra.iter().cloned());
            self.set(refs[index], PdfObject::Dict(dict));
            total += 1 + if item.open { descendants } else { 0 };
        }
        Some((refs[0], refs[refs.len() - 1], total))
    }

    /// 🌳 A flat name tree (§7.9.6) — one node, sorted keys.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn name_tree(&mut self, pairs: Vec<(Vec<u8>, PdfObject)>) -> PdfObject {
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut names = Vec::new();
        for (key, value) in pairs {
            names.push(PdfObject::Str(key));
            names.push(value);
        }
        PdfObject::Ref(self.add(PdfObject::Dict(vec![entry("Names", PdfObject::Array(names))])))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_annotation(&mut self, annotation: &PdfAnnotation, page_index: usize, annotation_index: usize, page_ref: ObjRef) -> Vec<PdfDictEntry> {
        let text = |value: &str| PdfObject::Str(encode_text_string(value));
        let annotation_ref = |lowering: &Self, index: Option<usize>| index.and_then(|index| lowering.annotation_refs.get(page_index).and_then(|refs| refs.get(index))).map(|r| PdfObject::Ref(*r));
        let mut dict = vec![entry("Type", PdfObject::name("Annot"))];
        let (subtype, mut specific): (&str, Vec<PdfDictEntry>) = match &annotation.kind {
            PdfAnnotationKind::Text { open, icon, state, state_model } => {
                let mut d = Vec::new();
                if *open {
                    d.push(entry("Open", PdfObject::Bool(true)));
                }
                push_opt(&mut d, "Name", icon.as_ref().map(PdfObject::name));
                push_opt(&mut d, "State", state.as_ref().map(|s| text(s)));
                push_opt(&mut d, "StateModel", state_model.as_ref().map(|s| text(s)));
                ("Text", d)
            }
            PdfAnnotationKind::Link { action, destination, highlight, quad_points } => {
                let mut d = Vec::new();
                push_opt(&mut d, "A", action.as_ref().map(|a| self.lower_action(a)));
                push_opt(&mut d, "Dest", destination.as_ref().map(|dest| self.lower_destination(dest)));
                push_opt(&mut d, "H", highlight.as_ref().map(PdfObject::name));
                if !quad_points.is_empty() {
                    d.push(entry("QuadPoints", PdfObject::numbers(quad_points)));
                }
                ("Link", d)
            }
            PdfAnnotationKind::FreeText { default_appearance, quadding, callout, line_ending, rich_text } => {
                let mut d = vec![entry("DA", text(default_appearance))];
                if *quadding != 0 {
                    d.push(entry("Q", PdfObject::Int(*quadding as i64)));
                }
                push_opt(&mut d, "CL", callout.as_ref().map(|c| PdfObject::numbers(c)));
                push_opt(&mut d, "LE", line_ending.as_ref().map(PdfObject::name));
                push_opt(&mut d, "RC", rich_text.as_ref().map(|r| text(r)));
                ("FreeText", d)
            }
            PdfAnnotationKind::Line { points, line_endings, interior_color, leader_length, caption } => {
                let mut d = vec![entry("L", PdfObject::numbers(points))];
                push_opt(&mut d, "LE", line_endings.as_ref().map(|le| PdfObject::Array(vec![PdfObject::name(&le[0]), PdfObject::name(&le[1])])));
                push_opt(&mut d, "IC", interior_color.as_ref().map(|c| PdfObject::numbers(c)));
                push_opt(&mut d, "LL", leader_length.map(PdfObject::number));
                if *caption {
                    d.push(entry("Cap", PdfObject::Bool(true)));
                }
                ("Line", d)
            }
            PdfAnnotationKind::Square { interior_color, rect_differences } | PdfAnnotationKind::Circle { interior_color, rect_differences } => {
                let mut d = Vec::new();
                push_opt(&mut d, "IC", interior_color.as_ref().map(|c| PdfObject::numbers(c)));
                push_opt(&mut d, "RD", rect_differences.as_ref().map(|r| PdfObject::numbers(r)));
                (if matches!(annotation.kind, PdfAnnotationKind::Square { .. }) { "Square" } else { "Circle" }, d)
            }
            PdfAnnotationKind::Polygon { vertices, interior_color } => {
                let mut d = vec![entry("Vertices", PdfObject::numbers(vertices))];
                push_opt(&mut d, "IC", interior_color.as_ref().map(|c| PdfObject::numbers(c)));
                ("Polygon", d)
            }
            PdfAnnotationKind::PolyLine { vertices, line_endings, interior_color } => {
                let mut d = vec![entry("Vertices", PdfObject::numbers(vertices))];
                push_opt(&mut d, "LE", line_endings.as_ref().map(|le| PdfObject::Array(vec![PdfObject::name(&le[0]), PdfObject::name(&le[1])])));
                push_opt(&mut d, "IC", interior_color.as_ref().map(|c| PdfObject::numbers(c)));
                ("PolyLine", d)
            }
            PdfAnnotationKind::Highlight { quad_points } => ("Highlight", vec![entry("QuadPoints", PdfObject::numbers(quad_points))]),
            PdfAnnotationKind::Underline { quad_points } => ("Underline", vec![entry("QuadPoints", PdfObject::numbers(quad_points))]),
            PdfAnnotationKind::Squiggly { quad_points } => ("Squiggly", vec![entry("QuadPoints", PdfObject::numbers(quad_points))]),
            PdfAnnotationKind::StrikeOut { quad_points } => ("StrikeOut", vec![entry("QuadPoints", PdfObject::numbers(quad_points))]),
            PdfAnnotationKind::Stamp { icon } => {
                let mut d = Vec::new();
                push_opt(&mut d, "Name", icon.as_ref().map(PdfObject::name));
                ("Stamp", d)
            }
            PdfAnnotationKind::Caret { rect_differences, symbol } => {
                let mut d = Vec::new();
                push_opt(&mut d, "RD", rect_differences.as_ref().map(|r| PdfObject::numbers(r)));
                push_opt(&mut d, "Sy", symbol.as_ref().map(PdfObject::name));
                ("Caret", d)
            }
            PdfAnnotationKind::Ink { paths } => ("Ink", vec![entry("InkList", PdfObject::Array(paths.iter().map(|p| PdfObject::numbers(p)).collect()))]),
            PdfAnnotationKind::Popup { parent, open } => {
                let mut d = Vec::new();
                push_opt(&mut d, "Parent", annotation_ref(self, *parent));
                if *open {
                    d.push(entry("Open", PdfObject::Bool(true)));
                }
                ("Popup", d)
            }
            PdfAnnotationKind::FileAttachment { file, icon } => {
                let mut d = vec![entry("FS", self.lower_file_specification(file))];
                push_opt(&mut d, "Name", icon.as_ref().map(PdfObject::name));
                ("FileAttachment", d)
            }
            PdfAnnotationKind::Sound { sound, icon } => {
                let mut d = vec![entry("Sound", PdfObject::Dict(sound.clone()))];
                push_opt(&mut d, "Name", icon.as_ref().map(PdfObject::name));
                ("Sound", d)
            }
            PdfAnnotationKind::Movie { title, movie, activation } => {
                let mut d = vec![entry("Movie", PdfObject::Dict(movie.clone()))];
                push_opt(&mut d, "T", title.as_ref().map(|t| text(t)));
                push_opt(&mut d, "A", activation.as_ref().map(|a| PdfObject::Dict(a.clone())));
                ("Movie", d)
            }
            PdfAnnotationKind::Widget { highlight, characteristics, action, additional_actions, .. } => {
                let mut d = Vec::new();
                push_opt(&mut d, "H", highlight.as_ref().map(PdfObject::name));
                if !characteristics.is_empty() {
                    d.push(entry("MK", PdfObject::Dict(characteristics.clone())));
                }
                push_opt(&mut d, "A", action.as_ref().map(|a| self.lower_action(a)));
                if !additional_actions.is_empty() {
                    d.push(entry("AA", PdfObject::Dict(additional_actions.clone())));
                }
                ("Widget", d)
            }
            PdfAnnotationKind::Screen { title, characteristics, action, additional_actions } => {
                let mut d = Vec::new();
                push_opt(&mut d, "T", title.as_ref().map(|t| text(t)));
                if !characteristics.is_empty() {
                    d.push(entry("MK", PdfObject::Dict(characteristics.clone())));
                }
                push_opt(&mut d, "A", action.as_ref().map(|a| self.lower_action(a)));
                if !additional_actions.is_empty() {
                    d.push(entry("AA", PdfObject::Dict(additional_actions.clone())));
                }
                ("Screen", d)
            }
            PdfAnnotationKind::PrinterMark { mark_style, .. } => {
                let mut d = Vec::new();
                push_opt(&mut d, "MN", mark_style.as_ref().map(PdfObject::name));
                ("PrinterMark", d)
            }
            PdfAnnotationKind::TrapNet { entries } => ("TrapNet", entries.clone()),
            PdfAnnotationKind::Watermark { fixed_print } => {
                let mut d = Vec::new();
                push_opt(&mut d, "FixedPrint", fixed_print.as_ref().map(|f| PdfObject::Dict(f.clone())));
                ("Watermark", d)
            }
            PdfAnnotationKind::ThreeD { entries } => ("3D", entries.clone()),
            PdfAnnotationKind::Redact { quad_points, interior_color, overlay_text, repeat, default_appearance, quadding } => {
                let mut d = vec![entry("QuadPoints", PdfObject::numbers(quad_points))];
                push_opt(&mut d, "IC", interior_color.as_ref().map(|c| PdfObject::numbers(c)));
                push_opt(&mut d, "OverlayText", overlay_text.as_ref().map(|t| text(t)));
                if *repeat {
                    d.push(entry("Repeat", PdfObject::Bool(true)));
                }
                push_opt(&mut d, "DA", default_appearance.as_ref().map(|t| text(t)));
                if *quadding != 0 {
                    d.push(entry("Q", PdfObject::Int(*quadding as i64)));
                }
                ("Redact", d)
            }
            PdfAnnotationKind::Unknown { subtype, entries } => (subtype.as_str(), entries.clone()),
        };
        let subtype = subtype.to_string();
        dict.push(entry("Subtype", PdfObject::name(&subtype)));
        dict.push(entry("Rect", PdfObject::numbers(&annotation.rect)));
        dict.push(entry("P", PdfObject::Ref(page_ref)));
        push_opt(&mut dict, "Contents", annotation.contents.as_ref().map(|c| text(c)));
        push_opt(&mut dict, "NM", annotation.name.as_ref().map(|n| text(n)));
        push_opt(&mut dict, "M", annotation.modified.as_ref().map(|m| text(m)));
        if annotation.flags != 0 {
            dict.push(entry("F", PdfObject::Int(annotation.flags as i64)));
        }
        if let Some(border) = &annotation.border {
            match &border.radii {
                Some(radii) => {
                    let mut items = vec![PdfObject::number(radii[0]), PdfObject::number(radii[1]), PdfObject::number(border.width)];
                    if let Some(dash) = &border.dash {
                        items.push(PdfObject::numbers(dash));
                    }
                    dict.push(entry("Border", PdfObject::Array(items)));
                }
                None => {
                    let mut bs = vec![entry("Type", PdfObject::name("Border")), entry("W", PdfObject::number(border.width))];
                    push_opt(&mut bs, "S", border.style.as_ref().map(PdfObject::name));
                    push_opt(&mut bs, "D", border.dash.as_ref().map(|d| PdfObject::numbers(d)));
                    dict.push(entry("BS", PdfObject::Dict(bs)));
                }
            }
        }
        if !annotation.color.is_empty() {
            dict.push(entry("C", PdfObject::numbers(&annotation.color)));
        }
        if let Some(appearance) = &annotation.appearance {
            let mut ap = Vec::new();
            let lower_entry = |lowering: &Self, ap_entry: &PdfAppearanceEntry| -> Option<PdfObject> {
                match ap_entry {
                    PdfAppearanceEntry::Single { form } => lowering.reference(Category::XObject, form),
                    PdfAppearanceEntry::States { states } => Some(PdfObject::Dict(states.iter().filter_map(|state| lowering.reference(Category::XObject, &state.form).map(|r| PdfDictEntry::new(&state.state, r))).collect())),
                }
            };
            push_opt(&mut ap, "N", lower_entry(self, &appearance.normal));
            push_opt(&mut ap, "R", appearance.rollover.as_ref().and_then(|e| lower_entry(self, e)));
            push_opt(&mut ap, "D", appearance.down.as_ref().and_then(|e| lower_entry(self, e)));
            dict.push(entry("AP", PdfObject::Dict(ap)));
        }
        push_opt(&mut dict, "AS", annotation.appearance_state.as_ref().map(PdfObject::name));
        if let Some(markup) = &annotation.markup {
            push_opt(&mut dict, "T", markup.title.as_ref().map(|t| text(t)));
            push_opt(&mut dict, "Popup", annotation_ref(self, markup.popup));
            push_opt(&mut dict, "CA", markup.opacity.map(PdfObject::number));
            push_opt(&mut dict, "RC", markup.rich_contents.as_ref().map(|t| text(t)));
            push_opt(&mut dict, "CreationDate", markup.creation_date.as_ref().map(|d| PdfObject::Str(d.to_string().into_bytes())));
            push_opt(&mut dict, "IRT", annotation_ref(self, markup.in_reply_to));
            push_opt(&mut dict, "Subj", markup.subject.as_ref().map(|t| text(t)));
            push_opt(&mut dict, "RT", markup.reply_type.as_ref().map(PdfObject::name));
            push_opt(&mut dict, "IT", markup.intent.as_ref().map(PdfObject::name));
        }
        push_opt(&mut dict, "OC", annotation.optional_content.as_deref().and_then(|id| self.reference(Category::OptionalContent, id)));
        push_opt(&mut dict, "StructParent", annotation.struct_parent.map(|v| PdfObject::Int(v as i64)));
        dict.append(&mut specific);
        if let Some(parent) = self.widget_parents.get(&[page_index as u32, annotation_index as u32]) {
            dict.push(entry("Parent", PdfObject::Ref(*parent)));
        }
        dict.extend(annotation.extra.iter().cloned());
        dict
    }
}
//#endregion 🔖️Navigation

//#region 🔖️Catalog
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_info(&mut self) -> Vec<PdfDictEntry> {
        let info = &self.snapshot.info;
        let text = |value: &Option<String>| value.as_ref().map(|v| PdfObject::Str(encode_text_string(v)));
        let mut dict = Vec::new();
        push_opt(&mut dict, "Title", text(&info.title));
        push_opt(&mut dict, "Author", text(&info.author));
        push_opt(&mut dict, "Subject", text(&info.subject));
        push_opt(&mut dict, "Keywords", text(&info.keywords));
        push_opt(&mut dict, "Creator", text(&info.creator));
        push_opt(&mut dict, "Producer", text(&info.producer));
        push_opt(&mut dict, "CreationDate", info.creation_date.as_ref().map(|d| PdfObject::Str(d.to_string().into_bytes())));
        push_opt(&mut dict, "ModDate", info.modification_date.as_ref().map(|d| PdfObject::Str(d.to_string().into_bytes())));
        push_opt(&mut dict, "Trapped", info.trapped.as_ref().map(PdfObject::name));
        dict.extend(info.extra.iter().cloned());
        dict
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_catalog(&mut self, pages_ref: ObjRef, include_acro_form: bool) -> PResult<Vec<PdfDictEntry>> {
        let snapshot = self.snapshot;
        let mut dict = vec![entry("Type", PdfObject::name("Catalog")), entry("Pages", PdfObject::Ref(pages_ref))];
        if !snapshot.outlines.is_empty() {
            let outlines_ref = self.reserve();
            let (first, last, count) = self.lower_outline_items(&snapshot.outlines, outlines_ref).expect("non-empty outlines");
            self.set(outlines_ref, PdfObject::Dict(vec![entry("Type", PdfObject::name("Outlines")), entry("First", PdfObject::Ref(first)), entry("Last", PdfObject::Ref(last)), entry("Count", PdfObject::Int(count))]));
            dict.push(entry("Outlines", PdfObject::Ref(outlines_ref)));
        }
        let mut names = Vec::new();
        if !snapshot.named_destinations.is_empty() {
            let pairs: Vec<(Vec<u8>, PdfObject)> = snapshot.named_destinations.iter().map(|dest| (encode_text_string(&dest.name), PdfObject::Dict(vec![entry("D", self.lower_destination(&dest.destination))]))).collect();
            let tree = self.name_tree(pairs);
            names.push(entry("Dests", tree));
        }
        let listed: Vec<&PdfEmbeddedFile> = snapshot.embedded_files.iter().filter(|file| file.listed).collect();
        if !listed.is_empty() {
            let pairs: Vec<(Vec<u8>, PdfObject)> = listed.iter().map(|file| (encode_text_string(&file.id), self.reference(Category::EmbeddedFile, &file.id).unwrap_or(PdfObject::Null))).collect();
            let tree = self.name_tree(pairs);
            names.push(entry("EmbeddedFiles", tree));
        }
        if !names.is_empty() {
            dict.push(entry("Names", PdfObject::Dict(names)));
        }
        if !snapshot.page_labels.is_empty() {
            let mut nums = Vec::new();
            for range in &snapshot.page_labels {
                nums.push(PdfObject::Int(range.start_index as i64));
                let mut label = Vec::new();
                push_opt(&mut label, "S", range.style.map(|style| PdfObject::name(match style {
                    PdfPageLabelStyle::Decimal => "D",
                    PdfPageLabelStyle::RomanUpper => "R",
                    PdfPageLabelStyle::RomanLower => "r",
                    PdfPageLabelStyle::LettersUpper => "A",
                    PdfPageLabelStyle::LettersLower => "a",
                })));
                push_opt(&mut label, "P", range.prefix.as_ref().map(|p| PdfObject::Str(encode_text_string(p))));
                if range.start != 1 {
                    label.push(entry("St", PdfObject::Int(range.start as i64)));
                }
                nums.push(PdfObject::Dict(label));
            }
            dict.push(entry("PageLabels", PdfObject::Dict(vec![entry("Nums", PdfObject::Array(nums))])));
        }
        if !snapshot.output_intents.is_empty() {
            let mut intents = Vec::new();
            for intent in &snapshot.output_intents {
                let mut d = vec![entry("Type", PdfObject::name("OutputIntent")), entry("S", PdfObject::name(&intent.subtype)), entry("OutputConditionIdentifier", PdfObject::Str(encode_text_string(&intent.condition_identifier)))];
                push_opt(&mut d, "OutputCondition", intent.condition.as_ref().map(|v| PdfObject::Str(encode_text_string(v))));
                push_opt(&mut d, "RegistryName", intent.registry_name.as_ref().map(|v| PdfObject::Str(encode_text_string(v))));
                push_opt(&mut d, "Info", intent.info.as_ref().map(|v| PdfObject::Str(encode_text_string(v))));
                if let Some(profile) = &intent.profile {
                    let reference = self.add(stream(vec![entry("N", PdfObject::Int(if profile.len() > 20 && &profile[16..20] == b"GRAY" { 1 } else if profile.len() > 20 && &profile[16..20] == b"CMYK" { 4 } else { 3 }))], profile.clone()));
                    d.push(entry("DestOutputProfile", PdfObject::Ref(reference)));
                }
                intents.push(PdfObject::Ref(self.add(PdfObject::Dict(d))));
            }
            dict.push(entry("OutputIntents", PdfObject::Array(intents)));
        }
        if let Some(form) = snapshot.acro_form.as_ref().filter(|_| include_acro_form) {
            let value = self.lower_acro_form(form);
            dict.push(entry("AcroForm", PdfObject::Ref(self.add(PdfObject::Dict(value)))));
        }
        push_opt(&mut dict, "PageLayout", snapshot.page_layout.map(|layout| PdfObject::name(match layout {
            PdfPageLayout::SinglePage => "SinglePage",
            PdfPageLayout::OneColumn => "OneColumn",
            PdfPageLayout::TwoColumnLeft => "TwoColumnLeft",
            PdfPageLayout::TwoColumnRight => "TwoColumnRight",
            PdfPageLayout::TwoPageLeft => "TwoPageLeft",
            PdfPageLayout::TwoPageRight => "TwoPageRight",
        })));
        push_opt(&mut dict, "PageMode", snapshot.page_mode.map(|mode| PdfObject::name(page_mode_name(mode))));
        if let Some(preferences) = &snapshot.viewer_preferences {
            let mut d = Vec::new();
            for (key, flag) in [("HideToolbar", preferences.hide_toolbar), ("HideMenubar", preferences.hide_menubar), ("HideWindowUI", preferences.hide_window_ui), ("FitWindow", preferences.fit_window), ("CenterWindow", preferences.center_window), ("DisplayDocTitle", preferences.display_doc_title), ("PickTrayByPDFSize", preferences.pick_tray_by_pdf_size)] {
                if flag {
                    d.push(entry(key, PdfObject::Bool(true)));
                }
            }
            push_opt(&mut d, "NonFullScreenPageMode", preferences.non_full_screen_page_mode.map(|mode| PdfObject::name(page_mode_name(mode))));
            for (key, value) in [("Direction", &preferences.direction), ("ViewArea", &preferences.view_area), ("ViewClip", &preferences.view_clip), ("PrintArea", &preferences.print_area), ("PrintClip", &preferences.print_clip), ("PrintScaling", &preferences.print_scaling), ("Duplex", &preferences.duplex)] {
                push_opt(&mut d, key, value.as_ref().map(PdfObject::name));
            }
            if !preferences.print_page_range.is_empty() {
                d.push(entry("PrintPageRange", PdfObject::Array(preferences.print_page_range.iter().map(|v| PdfObject::Int(*v as i64)).collect())));
            }
            push_opt(&mut d, "NumCopies", preferences.num_copies.map(|v| PdfObject::Int(v as i64)));
            d.extend(preferences.extra.iter().cloned());
            dict.push(entry("ViewerPreferences", PdfObject::Dict(d)));
        }
        if let Some(open_action) = &snapshot.open_action {
            let value = match open_action {
                PdfOpenAction::Destination { destination } => self.lower_destination(destination),
                PdfOpenAction::Action { action } => self.lower_action(action),
            };
            dict.push(entry("OpenAction", value));
        }
        push_opt(&mut dict, "Lang", snapshot.language.as_ref().map(|l| PdfObject::Str(encode_text_string(l))));
        if let Some(mark) = &snapshot.mark_info {
            let mut d = Vec::new();
            for (key, flag) in [("Marked", mark.marked), ("UserProperties", mark.user_properties), ("Suspects", mark.suspects)] {
                if flag {
                    d.push(entry(key, PdfObject::Bool(true)));
                }
            }
            dict.push(entry("MarkInfo", PdfObject::Dict(d)));
        }
        if let Some(metadata) = &snapshot.metadata {
            let reference = self.add(raw_stream(vec![entry("Type", PdfObject::name("Metadata")), entry("Subtype", PdfObject::name("XML"))], metadata.as_bytes().to_vec()));
            dict.push(entry("Metadata", PdfObject::Ref(reference)));
        }
        if let Some(optional_content) = &snapshot.optional_content {
            let groups: Vec<PdfObject> = optional_content.groups.iter().filter_map(|group| self.reference(Category::OptionalContent, &group.id)).collect();
            let mut config = vec![entry("Type", PdfObject::name("OCCD"))];
            push_opt(&mut config, "Name", optional_content.name.as_ref().map(|n| PdfObject::Str(encode_text_string(n))));
            if optional_content.base_state_off {
                config.push(entry("BaseState", PdfObject::name("OFF")));
            }
            let ids = |lowering: &Self, ids: &[String]| PdfObject::Array(ids.iter().filter_map(|id| lowering.reference(Category::OptionalContent, id)).collect());
            if !optional_content.on.is_empty() {
                config.push(entry("ON", ids(self, &optional_content.on)));
            }
            if !optional_content.off.is_empty() {
                config.push(entry("OFF", ids(self, &optional_content.off)));
            }
            if !optional_content.order.is_empty() {
                let order: Vec<PdfObject> = optional_content.order.iter().map(|item| self.unmap_oc_ids(item)).collect();
                config.push(entry("Order", PdfObject::Array(order)));
            }
            config.extend(optional_content.extra.iter().cloned());
            dict.push(entry("OCProperties", PdfObject::Dict(vec![entry("OCGs", PdfObject::Array(groups)), entry("D", PdfObject::Dict(config))])));
        }
        dict.extend(snapshot.catalog_extra.iter().cloned());
        Ok(dict)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn unmap_oc_ids(&self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Name(id) => self.reference(Category::OptionalContent, id).unwrap_or_else(|| value.clone()),
            PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| self.unmap_oc_ids(item)).collect()),
            other => other.clone(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_acro_form(&mut self, form: &PdfAcroForm) -> Vec<PdfDictEntry> {
        let mut cursor = 0usize;
        let fields: Vec<PdfObject> = form.fields.iter().map(|field| PdfObject::Ref(self.lower_form_field(field, None, &mut cursor))).collect();
        let mut dict = vec![entry("Fields", PdfObject::Array(fields))];
        if form.need_appearances {
            dict.push(entry("NeedAppearances", PdfObject::Bool(true)));
        }
        if form.signature_flags != 0 {
            dict.push(entry("SigFlags", PdfObject::Int(form.signature_flags as i64)));
        }
        push_opt(&mut dict, "DA", form.default_appearance.as_ref().map(|d| PdfObject::Str(encode_text_string(d))));
        push_opt(&mut dict, "Q", form.quadding.map(|q| PdfObject::Int(q as i64)));
        if !form.default_fonts.is_empty() {
            let fonts: Vec<PdfDictEntry> = form.default_fonts.iter().filter_map(|id| self.reference(Category::Font, id).map(|r| PdfDictEntry::new(id, r))).collect();
            dict.push(entry("DR", PdfObject::Dict(vec![entry("Font", PdfObject::Dict(fonts))])));
        }
        dict.extend(form.extra.iter().cloned());
        dict
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_form_field(&mut self, field: &PdfFormField, parent: Option<ObjRef>, cursor: &mut usize) -> ObjRef {
        let reference = match self.field_refs.get(*cursor) {
            Some(reference) => *reference,
            None => self.reserve(),
        };
        *cursor += 1;
        let text = |value: &str| PdfObject::Str(encode_text_string(value));
        let mut dict = vec![entry("T", text(&field.name))];
        push_opt(&mut dict, "Parent", parent.map(PdfObject::Ref));
        let (field_type, mut specific): (Option<&str>, Vec<PdfDictEntry>) = match &field.kind {
            PdfFormFieldKind::Button { value, default_value, options } => {
                let mut d = Vec::new();
                push_opt(&mut d, "V", value.as_ref().map(PdfObject::name));
                push_opt(&mut d, "DV", default_value.as_ref().map(PdfObject::name));
                if !options.is_empty() {
                    d.push(entry("Opt", PdfObject::Array(options.iter().map(|o| text(o)).collect())));
                }
                (Some("Btn"), d)
            }
            PdfFormFieldKind::Text { value, default_value, max_length, rich_value } => {
                let mut d = Vec::new();
                push_opt(&mut d, "V", value.as_ref().map(|v| text(v)));
                push_opt(&mut d, "DV", default_value.as_ref().map(|v| text(v)));
                push_opt(&mut d, "MaxLen", max_length.map(|v| PdfObject::Int(v as i64)));
                push_opt(&mut d, "RV", rich_value.as_ref().map(|v| text(v)));
                (Some("Tx"), d)
            }
            PdfFormFieldKind::Choice { values, default_values, options, top_index } => {
                let mut d = Vec::new();
                if values.len() == 1 {
                    d.push(entry("V", text(&values[0])));
                } else if !values.is_empty() {
                    d.push(entry("V", PdfObject::Array(values.iter().map(|v| text(v)).collect())));
                }
                if default_values.len() == 1 {
                    d.push(entry("DV", text(&default_values[0])));
                } else if !default_values.is_empty() {
                    d.push(entry("DV", PdfObject::Array(default_values.iter().map(|v| text(v)).collect())));
                }
                if !options.is_empty() {
                    d.push(entry("Opt", PdfObject::Array(options.iter().map(|(export, label)| if export == label { text(label) } else { PdfObject::Array(vec![text(export), text(label)]) }).collect())));
                }
                push_opt(&mut d, "TI", top_index.map(|v| PdfObject::Int(v as i64)));
                (Some("Ch"), d)
            }
            PdfFormFieldKind::Signature { value } => {
                let mut d = Vec::new();
                push_opt(&mut d, "V", value.as_ref().map(|v| PdfObject::Dict(v.clone())));
                (Some("Sig"), d)
            }
            PdfFormFieldKind::Container => (None, Vec::new()),
        };
        push_opt(&mut dict, "FT", field_type.map(PdfObject::name));
        if field.flags != 0 {
            dict.push(entry("Ff", PdfObject::Int(field.flags as i64)));
        }
        dict.append(&mut specific);
        push_opt(&mut dict, "TU", field.alternate_name.as_ref().map(|v| text(v)));
        push_opt(&mut dict, "TM", field.mapping_name.as_ref().map(|v| text(v)));
        push_opt(&mut dict, "DA", field.default_appearance.as_ref().map(|v| text(v)));
        push_opt(&mut dict, "Q", field.quadding.map(|q| PdfObject::Int(q as i64)));
        let mut kids: Vec<PdfObject> = Vec::new();
        for [page, index] in &field.widgets {
            if let Some(widget) = self.annotation_refs.get(*page as usize).and_then(|refs| refs.get(*index as usize)) {
                kids.push(PdfObject::Ref(*widget));
            }
        }
        for child in &field.children {
            kids.push(PdfObject::Ref(self.lower_form_field(child, Some(reference), cursor)));
        }
        if !kids.is_empty() {
            dict.push(entry("Kids", PdfObject::Array(kids)));
        }
        if !field.additional_actions.is_empty() {
            dict.push(entry("AA", PdfObject::Dict(field.additional_actions.clone())));
        }
        dict.extend(field.extra.iter().cloned());
        self.set(reference, PdfObject::Dict(dict));
        reference
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_mode_name(mode: PdfPageMode) -> &'static str {
    match mode {
        PdfPageMode::UseNone => "UseNone",
        PdfPageMode::UseOutlines => "UseOutlines",
        PdfPageMode::UseThumbs => "UseThumbs",
        PdfPageMode::FullScreen => "FullScreen",
        PdfPageMode::UseOc => "UseOC",
        PdfPageMode::UseAttachments => "UseAttachments",
    }
}
//#endregion 🔖️Catalog

#[allow(dead_code)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn _unused(_: &BTreeMap<u8, u8>, _: PdfEngineError, _: &dyn Fn(&PdfFunction)) {
    let _ = lower_function;
}
