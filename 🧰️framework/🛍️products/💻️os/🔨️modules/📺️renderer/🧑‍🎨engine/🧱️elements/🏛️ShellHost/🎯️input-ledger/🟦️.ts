// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/input-ledger/component.ts
/** @emoji 🎯️ The Input Causality Ledger — every host input (a pointer gesture, a utility or tool toggle,
 * a ribbon action, a guest follow-up) is ONE ledger entry with provenance, and every entry reaches a
 * typed terminal outcome: `applied`, `superseded` or `refused(reason)`. No React, no shell imports, so
 * every law here runs over a fixture without the shell's element graph (the same separation
 * `🔀️surface-switch` keeps).
 *
 * The laws this module carries (ticket 26/09/16/INPUT-CAUSALITY-LEDGER, design §1):
 *   L1 admit or refuse, never drop — an input's outcome is always one of the three above, awaitable;
 *   L2 causal closure — a guest follow-up to input N sorts BEFORE any later input (`causalOrderKeyV1`);
 *   L3 versioned registers — host-owned interaction state is `{ value, generation }` written by CAS
 *      (`createVersionedRegisterV1`), never gated by a wall clock;
 *   L4 batched samples — a gesture's samples are appended to one owed batch, never dropped, with at most
 *      one send in flight (`createGestureSampleLaneV1`, the accumulating twin of
 *      `createContinuousGestureLane`).
 *
 * What this replaced: `ShellHost`'s `lastUtilityArmAtRef` 8 s "echo-off" window (commit 46c3cb9de0),
 * which swallowed a user's toggle-off click within 8 s of arming a utility, and the per-`pointermove`
 * guest round trip that made a one-second marquee drain for 1–3 s after mouseup. */
// #endregion 🧲️Header

//#region 🪪️Provenance
/** 🪪️ Who issued an input. `user` is a DOM gesture or widget press; `guest` is a follow-up the guest armed
 * (`replayShellCommand`, `dispatchAction`, `setActiveUtility` effects); `tutorial` and `replay` are the
 * director/recorder lanes; `tick` is a self-gating background loop. */
export type InputOriginV1 = "user" | "guest" | "tutorial" | "replay" | "tick";

export type InputProvenanceV1 = Readonly<{
  /** 🪟️ The window instance the input addresses, or `null` for a mode-level / windowless input. */
  windowId: string | null;
  /** 🔢️ Monotonic per ledger. */
  inputSeq: number;
  /** 🔗️ The `inputSeq` whose guest answer produced this entry, or `null` for a root input. */
  causedBy: number | null;
  origin: InputOriginV1;
}>;

/** 🎯️ The host-side action shape: the generated wire `ActionDescriptor` fields plus optional provenance.
 * Provenance never crosses to the guest — it is the host's bookkeeping, stamped by the ledger when absent. */
export type ShellInputActionV1 = Readonly<{
  controllerId: string;
  action: string;
  args?: unknown;
  /** 🪪️ `inputSeq` is re-minted by `issue`, so a caller stamping a follow-up names only what it knows. */
  provenance?: Omit<InputProvenanceV1, "inputSeq"> & Partial<Pick<InputProvenanceV1, "inputSeq">>;
}>;

/** 🔗️ The sort key L2 dequeues by: a follow-up inherits its cause's position, a root input its own. */
export function causalOrderKeyV1(provenance: Pick<InputProvenanceV1, "inputSeq" | "causedBy">): number {
  return provenance.causedBy ?? provenance.inputSeq;
}

/** 🔗️ Total order over entries: causal key first, then issue order — a stable sort key for any queue. */
export function compareCausalV1(a: Pick<InputProvenanceV1, "inputSeq" | "causedBy">, b: Pick<InputProvenanceV1, "inputSeq" | "causedBy">): number {
  const key = causalOrderKeyV1(a) - causalOrderKeyV1(b);
  return key !== 0 ? key : a.inputSeq - b.inputSeq;
}
//#endregion 🪪️Provenance

//#region 🚦️Outcome
/** 🚦️ Why an input was refused. Every reason maps to exactly one former silent `return` in `onAction` or one
 * transport rejection, so a refusal always names its gate. */
export type InputRefusalReasonV1 =
  | "no-session"
  | "stale-generation"
  | "instance-sealed"
  | "instance-retired"
  | "queue-full"
  | "undeclared-action"
  | "viewer-read-only"
  | "owner-mismatch"
  | "view-state-unresolved"
  | "mutation-rejected"
  | "dispatch-failed";

export type InputOutcomeV1 =
  | Readonly<{ kind: "applied"; inputSeq: number }>
  | Readonly<{ kind: "superseded"; inputSeq: number; by: number }>
  | Readonly<{ kind: "refused"; inputSeq: number; reason: InputRefusalReasonV1; retryable: boolean; detail?: string }>;

/** 🔁️ Whether a refusal is worth retrying unchanged: a full queue or a stale owner drains; a sealed instance,
 * an undeclared action or a read-only viewer never becomes admissible by waiting. */
export const INPUT_REFUSAL_RETRYABLE_V1: Readonly<Record<InputRefusalReasonV1, boolean>> = {
  "no-session": false,
  "stale-generation": false,
  "instance-sealed": false,
  "instance-retired": false,
  "queue-full": true,
  "undeclared-action": false,
  "viewer-read-only": false,
  "owner-mismatch": true,
  "view-state-unresolved": false,
  "mutation-rejected": false,
  "dispatch-failed": true,
};

/** 🔇️ Reasons a USER-origin refusal is shown as a shell notice. A guest echo that lost a CAS race is logged,
 * never toasted — it is not the user's mistake. Sealed/retired instances already print one typed line each. */
export const INPUT_REFUSAL_NOTIFIED_V1: Readonly<Record<InputRefusalReasonV1, boolean>> = {
  "no-session": false,
  "stale-generation": true,
  "instance-sealed": false,
  "instance-retired": false,
  "queue-full": true,
  "undeclared-action": true,
  "viewer-read-only": true,
  "owner-mismatch": true,
  "view-state-unresolved": true,
  "mutation-rejected": true,
  "dispatch-failed": true,
};

export function inputAppliedV1(inputSeq: number): InputOutcomeV1 {
  return { kind: "applied", inputSeq };
}

export function inputSupersededV1(inputSeq: number, by: number): InputOutcomeV1 {
  return { kind: "superseded", inputSeq, by };
}

export function inputRefusedV1(inputSeq: number, reason: InputRefusalReasonV1, detail?: string): InputOutcomeV1 {
  return detail === undefined
    ? { kind: "refused", inputSeq, reason, retryable: INPUT_REFUSAL_RETRYABLE_V1[reason] }
    : { kind: "refused", inputSeq, reason, retryable: INPUT_REFUSAL_RETRYABLE_V1[reason], detail };
}

/** 🗣️ One plain console line per refusal — never `[DEBUG]`-gated, so a swallowed click is greppable. */
export function inputRefusalTextV1(action: string, outcome: Extract<InputOutcomeV1, { kind: "refused" }>, provenance: InputProvenanceV1): string {
  const where = provenance.windowId === null ? "" : ` window=${provenance.windowId}`;
  const cause = provenance.causedBy === null ? "" : ` causedBy=#${provenance.causedBy}`;
  const detail = outcome.detail === undefined ? "" : ` — ${outcome.detail}`;
  return `input #${outcome.inputSeq} ${action} refused: ${outcome.reason} (${provenance.origin}${where}${cause})${detail}`;
}

/** 🔔️ Should a user-facing notice fire for this refusal? Guest/tick origins never toast; the reason table
 * decides for the rest. */
export function inputRefusalNotifiesV1(outcome: Extract<InputOutcomeV1, { kind: "refused" }>, provenance: InputProvenanceV1): boolean {
  return (provenance.origin === "user" || provenance.origin === "tutorial" || provenance.origin === "replay") && INPUT_REFUSAL_NOTIFIED_V1[outcome.reason];
}
//#endregion 🚦️Outcome

//#region 📒️Ledger
export type InputLedgerEntryV1 = Readonly<{ provenance: InputProvenanceV1; action: string }>;

export type InputLedgerCensusV1 = Readonly<{
  issued: number;
  applied: number;
  superseded: number;
  refused: Readonly<Record<InputRefusalReasonV1, number>>;
  pending: number;
}>;

export type InputLedgerV1 = {
  /** 📥️ Mints the next entry. A descriptor that already carries provenance (a re-issued or guest-caused
   * action) keeps its `causedBy`/`origin`/`windowId` and receives a fresh `inputSeq`. */
  readonly issue: (action: ShellInputActionV1, defaults?: Partial<Pick<InputProvenanceV1, "windowId" | "origin" | "causedBy">>) => InputLedgerEntryV1;
  /** 🏁️ Records the terminal outcome of `inputSeq` exactly once; a second settle of the same entry is
   * ignored (the first outcome wins), so a watchdog and a late completion cannot disagree. */
  readonly settle: (outcome: InputOutcomeV1) => boolean;
  /** 🏁️ Resolves with the outcome — immediately when already settled, otherwise when `settle` lands. Past
   * `waiterSlots` concurrent waiters the promise resolves with the entry's current state
   * (`{ kind: "applied" }` is NOT assumed: an unknown entry resolves to a `dispatch-failed` refusal). */
  readonly settled: (inputSeq: number) => Promise<InputOutcomeV1>;
  readonly outcome: (inputSeq: number) => InputOutcomeV1 | null;
  readonly entry: (inputSeq: number) => InputLedgerEntryV1 | null;
  readonly census: () => InputLedgerCensusV1;
  readonly pending: () => number;
};

/** 📏️ Concurrent `settled` waiters (mirrors `OPERATION_SETTLE_WAITER_SLOTS`) and how many settled entries are
 * remembered for a late `outcome()` read before the oldest falls off. */
export const INPUT_LEDGER_WAITER_SLOTS_V1 = 64;
export const INPUT_LEDGER_HISTORY_SLOTS_V1 = 256;

function freshRefusalCounts(): Record<InputRefusalReasonV1, number> {
  return {
    "no-session": 0,
    "stale-generation": 0,
    "instance-sealed": 0,
    "instance-retired": 0,
    "queue-full": 0,
    "undeclared-action": 0,
    "viewer-read-only": 0,
    "owner-mismatch": 0,
    "view-state-unresolved": 0,
    "mutation-rejected": 0,
    "dispatch-failed": 0,
  };
}

export function createInputLedgerV1(options?: { readonly waiterSlots?: number; readonly historySlots?: number }): InputLedgerV1 {
  const waiterSlots = options?.waiterSlots ?? INPUT_LEDGER_WAITER_SLOTS_V1;
  const historySlots = options?.historySlots ?? INPUT_LEDGER_HISTORY_SLOTS_V1;
  let nextSeq = 0;
  const open = new Map<number, InputLedgerEntryV1>();
  const closed = new Map<number, { readonly entry: InputLedgerEntryV1; readonly outcome: InputOutcomeV1 }>();
  const waiters = new Map<number, ((outcome: InputOutcomeV1) => void)[]>();
  let waiterCount = 0;
  const counts = { issued: 0, applied: 0, superseded: 0, refused: freshRefusalCounts() };

  const forget = (): void => {
    while (closed.size > historySlots) {
      const oldest = closed.keys().next().value;
      if (oldest === undefined) break;
      closed.delete(oldest);
    }
  };

  return {
    issue(action, defaults) {
      nextSeq += 1;
      const given = action.provenance;
      const provenance: InputProvenanceV1 = {
        windowId: given?.windowId ?? defaults?.windowId ?? null,
        inputSeq: nextSeq,
        causedBy: given?.causedBy ?? defaults?.causedBy ?? null,
        origin: given?.origin ?? defaults?.origin ?? "user",
      };
      const entry: InputLedgerEntryV1 = { provenance, action: action.action };
      open.set(nextSeq, entry);
      counts.issued += 1;
      return entry;
    },
    settle(outcome) {
      const entry = open.get(outcome.inputSeq);
      if (entry === undefined) return false;
      open.delete(outcome.inputSeq);
      closed.set(outcome.inputSeq, { entry, outcome });
      forget();
      if (outcome.kind === "applied") counts.applied += 1;
      else if (outcome.kind === "superseded") counts.superseded += 1;
      else counts.refused[outcome.reason] += 1;
      const pending = waiters.get(outcome.inputSeq);
      if (pending !== undefined) {
        waiters.delete(outcome.inputSeq);
        waiterCount -= pending.length;
        for (const resolve of pending) resolve(outcome);
      }
      return true;
    },
    settled(inputSeq) {
      const done = closed.get(inputSeq);
      if (done !== undefined) return Promise.resolve(done.outcome);
      if (!open.has(inputSeq)) return Promise.resolve(inputRefusedV1(inputSeq, "dispatch-failed", "unknown input"));
      if (waiterCount >= waiterSlots) return Promise.resolve(inputRefusedV1(inputSeq, "dispatch-failed", "waiter slots exhausted"));
      return new Promise<InputOutcomeV1>((resolve) => {
        const list = waiters.get(inputSeq) ?? [];
        list.push(resolve);
        waiters.set(inputSeq, list);
        waiterCount += 1;
      });
    },
    outcome: (inputSeq) => closed.get(inputSeq)?.outcome ?? null,
    entry: (inputSeq) => open.get(inputSeq) ?? closed.get(inputSeq)?.entry ?? null,
    census: () => ({ issued: counts.issued, applied: counts.applied, superseded: counts.superseded, refused: { ...counts.refused }, pending: open.size }),
    pending: () => open.size,
  };
}
//#endregion 📒️Ledger

//#region 🔢️VersionedRegister
/** 🔢️ A host-owned interaction register (active utility per window, active tool, …): the value plus the
 * generation that produced it. A widget renders BOTH and stamps the generation it rendered onto the input it
 * emits, so a write that observed a stale generation is refused instead of racing. */
export type VersionedRegisterV1<T> = Readonly<{ value: T; generation: number }>;

export type VersionedWriteV1<T> =
  | Readonly<{ kind: "applied"; register: VersionedRegisterV1<T>; changed: boolean }>
  | Readonly<{ kind: "refused"; reason: "stale-generation"; register: VersionedRegisterV1<T>; expected: number }>;

export type VersionedRegisterCellV1<T> = {
  readonly read: () => VersionedRegisterV1<T>;
  /** ✍️ Compare-and-set. `expectedGeneration === null` is the UNCONDITIONAL write reserved for the shell's own
   * authoritative paths (a session switch, a tool activation clearing every window's utility); every
   * widget- or guest-originated write passes the generation it observed. */
  readonly write: (next: T, expectedGeneration: number | null) => VersionedWriteV1<T>;
};

export function createVersionedRegisterV1<T>(initial: T, equals: (a: T, b: T) => boolean = Object.is): VersionedRegisterCellV1<T> {
  let register: VersionedRegisterV1<T> = { value: initial, generation: 0 };
  return {
    read: () => register,
    write(next, expectedGeneration) {
      if (expectedGeneration !== null && expectedGeneration !== register.generation) {
        return { kind: "refused", reason: "stale-generation", register, expected: expectedGeneration };
      }
      const changed = !equals(register.value, next);
      // 🔢️ A write that changes nothing leaves the generation alone: a widget that rendered the current
      // register must not be refused because an idle resync wrote the same value back.
      if (changed) register = { value: next, generation: register.generation + 1 };
      return { kind: "applied", register, changed };
    },
  };
}

/** 🧰️ Pure P5 activation decision, unchanged from `ShellHelpers.resolveUtilityActivation`: an empty request or
 * re-requesting the active utility deactivates; otherwise the requested one becomes active. */
export function resolveUtilityActivationV1(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🔢️ The `expectedGeneration` an input carries, read off its args: a finite non-negative integer, or `null`
 * when the emitter did not observe a register (an unconditional shell-authoritative write, or a legacy caller). */
export function expectedGenerationFromArgsV1(args: unknown): number | null {
  if (typeof args !== "object" || args === null) return null;
  const raw = (args as { readonly expectedGeneration?: unknown }).expectedGeneration;
  return typeof raw === "number" && Number.isSafeInteger(raw) && raw >= 0 ? raw : null;
}
//#endregion 🔢️VersionedRegister

//#region 🔔️RefusalNotice
/** 🗣️ Notice text per reason, authored in the unit that decides the refusal (the `🔀️surface-switch` rule):
 * no shell i18n import, `locale` picks, only an unknown locale falls back to English. */
export type InputRefusalLabelV1 = { readonly en: string; readonly de: string };
export const INPUT_REFUSAL_LABELS_V1: Readonly<Record<InputRefusalReasonV1, InputRefusalLabelV1>> = {
  "no-session": { en: "No app session is open.", de: "Keine App-Sitzung geöffnet." },
  "stale-generation": { en: "That control changed under you — try again.", de: "Das Bedienelement hat sich geändert — bitte erneut versuchen." },
  "instance-sealed": { en: "That app instance was switched away.", de: "Diese App-Instanz wurde gewechselt." },
  "instance-retired": { en: "That app instance was restarted.", de: "Diese App-Instanz wurde neu gestartet." },
  "queue-full": { en: "Too many inputs are still being processed — wait a moment.", de: "Zu viele Eingaben in Bearbeitung — bitte kurz warten." },
  "undeclared-action": { en: "This app does not declare that action.", de: "Diese App kennt diese Aktion nicht." },
  "viewer-read-only": { en: "The viewer is read-only.", de: "Der Betrachter ist schreibgeschützt." },
  "owner-mismatch": { en: "The document owner changed — try again.", de: "Der Dokumentbesitzer hat gewechselt — bitte erneut versuchen." },
  "view-state-unresolved": { en: "That window is no longer open.", de: "Dieses Fenster ist nicht mehr geöffnet." },
  "mutation-rejected": { en: "The change was rejected.", de: "Die Änderung wurde abgelehnt." },
  "dispatch-failed": { en: "The input could not be delivered.", de: "Die Eingabe konnte nicht zugestellt werden." },
};
export function inputRefusalNoticeTextV1(reason: InputRefusalReasonV1, locale: string): string {
  const label = INPUT_REFUSAL_LABELS_V1[reason];
  return locale === "de" ? label.de : label.en;
}

export type RefusalNoticeThrottleV1 = {
  /** 🔔️ `true` at most once per `windowMs` per reason — a burst of identical refusals is one toast. */
  readonly admit: (reason: InputRefusalReasonV1, nowMs: number) => boolean;
};

export const INPUT_REFUSAL_NOTICE_WINDOW_MS_V1 = 2_000;

export function createRefusalNoticeThrottleV1(windowMs: number = INPUT_REFUSAL_NOTICE_WINDOW_MS_V1): RefusalNoticeThrottleV1 {
  const lastAt = new Map<InputRefusalReasonV1, number>();
  return {
    admit(reason, nowMs) {
      const previous = lastAt.get(reason);
      if (previous !== undefined && nowMs - previous < windowMs) return false;
      lastAt.set(reason, nowMs);
      return true;
    },
  };
}
//#endregion 🔔️RefusalNotice

//#region 🖱️GestureSampleLane
/** 🖱️ One phase of a pointer gesture as the lane sends it. `begin` and `end`/`cancel` are discrete (never
 * merged); consecutive `live` samples merge into ONE batch, oldest first. */
export type GesturePhaseV1 = "begin" | "live" | "end" | "cancel";

export type GestureSendV1<Sample, Extra> =
  | Readonly<{ phase: "begin"; gestureId: number; sample: Sample; extra: Extra }>
  | Readonly<{ phase: "live"; gestureId: number | null; samples: readonly Sample[] }>
  | Readonly<{ phase: "end"; gestureId: number; sample: Sample; extra: Extra }>
  | Readonly<{ phase: "cancel"; gestureId: number; sample: Sample | null }>;

export type GestureSampleLanePortsV1<Sample, Extra> = Readonly<{
  /** 📤️ One send; its promise settles when the receiver has finished with it (the `onAction` promise). */
  readonly send: (item: GestureSendV1<Sample, Extra>) => void | Promise<unknown>;
  readonly onFault?: (error: unknown) => void;
  /** 📏️ Samples one live batch may carry before the lane starts a new batch item behind it (bounds one
   * command's wire size; a long stall still loses nothing, it costs one more send). */
  readonly maxBatch?: number;
}>;

export type GestureSampleLaneV1<Sample, Extra> = {
  /** 🖱️ pointerdown: opens gesture `gestureId` (the ledger `inputSeq` of the press). */
  readonly begin: (gestureId: number, sample: Sample, extra: Extra) => void;
  /** 🖱️ pointermove: appended to the owed live batch (inside or outside a gesture — hover moves batch too). */
  readonly offer: (sample: Sample) => void;
  /** 🖱️ pointerup: discrete, always sent, always after every owed sample. */
  readonly end: (sample: Sample, extra: Extra) => void;
  /** 🖱️ pointerleave / capture loss: a cancel, never a forged release. No-op outside a gesture. */
  readonly cancel: (sample: Sample | null) => void;
  readonly inFlight: () => boolean;
  readonly owed: () => number;
  readonly sent: () => number;
  readonly activeGesture: () => number | null;
};

export const GESTURE_LANE_MAX_BATCH_V1 = 256;

/** 🖱️ The accumulating twin of `createContinuousGestureLane`: at most ONE send in flight, and everything
 * offered while it is out is OWED — live samples merge into one ordered batch, `begin`/`end`/`cancel` stay
 * discrete items behind it in issue order. Properties, stated so they can fail:
 *   1. N moves during one round trip cost ONE send carrying all N samples in order (up to `maxBatch`);
 *   2. `begin` never overtakes a previous gesture's `end`, and `end` never overtakes owed samples;
 *   3. a refused/rejected send frees the lane and the owed items are still sent;
 *   4. samples are never dropped — only merged. */
export function createGestureSampleLaneV1<Sample, Extra>(ports: GestureSampleLanePortsV1<Sample, Extra>): GestureSampleLaneV1<Sample, Extra> {
  const maxBatch = Math.max(1, ports.maxBatch ?? GESTURE_LANE_MAX_BATCH_V1);
  type Owed = { phase: "begin"; gestureId: number; sample: Sample; extra: Extra } | { phase: "live"; gestureId: number | null; samples: Sample[] } | { phase: "end"; gestureId: number; sample: Sample; extra: Extra } | { phase: "cancel"; gestureId: number; sample: Sample | null };
  const owed: Owed[] = [];
  let inFlight = false;
  let sent = 0;
  let active: number | null = null;

  const pump = (): void => {
    if (inFlight) return;
    const next = owed.shift();
    if (next === undefined) return;
    inFlight = true;
    sent += 1;
    const item: GestureSendV1<Sample, Extra> = next.phase === "live" ? { phase: "live", gestureId: next.gestureId, samples: next.samples.slice() } : next;
    let settled: void | Promise<unknown>;
    try {
      settled = ports.send(item);
    } catch (error) {
      inFlight = false;
      ports.onFault?.(error);
      pump();
      return;
    }
    if (settled === undefined || typeof (settled as Promise<unknown>).then !== "function") {
      inFlight = false;
      pump();
      return;
    }
    void (settled as Promise<unknown>)
      .catch((error: unknown) => ports.onFault?.(error))
      .finally(() => {
        inFlight = false;
        pump();
      });
  };

  return {
    begin(gestureId, sample, extra) {
      active = gestureId;
      owed.push({ phase: "begin", gestureId, sample, extra });
      pump();
    },
    offer(sample) {
      const tail = owed[owed.length - 1];
      if (tail !== undefined && tail.phase === "live" && tail.gestureId === active && tail.samples.length < maxBatch) tail.samples.push(sample);
      else owed.push({ phase: "live", gestureId: active, samples: [sample] });
      pump();
    },
    end(sample, extra) {
      const gestureId = active;
      if (gestureId === null) return;
      active = null;
      owed.push({ phase: "end", gestureId, sample, extra });
      pump();
    },
    cancel(sample) {
      const gestureId = active;
      if (gestureId === null) return;
      active = null;
      owed.push({ phase: "cancel", gestureId, sample });
      pump();
    },
    inFlight: () => inFlight,
    owed: () => owed.reduce((count, item) => count + (item.phase === "live" ? item.samples.length : 1), 0),
    sent: () => sent,
    activeGesture: () => active,
  };
}
//#endregion 🖱️GestureSampleLane
