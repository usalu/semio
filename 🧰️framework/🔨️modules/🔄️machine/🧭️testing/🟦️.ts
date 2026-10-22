/** 🧭️ Statechart testing: the explorer, invariants, conformance runner and the in-source suite of the machine package. TypeScript twin of `🧭️testing/🦀️.rs`. */

import { type Command, type Configuration, EventId, GuardId, type Machine, type MachineSpec, NodeId, type NodeDef, ROOT, Snapshot } from "../🟦️.ts";
import { NullInspector } from "../🔎️inspect/🟦️.ts";
import { init, macrostep } from "../🧠️kernel/🟦️.ts";

//#region 🧭️Testing
/** 🧭 A set of representative events tried from every reachable configuration. */
export class Model<M extends MachineSpec> {
  readonly events: readonly M["Event"][];
  constructor(events: readonly M["Event"][]) {
    this.events = events;
  }
}

/** 🧭 What a BFS {@link explore} found: distinct configurations visited and every stable state id
 * reached across them. */
export interface Coverage {
  readonly visitedConfigurations: number;
  readonly reachedStableIds: readonly string[];
}

function activeStableIds<M extends MachineSpec>(nodes: readonly NodeDef[], snapshot: Snapshot<M>): string[] {
  return [...snapshot.configuration.iterOnes()].map((id) => nodes[id]!.stableId);
}

/** 🧭 Breadth-first walk over reachable configurations, trying every event in `model` from each
 * newly-discovered configuration. Approximates reachability by configuration only — guard outcomes
 * that depend on context may under-approximate. */
export function explore<M extends MachineSpec>(machine: Machine<M>, model: Model<M>, input: M["Input"]): Coverage {
  const nodes = machine.definition.nodes;
  const root = init(machine, input, []);
  const visited: Configuration[] = [];
  const frontier: Snapshot<M>[] = [root];
  const reachedIds: string[] = [];

  let snapshot = frontier.pop();
  while (snapshot) {
    if (visited.some((v) => v.equals(snapshot!.configuration))) {
      snapshot = frontier.pop();
      continue;
    }
    for (const stable of activeStableIds(nodes, snapshot)) if (!reachedIds.includes(stable)) reachedIds.push(stable);
    visited.push(snapshot.configuration.clone());

    for (const event of model.events) {
      const next = snapshot.branchForExploration();
      macrostep(machine, next, event, [], new NullInspector());
      frontier.push(next);
    }
    snapshot = frontier.pop();
  }

  return { visitedConfigurations: visited.length, reachedStableIds: reachedIds };
}

/** ⚠️ Why an {@link Invariant} check or {@link runConformance} fixture failed. */
export type FsmError = { readonly kind: "violation"; readonly message: string };

/** 🧭 A named property that must hold of every {@link Snapshot} visited during exploration. */
export interface Invariant<M extends MachineSpec> {
  readonly name: string;
  readonly check: (snapshot: Snapshot<M>) => { readonly ok: true } | { readonly ok: false; readonly error: FsmError };
}

/** 🧭 Runs every invariant against `snapshot`, returning one formatted message per violation. */
export function checkInvariants<M extends MachineSpec>(snapshot: Snapshot<M>, invariants: readonly Invariant<M>[]): readonly string[] {
  const violations: string[] = [];
  for (const invariant of invariants) {
    const result = invariant.check(snapshot);
    if (!result.ok) violations.push(`${invariant.name}: ${result.error.message}`);
  }
  return violations;
}

/** 🧭 One step of an inline conformance fixture: send `event`, then assert every stable id in
 * `expectActive` is part of the settled configuration. */
export interface ConformanceStep<M extends MachineSpec> {
  readonly event: M["Event"];
  readonly expectActive: readonly string[];
}

/** 🧭 Runs `steps` against a freshly-initialized machine, failing fast with a descriptive message
 * naming the offending step and the actual active configuration. */
export function runConformance<M extends MachineSpec>(machine: Machine<M>, input: M["Input"], steps: readonly ConformanceStep<M>[]): { readonly ok: true } | { readonly ok: false; readonly error: FsmError } {
  const nodes = machine.definition.nodes;
  const sink: Command<M>[] = [];
  const snapshot = init(machine, input, sink);
  for (let index = 0; index < steps.length; index += 1) {
    const step = steps[index]!;
    macrostep(machine, snapshot, step.event, sink, new NullInspector());
    for (const expected of step.expectActive) {
      if (!snapshot.matches(expected)) {
        return { ok: false, error: { kind: "violation", message: `conformance step ${index}: expected active state '${expected}', got ${JSON.stringify(activeStableIds(nodes, snapshot))}` } };
      }
    }
  }
  return { ok: true };
}

//#endregion 🧭️Testing

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../🧪️tests/🧪️semio-tech-machine/🟦️.ts");
  await registerTests1(import.meta.vitest, { EventId, GuardId, NodeId, ROOT, init, runConformance }, { directory: import.meta.dirname, url: import.meta.url });
}
//#endregion 🧪️Tests
