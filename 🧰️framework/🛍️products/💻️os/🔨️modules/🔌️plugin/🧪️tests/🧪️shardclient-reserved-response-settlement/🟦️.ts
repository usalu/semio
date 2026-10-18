import { createShardClientSettlementTestKit } from "./🧰️settlement-test-kit/🟦️.ts";
import type { TestSource } from "./🧰️settlement-test-kit/🟦️.ts";
import { registerShardWorkerBootstrapTests } from "./🚀️worker-bootstrap/🟦️.ts";
import { registerShardResidentCompositionTests } from "./💾️resident-composition/🟦️.ts";
import { registerReservedResponseSettlementTests } from "./🧾️reserved-response-settlement/🟦️.ts";
import { registerCapturedReturnAuthorityTests } from "./🪪️captured-return-authority/🟦️.ts";
import { registerCapturedInstanceLifecycleTests } from "./🔒️captured-instance-lifecycle/🟦️.ts";
import { registerActivationLeaseTests } from "./🎟️activation-lease/🟦️.ts";
import { registerInstanceCloseAndRoundTripTests } from "./🚪️instance-close-and-round-trip/🟦️.ts";
import { registerSegmentedDownloadAndMultiplexingTests } from "./📥️segmented-download-and-multiplexing/🟦️.ts";
import { registerHeartbeatAndLivenessTests } from "./💓️heartbeat-and-liveness-watchdogs/🟦️.ts";
import { registerLeaseWatchdogAndShardLossTests } from "./🛰️lease-watchdog-and-shard-loss/🟦️.ts";
import { registerEnvelopeOrderingAndFrameTests } from "./📨️envelope-ordering-and-frames/🟦️.ts";
import { registerHostEffectBridgeTests } from "./🌉️host-effect-bridge/🟦️.ts";
import { registerInboundActivationAuthorityTests } from "./🛂️inbound-activation-authority/🟦️.ts";
import { registerJobWireTests } from "./💼️job-wire/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts").ShardClientTestDependenciesV1, testSource: TestSource): Promise<void> {
  const kit = createShardClientSettlementTestKit(vitest, dependencies, testSource);
  const { vi } = kit;
  registerShardWorkerBootstrapTests(kit);
  registerShardResidentCompositionTests(kit);
  //#region 🚪️CapturedLifecycle
  registerReservedResponseSettlementTests(kit);
  registerCapturedReturnAuthorityTests(kit);
  registerCapturedInstanceLifecycleTests(kit);
  //#endregion 🚪️CapturedLifecycle
  //#region 🪪️ActivationLease
  registerActivationLeaseTests(kit);
  //#endregion 🪪️ActivationLease
  registerInstanceCloseAndRoundTripTests(kit);
  registerSegmentedDownloadAndMultiplexingTests(kit);
  registerHeartbeatAndLivenessTests(kit);
  registerLeaseWatchdogAndShardLossTests(kit);
  //#region 📨️ShardFrame tests
  registerEnvelopeOrderingAndFrameTests(kit);
  //#endregion 📨️ShardFrame tests
  //#region 🌉️HostEffectBridge tests
  registerHostEffectBridgeTests(kit);
  //#endregion 🌉️HostEffectBridge tests
  //#region 🪪️InboundActivation
  registerInboundActivationAuthorityTests(kit);
  //#endregion 🪪️InboundActivation
  //#region 💼️JobWire
  registerJobWireTests(kit);
  //#endregion 💼️JobWire
  void vi;
}
