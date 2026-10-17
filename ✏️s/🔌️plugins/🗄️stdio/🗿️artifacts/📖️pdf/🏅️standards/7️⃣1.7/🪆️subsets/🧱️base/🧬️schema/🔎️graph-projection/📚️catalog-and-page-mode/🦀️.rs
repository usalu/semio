//! 📚️ Lifting of the document catalog (interactive form, optional content, viewer preferences, metadata) and the page mode names.

use super::*;

//#region 🔖️Catalog
impl Lifter<'_> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn lift_optional_content(&mut self, catalog: &[PdfDictEntry]) {
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
    pub(super) fn map_oc_refs(&mut self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Ref(_) => self.existing_id(Category::OptionalContent, value).map(PdfObject::Name).unwrap_or(PdfObject::Null),
            PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| self.map_oc_refs(item)).collect()),
            other => other.clone(),
        }
    }

    /// 📎 Lifts the `/EmbeddedFiles` name tree first so its keys become the file ids.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn lift_embedded_files(&mut self, catalog: &[PdfDictEntry]) {
        let Some(names) = dict_get(catalog, "Names").map(|n| self.source.deref(n)) else { return };
        let Some(files) = names.dict_get("EmbeddedFiles").cloned() else { return };
        let mut pairs = Vec::new();
        self.name_tree(&files, &mut pairs, &mut HashSet::new());
        for (key, value) in pairs {
            let name = key;
            self.lift_embedded_file(&name, &value, true);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn lift_catalog(&mut self, catalog: &[PdfDictEntry]) {
        if let Some(outlines) = dict_get(catalog, "Outlines") {
            self.snapshot.outlines = self.lift_outlines(outlines);
        }
        if let Some(names) = dict_get(catalog, "Names").map(|n| self.source.deref(n)) {
            if let Some(dests) = names.dict_get("Dests").cloned() {
                let mut pairs = Vec::new();
                self.name_tree(&dests, &mut pairs, &mut HashSet::new());
                for (key, value) in pairs {
                    if let Some(destination) = self.lift_destination(&value) {
                        self.snapshot.named_destinations.push(PdfNamedDestination { name: key, destination });
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
                    PdfObject::Stream { .. } => match self.source.semantic_role(dict_get(&dict, "DestOutputProfile").unwrap_or(&PdfObject::Null), PdfStreamRoleKind::ReferenceBody) { Some(PdfStreamRoleValue::ReferenceBody { reference }) => Some(reference), _ => None },
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
            stream @ PdfObject::Stream { .. } => Some(role_text(self.source, &stream)),
            _ => None,
        });
        self.snapshot.catalog_extra = extra_entries(catalog, &["Type", "Version", "Pages", "Outlines", "Names", "Dests", "PageLabels", "OutputIntents", "AcroForm", "PageLayout", "PageMode", "ViewerPreferences", "OpenAction", "Lang", "MarkInfo", "Metadata", "OCProperties"]);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn lift_acro_form(&mut self, form: &[PdfDictEntry]) -> PdfAcroForm {
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
    pub(super) fn lift_form_field(&mut self, value: &PdfObject, inherited_type: Option<&str>, depth: usize) -> Option<PdfFormField> {
        if depth > 32 {
            return None;
        }
        let resolved = self.source.deref(value);
        let dict = resolved.as_dict()?.to_vec();
        let field_type = dict_name(&dict, "FT").map(str::to_string).or_else(|| inherited_type.map(str::to_string));
        let text_value = |lifter: &mut Self, key: &str| -> Option<String> {
            match dict_get(&dict, key).map(|v| lifter.source.deref(v)) {
                Some(PdfObject::Text(text)) => Some(text),
                Some(PdfObject::Str(bytes)) => Some(role_text(lifter.source, &PdfObject::Str(bytes.clone()))),
                Some(PdfObject::Name(name)) => Some(name),
                Some(stream @ PdfObject::Stream { .. }) => Some(role_text(lifter.source, &stream)),
                _ => None,
            }
        };
        let text_values = |lifter: &mut Self, key: &str| -> Vec<String> {
            match dict_get(&dict, key).map(|v| lifter.source.deref(v)) {
                Some(PdfObject::Array(items)) => items.iter().filter_map(|item| match item {
                    PdfObject::Text(text) => Some(text.clone()),
                    PdfObject::Str(bytes) => Some(role_text(lifter.source, &PdfObject::Str(bytes.clone()))),
                    PdfObject::Name(name) => Some(name.clone()),
                    _ => None,
                }).collect(),
                Some(PdfObject::Text(text)) => vec![text],
                Some(PdfObject::Str(bytes)) => vec![role_text(lifter.source, &PdfObject::Str(bytes.clone()))],
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
                    PdfObject::Array(pair) => (self.text(pair.first()).unwrap_or_default(), self.text(pair.get(1)).unwrap_or_default()),
                    PdfObject::Text(text) => (text.clone(),text.clone()),
                    PdfObject::Str(bytes) => (role_text(self.source, &PdfObject::Str(bytes.clone())), role_text(self.source, &PdfObject::Str(bytes.clone()))),
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
