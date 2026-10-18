/** 🔁️ The settle loop: UI continuation limits and cadence, the retained-UI close ladder, command-ingress ceilings and the turn settle and acknowledgement functions.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { decodePackValue } from "@semio-tech/framework/packs";
import { decodeAppFrame, faultDisplayMessage } from "@semio-tech/framework/app-channel";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../📃️UiDocumentStore/🟦️.tsx";
import { retainedUiIntakeStepCeiling } from "../../📃️UiDocumentStore/📥️intake/🟦️.ts";
import { type ShardActorActivationLease } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts";
import { type ShardEventEnvelope } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { type Lane } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts";
import { hostContinuations } from "@semio-tech/framework/async/continuation";
import { hopTrace } from "@semio-tech/framework/trace";
import { wireTurnStatusTag } from "../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import {
  leftoverShellInvocationFrames,
  shellFrameBytes,
  wirePatchSurfaceId,
  type WireTurnResult,
  type WireVariant
} from "../🪝️wire-codec/🟦️.ts";
import { consumeTypedOperationEffects, typedOperationAcknowledgements, TypedOperationCall } from "../📬️typed-result/🟦️.ts";
import { DEFAULT_SHARD_BUDGET } from "../🛰️runtime-singletons/🟦️.ts";
import { submitPluginTurn } from "../🧵️turn-dispatch/🟦️.ts";
import { retainTurnPresence } from "../🖼️surface-refresh/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
//#region 🔖️GenericThunkQueue
/** 🎚 Catalog mesh registration is Background so reserved/user verbs stay Interactive and are not starved. */
export function commandIngressLaneForActionV1(actionId: string): "Interactive" | "Background" {
  return actionId === "registerBrushMesh" ? "Background" : "Interactive";
}

/** 🏷️ A command waiter hangs forever when the outcome has no in_reply_to matching its seq. */
export function commandIngressNeedsReplyStampV1(replySequences: readonly (number | null)[], seq: number | null): boolean {
  return seq !== null && !replySequences.some((reply) => reply === seq);
}

/** 📥️ Host round trips ONE admitted page of a command may cost, read off the reactor's own admission
 * chain (`⚛️reactor/🔄️turn/🦀️.rs`): the page is admitted on the turn it arrives (`page-accepted`) and
 * the assembled command is exchanged on the next one (`command-pending` → `command-complete`). */
const COMMAND_INGRESS_TURNS_PER_PAGE = 2;

/** 📥️ The drain's LIVENESS backstop for one command, derived from the command's own shape instead of
 * chosen: its page count priced at {@link COMMAND_INGRESS_TURNS_PER_PAGE}, with one whole
 * {@link PLUGIN_UI_CONTINUATION_BATCH_SIZE} of slack per page for the worker drive's own cadence.
 *
 * 🐛️ It replaces the literal `1_024` the drain used to count to. That number was never derived from
 * anything — a maximal 67-page command happens to land near it, which is why it looked adequate — and
 * because it is a COUNT with no progress rule behind it, a command the reactor had silently stopped
 * owning cost 1 024 round trips before the host said anything, and what it then said named no cause:
 * `command ingress did not complete within 1024 continuations (observed statuses: idle)`, measured on
 * the served procedural editor at 1 024 crossings in 352 ms per occurrence
 * (`🗑️generated/react-gen-wire/flow-wire/console.txt`, 2026-09-15 10:30). The backstop is never the
 * progress guarantee — {@link commandIngressUnownedV1} is. */
export function commandIngressContinuationCeilingV1(pages: number): number {
  return Math.max(1, pages) * COMMAND_INGRESS_TURNS_PER_PAGE * PLUGIN_UI_CONTINUATION_BATCH_SIZE;
}

/** 📥️ Whether THIS continuation proves the reactor owns no ingress for the command being drained.
 *
 * The reactor answers `idle` for exactly one reason now: no retained owner holds this cursor. A turn
 * that holds one answers `command-pending` even when it advanced nothing this turn
 * (`command_ingress_owner_cursor`'s rule) — so "the ingress status is idle" and "the actor itself is
 * idle" together are not slowness, they are absence, and no number of further crossings changes it.
 * One continuation is therefore the whole progress budget for that shape. */
export function commandIngressUnownedV1(ingressTag: string | undefined, turnStatusTag: string | undefined): boolean {
  return (ingressTag ?? "idle") === "idle" && turnStatusTag !== "more-work";
}
//#endregion 🔖️GenericThunkQueue

//#region 🔖️PluginTurnScheduler
/** 🧺️ Splits one turn's patch page into consecutive ADMISSION ROUNDS: a surface may appear at most once
 * per round, a repeat opens the next round, and the guest's publication order survives unchanged (the
 * returned numbers are the patch's own index in `turn.uiPatches`, which is what
 * `captureUiPatchAuthority(turn.original, index)` is addressed by).
 *
 * 🐛️ A turn page holds EVERY patch that was ready when the turn assembled its result (the batching lane
 * removed the one-patch floor on 2026-09-15), and a run's first progress tick re-dirties the tool-run
 * panel while its own first patch is still in the page — so one turn legitimately carries
 * `[…, "1:framework.panel.toolRun", …, "1:framework.panel.toolRun"]` as two consecutive deltas of the
 * same surface. The host admitted all of them BEFORE acknowledging any, so the second met the first's
 * still-open wire and receipt outbox in `OwnedUiInstance.beginPatch` and threw
 * `plugin-ui.intake-rejected:intake:Foreign or busy instance surface owner`; the throw skipped the whole
 * round's close, and the panel's cell stayed busy for the rest of the instance's life — every later
 * refresh and every `toolRunAbort`/`toolRunFinalize` rejected (peer ticket
 * 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, `abort-finalize-midrun-*.txt`, :6013 2026-09-15 17:58).
 * Two deltas of one surface are a chain, not a conflict: they must be applied one after the other, which
 * is exactly what a round boundary is. */
export function uiPatchAdmissionRoundsV1(surfaceIds: readonly (string | null)[]): readonly (readonly number[])[] {
  const rounds: number[][] = [];
  let current: number[] = [];
  let seen = new Set<string>();
  for (const [index, surfaceId] of surfaceIds.entries()) {
    const key = surfaceId ?? `unnamed#${index}`;
    if (seen.has(key)) {
      rounds.push(current);
      current = [];
      seen = new Set<string>();
    }
    seen.add(key);
    current.push(index);
  }
  if (current.length > 0) rounds.push(current);
  return rounds;
}

export function hasRequiredUiPatches(results: readonly WireTurnResult[], requiredSurfaceIds?: ReadonlySet<string>): boolean {
  if (!requiredSurfaceIds) return results.some((result) => result.uiPatches.length > 0);
  if (requiredSurfaceIds.size === 0) return true;
  const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
  return [...requiredSurfaceIds].every((surface) => published.has(surface));
}

export const PLUGIN_UI_CONTINUATION_LIMIT = 4_096;
export const PLUGIN_UI_CONTINUATION_BATCH_SIZE = 8;
/** ⛔️ Consecutive continuations one settle may spend making NO progress — publishing no patch,
 * emitting no effect and producing no acknowledgement — before it is declared stalled and named.
 *
 * A continuation is a host round trip, and {@link PLUGIN_UI_CONTINUATION_LIMIT} counts every one of
 * them the same way, so a guest that answers `MoreWork` while publishing nothing spins 4 096 round
 * trips and then throws a timeout that names neither the stalled surface nor the reason. That is the
 * shape the Nakagin `interactionSelect`/`interactionHover` fault has (ticket 26/09/02, evidence
 * `w-ab-41-nakagin-brush4.txt`: `acks=0` for over a thousand consecutive continuations).
 *
 * The bound is derived from the retained document contract this renderer admits, not chosen: one
 * patch may carry at most `maxPatchBytes`, and the smallest owner a single guest retirement turn is
 * guaranteed to release is one maximal text (`maxTextBytes`), so a HEALTHY guest needs at most
 * `maxPatchBytes / maxTextBytes` publication-free turns per admitted patch; one whole continuation
 * batch of slack per such turn covers this loop's own acknowledgement cadence. */
export const PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT = Math.ceil(DEFAULT_UI_DOCUMENT_LIMITS.maxPatchBytes / DEFAULT_UI_DOCUMENT_LIMITS.maxTextBytes) * PLUGIN_UI_CONTINUATION_BATCH_SIZE;
/** 😴️ The same bound read the other way: consecutive publication-free continuations a settle that has
 * NOTHING outstanding spends before it calls the actor quiesced FOR THIS TURN and returns what it
 * collected. One bound, two outcomes — a settle waiting on named surfaces that never come is a fault
 * ({@link pluginTurnStalledError}); a settle that asked for nothing has simply collected everything
 * this turn produces, and the poll behind `subscribeOperationCompletions` owns what comes later.
 *
 * It replaces ticket 26/09/02's `empty-required stop`, which ended such a turn at the FIRST
 * acknowledgement-free continuation. A guest keeps running its typed operation across continuations
 * and publishes nothing until it has something to publish, so that stop cut every mutation's turn
 * before its own publication: `addObjectKind`/`addTargetVolume` settled with
 * `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0` and the edit never reached the world
 * lane (`📓️2026-09-11-wave-B11-editor-actions.md` §6, reproduced in §2 of W-B14's report). */
export const PLUGIN_UI_QUIESCENT_CONTINUATIONS = PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT;
/** 📏️ Liveness backstop for one surface patch, priced off the retained document contract this renderer
 * admits — see {@link retainedUiIntakeStepCeiling} for why a patch-scaled budget is structurally wrong. */
export const PLUGIN_UI_INTAKE_STEP_CEILING = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS);
/** 🚪️ Outer close steps ONE retained node may cost, read off `OwnedUiInstance.closeStep`'s own
 * branches: a cell is observed, its page and receipt outbox are drained, its wire is retired (input
 * retirement, then release), its surface is closed child-step by child-step, and the cell is finally
 * unlinked — every one of which `closeChild` reports as another `pending` to the outer ladder. Twelve
 * branches, rounded to the next power of two so the bound is a ceiling and not a fit. */
export const PLUGIN_UI_CLOSE_STEPS_PER_NODE = 16;
/** 🚪️ Liveness backstop for ONE whole-instance UI close, priced off what that close has to RETIRE.
 *
 * A close retires every surface cell, every wire cursor and every node of every retained surface the
 * instance holds — a quantity that scales with the DOCUMENT and with how many surfaces this instance
 * kept, so the bound is `surfaces × maxNodes × {@link PLUGIN_UI_CLOSE_STEPS_PER_NODE}`. The ladder
 * used to spend {@link PLUGIN_UI_CONTINUATION_LIMIT} instead, which is a SETTLE bound (host round
 * trips for ONE turn, by its own docstring) and has nothing to do with retirement: retiring a
 * converged generation3d editor instance threw `plugin-ui.owner-close-budget-exhausted` on 1 of 4 role
 * switches, naming neither a phase nor a count (`📓️role-switch-regression-2026-09-14.md` §6).
 *
 * The ceiling is a backstop and never the progress guarantee — {@link PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS}
 * is. `📓️close-ladder-budget-2026-09-12.md` §3.1 is right that raising a budget is not a fix; this is
 * not a raise but a re-derivation of a number that was measuring the wrong thing, landed together with
 * a STRICTER stall rule (33 steps to a named fault, where 4 096 unnamed ones used to grind first).
 *
 * What it is compared against is a count of close TURNS — runs of consecutive steps reporting the same
 * ladder phase — and not of steps. `closeChild` caps every step at ONE item however large the grant
 * (`uiGrant` offers 256), so a step count is a count of the retained BYTES under an instance and has
 * nothing to do with the surfaces this bound is derived from: that mismatch is the same per-byte ladder
 * the Flow domain's own session close had, where a 32 KB retained document needed 5 719 close turns
 * against a declared bound of 4 096 (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️flow-surface-followup-2026-09-15.md` §2.4). A turn is one retained item, so the count and the
 * ceiling now measure the same quantity and a converged instance cannot exhaust it. */
export const retainedUiCloseStepCeilingV1 = (surfaces: number): number => Math.max(1, surfaces) * DEFAULT_UI_DOCUMENT_LIMITS.maxNodes * PLUGIN_UI_CLOSE_STEPS_PER_NODE;
/** 🚪️ Consecutive close steps that may release NOTHING before the ladder is declared stalled.
 *
 * The ceiling above is a backstop, never the progress guarantee — the guarantee is byte-aware, the
 * same rule `OwnedUiPatchIntake` already applies to a mint (32 consecutive steps carrying neither an
 * item nor a byte). Every progressing branch of `OwnedUiInstance.closeStep` reports the bytes it
 * released; a ladder that keeps answering with none is not slow, it is stuck, and the fault names the
 * phase it is stuck in instead of a step count. */
export const PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS = 32;
/** 🪜️ One step of a retained-UI close ladder, as `OwnedUiInstance.closeStep` reports it. */
export interface RetainedUiCloseLadderStepV1 { readonly phase: string; readonly bytes: number }
/** 🪜️ The running account of ONE whole-instance UI close: how many TURNS it has spent and the fault
 * that ends it, if any. */
export interface RetainedUiCloseLadderV1 { readonly ceiling: number; readonly turns: number; admit(step: RetainedUiCloseLadderStepV1): string | null }
/** 🪜️ The namespace `OwnedUiInstance.closeStep` gives its OWN branches — the instance's retained items,
 * one turn each: a lookup, a work-queue release, an input retirement, a receipt outbox, a wire, a
 * surface, the instance itself. Every other phase a close step reports arrives through `closeChild`
 * from a descendant and is payload UNDER the item the ladder is on, measured live on the converged
 * hexagonal column at `node-field-retire:1085`, `typed-object-retire:1282`,
 * `prepared-scene-source-close:3920` against `instance-surface-release:14`
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `🗑️generated/react-retire/phases/`). */
const RETAINED_UI_CLOSE_OWNER_PHASE_PREFIX = "instance-";
/** 🪜️ Prices a whole-instance UI close in retained ITEMS, never in the bytes under them.
 *
 * `closeChild` caps every ladder step at ONE item however large the grant (`uiGrant` offers 256), so
 * counting steps counts an instance's retained BYTES — the quantity
 * {@link retainedUiCloseStepCeilingV1} is explicitly NOT derived from. A turn here is one step in
 * {@link RETAINED_UI_CLOSE_OWNER_PHASE_PREFIX}'s namespace, so the count and the ceiling measure the
 * same quantity and a converged instance cannot exhaust it. The byte-aware
 * {@link PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS} rule stays on the STEPS, because a stall is a property
 * of a step and not of a turn (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function createRetainedUiCloseLadderV1(retainedSurfaces: number): RetainedUiCloseLadderV1 {
  const ceiling = retainedUiCloseStepCeilingV1(retainedSurfaces);
  let turns = 0;
  let idle = 0;
  return {
    ceiling,
    get turns() { return turns; },
    admit(current: RetainedUiCloseLadderStepV1): string | null {
      if (current.phase.startsWith(RETAINED_UI_CLOSE_OWNER_PHASE_PREFIX)) {
        turns += 1;
        if (turns > ceiling) return `plugin-ui.owner-close-budget-exhausted:${current.phase} after ${turns - 1} turns over ${retainedSurfaces} retained surfaces`;
      }
      idle = current.bytes > 0 ? 0 : idle + 1;
      return idle > PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS ? `plugin-ui.owner-close-stalled:${current.phase} released nothing for ${idle} steps` : null;
    },
  };
}
/** 🪃️ Intake steps per macrotask yield. One yield per 8 steps is the continuation cadence for TURNS,
 * where a step is a whole guest turn; an intake step is one wire phase costing ~3.4 µs, and a
 * Nakagin-scale world-3d publication takes 669 403 of them (`OwnedIntake drives a Nakagin-scale paged
 * scene-lane surface patch`), so that cadence spent 83 675 macrotasks — dwarfing the work itself. A
 * 1024-step stride is a ~3.5 ms slice, still well inside one frame, and 653 macrotasks for that patch. */
export const PLUGIN_UI_INTAKE_YIELD_STRIDE = 1_024;

/** 🪃️ Whether THIS intake step owes the macrotask yield — a SYNCHRONOUS predicate on purpose.
 *
 * 🐛️ It used to be an `async` helper every intake step awaited, so a drive that yields once per
 * {@link PLUGIN_UI_INTAKE_YIELD_STRIDE} steps still allocated a promise and spent two microtask ticks
 * on each of the other 1 023. An intake step is a wire phase costing ~3.4 µs and a world-3d surface
 * publication takes hundreds of thousands of them, so the wrapper was a constant multiplier on the
 * whole retained-UI admission — measured on the React door at 222 ms per `flowEvalTick` hop under the
 * `turn.accept` span (`📓️react-guest-turn-cost-2026-09-14.md`). The wgpu shell's frame worker carried
 * the identical defect on its own intake drive (`📓️wgpu-edit-convergence-perf-2026-09-14.md`:
 * "allocated a promise per decoder phase"). The yield CADENCE is unchanged — this is the same stride,
 * asked without paying for a promise to answer "no". */
export function uiIntakeOwesYieldV1(step: number): boolean {
  return step % PLUGIN_UI_INTAKE_YIELD_STRIDE === 0;
}
/** 🩺️ Why one settle ended, how many crossings it spent and how much it carried — pure accounting, read
 * by `refresh.turn`'s span detail so "the refresh returned before its own surface was ready" is a
 * measurement instead of an inference. `owed` is the one that names the defect: a settle that returned
 * while a surface it asked for was still reconciling. */
type SettleOutcomeV1 = Readonly<{ readonly stop: "idle" | "quiesced" | "sliced" | "ceiling"; readonly continuations: number; readonly patches: number; readonly owed: boolean }>;

export type PluginPatchAcceptance = Readonly<{
  acknowledgements: readonly ShardEventEnvelope[];
  turns: readonly WireTurnResult[];
}>;
function isPluginPatchAcceptance(value: readonly ShardEventEnvelope[] | PluginPatchAcceptance): value is PluginPatchAcceptance { return !Array.isArray(value); }

/** 🪃️ Macrotask yield for the UI continuation loops, delegated to the host's ONE
 * {@link hostContinuations} scheduler — the same object the guest's `Effect::DispatchAction` re-arm
 * goes through, so there is a single owner of "run this again, soon" on this target rather than one
 * primitive per loop. The hazard it exists for is unchanged: a `setTimeout(0)` chain is throttled to
 * one tick per second in a hidden tab (once per minute under Chrome's intensive throttling), which
 * turned a few thousand intake steps into minutes of silence; a `MessageChannel` message is a
 * macrotask page visibility never throttles. */
/** 🎞️ Wall slice a draining settle keeps the actor once a continuation asked the shell for a refresh mid-operation (an
 * unsolicited `Invocation` frame, `in_reply_to` 0, carrying the operation's dirty scope). A long typed operation — a tool
 * run testing thousands of candidates — used to run to completion inside ONE settle, holding the actor's ingress while the
 * refreshes it asked for queued behind it, so the renderer saw nothing and then everything at once. Ending the settle after
 * one frame hands the rest to the yielding operation drain, and the queued refreshes interleave with its polls: the
 * operation never waits on the renderer, and the renderer shows as much of the process as it keeps up with. */
export const PLUGIN_OPERATION_REFRESH_SLICE_MS = 16;

/** 🎞️ The LONGEST a draining settle may hold the actor past that slice because a surface it asked for
 * is still reconciling — `DEFAULT_SHARD_BUDGET.wallMs`, the wall the host grants ONE crossing, which is
 * also the wall the worker's own `MoreWork` drive spends before it crosses back.
 *
 * 🐛️ The owed clause below had no wall at all, and `settleOwesRequestedSurfacesV1` cannot tell "the
 * guest is still reconciling MY surface" from "the guest is in `more-work` for something else": a live
 * tool run keeps the guest in `more-work` for the whole run, so an owed refresh held the actor from the
 * first frame to the last. Measured on release :6013 with a 100-object fill (ticket
 * 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, `🔍️w5-fill-timeline-probe.ts --settle=20000`): 3
 * refreshes instead of 12, one `refresh.turn` lasting 1 281 ms, and the scene stuck at 36–40 of 100
 * provisional objects twenty seconds after the run completed. One crossing's wall is the grace the
 * owed surface gets; past it the refresh lane is let back in and re-asks, which is the whole point of
 * a lane that coalesces. */
export const PLUGIN_OPERATION_OWED_SLICE_MS = DEFAULT_SHARD_BUDGET.wallMs;

/** 🎞️ Whether a draining settle should return now so a refresh the operation asked for can run: the operation asked for one,
 * the slice is spent, and nothing is owed — no acknowledgement to submit, no required surface still unpublished, and no
 * surface this settle itself ASKED the guest to publish still reconciling.
 *
 * 🐛️ The last clause used to be missing, and `outstanding` alone could never supply it: a refresh whose windows are all
 * already retained declares an EMPTY required-surface set, which makes `hasRequiredUiPatches` vacuously true and
 * `outstanding` permanently false. So a refresh of a window whose body the guest was still reconciling sliced out one
 * crossing in, projected the tree it already held, and the patch it had asked for landed on some later unrelated drain
 * turn — the peer's 250–800 ms per-request latency, ~1 scene update per second during a live tool run
 * (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS phase 8). A settle may hand the actor back to the operation's own
 * refresh lane only once it owes that lane nothing itself. */
export function settleYieldsToRefreshV1(results: readonly WireTurnResult[], startedMs: number, nowMs: number, acknowledgementsOwed: boolean, outstanding: boolean, owed = false): boolean {
  if (acknowledgementsOwed || outstanding) return false;
  return nowMs - startedMs >= (owed ? PLUGIN_OPERATION_OWED_SLICE_MS : PLUGIN_OPERATION_REFRESH_SLICE_MS) && results.some((result) => effectsRequestOperationProgressV1(result.effects));
}

/** 🪟️ Whether a surface this settle ASKED to be made visible is still owed: the guest has not published it in this
 * settle and is still answering `more-work`, so the drive that finishes the reconcile is THIS one. An `idle` guest owes
 * nothing — an unchanged body publishes no patch by contract (`PatchTracker` emits nothing for an unchanged tree), so
 * this predicate must never be the reason a settle waits on a guest that has stopped. */
function settleOwesRequestedSurfacesV1(results: readonly WireTurnResult[], requestedSurfaceIds: ReadonlySet<string> | undefined): boolean {
  if (!requestedSurfaceIds?.size) return false;
  if (wireTurnStatusTag(results.at(-1)?.status) !== "more-work") return false;
  return !hasRequiredUiPatches(results, requestedSurfaceIds);
}

/** 🎞️ Whether turn effects carry an operation progress refresh: an `Invocation` frame answering no command with a dirty scope. */
export function effectsRequestOperationProgressV1(effects: readonly WireVariant[]): boolean {
  return leftoverShellInvocationFrames(effects).some((frame) => "Invocation" in frame && frame.Invocation.in_reply_to === 0 && frame.Invocation.ui_scope.length > 0);
}

export async function yieldPluginUiContinuation(): Promise<void> {
  await hostContinuations.yieldContinuation();
}

/** 🛑️ The attributable twin of the continuation timeout: it names the surfaces the settle is still
 * waiting on instead of only counting the round trips it spent. A settle waiting on named surfaces
 * reports the ones that never published; a drain reports the surfaces it DID publish, because those
 * are the owners whose retirement is holding the guest in `MoreWork`. */
/** 🧯️ The guest-side `Error` shell frames a settle collected (`shell_fault_effect` in
 * `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — a dirty surface whose render faulted, a refused commit), decoded to
 * their display messages. A settle that throws because a requested surface never published would
 * otherwise discard exactly the frame that says WHY (the render fault rides the same turn's effects,
 * which `retainedUiRefreshEffects` only reads after a successful settle), leaving only
 * `missing=[…], status=idle` to debug from. */
function settleShellFaultMessages(actorId: string, results: readonly WireTurnResult[]): string[] {
  const instanceId = Number(actorId.slice(actorId.lastIndexOf("#") + 1));
  if (!Number.isFinite(instanceId)) return [];
  const messages: string[] = [];
  for (const result of results) {
    for (const effect of result.effects) {
      const bytes = shellFrameBytes(effect, instanceId);
      if (!bytes) continue;
      try {
        const frame = decodeAppFrame(bytes);
        if ("Error" in frame) messages.push(faultDisplayMessage(frame.Error.fault, decodePackValue));
      } catch (error) {
        messages.push(`undecodable shell frame: ${error instanceof Error ? error.message : String(error)}`);
      }
    }
  }
  return messages;
}

function pluginTurnStalledError(actorId: string, results: readonly WireTurnResult[], requiredSurfaceIds: ReadonlySet<string> | undefined, zeroProgress: number, continuations: number, call?: TypedOperationCall): Error {
  const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
  const missing = [...(requiredSurfaceIds ?? [])].filter((surface) => !published.has(surface));
  const pending = missing.length > 0 ? missing : [...published];
  return new Error(
    `PluginRuntime: actor ${actorId} published, acknowledged and emitted nothing for ${zeroProgress} consecutive continuations ` +
      `(operation=${call?.label ?? "none"}, pending=${JSON.stringify(pending)}, required=${JSON.stringify([...(requiredSurfaceIds ?? [])])}, ` +
      `continuations=${continuations}, status=${wireTurnStatusTag(results.at(-1)?.status)}, zeroProgressLimit=${PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT})`,
  );
}

/** 🔄️ Drives a reactor-owned continuation until its requested patch set is published or the actor
 * quiesces. UI reconciliation is deliberately incremental: the turn that marks surfaces dirty may
 * publish them across multiple MoreWork frames, and a guest running a typed operation publishes
 * nothing at all until that operation has something to publish — so a turn ends on PROGRESS, never on
 * one acknowledgement-free round trip. A supplied empty `requiredSurfaceIds` means every requested
 * surface is already retained, so an unchanged refresh needs no continuation at all; it does not mean
 * the actor has nothing left to hand this turn. Accepted patches are acknowledged between turns to
 * release bounded publication capacity. See {@link PLUGIN_UI_QUIESCENT_CONTINUATIONS} for the two
 * outcomes of a publication-free streak. */
export async function settlePluginTurn(actorId: string, initial: WireTurnResult, lane: Lane, requiredSurfaceIds?: ReadonlySet<string>, acceptPatches?: (result: WireTurnResult) => readonly ShardEventEnvelope[] | PluginPatchAcceptance | Promise<readonly ShardEventEnvelope[] | PluginPatchAcceptance>, drainOperations = false, activation?: ShardActorActivationLease, call?: TypedOperationCall, requestedSurfaceIds?: ReadonlySet<string>, report?: (outcome: SettleOutcomeV1) => void): Promise<WireTurnResult> {
  const results: WireTurnResult[] = [initial];
  const acknowledge = async (result: WireTurnResult): Promise<readonly ShardEventEnvelope[]> => {
    activation?.assertActive();
    retainTurnPresence(result);
    const accepted = await acceptPatches?.(result) ?? [];
    if (!isPluginPatchAcceptance(accepted)) return [...accepted, ...typedOperationAcknowledgements(result)];
    results.push(...accepted.turns);
    return [...accepted.acknowledgements, ...typedOperationAcknowledgements(result), ...accepted.turns.flatMap(typedOperationAcknowledgements)];
  };
  const hasWork = () => (drainOperations || !hasRequiredUiPatches(results, requiredSurfaceIds)) && wireTurnStatusTag(results.at(-1)?.status) === "more-work";
  const outstanding = () => !hasRequiredUiPatches(results, requiredSurfaceIds);
  const owed = () => settleOwesRequestedSurfacesV1(results, requestedSurfaceIds);
  let acknowledgements = await hopTrace.timeAsync("turn.accept", { actorId, patches: initial.uiPatches.length }, () => acknowledge(initial));
  let quiesced = false;
  let sliced = false;
  let zeroProgress = 0;
  const startedMs = performance.now();
  let continuations = 0;
  for (let continuation = 0; !quiesced && !sliced && (acknowledgements.length > 0 || hasWork()) && continuation < PLUGIN_UI_CONTINUATION_LIMIT; continuation += 1) {
    continuations = continuation + 1;
    const collected = results.length;
    const continued = await submitPluginTurn(actorId, acknowledgements, lane, undefined, undefined, activation);
    results.push(continued);
    acknowledgements = await hopTrace.timeAsync("turn.accept", { actorId, patches: continued.uiPatches.length }, () => acknowledge(continued));
    const closeDecide = hopTrace.open("turn.decide", { actorId, results: results.length });
    const progressed = acknowledgements.length > 0 || results.slice(collected).some((turn) => turn.uiPatches.length > 0 || turn.effects.length > 0);
    zeroProgress = progressed ? 0 : zeroProgress + 1;
    if (zeroProgress >= PLUGIN_UI_QUIESCENT_CONTINUATIONS && !outstanding()) quiesced = true;
    else if (zeroProgress >= PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT) { closeDecide(); throw pluginTurnStalledError(actorId, results, requiredSurfaceIds, zeroProgress, continuation + 1, call); }
    sliced = drainOperations && !quiesced && settleYieldsToRefreshV1(results, startedMs, performance.now(), acknowledgements.length > 0, outstanding(), owed());
    const yielding = !quiesced && !sliced && (continuation + 1) % PLUGIN_UI_CONTINUATION_BATCH_SIZE === 0 && hasWork();
    closeDecide();
    if (yielding) await hopTrace.timeAsync("turn.yield", { actorId }, () => yieldPluginUiContinuation());
  }
  if (!quiesced && !sliced && (acknowledgements.length > 0 || hasWork())) {
    const published = results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null));
    throw new Error(
      `[TRACE] PluginRuntime: actor ${actorId} did not publish its requested UI surfaces within ${PLUGIN_UI_CONTINUATION_LIMIT} continuations ` +
        `(required=${JSON.stringify([...(requiredSurfaceIds ?? [])])}, published=${JSON.stringify(published)}, ` +
        `effects=${results.reduce((count, result) => count + result.effects.length, 0)}, status=${wireTurnStatusTag(results.at(-1)?.status)}, faults=${JSON.stringify(settleShellFaultMessages(actorId, results))})`,
    );
  }
  if (requiredSurfaceIds?.size && !hasRequiredUiPatches(results, requiredSurfaceIds)) {
    const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
    const missing = [...requiredSurfaceIds].filter((surface) => !published.has(surface));
    throw new Error(`PluginRuntime: actor ${actorId} stopped without publishing requested UI surfaces (missing=${JSON.stringify(missing)}, status=${wireTurnStatusTag(results.at(-1)?.status)}, faults=${JSON.stringify(settleShellFaultMessages(actorId, results))})`);
  }
  activation?.assertActive();
  report?.({
    stop: quiesced ? "quiesced" : sliced ? "sliced" : hasWork() ? "ceiling" : "idle",
    continuations,
    patches: results.reduce((count, result) => count + result.uiPatches.length, 0),
    owed: owed(),
  });
  return {
    uiPatches: results.flatMap((result) => result.uiPatches),
    effects: consumeTypedOperationEffects(results.flatMap((result) => result.effects), call),
    nextWake: [...results].reverse().find((result) => result.nextWake !== null)?.nextWake ?? null,
    status: results.at(-1)?.status,
    commandIngress: results.at(-1)?.commandIngress,
  };
}
export async function settleAcknowledgedPluginTurns(actorId: string, results: readonly WireTurnResult[], acknowledgements: readonly ShardEventEnvelope[], acceptPatches?: (result: WireTurnResult) => PluginPatchAcceptance | Promise<PluginPatchAcceptance>, activation?: ShardActorActivationLease, call?: TypedOperationCall): Promise<WireTurnResult> {
  for (const settled of results) retainTurnPresence(settled);
  const initial: WireTurnResult = {
    uiPatches: [],
    effects: [],
    nextWake: null,
    commandIngress: results.at(-1)?.commandIngress,
    status: results.at(-1)?.status,
  };
  const continued = await settlePluginTurn(actorId, initial, "Interactive", new Set(), (turn) => turn === initial ? acknowledgements : acceptPatches?.(turn) ?? [], true, activation, call);
  return {
    ...continued,
    uiPatches: [...results.flatMap((turn) => turn.uiPatches), ...continued.uiPatches],
    effects: consumeTypedOperationEffects([...results.flatMap((turn) => turn.effects), ...continued.effects], call),
    nextWake: continued.nextWake ?? [...results].reverse().find((turn) => turn.nextWake !== null)?.nextWake ?? null,
  };
}
//#endregion 🔖️PluginTurnScheduler
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime
