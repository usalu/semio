//! 🧬️ PdfSnapshot schema (1.7) — the typed ISO 32000-1 document model. Ticket
//! 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened the former text-only page view into the full
//! vocabulary: pages carry their content stream as typed operators (§8 graphics, §9 text) over
//! named resources; fonts (§9.6–9.10), images and form XObjects (§8.9–8.10), colour spaces,
//! functions, shadings and patterns (§8.6–8.7), extended graphics states (§8.4.5), annotations and
//! actions (§12.5–12.6), outlines, destinations and page labels (§12.3–12.4), embedded files
//! (§7.11), interactive forms (§12.7), optional content (§8.11), document metadata (§14.3) and
//! the standard security handler (§7.6) are document-level collections keyed by id.
//!
//! Two lanes, one carrier: the typed lanes are what mutations edit and what every consumer builds;
//! `objects`/`trailer` is the retained COS graph the file was read from (lossless carrier, what the
//! conformance subsets inspect). `🚪️io` lifts the typed lanes out of the graph on decode and lowers
//! them back onto it on encode — see that module for the fixed-point law `lift(lower(t)) == t`.
//!
//! Ground rule: byte/token parsing lives in `🚪️io`; this module is the *typed* logical model.
//! Native lexical choices and encoded stream representations never enter the typed lanes.

use framework_schema::ArtifactSchema;
use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole;

pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model;
pub use model::*;

/// 🏷️ Document schema id for `stdio.pdf` (1.7) -- deliberately distinct from 1.4's flat
/// `stdio.pdf` (avoids colliding with 1.4's own `store::os_store::codec_registry::register_document_codec` registration,
/// which the flat pre-D4 registry can't disambiguate by standard; same shape as gif 89a's
/// `STDIO_GIF89A_DOCUMENT_SCHEMA`, see that module's doc comment).
pub const STDIO_PDF17_DOCUMENT_SCHEMA: &str = "stdio.pdf.1.7";

//#region 🔖️Snapshot
/// 🧬️ `stdio.pdf` (1.7) persistent snapshot: the typed document lanes plus the retained COS
/// carrier (`objects`/`trailer`, the full logical indirect-object graph as read — lossless
/// retention per D2 ground rules, what the conformance subsets inspect).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::RetainedClone)]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub declared_version: String,
    #[state(artifact)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    pub fonts: Vec<PdfFont>,
    #[state(artifact)]
    pub images: Vec<PdfImage>,
    #[state(artifact)]
    pub forms: Vec<PdfFormXObject>,
    #[state(artifact)]
    pub ext_g_states: Vec<PdfExtGState>,
    #[state(artifact)]
    pub shadings: Vec<PdfShading>,
    #[state(artifact)]
    pub patterns: Vec<PdfPattern>,
    #[state(artifact)]
    pub color_spaces: Vec<PdfNamedColorSpace>,
    #[state(artifact)]
    pub properties: Vec<PdfNamedProperties>,
    #[state(artifact)]
    pub outlines: Vec<PdfOutlineItem>,
    #[state(artifact)]
    pub named_destinations: Vec<PdfNamedDestination>,
    #[state(artifact)]
    pub page_labels: Vec<PdfPageLabelRange>,
    #[state(artifact)]
    pub embedded_files: Vec<PdfEmbeddedFile>,
    #[state(artifact)]
    pub output_intents: Vec<PdfOutputIntent>,
    #[state(artifact)]
    pub acro_form: Option<PdfAcroForm>,
    #[state(artifact)]
    pub optional_content: Option<PdfOptionalContent>,
    #[state(artifact)]
    pub page_layout: Option<PdfPageLayout>,
    #[state(artifact)]
    pub page_mode: Option<PdfPageMode>,
    #[state(artifact)]
    pub viewer_preferences: Option<PdfViewerPreferences>,
    #[state(artifact)]
    pub open_action: Option<PdfOpenAction>,
    #[state(artifact)]
    pub language: Option<String>,
    #[state(artifact)]
    pub mark_info: Option<PdfMarkInfo>,
    #[state(artifact)]
    pub metadata: Option<String>,
    #[state(artifact)]
    pub document_id: Option<[Vec<u8>; 2]>,
    #[state(artifact)]
    pub encryption: Option<PdfEncryption>,
    #[state(artifact)]
    pub info: PdfInfo,
    #[state(artifact)]
    pub catalog_extra: Vec<PdfDictEntry>,
    #[state(artifact)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    pub trailer: Vec<PdfDictEntry>,
    #[state(artifact)]
    pub admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: Vec::new(),
            fonts: Vec::new(),
            images: Vec::new(),
            forms: Vec::new(),
            ext_g_states: Vec::new(),
            shadings: Vec::new(),
            patterns: Vec::new(),
            color_spaces: Vec::new(),
            properties: Vec::new(),
            outlines: Vec::new(),
            named_destinations: Vec::new(),
            page_labels: Vec::new(),
            embedded_files: Vec::new(),
            output_intents: Vec::new(),
            acro_form: None,
            optional_content: None,
            page_layout: None,
            page_mode: None,
            viewer_preferences: None,
            open_action: None,
            language: None,
            mark_info: None,
            metadata: None,
            document_id: None,
            encryption: None,
            info: PdfInfo::default(),
            catalog_extra: Vec::new(),
            objects: Vec::new(),
            trailer: Vec::new(),
            admitted_stream_roles: Vec::new(),
        }
    }
}

impl PdfSnapshot {
    pub fn font(&self, id: &str) -> Option<&PdfFont> {
        self.fonts.iter().find(|font| font.id == id)
    }
    pub fn image(&self, id: &str) -> Option<&PdfImage> {
        self.images.iter().find(|image| image.id == id)
    }
    pub fn form(&self, id: &str) -> Option<&PdfFormXObject> {
        self.forms.iter().find(|form| form.id == id)
    }
    pub fn ext_g_state(&self, id: &str) -> Option<&PdfExtGState> {
        self.ext_g_states.iter().find(|state| state.id == id)
    }
    pub fn shading(&self, id: &str) -> Option<&PdfShading> {
        self.shadings.iter().find(|shading| shading.id == id)
    }
    pub fn pattern(&self, id: &str) -> Option<&PdfPattern> {
        self.patterns.iter().find(|pattern| pattern.id == id)
    }
    /// 🆔 Every id the document-level collections currently use (fonts, XObjects, graphics
    /// states, shadings, patterns, embedded files, optional content groups).
    pub fn resource_ids(&self) -> std::collections::BTreeSet<String> {
        let mut ids = std::collections::BTreeSet::new();
        ids.extend(self.fonts.iter().map(|font| font.id.clone()));
        ids.extend(self.images.iter().map(|image| image.id.clone()));
        ids.extend(self.forms.iter().map(|form| form.id.clone()));
        ids.extend(self.ext_g_states.iter().map(|state| state.id.clone()));
        ids.extend(self.shadings.iter().map(|shading| shading.id.clone()));
        ids.extend(self.patterns.iter().map(|pattern| pattern.id.clone()));
        ids.extend(self.color_spaces.iter().map(|space| space.name.clone()));
        ids.extend(self.properties.iter().map(|properties| properties.name.clone()));
        ids.extend(self.embedded_files.iter().map(|file| file.id.clone()));
        if let Some(optional_content) = &self.optional_content {
            ids.extend(optional_content.groups.iter().map(|group| group.id.clone()));
        }
        ids
    }
    /// 🆔 A fresh id of the form `{prefix}{n}` not yet used by any collection.
    pub fn fresh_id(&self, prefix: &str) -> String {
        let ids = self.resource_ids();
        (1..).map(|index| format!("{prefix}{index}")).find(|candidate| !ids.contains(candidate)).expect("an unbounded id sequence has a free member")
    }
}

/// 🌱️ First-party value encoding for the public PDF snapshot, preserving the camelCase object
/// shape `value_derive::ToValue` would emit; `schema` is the only required key on decode.
impl pack::value::ToValue for PdfSnapshot {
    fn to_value(&self) -> pack::value::DslValue {
        pack::value::DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("declaredVersion".to_string(), self.declared_version.to_value()),
            ("pages".to_string(), self.pages.to_value()),
            ("fonts".to_string(), self.fonts.to_value()),
            ("images".to_string(), self.images.to_value()),
            ("forms".to_string(), self.forms.to_value()),
            ("extGStates".to_string(), self.ext_g_states.to_value()),
            ("shadings".to_string(), self.shadings.to_value()),
            ("patterns".to_string(), self.patterns.to_value()),
            ("colorSpaces".to_string(), self.color_spaces.to_value()),
            ("properties".to_string(), self.properties.to_value()),
            ("outlines".to_string(), self.outlines.to_value()),
            ("namedDestinations".to_string(), self.named_destinations.to_value()),
            ("pageLabels".to_string(), self.page_labels.to_value()),
            ("embeddedFiles".to_string(), self.embedded_files.to_value()),
            ("outputIntents".to_string(), self.output_intents.to_value()),
            ("acroForm".to_string(), self.acro_form.to_value()),
            ("optionalContent".to_string(), self.optional_content.to_value()),
            ("pageLayout".to_string(), self.page_layout.to_value()),
            ("pageMode".to_string(), self.page_mode.to_value()),
            ("viewerPreferences".to_string(), self.viewer_preferences.to_value()),
            ("openAction".to_string(), self.open_action.to_value()),
            ("language".to_string(), self.language.to_value()),
            ("markInfo".to_string(), self.mark_info.to_value()),
            ("metadata".to_string(), self.metadata.to_value()),
            ("documentId".to_string(), octets::document_id_to_value(&self.document_id)),
            ("encryption".to_string(), self.encryption.to_value()),
            ("info".to_string(), self.info.to_value()),
            ("catalogExtra".to_string(), self.catalog_extra.to_value()),
            ("objects".to_string(), self.objects.to_value()),
            ("trailer".to_string(), self.trailer.to_value()),
            ("admittedStreamRoles".to_string(), self.admitted_stream_roles.to_value()),
        ])
    }
    fn to_value_controlled(&self,control:&mut pack::value::NativeEncodeControl<'_>)->Result<pack::value::DslValue,pack::value::ValueError>{
        use pack::value::{DslValue as V,ValueError as E};
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            control.begin_stage(32)?;let mut fields=V::object_encoding_controlled(32,control)?;
            macro_rules! field{($key:literal,$value:expr)=>{{let key=control.copy_text($key)?;let value=$value.map_err(|error:E|error.under($key))?;fields.get_mut().push((key,value));control.step()?;}};}
            field!("schema",self.schema.to_value_controlled(control));
            field!("declaredVersion",self.declared_version.to_value_controlled(control));
            field!("pages",self.pages.to_value_controlled(control));
            field!("fonts",self.fonts.to_value_controlled(control));
            field!("images",self.images.to_value_controlled(control));
            field!("forms",self.forms.to_value_controlled(control));
            field!("extGStates",self.ext_g_states.to_value_controlled(control));
            field!("shadings",self.shadings.to_value_controlled(control));
            field!("patterns",self.patterns.to_value_controlled(control));
            field!("colorSpaces",self.color_spaces.to_value_controlled(control));
            field!("properties",self.properties.to_value_controlled(control));
            field!("outlines",self.outlines.to_value_controlled(control));
            field!("namedDestinations",self.named_destinations.to_value_controlled(control));
            field!("pageLabels",self.page_labels.to_value_controlled(control));
            field!("embeddedFiles",self.embedded_files.to_value_controlled(control));
            field!("outputIntents",self.output_intents.to_value_controlled(control));
            field!("acroForm",self.acro_form.to_value_controlled(control));
            field!("optionalContent",self.optional_content.to_value_controlled(control));
            field!("pageLayout",self.page_layout.to_value_controlled(control));
            field!("pageMode",self.page_mode.to_value_controlled(control));
            field!("viewerPreferences",self.viewer_preferences.to_value_controlled(control));
            field!("openAction",self.open_action.to_value_controlled(control));
            field!("language",self.language.to_value_controlled(control));
            field!("markInfo",self.mark_info.to_value_controlled(control));
            field!("metadata",self.metadata.to_value_controlled(control));
            field!("documentId",octets::document_id_to_value_controlled(&self.document_id,control));
            field!("encryption",self.encryption.to_value_controlled(control));
            field!("info",self.info.to_value_controlled(control));
            field!("catalogExtra",self.catalog_extra.to_value_controlled(control));
            field!("objects",self.objects.to_value_controlled(control));
            field!("trailer",self.trailer.to_value_controlled(control));
            field!("admittedStreamRoles",self.admitted_stream_roles.to_value_controlled(control));
            Ok(V::Object(fields.take()))
        }))
    }

}

/// 🔀️ First-party value decoding for the public PDF snapshot. `schema` remains required while
/// every other field falls back to its default; a path edit goes through the value tree ([`pack::value::edit_through_value`]).
impl pack::value::FromValue for PdfSnapshot {
    fn retire_decoded(self){
        fn retire<T:pack::value::FromValue>(value:T){T::retire_decoded(value)}
        retire(self.schema);retire(self.declared_version);retire(self.pages);retire(self.fonts);retire(self.images);retire(self.forms);retire(self.ext_g_states);retire(self.shadings);retire(self.patterns);retire(self.color_spaces);retire(self.properties);retire(self.outlines);retire(self.named_destinations);retire(self.page_labels);retire(self.embedded_files);retire(self.output_intents);retire(self.acro_form);retire(self.optional_content);retire(self.page_layout);retire(self.page_mode);retire(self.viewer_preferences);retire(self.open_action);retire(self.language);retire(self.mark_info);retire(self.metadata);retire(self.document_id);retire(self.encryption);retire(self.info);retire(self.catalog_extra);retire(self.objects);retire(self.trailer);retire(self.admitted_stream_roles);
    }
    fn edit_value_at_path(&mut self, path: &[&str], edit: pack::value::ValueEdit) -> Result<(), pack::value::ValueError> {
        pack::value::edit_through_value(self, path, edit)
    }

    fn from_value(value: pack::value::DslValue) -> Result<Self, pack::value::ValueError> {
        use pack::value::FromValue;
        let entries = value.into_object()?;
        let field = |key: &str| entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value.clone());
        fn decode_or_default<T: FromValue + Default>(value: Option<pack::value::DslValue>, key: &str) -> Result<T, pack::value::ValueError> {
            match value {
                Some(pack::value::DslValue::Null) | None => Ok(T::default()),
                Some(value) => T::from_value(value).map_err(|error| error.under(key)),
            }
        }
        let schema = field("schema").ok_or_else(|| pack::value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"missing field `schema`"))?;
        let snapshot=Self {
            schema: String::from_value(schema).map_err(|error| error.under("schema"))?,
            declared_version: decode_or_default(field("declaredVersion"), "declaredVersion")?,
            pages: decode_or_default(field("pages"), "pages")?,
            fonts: decode_or_default(field("fonts"), "fonts")?,
            images: decode_or_default(field("images"), "images")?,
            forms: decode_or_default(field("forms"), "forms")?,
            ext_g_states: decode_or_default(field("extGStates"), "extGStates")?,
            shadings: decode_or_default(field("shadings"), "shadings")?,
            patterns: decode_or_default(field("patterns"), "patterns")?,
            color_spaces: decode_or_default(field("colorSpaces"), "colorSpaces")?,
            properties: decode_or_default(field("properties"), "properties")?,
            outlines: decode_or_default(field("outlines"), "outlines")?,
            named_destinations: decode_or_default(field("namedDestinations"), "namedDestinations")?,
            page_labels: decode_or_default(field("pageLabels"), "pageLabels")?,
            embedded_files: decode_or_default(field("embeddedFiles"), "embeddedFiles")?,
            output_intents: decode_or_default(field("outputIntents"), "outputIntents")?,
            acro_form: decode_or_default(field("acroForm"), "acroForm")?,
            optional_content: decode_or_default(field("optionalContent"), "optionalContent")?,
            page_layout: decode_or_default(field("pageLayout"), "pageLayout")?,
            page_mode: decode_or_default(field("pageMode"), "pageMode")?,
            viewer_preferences: decode_or_default(field("viewerPreferences"), "viewerPreferences")?,
            open_action: decode_or_default(field("openAction"), "openAction")?,
            language: decode_or_default(field("language"), "language")?,
            mark_info: decode_or_default(field("markInfo"), "markInfo")?,
            metadata: decode_or_default(field("metadata"), "metadata")?,
            document_id: octets::document_id_from_value(field("documentId").unwrap_or(pack::value::DslValue::Null)).map_err(|error|error.under("documentId"))?,
            encryption: decode_or_default(field("encryption"), "encryption")?,
            info: decode_or_default(field("info"), "info")?,
            catalog_extra: decode_or_default(field("catalogExtra"), "catalogExtra")?,
            objects: decode_or_default(field("objects"), "objects")?,
            trailer: decode_or_default(field("trailer"), "trailer")?,
            admitted_stream_roles: decode_or_default(field("admittedStreamRoles"), "admittedStreamRoles")?,
        };
        crate::standards::v1_7::subsets::base::schema::stream_roles::validate_role_inputs(&snapshot.objects,&snapshot.objects,&[],&snapshot.admitted_stream_roles).map_err(|message|pack::value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message).under("admittedStreamRoles"))?;
        Ok(snapshot)
    }
}






//#endregion 🔖️Snapshot

//#region 🔖️SnapshotFixtures



//#endregion 🔖️SnapshotFixtures



