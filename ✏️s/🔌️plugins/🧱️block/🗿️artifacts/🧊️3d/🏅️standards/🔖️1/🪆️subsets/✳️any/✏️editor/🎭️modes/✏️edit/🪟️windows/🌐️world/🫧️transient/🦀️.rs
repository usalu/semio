//! 🖌️ Concrete Block3d world-window brush preview state.

pub const WINDOW_KIND_ID: &str = crate::editor::block3d::modes::edit::windows::world::BLOCK3D_WINDOW_WORLD;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "block.3dworldwindowtransient")]
#[dsl(layout = "lines")]
pub struct Block3dWorldWindowTransient {
    #[dsl(block)]
    pub brush_preview: Option<Block3dBrushPreview>,
}

impl store::ArtifactDsl for Block3dWorldWindowTransient {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Block3d world-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Block3dWorldWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
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
    fn inverse(&self, base: &Block3dWorldWindowTransient) -> Self {
        Self {
            brush_preview: self.brush_preview.as_ref().map(|_| Block3dBrushPreviewSet { value: base.brush_preview.clone() }),
        }
    }
    fn between(base: &Block3dWorldWindowTransient, other: &Block3dWorldWindowTransient) -> Self {
        Self {
            brush_preview: (base.brush_preview != other.brush_preview).then(|| Block3dBrushPreviewSet { value: other.brush_preview.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.brush_preview.is_none()
    }
}

impl protocol::MutationDiff<Block3dWorldWindowTransient> for Block3dWorldWindowTransientDiff {
    fn apply(&self, base: &Block3dWorldWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block3dWorldWindowTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.brush_preview {
            next.brush_preview.clone_from(&value.value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.brush_preview.is_some() {
            self.brush_preview = later.brush_preview;
        }
    }
}
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct Block3dWorldWindowTransientOwner;

impl semio_framework_value::retirement::RetireOwned for Block3dBrushPreview {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(self.position), semio_framework_value::retirement::leaf(self.direction)])
    }
}

impl semio_framework_value::retirement::RetireOwned for Block3dWorldWindowTransient {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.brush_preview)
    }
}

impl semio_framework_value::retirement::RetireOwned for Block3dWorldWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self::SetBrushPreview(mutation) = self;
        semio_framework_value::retirement::RetireOwned::retirement(mutation.preview)
    }
}

#[expect(clippy::unnecessary_wraps, reason = "ArtifactEphemeralTransferPreparationFactory requires a fallible footprint callback")]
fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<Block3dWorldWindowTransientMutation>()))
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
        let state: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::State>> = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>> = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preview_footprint, preview_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
