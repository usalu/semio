//! 🫧️ Jack app-local transient state and its typed mutation channel.

use crate::ast::QueryResult;

#[path = "../🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🦀️.rs"]
mod editor_window;
pub use editor_window::{JackEditorSelection, JackEditorWindowTransient, JackEditorWindowTransientMutation, JackEditorWindowTransientOwner, SetEditorSelection, WINDOW_KIND_ID as JACK_EDITOR_WINDOW_KIND_ID};

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jacktransient")]
#[dsl(layout = "lines")]
pub struct JackTransient {
    pub query_execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub query_error: Option<String>,
}

impl store::ArtifactDsl for JackTransient {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Jack transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackTransient {
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

impl protocol::MutationDiff<JackTransient> for JackTransient {
    fn apply(&self, _base: &JackTransient) -> protocol::MutationApplyResult<JackTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct JackTransientPreparationFactory;

struct JackTransientPreparation {
    request: Option<store::ArtifactEphemeralOneItemPreparationRequest<JackTransient, JackTransientMutation>>,
    prepared: Option<store::ArtifactEphemeralOneItemPrepared<JackTransient>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactEphemeralOneItemPreparationFactory<JackTransient, JackTransientMutation> for JackTransientPreparationFactory {
    fn preflight(&self, mutation: &JackTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let retained_bytes = protocol::OpBinary::encode_op(mutation).map_err(|error| error.to_string())?.len();
        let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
        footprint.is_admissible().then_some(footprint).ok_or_else(|| "Jack transient mutation exceeds the one-item publication bound".into())
    }

    fn begin(
        &self,
        request: store::ArtifactEphemeralOneItemPreparationRequest<JackTransient, JackTransientMutation>,
    ) -> Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<JackTransient, JackTransientMutation>>, store::ArtifactEphemeralOneItemPreparationRequest<JackTransient, JackTransientMutation>> {
        let retained_bytes = protocol::OpBinary::encode_op(&request.mutation).map(|bytes| bytes.len()).unwrap_or(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        Ok(Box::new(JackTransientPreparation { request: Some(request), prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false, closing: false }))
    }
}

impl store::ArtifactEphemeralOneItemPreparation<JackTransient, JackTransientMutation> for JackTransientPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || !grant.permits_one() || grant.maximum_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_none() {
            let request = self.request.take().ok_or_else(|| "Jack transient preparation lost its request".to_string())?;
            let outcome = protocol::Mutation::diff(&request.mutation, request.base.as_ref());
            if outcome.worst_level().is_some_and(|level| level >= dsl::Severity::Error) {
                return Err("Jack transient mutation was rejected against its captured base".into());
            }
            let next_root = protocol::MutationDiff::apply(outcome.diff(), request.base.as_ref()).map_err(|error| error.to_string())?;
            self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: [0; 32] };
        }
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactEphemeralOneItemPrepared<JackTransient>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactEphemeralOneItemPrepared<JackTransient>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < self.retained_bytes {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.request.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request.is_none() && self.prepared.is_none()
    }
}

struct JackTransientRootRetirement {
    root: Option<std::sync::Arc<JackTransient>>,
    retained_bytes: usize,
}

impl store::ErasedSnapshotRetirement for JackTransientRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes < self.retained_bytes {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.root.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}

pub struct JackTransientRootRetirementFactory;

impl store::SnapshotRetirementFactory<JackTransient> for JackTransientRootRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<JackTransient>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Box::new(JackTransientRootRetirement { root: Some(snapshot), retained_bytes })
    }
}

#[derive(Default)]
pub struct JackTransientStoreDisposer {
    retired: Option<store::TransientStore<JackTransient, JackTransientMutation>>,
    retained_bytes: usize,
    terminal_root: Option<std::sync::Weak<JackTransient>>,
    terminal_generation: u64,
}

impl semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<JackTransient, JackTransientMutation>> for JackTransientStoreDisposer {
    fn close_step(&mut self, owner: &mut store::TransientStore<JackTransient, JackTransientMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_plugin::Fault> {
        if self.terminal_root.is_none() {
            if maximum_items == 0 {
                return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.retained_bytes = store::ArtifactDsl::print_dsl(owner.current_root().as_ref()).len();
            self.retired = Some(std::mem::replace(owner, store::TransientStore::new(JackTransient::default())));
            self.terminal_root = Some(std::sync::Arc::downgrade(&owner.current_root()));
            self.terminal_generation = owner.generation_now();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retired.is_some() {
            if maximum_items == 0 || maximum_bytes < self.retained_bytes {
                return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.retired = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        self.owns_terminal(owner).then_some(semio_framework_plugin::PluginCloseStep::Complete).ok_or_else(|| semio_framework_plugin::Fault::from("Jack transient terminal owner changed during disposal"))
    }

    fn terminal_is_empty(&self, owner: &store::TransientStore<JackTransient, JackTransientMutation>) -> bool {
        self.retired.is_none() && self.owns_terminal(owner)
    }
}

impl JackTransientStoreDisposer {
    fn owns_terminal(&self, owner: &store::TransientStore<JackTransient, JackTransientMutation>) -> bool {
        owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().and_then(std::sync::Weak::upgrade).is_some_and(|root| std::sync::Arc::ptr_eq(&root, &owner.current_root()))
    }
}
