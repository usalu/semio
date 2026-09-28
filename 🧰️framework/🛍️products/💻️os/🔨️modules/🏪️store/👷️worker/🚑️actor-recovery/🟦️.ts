/** 🚑️ Document actor recovery policy (`🔣️.json`): when a mounted browser actor is lost without anyone asking, the store worker
 * reopens the document at once along its ordinary open path — never a silent replay, never a freeze — and a bounded count of
 * consecutive losses turns a loss during every reopen into one typed fault instead of a reconnect storm (ticket 26/09/23 S18
 * §14c: one unconfirmed invocation left a hub document refusing every later action `action-owner-mismatch`, and nothing
 * reopened it). Pure: the worker feeds it events and acts on the decision. */

import contract from "./🔣️.json" with { type: "json" };

export const DOCUMENT_ACTOR_RECOVERY_V1 = contract;

/** 🚑️ Why an actor was lost: its action's outcome could not be confirmed, its child faulted on an inbound hub frame, or a
 * render turn failed. */
export type DocumentActorLossCauseV1 = "action-unconfirmed" | "inbound-frame" | "turn-failed";

export type DocumentActorRecoveryEventV1 =
  | Readonly<{ kind: "lost"; cause: DocumentActorLossCauseV1; atMs: number }>
  | Readonly<{ kind: "mounted"; atMs: number }>
  | Readonly<{ kind: "applied"; atMs: number }>;

/** 🧠️ What the policy remembers per open document: consecutive losses, when the actor last mounted, and whether the bound
 * was spent (terminal until the document is opened again). */
export type DocumentActorRecoveryMemoryV1 = Readonly<{ losses: number; lastMountedAtMs: number | null; exhausted: boolean }>;

export type DocumentActorRecoveryDecisionV1 = "reopen-now" | "exhausted" | "none";

export const DOCUMENT_ACTOR_RECOVERY_FRESH_V1: DocumentActorRecoveryMemoryV1 = { losses: 0, lastMountedAtMs: null, exhausted: false };

/** 🚑️ One event through the policy: a loss reopens at once while the consecutive count stays within the bound (an actor mounted
 * for `stableAfterMs`, or an applied action, starts a fresh count) and answers `exhausted` past it, for good; a mount is
 * remembered; an applied action clears the count. */
export function documentActorRecoveryStepV1(memory: DocumentActorRecoveryMemoryV1, event: DocumentActorRecoveryEventV1): Readonly<{ memory: DocumentActorRecoveryMemoryV1; decision: DocumentActorRecoveryDecisionV1 }> {
  if (event.kind === "mounted") return { memory: { ...memory, lastMountedAtMs: event.atMs }, decision: "none" };
  if (event.kind === "applied") return { memory: { ...memory, losses: 0 }, decision: "none" };
  if (memory.exhausted) return { memory, decision: "exhausted" };
  const stable = memory.lastMountedAtMs !== null && event.atMs - memory.lastMountedAtMs >= contract.stableAfterMs;
  const losses = (stable ? 0 : memory.losses) + 1;
  const exhausted = losses > contract.maxConsecutiveLosses;
  return { memory: { ...memory, losses, exhausted }, decision: exhausted ? "exhausted" : "reopen-now" };
}
