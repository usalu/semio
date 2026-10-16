/** 🧪️ Self-test lanes the composition root injects into the verification gates, so no production gate imports a test module.
 * @see 📜️script.ts (builds the lane registry)
 */
import type { PolicyRustSourceEvidence } from "../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** 📥️ The structural slice of the member-store producer sources the self-test mutates: the producer and member bodies of each explicit extension. */
export type MemberProducerSourcesPort = Readonly<{ extensions: readonly Readonly<{ producer: Readonly<{ source: string }>; members: Readonly<{ source: string }> }>[] }>;

/** 🧭️ One entry per hostile-mutation self-test lane; `📜️script.ts` binds each to its test module. */
export interface SelfTestLanes {
  readonly semioMemberStoreProducerSourceSelfTests: (check: (sources: MemberProducerSourcesPort) => boolean, sources: MemberProducerSourcesPort, domainSchema: Readonly<{ $id: string }>) => number;
  readonly toolJobArtifactEnvelopeRejectionTransferSelfTests: (store: PolicyRustSourceEvidence) => number;
  readonly toolJobLiveFixedReplaySelfTests: (actor: string, shard: string, executor: string, wgpu: string, actionBus: string, pluginApp: string, pluginHost: string, programBridge: string) => number;
  readonly toolJobFixedOperationRegistrySelfTests: (source: string) => number;
  readonly toolJobDrawingGestureOperationOwnerSelfTests: () => number;
  readonly toolJobArtifactRetainedCommandSelfTests: (source: string, runtimeLawSource: string) => number;
  readonly interactivityAllAppDiscoverySelfTests: () => Promise<number>;
  readonly interactivityRuntimeSourceSelfTests: () => void;
  readonly interactivityToolRunPolicySelfTests: () => number;
  readonly historyClosurePolicySelfTests: () => number;
  readonly interactivityLiveReconcileSelfTests: (repoRoot: string) => void;
  readonly interactivityMountedLayoutTextSelfTests: (repoRoot: string) => void;
  readonly interactivityMountedFrameTransactionSelfTests: (repoRoot: string) => void;
  readonly interactivityMountedEngineSurfaceLifetimeSelfTests: (repoRoot: string) => void;
  readonly interactivityMountedPreparedRenderSelfTests: (repoRoot: string) => void;
  readonly interactivityMountedSurfaceLaneSelfTests: (repoRoot: string) => void;
  readonly interactivityPreparedRasterProducerSelfTests: () => void;
  readonly interactivityShardExecutorSelfTests: () => void;
  readonly interactivityStoreSyncSelfTests: () => void;
  readonly interactivityDbIoB1B6SelfTests: () => void;
  readonly interactivityP1qR4SelfTests: () => void;
  readonly interactivityDbIoCallerMigrationSelfTests: () => void;
  readonly interactivityDbIoDirectWriterSelfTests: () => void;
  readonly interactivityDbIoSelfTests: () => void;
  readonly interactivityArtifactSubmitSelfTests: () => void;
  readonly interactivityDatabaseCapabilityOpenSelfTests: () => void;
  readonly interactivityDatabaseCatalogReadSelfTests: () => void;
  readonly interactivityDatabaseCatalogBootstrapSelfTests: () => void;
  readonly interactivityDatabaseCreateCatalogSelfTests: () => void;
  readonly interactivityDatabaseCompactionSelfTests: (compact: string, engine: string, snapshot: string, index: string, contract: string) => void;
  readonly interactivityDatabaseSyncHelloSelfTests: (sync: string, engine: string, hub: string, wal: string, protocol: string, contract: string) => void;
  readonly interactivityArtifactHistorySelfTests: () => void;
  readonly interactivityDbCliSelfTests: () => void;
  readonly interactivityMcpHttpTransportSelfTests: () => void;
  readonly dependencyJsLockParitySelfTests: () => number;
  readonly dependencyTruthSelfTests: () => number;
  readonly toolJobCoverageSelfTests: () => Promise<number>;
  readonly toolJobLatestWinsSelfTests: () => Promise<number>;
}
