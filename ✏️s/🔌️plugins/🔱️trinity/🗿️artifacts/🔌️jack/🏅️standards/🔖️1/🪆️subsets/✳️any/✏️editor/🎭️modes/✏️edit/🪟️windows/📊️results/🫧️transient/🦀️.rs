//! 📊️ Ephemeral execution output for one concrete Jack results window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::JackResultsWindowTransient;

impl store::ArtifactDsl for JackResultsWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, rest)| rest);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackResultsWindowTransient {
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

impl protocol::MutationDiff<JackResultsWindowTransient> for JackResultsWindowTransient {
    fn apply(&self, _base: &JackResultsWindowTransient) -> protocol::MutationApplyResult<JackResultsWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

struct JackResultsWindowTransientRetirement {
    children: std::mem::ManuallyDrop<Vec<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackResultsWindowTransientRetirement {
    fn new(execution_id: Option<String>, result: Option<crate::ast::QueryResult>, error: Option<String>) -> Self {
        let mut children = vec![store::retirement::owned_retirement((execution_id, error))];
        if let Some(result) = result {
            let crate::ast::QueryResult { kind: _, columns, rows, graph_fixture } = result;
            children.push(store::retirement::owned_retirement((columns, rows)));
            if let Some(snapshot) = graph_fixture {
                children.push(store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::standards::v1::subsets::any::schema::wire_runtime::JackSnapshotRetirementFactory, snapshot));
            }
        }
        Self { children: std::mem::ManuallyDrop::new(children) }
    }
}

impl store::ErasedSnapshotRetirement for JackResultsWindowTransientRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(child) = self.children.last_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match child.close_step(maximum_items, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if child.terminal_is_empty() => {
                drop(self.children.pop());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err("Jack results-window child retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl Drop for JackResultsWindowTransientRetirement {
    fn drop(&mut self) {
        assert!(self.children.is_empty(), "Jack results-window state reached Drop before exact retirement");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.children) };
    }
}

struct JackResultsWindowTransientRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<JackResultsWindowTransient> for JackResultsWindowTransientRetirementFactory {
    fn retire_owned(&self, value: JackResultsWindowTransient) -> Box<dyn store::ErasedSnapshotRetirement> {
        let JackResultsWindowTransient { query_execution_id, result, query_error } = value;
        Box::new(JackResultsWindowTransientRetirement::new(query_execution_id, result, query_error))
    }
}

impl store::ArtifactOwnedValueRetirementFactory<JackResultsWindowTransientMutation> for JackResultsWindowTransientRetirementFactory {
    fn retire_owned(&self, value: JackResultsWindowTransientMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        let JackResultsWindowTransientMutation::ReplaceQueryResult(value) = value;
        Box::new(JackResultsWindowTransientRetirement::new(value.execution_id, value.result, value.error))
    }
}

fn results_window_transient_footprint(mutation: &JackResultsWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let JackResultsWindowTransientMutation::ReplaceQueryResult(value) = mutation;
    let retained_bytes = std::mem::size_of::<JackResultsWindowTransient>()
        .checked_add(value.execution_id.as_ref().map_or(0, String::len))
        .and_then(|bytes| bytes.checked_add(value.error.as_ref().map_or(0, String::len)))
        .and_then(|bytes| bytes.checked_add(value.result.as_ref().map_or(0, |_| crate::executor::QUERY_OUTPUT_MAXIMUM_BYTES)))
        .ok_or_else(|| "Jack results-window footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Jack results-window transient exceeds its retained publication envelope".into())
}

fn results_window_transient_transfer(mutation: JackResultsWindowTransientMutation) -> JackResultsWindowTransient {
    let JackResultsWindowTransientMutation::ReplaceQueryResult(value) = mutation;
    JackResultsWindowTransient { query_execution_id: value.execution_id, result: value.result, query_error: value.error }
}

#[cfg(test)]
#[path = "🧪️tests/🧩️bounded-publication/🦀️.rs"]
mod bounded_publication_tests;

pub struct JackResultsWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for JackResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_RESULTS;
    type State = JackResultsWindowTransient;
    type Mutation = JackResultsWindowTransientMutation;

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(JackResultsWindowTransientRetirementFactory);
        let mutation = std::sync::Arc::new(JackResultsWindowTransientRetirementFactory);
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(results_window_transient_footprint, results_window_transient_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<JackResultsWindowTransientOwner>()
}

pub fn addressed(window_id: &str, view: &semio_framework_plugin::ViewModel, mutation: JackResultsWindowTransientMutation) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, WindowTransientMutation};
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_RESULTS) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.results-window-required"), "Jack query output requires the explicit attached results window"));
    }
    Ok(WindowTransientMutation::of::<JackResultsWindowTransientOwner>(window_id, mutation))
}
