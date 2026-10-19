#!/usr/bin/env bun
import { StableIdentityCheckScript } from "./🎛️commands/📸️snapshot-checks/🟦️.ts";
import { DiagnosticAdmissionCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
import { OwnedReplayCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
import { CanonicalSourceIdentityCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
import { RetainedLaneScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
import { OriginalIngressScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
import { OriginalHistoryInputScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
import { OriginalIoRetirementScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
import { SharedSemanticSourceScript } from "../../🔨️modules/🏪️store/🏗️initialization/🏠️owners/📜️script.ts";
import { MemberFacetsScript } from "./🎛️commands/👥️member-dialect-checks/🟦️.ts";
import { OriginalVcsCommandScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
import { DbErrorOwnershipCheckScript } from "./🎛️commands/🗄️database-checks/🟦️.ts";
import { OriginalSnapshotReceivingScript } from "./🎛️commands/📸️snapshot-checks/🟦️.ts";
/** 🦀️ `@semio-tech/framework-os-kernel` task router: parses the command and dispatches it to the check and test scripts of `🎛️commands`. */
import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CheckScript, TestScript, NativeTestScript } from "./🎛️commands/🧱️script-bases/🟦️.ts";
import { PagedHistoryStackScript, DatabaseHistoryCompletionCheckScript, DatabaseCatalogReadOwnershipCheckScript, DatabaseCapabilityCompletionCheckScript, DatabaseShutdownCheckScript } from "./🎛️commands/🗄️database-checks/🟦️.ts";
import { WalWriterAuthorityCheckScript, WalWriterFenceLiveScript, PostgresRoundTripsLiveScript, WalCommittedTransactionsCheckScript, WalCommittedCompactionCheckScript, WalRecoveryCheckScript, WalCapacityCheckScript, WalSegmentStateCheckScript } from "./🎛️commands/✍️wal-checks/🟦️.ts";
import { DocumentMountSingleFlightCheckScript, DurableOwnedGroupDecisionCheckScript, DurableGroupJournalCheckScript, DocumentOpeningAttemptNativeCheckScript, ReopenStormCheckScript, DocumentHttpCheckScript } from "./🎛️commands/📄️document-checks/🟦️.ts";
import { RetainedCloneCheckScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
import { DirectoryDescriptorDigestSourceScript, DirectoryDescriptorDigestNativeScript, DirectoryCheckpointPairSourceScript, DirectoryCheckpointPairNativeScript, DirectoryRuntimeSourceScript, DirectorySessionAuthorityCheckScript, DirectoryEventPageContractCheckScript, DirectoryEventPageClientCheckScript, DirectoryEventPageBootstrapCheckScript, DirectoryLeaseFixtureScript } from "./🎛️commands/📇️directory-checks/🟦️.ts";
import { SpaceHistorySqliteNativeTestScript, SpaceHistorySqliteSourceTestScript, SnapshotRouteAdmissionScript, SnapshotCodecReceivingScript, SnapshotNativeAdmissionTestScript, SnapshotSqliteAdmissionScript, TypedSnapshotFileSourceScript, TypedSnapshotFileNativeScript, SpaceHistoryValidationScript } from "./🎛️commands/📸️snapshot-checks/🟦️.ts";
import { CodecSendSourceScript, GroupVisibilitySourceScript, BackboneDetachSourceScript, MemberDialectSourceScript, MemberInputCensusSourceScript, MemberGroupPublicationSourceScript, MemberDialectCheckScript } from "./🎛️commands/👥️member-dialect-checks/🟦️.ts";
import { ScalarWireSourceScript, PackSingleOwnerSourceScript, CanonicalArchitectureScript, CommandTransportSourceTestScript, CommandTransportNativeTestScript, ListRecordNativeTestScript, ComposedPackSchemaTestScript, IeeePayloadNativeTestScript, IeeePayloadSourceTestScript, IntrinsicRetirementNativeTestScript, BorrowedKeyIndexNativeTestScript, AssemblyLifetimeNativeTestScript, GenerateJcoPackageAdapterScript, PreviewGeneratedScript, CheckJcoPackageAdapterScript, CommandIngressConsumerScript, DslArchitectureProofScript, ArtifactKindNativeScript, ReturnedReadCustodyScript, ReturnedReadFactoryMountScript, NativeEditMetadataSourceScript, SyncActorIdentitySourceScript, CanonicalPreparationControlSourceScript, ConfigPagedSourceScript, OriginalCleanupContextNativeScript, CrateBoundariesSourceScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-command-ingress-consumer",CommandIngressConsumerScript)
  .register("check", CheckScript)
  .register("retained-clone-check", RetainedCloneCheckScript)
  .register("test", TestScript)
  .register("test-list-record-boundaries", ListRecordNativeTestScript)
  .register("test-composed-pack-schema", ComposedPackSchemaTestScript)
  .register("test-ieee-payload-native", IeeePayloadNativeTestScript)
  .register("test-ieee-payload-source", IeeePayloadSourceTestScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("document-opening-attempt-native-check", DocumentOpeningAttemptNativeCheckScript)
  .register("test-scalar-wire-source", ScalarWireSourceScript)
  .register("test-pack-single-owner-source", PackSingleOwnerSourceScript)
  .register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-jco-package-adapter", CheckJcoPackageAdapterScript)
  .register("test-native", NativeTestScript)
  .register("test-command-transport-source", CommandTransportSourceTestScript)
  .register("test-command-transport-native", CommandTransportNativeTestScript)
  .register("test-intrinsic-retirement-native", IntrinsicRetirementNativeTestScript)
  .register("test-borrowed-key-index-native", BorrowedKeyIndexNativeTestScript)
  .register("test-snapshot-native-admission", SnapshotNativeAdmissionTestScript)
  .register("test-snapshot-codec-receiving", SnapshotCodecReceivingScript)
  .register("test-snapshot-route-admission", SnapshotRouteAdmissionScript)
  .register("test-space-history-sqlite-native",SpaceHistorySqliteNativeTestScript)
  .register("test-directory-descriptor-digest-source",DirectoryDescriptorDigestSourceScript)
  .register("test-directory-descriptor-digest-native",DirectoryDescriptorDigestNativeScript)
  .register("test-directory-checkpoint-pair-source",DirectoryCheckpointPairSourceScript)
  .register("test-directory-checkpoint-pair-native",DirectoryCheckpointPairNativeScript)
  .register("test-space-history-sqlite-source",SpaceHistorySqliteSourceTestScript)
  .register("test-assembly-lifetime-native", AssemblyLifetimeNativeTestScript)
  .register("test-directory-runtime-source", DirectoryRuntimeSourceScript)
  .register("directory-session-authority-check", DirectorySessionAuthorityCheckScript)
  .register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript)
  .register("directory-event-page-client-check", DirectoryEventPageClientCheckScript)
  .register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript)
  .register("wal-segment-state-check", WalSegmentStateCheckScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("test-group-visibility-source", GroupVisibilitySourceScript)
  .register("test-backbone-detach-source", BackboneDetachSourceScript)
  .register("test-member-dialect-source", MemberDialectSourceScript)
  .register("test-member-input-census-source", MemberInputCensusSourceScript)
  .register("test-member-group-publication-source", MemberGroupPublicationSourceScript)
  .register("member-dialect-check", MemberDialectCheckScript);

router.register("document-http-check", DocumentHttpCheckScript);

router.register("paged-history-stack-check", PagedHistoryStackScript);

router.register("wal-recovery-check", WalRecoveryCheckScript);

router.register("wal-capacity-check", WalCapacityCheckScript);

router.register("wal-committed-transactions-check", WalCommittedTransactionsCheckScript);

router.register("wal-writer-authority-check", WalWriterAuthorityCheckScript);

router.register("wal-writer-fence-live", WalWriterFenceLiveScript);

router.register("postgres-round-trips-live", PostgresRoundTripsLiveScript);

router.register("database-history-completion-check", DatabaseHistoryCompletionCheckScript);

router.register("database-catalog-read-ownership-check", DatabaseCatalogReadOwnershipCheckScript);

router.register("database-capability-completion-check", DatabaseCapabilityCompletionCheckScript);

router.register("wal-committed-compaction-check", WalCommittedCompactionCheckScript);

router.register("database-shutdown-check", DatabaseShutdownCheckScript);

router.register("document-mount-single-flight-check", DocumentMountSingleFlightCheckScript);

router.register("durable-owned-group-decision-check", DurableOwnedGroupDecisionCheckScript);

router.register("durable-group-journal-check", DurableGroupJournalCheckScript);

router.register("reopen-storm-check", ReopenStormCheckScript);

router.register("test-directory-lease-fixture", DirectoryLeaseFixtureScript);


router.register("test-dsl-architecture", DslArchitectureProofScript);

router.register("test-artifact-kind",ArtifactKindNativeScript).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript);

router.register("test-returned-read-custody",ReturnedReadCustodyScript);

router.register("test-returned-read-factory-mount",ReturnedReadFactoryMountScript);


router.register("test-native-edit-metadata-source",NativeEditMetadataSourceScript);

router.register("test-sync-actor-identity-source",SyncActorIdentitySourceScript);

router.register("test-canonical-preparation-control-source",CanonicalPreparationControlSourceScript);

router.register("test-typed-snapshot-file-source",TypedSnapshotFileSourceScript);

router.register("test-typed-snapshot-file-native",TypedSnapshotFileNativeScript);

router.register("test-space-history-validation",SpaceHistoryValidationScript);

router.register("test-config-paged-source",ConfigPagedSourceScript);

router.register("test-original-cleanup-context-native",OriginalCleanupContextNativeScript);

router.register("test-crate-boundaries",CrateBoundariesSourceScript);

router.register("stable-identity-check", StableIdentityCheckScript);
router.register("test-diagnostic-admission", DiagnosticAdmissionCheckScript);
router.register("test-owned-replay", OwnedReplayCheckScript);
router.register("test-canonical-source-identity", CanonicalSourceIdentityCheckScript);
router.register("test-retained-lane", RetainedLaneScript);
router.register("test-original-ingress", OriginalIngressScript);
router.register("test-original-history-input", OriginalHistoryInputScript);
router.register("test-original-io-retirement", OriginalIoRetirementScript);
router.register("semantic-source", SharedSemanticSourceScript);
router.register("member-facets", MemberFacetsScript);
router.register("test-original-vcs-command", OriginalVcsCommandScript);
router.register("db-error-ownership-check", DbErrorOwnershipCheckScript);
router.register("test-original-snapshot-receiving", OriginalSnapshotReceivingScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
