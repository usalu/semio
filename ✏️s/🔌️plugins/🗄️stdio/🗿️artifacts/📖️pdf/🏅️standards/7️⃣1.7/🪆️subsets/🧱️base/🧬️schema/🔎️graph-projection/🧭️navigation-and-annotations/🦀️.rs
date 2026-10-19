//! 🧭️ Lifting of the page tree, outlines, named destinations, page labels and annotations.

use super::*;

//#region 🔖️Navigation
impl Lifter<'_> {
    /// 🎯 Lifts a destination: an explicit array, a name/string, or a dictionary with `/D`.
    pub(super) fn lift_destination(&mut self, value: &PdfObject) -> Option<PdfDestination> {
        let resolved = self.source.deref(value);
        match resolved {
            PdfObject::Name(name) => Some(PdfDestination::Named { name }),
            PdfObject::Text(name) => Some(PdfDestination::Named { name }),
            PdfObject::Str(bytes) => Some(PdfDestination::Named { name: role_text(self.source, &PdfObject::Str(bytes.clone())) }),
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

    pub(super) fn lift_file_specification(&mut self, value: &PdfObject) -> Option<PdfFileSpecification> {
        let resolved = self.source.deref(value);
        match resolved {
            PdfObject::Text(path) => Some(PdfFileSpecification::Path { path }),
            PdfObject::Str(bytes) => Some(PdfFileSpecification::Path { path: role_text(self.source, &PdfObject::Str(bytes.clone())) }),
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
    pub(super) fn lift_embedded_file(&mut self, name: &str, value: &PdfObject, listed: bool) -> Option<String> {
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
        let PdfObject::Stream { dict: stream_dict, .. } = self.source.deref(&stream_ref) else { return None };
        let data = role_words(self.source, &stream_ref);
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
    pub(super) fn lift_action(&mut self, value: &PdfObject) -> Option<PdfAction> {
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict()?.to_vec();
        let subtype = dict_name(&dict, "S").unwrap_or("").to_string();
        let new_window = dict_get(&dict, "NewWindow").and_then(PdfObject::as_bool);
        let names = |lifter: &mut Self, key: &str| -> Vec<String> {
            dict_get(&dict, key).map(|v| lifter.source.deref(v)).map(|v| match v {
                PdfObject::Array(items) => items.iter().filter_map(|item| match item {
                    PdfObject::Text(text) => Some(text.clone()),
                    PdfObject::Str(bytes) => Some(role_text(lifter.source, &PdfObject::Str(bytes.clone()))),
                    PdfObject::Name(name) => Some(name.clone()),
                    _ => None,
                }).collect(),
                PdfObject::Text(text) => vec![text],
                PdfObject::Str(bytes) => vec![role_text(lifter.source, &PdfObject::Str(bytes.clone()))],
                _ => Vec::new(),
            }).unwrap_or_default()
        };
        let kind = match subtype.as_str() {
            "GoTo" => PdfActionKind::GoTo { destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d))? },
            "GoToR" => PdfActionKind::GoToRemote { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f))?, destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d)).unwrap_or(PdfDestination::RemotePage { page: 0, fit: PdfDestinationFit::Fit }), new_window },
            "GoToE" => PdfActionKind::GoToEmbedded { destination: dict_get(&dict, "D").cloned().and_then(|d| self.lift_destination(&d)).unwrap_or(PdfDestination::RemotePage { page: 0, fit: PdfDestinationFit::Fit }), new_window },
            "Launch" => PdfActionKind::Launch { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)).unwrap_or(PdfFileSpecification::Path { path: String::new() }), new_window },
            "Thread" => PdfActionKind::Thread { file: dict_get(&dict, "F").cloned().and_then(|f| self.lift_file_specification(&f)), thread: dict_i64(&dict, "D").unwrap_or(0).max(0) as u32 },
            "URI" => PdfActionKind::Uri { uri: self.text(dict_get(&dict, "URI")).unwrap_or_default(), is_map: dict_get(&dict, "IsMap").and_then(PdfObject::as_bool).unwrap_or(false) },
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
                Some(PdfObject::Text(text)) => text,
                Some(PdfObject::Str(bytes)) => role_text(self.source, &PdfObject::Str(bytes.clone())),
                Some(stream @ PdfObject::Stream { .. }) => role_text(self.source, &stream),
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
    pub(super) fn lift_outlines(&mut self, root: &PdfObject) -> Vec<PdfOutlineItem> {
        let root = self.source.deref(root);
        let Some(first) = root.dict_get("First").cloned() else { return Vec::new() };
        let mut visited = HashSet::new();
        self.lift_outline_siblings(&first, &mut visited, 0)
    }

    pub(super) fn lift_outline_siblings(&mut self, first: &PdfObject, visited: &mut HashSet<u32>, depth: usize) -> Vec<PdfOutlineItem> {
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
    pub(super) fn name_tree(&mut self, node: &PdfObject, out: &mut Vec<(String, PdfObject)>, visited: &mut HashSet<u32>) {
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
                    if let [key, value] = pair {
                        if let Some(key) = self.text(Some(key)) { out.push((key, value.clone())); }
                    }
                }
            }
        }
    }

    /// 🔢 Flattens a number tree (§7.9.7) into (key, value) pairs.
    pub(super) fn number_tree(&mut self, node: &PdfObject, out: &mut Vec<(i64, PdfObject)>, visited: &mut HashSet<u32>) {
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
    pub(super) fn lift_appearance_entry(&mut self, id: &str, value: &PdfObject) -> Option<PdfAppearanceEntry> {
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

    pub(super) fn lift_annotation(&mut self, dict: &[PdfDictEntry]) -> PdfAnnotation {
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
            "Popup" => PdfAnnotationKind::Popup { parent: dict_get(dict, "Parent").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| u64::from(*index))), open: dict_get(dict, "Open").and_then(PdfObject::as_bool).unwrap_or(false) },
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
                popup: dict_get(dict, "Popup").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| u64::from(*index))),
                opacity: dict_f64(dict, "CA"),
                rich_contents: self.text(dict_get(dict, "RC")),
                creation_date: self.date(dict_get(dict, "CreationDate")),
                in_reply_to: dict_get(dict, "IRT").and_then(PdfObject::as_ref).and_then(|r| self.annotation_refs.get(&r).map(|(_, index)| u64::from(*index))),
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

pub(super) const MARKUP_SUBTYPES: &[&str] = &["Text", "FreeText", "Line", "Square", "Circle", "Polygon", "PolyLine", "Highlight", "Underline", "Squiggly", "StrikeOut", "Stamp", "Caret", "Ink", "FileAttachment", "Sound", "Redact"];

pub(super) const ANNOTATION_KEYS: &[&str] = &[
    "Type", "Subtype", "Rect", "Contents", "P", "NM", "M", "F", "AP", "AS", "Border", "BS", "C", "StructParent", "OC", "T", "Popup", "CA", "RC", "CreationDate", "IRT", "Subj", "RT", "IT", "ExData", "Open", "Name", "State", "StateModel", "A", "Dest", "H", "PA", "QuadPoints", "DA", "Q", "DS", "CL", "LE", "BE", "RD", "L", "LL", "LLE", "Cap", "LLO", "CP", "IC", "Vertices", "InkList", "Parent", "FS", "Sound", "Movie", "MK", "AA", "MN", "FixedPrint", "OverlayText", "Repeat", "Sy", "BM", "ca",
];
//#endregion 🔖️Annotations
