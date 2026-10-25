//! 📨️ Envelope builders and one-item grants shared by the replay tests.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use semio_framework_os_store_testing_prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use crate::*;
use semio_framework_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_framework_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_framework_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_framework_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_framework_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;
use crate::store::ArtifactStore;

pub fn one_item_test_policy() -> ArtifactStoreOneItemGrant {
    let grant = physical_test_close_grant();
    ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth }
}


//#region 🧺️BatchedPublication
/// 🎟️ One spent item per turn, with the whole capacity a thirty-two to two-hundred item gesture reserves for its staged forwards, metadata and flags in a single turn.
/// 🧮️ The per-turn copy credit that funds one quoted preparation birth (the retained preparation frame is about 6.4 KiB).
pub const PREPARATION_TURN_COPY_BYTES: usize = 16384;

pub fn batch_turn_grant() -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 262144, maximum_depth: 4096 }
}


/// 🛰️ Builds a foreign {@link MutationEnvelope} (as if authored by `actor` on another peer) by

/// 🧬️ applying `operation` in a throwaway peer store and stamping the envelope's actor id.
pub async fn foreign_mutation_envelope(actor: &str, operation: DemoMutation) -> semio_framework_os_spr::MutationEnvelope {
    let mut peer = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &crate::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut peer, ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("peer apply");
    let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
    let document_id = ArtifactId(peer.envelope().id.clone());
    let schema = SchemaId(peer.envelope().schema.clone());
    let mut envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
    let mut envelope = envelopes.pop().expect("exactly one op envelope for a single-op edit");
    envelope.actor = ActorId(actor.to_string().into());
    envelope
}


//#region 🔖️MergePolicyTests
// 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6 acceptance —
// two-peer merge-policy/conflict tests, written standalone (not depending on 1-D's
// `📡️spr/🧫️fixtures` `🔖️Laws` helpers landing first).

/// 🛰️ Builds a `MutationEnvelope` with an explicit HLC (bypassing wall-clock timing) so arrival
/// order and HLC order can be controlled independently — the two-peer tests below need both.
pub fn mutation_envelope_at(actor: &str, mutation_id: &str, operation: DemoMutation, hlc: HybridLogicalTimestamp, dependencies: Vec<MutationId>) -> semio_framework_os_spr::MutationEnvelope {
    semio_framework_os_spr::MutationEnvelope {
        mutation_id: MutationId(mutation_id.to_string()),
        document_id: ArtifactId("demo".to_string()),
        actor: ActorId(actor.to_string().into()),
        dependencies,
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode demo mutation") },
        inverse: semio_framework_os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp: hlc,
        transaction: None,
        verb: None,
        line: None,
    }
}

pub async fn fresh_demo_store() -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &crate::DemoMemberStoreProducer::<_>::new()).await
}

pub async fn owned_test_envelope<P, Mutation>(store: &ArtifactStore<P, Mutation>) -> ArtifactEnvelope<P, Mutation>
where
    P: Clone + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + OpText + OpBinary + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + PartialEq + ToValue + FromValue + Send + 'static,
{
    let files = print_document_pack(store.envelope()).await.expect("print owned test envelope");
    parse_document_pack::<P, Mutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse owned test envelope").envelope
}
//#endregion 🔖️MergePolicyTests
//#endregion 🧺️BatchedPublication
