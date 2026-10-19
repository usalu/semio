// #region 🧲️Header
/** 📥️ The drive of one inbound request over its turns, bounded by the turn budget. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { type WireVariant, wireTurnStatusTag } from "../🔖️turn-result/🟦️.ts";
import { wireRespondAnswer } from "../🟦️.ts";
// #endregion 🔌️Imports

//#region 📥️InboundRequest
/** ⏱️ How many guest turns ONE inbound `request` may take to produce its `respond` — the ABI's own
 * "answered … within a bounded number of turns, or by spawning a job" (`🔌️plugin/🧬️schema/📜️.wit`'s
 * `request-event`). A capability handler is a pure synchronous function, so the answer lands on turn
 * 1; the budget exists so a guest that parks a request instead of answering it fails loudly at the
 * caller rather than leaving an outstanding completion forever. */
export const INBOUND_REQUEST_TURN_BUDGET = 64;

/** 📥️ One inbound call, as this side of the ABI states it. `originInstanceId` becomes the request's
 * `message-endpoint::shell` origin — the instance the answer is being fetched FOR; `0` means the
 * shell itself asked, owning no instance of the callee. */
export type InboundRequestDrive = {
  readonly req: bigint;
  readonly capability: string;
  readonly payload: Uint8Array;
  readonly originInstanceId: number;
  /** 🔁️ Submits one turn on the CALLEE's actor and hands back its result — each target keeps its own
   * scheduling, activation and serialization discipline; this module owns only the protocol. Typed on
   * the two fields the protocol reads, not on {@link WireTurnResult}, so a target carrying a richer
   * turn record of its own satisfies it structurally without converting. */
  readonly submit: (events: readonly { readonly kind: string; readonly payload: unknown }[]) => Promise<InboundRequestTurn>;
  readonly turnBudget?: number;
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: { readonly capability: string; readonly turns: number; readonly budget: number }) => void;
};

/** 🔁️ The slice of one turn result {@link driveInboundRequest} reads: the effects it scans for a
 * matching `respond`, and the status that says whether the guest has work left to drain. */
export type InboundRequestTurn = { readonly effects: readonly WireVariant[]; readonly status?: unknown };

export type InboundRequestAnswer =
  | { readonly status: "answered"; readonly turns: number; readonly result: { readonly ok: Uint8Array } | { readonly fault: Uint8Array } }
  | { readonly status: "cancelled"; readonly turns: number }
  | { readonly status: "unanswered"; readonly turns: number };

/**
 * 📥️ Drives ONE `Event::Request` to its `respond` on the callee's actor — the host half of the
 * ABI's single inbound-call seam, shared verbatim by the React `PluginRuntime` door and the wgpu
 * `plugin-bridge` one so the two targets can never drift into two protocols (this module's own
 * header: the "third divergent copy" hazard).
 *
 * The request is submitted once; afterwards the actor is drained with EMPTY turns while it still
 * reports `more-work`, because a guest that parked the request answers on a later turn nobody else
 * would drive. Cancellation and progress are read at turn boundaries only — a turn already handed to
 * the worker is never half-abandoned. Fault vocabulary is deliberately NOT chosen here: each target
 * maps `cancelled`/`unanswered` onto its own typed refusal.
 */
export async function driveInboundRequest(drive: InboundRequestDrive): Promise<InboundRequestAnswer> {
  const budget = drive.turnBudget ?? INBOUND_REQUEST_TURN_BUDGET;
  const answerFor = (turn: InboundRequestTurn): { readonly ok: Uint8Array } | { readonly fault: Uint8Array } | null => {
    for (const effect of turn.effects) {
      if (effect.tag !== "respond") continue;
      const { respond } = wireRespondAnswer(effect);
      if (respond.req === drive.req) return respond.result;
    }
    return null;
  };
  let turn = await drive.submit([
    { kind: "request", payload: { req: drive.req, params: { origin: { tag: "shell", val: drive.originInstanceId }, capability: drive.capability, payload: Array.from(drive.payload) } } },
  ]);
  for (let turns = 1; turns <= budget; turns += 1) {
    const result = answerFor(turn);
    if (result) return { status: "answered", turns, result };
    if (drive.signal?.aborted === true) return { status: "cancelled", turns };
    drive.onProgress?.({ capability: drive.capability, turns, budget });
    if (wireTurnStatusTag(turn.status) !== "more-work") return { status: "unanswered", turns };
    turn = await drive.submit([]);
  }
  return { status: "unanswered", turns: budget };
}
//#endregion 📥️InboundRequest
