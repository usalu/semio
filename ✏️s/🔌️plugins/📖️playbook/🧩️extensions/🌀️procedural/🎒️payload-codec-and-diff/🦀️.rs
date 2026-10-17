//! 🎒️ Payload of the procedural module: the render payload record with its codec and the sparse payload diff. See [`super`].

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "procmodule")]
pub struct ModuleRenderPayload {
    pub(super) example_id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch because the key
    /// set is driven entirely by whichever `Widget::InputSlider`/`Widget::Neuron` ids the referenced
    /// `example_id`'s flow graph happens to define (see `apply_flow_params`, which walks `params` as
    /// an arbitrary `key -> f64` map and forwards every entry to `FlowHost::set_slider_value`) — no
    /// fixed schema spans all fixtures, so a typed `dsl::DslArtifact` derive doesn't apply here.
    #[dsl(value)]
    #[value(default = "default_params_field")]
    pub(super) params: DslValue,
    pub(super) question_id: String,
    pub(super) controller_id: String,
    pub(super) surface: String,
    pub(super) interactive: bool,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl pack::snapshot::ArtifactDsl for ModuleRenderPayload {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.procedural"
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl pack::snapshot::ArtifactPack for ModuleRenderPayload {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for ModuleRenderPayload {

}

/// 🧒️ Composition view of the module payload — every field is a scalar or the untyped `params`
/// escape hatch, so this block-kind module owns no child artifacts and links to none.
impl semio_framework_schema_composition::ArtifactCompositionFields for ModuleRenderPayload {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}
//#endregion 🔖️ArtifactCodec

fn default_params_field() -> DslValue {
    DslValue::Null
}

/// 🌱️ The module's default document — the hex-column learning program with its stock procedural params. Used
/// as `ArtifactApp::initial_snapshot`; live slot renders override it with the forms-supplied payload.
pub(super) fn default_payload() -> ModuleRenderPayload {
    ModuleRenderPayload {
        example_id: "hexagonal-mushroom-column".into(),
        params: json_to_dsl_value(&semio_framework_pack_json::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })),
        question_id: String::new(),
        controller_id: String::new(),
        surface: "try".into(),
        interactive: true,
    }
}

pub(super) fn params_as_json(params: &DslValue) -> Value {
    json_from_dsl_value(params)
}

//#region 🔖️DocumentMutation
/// 🔺️ Sparse field delta over [`ModuleRenderPayload`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ModulePayloadDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub example_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub params: Option<DslValue>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub controller_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
}

impl protocol::MutationDiff<ModuleRenderPayload> for ModulePayloadDiff {
    fn apply(&self, base: &ModuleRenderPayload, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ModuleRenderPayload> {
        let mut next = base.clone();
        if let Some(value) = &self.example_id {
            next.example_id = value.clone();
        }
        if let Some(value) = &self.params {
            next.params = value.clone();
        }
        if let Some(value) = &self.question_id {
            next.question_id = value.clone();
        }
        if let Some(value) = &self.controller_id {
            next.controller_id = value.clone();
        }
        if let Some(value) = &self.surface {
            next.surface = value.clone();
        }
        if let Some(value) = &self.interactive {
            next.interactive = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.example_id.is_some() {
            self.example_id = other.example_id;
        }
        if other.params.is_some() {
            self.params = other.params;
        }
        if other.question_id.is_some() {
            self.question_id = other.question_id;
        }
        if other.controller_id.is_some() {
            self.controller_id = other.controller_id;
        }
        if other.surface.is_some() {
            self.surface = other.surface;
        }
        if other.interactive.is_some() {
            self.interactive = other.interactive;
        }
    }
}

impl protocol::DiffAlgebra<ModuleRenderPayload> for ModulePayloadDiff {
    fn inverse(&self, base: &ModuleRenderPayload) -> Self {
        Self {
            example_id: self.example_id.as_ref().map(|_| base.example_id.clone()),
            params: self.params.as_ref().map(|_| base.params.clone()),
            question_id: self.question_id.as_ref().map(|_| base.question_id.clone()),
            controller_id: self.controller_id.as_ref().map(|_| base.controller_id.clone()),
            surface: self.surface.as_ref().map(|_| base.surface.clone()),
            interactive: self.interactive.as_ref().map(|_| base.interactive.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.example_id.is_none() && self.params.is_none() && self.question_id.is_none() && self.controller_id.is_none() && self.surface.is_none() && self.interactive.is_none()
    }
}
//#endregion 🔖️DocumentMutation
//#endregion 🔖️Payload
