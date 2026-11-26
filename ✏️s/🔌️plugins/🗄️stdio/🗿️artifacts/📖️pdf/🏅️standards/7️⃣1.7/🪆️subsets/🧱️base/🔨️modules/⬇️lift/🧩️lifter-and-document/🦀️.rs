//! 🧩️ The lifter state, the resource categories, the document entry points and the document-level lanes of the lift.

use crate::standards::v1_7::subsets::base::modules::colour::{extra_entries, lift_colour_space, rect_of};
use crate::standards::v1_7::subsets::base::modules::content::{parse_content, FontTable};
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_f64, dict_get, dict_i64, dict_name};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::io::error::PdfEngineError;
use crate::standards::v1_7::subsets::base::io::text_string::decode_text_string;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue};
use crate::standards::v1_7::subsets::base::schema::content_mapping::{ResourceMap, rename_content};
use crate::standards::v1_7::subsets::base::modules::lift::*;

use semio_s_artifact_stdio_pdf_page_content::project_content_bytes;

//#region 🔖️Lifter
/// 🏷️ Resource categories whose ids share one namespace each (`Page` keys pages by index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(semio_framework_value::RetireOwned)]
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

/// 📦️ Holds the native lifting state independently of each turn's graph borrow.
#[derive(Default)]
pub struct DocumentLiftState {
    pub snapshot: PdfSnapshot,
    pub admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
    pub ids: HashMap<(Category, ObjRef), String>,
    pub used: HashMap<Category, HashSet<String>>,
    pub inline_ids: HashMap<(Category, usize, String), String>,
    pub in_progress: HashSet<(Category, ObjRef)>,
    pub page_refs: Vec<ObjRef>,
    pub annotation_refs: HashMap<ObjRef, (u32, u32)>,
    pub font_codecs: HashMap<String, FontCodec>,
    pub scope: usize,
    pub admission_error: Option<PdfEngineError>,
    pub artifacts: NativePdfArtifactResources<'static>,
}

/// 🧭 One lifting pass over a document.
pub struct Lifter<'s> {
    pub source: &'s mut dyn ObjectSource,
    pub snapshot: PdfSnapshot,
    pub admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
    pub ids: HashMap<(Category, ObjRef), String>,
    pub used: HashMap<Category, HashSet<String>>,
    pub inline_ids: HashMap<(Category, usize, String), String>,
    pub in_progress: HashSet<(Category, ObjRef)>,
    pub page_refs: Vec<ObjRef>,
    pub annotation_refs: HashMap<ObjRef, (u32, u32)>,
    pub font_codecs: HashMap<String, FontCodec>,
    pub scope: usize,
    pub admission_error:Option<PdfEngineError>,
    pub artifacts:NativePdfArtifactResources<'static>,
}

struct MappedFonts<'a> {
    pub map: &'a HashMap<String, String>,
    pub codecs: &'a HashMap<String, FontCodec>,
}

impl FontTable for MappedFonts<'_> {
    fn font(&self, name: &str) -> Option<&FontCodec> {
        self.codecs.get(self.map.get(name)?)
    }
}

impl<'s> Lifter<'s> {
    pub fn bind(source: &'s mut dyn ObjectSource, state: DocumentLiftState) -> Self {
        let DocumentLiftState {
            snapshot, admitted_stream_roles, ids, used, inline_ids, in_progress, page_refs, annotation_refs,
            font_codecs, scope, admission_error, artifacts,
        } = state;
        Self {
            source, snapshot, admitted_stream_roles, ids, used, inline_ids, in_progress, page_refs, annotation_refs,
            font_codecs, scope, admission_error, artifacts,
        }
    }

    pub fn unbind(self) -> DocumentLiftState {
        let Self {
            source: _, snapshot, admitted_stream_roles, ids, used, inline_ids, in_progress, page_refs,
            annotation_refs, font_codecs, scope, admission_error, artifacts,
        } = self;
        DocumentLiftState {
            snapshot, admitted_stream_roles, ids, used, inline_ids, in_progress, page_refs, annotation_refs,
            font_codecs, scope, admission_error, artifacts,
        }
    }

    pub fn text(&mut self, value: Option<&PdfObject>) -> Option<String> {
        let value = self.source.deref(value?);
        match value {
            PdfObject::Text(text) => Some(text),
            PdfObject::Name(name) => Some(name),
            PdfObject::Str(bytes) => Some(decode_text_string(&bytes)),
            _ => None,
        }
    }

    pub(crate) fn date(&mut self, value: Option<&PdfObject>) -> Option<PdfDate> {
        match value.map(|value|self.source.deref(value)) {Some(PdfObject::Date(date))=>Some(date),_=>None}
    }

    /// 🆔 Registers (or looks up) the id of a resource object bound under `name`.
    pub(crate) fn claim_id(&mut self, category: Category, name: &str, reference: Option<ObjRef>) -> (String, bool) {
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

    pub(crate) fn existing_id(&self, category: Category, value: &PdfObject) -> Option<String> {
        self.ids.get(&(category, value.as_ref()?)).cloned()
    }

    pub(crate) fn page_index(&self, value: &PdfObject) -> Option<u32> {
        let reference = value.as_ref()?;
        self.page_refs.iter().position(|candidate| *candidate == reference).map(|index| index as u32)
    }
}
//#endregion 🔖️Lifter

//#region 🔖️Document
impl Lifter<'_> {
    pub fn lift_info(&mut self, dict: &[PdfDictEntry]) -> PdfInfo {
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

    pub fn attribute_value(&mut self, attribute: PageAttribute) -> Option<PdfObject> {
        let mut result = None;
        self.source.inspect(attribute.owner, &mut |node| {
            result = node.and_then(PdfObject::as_dict).and_then(|dict| dict.get(attribute.entry)).map(|entry| entry.value.clone());
        });
        result
    }

    fn attribute_identity(&mut self, attribute: PageAttribute) -> Option<PdfGraphIdentity> {
        let mut identity = None;
        self.source.inspect(attribute.owner, &mut |object| {
            let entry = object.and_then(PdfObject::as_dict).and_then(|entries| entries.get(attribute.entry));
            identity = entry.map(|entry| {
                if let Some(owner) = entry.value.as_ref() {
                    PdfGraphIdentity { owner, path: Vec::new() }
                } else {
                    PdfGraphIdentity { owner: attribute.owner, path: vec![PdfGraphPath::Entry { key: entry.key.clone() }] }
                }
            });
        });
        identity
    }

    fn inherited(&mut self, page_ref: ObjRef, page: &[PdfDictEntry], attribute: Option<PageAttribute>) -> Option<PdfObject> {
        let attribute = attribute?;
        if attribute.owner == page_ref {
            return Some(self.source.deref(&page.get(attribute.entry)?.value));
        }
        let value = self.attribute_value(attribute)?;
        Some(self.source.deref(&value))
    }

    pub fn lift_page(&mut self, index: u32, leaf: PageTreeLeaf, prepared: Option<Vec<PdfOp>>) -> PdfPage {
        let PageTreeLeaf { reference: page_ref, inheritance } = leaf;
        let dict = match self.source.get(page_ref) {
            Some(PdfObject::Dict(dict) | PdfObject::Stream { dict, .. }) => dict,
            _ => Vec::new(),
        };
        let media_box = self.inherited(page_ref, &dict, inheritance.media_box).and_then(|v| rect_of(Some(&v))).unwrap_or([0.0, 0.0, 612.0, 792.0]);
        let mut page = PdfPage::new(media_box[2] - media_box[0], media_box[3] - media_box[1]);
        page.media_box = media_box;
        page.crop_box = self.inherited(page_ref, &dict, inheritance.crop_box).and_then(|v| rect_of(Some(&v)));
        page.bleed_box = rect_of(dict_get(&dict, "BleedBox"));
        page.trim_box = rect_of(dict_get(&dict, "TrimBox"));
        page.art_box = rect_of(dict_get(&dict, "ArtBox"));
        page.rotate = self.inherited(page_ref, &dict, inheritance.rotate).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        page.user_unit = dict_f64(&dict, "UserUnit");
        let resources = self.inherited(page_ref, &dict, inheritance.resources).unwrap_or(PdfObject::Dict(Vec::new()));
        self.scope = page_ref.num as usize;
        let map = self.lift_resources(&resources);
        let program = match prepared {
            Some(operators) => Ok(operators),
            None => {
                let fonts = MappedFonts { map: &map.fonts, codecs: &self.font_codecs };
                let artifacts = &mut self.artifacts;
                project_content_bytes(self.source, dict_get(&dict, "Contents"), |data| parse_content(data, &fonts, artifacts))
            }
        };
        let operators = match program {
            Ok(operators) => operators,
            Err(error) => { self.admission_error = Some(error); Vec::new() }
        };
        page.content = rename_content(operators.clone(), &map);
        if dict_get(&dict, "Contents").is_some() {
            let identity = PdfGraphIdentity { owner: page_ref, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] };
            let mut dependencies = self.source.role_dependencies(&identity);
            if resources.dict_get("Font").is_some() {
                if let Some(mut font_context) = inheritance.resources.and_then(|attribute| self.attribute_identity(attribute)) {
                    font_context.path.push(PdfGraphPath::Entry { key: "Font".into() });
                    dependencies.extend(self.source.role_dependencies(&font_context));
                }
            }

            for owner in self.font_owners() {
                dependencies.extend(self.source.role_dependencies(&PdfGraphIdentity { owner, path: Vec::new() }));
            }
            let mut seen = HashSet::new();
            dependencies.retain(|dependency| seen.insert(dependency.clone()));
            self.admitted_stream_roles.push(PdfAdmittedStreamRole { identity, dependencies, value: PdfStreamRoleValue::Operators { content: operators } });
        }
        page.group = dict_get(&dict, "Group").map(|g| self.source.deref(g)).and_then(|g| g.as_dict().map(|g| self.lift_group(g)));
        page.thumbnail = dict_get(&dict, "Thumb").cloned().map(|thumb| self.lift_x_object("Thumb", &thumb)).filter(|id| !id.is_empty());
        page.struct_parents = dict_i64(&dict, "StructParents").map(|v| v as u32);
        page.transition = dict_get(&dict, "Trans").map(|t| self.source.deref(t)).and_then(|t| t.as_dict().map(<[PdfDictEntry]>::to_vec));
        page.duration = dict_f64(&dict, "Dur");
        page.metadata = dict_get(&dict, "Metadata").map(|m| self.source.deref(m)).and_then(|m| match m {
            PdfObject::Stream { data, .. } => { let text = String::from_utf8_lossy(&data.to_vec_owner()).into_owned(); if let Some(value) = dict_get(&dict, "Metadata") { self.source.record_role(value, PdfStreamRoleValue::MetadataText { text: text.clone() }); } Some(text) },
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

    pub(crate) fn lift_group(&mut self, group: &[PdfDictEntry]) -> PdfTransparencyGroup {
        PdfTransparencyGroup { color_space: dict_get(group, "CS").map(|cs| lift_colour_space(cs, self.source)), isolated: dict_get(group, "I").and_then(PdfObject::as_bool).unwrap_or(false), knockout: dict_get(group, "K").and_then(PdfObject::as_bool).unwrap_or(false) }
    }

    /// 🖋️ Parses a content stream against `map` and rewrites its resource names to ids.
    pub(crate) fn parse_and_rename(&mut self, owner: &PdfObject, content: &[u8], map: &ResourceMap) -> Vec<PdfOp> {
        let ops = match parse_content(content, &MappedFonts {map:&map.fonts,codecs:&self.font_codecs},&mut self.artifacts){Ok(ops)=>ops,Err(error)=>{self.admission_error=Some(error);Vec::new()}};
        self.source.record_role(owner, PdfStreamRoleValue::Operators { content: ops.clone() });
        let mut dependencies = Vec::new();
        for font in self.font_owners() { dependencies.extend(self.source.role_dependencies(&PdfGraphIdentity { owner: font, path: Vec::new() })); }
        self.source.record_role_dependencies(owner, crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleKind::Operators, &dependencies);
        rename_content(ops, map)
    }

    /// 🔢️ Every claimed font object in ascending object order, the one order content roles depend on fonts in.
    pub(crate) fn font_owners(&self) -> Vec<ObjRef> {
        let mut fonts: Vec<_> = self.ids.keys().filter_map(|&(category, reference)| (category == Category::Font).then_some(reference)).collect();
        fonts.sort_unstable();
        fonts
    }
}

//#endregion 🔖️Document
