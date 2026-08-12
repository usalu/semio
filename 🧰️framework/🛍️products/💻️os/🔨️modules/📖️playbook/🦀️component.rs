//! 🧩️ Playbook document domain + typed VCS on `vcs`.
//!
//! A strict, ordered list of steps containing typed blocks — a Blockly-like
//! visual editor for generating code/data that is list-based, not canvas-based.
//! Block `kind`s beyond [`PLAYBOOK_BUILTIN_KINDS`] are module-contributed
//! (see the `"playbook.blockKind"` topic contribution in `semio-framework-manifest`).

use dsl::DslValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PLAYBOOK_DOCUMENT_SCHEMA: &str = "playbook.program";

pub use builder_kit::{build_palette, build_playbook_list_scene, playbook_builder_action, render_playbook_builder, PlaybookBuilderConfig, PlaybookBuilderLabels, PLAYBOOK_BUILDER_LABELS_EN};
/// 🧬️ Flattens `generation_forms`/`builder_kit` onto the crate root so callers keep the flat
/// `playbook::*` import surface (mirrors how `semio-framework-plugin` flattened these before the move).
pub use generation_forms::{
    add_generation, apply_generation_mutation, generation_operations, handle_generation_action, initial_generation_values, invert_generation_operation, remove_generation, rename_generation, render_generation_form_body,
    render_generation_preview_text, render_generations_tree, select_generation, selected_generation, selected_generation_mut, update_generation_values, FormGeneration, GenerationMutation, GenerationPlayState,
};

//#region 🔖️Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookStep {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub blocks: Vec<PlaybookBlock>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookBlock {
    pub id: String,
    pub label: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<DslValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<PlaybookBlockOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<PlaybookVectorField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixture_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<DslValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(statements, block)]
    pub condition: Option<PlaybookExpr>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookVectorField {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookBlockOption {
    #[serde(alias = "id")]
    pub value: String,
    pub label: String,
}

/// 🧮️ Recursive boolean/comparison expression tree, self-referential via `Box` (`Eq`/`Truthy`) and
/// `Vec` (`And`/`Or`) — the dsl:: engine's lazy `fn() -> RecordSpec` internals handle both forms of
/// recursion natively, so this derives like any other `DslEnum`. `Box<PlaybookExpr>` has no direct
/// `DslField` impl (only named `DslRecord`/`DslScalar`/`DslEnum` types do), so every `Box`/`Vec<Self>`
/// field routes through `#[dsl(statements, block)]` (tagged-variant dispatch, wrapped in its own
/// `{ }` so `Eq`'s two boxed fields don't collide as two bare "the record's one Statements field").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PlaybookExpr {
    Const {
        value: DslValue,
    },
    Var {
        name: String,
    },
    Eq {
        #[dsl(statements, block)]
        left: Box<PlaybookExpr>,
        #[dsl(statements, block)]
        right: Box<PlaybookExpr>,
    },
    And {
        #[dsl(statements, block)]
        items: Vec<PlaybookExpr>,
    },
    Or {
        #[dsl(statements, block)]
        items: Vec<PlaybookExpr>,
    },
    Truthy {
        #[dsl(statements, block)]
        expr: Box<PlaybookExpr>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookValidationError {
    pub block_id: String,
    pub message: String,
}

pub const PLAYBOOK_BUILTIN_KINDS: &[&str] = &["text", "longText", "number", "slider", "boolean", "single", "multi", "date", "color", "vector", "note", "image", "file"];

pub fn is_extension_block_kind(kind: &str) -> bool {
    !PLAYBOOK_BUILTIN_KINDS.contains(&kind)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "playbook", layout = "lines")]
pub struct PlaybookSpec {
    pub schema: String,
    pub id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub steps: Vec<PlaybookStep>,
}


pub fn empty_playbook_snapshot() -> PlaybookSpec {
    PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "playbook".into(), version: "1".into(), title: None, steps: vec![PlaybookStep { id: "s".into(), title: "Steps".into(), description: None, blocks: Vec::new() }] }
}
//#endregion 🔖️Domain

//#region 🔖️Runtime
pub fn flatten_playbook_blocks(spec: &PlaybookSpec) -> Vec<&PlaybookBlock> {
    spec.steps.iter().flat_map(|step| step.blocks.iter()).collect()
}

pub fn find_block_location<'a>(spec: &'a PlaybookSpec, block_id: &str) -> Option<(&'a PlaybookStep, usize, &'a PlaybookBlock)> {
    for step in &spec.steps {
        if let Some(index) = step.blocks.iter().position(|block| block.id == block_id) {
            return Some((step, index, &step.blocks[index]));
        }
    }
    None
}

pub fn dsl_value_to_json(value: DslValue) -> serde_json::Value {
    dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null)
}

pub type PlaybookValues = HashMap<String, DslValue>;

fn playbook_values_from_json(values: &serde_json::Map<String, serde_json::Value>) -> PlaybookValues {
    values.iter().filter_map(|(key, value)| dsl::to_dsl_value(value).ok().map(|dsl| (key.clone(), dsl))).collect()
}

fn dsl_object_nonempty(value: &DslValue) -> bool {
    matches!(value, DslValue::Object(entries) if !entries.is_empty())
}

pub fn eval_playbook_expr(expr: &PlaybookExpr, values: &PlaybookValues) -> DslValue {
    match expr {
        PlaybookExpr::Const { value } => value.clone(),
        PlaybookExpr::Var { name } => values.get(name).cloned().unwrap_or(DslValue::Null),
        PlaybookExpr::Eq { left, right } => DslValue::Bool(eval_playbook_expr(left, values) == eval_playbook_expr(right, values)),
        PlaybookExpr::And { items } => DslValue::Bool(items.iter().all(|item| eval_playbook_expr(item, values).as_bool().unwrap_or(false))),
        PlaybookExpr::Or { items } => DslValue::Bool(items.iter().any(|item| eval_playbook_expr(item, values).as_bool().unwrap_or(false))),
        PlaybookExpr::Truthy { expr } => DslValue::Bool(eval_playbook_expr(expr, values).as_bool().unwrap_or(false)),
    }
}

pub fn is_block_visible(block: &PlaybookBlock, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    block.condition.as_ref().map(|expr| eval_playbook_expr(expr, &playbook_values_from_json(values)).as_bool().unwrap_or(false)).unwrap_or(true)
}

pub fn default_value_for_block(block: &PlaybookBlock) -> DslValue {
    match block.kind.as_str() {
        "text" | "longText" => block.default.clone().unwrap_or(DslValue::String(String::new())),
        "number" | "slider" => block.default.clone().or_else(|| block.min.map(DslValue::Number)).unwrap_or(DslValue::Number(0.0)),
        "boolean" => block.default.clone().unwrap_or(DslValue::Bool(false)),
        "single" => block.default.clone().or_else(|| block.options.as_ref().and_then(|options| options.first()).map(|option| DslValue::String(option.value.clone()))).unwrap_or(DslValue::String(String::new())),
        "multi" => block.default.clone().unwrap_or(DslValue::Array(vec![])),
        "date" | "color" => block.default.clone().unwrap_or(DslValue::String(String::new())),
        "vector" => {
            let values: Vec<DslValue> = block.fields.as_ref().map(|fields| fields.iter().map(|field| DslValue::Number(field.value.unwrap_or(0.0))).collect()).unwrap_or_default();
            DslValue::Array(values)
        }
        "note" | "image" | "file" => DslValue::Null,
        _ if is_extension_block_kind(&block.kind) => block.params.clone().filter(dsl_object_nonempty).unwrap_or(DslValue::Object(vec![])),
        _ => DslValue::Null,
    }
}

pub fn visible_blocks<'a>(step: &'a PlaybookStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<&'a PlaybookBlock> {
    step.blocks.iter().filter(|block| is_block_visible(block, values)).collect()
}

pub fn step_errors(step: &PlaybookStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<PlaybookValidationError> {
    let mut errors = Vec::new();
    for block in visible_blocks(step, values) {
        if block.kind == "note" || block.kind == "image" {
            continue;
        }
        if !block.required.unwrap_or(false) {
            continue;
        }
        let value = values.get(&block.id);
        if is_extension_block_kind(&block.kind) {
            let empty = value.is_none_or(|value| !value.is_object() || value.as_object().is_none_or(|obj| obj.is_empty()));
            if empty {
                errors.push(PlaybookValidationError { block_id: block.id.clone(), message: format!("{} is required", block.label) });
            }
            continue;
        }
        let missing = match value {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(text)) => text.is_empty(),
            Some(serde_json::Value::Array(items)) => items.is_empty(),
            _ => false,
        };
        if missing {
            errors.push(PlaybookValidationError { block_id: block.id.clone(), message: format!("{} is required", block.label) });
        }
    }
    errors
}

pub fn can_advance(step: &PlaybookStep, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    step_errors(step, values).is_empty()
}

pub fn initial_values(spec: &PlaybookSpec, overrides: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
    let mut values = serde_json::Map::new();
    for block in flatten_playbook_blocks(spec) {
        values.insert(block.id.clone(), dsl_value_to_json(default_value_for_block(block)));
    }
    for (key, value) in overrides {
        if values.contains_key(key) {
            values.insert(key.clone(), value.clone());
        }
    }
    values
}
//#endregion 🔖️Runtime


//#region 🔖️Dsl
// 🧬️ `store::ArtifactDsl` for `PlaybookSpec` is generated by the `dsl::DslArtifact` derive on its
// struct definition above (see {@link PlaybookSpec}) — no hand-rolled parser module. The mutation
// vocabulary (`PlaybookMutation` and its `dsl::DslOps`/`dsl::Mutations` derives) now lives in the
// plugin facet, not here — see ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`.
//#endregion 🔖️Dsl


//#region 🔖️HandcraftedDocumentAndOpCodecs
/// 🧬️ P6: `DslArtifact`/`DslOps` emit helpers/`DslVariants` only — trait impls are handcrafted here.
impl store::ArtifactDsl for PlaybookSpec {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlaybookSpec {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️HandcraftedDocumentAndOpCodecs


//#region 🔖️GenerationForms
pub mod generation_forms {
    //! 🧬️ Shared Generate-mode state, CRUD, and declarative UI helpers for answering a `PlaybookSpec` as
    //! a set of named "generations" (parameter presets) — moved here (from `semio-framework-plugin`) since
    //! it is typed end-to-end on `PlaybookSpec`/`PlaybookBlock`, i.e. playbook-domain code, not SDK code.

    use super::{default_value_for_block, flatten_playbook_blocks, is_block_visible, PlaybookBlock, PlaybookSpec};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value};
    use ui_wgpu::wgpu::{
        build_text_editor_scene, ui_stack_vertical, ui_text, ActionDescriptor, Label, Locale, LocalizedLabel, Terminology, TextEditorScene, UiControlNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeActionPlacement,
        UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    };

    //#region 🔖️Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FormGeneration {
        pub id: String,
        pub name: String,
        pub values: Map<String, Value>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GenerationPlayState {
        #[serde(default)]
        pub generations: Vec<FormGeneration>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub selected_generation_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preview_text: Option<String>,
    }
    //#endregion 🔖️Types

    //#region 🔖️Crud
    fn next_generation_id(generations: &[FormGeneration]) -> String {
        format!("generation-{}", generations.len() + 1)
    }

    fn next_generation_name(generations: &[FormGeneration]) -> String {
        format!("Generation {}", generations.len() + 1)
    }

    pub fn initial_generation_values(spec: &PlaybookSpec) -> Map<String, Value> {
        let mut values = Map::new();
        for question in flatten_playbook_blocks(spec) {
            values.insert(question.id.clone(), super::dsl_value_to_json(default_value_for_block(question)));
        }
        values
    }

    pub fn add_generation(state: &mut GenerationPlayState, spec: &PlaybookSpec) -> String {
        let id = next_generation_id(&state.generations);
        let name = next_generation_name(&state.generations);
        state.generations.push(FormGeneration { id: id.clone(), name, values: initial_generation_values(spec) });
        state.selected_generation_id = Some(id.clone());
        id
    }

    pub fn remove_generation(state: &mut GenerationPlayState, generation_id: &str) {
        state.generations.retain(|entry| entry.id != generation_id);
        if state.selected_generation_id.as_deref() == Some(generation_id) {
            state.selected_generation_id = state.generations.first().map(|entry| entry.id.clone());
        }
    }

    pub fn rename_generation(state: &mut GenerationPlayState, generation_id: &str, name: &str) {
        if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
            entry.name = name.to_string();
        }
    }

    pub fn select_generation(state: &mut GenerationPlayState, generation_id: &str) {
        if state.generations.iter().any(|entry| entry.id == generation_id) {
            state.selected_generation_id = Some(generation_id.to_string());
        }
    }

    pub fn selected_generation(state: &GenerationPlayState) -> Option<&FormGeneration> {
        let selected_id = state.selected_generation_id.as_deref()?;
        state.generations.iter().find(|entry| entry.id == selected_id)
    }

    pub fn selected_generation_mut(state: &mut GenerationPlayState) -> Option<&mut FormGeneration> {
        let selected_id = state.selected_generation_id.clone()?;
        state.generations.iter_mut().find(|entry| entry.id == selected_id)
    }

    pub fn update_generation_values(state: &mut GenerationPlayState, generation_id: &str, question_id: &str, value: Value) {
        if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
            entry.values.insert(question_id.to_string(), value);
        }
    }

    pub fn handle_generation_action(action: &str, args: Option<&Value>, state: &mut GenerationPlayState, spec: &PlaybookSpec, controller_id: &str) -> bool {
        match action {
            "addGeneration" => {
                add_generation(state, spec);
                true
            }
            "removeGeneration" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    remove_generation(state, id);
                }
                true
            }
            "selectGeneration" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    select_generation(state, id);
                }
                true
            }
            "renameGeneration" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str());
                if let (Some(id), Some(name)) = (id, name) {
                    rename_generation(state, id, name);
                }
                true
            }
            "updateGenerationValues" => {
                let generation_id = args.and_then(|value| value.get("generationId")).and_then(|value| value.as_str()).map(str::to_string).or_else(|| state.selected_generation_id.clone());
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
                let value = args.and_then(|value| value.get("value"));
                if let (Some(generation_id), Some(question_id), Some(value)) = (generation_id, question_id, value) {
                    update_generation_values(state, &generation_id, question_id, value.clone());
                }
                let _ = controller_id;
                true
            }
            _ => false,
        }
    }
    //#endregion 🔖️Crud

    //#region 🔖️Mutations
    /// @emoji 🧬️ Typed, invertible Generate-mode operation vocabulary. WS-F embeds this as a variant in
    /// `forms/module/procedural`'s own `Mutation` enum so generation edits flow through the document store with
    /// true inverses (replacing the in-place-mutating CRUD helpers as the document mutation surface).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum GenerationMutation {
        Add { generation: FormGeneration },
        Remove { id: String },
        Rename { id: String, name: String },
        UpdateValues { id: String, question_id: String, value: Value },
    }

    /// @emoji 🎛️ Maps a Generate-mode action id to the document operations it produces, or `None` for
    /// non-document (view) actions like `selectGeneration`. Pure — reads `state`/`spec` but mutates
    /// nothing; the caller applies the returned operations through its store.
    pub fn generation_operations(action: &str, args: Option<&Value>, state: &GenerationPlayState, spec: &PlaybookSpec) -> Option<Vec<GenerationMutation>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "addGeneration" => Some(vec![GenerationMutation::Add { generation: FormGeneration { id: next_generation_id(&state.generations), name: next_generation_name(&state.generations), values: initial_generation_values(spec) } }]),
            "removeGeneration" => arg_str("id").map(|id| vec![GenerationMutation::Remove { id }]),
            "renameGeneration" => {
                let id = arg_str("id")?;
                let name = arg_str("name")?;
                Some(vec![GenerationMutation::Rename { id, name }])
            }
            "updateGenerationValues" => {
                let id = arg_str("generationId").or_else(|| state.selected_generation_id.clone())?;
                let question_id = arg_str("questionId")?;
                let value = args.and_then(|value| value.get("value")).cloned()?;
                Some(vec![GenerationMutation::UpdateValues { id, question_id, value }])
            }
            _ => None,
        }
    }

    /// @emoji ▶️ Applies a {@link GenerationMutation} to `state` in place.
    pub fn apply_generation_mutation(state: &mut GenerationPlayState, operation: &GenerationMutation) {
        match operation {
            GenerationMutation::Add { generation } => {
                state.generations.push(generation.clone());
                state.selected_generation_id = Some(generation.id.clone());
            }
            GenerationMutation::Remove { id } => remove_generation(state, id),
            GenerationMutation::Rename { id, name } => rename_generation(state, id, name),
            GenerationMutation::UpdateValues { id, question_id, value } => update_generation_values(state, id, question_id, value.clone()),
        }
    }

    /// @emoji ↩️ Computes the inverse of a {@link GenerationMutation} from the pre-state `state`.
    pub fn invert_generation_operation(state: &GenerationPlayState, operation: &GenerationMutation) -> Vec<GenerationMutation> {
        match operation {
            GenerationMutation::Add { generation } => vec![GenerationMutation::Remove { id: generation.id.clone() }],
            GenerationMutation::Remove { id } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationMutation::Add { generation: entry.clone() }]).unwrap_or_default(),
            GenerationMutation::Rename { id, .. } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationMutation::Rename { id: id.clone(), name: entry.name.clone() }]).unwrap_or_default(),
            GenerationMutation::UpdateValues { id, question_id, .. } => state
                .generations
                .iter()
                .find(|entry| entry.id == *id)
                .map(|entry| vec![GenerationMutation::UpdateValues { id: id.clone(), question_id: question_id.clone(), value: entry.values.get(question_id).cloned().unwrap_or(Value::Null) }])
                .unwrap_or_default(),
        }
    }
    //#endregion 🔖️Mutations

    //#region 🔖️Render
    fn generation_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: controller_id.into(), action: action.into(), args: args.map(|value| dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null)) }
    }

    
    /// 🗣️ Chrome labels for the generations tree — localized at the call site via {@link Locale}/{@link Terminology}.
    fn generation_tree_label(key: &str, locale: Locale, terminology: Terminology) -> Label {
        let localized = LocalizedLabel::from_fn(|_terminology, locale| match (key, locale) {
            ("remove", Locale::De) => "Entfernen".into(),
            ("remove", _) => "Remove".into(),
            ("rename", Locale::De) => "Umbenennen".into(),
            ("rename", _) => "Rename".into(),
            ("generations", Locale::De) => "Generierungen".into(),
            ("generations", _) => "Generations".into(),
            ("add", Locale::De) => "Generierung hinzufügen".into(),
            ("add", _) => "Add Generation".into(),
            ("empty", Locale::De) => "(keine Generierungen)".into(),
            ("empty", _) => "(no generations)".into(),
            ("actions", Locale::De) => "Aktionen".into(),
            ("actions", _) => "Actions".into(),
            _ => key.into(),
        });
        Label::data(localized.resolve(terminology, locale).to_string())
    }

pub fn render_generations_tree(controller_id: &str, surface_prefix: &str, generations: &[FormGeneration], selected_id: Option<&str>, locale: Locale, terminology: Terminology) -> UiNode {
        let items: Vec<UiTreeItemNode> = generations
            .iter()
            .map(|generation| {
                let mut actions = vec![UiTreeItemAction {
                    icon_id: "trash-2".into(),
                    label: Some(generation_tree_label("remove", locale, terminology)),
                    action: generation_action(controller_id, "removeGeneration", Some(json!({ "id": generation.id }))),
                    placement: Some(UiTreeActionPlacement::Menu),
                }];
                actions.insert(
                    0,
                    UiTreeItemAction {
                        icon_id: "pencil".into(),
                        label: Some(generation_tree_label("rename", locale, terminology)),
                        action: generation_action(controller_id, "renameGeneration", Some(json!({ "id": generation.id, "name": format!("{} copy", generation.name) }))),
                        placement: Some(UiTreeActionPlacement::Menu),
                    },
                );
                UiTreeItemNode {
                    id: format!("{surface_prefix}.generation.{}", generation.id),
                    label: Label::data(generation.name.clone()),
                    description: Some(format!("{} values", generation.values.len())),
                    icon_id: Some("layers".into()),
                    presence: UiPresence::selected(selected_id == Some(generation.id.as_str())),
                    default_open: None,
                    action: Some(generation_action(controller_id, "selectGeneration", Some(json!({ "id": generation.id })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: Some(actions),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    dimmed: None,
                    menu: None,
                }
            })
            .collect();
        let mut sections = vec![UiTreeSectionNode {
            id: format!("{surface_prefix}.generations"),
            label: Some(generation_tree_label("generations", locale, terminology)),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    id: format!("{surface_prefix}.generations.empty"),
                    label: generation_tree_label("empty", locale, terminology),
                    description: None,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: None,
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    dimmed: None,
                    menu: None,
                }]
            } else {
                items
            },
            presence: UiPresence::default(),
        }];
        sections.push(UiTreeSectionNode {
            id: format!("{surface_prefix}.actions"),
            label: Some(generation_tree_label("actions", locale, terminology)),
            default_open: Some(true),
            items: vec![UiTreeItemNode {
                id: format!("{surface_prefix}.add-generation"),
                label: generation_tree_label("add", locale, terminology),
                description: None,
                icon_id: Some("plus".into()),
                presence: UiPresence::default(),
                default_open: None,
                action: Some(generation_action(controller_id, "addGeneration", None)),
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
                menu: None,
            }],
            presence: UiPresence::default(),
        });
        UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: Some(generation_action(controller_id, "selectGeneration", None)), drop_action: None, menu: None })
    }

    fn render_question_field(question: &PlaybookBlock, values: &Map<String, Value>, controller_id: &str, patch_action: &str, generation_id: &str) -> Option<UiNode> {
        if !is_block_visible(question, values) {
            return None;
        }
        let value = values.get(&question.id).cloned().unwrap_or_else(|| super::dsl_value_to_json(default_value_for_block(question)));
        let field_id = format!("generate.form.{}", question.id);
        let on_change = || {
            generation_action(
                controller_id,
                patch_action,
                Some(json!({
                    "generationId": generation_id,
                    "questionId": question.id,
                })),
            )
        };
        let child = match question.kind.as_str() {
            "text" | "longText" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: if question.kind == "longText" { "textarea".into() } else { "text".into() },
                value: value.as_str().unwrap_or_default().to_string(),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            "number" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "number".into(),
                value: value.as_f64().map(|number| number.to_string()).unwrap_or_default(),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            "slider" => UiControlNode::Slider(UiSliderNode {
                id: format!("{field_id}.slider"),
                value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: on_change(),
                unit: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            "boolean" => UiControlNode::Toggle(UiToggleNode {
                id: format!("{field_id}.toggle"),
                icon_id: "toggle-left".into(),
                text: Some(Label::data(question.label.clone())),
                on_change: on_change(),
                presence: UiPresence::selected(value.as_bool().unwrap_or(false)),
                menu: None,
            }),
            "single" => {
                let items = question.options.as_ref().map(|options| options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.clone()) }).collect()).unwrap_or_default();
                UiControlNode::Select(UiSelectNode {
                    id: format!("{field_id}.select"),
                    value: value.as_str().unwrap_or_default().to_string(),
                    items,
                    placeholder: question.placeholder.clone().map(Label::data),
                    on_change: on_change(),
                    presence: UiPresence::default(),
                    menu: None,
                })
            }
            "vector" => {
                let numbers = value.as_array().cloned().unwrap_or_else(|| question.fields.as_ref().map(|fields| fields.iter().map(|field| json!(field.value.unwrap_or(0.0))).collect()).unwrap_or_default());
                let labels: Vec<String> = question
                    .fields
                    .as_ref()
                    .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
                    .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
                let children: Vec<UiNode> = numbers
                    .iter()
                    .enumerate()
                    .map(|(index, number)| {
                        let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                        UiNode::Field(UiFieldNode {
                            id: format!("{field_id}.vector.{index}"),
                            label: Label::data(label),
                            child: Box::new(UiNode::Input(UiInputNode {
                                id: format!("{field_id}.vector.{index}.input"),
                                input_kind: "number".into(),
                                value: number.as_f64().map(|entry| entry.to_string()).unwrap_or_default(),
                                placeholder: None,
                                commit: None,
                                on_change: generation_action(
                                    controller_id,
                                    patch_action,
                                    Some(json!({
                                        "generationId": generation_id,
                                        "questionId": question.id,
                                        "fieldIndex": index,
                                    })),
                                ),
                                min: None,
                                max: None,
                                step: None,
                                accept: None,
                                presence: UiPresence::default(),
                                menu: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })
                    })
                    .collect();
                return Some(ui_stack_vertical(children));
            }
            "note" => return Some(ui_text(Label::data(question.text.clone().unwrap_or_default()))),
            "image" => return Some(ui_text(Label::data(question.src.clone().unwrap_or_else(|| "(no image)".into())))),
            _ => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "text".into(),
                value: value.to_string(),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        };
        Some(UiNode::Field(UiFieldNode { id: field_id, label: Label::data(question.label.clone()), child: Box::new(ui_wgpu::wgpu::ui_control_to_node(child)), description: None, required: None, error: None, presence: UiPresence::default(), menu: None }))
    }

    pub fn render_generation_form_body(form_spec: &PlaybookSpec, values: &Map<String, Value>, controller_id: &str, patch_action: &str, generation_id: &str) -> UiNode {
        let mut children = Vec::new();
        for step in &form_spec.steps {
            if !step.blocks.is_empty() {
                children.push(ui_text(Label::data(step.title.clone())));
            }
            for question in &step.blocks {
                if let Some(field) = render_question_field(question, values, controller_id, patch_action, generation_id) {
                    children.push(field);
                }
            }
        }
        if children.is_empty() {
            return ui_text(Label::data("No input widgets to generate from."));
        }
        ui_stack_vertical(children)
    }

    pub fn render_generation_preview_text(surface: &str, controller_id: &str, text: &str) -> UiNode {
        build_text_editor_scene(surface, controller_id, TextEditorScene::base(text.to_string(), Some("json".into()), None))
    }
    //#endregion 🔖️Render

    #[cfg(test)]
    mod generation_forms_tests {
        use super::*;
        use super::super::{PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};

        fn sample_spec() -> PlaybookSpec {
            PlaybookSpec {
                schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
                id: "sample".into(),
                version: "1".into(),
                title: None,
                steps: vec![PlaybookStep {
                    id: "s".into(),
                    title: "Inputs".into(),
                    description: None,
                    blocks: vec![PlaybookBlock {
                        id: "width".into(),
                        label: "Width".into(),
                        kind: "slider".into(),
                        description: None,
                        required: None,
                        placeholder: None,
                        default: Some(json!(1.0)),
                        min: Some(0.0),
                        max: Some(10.0),
                        step: Some(0.5),
                        unit: None,
                        text: None,
                        options: None,
                        fields: None,
                        schema: None,
                        src: None,
                        accept: None,
                        fixture_slug: None,
                        params: None,
                        condition: None,
                    }],
                }],
            }
        }

        #[test]
        fn generation_crud_round_trip() {
            let spec = sample_spec();
            let mut state = GenerationPlayState::default();
            let id = add_generation(&mut state, &spec);
            assert_eq!(state.generations.len(), 1);
            rename_generation(&mut state, &id, "Variant A");
            update_generation_values(&mut state, &id, "width", json!(4.0));
            assert_eq!(selected_generation(&state).unwrap().name, "Variant A");
            remove_generation(&mut state, &id);
            assert!(state.generations.is_empty());
        }

        #[test]
        fn render_generations_tree_contains_add_action() {
            let json = serde_json::to_string(&render_generations_tree("flow-play", "flow-generate", &[], None, Locale::En, Terminology::Native)).unwrap();
            assert!(json.contains("addGeneration"));
        }
    }
}
//#endregion 🔖️GenerationForms

//#region 🔖️BuilderKit
pub mod builder_kit {
    //! 🧩️ Shared strict-list, Blockly-like builder engine: generic step/block CRUD operation-builders and
    //! [`BlockListScene`] rendering, reused by `playbook-plugin` (standalone) and `forms-plugin`
    //! (embedded Blueprint mode). Block-kind-specific property editing stays with the host app. Moved
    //! here (from `semio-framework-plugin`) since it is entirely playbook-domain code.

    use super::PlaybookSpec;
    use semio_framework::ProgramContributionEntry;
    use serde::Deserialize;
    use serde_json::Value;
    use ui_wgpu::wgpu::{ActionDescriptor, BlockListScene, BlockPaletteEntry, IconName, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence};

    //#region 🔖️Config
    #[derive(Clone, Debug)]
    pub struct PlaybookBuilderLabels {
        pub add_step: &'static str,
        pub remove_step: &'static str,
        pub move_up: &'static str,
        pub move_down: &'static str,
        pub add_block: &'static str,
    }

    pub const PLAYBOOK_BUILDER_LABELS_EN: PlaybookBuilderLabels = PlaybookBuilderLabels { add_step: "Add Step", remove_step: "Remove Step", move_up: "Move Up", move_down: "Move Down", add_block: "Add Block" };

    /// 🧩️ Configures the generic strict-list builder for a host app: an action-namespace prefix
    /// (used for element/surface ids so multiple embeddings don't collide), and its labels.
    #[derive(Clone, Debug)]
    pub struct PlaybookBuilderConfig {
        pub action_namespace: &'static str,
        pub controller_id: &'static str,
        pub labels: PlaybookBuilderLabels,
    }
    //#endregion 🔖️Config

    
    //#region 🔖️Render
    pub fn playbook_builder_action(config: &PlaybookBuilderConfig, action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: config.controller_id.into(), action: action.into(), args: args.map(|value| dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null)) }
    }

    //#region 🔖️ContributionResolution
    /// 🗂️ Payload shape decoded from an open `topic_contributions` entry tagged `"playbook.blockKind"` —
    /// carries the `block_kind`/`label`/`icon_id` fields [`build_palette`]'s `extensions` parameter
    /// expects (see `semio-framework-manifest`'s `component.rs`).
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BlockKindPayload {
        block_kind: String,
        label: String,
        icon_id: String,
    }

    /// 🧩️ Resolves a plugin's playbook block-kind palette entries from its manifest contributions'
    /// open `topic_contributions` entries tagged `"playbook.blockKind"` — decodes into the same
    /// `(block_kind, label, icon_id)` triple [`build_palette`]'s `extensions` parameter expects.
    pub fn resolve_block_kind_extensions(contributions: &[ProgramContributionEntry]) -> Vec<(String, String, String)> {
        contributions
            .iter()
            .filter_map(|entry| {
                let topic = entry.topic_contribution.as_ref().filter(|topic| topic.topic == "playbook.blockKind")?;
                let payload = topic.decode::<BlockKindPayload>().ok()?;
                Some((payload.block_kind, payload.label, payload.icon_id))
            })
            .collect()
    }
    //#endregion 🔖️ContributionResolution

    /// 🧩️ Builds the palette of insertable block kinds from a host app's built-in kinds plus any
    /// open `"playbook.blockKind"` topic-contributed modules — pass [`resolve_block_kind_extensions`]'s
    /// output, or a caller-resolved equivalent, as `extensions`.
    pub fn build_palette(builtin: &[(&str, &str, &str)], extensions: &[(String, String, String)]) -> Vec<BlockPaletteEntry> {
        let mut entries: Vec<BlockPaletteEntry> = builtin.iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: (*kind).into(), label: (*label).into(), icon_id: (*icon_id).into() }).collect();
        entries.extend(extensions.iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: kind.clone(), label: label.clone(), icon_id: IconName::from(icon_id.as_str()) }));
        entries
    }

    pub fn build_playbook_list_scene(spec: &PlaybookSpec, palette: &[BlockPaletteEntry], selected_id: Option<&str>) -> BlockListScene {
        BlockListScene { steps_json: serde_json::to_string(&spec.steps).unwrap_or_else(|_| "[]".into()), palette_json: serde_json::to_string(palette).unwrap_or_else(|_| "[]".into()), selected_id: selected_id.map(String::from), dragging_id: None }
    }

    /// 🧩️ Renders the strict-list Blockly-like builder as a [`SurfaceKind::BlockList`] component
    /// scene, handed off to the dedicated `block-list-host.tsx` React host for drag-and-drop.
    pub fn render_playbook_builder(surface_id: &str, spec: &PlaybookSpec, palette: &[BlockPaletteEntry], selected_id: Option<&str>, config: &PlaybookBuilderConfig) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: config.controller_id.into(),
            component_kind: SurfaceKind::BlockList,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: Some(build_playbook_list_scene(spec, palette, selected_id)),
            menu: None,
        })
    }
    //#endregion 🔖️Render

    #[cfg(test)]
    mod builder_kit_tests {
        use super::*;
        use super::super::empty_playbook_snapshot;

        fn sample_config() -> PlaybookBuilderConfig {
            PlaybookBuilderConfig { action_namespace: "playbook-play", controller_id: "playbook-play", labels: PLAYBOOK_BUILDER_LABELS_EN }
        }

        #[test]
        fn render_playbook_builder_emits_block_list_component_scene() {
            let spec = empty_playbook_snapshot();
            let config = sample_config();
            let node = render_playbook_builder("surface", &spec, &[], None, &config);
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"componentKind\":\"block-list\""));
            assert!(json.contains("\"blockList\""));
        }

        fn open_topic_entry() -> ProgramContributionEntry {
            ProgramContributionEntry {
                plugin_id: "playbook-module-procedural".into(),
                topic_contribution: Some(semio_framework::TopicContribution::new(
                    "playbook.blockKind",
                    serde_json::json!({ "appId": "playbook-module-procedural", "blockKind": "buildingComponent", "label": "Building Component", "iconId": "building", "defaultValueJson": "{}", "paramsBodyKey": "params", "previewBodyKey": "preview" }),
                )),
            }
        }

        #[test]
        fn resolve_block_kind_extensions_reads_open_topic_contribution() {
            let extensions = resolve_block_kind_extensions(&[open_topic_entry()]);
            assert_eq!(extensions, vec![("buildingComponent".to_string(), "Building Component".to_string(), "building".to_string())]);
        }

        #[test]
        fn resolve_block_kind_extensions_ignores_unrelated_topics() {
            let mut entry = open_topic_entry();
            entry.topic_contribution = Some(semio_framework::TopicContribution::new("cad.computer", serde_json::json!({ "unrelated": true })));
            let extensions = resolve_block_kind_extensions(&[entry]);
            assert!(extensions.is_empty());
        }
    }
}
//#endregion 🔖️BuilderKit


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_fields_roundtrip() {
        let json = r#"{
            "id":"b1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let block: PlaybookBlock = serde_json::from_str(json).expect("block json");
        assert_eq!(block.min, Some(1.0));
        assert_eq!(block.unit.as_deref(), Some("people"));
        assert!(block.required.unwrap_or(false));
    }

    #[test]
    fn conditional_visibility_filters_blocks() {
        let step = PlaybookStep {
            id: "s".into(),
            title: "Step".into(),
            description: None,
            blocks: vec![
                PlaybookBlock {
                    id: "show".into(),
                    label: "Show".into(),
                    kind: "boolean".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(serde_json::json!(false)),
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                },
                PlaybookBlock {
                    id: "team-size".into(),
                    label: "Team size".into(),
                    kind: "slider".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(serde_json::json!(5)),
                    min: Some(1.0),
                    max: Some(50.0),
                    step: Some(1.0),
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: Some(PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "show".into() }) }),
                },
            ],
        };
        let mut values = serde_json::Map::new();
        values.insert("show".into(), serde_json::json!(false));
        assert_eq!(visible_blocks(&step, &values).len(), 1);
        values.insert("show".into(), serde_json::json!(true));
        assert_eq!(visible_blocks(&step, &values).len(), 2);
    }

    //#region 🔖️DslAndOpText
    fn minimal_block(id: &str, kind: &str) -> PlaybookBlock {
        PlaybookBlock {
            id: id.into(),
            label: format!("Label {id}"),
            kind: kind.into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields: None,
            schema: None,
            src: None,
            accept: None,
            fixture_slug: None,
            params: None,
            condition: None,
        }
    }

    /// 🧱️ A block with EVERY optional property populated (including nested `options`/`fields` and a
    /// deeply nested `condition` exercising every `PlaybookExpr` variant) — the DSL round-trip fixture.
    fn fully_populated_block() -> PlaybookBlock {
        PlaybookBlock {
            id: "b-full".into(),
            label: "Team Size".into(),
            kind: "slider".into(),
            description: Some("How many people?".into()),
            required: Some(true),
            placeholder: Some("Enter a number".into()),
            // 🔢️ `default`/`params` bind through the engine's schema-less `Shape::Value` (arbitrary
            // `serde_json::Value`), whose `DslValue::Number` is `f64`-only — a bare JSON integer
            // literal round-trips back as a float (`5` → `5.0`), so DSL-round-tripped fixtures use
            // float literals throughout to stay byte-for-byte equal after `parse_dsl(print_dsl(_))`.
            default: Some(serde_json::json!(5.0)),
            min: Some(1.0),
            max: Some(50.0),
            step: Some(1.0),
            unit: Some("people".into()),
            text: Some("Some note text\nwith a newline".into()),
            options: Some(vec![PlaybookBlockOption { value: "red".into(), label: "Red".into() }, PlaybookBlockOption { value: "blue".into(), label: "Blue".into() }]),
            fields: Some(vec![PlaybookVectorField { key: "x".into(), label: Some("X".into()), value: Some(1.5) }, PlaybookVectorField { key: "y".into(), label: None, value: None }]),
            schema: Some("solid.step".into()),
            src: Some("https://example.com/img.png".into()),
            accept: Some("image/*".into()),
            fixture_slug: Some("hexagonal-mushroom-column".into()),
            params: Some(serde_json::json!({ "height": 6.0, "nested": { "a": [1.0, 2.0, "three\"quoted"] } })),
            condition: Some(PlaybookExpr::And {
                items: vec![
                    PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "show-team-size".into() }) },
                    PlaybookExpr::Eq { left: Box::new(PlaybookExpr::Var { name: "mode".into() }), right: Box::new(PlaybookExpr::Const { value: serde_json::json!("advanced") }) },
                    PlaybookExpr::Or { items: vec![PlaybookExpr::Var { name: "a".into() }, PlaybookExpr::Var { name: "b".into() }] },
                ],
            }),
        }
    }

    fn sample_spec() -> PlaybookSpec {
        PlaybookSpec {
            schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
            id: "recipe".into(),
            version: "1".into(),
            title: Some("Recipe".into()),
            steps: vec![
                PlaybookStep { id: "s1".into(), title: "Basics".into(), description: Some("First step".into()), blocks: vec![minimal_block("b1", "text"), fully_populated_block()] },
                PlaybookStep { id: "s2".into(), title: "Review".into(), description: None, blocks: Vec::new() },
            ],
        }
    }

    #[test]
    fn empty_playbook_snapshot_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&empty_playbook_snapshot());
        store::test_support::assert_dsl_pack_equivalence(&empty_playbook_snapshot());
    }

    #[test]
    fn sample_spec_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&sample_spec());
        store::test_support::assert_dsl_pack_equivalence(&sample_spec());
    }

    //#endregion 🔖️DslAndOpText
}
//#endregion 🧪️Tests
