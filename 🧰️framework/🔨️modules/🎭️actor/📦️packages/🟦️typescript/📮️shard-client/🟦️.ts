/** 📮️ Public entry `@semio-tech/framework-actor/shard-client`: the explicit surface that importers outside the module use. */
export { ACTOR_RETURN_RESULT_MAXIMUM_BYTES, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, SHARD_LIVENESS_POLICY, ShardClient, ShardClientHub, admitSegmentedDownloadChunk, admitSegmentedDownloadOperationId, decodeActorReturnDrive, decodeActorReturnResult, encodeActorReturnDrive, encodeActorReturnResult, shardClientRetryableLifecycleTestDependenciesV1, shardClientTestDependenciesV1 } from "../../../📮️shard-client/🟦️.ts";
export { MAINTENANCE_LANE_DEFAULT_BUDGET, SHARD_COMMAND_MAXIMUM_PAGES, SHARD_FRAME_VARIANT_FIELDS, createGrantedBudgetTracker, createShardCommandIngressPages, interpretShardFrame, orderEnvelopesByLane } from "../../../📮️shard-client/📜️wire/🟦️.ts";
export { OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt } from "../../../📮️shard-client/🩹️ui-patch/🟦️.ts";
export { OwnedShardReturn, OwnedShardReturnPage, RETRYABLE_LIFECYCLE_TURN, RETRYABLE_LIFECYCLE_TURN_ATTEMPTS, isRetryableLifecycleTurn } from "../../../📮️shard-client/🔐️activation/🟦️.ts";
export { SHARD_JSPI_FAULT_CODE, SHARD_JSPI_FAULT_TEXT, ShardJspiUnavailableError, assertShardJspiAvailable, describeShardMessageError, describeShardSilence, describeShardWorkerError, evaluateShardLiveness, formatShardWorkerFault, isShardLostError, settleFailedInstanceOpen, shardJspiAvailable } from "../../../📮️shard-client/💓️heartbeat/🟦️.ts";
export type { ActorReturnControl, ActorReturnDrive, ActorReturnFault, ActorReturnIdentity, ActorReturnOrigin, ActorReturnPageReceipt, ActorReturnResult, ShardClientRetryableLifecycleTestDependenciesV1, ShardClientTestDependenciesV1 } from "../../../📮️shard-client/🟦️.ts";
export type { InboundMessage, OutboundMessage } from "../../../📮️shard-client/📨️messages/🟦️.ts";
export type { PendingEntry, ShardInstanceOwner, ShardSlot } from "../../../📮️shard-client/🔐️activation/🟦️.ts";
export type { CreateShardWorker, HostEffectHandler, ShardQuotaBreach, ShardWorkerLike } from "../../../📮️shard-client/🚨️quota/🟦️.ts";
export type { GrantedBudgetTracker, ShardAsset, ShardBudget, ShardCapabilityGrant, ShardCommandIngressPage, ShardCommandPageCursor, ShardEnvelope, ShardEventEnvelope, ShardFrame, ShardFrameDispatch, ShardJobBudget, ShardJobStep, ShardMetrics, ShardMetricsSample, ShardOrigin } from "../../../📮️shard-client/📜️wire/🟦️.ts";
export type { OwnedNativeUiPatchValue, OwnedUiPatchAcknowledgementEntry } from "../../../📮️shard-client/🩹️ui-patch/🟦️.ts";
export type { ShardActorActivationLease, ShardInstanceLifecycleLease, ShardInstanceOpenInput, ShardReturnAdmission, ShardReturnReport } from "../../../📮️shard-client/🔐️activation/🟦️.ts";
export type { ShardCarriedBeat } from "../../../📮️shard-client/📨️messages/🟦️.ts";
export type { ShardClientOptions } from "../../../📮️shard-client/⚙️options/🟦️.ts";
export type { ShardJspiScope, ShardLivenessDecision, ShardLivenessWindow, ShardSilenceReport } from "../../../📮️shard-client/💓️heartbeat/🟦️.ts";
