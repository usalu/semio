/** ⏱️ Command stall watch: a command turn that holds an actor's serialized ingress lane longer than the contract's
 * `stallBoundMs` (`🔣️.json`) is reported — program, command, start — until it settles, and the person may cancel it:
 * the cancelled turn rejects with `plugin.command-cancelled` and releases the lane, so every later input runs. A held
 * lane is therefore always visible and always escapable, never a silent hold (ticket 26/09/23 S15: a program whose turn
 * waited 20 s behind a busy main thread swallowed every later undo without a trace). The watch starts a turn's clock
 * when the lane STARTS the turn, so a command waiting its turn is never reported. */

import contract from "./🔣️.json" with { type: "json" };

//#region 🧬️Contract
export const COMMAND_STALL_CONTRACT_V1 = contract;

/** 🏷️ What one watched turn is: the actor lane it holds, the program and the command the person asked for. */
export type CommandStallLabelV1 = Readonly<{ actorId: string; programId: string; commandId: string | null }>;

/** 🚧️ One turn that has held its lane past the bound. */
export type CommandStallV1 = CommandStallLabelV1 & Readonly<{ id: number; startedAtMs: number }>;

/** 🛑️ The fault a cancelled turn rejects with. */
export class CommandCancelledErrorV1 extends Error {
  readonly code = contract.cancelledFault;
  constructor(readonly stall: CommandStallV1) {
    super(`${contract.cancelledFault}: ${stall.programId} ${stall.commandId ?? "interaction"} (${stall.actorId})`);
  }
}

/** 🕰️ The clock a watch runs on; the real host passes `performance.now` and `setTimeout`. */
export type CommandStallClockV1 = Readonly<{
  now: () => number;
  setTimer: (run: () => void, delayMs: number) => unknown;
  clearTimer: (handle: unknown) => void;
}>;

export type CommandStallWatchV1 = Readonly<{
  /** Runs `run` as one watched turn; rejects with {@link CommandCancelledErrorV1} when the person cancels it. */
  watch<T>(label: CommandStallLabelV1, run: () => Promise<T>): Promise<T>;
  /** Cancels a reported stall; `false` when it already settled. */
  cancel(id: number): boolean;
  stalls(): readonly CommandStallV1[];
  subscribe(listener: () => void): () => void;
}>;
//#endregion 🧬️Contract

//#region ⏱️Watch
export function createCommandStallWatchV1(clock: CommandStallClockV1, boundMs: number = contract.stallBoundMs): CommandStallWatchV1 {
  let nextId = 0;
  let current: readonly CommandStallV1[] = [];
  const cancellers = new Map<number, () => void>();
  const listeners = new Set<() => void>();
  const publish = (next: readonly CommandStallV1[]): void => {
    current = next;
    for (const listener of [...listeners]) listener();
  };
  const forget = (id: number): void => {
    cancellers.delete(id);
    if (current.some((stall) => stall.id === id)) publish(current.filter((stall) => stall.id !== id));
  };
  return Object.freeze({
    watch<T>(label: CommandStallLabelV1, run: () => Promise<T>): Promise<T> {
      nextId += 1;
      const stall: CommandStallV1 = Object.freeze({ ...label, id: nextId, startedAtMs: clock.now() });
      const timer = clock.setTimer(() => {
        if (!cancellers.has(stall.id)) return;
        console.warn(`[os-shell] command stalled: ${stall.programId} ${stall.commandId ?? "interaction"} has held ${stall.actorId}'s lane for ${boundMs} ms`);
        publish([...current, stall]);
      }, boundMs);
      const cancelled = new Promise<never>((_, reject) => cancellers.set(stall.id, () => reject(new CommandCancelledErrorV1(stall))));
      const settled = run();
      void settled.catch(() => undefined);
      return Promise.race([settled, cancelled]).finally(() => {
        clock.clearTimer(timer);
        forget(stall.id);
      });
    },
    cancel(id: number): boolean {
      const cancel = cancellers.get(id);
      if (!cancel) return false;
      cancel();
      return true;
    },
    stalls: () => current,
    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  });
}
//#endregion ⏱️Watch

//#region 🔤️Text
/** ⏲️ How long a reported stall has held its lane, in seconds, read against the shell's display clock `nowMs`. A stall is
 * only reported once it has held the lane for `boundMs`, so a display clock that has not ticked since the report (the first
 * paint of the band, or a throttled page) still reads the bound — never "0 s" (ticket 26/09/23 S15, live de band). */
export function commandStallHeldSecondsV1(stall: Pick<CommandStallV1, "startedAtMs">, nowMs: number, boundMs: number = contract.stallBoundMs): number {
  return Math.max(boundMs, nowMs - stall.startedAtMs) / 1_000;
}

type FrozenText = Readonly<{ en: string; de: string }>;
const text = (label: FrozenText, locale: string): string => (locale.startsWith("de") ? label.de : label.en);

/** 🔤️ The band line for one stall, in the person's language; `commandLabel` is the program's own label of the command
 * (null for an input that is no declared command). */
export function commandStallBandTextV1(programLabel: string, commandLabel: string | null, seconds: number, locale: string): string {
  return text(contract.labels.band, locale)
    .replace("{program}", programLabel)
    .replace("{command}", commandLabel ?? text(contract.labels.interaction, locale))
    .replace("{seconds}", new Intl.NumberFormat(locale, { maximumFractionDigits: 0 }).format(seconds));
}

export function commandStallCancelTextV1(locale: string): string {
  return text(contract.labels.cancel, locale);
}
//#endregion 🔤️Text
