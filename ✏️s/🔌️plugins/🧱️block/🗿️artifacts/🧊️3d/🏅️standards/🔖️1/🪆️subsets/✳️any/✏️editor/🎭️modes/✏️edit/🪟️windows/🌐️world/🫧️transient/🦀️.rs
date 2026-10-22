//! 🖌️ Concrete Block3d world-window brush preview state.

pub const WINDOW_KIND_ID: &str = crate::edit::windows::world::BLOCK3D_WINDOW_WORLD;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "block.3dworldwindowtransient")]
#[dsl(layout = "lines")]
pub struct Block3dWorldWindowTransient {
    #[dsl(block)]
    pub brush_preview: Option<Block3dBrushPreview>,
}

impl pack::snapshot::ArtifactDsl for Block3dWorldWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Block3d world-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactPack for Block3dWorldWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Block3dWorldWindowTransient {

}

semio_s_plugin_block::block_optional!(test; /// 🖌️ The optional brush preview set to a value or cleared.
    Block3dBrushPreviewSet(Block3dBrushPreview));

/// 🔺️ Field-sparse diff of [`Block3dWorldWindowTransient`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block3dWorldWindowTransientDiff {
    pub brush_preview: Option<Block3dBrushPreviewSet>,
}

impl protocol::DiffAlgebra<Block3dWorldWindowTransient> for Block3dWorldWindowTransientDiff {
    fn is_empty(&self) -> bool {
        self.brush_preview.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Block3dWorldWindowTransient> for Block3dWorldWindowTransientDiff {
    fn inverse(&self, base: &Block3dWorldWindowTransient) -> Self {
        Self {
            brush_preview: self.brush_preview.as_ref().map(|_| Block3dBrushPreviewSet { value: base.brush_preview.clone() }),
        }
    }
}

impl protocol::MutationDiff<Block3dWorldWindowTransient> for Block3dWorldWindowTransientDiff {}

impl protocol::ColdMutationDiff<Block3dWorldWindowTransient> for Block3dWorldWindowTransientDiff {
    fn absorb(&mut self, later: Self) {
        if later.brush_preview.is_some() {
            self.brush_preview = later.brush_preview;
        }
    }

    fn apply(&self, unpublished: &mut Block3dWorldWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.brush_preview {
            next.brush_preview.clone_from(&value.value);
        }
        *unpublished = next;
        Ok(())
    }
}
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use crate::edit::windows::world::transient::mutations::*;

pub struct Block3dWorldWindowTransientOwner;

#[expect(clippy::unnecessary_wraps, reason = "ArtifactEphemeralTransferPreparationFactory requires a fallible footprint callback")]
fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<Block3dWorldWindowTransientMutation>()))
}

fn preview_transfer(mutation: Block3dWorldWindowTransientMutation) -> Block3dWorldWindowTransient {
    let Block3dWorldWindowTransientMutation::SetBrushPreview(mutation) = mutation;
    Block3dWorldWindowTransient { brush_preview: mutation.preview }
}

impl semio_framework_plugin::WindowTransientOwner for Block3dWorldWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = Block3dWorldWindowTransient;
    type Mutation = Block3dWorldWindowTransientMutation;

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::State>> = std::sync::Arc::new(semio_framework_value::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>> = std::sync::Arc::new(semio_framework_value::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preview_footprint, preview_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
