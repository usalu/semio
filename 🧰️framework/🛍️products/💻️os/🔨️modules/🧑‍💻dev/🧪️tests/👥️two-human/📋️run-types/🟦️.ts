/** 📋️ The run shapes of the two-human harness: the journey kinds, the run options, one kind's timings and row, and the report. */
import type { Human } from "../🔑️sessions/🟦️.ts";

//#region 🔖️TwoHuman
/** 🧭️ The per-kind journey of a run: both humans author (`edit`), B is a Spectator (`viewer`), or both author and undo/redo
 * across each other's edits (`cross-undo`). */
export type TwoHumanJourney = "edit" | "viewer" | "cross-undo";

/** 🎛️ One two-human run. */
export type TwoHumanOptions = Readonly<{
  journey: TwoHumanJourney;
  hub: string;
  serves: readonly [string, string];
  humans: readonly [Human, Human];
  locale: "en" | "de";
  kinds: readonly string[];
  spaceId: string | null;
  tag: string;
  outDir: string;
  adminCapabilityFile: string | null;
  mountBudgetMs: number;
  signal: AbortSignal;
}>;

/** ⏱️ One kind's timings: A's create → its first mount (the program installs from the hub catalog for A's profile), B's
 * Space-index open → its first mount, each edit's own apply and its arrival at the other human (submit → the first poll,
 * {@link EDIT_POLL_MS}, at which the peer shows the author's edited text), and the warm reopen after a reload (row click →
 * mounted; the program comes from the device's store). */
export type KindTimings = { createToMountedMs?: number; openRowToMountedMs?: number; createStartedAtMs?: number; openStartedAtMs?: number; aEditLocalMs?: number; aEditSeenByBMs?: number; bEditLocalMs?: number; bEditSeenByAMs?: number; reopenAToMountedMs?: number; reopenBToMountedMs?: number };

export type KindRow = { kindId: string; label: string; plugin?: string; artifactId?: string; timings?: KindTimings; checks: Record<string, { pass: boolean; detail: unknown }>; faults: { A: string[]; B: string[] }; pass: boolean };

/** 📊️ The run's report, rewritten after every kind. */
export type TwoHumanReport = { tag: string; journey: TwoHumanJourney; hub: string; serves: readonly string[]; locale: string; startedAt: string; finishedAt?: string; spaceId: string | null; kinds: string[]; rows: KindRow[]; misses: unknown[]; fatal?: string; cancelled?: boolean };
//#endregion 🔖️TwoHuman
