//! 🔌️ Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM plugins.

pub use semio_framework_os_kernel::io;

/// 🪪️ Runs one test body with a local native authoring identity: `with_authoring_identity!(|identity| body)` binds `identity` as a mutable
/// [`EntityIdentityAuthority`](semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority) for `body`, an expression that may await.
#[cfg(any(test, feature = "artifact-app-testing"))]
#[macro_export]
macro_rules! with_authoring_identity {
    (|$identity:ident| $body:expr) => {{
        let mut __observer = |_: $crate::__value::native_encoding::NativeEncodeProgress| true;
        let __observer: &mut $crate::__kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut __observer;
        let mut $identity = $crate::__kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, __observer).expect("test authoring identity admission");
        $body
    }};
}
pub use semio_framework_plugin_mounted_owner::{MountedOwnerIdentityV1, MountedOwnerJobReceiptV1, MountedOwnerPhaseV1, MountedOwnerPolicyV1, MountedOwnerTurnV1};

#[cfg(test)] use semio_framework_ui_locale::{Label, LabelText, Locale, LocalizedLabel, Terminology};

/// 🗣️ `#[derive(dsl::DslRecord)]` expands to `::dsl::…` paths, so the crate root must bind that name
/// — the same `extern crate semio_framework_os_kernel as dsl;` every technology crate declares.
/// Without it the plugin-owned `world-3d` records (`WorldSunConfig`, `WorldProjectionConfig`) could
/// not implement `DslField`, and every artifact embedding one in a `WindowConfigOwner::State` would
/// have to carry a hand-mirrored copy instead.
extern crate semio_framework_os_kernel as dsl;

#[doc(hidden)]
pub use semio_framework_async as __async;

//#region 🧮️HeapWitness
/// 🧮️ Weighs what ONE reactor turn RETAINS, in this crate's own test binary only. The wasm harness
/// (`🖥️host/🧪️tests/🔬️poll-turn-memory`) measured 4 437 B of guest linear memory per `poll` turn
/// against a live instance — app-independent and render-independent — and the guest has one fixed
/// linear memory with no process to restart. `semio_framework_trace::HeapWitness` is the shared
/// instrument (two relaxed atomics per allocation, TEST BINARIES ONLY) the puzzle-3d and
/// generation3d editors already install; the turn-retention law below reads it.
#[cfg(test)]
#[global_allocator]
static PLUGIN_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
//#endregion 🧮️HeapWitness

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🧪️tests/🧰️ui-action-collection/🦀️.rs"]
pub mod ui_action_laws;

//#region 📄️DeclarationFixtureMutationMount
#[cfg(test)]
#[path = "🧪️tests/🛰️declaration-channels/🦀️.rs"]
pub(crate) mod declaration_fixture_mutations;

#[cfg(test)]
#[path = "🧪️tests/🤝️cooperative-pump/🦀️.rs"]
mod cooperative_pump_tests;

#[cfg(test)]
#[path = "🧪️tests/🌲️fixture-projection/🦀️.rs"]
mod fixture_projection_retirement_tests;

#[cfg(test)]
#[path = "🧪️tests/🏃️plugin-runtime-runtime-laws/🦀️.rs"]
mod plugin_runtime_runtime_laws;
//#endregion 📄️DeclarationFixtureMutationMount

#[doc(hidden)]
#[path = "🧪️tests/🧬️generated-test-contracts/🦀️.rs"]
mod generated_test_contracts;

//#region 🔖️ComponentPersistentLocal
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub mod component_persistent_local {
    use std::{cell::UnsafeCell, mem::MaybeUninit, sync::Once};

    /// 🧵️ Component-async calls receive fresh Rust TLS contexts, so actor state that must survive
    /// from one exported call to the next lives in the component instance's linear-memory global.
    pub struct ComponentPersistentLocal<T> {
        once: Once,
        value: UnsafeCell<MaybeUninit<T>>,
        initialize: fn() -> T,
    }

    impl<T> ComponentPersistentLocal<T> {
        pub const fn new(initialize: fn() -> T) -> Self {
            Self { once: Once::new(), value: UnsafeCell::new(MaybeUninit::uninit()), initialize }
        }

        pub fn with<R>(&'static self, use_value: impl FnOnce(&'static T) -> R) -> R {
            self.once.call_once(|| unsafe {
                (*self.value.get()).write((self.initialize)());
            });
            use_value(unsafe { (&*self.value.get()).assume_init_ref() })
        }
    }

    unsafe impl<T> Sync for ComponentPersistentLocal<T> {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! component_persistent_local {
    ($( $(#[$attribute:meta])* static $name:ident: $value_type:ty = $initialize:expr; )+) => {
        $(
            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            $(#[$attribute])*
            static $name: $crate::component_persistent_local::ComponentPersistentLocal<$value_type> =
                $crate::component_persistent_local::ComponentPersistentLocal::new(|| $initialize);
        )+

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        std::thread_local! {
            $(
                $(#[$attribute])*
                static $name: $value_type = $initialize;
            )+
        }
    };
}
//#endregion 🔖️ComponentPersistentLocal

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub mod owned_abi {
    /// 📋️ Fixed stack authority preserves all original scalar fields and refuses malformed extents.
    pub fn original_job_budget(arguments:[u64;19])->Result<semio_framework::kernel::OriginalJobBudget,semio_framework_value::ValueRefusalKind>{semio_framework::kernel::OriginalJobBudget::from_scalar_arguments(arguments).map_err(|error|error.kind)}

    use semio_framework_value_derive::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct PollInput {
        pub events: Vec<semio_framework::kernel::Event>,
        pub command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
        pub cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
        pub budget: semio_framework::kernel::Budget,
    }

    /// 🌉️ Hand-written, not derived — `events`/`command_page`/`budget` are foreign kernel types
    /// (`semio_framework::kernel::*`, owned by the `semio-framework`/`semio-framework-os-kernel`
    /// crates), so neither `#[derive(ToValue, FromValue)]` nor a direct `impl … for
    /// semio_framework::kernel::Event` is legal here (orphan rule: neither the trait nor the type
    /// is local to this crate). Bridges through the already-derived `Serialize`/`Deserialize` via
    /// `serde_json` instead, reusing the existing `DslValue <-> serde_json::Value` conversion.
    impl semio_framework_value::ToValue for PollInput {
        fn to_value(&self) -> semio_framework_value::DslValue {
            semio_framework_value::DslValue::from(&serde_json::to_value(self).expect("PollInput always serializes"))
        }
    }
    impl semio_framework_value::FromValue for PollInput {
        fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
            serde_json::from_value(serde_json::Value::from(value)).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
        }
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct StartJobInput {
        pub source: Option<semio_framework::kernel::OriginalJobSource>,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct StepJobInput {
        pub job: u64,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CancelJobInput {
        pub job: u64,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct RestoreInput {
        pub state: Vec<u8>,
    }

    /// 🧬️ Input of the owned `codec` core exports. `document_id` is only read by `genesis`, `ops`
    /// only by `apply-ops` (an `encode_ops_vec` batch) and `replay-envelopes` (an `encode_envelopes`
    /// stream), and the pair only by `print-mirror`/`apply-ops`/`replay-envelopes` — one shape for
    /// all five keeps the owned ABI's JSON envelope single-valued per export, exactly as
    /// `RestoreInput` does.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecInput {
        pub artifact_schema: String,
        #[serde(default)]
        pub document_id: String,
        #[serde(default)]
        pub pack: Vec<u8>,
        #[serde(default)]
        pub spr: Vec<u8>,
        #[serde(default)]
        pub ops: Vec<u8>,
    }

    /// 📦️ Output of the owned `codec.genesis`/`codec.apply-ops`/`codec.replay-envelopes` core exports.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecPair {
        pub pack: Vec<u8>,
        pub spr: Vec<u8>,
    }

    /// 📥️ Output of the owned `codec.print-mirror` core export.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecMirror {
        pub dsl: String,
        pub ops: String,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum JobStep {
        Running { progress: Option<Vec<u8>> },
        Done { output: Vec<u8> },
        Failed { error: Vec<u8> },
    }

    pub fn allocate(length: u32) -> u32 {
        let bytes = vec![0_u8; length as usize].into_boxed_slice();
        Box::into_raw(bytes) as *mut u8 as usize as u32
    }

    pub unsafe fn deallocate(pointer: u32, length: u32) {
        if crate::reactor::original_actor_context::exports::fixed_receipt_pointer(pointer,length)||crate::reactor::original_actor_context::exports::release_owned_job_receipt(pointer,length) { return; }
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        drop(unsafe { Box::from_raw(slice) });
    }

    pub unsafe fn take_json<T: serde::de::DeserializeOwned>(pointer: u32, length: u32) -> Result<T, Vec<u8>> {
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        let bytes = unsafe { Box::from_raw(slice) };
        serde_json::from_slice(&bytes).map_err(|error| error.to_string().into_bytes())
    }

    unsafe fn take_actor_json<T: serde::de::DeserializeOwned>(pointer:u32,length:u32)->Result<T,semio_framework_value::ValueRefusalKind>{
        let slice=std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8,length as usize);
        let bytes=unsafe{Box::from_raw(slice)};
        serde_json::from_slice(&bytes).map_err(|_|semio_framework_value::ValueRefusalKind::InvalidValue)
    }

    /// 🌱️ The owned actor admission port never creates dynamic diagnostic prose on malformed input.
    pub unsafe fn take_actor_admission_input(pointer:u32,length:u32)->Result<(semio_framework::kernel::RetainedTurnInput,semio_framework_value::RetainedCloneGrant),semio_framework_value::ValueRefusalKind>{unsafe{take_actor_json(pointer,length)}}

    /// 📬️ Retains the original declared poll fields without an error-text carrier.
    pub unsafe fn take_actor_poll_input(pointer:u32,length:u32)->Result<PollInput,semio_framework_value::ValueRefusalKind>{unsafe{take_actor_json(pointer,length)}}

    pub fn return_json<T: Serialize>(value: &T) -> u64 {
        match serde_json::to_vec(value) {
            Ok(bytes) => return_bytes(bytes),
            Err(error) => return_bytes(error.to_string().into_bytes()),
        }
    }

    pub fn return_bytes(bytes: Vec<u8>) -> u64 {
        let bytes = bytes.into_boxed_slice();
        let length = bytes.len() as u32;
        let pointer = Box::into_raw(bytes) as *mut u8 as usize as u32;
        u64::from(pointer) | (u64::from(length) << 32)
    }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use wasip2::component_export_anchor;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn component_export_anchor() {}

#[cfg(test)]
#[path = "🧪️tests/📢️publication-fixtures/🦀️.rs"]
pub(crate) mod publication_fixture;

#[cfg(test)]
#[path = "🧪️tests/📢️publication-owners/🦀️.rs"]
pub(crate) mod window_transient_owners;

#[cfg(test)]
#[path = "🧪️tests/🖥️test-app-mutations/🦀️.rs"]
pub(crate) mod test_app_mutation_fixture;

#[cfg(test)]
#[path = "🧪️tests/📡️contributed-mutation-wire/🦀️.rs"]
pub(crate) mod contributed_mutation_wire;

#[cfg(test)]
#[path = "🧪️tests/📤️segmented-download/🦀️.rs"]
mod segmented_download_contract;

//#region 🏛️OwnershipFacet
#[path = "🏛️ownership/🦀️.rs"]
pub(crate) mod bounded_ownership;
pub use bounded_ownership::{PluginLifecycleFailure,PluginLifecyclePrimary};
#[path="👷️job/🚪️session/🦀️.rs"]
pub(crate) mod reserved_job_session;
#[path = "📤️fault/🦀️.rs"]
pub(crate) mod sdk_fault;
pub use semio_framework_job::StepContext;
#[path = "🧬️schema/🪶️sqlite/🦀️.rs"]
pub mod sqlite_wire;
#[path = "♻️lifecycle/🦀️.rs"]
pub(crate) mod store_lifecycle;
#[path = "🫧️transient/🧵️publication/🦀️.rs"]
pub(crate) mod transient_publication;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[path = "🧬️schema/🦀️.rs"]
pub mod wasip2;
//#endregion 🏛️OwnershipFacet
//#region 🪟️WindowFacet
#[path = "🪟️window-kits/📃️document/🦀️.rs"]
pub(crate) mod document_kit;
#[path = "🪟️window/🎛️engagement/🦀️.rs"]
pub mod engagement;
#[path = "🪟️window-kits/🖼️image/🦀️.rs"]
pub(crate) mod image_kit;
#[path = "🕹️interaction/🧭️topology/🦀️.rs"]
pub(crate) mod interaction_topology;
#[path = "🕹️interaction/🦀️.rs"]
pub(crate) mod local_interaction;
#[path = "🪟️window-kits/🎬️media/🦀️.rs"]
pub(crate) mod media_kit;
#[path = "🪟️window-kits/🧊️mesh/🦀️.rs"]
pub(crate) mod mesh_kit;
#[path = "🪟️window-kits/📊️table/🦀️.rs"]
pub(crate) mod table_kit;
#[path = "🪟️window-kits/📝️text/🦀️.rs"]
pub(crate) mod text_kit;
#[path = "🪟️window-kits/🌳️tree/🦀️.rs"]
pub(crate) mod tree_kit;
#[path = "🪟️window/🎚️config/🦀️.rs"]
pub(crate) mod window_config;
#[path = "🪟️window-kits/🦀️.rs"]
pub(crate) mod window_kits;
#[path="🪟️window/📬️mutation/🦀️.rs"]
pub(crate) mod window_mutation;
#[path = "🪟️window/🦀️.rs"]
pub(crate) mod window_surface;
#[path = "🪟️window/🫧️transient/🦀️.rs"]
pub(crate) mod window_transient;
#[path = "🪟️window-kits/🌍️world3d/🦀️.rs"]
pub mod world3d_host;
//#endregion 🪟️WindowFacet
//#region 📜️DeclarationFacet
#[path = "🏗️builder/🧩️app/🦀️.rs"]
pub(crate) mod app_builder;
#[path = "📜️declaration/🧩️app/🦀️.rs"]
pub(crate) mod app_contract;
#[path = "📜️declaration/📨️emission/🦀️.rs"]
pub(crate) mod app_emission;
#[path = "📜️declaration/👁️view/🦀️.rs"]
pub(crate) mod app_views;
#[path = "📜️declaration/🦀️.rs"]
pub(crate) mod artifact_declaration;
#[path = "🪪️identity/🦀️.rs"]
pub(crate) mod artifact_identity;
#[path = "💡️inference/🦀️.rs"]
pub(crate) mod artifact_inference;
pub use artifact_inference::{ArtifactInferenceGatewayFailure,ArtifactInferenceGatewayRefusal};
#[path = "🚪️io/🦀️.rs"]
pub(crate) mod artifact_io;
#[path = "📜️declaration/📇️registration/🦀️.rs"]
pub(crate) mod artifact_registration;
#[path="🧩️composition/📨️emission/📦️preparation/🦀️.rs"]
pub(crate) mod child_emit_preparation;
#[path = "📜️declaration/🪆️child/🦀️.rs"]
pub(crate) mod child_view;
#[path="📬️completion/⚠️fault/🦀️.rs"]
pub(crate) mod completion_fault;
#[path="📬️completion/♻️retirement/🚫️refusal/🦀️.rs"]
pub(crate) mod completion_refusal_retirement;
#[path="📬️completion/♻️retirement/🦀️.rs"]
pub(crate) mod completion_retirement;
#[path = "📜️declaration/🛠️tool-machine/🦀️.rs"]
pub(crate) mod gesture_contract;
#[path = "📜️declaration/🎮️command/🦀️.rs"]
pub(crate) mod history_view;
// 🧬️ A2 (design-abi.md §4): `host_port` is deleted — see `important.md`'s "Replace, never wrap"
// list (`host_port` · `component::host_*` · `set_host_backbone_channel` are all named explicitly).
// Replaced by `🌐host/🦀️.rs`'s async `Host`/`log`/`now_ms`/`trace_span` — see that
// module and the report's "peer-coexistence"/deferred-work sections for the backbone channel gap.
#[path = "🌐host/🦀️.rs"]
pub mod host;
#[path = "📜️declaration/🎞️media/🦀️.rs"]
pub(crate) mod media_contract;
#[path = "📜️declaration/📥️natural-file/🦀️.rs"]
pub(crate) mod natural_file_import;
#[path = "🧵️retained-command/🧬️context/🦀️.rs"]
pub(crate) mod owned_context;
#[path = "👥️presence/♻️retirement/🦀️.rs"]
pub(crate) mod presence_retirement;
pub use presence_retirement::{
    bounded_presence_root_retirement_factory, bounded_presence_store_disposer, no_presence_local_root_retirement_factory, no_presence_peer_retirement_factory, no_presence_store_disposer, NoPresenceRetirementFactory, PresenceStoreOwnedDisposer,
};
#[path = "📜️declaration/🪟️surface/🦀️.rs"]
pub(crate) mod surface_contract;
#[path = "♻️cancellation/🦀️.rs"]
pub(crate) mod tool_cancellation;
#[path = "📜️declaration/👷️job/🦀️.rs"]
pub(crate) mod tool_contract;
#[path = "📜️declaration/⏯️tool-run/🦀️.rs"]
pub(crate) mod tool_run_contract;
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
#[path = "🪆️child/🦀️.rs"]
pub(crate) mod child_content;
#[path = "📡️backbone/🔗️binding/🦀️.rs"]
pub mod document_backbone_binding;
#[path = "🗃️document/🦀️.rs"]
pub(crate) mod document_ingress;
#[path = "🎚️config/🚫️none/♻️retirement/🦀️.rs"]
pub(crate) mod no_config_retirement;
pub use no_config_retirement::{no_config_store_disposer, no_config_store_owners};
#[path = "📝️draft/🚫️none/♻️retirement/🦀️.rs"]
pub(crate) mod no_draft_retirement;
pub use no_draft_retirement::{no_draft_store_disposer, no_draft_store_owners};
#[path = "👥️peer/🦀️.rs"]
pub(crate) mod peer_roster;
#[path="🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs"]
pub(crate) mod private_child_input;
#[path = "🧩️composition/📬️publication/🦀️.rs"]
pub(crate) mod store_publication;
#[path = "🫧️transient/♻️retirement/🦀️.rs"]
pub(crate) mod transient_retirement;
pub use transient_retirement::{no_transient_local_root_retirement_factory, no_transient_store_disposer, NoTransientRetirementFactory};
//#endregion 📬️PublicationFacet
//#region 👷️JobsFacet
#[path = "🎮️command/📥️admission/🦀️.rs"]
pub(crate) mod command_admission;
#[path = "🎮️command/🦀️.rs"]
pub(crate) mod command_history;
#[path="📬️completion/♻️effects/🦀️.rs"]
pub(crate) mod completion_effects;
pub use completion_effects::CompletionEffect;
#[path="📬️completion/♻️vacant/🦀️.rs"]
pub(crate) mod completion_vacant;
pub use completion_vacant::{VacantVec,VacantDeque};
#[path="🧩️extension/📨️invoke/⚠️failure/🦀️.rs"]
pub(crate) mod extension_invocation_failure;
#[path = "🎞️media/🦀️.rs"]
pub(crate) mod media_export;
#[path="🎞️media/⚠️turn/🦀️.rs"]
pub(crate) mod media_operation_refusal;
#[cfg(feature="media-turn-testing")]
pub use media_operation_refusal::tests as media_turn_laws;
#[cfg(feature="reserved-producer-testing")]
pub use tool_contract::original_reserved_producer_laws;
#[path="🚪️io/🛂️authority/🦀️.rs"]
pub mod operation_authority;
#[path="👷️job/🧾️bytes/🦀️.rs"]
pub(crate) mod reserved_job_bytes;
#[path="👷️job/📤️outcomes/🦀️.rs"]
pub(crate) mod reserved_job_outcomes;
#[path = "👷️job/🏛️reserved/🦀️.rs"]
pub(crate) mod reserved_jobs;
#[path = "🧵️retained-command/🦀️.rs"]
pub mod retained_command;
#[path = "👷️job/🦀️.rs"]
pub(crate) mod tool_jobs;
#[path = "🧵️retained-command/🔄️full-operation/🦀️.rs"]
pub(crate) mod typed_operation;
//#endregion 👷️JobsFacet
//#region ⚛️ReactorFacet
#[path = "🛂️describe/🦀️.rs"]
pub mod describe;
#[path="💡️inference/🚪️gateway/🦀️.rs"]
pub(crate) mod inference_gateway;
pub use inference_gateway::{cancel_artifact_inference,wire_artifact_infer_cold,ArtifactInferenceGateway,ArtifactInferenceGatewayStep,wire_artifact_infer,original_inference_gateway_demands};
#[path = "⚛️reactor/🔌️app/🦀️.rs"]
pub(crate) mod plugin_app;
#[path = "🏗️builder/🔌️program/🦀️.rs"]
pub(crate) mod plugin_program;
#[path = "⚛️reactor/🏃️runtime/🦀️.rs"]
pub mod plugin_runtime;
#[path = "⚛️reactor/🦀️.rs"]
pub mod reactor;
//#endregion ⚛️ReactorFacet
//#region 🧵️InstanceFacet
#[path = "🧵️instance/🦀️.rs"]
pub(crate) mod artifact_instance;
#[path = "🏗️builder/🦀️.rs"]
pub(crate) mod builder;
#[path = "📜️declaration/🌳️tree/🦀️.rs"]
pub mod declarations;
//#endregion 🧵️InstanceFacet
//#region ⚖️AppLaws
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🧪️testing/🦀️.rs"]
pub(crate) mod app_testing;
//#endregion ⚖️AppLaws

pub mod app {
    //! 🧩️ The crate's public app vocabulary: declarative app builder, plugin traits and runtime types, composed from the facet modules that own them.
    pub use crate::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
    pub use super::child_emit_preparation::{ChildEmitPreparation,ChildEmitPreparationStep,OwnedChildEmit};
    pub use super::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, bounded_transient_store_disposer, transient_store_disposer};
    pub use super::window_config::{
        bounded_window_config_preparation_factory, bounded_window_config_store_disposer, bounded_window_config_store_owners, RejectedWindowConfigEmission, WindowConfigMutation, WindowConfigOwner, WindowConfigOwnerRegistry, WindowConfigPack,
        WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep, WindowConfigApplyCursor, WindowConfigApplyEdit, WindowConfigApplyMutation, WindowConfigReplaceCursor, WindowConfigReplaceEdit, WindowConfigReplaceMutation, WindowConfigSnapshot,
    };
    pub use super::window_transient::{TransientView, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientOwnerRegistry, WindowTransientSnapshot};
    /// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
    pub use semio_framework::AppDefinition;
    pub use semio_framework_ui_contract::BuiltNode;
    pub use semio_framework_ui_runtime::{ComponentTree, TreeNode};
    /// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
    /// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
    /// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
    /// `semio_framework_os`'s artifact catalog registry share one definition.
    pub use semio_framework::{ArtifactKindSpec, OsMediaCapability};
    /// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
    /// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
    /// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
    /// The legacy format enum was retired in ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
    /// Binary` now carries a plain format kind id string.
    pub use semio_framework::MediaWireFormat;
    /// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
    /// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
    /// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
    pub use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
    /// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
    /// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
    /// without a direct `semio-framework-core` dependency.
    pub use semio_framework::{MediaClass, MediaType};
    /// ⚠️ IO error alias from the framework media stack.
    pub use semio_framework::mesh_io::IoError;
    /// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
    /// 2). Defined in `semio_framework` so plugins and the OS product
    /// share one definition without an inverted dependency; re-exported here verbatim.
    pub use {semio_framework_os_kernel::io::io_compose_via,semio_framework_os_kernel::io::dialects_for,semio_framework_os_kernel::io::io_dispatch,semio_framework_os_kernel::io::io_keys_for,semio_framework_os_kernel::io::resolve,semio_framework_os_kernel::io::list_composer_entries,semio_framework_os_kernel::io::register_composer_entries,semio_framework_os_kernel::io::register_subset_validator,semio_framework_os_kernel::io::set_io_fallback_dispatcher,semio_framework_os_kernel::io::subset_validator_entry_of,semio_framework_os_kernel::io::wire_artifact_compose,semio_framework_os_kernel::io::wire_decode_composed_artifact,semio_framework_os_kernel::io::wire_list_composer_entries,semio_framework_os_kernel::io::Analysis,semio_framework_os_kernel::io::AnalyzeSource,semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::AsyncComposeFn,semio_framework_os_kernel::io::ComposeError,semio_framework_os_kernel::io::ComposeFuture,semio_framework_os_kernel::io::ComposeSource,semio_framework_os_kernel::io::ComposedArtifact,semio_framework_os_kernel::io::ComposerEntry,semio_framework_os_kernel::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_os_kernel::io::ErasedComposeSource,semio_framework_os_kernel::io::IoDirection,semio_framework_os_kernel::io::IoFallback,semio_framework_os_kernel::io::IoFallbackDispatcher,semio_framework_os_kernel::io::IoKey,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::IoResolveError,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_os_kernel::io::SubsetValidator,semio_framework_os_kernel::io::SubsetValidatorEntry,semio_framework_os_kernel::io::WireComposeSource,semio_framework_os_kernel::io::WireComposedArtifact};
    pub use super::inference_gateway::wire_artifact_infer;
    /// 🧾️ The body-wide node budget and its fixed-node reserve live ONCE, in the UI contract, because
    /// the React host spends the same number from the other side of the wire
    /// (`🌳️Tree/🟦️.tsx`'s `capTreeWindowRequests`). See
    /// [`semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET`] for the cost model both sides
    /// implement; [`TreeWindows`] is this side of it.
    pub use semio_framework_ui_contract::{TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_FIXED_NODE_HEADROOM, TREE_WINDOW_PATH_SEPARATOR};
    pub use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
    pub use crate::local_interaction::mutations::{InteractionConfigMutation, SetInteractionState};
    /// 🙋️ Explicit actor identity for standalone local births and fixtures.
    pub use protocol::LOCAL_ACTOR_ID;
    pub use super::completion_fault::ArtifactCompletionFault;
    pub use store::SnapshotReadProjection as ArtifactContextRoot;
    pub use super::owned_context::ArtifactOwnedContextHandle;
    pub use store::ArtifactStoreInitializationAuthority;
    pub use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
    pub use super::builder::{NeedsLabel, NeedsVersion, PluginBuilder, Ready};
    pub use super::bounded_ownership::*;
    pub use super::sdk_fault::*;
    pub use super::document_kit::*;
    pub use super::image_kit::*;
    pub use super::media_kit::*;
    pub use super::mesh_kit::*;
    pub use super::table_kit::*;
    pub use super::text_kit::*;
    pub use super::tree_kit::*;
    pub use super::window_kits::*;
    pub use super::window_surface::*;
    pub use super::app_builder::*;
    pub use super::app_contract::*;
    pub use super::app_emission::*;
    pub use super::app_views::*;
    pub use super::artifact_declaration::*;
    pub use super::artifact_identity::*;
    pub use super::artifact_inference::*;
    pub use super::artifact_io::*;
    pub use super::artifact_registration::*;
    pub use super::surface_contract::*;
    pub use super::child_view::*;
    pub use super::gesture_contract::*;
    pub use super::history_view::*;
    pub use super::media_contract::*;
    pub use super::tool_contract::*;
    pub use super::tool_run_contract::*;
    pub use super::child_content::*;
    pub use super::document_ingress::*;
    pub use super::peer_roster::*;
    pub use super::store_lifecycle::*;
    pub use super::store_publication::*;
    pub use super::command_history::*;
    pub use super::reserved_jobs::*;
    pub use super::tool_cancellation::*;
    pub use super::tool_jobs::*;
    pub use super::artifact_instance::*;
    pub use super::plugin_app::*;
    pub use super::plugin_program::*;
    pub use super::declarations;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub use super::app_testing::*;
}

/// 🧩️ Cross-crate closure vocabulary required by `dyn_enum_close!` for `PluginApp`.
pub mod plugin_app_close_prelude {
    pub use crate::app::*;
    pub use crate::app::{tree_item, PeerMark};
    pub use semio_framework::kernel::ActionId;
    pub use semio_framework::kernel::*;
    pub use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandInvocation as ManifestCommandInvocation};
    pub use semio_framework::*;
    pub use semio_framework_os_kernel as protocol;
    pub use semio_framework_os_kernel as store;
    pub use semio_framework_ui_contract::tree_item as tree_item_builder;
    pub use semio_framework_ui_contract::*;
    pub use semio_framework_ui_runtime::{ComponentTree, TreeNode};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
    pub use ui_wgpu::wgpu::{ContextMenuItemSpec, ContextMenuRequest, WindowEngagement, WindowMeasure};
}

#[cfg(any(test, feature = "artifact-app-testing"))]
pub use app::artifact_app_laws;
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use app::authoring_law;
pub use app::time_travel::{history_mutation_entry, HistoryMutationPage, HistoryMutationPages, SupersedeLedger, SupersedeRecord, SupersedeRole, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelEditor, TimeTravelEditorPanel, TimeTravelLedger, TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};
pub use app::tool_run::{is_tool_run_action_id, ToolRunActionOutcome, ToolRunDriver, ToolRunLedger, ToolRunTickReceipt};
pub use app::ActionFactory;
pub use app::{FRAMEWORK_TOOL_RUN_BODY_KEY, OriginalToolRunJobPortRetirement, ToolRunJob, ToolRunJobPort, ToolRunJobPurpose, ToolRunJobRequest, ToolRunRetargetableJob, ToolRunTraceKeys, ToolRunView, is_time_travel_action_id};
pub use app::{AsyncTaskSource,AsyncTaskRun};
pub use app::{activation_target, resolve_labels, row_action, row_target, selection_ids, tree_group, tree_item, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, LabelAxes};
pub use app::{
    artifact_app_apply_ops,
    artifact_app_genesis_pair,
    artifact_app_replay_envelopes,
    artifact_inference_service,
    artifact_pair_snapshot,
    bounded_config_store_disposer,
    bounded_config_store_one_item_preparation_factory,
    bounded_config_store_owners,
    bounded_document_store_disposer,
    bounded_document_store_initialization_job,
    bounded_document_store_owners,
    bounded_transient_preparation_factory,
    bounded_transient_root_retirement_factory,
    bounded_transient_store_disposer,
    bounded_window_config_preparation_factory,
    bounded_window_config_store_disposer,
    bounded_window_config_store_owners,
    built_text_node,
    built_text_to_component_tree,
    built_to_component_tree,
    built_to_tree,
    composer_entry_of,
    delete_selection,
    deserializer_entry_of,
    history_alternative_views,
    infer_artifact,
    inference_child,
    inference_child_dependency,
    inference_child_dependency_parts,
    list_artifact_inference_services,
    node_graph_delete_selection_spec,
    nothing_selected,
    paged_text_carrier,
    publish_document_store_candidate_if_authoritative,
    register_artifact_inference_service,

    scene_surface,
    section_component_tree,
    selection_count_phrase,
    selection_domains_from_surface,
    serializer_entry_of,
    transient_store_disposer,
    wire_list_artifact_inference_services,
    ActionMeta,
    AlternativeFact,
    AlternativeView,
    App,
    AppActionRegistry,
    AppBuilder,
    AppInstance,
    AppOperationContext,
    AppRenderOperationContext,
    ArtifactAnalysis,
    ArtifactAnalyzer,
    ArtifactApp,
    ArtifactBoundedFirstStepProof,
    ArtifactBuilder,
    ArtifactCapability,
    NaturalFileCodec,
    NaturalFileDecodeCursor,
    NaturalFileDecodeOwner,
    NaturalFileDecodeProgress,
    NaturalFileDecodeStep,
    ArtifactCapabilityKind,
    ArtifactCapabilityRow,
    ArtifactChildren,
    ArtifactComposer,
    ArtifactComposition,
    ArtifactDeclaration,
    ArtifactDecomposer,
    ArtifactDefinition,
    ArtifactDefinitionError,
    ArtifactDefinitionRegistry,
    ArtifactDeserializer,
    ArtifactDocumentPayload,
    ArtifactDocumentStoreDisposer,
    ArtifactDownloadOutput,
    ArtifactEditor,
    ArtifactEnvelopeDecodeOperationHandle,
    ArtifactEnvelopeDecodeOperationPoll,
    ArtifactExecutableIdentity,
    ArtifactIdentity,
    ArtifactIdentityClaim,
    ArtifactIdentityNamespace,
    ArtifactInference,
    ArtifactInferenceDemands,
    ArtifactInferenceCommitBinding,
    ArtifactInferenceDocumentBinding,
    ArtifactInferenceExecution,
    ArtifactInferenceExecutionStep,
    ArtifactInferenceStepContext,
    ArtifactInferenceExecutionError,
    ArtifactInferenceExecutionRequest,
    ArtifactInferencePayloadContract,
    ArtifactInferenceRegistrationError,
    ArtifactInferenceService,
    ArtifactInferenceServiceKey,
    ArtifactInferenceServiceMetadata,
    ArtifactInferenceServiceRegistry,
    ArtifactInstanceOperationOwner,
    ArtifactInstanceOperationOwnerHandle,
    ArtifactInferenceSessionAccess,
    ArtifactInstanceOperationOwnerAliasRetirement,
    ArtifactKindSpec,
    ArtifactLocale,
    ArtifactLocalization,
    ArtifactMediaExportCompletion,
    ArtifactMediaExportCredit,
    ArtifactMediaExportHandle,
    ArtifactMediaExportHandleLoan,
    ArtifactMediaExportJobRequest,
    ArtifactMediaExportPoll,
    ArtifactMediaExportResult,
    ArtifactMime,
    ArtifactOutputChunks,
    ArtifactOwnedDisposer,
    ArtifactOwnedToolJobFactory,
    ArtifactOwnedToolJobRequest,
    ArtifactReservedJob,
    ArtifactReservedToolInput,
    ArtifactReservedToolJob,
    ArtifactReservedToolJobRequest,
    ArtifactRuntimeCapabilityRequirement,
    ArtifactSerializer,
    ArtifactSnapshotCloseLease,
    ArtifactStoreInitializationAuthority,
    ArtifactStoreInitializationJob,
    ArtifactToolCompletion,
    ArtifactToolCompletionValue,
    ArtifactToolCompletionRejection,
    ArtifactDownloadCompletionRejection,
    ArtifactBoundedToolFault,
    ArtifactCompletionFault,
    ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract,
    ArtifactToolPublicationLane,
    ArtifactView,
    ArtifactViewer,
    AsyncComposeFn,
    BuiltNode,
    // 🧸️👥️🫧️ Composition child-read seam plus the two ephemeral state lanes.
    ChildContentView,
    ChildContentReadRetirement,
    ComponentTree,
    ComposeFuture,
    Confidence,
    ConfigView,
    DecomposeSource,
    Decomposition,
    DerivedArtifactAnalyzer,
    DerivedArtifactBuilder,
    DerivedArtifactComposer,
    DerivedArtifactParts,
    DerivedArtifactSpec,
    Dialect,
    DraftView,
    Editor,
    EditorApp,
    Emit,
    EphemeralEmit,
    ExampleSource,
    ExampleSourceBody,
    ExtensionInvocation,
    FlowExtensionContribution,
    FlowExtensionContributionKind,
    FlowExtensionDeclaration,
    FlowExtensionDescriptor,
    FlowExtensionExecutableIdentity,
    FlowExtensionManifest,
    HistoryView,
    HostEvent,
    HostMediaExecutableIdentity,
    HostMediaHandlerDeclaration,
    HostMediaHandlerDescriptor,
    HostMediaHandlerKind,
    HostMediaRuntimeError,
    InteractionView,
    InteractionWrite,
    KeybindingSpec,
    MediaClass,
    MediaType,
    Menu,
    MeshDocumentImporter,
    MeshImportRequest,
    MeshImportResult,
    MeshView,
    MeshWindowKit,
    ModeSpec,
    NoChildren,
    NoConfig,
    NoConfigMutation,
    NoDraft,
    NoDraftMutation,
    NoPresence,
    NoPresenceMutation,
    NoTransient,
    NoTransientMutation,
    NodeGraphDeleteDispatch,
    OsMediaCapability,
    OwnedDocumentMemberIngress,
    PanelTabSpec,
    PanelTreeBuilder,
    Plugin,
    PluginApp,
    PluginAssemblyError,
    OriginalRenderFailure,
    OriginalRenderResult,
    PluginBuilder,
    PluginLifecycleStep,
    PluginProgram,
    PresenceView,
    SelectionKind,
    StandardId,
    SubsetId,
    ToolCommandCatalog,
    TransientView,
    TreeNode,
    TwoDSvgDocumentRenderer,
    TwoDSvgExportRequest,
    TwoDSvgExportResult,
    UiAssemblyResult,
    VcsArtifactApp,
    ViewEmit,
    Viewer,
    ViewerApp,
    WindowConfigApplyCursor,
    WindowConfigApplyEdit,
    WindowConfigApplyMutation,
    WindowConfigMutation,
    WindowConfigOwner,
    WindowConfigOwnerRegistry,
    WindowConfigPack,
    WindowConfigPackLoad,
    WindowConfigPackLoadDiagnostic,
    WindowConfigPackLoadPhase,
    WindowConfigPackLoadProgress,
    WindowConfigPackLoadStep,
    WindowConfigSnapshot,
    WindowKindSpec,
    WindowKit,
    WindowTransientMutation,
    WindowTransientOwner,
    WindowTransientOwnerBundle,
    WindowTransientOwnerRegistry,
    WindowTransientSnapshot,
    WireArtifactInferenceBudget,
    WireArtifactInferenceCacheMode,
    WireArtifactInferenceDiagnostic,
    WireArtifactInferenceMetadata,
    WireArtifactInferenceProvenance,
    WireArtifactInferenceRequest,
    WireArtifactInferenceResult,
    WireInferenceArtifactBinding,
    WireInferenceCommitBinding,
    WireInferencePayloadContract,
    ARTIFACT_INFERENCE_WIRE_VERSION,
    CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES,
    MAINTENANCE_STAGES,
    NATURAL_FILE_PORT,
};
pub use app::{
    take_arena_unbuilt_rows, tree_window_indexed_item, tree_window_indexed_section, tree_window_item, tree_window_section, tree_window_section_or_placeholder, ui_node_list, TreeSlice, TreeWindows, TREE_WINDOW_BODY_NODE_BUDGET,
    TREE_WINDOW_DEFAULT_ROWS, TREE_WINDOW_FIXED_NODE_HEADROOM, TREE_WINDOW_PATH_SEPARATOR,
};
pub use engagement::{engagement_token_matches, strip_engagement_prefix};
pub use semio_framework_tool_machine::{TypingCommit, TypingFold, TYPING_BUFFER_ARG, TYPING_COMMIT_ARG, TYPING_IDLE_MS};
// 🧬️ A2 (design-abi.md §4): `host_port`'s re-export is deleted along with the module (see the
// "Replace, never wrap" note above `pub mod engagement`). `host::now_ms` replaces `host_now_ms` —
// `✏️s/🔌️plugins/🪐️space` is the one measured external caller (design-abi.md §0) and will be
// repointed during its own W3 migration wave, not here (SDK crate stays frozen during W3 per
// `important.md`'s sequencing constraints).
pub use plugin_runtime::{
    extension_activate, extension_cancel_close, extension_close_step, extension_deactivate, extension_invoke, extension_manifest, extension_retirement_demands, extension_resume_close, extension_terminal_is_empty, install_extension_bundle,
    install_plugin_bundle, install_plugin_bundle_result, plugin_artifact_apply_ops, plugin_artifact_genesis, plugin_artifact_pack_schema_hash, plugin_artifact_print_mirror, plugin_artifact_replay_envelopes, plugin_attach_backbone,
    plugin_cancel_media_export, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_poll_media_export, plugin_retain_media_export_result, plugin_submit_media_export,
    plugin_take_media_export_chunk, plugin_take_segmented_download_chunk, ExtensionBundle, ExtensionManifest, ExtensionResourceOwner, ExtensionInvokeStep,
};
pub use semio_framework::*;
pub use semio_framework::{MediaForm, MediaPortDirection, MediaPortSpec};
pub use semio_framework_ui_contract::{
    ActionBinding, ActionId, Buildable, Component, HasBase, HasChildren, Label as UiLabel, RowAction, RowActionPlacement, RowActionRefusal, RowTarget, Trigger, UiFixedList, UiFixedMap, UiListBuilder, UiMapBuilder, UiPublicationRevision, UiText, UiValue,
};
pub use world3d_host::{
    apply_world3d_projection_action, apply_world3d_sun_action, default_world3d_selection, merge_world_selection_ids, mesh_kind_from_json, scene_lane_hash, world3d_camera_projection_json, world3d_default_camera, world3d_environment_json,
    world3d_mesh_id_from_url, world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_projection_action_moves_pose, world3d_projection_measures, world3d_projection_pose,
    world3d_projection_spec_json, world3d_scene, world3d_selection_json, world3d_selection_json_with_granularity, world3d_sun_measures, SceneLanePayload, SceneLaneRef, SelectionSet, World3dScene, World3dSceneLane, WorldProjectionConfig,
    World3dAngle, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dDimension, World3dHighlight, World3dLeader, World3dLegendTick, World3dMarker, World3dMarkerShape, World3dModellingOptions, World3dPickGranularity, World3dPickTargets, World3dScalarDomain, World3dScalarField, World3dScalarLegend, World3dScalarRange, World3dSection, World3dSectionCap, World3dSubElementStyle, World3dText, World3dTone, WORLD3D_ANNOTATIONS_MAX, WorldSunConfig, WORLD3D_SCENE_LANE_BODY_KEYS, WORLD3D_SCENE_LANE_FIELDS, WORLD3D_SCENE_LANE_KEY_PREFIX, WORLD3D_SCENE_LANE_NAMES, WORLD3D_SCENE_LANE_OPTIONAL,
};
// 🧩️ Declarative component model (UiNode, layouts, utilities) — moved into ui_wgpu; re-exported here so
// apps keep the flat `semio_framework_plugin::*` import surface with zero Cargo.toml churn.
pub use ui_wgpu::wgpu::*;

#[cfg(test)]
#[path = "🧪️tests/📄️natural-file-lifecycle/🦀️.rs"]
mod natural_file_lifecycle_tests;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle_result($bundle);
    };
}

/// 🧬️ Declares uniform artifact lifecycle types from schema and IO hooks. UNIFIED-COMPOSABLE-
/// ARTIFACT-SYSTEM (C1, Task 5): accepts an optional trailing `children: $ty` field naming a real
/// `ArtifactChildren` impl (`$ty::Snapshot` must equal `<$construction as ArtifactBuilder>::Snapshot`,
/// enforced by `DerivedArtifactSpec::Children`'s own bound) — when present, `$composer`'s `reads()`
/// gains each declared slot's dialect and `compose()` routes matching sources through
/// `$ty::compose_from_children` (see `DerivedArtifactComposer`'s own doc comment for the mechanism;
/// this macro only has to name the type, the behavior lives centrally there). Omitting `children`
/// (every pre-C1 invocation) defaults `type Children` to `NoChildren<Snapshot>` — a leaf artifact
/// needs zero changes to keep compiling.
#[macro_export]
macro_rules! derive_artifact_facets {
    (
        $visibility:vis spec $spec:ident {
            construction: $construction:ty,
            analysis: $analysis:ty,
            composition: $composition:ty
            $(, children: $children:ty)? $(,)?
        }
        builder: $builder:ident,
        analyzer: $analyzer:ident,
        composer: $composer:ident $(,)?
    ) => {
        $visibility struct $spec;

        impl $crate::DerivedArtifactSpec for $spec {
            type Snapshot = <$construction as $crate::ArtifactBuilder>::Snapshot;
            type Mutation = <$construction as $crate::ArtifactBuilder>::Mutation;
            type Diff = <$construction as $crate::ArtifactBuilder>::Diff;
            type Construction = $construction;
            type Analysis = $analysis;
            type Composition = $composition;
            type Children = $crate::derive_artifact_facets!(@children_ty <$construction as $crate::ArtifactBuilder>::Snapshot $(, $children)?);
        }

        // 🕳️ `#[allow(dead_code)]` on this struct and the `$analyzer`/`$composer` inherent impls
        // below: every real (non-test) invocation of this macro exercises them through their trait
        // impls one paragraph up, but a grammar-only smoke test (proving a macro arm parses, not
        // that the generated code runs) never constructs or calls the inherent convenience surface
        // — a per-expansion false positive, not per-macro dead code.
        #[derive(Clone, Debug, Default)]
        #[allow(dead_code, reason = "grammar-smoke-test invocations never construct this — see comment above")]
        $visibility struct $builder($crate::DerivedArtifactBuilder<$spec>);

        impl $crate::ArtifactBuilder for $builder {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            type Mutation = <$spec as $crate::DerivedArtifactSpec>::Mutation;
            type Diff = <$spec as $crate::DerivedArtifactSpec>::Diff;

            fn empty() -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::empty()) }
            fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_snapshot(snapshot)) }
            fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_text(text)?)) }
            fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_binary(bytes)?)) }
            fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) { let (builder, outcome) = <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::mutate(self.0, mutation); (Self(builder), outcome) }
            fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> { <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::absorb(self.0, diff).map(Self) }
            fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> { <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::build(self.0) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never construct this — see comment above $builder")]
        $visibility struct $analyzer;

        impl $crate::ArtifactAnalyzer for $analyzer {
            type Parts = <$analysis as $crate::ArtifactAnalysis>::Parts;
            const DIALECT: $crate::Dialect = <$analysis as $crate::ArtifactAnalysis>::DIALECT;
            fn sniff(source: &$crate::io::AnalyzeSource<'_>) -> $crate::io::Confidence { <$analysis as $crate::ArtifactAnalysis>::sniff(source) }
            fn analyze(sources: &[$crate::io::AnalyzeSource<'_>]) -> $crate::io::Analysis<Self::Parts> { <$analysis as $crate::ArtifactAnalysis>::analyze(sources) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never call the inherent duplicates — see comment above $builder")]
        impl $analyzer {
            pub fn sniff(source: &$crate::io::AnalyzeSource<'_>) -> $crate::io::Confidence { <Self as $crate::ArtifactAnalyzer>::sniff(source) }
            pub fn analyze(sources: &[$crate::io::AnalyzeSource<'_>]) -> $crate::io::Analysis<<Self as $crate::ArtifactAnalyzer>::Parts> { <Self as $crate::ArtifactAnalyzer>::analyze(sources) }
        }

        $visibility struct $composer;

        // 🧩️ `reads()`/`compose()` delegate to `DerivedArtifactComposer<$spec>` (NOT `$composition`
        // directly, as before C1) so the child-slot union/routing logic lives in exactly ONE place
        // (`DerivedArtifactComposer`'s own `impl ArtifactComposer` — see its doc comment) rather than
        // being re-derived per macro expansion; `WRITES` is unaffected by children and stays a
        // direct read of `$composition::WRITES`.
        impl $crate::ArtifactComposer for $composer {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            const WRITES: $crate::Dialect = <$composition as $crate::ArtifactComposition>::WRITES;
            fn reads() -> &'static [$crate::Dialect] { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::reads() }
            fn compose(sources: &[$crate::io::ComposeSource<'_>]) -> Result<$crate::io::Composition<Self::Snapshot>, $crate::io::ComposeError> { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::compose(sources) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never call the inherent duplicate — see comment above $builder")]
        impl $composer {
            pub fn compose(sources: &[$crate::io::ComposeSource<'_>]) -> Result<$crate::io::Composition<<Self as $crate::ArtifactComposer>::Snapshot>, $crate::io::ComposeError> { <Self as $crate::ArtifactComposer>::compose(sources) }
        }
    };
    // 🧩️ Internal dispatch arm (Task 5): resolves `DerivedArtifactSpec::Children` — `$children` if
    // the caller supplied one, else `NoChildren<$snapshot>`. Never invoked directly by a caller (the
    // leading `@children_ty` token is not part of the public macro grammar above).
    (@children_ty $snapshot:ty) => { $crate::app::NoChildren<$snapshot> };
    (@children_ty $snapshot:ty, $children:ty) => { $children };
}

/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5) — `ArtifactChildren`/`DerivedArtifactSpec::Children`
/// tests. Placed here (rather than beside `🧬️DerivedArtifactFacets` near the top of the file) so
/// `derive_artifact_facets!` is already in scope textually — `macro_rules!` visibility is
/// definition-order-sensitive within a crate for a bare (non-`$crate`-qualified) invocation.
#[cfg(test)]
#[path = "🧪️tests/🔬️derived-artifact-children/🦀️.rs"]
mod derived_artifact_children_tests;

/// 🪆️ Whether a subset owns snapshot/diff/mutation types or derives behavior from a base dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubsetKind {
    Owning,
    Derived,
}

/// 🪆️ Declares one subset's lifecycle facets, runtime registration, and inline conformance tests.
#[macro_export]
macro_rules! subset {
    (
        $vis:vis owning dialect $artifact:literal / $standard:literal / $subset:literal {
            spec $spec:ident {
                construction: $construction:ty,
                analysis: $analysis:ty,
                composition: $composition:ty,
            }
            builder: $builder:ident,
            analyzer: $analyzer:ident,
            composer: $composer:ident,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(validator: $validator:ty,)?
            $(examples: [$($example:expr),+ $(,)?],)?
        }
    ) => {
        $crate::derive_artifact_facets! {
            $vis spec $spec {
                construction: $construction,
                analysis: $analysis,
                composition: $composition,
            }
            builder: $builder,
            analyzer: $analyzer,
            composer: $composer,
        }

        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Owning;
            static REGISTERED: Once = Once::new();
            $(static VALIDATOR_ENTRY: OnceLock<$crate::io::SubsetValidatorEntry> = OnceLock::new();)?

            // 🚫️async: E5 executor bridge — `Once::call_once`'s closure is std-fixed sync, so the
            // (pure, in-memory) registry calls it drives are resolved through `resolve_ready`
            // rather than forcing this whole registration path async (R9/R2).
            $(fn validator_entry() -> &'static $crate::io::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init($crate::io::subset_validator_entry_of::<$validator>)
            })?

            pub async fn register() {
                REGISTERED.call_once(|| {
                    let mut entries = vec![$crate::composer_entry_of::<$composer>()];
                    $(entries.extend([$($io_entry),+]);)?
                    $crate::io::register_composer_entries(&entries).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");
                    $($crate::io::register_subset_validator(validator_entry()).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");)?
                });
            }

            $(pub const EXAMPLES: &'static [$crate::ExampleSource] = &[$($example),+];)?

            $crate::__semio_subset_owning_conformance_tests!($composition);
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };

    (
        $vis:vis derived dialect $artifact:literal / $standard:literal / $subset:literal {
            validator: $validator:ty,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(positive: [$($pos_example:expr),+ $(,)?],)?
            $(negative: [$($neg_example:expr),+ $(,)?],)?
        }
    ) => {
        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Derived;
            static REGISTERED: Once = Once::new();
            static VALIDATOR_ENTRY: OnceLock<$crate::io::SubsetValidatorEntry> = OnceLock::new();

            // 🚫️async: E5 executor bridge — `Once::call_once`'s closure is std-fixed sync, so the
            // (pure, in-memory) registry calls it drives are resolved through `resolve_ready`
            // rather than forcing this whole registration path async (R9/R2).
            fn validator_entry() -> &'static $crate::io::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init($crate::io::subset_validator_entry_of::<$validator>)
            }

            pub async fn register() {
                REGISTERED.call_once(|| {
                    $crate::io::register_subset_validator(validator_entry()).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");
                    $( $crate::io::register_composer_entries(&[$($io_entry),+]).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error"); )?
                });
            }

            $(pub const POSITIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($pos_example),+];)?
            $(pub const NEGATIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($neg_example),+];)?

            $crate::__semio_subset_derived_conformance_tests!($validator);
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️subset-macro/🦀️.rs"]
mod subset_macro_tests;



#[path="🎮️command/📥️ingress/🦀️.rs"]
pub mod action_ingress;
