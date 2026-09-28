/** 🚑️ The document actor recovery policy (`🏪️store/👷️worker/🚑️actor-recovery/🔣️.json`) on its own: a loss reopens at once, an
 * applied action or a stable mount starts a fresh count, and a loss during every reopen ends with the typed fault after the
 * bound — never a reconnect storm. cockatiel's `ConsecutiveBreaker` (a success resets, it opens at its threshold) is the
 * independent oracle for the consecutive count: it must open exactly where the policy answers `exhausted`. */
import { describe, expect, it } from "vitest";
import { createRequire } from "node:module";
import { DOCUMENT_ACTOR_RECOVERY_FRESH_V1, DOCUMENT_ACTOR_RECOVERY_V1, documentActorRecoveryStepV1, type DocumentActorRecoveryEventV1 } from "../../🔨️modules/🏪️store/👷️worker/🚑️actor-recovery/🟦️.ts";

const { ConsecutiveBreaker } = createRequire(import.meta.url)("cockatiel") as { ConsecutiveBreaker: new (threshold: number) => { success(): void; failure(): boolean } };
const policy = DOCUMENT_ACTOR_RECOVERY_V1;

describe("🚑️ document actor recovery policy", () => {
  it.each(policy.cases.map((row) => [row.name, row] as const))("%s", (_name, row) => {
    let memory = DOCUMENT_ACTOR_RECOVERY_FRESH_V1;
    const decisions: string[] = [];
    const losses: number[] = [];
    for (const event of row.events as readonly DocumentActorRecoveryEventV1[]) {
      const step = documentActorRecoveryStepV1(memory, event);
      memory = step.memory;
      decisions.push(step.decision);
      losses.push(memory.losses);
    }
    expect(decisions).toEqual(row.decisions);
    expect(losses).toEqual(row.losses);
  });

  it("opens exactly where cockatiel's consecutive breaker opens, for every loss/applied sequence up to eight events", () => {
    for (let mask = 0; mask < 1 << 8; mask += 1) {
      for (let length = 1; length <= 8; length += 1) {
        const breaker = new ConsecutiveBreaker(policy.maxConsecutiveLosses + 1);
        let memory = DOCUMENT_ACTOR_RECOVERY_FRESH_V1;
        let open = false;
        for (let index = 0; index < length; index += 1) {
          const lost = ((mask >> index) & 1) === 1;
          const event: DocumentActorRecoveryEventV1 = lost ? { kind: "lost", cause: "action-unconfirmed", atMs: index } : { kind: "applied", atMs: index };
          const step = documentActorRecoveryStepV1(memory, event);
          memory = step.memory;
          if (!open) {
            if (lost) open = breaker.failure();
            else breaker.success();
          }
          if (lost) expect(step.decision, `mask ${mask} length ${length} index ${index}`).toBe(open ? "exhausted" : "reopen-now");
        }
      }
    }
  });

  it("never answers anything but `exhausted` once the bound is spent, whatever follows", () => {
    let memory = DOCUMENT_ACTOR_RECOVERY_FRESH_V1;
    for (let index = 0; index <= policy.maxConsecutiveLosses; index += 1) memory = documentActorRecoveryStepV1(memory, { kind: "lost", cause: "inbound-frame", atMs: index }).memory;
    expect(memory.exhausted).toBe(true);
    for (const event of [{ kind: "mounted", atMs: 10 }, { kind: "lost", cause: "turn-failed", atMs: 10 + policy.stableAfterMs * 2 }] as const) {
      const step = documentActorRecoveryStepV1(memory, event);
      memory = step.memory;
      if (event.kind === "lost") expect(step.decision).toBe("exhausted");
    }
  });
});
