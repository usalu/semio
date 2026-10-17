//! 🧭️ Lowering of the page tree and pages and of the navigation structures (outlines, named destinations, page labels, annotations).

use crate::standards::v1_7::subsets::base::modules::colour::{entry, push_opt, raw_stream};
use crate::standards::v1_7::subsets::base::io::text_string::encode_text_string;
use crate::standards::v1_7::subsets::base::io::error::PResult;
use crate::standards::v1_7::subsets::base::modules::lower::*;

//#region 🔖️Pages
impl Lowering<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn lower_page(&mut self, index: usize, page: &PdfPage, reference: ObjRef, parent: ObjRef) -> PResult<()> {
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
        push_opt(&mut dict, "Group", page.group.as_ref().map(|group| self.lower_group(group)).transpose()?);
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
    pub(crate) fn lower_destination(&self, destination: &PdfDestination) -> PdfObject {
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
    pub(crate) fn lower_action(&mut self, action: &PdfAction) -> PdfObject {
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
    pub(crate) fn lower_outline_items(&mut self, items: &[PdfOutlineItem], parent: ObjRef) -> Option<(ObjRef, ObjRef, i64)> {
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
    pub(crate) fn name_tree(&mut self, pairs: Vec<(Vec<u8>, PdfObject)>) -> PdfObject {
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
        let annotation_ref = |lowering: &Self, index: Option<u64>| index.and_then(|index| usize::try_from(index).ok()).and_then(|index| lowering.annotation_refs.get(page_index).and_then(|refs| refs.get(index))).map(|r| PdfObject::Ref(*r));
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
            push_opt(&mut dict, "CreationDate", markup.creation_date.as_ref().map(|d| PdfObject::Str(crate::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(d).into_bytes())));
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
