
use std::sync::Arc;
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_lag_rebootstrap::{CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, decode_canonical_checkpoint_pair};
use directory::os_directory::{DocumentScope, EditedArtifactFrontierV1};
use semio_hub_state::HubState;
use semio_hub_directory_contract::SessionCapability;
use semio_framework_hash::Sha256;
use std::net::SocketAddr;
use semio_hub_route_testkit::{TestIssuedSession, create_space_for_test, issue_test_session, publish_genesis_checkpoint_for_test, raw_http_request, tempdir, test_state, upsert_member_for_test};
use semio_hub_artifact_authority::ValidatingCanonicalArtifactAuthority;
use protocol::ArtifactId as WireArtifactId;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_route_models::check_in::ledger_artifact_frontier;
use semio_hub_directory::{AdminOperationDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog};
#[cfg(test)]
use directory::os_directory::{DirectorySpaceRole, DirectorySpaceVisibility, DocumentCheckInStatusV1, DocumentCheckInV1, DocumentDescriptor};
#[cfg(test)]
use protocol::MutationEnvelope;
#[cfg(test)]
use directory::os_directory;
#[cfg(feature = "integration-fixtures")]
#[path = "../../🪶️count-lease/🦀️.rs"]
mod count_lease_tests;

include!("🧰️law-harness/🦀️.rs");

#[path = "../../../🎲️hostile-generative/🦀️.rs"]
mod hostile_generative;

include!("⚡️quick-hostile-laws/🦀️.rs");

include!("🐢️long-hostile-laws/🦀️.rs");
