//! 🖌️ Concrete Block3d world-window brush preview state.

pub const WINDOW_KIND_ID: &str = crate::editor::block3d::modes::edit::windows::world::BLOCK3D_WINDOW_WORLD;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "block.3dworldwindowtransient")]
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

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Block3d world-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Block3dWorldWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl protocol::MutationDiff<Block3dWorldWindowTransient> for Block3dWorldWindowTransient {
    fn apply(&self, _base: &Block3dWorldWindowTransient) -> protocol::MutationApplyResult<Block3dWorldWindowTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct Block3dWorldWindowTransientOwner;

impl store::retirement::RetireOwned for Block3dBrushPreview {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![store::retirement::leaf(self.position), store::retirement::leaf(self.direction)])
    }
}

impl store::retirement::RetireOwned for Block3dWorldWindowTransient {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::RetireOwned::retirement(self.brush_preview)
    }
}

impl store::retirement::RetireOwned for Block3dWorldWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::SetBrushPreview(mutation) = self;
        store::retirement::RetireOwned::retirement(mutation.preview)
    }
}

#[expect(clippy::unnecessary_wraps, reason = "ArtifactEphemeralTransferPreparationFactory requires a fallible footprint callback")]
fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<Block3dWorldWindowTransientMutation>() })
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
        let state: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::State>> = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>> = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preview_footprint, preview_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
