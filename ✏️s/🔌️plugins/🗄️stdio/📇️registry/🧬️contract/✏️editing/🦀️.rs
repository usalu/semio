//! ✏️ Schema-erased, typed snapshot editing shared by every stdio artifact editor.

#[cfg(test)]
#[global_allocator]
static ORIGINAL_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as kernel;
use semio_framework_value_derive as value_derive;
use semio_s_artifact_stdio_contract_window_kit::load_example_effect;
use pack::snapshot::ArtifactDsl;
use kernel::{Mutation, MutationDiff, OpBinary};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use {semio_framework_plugin::AppOperationContext,semio_framework_plugin::ArtifactBoundedFirstStepProof,semio_framework_plugin::ArtifactEditor,semio_framework_plugin::ArtifactOwnedToolJobFactory,semio_framework_plugin::ArtifactOwnedToolJobRequest,semio_framework_plugin::ArtifactToolFactoryRegistry,semio_framework_plugin::ArtifactToolPublicationContract,semio_framework_plugin::ArtifactToolPublicationLane,semio_framework_artifact_reference::Dialect,semio_framework_plugin::EditorApp,semio_framework_plugin::Emit,semio_framework_plugin::InteractiveJobClassification,semio_framework_plugin::ToolExecutionContract,semio_framework_plugin::ToolFactoryKey,semio_framework_plugin::ToolJobFactory,semio_framework_plugin::ToolJobFactoryError,semio_framework_plugin::ToolOperationSpec};
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::{BTreeMap, BTreeSet};

#[path = "🧵️bytes/🦀️.rs"]
pub mod bytes;
#[path = "🖼️raster/🦀️.rs"]
pub mod raster;
#[path = "🧭️rules/🦀️.rs"]
pub mod rules;
pub use rules::{edited_subtree, Carried, EditPlan, EditRules, EntityRule, InsertRule, ItemField, RemoveRule, RowKey, Selector};

#[path = "📜️edit-contract/🦀️.rs"]
mod edit_contract;
pub use edit_contract::*;

#[macro_export]
macro_rules! snapshot_editing_command_roster {
    ($native:ty, [$($native_tool:expr),* $(,)?]) => {
        impl $crate::SnapshotEditingNativeCommand for $native {
            const ALL_TOOL_JOB_IDS: &'static [&'static str] = &[
                $($native_tool,)*
                $crate::SET_SNAPSHOT_VALUE_ACTION_ID,
                $crate::INSERT_SNAPSHOT_VALUE_ACTION_ID,
                $crate::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
                $crate::MOVE_SNAPSHOT_VALUE_ACTION_ID,
                $crate::RENAME_SNAPSHOT_KEY_ACTION_ID,
                $crate::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
            ];
        }
    };
}

#[path = "🪡️value-operations/🦀️.rs"]
mod value_operations;
pub use value_operations::*;

#[path = "📨️action-bridge/🦀️.rs"]
mod action_bridge;
pub use action_bridge::*;

#[path = "🧰️editor-traits-and-retained-copies/🦀️.rs"]
mod editor_traits_and_retained_copies;
pub use editor_traits_and_retained_copies::*;

#[macro_export]
macro_rules! bounded_native_editing_editor {
    (
        editor: $editor:ty,
        tools: [$($tool:literal),+ $(,)?],
        payload_schema: $payload_schema:literal,
        reduce: |$command:ident, $snapshot:ident| $reduce:block
        $(, work: $work:path)?
        $(, preparation_route: $preparation_route:path)?
        $(, checkpoint_resume: $checkpoint_resume:expr)?
        $(,)?) => {
        impl $crate::BoundedNativeEditingEditor for $editor {
            const NATIVE_TOOL_IDS: &'static [&'static str] = &[$($tool),+];
            const NATIVE_PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
                $(semio_framework_plugin::ArtifactToolPublicationContract {
                    tool_id: $tool,
                    lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact],
                }),+
            ];
            const NATIVE_PAYLOAD_SCHEMA: &'static str = $payload_schema;
            $(const NATIVE_CHECKPOINT_RESUME: bool = $checkpoint_resume;)?

            fn native_edit_mutations(
                $command: &Self::Command,
                $snapshot: &Self::Snapshot,
            ) -> Result<
                semio_framework_plugin::Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>,
                semio_framework_plugin::Fault,
            > $reduce

            $(
                fn native_edit_work(
                    tool_id: &'static str,
                ) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> {
                    $work(tool_id)
                }
            )?

            $(
                fn native_edit_preparation_route(
                    prefix: &'static str,
                ) -> Option<$crate::NativeEditPreparationRoute<Self::Snapshot, Self::Mutation>> {
                    $preparation_route(prefix)
                }
            )?
        }
    };
}

#[path = "🛠️tool-job-factories/🦀️.rs"]
mod tool_job_factories;
pub use tool_job_factories::*;

/// 🧩 Declares native retained-tool proofs and appends the shared snapshot-edit proofs.
#[macro_export]
macro_rules! snapshot_editing_bounded_first_step_tool_proofs {
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        contract: $contract:expr,
        tools: [$($tool:literal),+ $(,)?]
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: semio_framework_plugin::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            let mut proofs = vec![$(
                qualify(semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+];
            proofs.extend($crate::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            proofs
        }
    };
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        tools: { $($tool:literal => $contract:expr),+ $(,)? }
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: semio_framework_plugin::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            let mut proofs = vec![$(
                qualify(semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+];
            proofs.extend($crate::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            proofs
        }
    };
}

/// 📤️ Declares the document lane's one-item publication authority of a snapshot editor: the generic mutation-apply factory every snapshot edit publishes through, without which each such route fails closed with `interactive-job.publication-authority-missing`.
#[macro_export]
macro_rules! snapshot_editing_publication_authority {
    () => {
        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            Some(semio_framework_plugin::plugin_app_close_prelude::store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
        }
    };
}

#[macro_export]
macro_rules! snapshot_details_editor_support {
    (
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        preparation: $preparation:literal
        $(, document_store_owners: $document_store_owners:path)?
        $(, bounded_native: $bounded_native:tt)?
        $(, native: {
            factory_name: $native_factory_name:literal,
            factory_type: $native_factory_type:ty,
            contract: $native_contract:expr,
            build: $native_build:path,
            tools: [$($native_tool:literal),+ $(,)?]
        })?
        $(,)?
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let mut proofs = $crate::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema);
            $(
                let _ = stringify!($bounded_native);
                proofs.extend($crate::bounded_native_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            )?
            $($(
                proofs.push(
                    semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<semio_framework_plugin::EditorApp<Self>>(
                        $owner_file,
                        $controller,
                        $native_factory_name,
                        $native_tool,
                        $artifact_schema,
                        $native_contract,
                    )
                    .with_factory_type::<semio_framework_plugin::EditorApp<Self>, $native_factory_type>(),
                );
            )+)?
            proofs
        }

        fn register_tool_job_factories(
            registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>,
        ) -> Result<(), semio_framework_plugin::Fault> {
            $(
                let _ = stringify!($bounded_native);
                $crate::register_bounded_native_edit_tool_factory::<Self>(registry)?;
            )?
            $(
                let controller = registry.controller_id().to_string();
                registry.register(<$native_factory_type>::new(&controller))?;
            )?
            $crate::register_snapshot_edit_tool_factory::<Self>(registry)
        }

        fn build_tool_job(
            request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>,
        ) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, semio_framework_plugin::Fault> {
            if $crate::is_snapshot_edit_action(&request.tool_id) {
                return $crate::build_snapshot_edit_tool_job::<Self>(request);
            }
            $(
                let _ = stringify!($bounded_native);
                if <$crate::BoundedNativeEditToolJobFactory<Self> as semio_framework_plugin::ArtifactOwnedToolJobFactory>::TOOL_IDS.contains(&request.tool_id.as_str()) {
                    return $crate::build_bounded_native_edit_tool_job::<Self>(request);
                }
            )?
            $(return $native_build(request);)?
            Ok(None)
        }

        $(
            fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(semio_framework_plugin::plugin_app_close_prelude::store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_plugin::plugin_app_close_prelude::store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
                Some($document_store_owners(grant))
            }
        )?

        fn build_artifact_store_one_item_preparation_factory(
        ) -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            let factory = semio_framework_plugin::plugin_app_close_prelude::store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>();
            $(
                let _ = stringify!($bounded_native);
                let factory = $crate::routed_native_edit_preparation_factory(
                    <Self as $crate::BoundedNativeEditingEditor>::native_edit_preparation_route($preparation),
                    factory,
                );
            )?
            Some(semio_framework_plugin::plugin_app_close_prelude::store::operation_wire_preparation_factory(factory, <Self as $crate::SnapshotEditingEditor>::snapshot_operation_wire_source, |operation| { let semantics = semio_framework_plugin::plugin_app_close_prelude::protocol::SemanticMutation::semantics(operation); Some((semantics.entity, semantics.kind)) }))
        }

    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
