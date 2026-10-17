//! 🪟️ Lifting of the resource dictionaries (images, forms, graphics states, shadings, patterns, colour spaces, properties) and of fonts.

use crate::standards::v1_7::subsets::base::modules::colour::{array_n, extra_entries, lift_colour_space, lift_ext_g_state, lift_shading, matrix_of, numbers_of, rect_of};
use crate::standards::v1_7::subsets::base::modules::fonts::{base_encoding_from_name, cmap, FontCodec};
use crate::standards::v1_7::subsets::base::modules::images::lift_image;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_f64, dict_get, dict_i64, dict_name};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use std::collections::HashMap;
use crate::standards::v1_7::subsets::base::io::error::PdfEngineError;
use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleValue;
use crate::standards::v1_7::subsets::base::schema::content_mapping::ResourceMap;
use crate::standards::v1_7::subsets::base::modules::lift::*;

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
    pub(crate) fn lift_ext_g_state_entry(&mut self, name: &str, value: &PdfObject) -> String {
        let (id, fresh) = self.claim_id(Category::ExtGState, name, value.as_ref());
        if fresh {
            let resolved = self.source.deref(value);
            let dict = resolved.as_dict().map(<[PdfDictEntry]>::to_vec).unwrap_or_default();
            let form_id_of = |group: &PdfObject, _source: &mut dyn ObjectSource| -> Option<String> { self_form_id(group) };
            let font_id_of = |font: &PdfObject, _source: &mut dyn ObjectSource| -> Option<String> { self_font_id(font) };
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
    pub(crate) fn lift_shading_entry(&mut self, name: &str, value: &PdfObject) -> String {
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
    pub(crate) fn lift_pattern(&mut self, name: &str, value: &PdfObject) -> String {
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
                let content = self.parse_and_rename(value, data, &map);
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
    pub(crate) fn lift_x_object_claimed(&mut self, id: &str, value: &PdfObject) -> String {
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
                let image = match lift_image(&id, dict, data, filters, self.source, &mut id_of, &mut oc_id_of) {Ok(image)=>image,Err(error)=>{self.admission_error=Some(error);return String::new();}};
                self.source.record_role(value, PdfStreamRoleValue::Image { image: image.clone() });
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
                let content = self.parse_and_rename(value, data, &map);
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
pub(crate) fn self_form_id(_group: &PdfObject) -> Option<String> {
    None
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn self_font_id(_font: &PdfObject) -> Option<String> {
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
            PdfObject::Stream { data, .. } => { let mapping = cmap::parse_to_unicode(&data); if let Some(value) = dict_get(&dict, "ToUnicode") { self.source.record_role(value, PdfStreamRoleValue::UnicodeMap { mapping: mapping.clone() }); } Some(mapping) },
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
                        if let Some(value) = dict_get(&dict, "Encoding") { self.source.record_role(value, PdfStreamRoleValue::CharacterMap { cmap: parsed.clone() }); }
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
                        let content = self.parse_and_rename(&entry.value, &data, &map);
                        char_procs.push(PdfCharProc { name: entry.key, content });
                    }
                }
                self.scope = saved_scope;
                PdfFontKind::Type3 { font_matrix: matrix_of(dict_get(&dict, "FontMatrix")).unwrap_or([0.001, 0.0, 0.0, 0.001, 0.0, 0.0]), font_bbox: rect_of(dict_get(&dict, "FontBBox")).unwrap_or([0.0; 4]), encoding, first_char, widths, char_procs, descriptor }
            }
            _ => PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program },
        };
        let font = PdfFont { id: id.clone(), kind, to_unicode, extra: extra_entries(&dict, &["Type", "Subtype", "BaseFont", "Name", "FirstChar", "LastChar", "Widths", "FontDescriptor", "Encoding", "ToUnicode", "DescendantFonts", "FontMatrix", "FontBBox", "CharProcs", "Resources"]) };
        match FontCodec::new(&font,&self.artifacts) {Ok(codec)=>{self.font_codecs.insert(id.clone(),codec);},Err(error)=>self.admission_error=Some(error)};
        self.snapshot.fonts.push(font);
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lift_simple_encoding(&mut self, value: Option<&PdfObject>) -> PdfSimpleEncoding {
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
    pub(crate) fn lift_descriptor(&mut self, dict: &[PdfDictEntry]) -> (PdfFontDescriptor, Option<PdfFontProgram>) {
        let program = ["FontFile", "FontFile2", "FontFile3"].iter().find_map(|key| {
            let stream = self.source.deref(dict_get(dict, key)?);
            let PdfObject::Stream { dict:stream_dict,.. }=&stream else {return None};
            let kind=match *key {"FontFile"=>"type1","FontFile2"=>"truetype",_=>match dict_name(stream_dict,"Subtype") {Some("CIDFontType0C")=>"cid-cff",Some("OpenType")=>"opentype",_=>"cff"}};
            let reference=match self.artifacts.admit(&format!("s.stdio.font.{kind}"),stream) {Ok(reference)=>reference,Err(error)=>{self.admission_error=Some(error);return None;}};
            let program=match kind {"type1"=>PdfFontProgram::Type1 {reference},"truetype"=>PdfFontProgram::TrueType {reference},"cid-cff"=>PdfFontProgram::CidCff {reference},"opentype"=>PdfFontProgram::OpenType {reference},_=>PdfFontProgram::Cff {reference}};
            self.source.record_role(dict_get(dict, key)?, PdfStreamRoleValue::FontProgram { program: program.clone() });
            Some(program)
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
    pub(crate) fn lift_cid_font(&mut self, dict: &[PdfDictEntry]) -> PdfCidFont {
        let (descriptor, program) = dict_get(dict, "FontDescriptor").map(|d| self.source.deref(d)).and_then(|d| d.as_dict().map(|d| self.lift_descriptor(d))).unwrap_or_default();
        let system_info = dict_get(dict, "CIDSystemInfo").map(|s| self.source.deref(s)).and_then(|s| s.as_dict().map(<[PdfDictEntry]>::to_vec)).map(|s| PdfCidSystemInfo { registry: self.text(dict_get(&s, "Registry")).unwrap_or_else(|| "Adobe".into()), ordering: self.text(dict_get(&s, "Ordering")).unwrap_or_else(|| "Identity".into()), supplement: dict_i64(&s, "Supplement").unwrap_or(0).max(0) as u32 }).unwrap_or_default();
        let widths = dict_get(dict, "W").map(|w| self.source.deref(w)).and_then(|w| w.as_array().map(<[PdfObject]>::to_vec)).map(|items| self.lift_cid_widths(&items)).unwrap_or_default();
        let vertical_metrics = dict_get(dict, "W2").map(|w| self.source.deref(w)).and_then(|w| w.as_array().map(<[PdfObject]>::to_vec)).map(|items| self.lift_cid_vertical(&items)).unwrap_or_default();
        let cid_to_gid = match dict_get(dict,"CIDToGIDMap").map(|m|self.source.deref(m)) {
            Some(PdfObject::Name(name)) if name=="Identity"=>Some(PdfCidToGid::Identity),
            Some(PdfObject::Stream {data,..})=>{if data.len()%2!=0 {self.admission_error=Some(PdfEngineError::Malformed("CIDToGIDMap must contain complete glyph ids".into()));None}else{let glyphs=data.chunks_exact(2).map(|pair|u16::from_be_bytes([pair[0],pair[1]])).collect::<Vec<_>>();if let Some(value)=dict_get(dict,"CIDToGIDMap"){self.source.record_role(value,PdfStreamRoleValue::GlyphIds {glyphs:glyphs.clone()});}Some(PdfCidToGid::Map {glyphs})}},
            _=>None,
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
    pub(crate) fn lift_cid_widths(&mut self, items: &[PdfObject]) -> Vec<PdfCidWidthRun> {
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
    pub(crate) fn lift_cid_vertical(&mut self, items: &[PdfObject]) -> Vec<PdfCidVerticalRun> {
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
