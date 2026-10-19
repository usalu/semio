/** 🏁️ Per-instance carriers of a typed operation's host effects and terminal output, the completion fan-out registry and the bounded drain loop.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { drainTypedOperationTurns as driveTypedOperationDrain } from "@semio-tech/framework/actor/wire-turn";
import { type PluginOperationCompletion } from "../🧷️handle-contract/🟦️.ts";
import { type WireVariant } from "../🪝️wire-codec/🟦️.ts";
import { TYPED_OPERATION_PENDING_OUTPUT, TYPED_OPERATION_TERMINAL_OUTPUT } from "../📬️typed-result/🟦️.ts";
import { yieldPluginUiContinuation } from "../⏩️continuation-yield/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/** 🎯️ Per-instance "leftover" `TurnResult.effects` — everything a turn produced that was NOT a
 * `SendMessage{Shell}` reply frame (the old `AppFrame::Effects` wrapper's replacement, design-abi.md
 * §2). `runQueuedTurn` fills this on every turn; `performInvocation` drains it right after its own
 * `client.command()` call resolves — both operations share one turn, so this is never stale by more
 * than the caller's own await. */
export const pendingTurnEffects = new Map<number, WireVariant[]>();

/** 🎯️ Per-instance leftover effects produced by the CONTINUATION turns
 * {@link drainTypedOperations} drives, kept separate from {@link pendingTurnEffects}: that map is
 * owned by `performInvocation`, which drains it the microtask after its own command reply resolves.
 * A completion frame can ride the very outcome that resolves a command, so a shared map would let the
 * completion subscriber steal an in-flight invocation's own effects. Two owners, two carriers. */
export const pendingCompletionEffects = new Map<number, WireVariant[]>();

/** 🏁️ One instance's single `AppChannelClient.onOperationCompleted` registration and the subscribers
 * it fans out to. ONE registration per instance is the whole point: a completion's carriers live in
 * {@link pendingCompletionEffects}, which the publishing callback must DRAIN (an undrained map grows
 * without bound), so a second channel-level registration on the same instance would find it already
 * emptied and publish a completion with no effects and no terminal output at all. The shell really
 * does subscribe twice on one instance — the visible session's completion pass and the Home directory
 * bootstrap's receipt settle both bind `session.instanceId` — and before this fanout the second
 * subscriber silently received nothing. */
export const completionFanouts = new Map<number, { readonly listeners: Set<(completion: PluginOperationCompletion) => void>; readonly dispose: () => void }>();

/** 🧾️ The single terminal value a completed typed operation published, read out of the accumulated
 * completion leftover. `consumeTypedOperationEffects` tags that value `terminal-output` when the
 * TERMINAL page rode the same turn and `pending-output` when it did not — across a multi-turn drain
 * both spellings reach here, and the completion frame itself is the terminal witness, so either is the
 * operation's terminal result.
 *
 * One publication seen on BOTH carriers is still one output: a fast operation's admitting command turn
 * parks the receipt as `pending-output` and its completion carries the same receipt as `terminal-output`
 * (measured on the signed-in `s` Home, hub 7800, ticket 26/09/23 C10 — every directory page stalled at
 * "Updating directory" because the identical receipt was counted twice). Two DIFFERENT values remain a
 * contract violation, exactly as in {@link invocationFromFrames}. */
function typedOperationTerminalOutputV1(leftover: readonly WireVariant[]): unknown {
  const [first, ...rest] = leftover.filter((effect) => effect.tag === TYPED_OPERATION_TERMINAL_OUTPUT || effect.tag === TYPED_OPERATION_PENDING_OUTPUT);
  if (first !== undefined && rest.some((effect) => !sameTerminalValueV1(effect.val, first.val))) throw new Error("typed-operation returned more than one terminal output");
  return first?.val;
}

/** ⚖️ Structural equality of two decoded pack values (primitives, byte arrays, arrays, records) — key order never matters. */
function sameTerminalValueV1(left: unknown, right: unknown): boolean {
  if (Object.is(left, right)) return true;
  if (left instanceof Uint8Array || right instanceof Uint8Array) return left instanceof Uint8Array && right instanceof Uint8Array && left.length === right.length && left.every((byte, index) => byte === right[index]);
  if (Array.isArray(left) || Array.isArray(right)) return Array.isArray(left) && Array.isArray(right) && left.length === right.length && left.every((value, index) => sameTerminalValueV1(value, right[index]));
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return false;
  const leftKeys = Object.keys(left);
  return leftKeys.length === Object.keys(right).length && leftKeys.every((key) => Object.hasOwn(right, key) && sameTerminalValueV1((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]));
}

/** 🧾️ The settled operation's terminal value, taken from WHICHEVER carrier the turn it settled on
 * parked it in, and removed from that carrier so it is delivered exactly once.
 *
 * 🐛️ Two carriers, because a mounted operation may settle on either kind of turn: a continuation turn
 * the drain drove parks it in {@link pendingCompletionEffects}, and a command turn whose own settle ran
 * the operation to its terminal parks it in {@link pendingTurnEffects}. Reading only the first left
 * `terminalOutput` undefined for every operation that finished inside its own command turn — which is
 * every FAST one, including the `s` Home directory bootstrap once its config publication stopped
 * blocking (ticket 26/09/18 S8). Only the terminal-output entries are taken from the invocation's
 * carrier, never its requested effects: the completion must not steal what the in-flight invocation
 * owns, and an `output` that the admitting reply is contractually the OPERATION HANDLE of must not be
 * overwritten by the receipt in {@link invocationFromFrames}. */
export function takeTypedOperationTerminalOutputV1(instanceId: number): unknown {
  const completionLeftover = pendingCompletionEffects.get(instanceId) ?? [];
  const turnLeftover = pendingTurnEffects.get(instanceId) ?? [];
  const carried = turnLeftover.filter((effect) => effect.tag === TYPED_OPERATION_TERMINAL_OUTPUT || effect.tag === TYPED_OPERATION_PENDING_OUTPUT);
  if (carried.length > 0) pendingTurnEffects.set(instanceId, turnLeftover.filter((effect) => !carried.includes(effect)));
  return typedOperationTerminalOutputV1([...completionLeftover, ...carried]);
}

/** 📏️ Fixed retained authority for {@link pendingCompletionEffects}: an operation publishes at most
 * one host effect per continuation turn, so a completion that never arrives would otherwise
 * accumulate without bound. Exceeding it fails closed rather than growing. */
export const PLUGIN_OPERATION_EFFECT_CAPACITY = 4_096;

/** 📏️ Continuation turns {@link drainTypedOperations} submits before it gives up on one instance —
 * the drain twin of {@link PLUGIN_UI_CONTINUATION_LIMIT}, which bounds ONE settle rather than the
 * poll chain across settles. */
export const PLUGIN_OPERATION_DRAIN_BUDGET = 4_096;

/** ⏱️ Upper bound on the re-poll delay a `TurnResult.next-wake` may request. `next-wake` has no
 * settled host-side unit yet (nothing else in this file consumes it), so it is honoured only as
 * "wake again, no later than this" — clamping keeps a stale or absolute value from parking a
 * finished operation's publication for minutes. */
export const PLUGIN_OPERATION_WAKE_MAX_MS = 1_000;

/** 🔁️ The bounded poll loop behind {@link PluginWasmHandle.subscribeOperationCompletions}: submit
 * one continuation settle, stop the moment the actor reports anything other than `more-work`, and
 * yield one macrotask between polls so a long drain never starves rendering. Injected `live`/`settle`
 * (rather than a closure over one `loadPluginModule` scope) so the stop conditions this whole feature
 * rests on — stops at idle, stops when the instance closes, never exceeds its budget — are assertable
 * without a shard worker. */
export async function drainTypedOperationTurns(
  budget: number,
  live: () => boolean,
  settle: () => Promise<{ readonly status: unknown; readonly nextWake: number | null }>,
  yieldTurn: () => Promise<void> = yieldPluginUiContinuation,
): Promise<{ readonly polls: number; readonly stopped: "idle" | "closed" | "budget"; readonly nextWake: number | null }> {
  return driveTypedOperationDrain(budget, live, settle, yieldTurn);
}
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime
