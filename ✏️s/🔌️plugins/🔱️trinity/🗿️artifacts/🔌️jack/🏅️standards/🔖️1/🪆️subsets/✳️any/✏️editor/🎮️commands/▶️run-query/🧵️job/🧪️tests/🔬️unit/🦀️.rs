use semio_framework_plugin::retained_command::ArtifactCommandWork;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::set_query;
#[allow(unused_imports)]
use crate::editor::jack::transient::{JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
#[allow(unused_imports)]
use crate::editor::jack::{TrinityJackCommand, TrinityJackPlayApp};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA;
#[allow(unused_imports)]
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
#[allow(unused_imports)]
use semio_framework_plugin::app::ArtifactOwnedToolJobContext;
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
#[allow(unused_imports)]
use semio_framework_job::InteractiveJobCloseStep;
#[allow(unused_imports)]
use semio_framework_plugin::{AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, FaultCode, FaultOrigin};
#[allow(unused_imports)]
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[allow(unused_imports)]
use semio_framework_value::RetirementDemand;
#[allow(unused_imports)]
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::content_read::JackContentRead;
#[allow(unused_imports)]
use semio_framework_plugin::app::ChildEmitPreparation;
use crate::editor::jack::commands::query::job::{Owner, QUERY_CHECKPOINT_BYTES, QUERY_REPLAY_MAXIMUM_STEPS, JackQueryWork};
use crate::editor::jack::commands::query::job::*;

type Work = JackQueryWork<crate::test_support::FixtureComposition>;

    fn checkpoint(work: &Work) -> [u8; QUERY_CHECKPOINT_BYTES] {
        let mut bytes = [0_u8; QUERY_CHECKPOINT_BYTES];
        assert_eq!(<Work as ArtifactCommandWork<Owner>>::checkpoint(work, &mut bytes).expect("query checkpoint"), QUERY_CHECKPOINT_BYTES);
        bytes
    }

    #[test]
    fn query_ownership_checkpoint_binds_exact_operation_and_generation() {
        let mut first = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 501, 12);
        first.progress = 37;
        let bytes = checkpoint(&first);
        let mut exact = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 501, 12);
        <Work as ArtifactCommandWork<Owner>>::restore(&mut exact, &bytes).expect("exact owner restores");
        assert_eq!(exact.replay_target, Some(37));
        let mut xor_collision = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 4_194_805, 13);
        assert_eq!(first.identity(), xor_collision.identity(), "fixture reproduces the old XOR collision");
        assert!(<Work as ArtifactCommandWork<Owner>>::restore(&mut xor_collision, &bytes).is_err());
    }

    #[test]
    fn query_ownership_checkpoint_accepts_legal_long_scan_progress() {
        let mut source = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 700, 21);
        source.progress = 1_000_001;
        let bytes = checkpoint(&source);
        let mut restored = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 700, 21);
        <Work as ArtifactCommandWork<Owner>>::restore(&mut restored, &bytes).expect("legal long scan restores");
        assert_eq!(restored.replay_target, Some(1_000_001));
        source.progress = QUERY_REPLAY_MAXIMUM_STEPS + 1;
        let rejected = checkpoint(&source);
        let mut target = Work::new("runQuery", "RETURN 1".into(), false, "editor".into(), "results".into(), 700, 21);
        assert!(<Work as ArtifactCommandWork<Owner>>::restore(&mut target, &rejected).is_err());
    }
