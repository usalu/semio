/** 📚️ The worker-law suites: every browser-worker behaviour suite that runs against the neutral worker test bag, registered in their original order from the worker's in-source block. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";
import { registerSessionBrokerPortTests } from "../../🔨️modules/📇️directory/🧪️tests/🔑️session-broker-port/🟦️.ts";
import { registerSpaceArtifactCreationTests } from "../🏗️space-artifact-creation/🟦️.ts";
import { registerDirectoryEventPageBootstrapTests } from "../../🔨️modules/📇️directory/🧪️tests/📄️event-page-bootstrap/🟦️.ts";
import { registerHubSessionCapabilityTests } from "../../🔨️modules/📇️directory/🧪️tests/🎟️hub-session-capability/🟦️.ts";
import { registerDocumentActorRecoveryWorkerTests } from "../🚨️document-actor-recovery-worker/🟦️.ts";
import { registerWireBridgeTests } from "../🌉️wire-bridge/🟦️.ts";
import { registerArtifactBootstrapRestoreTests } from "../🧱️artifact-bootstrap-restore/🟦️.ts";
import { registerIdentityConfigFacetTests } from "../../🎚️config/🧪️tests/🪪️identity-config-facet/🟦️.ts";
import { registerConfigMutationParityTests } from "../../🎚️config/🧪️tests/🔧️config-mutation-parity/🟦️.ts";
import { registerSpaceAdministrationTests } from "../../🔨️modules/📇️directory/🧪️tests/🏘️space-administration/🟦️.ts";
import { registerDirectoryLaneTests } from "../../🔨️modules/📇️directory/🧪️tests/🛤️directory-lane/🟦️.ts";
import { registerDocumentOpenTests } from "../📂️document-open/🟦️.ts";
import { registerExecutionTargetLeaseTests } from "../🎯️execution-target-lease/🟦️.ts";
import { registerBrowserActorReservationTests } from "../🎭️browser-actor-reservation/🟦️.ts";
import { registerDocumentBackboneBatchTests } from "../📨️document-backbone-batches/🟦️.ts";
import { registerFolderWatchTests } from "../👀️folder-watch/🟦️.ts";
import { registerHubReconnectOutboxTests } from "../📴️hub-reconnect-outbox/🟦️.ts";
import { registerScopeSafePresenceTests } from "../👥️scope-safe-presence/🟦️.ts";
import { registerCheckpointPairDecoderTests } from "../📍️checkpoint-pair-decoder/🟦️.ts";
import { registerDocumentCatchUpTests } from "../🪜️document-catch-up/🟦️.ts";
import { registerActorCommandWindowTests } from "../🧭️actor-command-windows/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

/** 🔭️ Rebases the worker's own test source onto the OS root every fixture path is spelled against. */
export function workerLawSourceV1(registrar: TestSource): TestSource {
  return { directory: registrar.directory, url: new URL("../../../🟦️.ts", registrar.url).href };
}

export async function registerWorkerLawTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, registrar: TestSource): Promise<void> {
  const source = workerLawSourceV1(registrar);
  await registerSessionBrokerPortTests(vitest, dependencies, source);
  await registerSpaceArtifactCreationTests(vitest, dependencies, source);
  await registerDirectoryEventPageBootstrapTests(vitest, dependencies, source);
  await registerHubSessionCapabilityTests(vitest, dependencies, source);
  await registerDocumentActorRecoveryWorkerTests(vitest, dependencies, source);
  await registerWireBridgeTests(vitest, dependencies, source);
  await registerArtifactBootstrapRestoreTests(vitest, dependencies, source);
  await registerIdentityConfigFacetTests(vitest, dependencies, source);
  await registerConfigMutationParityTests(vitest, dependencies, source);
  await registerSpaceAdministrationTests(vitest, dependencies, source);
  await registerDirectoryLaneTests(vitest, dependencies, source);
  await registerDocumentOpenTests(vitest, dependencies, source);
  await registerExecutionTargetLeaseTests(vitest, dependencies, source);
  await registerBrowserActorReservationTests(vitest, dependencies, source);
  await registerDocumentBackboneBatchTests(vitest, dependencies, source);
  await registerFolderWatchTests(vitest, dependencies, source);
  await registerHubReconnectOutboxTests(vitest, dependencies, source);
  await registerScopeSafePresenceTests(vitest, dependencies, source);
  await registerCheckpointPairDecoderTests(vitest, dependencies, source);
  await registerDocumentCatchUpTests(vitest, dependencies, source);
  await registerActorCommandWindowTests(vitest, dependencies, source);
}
