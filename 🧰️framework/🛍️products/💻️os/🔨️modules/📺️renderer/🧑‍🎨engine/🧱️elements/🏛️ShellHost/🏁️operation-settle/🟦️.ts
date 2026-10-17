/** 🏁️ Operation-settle slot and watchdog constants, the uncommitted-edit count, started typed operations and typed-operation cancellation.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { shellDialogSessionIsCurrentV1, type ShellDialogSessionV1 } from "../🗨️dialog-origin/🟦️.ts";
import { type ShellInputActionV1 } from "../🎯️input-ledger/🟦️.ts";
import { type ShellHistoryProjectionV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";

//#region FrameworkOsShell
//#region 🏁️OperationSettle
/** 🏁️ Concurrent `onAction` callers that may wait on a typed operation's terminal completion at once.
 * Sized for a live desktop session (a few background tick loops plus whatever the user is driving) —
 * a caller past the ceiling settles immediately rather than growing an unbounded map. */
export const OPERATION_SETTLE_WAITER_SLOTS = 32;
/** 🏁️ Completions remembered for a waiter that has not registered yet — the admitting reply and the
 * `OperationCompleted` frame race, and the completion can win. Older ids fall off the ring. */
export const OPERATION_SETTLE_RING_SLOTS = 64;
/** 🏁️ Ceiling on how long an `onAction` promise may wait for a completion frame that never arrives (a
 * faulted worker, a torn-down instance). Past it the caller resumes: a background tick loop must be
 * able to recover from a lost frame, never wedge on one. */
export const OPERATION_SETTLE_WATCHDOG_MS = 30_000;

/** 📌️ Uncommitted-since-last-checkpoint count, derived purely from a projection's entries (no new wire field): every applied
 * `mutation`-kind entry counts, reset to 0 the moment a `commitCheckpoint` (`kind: "history"`) entry is seen — mirrors
 * `store::uncommitted_edit_ids`'s own "since the last Change" semantics closely enough for an auto-checkin heuristic
 * (undo/redo of an already-committed edit is the one case this under/over-counts by one entry). */
export function shellUncommittedEditCountV1(entries: ShellHistoryProjectionV1["entries"]): number {
  let pending = 0;
  for (const entry of Object.values(entries).sort((left, right) => left.seq - right.seq)) {
    if (entry.kind === "history" && entry.actionId === "commitCheckpoint") pending = 0;
    else if (entry.kind === "mutation" && entry.applied !== false) pending += 1;
  }
  return pending;
}

/** 🏁️ The typed-operation id an admitting `InvocationResult` started, or `undefined` when the action
 * started none. `dispatch_typed_command_inner` (`🔌️plugin/🦀️.rs`) answers a started operation with
 * `output = { operationId, generation }`, both decimal STRINGS; `OperationCompletionV1.operation` is
 * the same id as an exact u64 bigint, so this is where the two representations meet. */
export type StartedTypedOperationV1 = Readonly<{ operationId: string; generation: string; operation: bigint; generationValue: bigint }>;

/** 🎯️ Parses both exact u64 authorities from one typed-operation admission. */
export function startedTypedOperationV1(output: unknown): StartedTypedOperationV1 | undefined {
  if (typeof output !== "object" || output === null) return undefined;
  const row = output as { readonly operationId?: unknown; readonly generation?: unknown };
  const exact = (value: unknown): bigint | undefined => {
    if (typeof value !== "string" || !/^(0|[1-9][0-9]*)$/u.test(value)) return undefined;
    const number = BigInt(value);
    return number <= 0xffff_ffff_ffff_ffffn ? number : undefined;
  };
  const operation = exact(row.operationId);
  const generationValue = exact(row.generation);
  return operation === undefined || generationValue === undefined ? undefined : { operationId: row.operationId as string, generation: row.generation as string, operation, generationValue };
}

const CANCEL_TYPED_OPERATION_ACTION_ID_V1 = "cancelTypedOperation";
const exactU64HexV1 = (value: bigint): string => value.toString(16).padStart(16, "0");

/** ⏹️ Builds the framework cancellation gesture with the original controller, provenance and exact operation authority. */
export function typedOperationCancellationActionV1(origin: ShellInputActionV1, operation: StartedTypedOperationV1): ShellInputActionV1 {
  return { ...origin, action: CANCEL_TYPED_OPERATION_ACTION_ID_V1, args: { operationId: exactU64HexV1(operation.operation), generation: exactU64HexV1(operation.generationValue) } };
}

/** 🔒️ Refuses a coincident operation id after its admitting program or document owner retired. */
export function typedOperationCancellationOwnerIsCurrentV1(admitted: ShellDialogSessionV1, target: ShellDialogSessionV1, ownerCurrent: boolean): boolean {
  return ownerCurrent && shellDialogSessionIsCurrentV1(admitted, target);
}
//#endregion 🏁️OperationSettle
//#endregion FrameworkOsShell

export function operationSettleKeyV1(instanceId: number, operation: bigint): string { return `${instanceId}:${operation}`; }
