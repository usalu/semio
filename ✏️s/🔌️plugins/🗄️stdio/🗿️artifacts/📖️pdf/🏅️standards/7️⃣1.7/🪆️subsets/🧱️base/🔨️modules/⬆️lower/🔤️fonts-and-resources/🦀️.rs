//! 🔤️ Lowering of fonts and CID system info and of the shared resource dictionaries (images, forms, graphics states, shadings, patterns, colour spaces, properties).

use crate::standards::v1_7::subsets::base::modules::colour::{entry, lower_colour_space, lower_ext_g_state, lower_shading, push_opt, stream};
use crate::standards::v1_7::subsets::base::modules::fonts::{base_encoding_name, cmap};
use crate::standards::v1_7::subsets::base::io::text_string::encode_text_string;
use crate::standards::v1_7::subsets::base::modules::images::lower_image;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use std::collections::BTreeSet;
use crate::standards::v1_7::subsets::base::modules::lower::*;

//#region 🔖️Fonts
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_fonts(&mut self) -> PResult<()> {
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
    fn lower_program(&mut self,program:&PdfFontProgram,glyphs:Option<&BTreeSet<u16>>)->PResult<(&'static str,PdfObject)> {
        crate::standards::v1_7::subsets::base::io::foreign_artifacts::require_font_artifact_kind(program)?;
        let mut object=self.artifacts.resolve(program.reference())?;
        let PdfObject::Stream {dict,data,filters}=&mut object else {return Err(PdfEngineError::Unsupported("font artifact is not a native stream".into()));};
        let key=match program {PdfFontProgram::Type1 {..}=>"FontFile",PdfFontProgram::TrueType {..}=>{if let (Some(glyphs),true)=(glyphs,self.options.subset_fonts) {*data=crate::standards::v1_7::subsets::base::modules::fonts::TrueTypeFont::parse(data).map_err(PdfEngineError::Malformed)?.subset(glyphs);filters.clear();dict.retain(|entry|entry.key!="Length1");dict.push(entry("Length1",PdfObject::Int(data.len() as i64)));}"FontFile2"},_=>"FontFile3"};
        Ok((key,PdfObject::Ref(self.add(object))))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lower_descriptor(&mut self, descriptor: &PdfFontDescriptor, program: Option<&PdfFontProgram>, glyphs: Option<&BTreeSet<u16>>) -> PResult<PdfObject> {
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
            let (key, value) = self.lower_program(program, glyphs)?;
            dict.push(entry(key, value));
        }
        dict.extend(descriptor.extra.iter().cloned());
        Ok(PdfObject::Ref(self.add(PdfObject::Dict(dict))))
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
                    let value = self.lower_descriptor(descriptor, program.as_ref(), glyphs.as_ref())?;
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
                    let value = self.lower_descriptor(descriptor, None, None)?;
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
                let descendant_ref = self.lower_cid_font(descendant, glyphs.as_ref())?;
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
    fn lower_cid_font(&mut self, cid: &PdfCidFont, glyphs: Option<&BTreeSet<u16>>) -> PResult<ObjRef> {
        let mut dict = vec![entry("Type", PdfObject::name("Font")), entry("Subtype", PdfObject::name(if cid.true_type { "CIDFontType2" } else { "CIDFontType0" })), entry("BaseFont", PdfObject::name(&cid.base_font)), entry("CIDSystemInfo", cid_system_info(&cid.system_info))];
        let descriptor = self.lower_descriptor(&cid.descriptor, cid.program.as_ref(), glyphs)?;
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
            Some(PdfCidToGid::Map { glyphs }) => {
                let reference = self.add(stream(Vec::new(),glyphs.iter().flat_map(|value|value.to_be_bytes()).collect()));
                dict.push(entry("CIDToGIDMap", PdfObject::Ref(reference)));
            }
            None => {}
        }
        dict.extend(cid.extra.iter().cloned());
        Ok(self.add(PdfObject::Dict(dict)))
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
    pub(crate) fn lower_optional_content_groups(&mut self) {
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
    pub(crate) fn lower_images(&mut self) -> PResult<()> {
        for image in &self.snapshot.images {
            let reference = self.refs[&(Category::XObject, image.id.clone())];
            let refs = self.refs.clone();
            let mut ref_of = |id: &str| refs.get(&(Category::XObject, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut oc_ref_of = |id: &str| refs.get(&(Category::OptionalContent, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut object = lower_image(image, self, &mut ref_of, &mut oc_ref_of)?;
            if !self.options.compress {
                if let PdfObject::Stream { filters, .. } = &mut object {
                    filters.retain(|filter| filter.is_image_codec());
                }
            }
            self.set(reference, object);
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_forms(&mut self) -> PResult<()> {
        for form in &self.snapshot.forms {
            let reference = self.refs[&(Category::XObject, form.id.clone())];
            let mut dict = vec![entry("Type", PdfObject::name("XObject")), entry("Subtype", PdfObject::name("Form")), entry("FormType", PdfObject::Int(1)), entry("BBox", PdfObject::numbers(&form.bbox))];
            if form.matrix != PDF_IDENTITY_MATRIX {
                dict.push(entry("Matrix", PdfObject::numbers(&form.matrix)));
            }
            let resources = self.resources_for(&form.content);
            dict.push(entry("Resources", resources));
            push_opt(&mut dict, "Group", form.group.as_ref().map(|group| self.lower_group(group)).transpose()?);
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
    pub(crate) fn lower_group(&mut self, group: &PdfTransparencyGroup) -> PResult<PdfObject> {
        let mut dict = vec![entry("Type", PdfObject::name("Group")), entry("S", PdfObject::name("Transparency"))];
        push_opt(&mut dict, "CS", group.color_space.as_ref().map(|cs| lower_colour_space(cs, self)).transpose()?);
        if group.isolated {
            dict.push(entry("I", PdfObject::Bool(true)));
        }
        if group.knockout {
            dict.push(entry("K", PdfObject::Bool(true)));
        }
        Ok(PdfObject::Dict(dict))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_ext_g_states(&mut self) -> PResult<()> {
        for state in &self.snapshot.ext_g_states {
            let reference = self.refs[&(Category::ExtGState, state.id.clone())];
            let refs = self.refs.clone();
            let mut form_ref_of = |id: &str| refs.get(&(Category::XObject, id.to_string())).map(|r| PdfObject::Ref(*r));
            let mut font_ref_of = |id: &str| refs.get(&(Category::Font, id.to_string())).map(|r| PdfObject::Ref(*r));
            let object = lower_ext_g_state(state, self, &mut form_ref_of, &mut font_ref_of)?;
            self.set(reference, object);
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_shadings(&mut self) -> PResult<()> {
        for shading in &self.snapshot.shadings {
            let reference = self.refs[&(Category::Shading, shading.id.clone())];
            let lowered = lower_shading(shading, self)?;
            if let PdfObject::Ref(temporary) = lowered {
                let value = self.objects.iter().position(|object| object.id == temporary).map(|index| self.objects.remove(index).value).unwrap_or(PdfObject::Null);
                self.set(reference, value);
            }
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_patterns(&mut self) -> PResult<()> {
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
    pub(crate) fn lower_named_resources(&mut self) -> PResult<()> {
        for space in &self.snapshot.color_spaces {
            let reference = self.refs[&(Category::ColorSpace, space.name.clone())];
            let value = lower_colour_space(&space.color_space, self)?;
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
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_embedded_files(&mut self) {
        for file in &self.snapshot.embedded_files {
            let reference = self.refs[&(Category::EmbeddedFile, file.id.clone())];
            let mut params = vec![entry("Size", PdfObject::Int(file.data.len() as i64))];
            push_opt(&mut params, "CreationDate", file.creation_date.as_ref().map(|d| PdfObject::Str(crate::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(d).into_bytes())));
            push_opt(&mut params, "ModDate", file.modification_date.as_ref().map(|d| PdfObject::Str(crate::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(d).into_bytes())));
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
