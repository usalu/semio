/** 📊️ Tool run progress: verdicts, the step vocabulary, the bounded step ring and the progress counters. TypeScript twin of `🧮️step-progress/🦀️.rs`. */

import { TOOL_RUN_STEP_RING_CAPACITY, ToolRunCodecError, type ToolRunIdentity } from "../🟦️.ts";
import type { ToolRunState } from "../🚦️lifecycle/🟦️.ts";

//#region 🔖️Progress
export const TOOL_RUN_VERDICTS = ["testing", "success", "warning", "danger"] as const;
export type ToolRunVerdict = (typeof TOOL_RUN_VERDICTS)[number];

/** 🗑️ `warning` and `danger` records are the only ones residency may evict. */
export function isToolRunVerdictRejected(verdict: ToolRunVerdict): boolean {
  return verdict === "warning" || verdict === "danger";
}

export const TOOL_RUN_STEP_KINDS = ["info", "success", "warning", "danger"] as const;
export type ToolRunStepKind = (typeof TOOL_RUN_STEP_KINDS)[number];

export type ToolRunStepArg = { readonly unsigned: bigint } | { readonly float: number };

export type ToolRunStep = {
  readonly sequence: bigint;
  readonly kind: ToolRunStepKind;
  readonly stage: number;
  readonly reason: number;
  readonly subject?: bigint;
  readonly repeat: number;
  readonly args: readonly ToolRunStepArg[];
};

function toolRunStepArgEquals(a: ToolRunStepArg, b: ToolRunStepArg): boolean {
  if ("unsigned" in a) return "unsigned" in b && a.unsigned === b.unsigned;
  return "float" in b && Object.is(a.float, b.float);
}

/** 🧲️ Identical apart from `sequence` and `repeat`, so consecutive pushes coalesce. */
export function toolRunStepsCoalesce(a: ToolRunStep, b: ToolRunStep): boolean {
  return a.kind === b.kind && a.stage === b.stage && a.reason === b.reason && a.subject === b.subject && a.args.length === b.args.length && a.args.every((arg, index) => toolRunStepArgEquals(arg, b.args[index]!));
}

/** 💍️ Newest ≤ 64 steps, overwrite-oldest, identical consecutive steps coalesced. */
export class ToolRunStepRing {
  #steps: ToolRunStep[] = [];

  static fromSteps(steps: readonly ToolRunStep[]): ToolRunStepRing {
    if (steps.length > TOOL_RUN_STEP_RING_CAPACITY) throw new ToolRunCodecError("limit", "step ring capacity");
    const ring = new ToolRunStepRing();
    ring.#steps = [...steps];
    return ring;
  }

  push(step: ToolRunStep): void {
    const newest = this.#steps[this.#steps.length - 1];
    if (newest !== undefined && toolRunStepsCoalesce(newest, step)) {
      this.#steps[this.#steps.length - 1] = { ...newest, sequence: step.sequence, repeat: Math.min(newest.repeat + step.repeat, 0xffff_ffff) };
      return;
    }
    if (this.#steps.length === TOOL_RUN_STEP_RING_CAPACITY) this.#steps.shift();
    this.#steps.push(step);
  }

  get length(): number {
    return this.#steps.length;
  }

  steps(): readonly ToolRunStep[] {
    return this.#steps;
  }

  oldest(): ToolRunStep | undefined {
    return this.#steps[0];
  }

  newest(): ToolRunStep | undefined {
    return this.#steps[this.#steps.length - 1];
  }
}

export type ToolRunCounter = { readonly counter: number; readonly value: bigint };

export type ToolRunProgress = {
  readonly identity: ToolRunIdentity;
  readonly sequence: bigint;
  readonly state: ToolRunState;
  readonly stage: number;
  readonly completed: bigint;
  readonly total?: bigint;
  readonly counters: readonly ToolRunCounter[];
  readonly unitsPerSecond: number;
  readonly conflicts: number;
  readonly steps: readonly ToolRunStep[];
};

/** ➗️ `completed / total`; `undefined` while indeterminate. */
export function toolRunProgressFraction(progress: ToolRunProgress): number | undefined {
  return progress.total === undefined || progress.total === 0n ? undefined : Number(progress.completed) / Number(progress.total);
}
//#endregion 🔖️Progress
