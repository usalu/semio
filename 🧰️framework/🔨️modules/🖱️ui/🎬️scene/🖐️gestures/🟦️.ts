//#region 🎚️ContinuousGestureLane
/** 🎚️ What a continuous gesture (a dragged slider, a held spinner) needs from its host: ONE way to
 * send a value, whose promise settles when the receiver has finished with it, and one way to cancel
 * the press (`abort`, the scrub protocol's host cancel — the receiver drops the press with zero
 * trace). A sink that answers `void` makes the lane degenerate into "send everything", which is the
 * shape this type exists to forbid. */
export type ContinuousGestureLanePorts<Value> = Readonly<{
  readonly send: (value: Value, phase: ContinuousGesturePhase) => void | Promise<void>;
  readonly abort?: (reason: ContinuousGestureAbortReason) => void | Promise<void>;
  readonly onFault?: (error: unknown) => void;
}>;

/** 🏁️ `live` is a value the user is still moving through; `commit` is the value they released on.
 * The distinction is the receiver's, not the lane's: the lane guarantees only that the LAST value of
 * a gesture is always sent as `commit`, whatever the traffic before it. */
export type ContinuousGesturePhase = "live" | "commit";

/** 🧯️ Why a host cancels an open press (`🛠️tool-machine` `ToolAbortReason` minus the tool's own abort). */
export type ContinuousGestureAbortReason = "blur" | "captureLost" | "frozen" | "baseMoved" | "retired";

export type ContinuousGestureLane<Value> = {
  readonly offer: (value: Value) => void;
  readonly commit: (value?: Value) => void;
  readonly abort: (reason: ContinuousGestureAbortReason) => void;
  readonly open: () => boolean;
  readonly inFlight: () => boolean;
  readonly owed: () => Value | null;
  readonly sent: () => number;
};

/** 🔢️ Page-wide press serial behind {@link continuousPressIdentity}. */
let continuousPressSerial = 0;

/** 🪪️ A fresh press identity for the control `key`: `<key>:<ms>:<serial>` with a page-wide serial, so two presses of one
 * control in the same millisecond still carry distinct identities and the guest never takes the second for a late tick of
 * the first it already closed (scrub protocol, `📓️api-scrub-machine.md` of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). */
export function continuousPressIdentity(key: string): string {
  continuousPressSerial += 1;
  return `${key}:${Date.now()}:${continuousPressSerial}`;
}

/** 🤝️ Whether a port's answer is a promise to wait for; any other answer settles its round trip at once. */
function isPromiseLike(value: unknown): value is PromiseLike<void> {
  return typeof (value as { readonly then?: unknown } | null | undefined)?.then === "function";
}

/** 🎚️ At most ONE send in flight and at most ONE owed value — always the latest.
 *
 * A 60 Hz drag over a round trip that costs more than 16 ms has exactly two honest answers: queue
 * every value (the user then watches a backlog of stale geometry drain after they let go) or keep
 * only the newest one and send it when the receiver is free. This is the second answer, and it is the
 * same rule the ui-refresh coalescer runs on: the intermediate values a user swept THROUGH were never
 * asked for, only the one they are on now.
 *
 * The release is never dropped. `commit` marks the gesture's last value, and a `commit` is sent even
 * when it equals the value already sent — the receiver needs the release to commit its press as ONE
 * transaction. A release that names no value falls back to what is owed and then to what was last
 * offered, so a gesture whose final value happened to drain before the user let go is still committed;
 * only a lane that was never offered anything at all commits nothing.
 *
 * A release that names no value releases only an open press; a closed lane has nothing to release.
 *
 * A press can be cancelled while it is open (offered and not yet released): the owed value is dropped,
 * the cancel is sent after the round trip in flight and before anything offered later, and the release
 * of the cancelled press is ignored — the receiver drops the press with zero trace.
 *
 * A send that rejects frees the lane; the fault reaches `onFault` and the owed value is still sent,
 * because a gesture must not be wedged by one refused round trip. A port that answers anything but a
 * promise has settled its round trip at once.
 */
export function createContinuousGestureLane<Value>(ports: ContinuousGestureLanePorts<Value>): ContinuousGestureLane<Value> {
  let inFlight = false;
  let owed: { readonly value: Value; readonly phase: ContinuousGesturePhase } | null = null;
  let cancel: ContinuousGestureAbortReason | null = null;
  let last: { readonly value: Value } | null = null;
  let open = false;
  let cancelled = false;
  let sent = 0;
  const pump = (): void => {
    if (inFlight || (owed === null && cancel === null)) return;
    let settled: unknown;
    inFlight = true;
    try {
      if (cancel !== null) {
        const reason = cancel;
        cancel = null;
        settled = ports.abort?.(reason);
      } else {
        const next = owed!;
        owed = null;
        sent += 1;
        settled = ports.send(next.value, next.phase);
      }
    } catch (error) {
      inFlight = false;
      ports.onFault?.(error);
      pump();
      return;
    }
    if (!isPromiseLike(settled)) {
      inFlight = false;
      pump();
      return;
    }
    void Promise.resolve(settled)
      .catch((error: unknown) => ports.onFault?.(error))
      .finally(() => {
        inFlight = false;
        pump();
      });
  };
  return {
    offer(value) {
      last = { value };
      owed = { value, phase: "live" };
      open = true;
      cancelled = false;
      pump();
    },
    commit(value) {
      if (cancelled || (value === undefined && !open)) {
        cancelled = false;
        return;
      }
      const released = value ?? owed?.value ?? last?.value;
      if (released === undefined) return;
      last = { value: released as Value };
      owed = { value: released as Value, phase: "commit" };
      open = false;
      pump();
    },
    abort(reason) {
      if (!open) return;
      owed = null;
      cancel = reason;
      open = false;
      cancelled = true;
      pump();
    },
    open: () => open,
    inFlight: () => inFlight,
    owed: () => owed?.value ?? null,
    sent: () => sent,
  };
}
//#endregion 🎚️ContinuousGestureLane

export const textEditorActions = {
  edit: "textEdit",
  splice: "textSplice",
  select: "textSelect",
  hover: "textHover",
  requestCompletions: "requestCompletions",
  commitRename: "commitRename",
  formatDocument: "formatDocument",
} as const;
