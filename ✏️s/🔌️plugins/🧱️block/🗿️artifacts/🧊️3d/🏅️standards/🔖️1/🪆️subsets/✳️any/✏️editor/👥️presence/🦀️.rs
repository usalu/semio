//! 👥️ Block3dPresence — shareable live ephemeral state + mutations.


use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live presence for the block 3d surface. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection/hover used to live here
/// (`selected_ids`/`hovered_vortex_full_id`) — both now broadcast automatically via the framework's
/// typed `PresenceInteraction` for the declared `vortex` domain (see `crate::editor::block3d::create_block3d_app`),
/// so this facet is empty until block3d grows genuinely app-specific live state (e.g. a live camera).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "block3d.presence")]
#[dsl(layout = "lines")]
pub struct Block3dPresence {}

impl store::ArtifactDsl for Block3dPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for Block3dPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
/// 🔺️ The diff of an empty presence: there is nothing to change.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dPresenceDiff {}

impl protocol::DiffAlgebra<Block3dPresence> for Block3dPresenceDiff {
    fn inverse(&self, _base: &Block3dPresence) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl protocol::MutationDiff<Block3dPresence> for Block3dPresenceDiff {
    fn apply(&self, base: &Block3dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block3dPresence> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

/// 🧮️ An empty presence has no mutation: the enum is uninhabited, so no diff can be raised against it.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum Block3dPresenceMutation {}

impl semio_framework_value::retirement::RetireOwned for Block3dPresenceMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { match self {} }
}

impl store::ArtifactPresenceSnapshot for Block3dPresence {}

impl protocol::Mutation<Block3dPresence> for Block3dPresenceMutation {
    type Diff = Block3dPresenceDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }

    fn diff(&self, _base: &Block3dPresence) -> protocol::MutationOutcome<Block3dPresenceDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &Block3dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

impl protocol::OpText for Block3dPresenceMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no presence mutations exist", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl protocol::OpBinary for Block3dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "no-presence-mutation", offset: 0, detail: "no presence mutations exist".into() })
    }
}
