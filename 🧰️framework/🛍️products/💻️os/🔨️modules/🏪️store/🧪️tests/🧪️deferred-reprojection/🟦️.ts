/** 🐢️ Deferred-reprojection corpus (design §16.6, N17): Ajv validates its shape. An independent TS twin of the store's deferred
 * history steps reproduces every step the Rust store is checked against (`🦀️.rs` beside this file). The twin covers undo, redo,
 * finalize, alternative and trunk switch, remote supersession, discard, cancel and an interrupting edit, all folded with
 * fast-json-patch, and replays them in turns with the store's stepping rule: ask after every operation, and closing an edit
 * folds nothing. fast-check sweeps histories and budgets. The stepped fold always ends where the one-shot fold ends. A replay
 * of R operations waits iff R >= budget and needs floor(R / budget) turns after its dispatch, or floor(R' / budget) + 1 once
 * restarted. The adoption never depends on the budget. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

import fc from "fast-check";
import { applyPatch, type Operation as Patch } from "fast-json-patch";

//#region 🧮️Twin
type Snapshot = { n: number | null };
type DemoOperation = { operation: "setN"; n: number } | { operation: "addN"; delta: number } | { operation: "deleteN" } | { operation: "restoreN"; n?: number | null };
type Level = "info" | "warning" | "error" | "fatal";
type Replacement = DemoOperation | "withdrawn";
type Target = { edit: number; op: number };
type Command =
  | { kind: "undo" | "redo" | "trunk" }
  | { kind: "supersede" | "remoteSupersede"; target: Target; replacement: Replacement }
  | { kind: "alternative"; name: string; target: Target; replacement: Replacement };
type Interrupt = { after: number; discard: true } | { after: number; cancel: true } | { after: number; edit: DemoOperation[] };
type Expected = { replayed: number; waits: boolean; turns: number; refused: "rejected" | null; interruptedState?: Snapshot; state: Snapshot; applied: number; supersessions: number };
type Step = { command: Command; interrupt?: Interrupt; expected: Expected };
type Case = { name: string; initial: Snapshot; history: { author: "local" | "other"; edit: DemoOperation[]; times: number }[]; budget: number | null; steps: Step[] };
type Supersessions = Map<string, Replacement>;
type History = { initial: Snapshot; edits: { author: "local" | "other"; operations: DemoOperation[] }[]; reverted: Set<number>; redo: number[]; trunk: Supersessions; alternatives: Map<string, Supersessions>; head: string | null };

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const rank: Record<Level, number> = { info: 0, warning: 1, error: 2, fatal: 3 };
const saturate = (value: number) => Math.min(2147483647, Math.max(-2147483648, value));
const key = (target: Target) => `${target.edit}/${target.op}`;

/** 🧮️ One demo operation as the JSON Patch it writes and the worst level it raises against `state` (twin of `🧪️supersede-replay/🟦️.ts`). */
function diff(operation: DemoOperation, state: Snapshot): { patch: Patch[]; worst: Level | null } {
  switch (operation.operation) {
    case "setN":
      return state.n === null ? { patch: [], worst: "error" } : { patch: [{ op: "replace", path: "/n", value: operation.n }], worst: null };
    case "addN":
      return state.n === null ? { patch: [], worst: "error" } : { patch: [{ op: "replace", path: "/n", value: saturate(state.n + operation.delta) }], worst: "info" };
    case "deleteN":
      return { patch: state.n === null ? [] : [{ op: "replace", path: "/n", value: null }], worst: null };
    case "restoreN":
      return { patch: [{ op: "replace", path: "/n", value: operation.n ?? null }], worst: null };
  }
}

/** ⏯️ The Report replay of `order[from..]` from `base` in turns of at most `budget` operations (`null`: one turn), asking after
 * every operation as the store's replay does. Closing an edit folds nothing. Answers the operations each turn folded (the first
 * is the dispatch's turn), the reached state, and whether an error or fatal outcome blocks finalizing. */
function steppedReplay(history: History, order: number[], from: number, base: Snapshot, supersessions: Supersessions, budget: number | null): { turns: number[]; state: Snapshot; blocks: boolean } {
  const cap = budget ?? Number.POSITIVE_INFINITY;
  const turns: number[] = [];
  let state = structuredClone(base);
  let blocks = false;
  let position = from;
  let operation = 0;
  let finished = false;
  while (!finished) {
    let replayed = 0;
    for (;;) {
      if (position >= order.length) {
        finished = true;
        break;
      }
      const edit = history.edits[order[position]];
      if (operation < edit.operations.length) {
        const effective = supersessions.get(key({ edit: order[position], op: operation })) ?? edit.operations[operation];
        if (effective !== "withdrawn") {
          const { patch, worst } = diff(effective, state);
          state = applyPatch(state, patch, true, false).newDocument;
          blocks ||= worst !== null && rank[worst] >= rank.error;
        }
        operation += 1;
        replayed += 1;
        if (replayed >= cap) break;
        continue;
      }
      position += 1;
      operation = 0;
    }
    turns.push(replayed);
  }
  return { turns, state, blocks };
}

const appliedOrder = (history: History) => history.edits.map((_, index) => index).filter((index) => !history.reverted.has(index));
const effectiveSupersessions = (history: History) => (history.head === null ? history.trunk : history.alternatives.get(history.head)!);
const fold = (history: History, order: number[], supersessions: Supersessions) => steppedReplay(history, order, 0, history.initial, supersessions, null).state;
const sameSupersessions = (left: Supersessions, right: Supersessions) => left.size === right.size && [...left].every(([target, replacement]) => JSON.stringify(right.get(target)) === JSON.stringify(replacement));

/** ✏️ `history` after `command` is adopted. */
function adopt(history: History, command: Command): History {
  const next: History = { ...history, reverted: new Set(history.reverted), redo: [...history.redo], trunk: new Map(history.trunk), alternatives: new Map(history.alternatives) };
  if (history.head !== null) next.alternatives.set(history.head, new Map(effectiveSupersessions(history)));
  switch (command.kind) {
    case "undo": {
      const target = appliedOrder(history).findLast((index) => history.edits[index].author === "local");
      if (target === undefined) throw new Error("nothing of the local actor to undo");
      next.reverted.add(target);
      next.redo.push(target);
      return next;
    }
    case "redo": {
      const target = next.redo.pop();
      if (target === undefined) throw new Error("nothing to redo");
      next.reverted.delete(target);
      return next;
    }
    case "trunk":
      return { ...next, head: null };
    case "supersede":
    case "remoteSupersede":
      effectiveSupersessions(next).set(key(command.target), command.replacement);
      return next;
    case "alternative":
      next.alternatives.set(command.name, new Map([...effectiveSupersessions(history), [key(command.target), command.replacement]]));
      return { ...next, head: command.name };
  }
}

/** 🪟️ Where the replay from `live` to `next` starts: the first position whose order or effective input differs, or `null` when
 * the step needs no replay (the same supersessions over a prefix of the live order). */
function replayStart(live: History, next: History): number | null {
  const [liveOrder, nextOrder] = [appliedOrder(live), appliedOrder(next)];
  const [liveSupersessions, nextSupersessions] = [effectiveSupersessions(live), effectiveSupersessions(next)];
  if (sameSupersessions(liveSupersessions, nextSupersessions) && nextOrder.every((edit, position) => liveOrder[position] === edit)) return null;
  let common = 0;
  while (common < nextOrder.length && nextOrder[common] === liveOrder[common]) common += 1;
  const reordered = common !== nextOrder.length || common !== liveOrder.length;
  const changed = nextOrder.findIndex((edit) => next.edits[edit].operations.some((_, op) => JSON.stringify(liveSupersessions.get(key({ edit, op }))) !== JSON.stringify(nextSupersessions.get(key({ edit, op })))));
  return Math.min(changed === -1 ? nextOrder.length : changed, reordered ? common : nextOrder.length);
}

/** 🧾️ The twin of one corpus step: dispatches `step.command` over `history` with `budget`, runs its interrupt, steps it to
 * its end, and answers what the store must show then. */
function runStep(history: History, step: Step, budget: number | null): { history: History; observed: Expected } {
  const finalizing = step.command.kind === "supersede" || step.command.kind === "alternative";
  const live = fold(history, appliedOrder(history), effectiveSupersessions(history));
  let target = adopt(history, step.command);
  let from = replayStart(history, target);
  if (from === null) {
    const state = fold(target, appliedOrder(target), effectiveSupersessions(target));
    return { history: target, observed: { replayed: 0, waits: false, turns: 0, refused: null, state, applied: appliedOrder(target).length, supersessions: effectiveSupersessions(target).size } };
  }
  const liveLength = appliedOrder(history).length;
  if (from !== 0 && from !== liveLength) throw new Error(`replay starts at ${from}, neither the genesis nor the live head ${liveLength}`);
  const replay = (source: History, start: number) => {
    const order = appliedOrder(source);
    const base = start === 0 ? source.initial : fold(source, order.slice(0, start), effectiveSupersessions(source));
    return steppedReplay(source, order, start, base, effectiveSupersessions(source), budget);
  };
  const first = replay(target, from);
  const replayed = first.turns.reduce((sum, folded) => sum + folded, 0);
  const waits = first.turns.length > 1;
  let turns = first.turns.length - 1;
  let blocks = first.blocks;
  let interruptedState: Snapshot | undefined;
  let current = history;
  if (step.interrupt !== undefined) {
    if (!waits || step.interrupt.after >= turns) throw new Error("an interrupt needs a step that still waits");
    if ("discard" in step.interrupt) {
      return { history, observed: { replayed, waits, turns: 0, refused: null, state: live, applied: liveLength, supersessions: effectiveSupersessions(history).size } };
    }
    if ("edit" in step.interrupt) {
      const edit = { author: "local" as const, operations: step.interrupt.edit };
      current = { ...history, edits: [...history.edits, edit] };
      target = { ...target, edits: [...target.edits, edit] };
      interruptedState = fold(current, appliedOrder(current), effectiveSupersessions(current));
      from = replayStart(current, target)!;
    }
    const restarted = replay(target, from);
    turns = restarted.turns.length;
    blocks = restarted.blocks;
  }
  const refused = finalizing && blocks;
  const adopted = refused ? current : target;
  const state = fold(adopted, appliedOrder(adopted), effectiveSupersessions(adopted));
  const observed: Expected = { replayed, waits, turns, refused: refused ? "rejected" : null, state, applied: appliedOrder(adopted).length, supersessions: effectiveSupersessions(adopted).size };
  if (interruptedState !== undefined) observed.interruptedState = interruptedState;
  return { history: adopted, observed };
}

/** 🌱️ The history a case authors: every run unrolled into its edits. */
function authored(testCase: Case): History {
  const edits = testCase.history.flatMap((run) => Array.from({ length: run.times }, () => ({ author: run.author, operations: run.edit })));
  return { initial: testCase.initial, edits, reverted: new Set(), redo: [], trunk: new Map(), alternatives: new Map(), head: null };
}

/** 🎬️ Every step of `testCase` with `budget`, each with what the twin observed. */
function runCase(testCase: Case, budget: number | null, steps: Step[] = testCase.steps): Expected[] {
  let history = authored(testCase);
  return steps.map((step) => {
    const ran = runStep(history, step, budget);
    history = ran.history;
    return ran.observed;
  });
}
//#endregion 🧮️Twin

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️deferred-reprojection/🔣️.json");
const cases = corpus.cases as Case[];



test("🧮️ the twin reproduces every step of every case", () => {
  for (const testCase of cases) {
    expect(runCase(testCase, testCase.budget), testCase.name).toEqual(testCase.steps.map((step) => step.expected));
  }
});

test("📏️ the corpus covers waiting, short, exact-budget, multi-operation, discarded, cancelled, restarted, refused, remote and undeferred steps", () => {
  const steps = cases.flatMap((testCase) => testCase.steps.map((step) => ({ budget: testCase.budget, step })));
  const covered = (predicate: (entry: (typeof steps)[number]) => boolean) => steps.some(predicate);
  expect(covered(({ step }) => step.expected.waits && step.interrupt === undefined)).toBe(true);
  expect(covered(({ budget, step }) => budget !== null && !step.expected.waits && step.expected.replayed > 0)).toBe(true);
  expect(covered(({ budget, step }) => step.expected.replayed === budget)).toBe(true);
  expect(covered(({ budget, step }) => budget !== null && step.expected.replayed === budget - 1)).toBe(true);
  expect(cases.some((testCase) => testCase.history.some((run) => run.edit.length > 1))).toBe(true);
  for (const kind of ["discard", "cancel", "edit"]) expect(covered(({ step }) => step.interrupt !== undefined && kind in step.interrupt)).toBe(true);
  expect(covered(({ step }) => step.expected.refused === "rejected")).toBe(true);
  for (const kind of ["undo", "redo", "supersede", "alternative", "trunk", "remoteSupersede"]) expect(covered(({ step }) => step.command.kind === kind)).toBe(true);
  expect(covered(({ budget }) => budget === null)).toBe(true);
});
//#endregion 🧪️Corpus

//#region 🎲️Properties
const operation: fc.Arbitrary<DemoOperation> = fc.oneof(
  fc.record({ operation: fc.constant("setN" as const), n: fc.integer({ min: -50, max: 50 }) }),
  fc.record({ operation: fc.constant("addN" as const), delta: fc.integer({ min: -5, max: 5 }) }),
  fc.constant({ operation: "deleteN" as const }),
  fc.record({ operation: fc.constant("restoreN" as const), n: fc.integer({ min: -50, max: 50 }) }),
);
const budget = fc.option(fc.integer({ min: 1, max: 48 }), { nil: null });

test("🎲️ a stepped replay ends where the one-shot fold ends, and its turns follow floor(R / budget)", () => {
  fc.assert(
    fc.property(fc.array(fc.array(operation, { minLength: 1, maxLength: 3 }), { minLength: 1, maxLength: 40 }), budget, (edits, cap) => {
      const history: History = { initial: { n: 0 }, edits: edits.map((operations) => ({ author: "other", operations })), reverted: new Set(), redo: [], trunk: new Map(), alternatives: new Map(), head: null };
      const order = appliedOrder(history);
      const stepped = steppedReplay(history, order, 0, history.initial, history.trunk, cap);
      const total = edits.reduce((sum, edit) => sum + edit.length, 0);
      expect(stepped.state).toEqual(fold(history, order, history.trunk));
      expect(stepped.turns.reduce((sum, folded) => sum + folded, 0)).toBe(total);
      expect(stepped.turns.every((folded) => cap === null || folded <= cap)).toBe(true);
      expect(stepped.turns.length - 1).toBe(cap !== null && total >= cap ? Math.floor(total / cap) : 0);
    }),
    { numRuns: 300 },
  );
});

test("🎲️ every corpus case adopts the same history under any budget", () => {
  fc.assert(
    fc.property(fc.constantFrom(...cases), budget, (testCase, cap) => {
      const plain = testCase.steps.map(({ command, expected }) => ({ command, expected }));
      const adoption = (observed: Expected[]) => observed.map(({ refused, state, applied, supersessions }) => ({ refused, state, applied, supersessions }));
      const ran = runCase(testCase, cap, plain);
      expect(adoption(ran)).toEqual(adoption(runCase(testCase, null, plain)));
      for (const observed of ran) expect(observed.turns).toBe(observed.waits ? Math.floor(observed.replayed / cap!) : 0);
    }),
    { numRuns: 200 },
  );
});
//#endregion 🎲️Properties
