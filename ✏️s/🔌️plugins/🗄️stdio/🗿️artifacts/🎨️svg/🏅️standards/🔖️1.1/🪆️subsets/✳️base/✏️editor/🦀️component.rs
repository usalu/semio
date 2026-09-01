//! ✏️ `svg` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! SVG has no pixel buffer: `set-pixel-region` parses the vector DSL and emits direct prolog, attribute, and child mutations rather than editing pixels.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::svg::standards::v1_1::subsets::base::schema::mutations::{InsertElementMutation, InsertElementPayload, RemoveElementMutation, RemoveElementPayload, SetAttributeMutation, SetAttributePayload, SetDeclarationMutation, SetDeclarationPayload, SetDoctypeMutation, SetDoctypePayload, SetElementNameMutation, SetElementNamePayload, SvgMutation};
use crate::artifacts::svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
use crate::artifacts::svg::{STDIO_SVG_DOCUMENT_SCHEMA, SVG_ANY_DIALECT};
use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::editor::svg_any::modes::edit;
use crate::editor::svg_any::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum SvgAnyEditCommand {
    SetPixelRegion { source: String },
}

impl protocol::OpBinary for SvgAnyEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_any-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_any-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SvgAnyEditor;

impl ArtifactEditor for SvgAnyEditor {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgAnyEditCommand;

    const DIALECT: Dialect = SVG_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SvgAnyEditCommand::SetPixelRegion { source } => {
                let Ok(snapshot) = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(source) else { return Ok(Emit::default()) };
                if snapshot.doc.prolog != doc.snapshot.doc.prolog { return Ok(Emit::default()) }
                let (Some(XmlNode::Element { name: current_name, attrs: current_attrs, children: current_children }), Some(XmlNode::Element { name, attrs, children })) = (&doc.snapshot.doc.root, &snapshot.doc.root) else { return Ok(Emit::default()) };
                let mut mutations = vec![
                    SvgMutation::SetDeclaration(SetDeclarationMutation::Apply(SetDeclarationPayload { declaration: snapshot.doc.declaration.clone() })),
                    SvgMutation::SetDoctype(SetDoctypeMutation::Apply(SetDoctypePayload { doctype: snapshot.doc.doctype.clone() })),
                ];
                if current_name != name { mutations.push(SvgMutation::SetElementName(SetElementNameMutation::Apply(SetElementNamePayload { path: Vec::new(), name: name.clone() }))); }
                mutations.extend(current_attrs.iter().filter(|current| !attrs.iter().any(|target| target.name == current.name)).map(|current| SvgMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: Vec::new(), name: current.name.clone(), value: None }))));
                mutations.extend(attrs.iter().map(|attribute| SvgMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: Vec::new(), name: attribute.name.clone(), value: Some(attribute.value.clone()) }))));
                mutations.extend((0..current_children.len()).rev().map(|index| SvgMutation::RemoveElement(RemoveElementMutation::Apply(RemoveElementPayload { parent: Vec::new(), index }))));
                mutations.extend(children.iter().cloned().enumerate().map(|(index, node)| SvgMutation::InsertElement(InsertElementMutation::Apply(InsertElementPayload { parent: Vec::new(), index, node }))));
                Ok(Emit::mutations(mutations))
            }
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_svg_any_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SVG_ANY_DIALECT).document(["semio", "svg"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_svg_any_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, SVG_ANY_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SvgAnyEditor as ArtifactEditor>::DIALECT, SVG_ANY_DIALECT);
    }
}
//#endregion 🧪️Tests
