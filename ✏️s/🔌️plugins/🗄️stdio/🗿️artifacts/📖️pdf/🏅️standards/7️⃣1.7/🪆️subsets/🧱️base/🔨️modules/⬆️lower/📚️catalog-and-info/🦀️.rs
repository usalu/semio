//! 📚️ Lowering of the document catalog, the information dictionary and the page mode names.

use crate::standards::v1_7::subsets::base::modules::colour::{entry, push_opt, raw_stream};
use crate::standards::v1_7::subsets::base::io::text_string::encode_text_string;
use crate::standards::v1_7::subsets::base::io::error::PResult;
use crate::standards::v1_7::subsets::base::modules::lower::*;

//#region 🔖️Catalog
impl Lowering<'_> {
    pub(crate) fn lower_catalog(&mut self, pages_ref: ObjRef, include_acro_form: bool) -> PResult<Vec<PdfDictEntry>> {
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
                    let reference = self.add(self.resolve_artifact(profile)?);
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

    fn unmap_oc_ids(&self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Name(id) => self.reference(Category::OptionalContent, id).unwrap_or_else(|| value.clone()),
            PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| self.unmap_oc_ids(item)).collect()),
            other => other.clone(),
        }
    }

    pub(crate) fn lower_acro_form(&mut self, form: &PdfAcroForm) -> Vec<PdfDictEntry> {
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

/// ℹ️ The document information dictionary (ISO 32000-1 §14.3.3) `info` spells.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_info(info: &PdfInfo) -> Vec<PdfDictEntry> {
    let text = |value: &Option<String>| value.as_ref().map(|v| PdfObject::Str(encode_text_string(v)));
    let mut dict = Vec::new();
    push_opt(&mut dict, "Title", text(&info.title));
    push_opt(&mut dict, "Author", text(&info.author));
    push_opt(&mut dict, "Subject", text(&info.subject));
    push_opt(&mut dict, "Keywords", text(&info.keywords));
    push_opt(&mut dict, "Creator", text(&info.creator));
    push_opt(&mut dict, "Producer", text(&info.producer));
    push_opt(&mut dict, "CreationDate", info.creation_date.as_ref().map(|d| PdfObject::Str(crate::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(d).into_bytes())));
    push_opt(&mut dict, "ModDate", info.modification_date.as_ref().map(|d| PdfObject::Str(crate::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(d).into_bytes())));
    push_opt(&mut dict, "Trapped", info.trapped.as_ref().map(PdfObject::name));
    dict.extend(info.extra.iter().cloned());
    dict
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
