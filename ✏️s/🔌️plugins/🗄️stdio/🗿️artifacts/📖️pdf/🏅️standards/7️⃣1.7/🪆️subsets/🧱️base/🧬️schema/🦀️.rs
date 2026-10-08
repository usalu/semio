//! 🧬️ PdfArtifact schema (1.7) — full artifact state.

use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;

#[path = "🔗️graph-source/🦀️.rs"]
pub mod graph_source;

#[path = "🪪️stream-roles/🦀️.rs"]
pub mod stream_roles;

#[path = "🔎️graph-projection/🦀️.rs"]
pub mod graph_projection;

#[path = "🧭️content-mapping/🦀️.rs"]
pub mod content_mapping;

//#region 🏅️ConformanceSupport
#[path = "🏅️conformance-support/🦀️.rs"]
pub mod conformance_support;
//#endregion 🏅️ConformanceSupport

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub declared_version: String,
    #[state(artifact)]
    #[value(default)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    #[value(default)]
    pub fonts: Vec<PdfFont>,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<PdfImage>,
    #[state(artifact)]
    #[value(default)]
    pub forms: Vec<PdfFormXObject>,
    #[state(artifact)]
    #[value(default)]
    pub ext_g_states: Vec<PdfExtGState>,
    #[state(artifact)]
    #[value(default)]
    pub shadings: Vec<PdfShading>,
    #[state(artifact)]
    #[value(default)]
    pub patterns: Vec<PdfPattern>,
    #[state(artifact)]
    #[value(default)]
    pub color_spaces: Vec<PdfNamedColorSpace>,
    #[state(artifact)]
    #[value(default)]
    pub properties: Vec<PdfNamedProperties>,
    #[state(artifact)]
    #[value(default)]
    pub outlines: Vec<PdfOutlineItem>,
    #[state(artifact)]
    #[value(default)]
    pub named_destinations: Vec<PdfNamedDestination>,
    #[state(artifact)]
    #[value(default)]
    pub page_labels: Vec<PdfPageLabelRange>,
    #[state(artifact)]
    #[value(default)]
    pub embedded_files: Vec<PdfEmbeddedFile>,
    #[state(artifact)]
    #[value(default)]
    pub output_intents: Vec<PdfOutputIntent>,
    #[state(artifact)]
    #[value(default)]
    pub acro_form: Option<PdfAcroForm>,
    #[state(artifact)]
    #[value(default)]
    pub optional_content: Option<PdfOptionalContent>,
    #[state(artifact)]
    #[value(default)]
    pub page_layout: Option<PdfPageLayout>,
    #[state(artifact)]
    #[value(default)]
    pub page_mode: Option<PdfPageMode>,
    #[state(artifact)]
    #[value(default)]
    pub viewer_preferences: Option<PdfViewerPreferences>,
    #[state(artifact)]
    #[value(default)]
    pub open_action: Option<PdfOpenAction>,
    #[state(artifact)]
    #[value(default)]
    pub language: Option<String>,
    #[state(artifact)]
    #[value(default)]
    pub mark_info: Option<PdfMarkInfo>,
    #[state(artifact)]
    #[value(default)]
    pub metadata: Option<String>,
    #[state(artifact)]
    #[value(default)]
    pub document_id: Option<[Vec<u8>; 2]>,
    #[state(artifact)]
    #[value(default)]
    pub encryption: Option<PdfEncryption>,
    #[state(artifact)]
    #[value(default)]
    pub info: PdfInfo,
    #[state(artifact)]
    #[value(default)]
    pub catalog_extra: Vec<PdfDictEntry>,
    #[state(artifact)]
    #[value(default)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    #[value(default)]
    pub trailer: Vec<PdfDictEntry>,
    #[state(artifact)]
    #[value(default)]
    pub admitted_stream_roles: Vec<stream_roles::PdfAdmittedStreamRole>,
}

impl Default for PdfArtifact {
    fn default() -> Self {
        Self::from_snapshot(PdfSnapshot::default())
    }
}

impl PdfArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot { schema: self.schema.clone(), declared_version: self.declared_version.clone(), pages: self.pages.clone(), fonts: self.fonts.clone(), images: self.images.clone(), forms: self.forms.clone(), ext_g_states: self.ext_g_states.clone(), shadings: self.shadings.clone(), patterns: self.patterns.clone(), color_spaces: self.color_spaces.clone(), properties: self.properties.clone(), outlines: self.outlines.clone(), named_destinations: self.named_destinations.clone(), page_labels: self.page_labels.clone(), embedded_files: self.embedded_files.clone(), output_intents: self.output_intents.clone(), acro_form: self.acro_form.clone(), optional_content: self.optional_content.clone(), page_layout: self.page_layout.clone(), page_mode: self.page_mode.clone(), viewer_preferences: self.viewer_preferences.clone(), open_action: self.open_action.clone(), language: self.language.clone(), mark_info: self.mark_info.clone(), metadata: self.metadata.clone(), document_id: self.document_id.clone(), encryption: self.encryption.clone(), info: self.info.clone(), catalog_extra: self.catalog_extra.clone(), objects: self.objects.clone(), trailer: self.trailer.clone(), admitted_stream_roles: self.admitted_stream_roles.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self { schema: snapshot.schema, declared_version: snapshot.declared_version, pages: snapshot.pages, fonts: snapshot.fonts, images: snapshot.images, forms: snapshot.forms, ext_g_states: snapshot.ext_g_states, shadings: snapshot.shadings, patterns: snapshot.patterns, color_spaces: snapshot.color_spaces, properties: snapshot.properties, outlines: snapshot.outlines, named_destinations: snapshot.named_destinations, page_labels: snapshot.page_labels, embedded_files: snapshot.embedded_files, output_intents: snapshot.output_intents, acro_form: snapshot.acro_form, optional_content: snapshot.optional_content, page_layout: snapshot.page_layout, page_mode: snapshot.page_mode, viewer_preferences: snapshot.viewer_preferences, open_action: snapshot.open_action, language: snapshot.language, mark_info: snapshot.mark_info, metadata: snapshot.metadata, document_id: snapshot.document_id, encryption: snapshot.encryption, info: snapshot.info, catalog_extra: snapshot.catalog_extra, objects: snapshot.objects, trailer: snapshot.trailer, admitted_stream_roles: snapshot.admitted_stream_roles }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.declared_version = snapshot.declared_version;
        self.pages = snapshot.pages;
        self.fonts = snapshot.fonts;
        self.images = snapshot.images;
        self.forms = snapshot.forms;
        self.ext_g_states = snapshot.ext_g_states;
        self.shadings = snapshot.shadings;
        self.patterns = snapshot.patterns;
        self.color_spaces = snapshot.color_spaces;
        self.properties = snapshot.properties;
        self.outlines = snapshot.outlines;
        self.named_destinations = snapshot.named_destinations;
        self.page_labels = snapshot.page_labels;
        self.embedded_files = snapshot.embedded_files;
        self.output_intents = snapshot.output_intents;
        self.acro_form = snapshot.acro_form;
        self.optional_content = snapshot.optional_content;
        self.page_layout = snapshot.page_layout;
        self.page_mode = snapshot.page_mode;
        self.viewer_preferences = snapshot.viewer_preferences;
        self.open_action = snapshot.open_action;
        self.language = snapshot.language;
        self.mark_info = snapshot.mark_info;
        self.metadata = snapshot.metadata;
        self.document_id = snapshot.document_id;
        self.encryption = snapshot.encryption;
        self.info = snapshot.info;
        self.catalog_extra = snapshot.catalog_extra;
        self.objects = snapshot.objects;
        self.trailer = snapshot.trailer;
        self.admitted_stream_roles = snapshot.admitted_stream_roles;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf.1.7",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
