// #region 🧲️Header
/** 🚚️ The more-work drive of a shard turn: what a result carries, when it may be dropped and how many steps a grant affords. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { wireTurnStatusTag } from "../🔖️turn-result/🟦️.ts";
// #endregion 🔌️Imports

//#region 🚚️MoreWorkDrive
/** 🤫️ Every field of WIT `turn-result` (`🔌️plugin/🧬️schema/📜️.wit`) a host reads, as one shape. It is
 * spelled out rather than reused from {@link WireTurnResult} because {@link shardTurnCarriesNothingV1}
 * decides whether a result may be DROPPED, and a carrier it forgot would be dropped silently. */
export type ShardTurnCarriers = {
  readonly uiPatches?: unknown;
  readonly effects?: unknown;
  readonly presence?: unknown;
  readonly nextWake?: unknown;
  readonly lifecycleReceipt?: unknown;
  readonly uiPatchReceipt?: unknown;
  readonly commandIngress?: unknown;
  readonly coldPairIngress?: unknown;
  readonly status?: unknown;
};

/** 🤫️ Whether one reactor turn result carried NOTHING the host could act on — no ui patch, no
 * effect, no presence, no wake, neither receipt, and both ingress lanes idle.
 *
 * This is the whole safety argument of the shard worker's silent-turn hold: a result this predicate
 * admits may be discarded without telling anyone, because the ONLY thing it said was `more-work`, and
 * `more-work` on the far side of a Worker boundary means "pump me again" — which the worker can do
 * itself. `fuelUsed` is the single `turn-result` field with no host reader and is therefore the one
 * omission, stated rather than implied. */
export function shardTurnCarriesNothingV1(result: ShardTurnCarriers | null | undefined): boolean {
  if (!result || typeof result !== "object") return false;
  if (!Array.isArray(result.uiPatches) || result.uiPatches.length !== 0) return false;
  if (!Array.isArray(result.effects) || result.effects.length !== 0) return false;
  if (result.presence !== undefined && (!Array.isArray(result.presence) || result.presence.length !== 0)) return false;
  if (result.nextWake !== null && result.nextWake !== undefined) return false;
  if (result.lifecycleReceipt !== undefined && result.lifecycleReceipt !== null) return false;
  if (result.uiPatchReceipt !== undefined && result.uiPatchReceipt !== null) return false;
  if (!isIdleIngress(result.commandIngress)) return false;
  if (!isIdleIngress(result.coldPairIngress)) return false;
  return true;
}

function isIdleIngress(status: unknown): boolean {
  return Boolean(status) && typeof status === "object" && (status as { readonly tag?: unknown }).tag === "idle";
}

/** 🚚️ How many consecutive guest turns ONE worker-owned drive may run before it crosses back anyway —
 * the wall the host GRANTED for this crossing, divided by what one guest turn measures.
 *
 * Twin of `⚛️reactor/🔄️turn/🦀️.rs`'s `more_work_drive_steps`, and held equal to it by the shared
 * contract `⚛️reactor/🧫️fixtures/🚚️more-work-drive.json`.
 *
 * 🐛️ It replaces a WALL bound of `SHARD_TURN_SILENT_HOLD_MS = 8`, which was the reactor's own
 * executor slice (`run_until_deadline(64, 256 KiB, now + 8 ms)`) borrowed as a drive budget. That was
 * incoherent from the day it landed: a whole guest turn measures ~13 ms on the procedural 3d React
 * door — the executor slice plus everything else a turn does — so an 8 ms wall admitted
 * `floor(8 / 13) = 0` further turns and the hold degenerated into "one extra poll"
 * (`📓️reactor-reconcile-spin-2026-09-14.md` §7 item 2 named it and did not take it). The bound is now
 * a step COUNT derived from the measurement, and its wall is
 * {@link shardTurnDriveBudgetMsV1} — the steps priced at that same measurement, never more than the
 * grant that produced them. */
export function shardTurnDriveStepsV1(grantWallMs: number, guestTurnCostMs: number): number {
  if (!Number.isFinite(grantWallMs) || !Number.isFinite(guestTurnCostMs) || guestTurnCostMs <= 0) return 1;
  return Math.max(1, Math.floor(grantWallMs / guestTurnCostMs));
}

/** 🚚️ The WALL one drive may spend — the whole grant, never the grant rounded down to whole measured
 * turns.
 *
 * The step count above is the grant's DERIVED EXPECTATION, and its job is the coherence law: it is what
 * says a grant of 100 ms fits seven 13 ms turns while the reactor's own 8 ms slice fits none. It is not
 * the runtime cap, because rounding the wall down to `steps × cost` makes the drive cross back EMPTY
 * with part of its own grant unspent whenever turns come in cheaper than they measured — and a drive
 * that crosses carrying nothing is the exact cost this whole lane exists to remove.
 *
 * 🐛️ What that rounding cost, measured: a live tool run on the puzzle 3d shell requests window
 * refreshes every 100–400 ms, its world-window patches cost ~30 ms of host intake each, and they
 * arrived **250–800 ms** after the request rather than once per request (ticket
 * `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` `📓️status.md` phase 8, `🔍️w5-fill-timeline-probe.ts
 * --profile`). A reconcile that needs more turns than the rounded wall admits is one the drive hands
 * back unfinished, and the patch then leaves on some later unrelated drain turn.
 *
 * A grant smaller than one measured turn still buys one whole turn, for the same reason
 * {@link shardTurnDriveStepsV1} always admits one. */
export function shardTurnDriveBudgetMsV1(grantWallMs: number, guestTurnCostMs: number): number {
  if (!Number.isFinite(grantWallMs) || grantWallMs <= 0) return Math.max(0, guestTurnCostMs);
  return Math.max(grantWallMs, guestTurnCostMs);
}

/** 🚚️ The WORKER-owned `MoreWork` drive, as a law rather than as inlined worker text.
 *
 * In the browser the host round trip IS the guest's pump: there is no self-driving loop on the far
 * side of the worker boundary, so a reactor that answers `more-work` while publishing nothing costs a
 * POST, a poll, a structured clone and a main-thread pickup to be asked again. Measured on the
 * procedural 3d React door, 2026-09-15: **32.9 of 60.7 worker crossings per `flowEvalTick` hop posted
 * no events and returned no patch** (`📓️ui-turn-patch-batching-2026-09-15.md` §7 item 2).
 *
 * The drive owns that pump. It re-enters the guest until the turn produces something the host can act
 * on, and it crosses back for exactly five other reasons, each named by its stop tag:
 *
 * | stop | means |
 * |---|---|
 * | `carried` | the turn produced something for the host — the ONLY reason the host is told anything |
 * | `idle` | the guest stopped answering `more-work` |
 * | `input` | a host-owned input is pending behind the drive — an ingress message, a cancel, a view-state change — and the drive may not hold the wire while the host has something to say |
 * | `steps` | the derived step ceiling is spent |
 * | `closed` | the actor was disposed or re-activated under a new generation |
 *
 * A result is only ever discarded when {@link shardTurnCarriesNothingV1} admits it, so the drive is
 * lossless by construction. `poll`/`now`/`live`/`inputPending` are injected so every stop is
 * assertable without a Worker, and so the generated worker
 * (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts`, whose inline twin this owns) and any future
 * target drive the identical loop. */
export async function driveShardTurnMoreWorkV1(drive: {
  readonly first: ShardTurnCarriers;
  readonly poll: () => Promise<ShardTurnCarriers>;
  readonly now: () => number;
  readonly live: () => boolean;
  readonly inputPending: () => boolean;
  readonly steps: number;
  readonly budgetMs: number;
}): Promise<{ readonly result: ShardTurnCarriers; readonly polls: number; readonly stopped: "carried" | "idle" | "input" | "steps" | "budget" | "closed" }> {
  const deadline = drive.now() + drive.budgetMs;
  let result = drive.first;
  let polls = 0;
  for (;;) {
    if (wireTurnStatusTag(result.status) !== "more-work") return { result, polls, stopped: "idle" };
    if (!shardTurnCarriesNothingV1(result)) return { result, polls, stopped: "carried" };
    if (drive.inputPending()) return { result, polls, stopped: "input" };
    if (!drive.live()) return { result, polls, stopped: "closed" };
    if (drive.now() >= deadline) return { result, polls, stopped: "budget" };
    if (polls + 1 >= drive.steps) return { result, polls, stopped: "steps" };
    result = await drive.poll();
    polls += 1;
  }
}
//#endregion 🚚️MoreWorkDrive
