import { registerActorInstanceCloseFaultTests } from "./🔶️actor-instance-close-fault/🟦️.ts";
import { registerActorArenaHandbackFixtureTests } from "./🔸️actor-arena-handback-fixture/🟦️.ts";
import { registerActorInstanceCloseReceiptsTests } from "./🟠️actor-instance-close-receipts/🟦️.ts";
import type { TestSource } from "./🟢️shared/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES" | "actorInstanceCapturedReceiptMatches" | "actorInstanceCloseReceiptMatches" | "actorInstanceLifecycleReceiptEquals" | "actorInstanceLifetimeEquals" | "decodeActorInstanceLifecycle" | "encodeActorInstanceLifecycle">, source: TestSource): Promise<void> {
  const { it, expect, encodeActorInstanceLifecycle, ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, decodeActorInstanceLifecycle, actorInstanceLifetimeEquals, actorInstanceCloseReceiptMatches, actorInstanceCapturedReceiptMatches, actorInstanceLifecycleReceiptEquals } = registerActorInstanceCloseFaultTests({ dependencies, vitest, source });
  registerActorArenaHandbackFixtureTests({ it, source, expect, encodeActorInstanceLifecycle, ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, decodeActorInstanceLifecycle });
  registerActorInstanceCloseReceiptsTests({ it, source, expect, actorInstanceLifetimeEquals, actorInstanceCloseReceiptMatches, actorInstanceCapturedReceiptMatches, actorInstanceLifecycleReceiptEquals });
}
