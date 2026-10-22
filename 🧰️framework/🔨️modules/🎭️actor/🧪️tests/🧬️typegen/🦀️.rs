//! 🧬️ Native TypeScript exporter: the versioned wire schema table renders the generated mirror, or is compared with it when no export path is requested.

mod schema_metadata {
    use std::collections::HashSet;

    /// 🧬️ One versioned wire type and its owned TypeScript projection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SchemaMetadata {
        pub name: &'static str,
        pub version: u16,
        pub typescript: &'static str,
    }

    pub const TYPES: &[SchemaMetadata] = &[
        SchemaMetadata { name: "ActivationEvent", version: 1, typescript: r#"export type ActivationEvent = { "kind": "manual" } | { "kind": "windowOpen", window: WindowId, } | { "kind": "restart" };"# },
        SchemaMetadata { name: "ActorId", version: 1, typescript: "export type ActorId = bigint;" },
        SchemaMetadata {
            name: "ActorKind",
            version: 1,
            typescript: r#"export type ActorKind = { "kind": "pluginApp", plugin: PackageId, appId: string, instanceId: number, } | { "kind": "extension", plugin: PackageId, extensionId: string, } | { "kind": "job", owner: ActorId, jobId: bigint, };"#,
        },
        SchemaMetadata {
            name: "ActorMetrics",
            version: 1,
            typescript: "export type ActorMetrics = { turns: bigint, fuel_total: bigint, wall_us_total: bigint, wall_us_ring: Array<number>, wall_us_ring_len: number, wall_us_ring_pos: number, memory_bytes: bigint, mailbox_len: number, mailbox_lag_ms: number, coalesced: bigint, dropped: bigint, traps: number, restarts: number, stage: FailureStage, shard: ShardId, };",
        },
        SchemaMetadata { name: "ActorMetricsSample", version: 1, typescript: "export type ActorMetricsSample = { id: ActorId, package: PackageId, lane: Lane, status: ActorStatus, metrics: ActorMetrics, };" },
        SchemaMetadata {
            name: "ActorRecord",
            version: 1,
            typescript: "export type ActorRecord = { id: ActorId, kind: ActorKind, package: PackageId, shard: ShardId, capabilities: Array<CapabilityGrant>, budget: Budget, mailbox: Mailbox, status: ActorStatus, failure: FailureState, metrics: ActorMetrics, };",
        },
        SchemaMetadata {
            name: "ActorStatus",
            version: 1,
            typescript: r#"export type ActorStatus = { "kind": "cold" } | { "kind": "activating" } | { "kind": "active" } | { "kind": "suspended", checkpoint: Array<number> | null, } | { "kind": "draining" } | { "kind": "trapped" } | { "kind": "quarantined" } | { "kind": "disabled" };"#,
        },
        SchemaMetadata { name: "Backpressure", version: 1, typescript: r#"export type Backpressure = { "kind": "accept" } | { "kind": "coalesced" } | { "kind": "dropped", lane: Lane, } | { "kind": "rejected" };"# },
        SchemaMetadata { name: "Budget", version: 1, typescript: "export type Budget = { fuel: bigint, wall_ms: number, memory_bytes: bigint, ui_nodes: number, mailbox_len: number, max_effects: number, max_patch_bytes: number, max_frames: number, };" },
        SchemaMetadata { name: "CapabilityGrant", version: 1, typescript: "export type CapabilityGrant = { capability: string, scope: Array<number> | null, };" },
        SchemaMetadata { name: "CoalesceKey", version: 1, typescript: "export type CoalesceKey = string;" },
        SchemaMetadata { name: "Decision", version: 1, typescript: "export type Decision = { run: Array<TurnGrant>, wake_at: bigint | null, };" },
        SchemaMetadata {
            name: "Envelope",
            version: 1,
            typescript: "export type Envelope = { to: ActorId, from: Origin, lane: Lane, seq: bigint, deadline_ms: bigint | null, coalesce: CoalesceKey | null, cancel_of: bigint | null, payload: Payload, };",
        },
        SchemaMetadata {
            name: "FailureSignal",
            version: 1,
            typescript: r#"export type FailureSignal = { "kind": "deadlineOverrun", ratio: number, } | { "kind": "fuelExhausted" } | { "kind": "memoryLimit" } | { "kind": "mailboxOverflow" } | { "kind": "uiQuota" } | { "kind": "trap", detail: string, } | { "kind": "heartbeatMissed", count: number, } | { "kind": "manualReset" };"#,
        },
        SchemaMetadata {
            name: "FailureStage",
            version: 1,
            typescript: r#"export type FailureStage = { "kind": "healthy" } | { "kind": "warned" } | { "kind": "throttled", factor: number, } | { "kind": "suspended", until: bigint, } | { "kind": "cancelled" } | { "kind": "trapped", restarts: number, } | { "kind": "quarantined", until: bigint, } | { "kind": "disabled" };"#,
        },
        SchemaMetadata { name: "FailureState", version: 1, typescript: "export type FailureState = { stage: FailureStage, clean_turns: number, warn_count: number, restart_count: number, last_signal_ms: bigint, };" },
        SchemaMetadata { name: "JobCheckpoint", version: 1, typescript: "export type JobCheckpoint = { state: Array<number>, applied_progress: bigint, };" },
        SchemaMetadata { name: "JobCommitCandidate", version: 1, typescript: "export type JobCommitCandidate = { state: Array<number>, output: Array<number>, };" },
        SchemaMetadata { name: "JobOperation", version: 1, typescript: "export type JobOperation = { operation: bigint, base_revision: bigint, generation: bigint, preview_sequence: bigint, seed: bigint, };" },
        SchemaMetadata {
            name: "JobProgressIdentity",
            version: 1,
            typescript: "export type JobProgressIdentity = { actor: ActorId, job: bigint, operation: bigint, base_revision: bigint, generation: bigint, step_sequence: bigint, preview_sequence: bigint, };",
        },
        SchemaMetadata { name: "JobProgressKind", version: 1, typescript: r#"export type JobProgressKind = "yield" | "preview" | "checkpoint" | "commitValidated" | "cancelled" | "fault";"# },
        SchemaMetadata { name: "JobProgressReceipt", version: 1, typescript: "export type JobProgressReceipt = { identity: JobProgressIdentity, kind: JobProgressKind, applied_progress: bigint, owner_bytes: number, };" },
        SchemaMetadata { name: "JobPublication", version: 1, typescript: "export type JobPublication = { turn: JobTurn, outcome: JobStepOutcome, };" },
        SchemaMetadata { name: "JobReplayLog", version: 2, typescript: "export type JobReplayLog = { route: JobReplayRoute, sealedRecords: number, };" },
        SchemaMetadata { name: "JobReplayRequest", version: 1, typescript: "export type JobReplayRequest = { controller: number[], tool: number[], schema: number[], version: number, digest: number[], };" },
        SchemaMetadata {
            name: "JobReplayRoute",
            version: 1,
            typescript: "export type JobReplayRoute = { plugin: number[], package: number[], controller: number[], tool: number[], window: bigint, artifact: number[], requestSchema: number[], requestVersion: number, requestDigest: number[], };",
        },
        SchemaMetadata {
            name: "JobStepOutcome",
            version: 1,
            typescript: r#"export type JobStepOutcome = { "kind": "yield" } | { "kind": "previewReady", preview: Array<number>, } | { "kind": "checkpointReady", checkpoint: JobCheckpoint, } | { "kind": "complete", candidate: JobCommitCandidate, } | { "kind": "cancelled" } | { "kind": "fault", detail: Array<number>, };"#,
        },
        SchemaMetadata { name: "JobTurn", version: 1, typescript: "export type JobTurn = { job: bigint, operation: JobOperation, step_sequence: bigint, };" },
        SchemaMetadata { name: "KernelMetrics", version: 1, typescript: "export type KernelMetrics = { actors: number, shards: number, packages: number, };" },
        SchemaMetadata { name: "Lane", version: 1, typescript: r#"export type Lane = "Interactive" | "UserVisible" | "Background" | "Maintenance";"# },
        SchemaMetadata { name: "Mailbox", version: 1, typescript: "export type Mailbox = { capacity: number, len: number, };" },
        SchemaMetadata { name: "OriginalActorTurnWork", version: 1, typescript: "export type OriginalActorTurnWork = { clockBinding: bigint, absoluteDeadlineUs: bigint, fuel: bigint };" },
        SchemaMetadata { name: "OriginalActorTurnLoan", version: 1, typescript: "export type OriginalActorTurnLoan = { retainedPolicy: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, workFuel: bigint, deadlineUs: bigint };" },
        SchemaMetadata { name: "Origin", version: 1, typescript: r#"export type Origin = { "kind": "ui", window: WindowId, } | { "kind": "actor", id: ActorId, } | { "kind": "kernel" } | { "kind": "bus", topic: string, };"# },
        SchemaMetadata { name: "PackageHash", version: 1, typescript: "export type PackageHash = number[];" },
        SchemaMetadata { name: "PackageId", version: 1, typescript: "export type PackageId = string;" },
        SchemaMetadata {
            name: "Payload",
            version: 1,
            typescript: r#"export type Payload = { "kind": "event", bytes: Array<number>, } | { "kind": "suspend", operation: JobOperation, appliedProgress: bigint, } | { "kind": "resume", operation: JobOperation, checkpoint: JobCheckpoint, } | { "kind": "cancel", seq: bigint, } | { "kind": "jobStep", turn: JobTurn, };"#,
        },
        SchemaMetadata { name: "RetainedTurnInput", version: 3, typescript: "export type RetainedTurnInput = { operation: bigint, generation: bigint, epoch: bigint, work: OriginalActorTurnWork, grant: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, };" },
        SchemaMetadata { name: "RetainedTurnReceipt", version: 3, typescript: "export type RetainedTurnReceipt = { input: RetainedTurnInput, spent: { copiedItems: number, copiedBytes: number, retainedCapacityBytes: number, releasedBytes: number }, workUnitsSpent: bigint, remainingFuel: bigint, remaining: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, };" },
        SchemaMetadata { name: "RuntimeMetricsSnapshot", version: 1, typescript: "export type RuntimeMetricsSnapshot = { kernel: KernelMetrics, actors: Array<ActorMetricsSample>, shards: Array<ShardMetricsSample>, sampled_at_ms: bigint, };" },
        SchemaMetadata { name: "SceneSnapshot", version: 1, typescript: "export type SceneSnapshot = { revision: bigint, committed_ms: bigint, patches: Array<number>, node_count: number, };" },
        SchemaMetadata { name: "ShardId", version: 1, typescript: "export type ShardId = number;" },
        SchemaMetadata { name: "ShardKind", version: 1, typescript: r#"export type ShardKind = "Native" | "WebWorker" | "Process";"# },
        SchemaMetadata { name: "ShardMetrics", version: 1, typescript: "export type ShardMetrics = { actors: number, busy_ratio: number, heartbeat_age_ms: number, };" },
        SchemaMetadata { name: "ShardMetricsSample", version: 1, typescript: "export type ShardMetricsSample = { shard: ShardId, metrics: ShardMetrics, };" },
        SchemaMetadata { name: "ShardTable", version: 1, typescript: "export type ShardTable = { kind: ShardKind, shard_count: number, exclusive_reserve: number, assignment: Record<string, ShardId>, exclusive_leases: Record<string, ActorId>, };" },
        SchemaMetadata { name: "TurnGrant", version: 2, typescript: "export type TurnGrant = { actor: ActorId, shard: ShardId, budget: Budget, retained: RetainedTurnInput, loan: OriginalActorTurnLoan, envelopes: Array<Envelope>, };" },
        SchemaMetadata {
            name: "TurnResult",
            version: 1,
            typescript: "export type TurnResult = { retained_receipt: RetainedTurnReceipt, ui_patches: Array<number>, effects: Array<number>, command_ingress: Array<number>, cold_pair_ingress: ColdPairIngressStatus, lifecycle_receipt: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifecycleReceipt | null, ui_patch_receipt: import(\"../../🚪️lifetime/🩹️patch/🟦️.ts\").ActorUiPatchReceipt | null, next_wake: bigint | null, status: TurnStatus, usage: Usage, };",
        },
        SchemaMetadata {
            name: "ColdPairIngressStatus",
            version: 1,
            typescript: r#"export type ColdPairIngressStatus = { "kind": "idle" } | { "kind": "pageAccepted", cursor: ColdDocumentPairCursor, } | { "kind": "backpressure", cursor: ColdDocumentPairCursor, } | { "kind": "loading", cursor: ColdDocumentPairCursor, } | { "kind": "applied", receipt: ColdDocumentPairApplied, } | { "kind": "fault", cursor: ColdDocumentPairCursor, fault: Array<number>, };"#,
        },
        SchemaMetadata {
            name: "ColdDocumentPairCursor",
            version: 1,
            typescript: "export type ColdDocumentPairCursor = { lifetime: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifetime, transfer_generation: bigint, page_index: number, page_count: number, };",
        },
        SchemaMetadata {
            name: "ColdDocumentPairFrontier",
            version: 1,
            typescript: "export type ColdDocumentPairFrontier = { document_id: string, head_edit_ordinal: bigint, head_edit_id: string, last_commit_seq: bigint, chain_sha256: Array<number>, };",
        },
        SchemaMetadata {
            name: "ColdDocumentPairApplied",
            version: 1,
            typescript: "export type ColdDocumentPairApplied = { lifetime: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifetime, transfer_generation: bigint, baseline_frontier: ColdDocumentPairFrontier, aggregate_sha256: Array<number>, };",
        },
        SchemaMetadata {
            name: "TurnStatus",
            version: 1,
            typescript: r#"export type TurnStatus = { "kind": "idle" } | { "kind": "moreWork" } | { "kind": "checkpointReady", checkpoint: JobCheckpoint, } | { "kind": "faulted", detail: Array<number>, } | { "kind": "previewReady", preview: Array<number>, sequence: bigint, } | { "kind": "commitReady", candidate: JobCommitCandidate, } | { "kind": "cancelled" };"#,
        },
        SchemaMetadata { name: "Usage", version: 1, typescript: "export type Usage = { fuel: bigint, wall_us: bigint, memory_bytes: bigint, };" },
        SchemaMetadata { name: "WindowId", version: 1, typescript: "export type WindowId = number;" },
    ];

    /// 🔍️ Rejects unversioned, duplicate, or name-mismatched schema rows before generation.
    pub fn validate() -> Result<(), String> {
        let mut names = HashSet::with_capacity(TYPES.len());
        for metadata in TYPES {
            if metadata.version == 0 {
                return Err(format!("schema `{}` has version zero", metadata.name));
            }
            if !names.insert(metadata.name) {
                return Err(format!("duplicate schema `{}`", metadata.name));
            }
            let prefix = format!("export type {} = ", metadata.name);
            if !metadata.typescript.starts_with(&prefix) {
                return Err(format!("schema `{}` declaration has a mismatched name", metadata.name));
            }
        }
        Ok(())
    }

    /// 🟦️ Renders the stable language projection consumed by the TypeScript package.
    pub fn render_typescript() -> String {
        let mut output = String::from("/** @generated by `repo run @semio-tech/framework-actor-rs:typegen` from 🎭️actor owned schema metadata. Do not edit. */\n\n");
        for metadata in TYPES {
            output.push_str(metadata.typescript);
            output.push_str("\n\n");
        }
        output
    }
}

//#region 🔖️Typegen
#[semio_framework_async_macros::async_test]
async fn exports_typescript_bindings() {
    schema_metadata::validate().unwrap();
    let rendered = schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../🤖️generated/🎭️actor/🟦️.ts"));
    }
}
//#endregion 🔖️Typegen
