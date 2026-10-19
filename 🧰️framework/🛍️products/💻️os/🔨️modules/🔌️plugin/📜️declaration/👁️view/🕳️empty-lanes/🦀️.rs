//! 🕳️ The uninhabited lanes for apps without config, draft, presence or transient state: their empty diffs, snapshots, packs, mutations and operation codecs. See [`super`].

use super::*;

/// 🕳️ The diff of an uninhabited lane (`NoConfig`, `NoPresence`, `NoTransient`): there is no mutation to produce one, so it
/// carries nothing and leaves every state as it is.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoStateDiff {}

/// 🧮️ Default `ArtifactApp::Config` for apps with no config artifact yet.
#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoConfig {}

impl semio_framework_schema_composition::ArtifactCompositionFields for NoConfig {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactDsl for NoConfig {
    const EXTENSION: &'static str = "nocfg";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no config", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl semio_framework_dsl_record::DslField for NoConfig {
    fn shape() -> semio_framework_dsl_record::Shape {
        fn spec() -> semio_framework_dsl_record::RecordSpec {
            semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, Vec::new())
        }
        semio_framework_dsl_record::Shape::Record(semio_framework_dsl_record::RecordSpecProducer {
            ordinary: spec,
            decoding: |control| {
                control.checkpoint()?;
                Ok(semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, control.allocate_vec(0)?))
            },
            encoding: |control| {
                control.checkpoint()?;
                Ok(semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, control.allocate_vec(0)?))
            },
        })
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue::default())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) if record.fields.is_empty() => Ok(Self::default()),
            _ => Err("no config state requires an empty record".into()),
        }
    }
}

impl ArtifactPack for NoConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoConfig {

}

impl store::ConfigRecord for NoConfig {}

impl ::protocol::ColdDiffAlgebra<NoConfig> for NoStateDiff {
    fn inverse(&self, _base: &NoConfig) -> Self {
        Self {}
    }
}

impl ::protocol::DiffAlgebra<NoConfig> for NoStateDiff {
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::ColdMutationDiff<NoConfig> for NoStateDiff {
    fn absorb(&mut self, _other: Self) {}

    fn apply(&self, _unpublished: &mut NoConfig, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        Ok(())
    }
}

impl ::protocol::MutationDiff<NoConfig> for NoStateDiff {

}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum NoConfigMutation {}

semio_framework_os_kernel::original_ordinal_operation_receiving!(NoConfigMutation);

impl ::protocol::ColdMutation<NoConfig> for NoConfigMutation {
    type ReferenceDiff = NoStateDiff;
    fn diff(&self, _base: &NoConfig) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoConfig) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }}

impl ::protocol::Mutation<NoConfig> for NoConfigMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }


}

/// 🕳️ The uninhabited lane has no kinds, so no operation of it is ever labelled.
impl protocol::SemanticMutation<NoConfig> for NoConfigMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}

impl ::protocol::OpText for NoConfigMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no config mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-config-mutation", offset: 0, detail: "no config mutations exist".into() })
    }
}

/// 📝️ Default `ArtifactApp::Draft` for apps with no draft lane yet.
pub type NoDraft = NoConfig;

/// 📝️ Default `ArtifactApp::DraftMutation` twin of {@link NoDraft}.
pub type NoDraftMutation = NoConfigMutation;

/// 👥️ Default `ArtifactApp::Presence` for apps with no shareable live state yet.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoPresence {}

impl store::ArtifactPresenceSnapshot for NoPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        let grant = owner.remaining_grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "empty presence needs original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "empty presence needs original depth admission")); }
        owner.native().checkpoint()?;
        if !bytes.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "empty presence has no encoded fields")); }
        owner.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(Self {})
    }

    fn encode_presence_native(&self, owner: &mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
        let grant = owner.remaining_grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "empty presence needs original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "empty presence needs original depth admission")); }
        owner.native().checkpoint()?;
        owner.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(Vec::new())
    }
}

impl pack::snapshot::ArtifactDsl for NoPresence {
    const EXTENSION: &'static str = "nopres";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no presence", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for NoPresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoPresence {

}

impl ::protocol::ColdDiffAlgebra<NoPresence> for NoStateDiff {
    fn inverse(&self, _base: &NoPresence) -> Self {
        Self {}
    }
}

impl ::protocol::DiffAlgebra<NoPresence> for NoStateDiff {
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::ColdMutationDiff<NoPresence> for NoStateDiff {
    fn absorb(&mut self, _other: Self) {}

    fn apply(&self, _unpublished: &mut NoPresence, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        Ok(())
    }
}

impl ::protocol::MutationDiff<NoPresence> for NoStateDiff {

}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[derive(semio_framework_value::RetireOwned)]
pub enum NoPresenceMutation {}

impl ::protocol::ColdMutation<NoPresence> for NoPresenceMutation {
    type ReferenceDiff = NoStateDiff;
    fn diff(&self, _base: &NoPresence) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoPresence) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }}

impl ::protocol::Mutation<NoPresence> for NoPresenceMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }


}

impl ::protocol::OpText for NoPresenceMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no presence mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-presence-mutation", offset: 0, detail: "no presence mutations exist".into() })
    }
}

/// 🫧️ Default `ArtifactApp::Transient` for apps with no ephemeral local UI state yet.
///
/// 🎯️ The FOURTH and last state mechanism. The four are exhaustive and mutually exclusive:
/// **artifact** = persisted + shared, **config** = persisted + local-only, **presence** =
/// ephemeral + shared, **transient** = ephemeral + local-only. Anything that used to live in a
/// plugin `thread_local!`, a process-local ephemeral box, or an untyped shell field belongs
/// here — typed, dispatched through `Emit`, and readable through `Lanes`, exactly like the other
/// three, rather than reachable from anywhere at any time.
///
/// ⚠️ Transient is NOT the draft lane. A DRAFT is an ephemeral *artifact* — real document
/// content that simply has not been committed. TRANSIENT is UI state that is never document
/// content at all (which pane is focused, what is hovered, an in-flight gesture). They differ in
/// what the state IS, not in how long it lives.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoTransient {}

impl pack::snapshot::ArtifactDsl for NoTransient {
    const EXTENSION: &'static str = "notrans";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no transient", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for NoTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoTransient {

}

impl ::protocol::ColdDiffAlgebra<NoTransient> for NoStateDiff {
    fn inverse(&self, _base: &NoTransient) -> Self {
        Self {}
    }
}

impl ::protocol::DiffAlgebra<NoTransient> for NoStateDiff {
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::ColdMutationDiff<NoTransient> for NoStateDiff {
    fn absorb(&mut self, _other: Self) {}

    fn apply(&self, _unpublished: &mut NoTransient, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        Ok(())
    }
}

impl ::protocol::MutationDiff<NoTransient> for NoStateDiff {

}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[derive(semio_framework_value::RetireOwned)]
pub enum NoTransientMutation {}

impl ::protocol::ColdMutation<NoTransient> for NoTransientMutation {
    type ReferenceDiff = NoStateDiff;
    fn diff(&self, _base: &NoTransient) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoTransient) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }}

impl ::protocol::Mutation<NoTransient> for NoTransientMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }


}

impl ::protocol::OpText for NoTransientMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no transient mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-transient-mutation", offset: 0, detail: "no transient mutations exist".into() })
    }
}
