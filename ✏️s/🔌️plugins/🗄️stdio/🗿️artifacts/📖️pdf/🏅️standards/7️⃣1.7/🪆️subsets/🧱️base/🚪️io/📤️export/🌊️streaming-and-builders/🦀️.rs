//! 🌊️ The page-at-a-time `DocumentStream`, the text layout and the typed document builders.

use super::*;

//#region 🔖️Streaming
/// 🌊 A page-at-a-time document writer for guests with per-step budgets: the document-level
/// lanes are lowered up front, then every `page` call yields that page's bytes, and `finish`
/// closes the file. Pages appended this way reference the snapshot's fonts, images, forms and
/// graphics states by id exactly like [`PdfPage::content`] does.
pub struct DocumentStream {
    writer: PdfWriter,
    lowered: LoweredDocument,
    pub pages_ref: ObjRef,
    pub page_refs: Vec<ObjRef>,
    written_pages: usize,
    pub next_number: u32,
    pub snapshot: PdfSnapshot,
    pub options: LowerOptions,
    root_resources: PdfObject,
    pub annotation_refs: Vec<Vec<ObjRef>>,
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
//#region 🔖️Builders
/// 📐 Simple text layout over a font: line breaking on words by real advance widths, returning
/// the operators that show the lines top-down from `(x, top)`.
pub struct PdfTextLayout<'a> {
    pub font: &'a PdfFont,
    pub size: f64,
    pub leading: f64,
    pub codec: FontCodec,
}
impl<'a> PdfTextLayout<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(font: &'a PdfFont, size: f64,resources:&dyn foreign_artifacts::PdfArtifactResourcePort) -> PResult<Self> {
        Ok(Self { font, size, leading: size * 1.2, codec: FontCodec::new(font,resources)? })
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
    let layout = PdfTextLayout::new(&font, 12.0,&foreign_artifacts::NativePdfArtifactResources::default()).expect("standard font has no foreign resource");
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
pub fn embedded_true_type_font(snapshot:&mut PdfSnapshot,id: &str, program: &[u8], base_font: &str, text: Option<&str>) -> Result<PdfFont, String> {
    use crate::standards::v1_7::subsets::base::modules::fonts::TrueTypeFont;
    let font = TrueTypeFont::parse(program)?;
    let reference=foreign_artifacts::admit_pdf_artifact(snapshot,"s.stdio.font.truetype",PdfObject::Stream {dict:vec![PdfDictEntry::new("Length1",PdfObject::Int(program.len() as i64))],data:program.to_vec(),filters:Vec::new()}).map_err(|error|format!("{error:?}"))?;
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
            descendant: PdfCidFont { true_type: true, base_font: base_font.to_string(), system_info: PdfCidSystemInfo::default(), descriptor, default_width: 1000.0, widths: runs, default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: Some(PdfCidToGid::Identity), program: Some(PdfFontProgram::TrueType { reference }), extra: Vec::new() },
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
