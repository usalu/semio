/** 👁️ Viewer-head corpus (design §22.3): Ajv validates its shape. An independent TS model of two replicas over one shared
 * event log reproduces every step the Rust store is checked against (`🦀️.rs` beside this file). The model holds the events as
 * a set and each replica's head beside it. `switch` and `checkout` author nothing. A new-alternative finalize authors the commit
 * of the author's uncommitted edits, the registration and a supersession scoped to the new alternative, and moves only its
 * author. The projection is folded with fast-json-patch. fast-check sweeps arrival orders and histories: a replica holding the
 * whole log shows every alternative's tip as the corpus says whatever order the events arrived in, and receiving another
 * replica's new alternative never changes what a replica shows. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import fc from "fast-check";
import { applyPatch, type Operation as Patch } from "fast-json-patch";

//#region 🧮️Twin
type Snapshot = { n: number | null };
type DemoOperation = { operation: "setN"; n: number } | { operation: "addN"; delta: number };
type Replacement = DemoOperation | "withdrawn";
type Name = "a" | "b";
type Target = { by: Name; edit: number; op: number };
type Action =
  | { kind: "edit"; edit: DemoOperation[] }
  | { kind: "commit" }
  | { kind: "alternative"; name: string; target: Target; replacement: Replacement }
  | { kind: "supersede"; scope: "document" | "line"; target: Target; replacement: Replacement }
  | { kind: "switch"; to: string }
  | { kind: "checkout"; checkpoint: number }
  | { kind: "receive"; from: Name; order: "authored" | "reversed" }
  | { kind: "reload"; through: "spr" | "ops" };
type Projection = { state: Snapshot; applied: number; supersessions: number };
type View = Projection & { line: string; checkpoint: number | null; alternatives: string[] };
type Step = { at: Name; do: Action; expect: Record<Name, View> };
type Case = { name: string; initial: Snapshot; steps: Step[]; converged: { alternatives: string[]; lines: Record<string, Projection> } };
type Stamp = { clock: number; id: string };
type LogEvent = Stamp &
  (
    | { kind: "edit"; line: string; operations: DemoOperation[] }
    | { kind: "commit"; line: string; parent: string | null; edits: string[] }
    | { kind: "branch"; name: string; checkpoint: string }
    | { kind: "supersede"; scope: string | null; target: string; replacement: Replacement }
  );
type Head = { line: string; checkpoint: string | null };
type Replica = { name: Name; minted: number; authored: string[]; clock: number; events: Map<string, LogEvent>; head: Head };
type Fold = { checkpoints: { id: string; edits: string[] }[]; lines: { name: string; chain: string[] }[]; shown: string | null; applied: string[]; committed: Set<string>; supersessions: Map<string, Replacement>; state: Snapshot };

const TRUNK = "trunk";
const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const saturate = (value: number) => Math.min(2147483647, Math.max(-2147483648, value));
const byStamp = (left: Stamp, right: Stamp) => left.clock - right.clock || (left.id < right.id ? -1 : left.id > right.id ? 1 : 0);

/** 🧮️ One demo operation as the JSON Patch it writes against `state` (an operation on an absent `n` writes nothing). */
function diff(operation: DemoOperation, state: Snapshot): Patch[] {
  if (state.n === null) return [];
  return [{ op: "replace", path: "/n", value: operation.operation === "setN" ? operation.n : saturate(state.n + operation.delta) }];
}

/** 🧮️ What `head` shows of the event set `events`: registrations and checkpoints do not depend on the head; the applied
 * edits, the effective supersessions and the state do. */
function fold(initial: Snapshot, events: Iterable<LogEvent>, head: Head): Fold {
  const ordered = [...events].sort(byStamp);
  const edits = new Map<string, Extract<LogEvent, { kind: "edit" }>>();
  const checkpoints: { id: string; edits: string[] }[] = [];
  const trunk: string[] = [];
  const branches: { name: string; chain: string[] }[] = [];
  const committed = new Set<string>();
  const candidates: { scope: string | null; target: string; replacement: Replacement }[] = [];
  const checkpoint = (id: string) => {
    const known = checkpoints.find((entry) => entry.id === id);
    if (known === undefined) throw new Error(`unknown checkpoint ${id}`);
    return known;
  };
  const branch = (name: string) => {
    const known = branches.find((entry) => entry.name === name);
    if (known === undefined) throw new Error(`unknown alternative ${name}`);
    return known;
  };
  for (const event of ordered) {
    if (event.kind === "edit") edits.set(event.id, event);
    else if (event.kind === "commit") {
      checkpoints.push({ id: event.id, edits: [...(event.parent === null ? [] : checkpoint(event.parent).edits), ...event.edits] });
      for (const edit of event.edits) committed.add(edit);
      (event.line === TRUNK ? trunk : branch(event.line).chain).push(event.id);
    } else if (event.kind === "branch") {
      if (event.name === TRUNK) throw new Error("a branch cannot claim the trunk");
      checkpoint(event.checkpoint);
      branches.push({ name: event.name, chain: [event.checkpoint] });
    } else candidates.push({ scope: event.scope, target: event.target, replacement: event.replacement });
  }
  const chain = head.line === TRUNK ? trunk : branch(head.line).chain;
  if (head.checkpoint !== null && !chain.includes(head.checkpoint)) throw new Error(`checkpoint ${head.checkpoint} is not on ${head.line}`);
  const shown = head.checkpoint ?? chain.at(-1) ?? null;
  const visible = new Set<string>(shown === null ? [] : checkpoint(shown).edits);
  if (head.checkpoint === null) for (const edit of edits.values()) if (!committed.has(edit.id) && edit.line === head.line) visible.add(edit.id);
  const applied = ordered.filter((event) => event.kind === "edit" && visible.has(event.id)).map((event) => event.id);
  const supersessions = new Map<string, Replacement>();
  for (const candidate of candidates) {
    if (!edits.has(candidate.target.split("/")[0])) throw new Error(`supersession of unknown operation ${candidate.target}`);
    if (candidate.scope === null || candidate.scope === head.line) supersessions.set(candidate.target, candidate.replacement);
  }
  let state = structuredClone(initial);
  for (const id of applied) {
    edits.get(id)!.operations.forEach((operation, op) => {
      const effective = supersessions.get(`${id}/${op}`) ?? operation;
      if (effective !== "withdrawn") state = applyPatch(state, diff(effective, state), true, false).newDocument;
    });
  }
  const lines = [...(trunk.length > 0 ? [{ name: TRUNK, chain: trunk }] : []), ...branches];
  return { checkpoints, lines, shown, applied, committed, supersessions, state };
}

const replica = (name: Name): Replica => ({ name, minted: 0, authored: [], clock: 0, events: new Map(), head: { line: TRUNK, checkpoint: null } });

/** ✍️ Records an event `author` mints now: after everything it holds. */
function author<E extends Omit<LogEvent, "clock" | "id">>(at: Replica, event: E): string {
  at.clock += 1;
  at.minted += 1;
  const id = `${at.name}${at.minted}`;
  at.events.set(id, { ...event, clock: at.clock, id } as unknown as LogEvent);
  return id;
}

/** 🚩️ The checkpoint of every applied-but-uncommitted edit `at` shows, on its alternative. */
function commitPending(at: Replica, initial: Snapshot): string {
  const shown = fold(initial, at.events.values(), at.head);
  const pending = shown.applied.filter((edit) => !shown.committed.has(edit));
  if (pending.length === 0) throw new Error("nothing to checkpoint");
  return author(at, { kind: "commit", line: at.head.line, parent: shown.shown, edits: pending });
}

/** 🎬️ One corpus action at `at`. */
function act(initial: Snapshot, replicas: Record<Name, Replica>, at: Replica, action: Action): void {
  const operation = (target: Target) => {
    const edit = replicas[target.by].authored[target.edit];
    if (edit === undefined || !at.events.has(edit)) throw new Error(`replica ${at.name} does not hold edit ${target.edit} of ${target.by}`);
    return `${edit}/${target.op}`;
  };
  switch (action.kind) {
    case "edit":
      if (at.head.checkpoint !== null) throw new Error("the corpus edits at a tip");
      at.authored.push(author(at, { kind: "edit", line: at.head.line, operations: action.edit }));
      return;
    case "commit":
      commitPending(at, initial);
      return;
    case "alternative": {
      const shown = fold(initial, at.events.values(), at.head);
      if (shown.lines.some((line) => line.name === action.name) || action.name === TRUNK) throw new Error(`alternative ${action.name} already exists`);
      const target = operation(action.target);
      const pending = shown.applied.some((edit) => !shown.committed.has(edit));
      const checkpoint = pending || shown.checkpoints.length === 0 ? commitPending(at, initial) : (shown.shown ?? shown.checkpoints.at(-1)!.id);
      author(at, { kind: "branch", name: action.name, checkpoint });
      author(at, { kind: "supersede", scope: action.name, target, replacement: action.replacement });
      at.head = { line: action.name, checkpoint: null };
      return;
    }
    case "supersede":
      author(at, { kind: "supersede", scope: action.scope === "document" ? null : at.head.line, target: operation(action.target), replacement: action.replacement });
      return;
    case "switch":
      fold(initial, at.events.values(), { line: action.to, checkpoint: null });
      at.head = { line: action.to, checkpoint: null };
      return;
    case "checkout": {
      const shown = fold(initial, at.events.values(), at.head);
      const checkpoint = shown.checkpoints[action.checkpoint]?.id;
      if (checkpoint === undefined) throw new Error(`no checkpoint ${action.checkpoint}`);
      const on = (name: string) => shown.lines.some((line) => line.name === name && line.chain.includes(checkpoint));
      at.head = { line: on(at.head.line) ? at.head.line : (shown.lines.find((line) => line.chain.includes(checkpoint))?.name ?? at.head.line), checkpoint };
      return;
    }
    case "receive": {
      const missing = [...replicas[action.from].events.values()].filter((event) => !at.events.has(event.id)).sort(byStamp);
      receive(at, action.order === "reversed" ? missing.reverse() : missing);
      return;
    }
    case "reload": {
      const persisted = JSON.stringify(action.through === "spr" ? { events: [...at.events.values()], head: at.head } : { events: [...at.events.values()] });
      const restored = JSON.parse(persisted) as { events: LogEvent[]; head?: Head };
      at.events = new Map();
      at.clock = 0;
      receive(at, restored.events);
      at.head = restored.head ?? { line: TRUNK, checkpoint: null };
      return;
    }
  }
}

/** 📥️ `at` takes `events` in the order given; its clock moves past every one of them. */
function receive(at: Replica, events: LogEvent[]): void {
  for (const event of events) {
    at.events.set(event.id, event);
    at.clock = Math.max(at.clock, event.clock);
  }
}

const projection = (shown: Fold): Projection => ({ state: shown.state, applied: shown.applied.length, supersessions: shown.supersessions.size });
const listed = (shown: Fold) => shown.lines.map((line) => (line.name === TRUNK ? "" : line.name));

/** 🪟️ What `at` shows. */
function view(initial: Snapshot, at: Replica): View {
  const shown = fold(initial, at.events.values(), at.head);
  return { line: at.head.line, checkpoint: at.head.checkpoint === null ? null : shown.checkpoints.findIndex((entry) => entry.id === at.head.checkpoint), ...projection(shown), alternatives: listed(shown) };
}

/** 🎞️ Every step of `testCase`: what both replicas show after each, and the replicas at the end. */
function run(testCase: Case): { observed: Record<Name, View>[]; replicas: Record<Name, Replica> } {
  const replicas: Record<Name, Replica> = { a: replica("a"), b: replica("b") };
  const observed = testCase.steps.map((step) => {
    act(testCase.initial, replicas, replicas[step.at], step.do);
    return { a: view(testCase.initial, replicas.a), b: view(testCase.initial, replicas.b) };
  });
  return { observed, replicas };
}

/** 🌐️ What a replica holding `events` shows at every alternative's tip. */
function converged(initial: Snapshot, events: LogEvent[]): Case["converged"] {
  const fresh = replica("a");
  receive(fresh, events);
  const trunk = fold(initial, fresh.events.values(), fresh.head);
  const lines: Record<string, Projection> = { [TRUNK]: projection(trunk) };
  for (const line of trunk.lines) lines[line.name] = projection(fold(initial, fresh.events.values(), { line: line.name, checkpoint: null }));
  return { alternatives: listed(trunk), lines };
}

const wholeLog = (replicas: Record<Name, Replica>) => [...new Map([...replicas.a.events, ...replicas.b.events]).values()].sort(byStamp);
//#endregion 🧮️Twin

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️viewer-head/🔣️.json");
const cases = corpus.cases as Case[];

test("🧬️ the viewer-head corpus satisfies its JSON Schema", () => {
});

test("🧮️ the twin reproduces what both replicas show after every step, and what the whole log converges to", () => {
  for (const testCase of cases) {
    const ran = run(testCase);
    ran.observed.forEach((observed, index) => expect(observed, `${testCase.name} step ${index}`).toEqual(testCase.steps[index].expect));
    expect(converged(testCase.initial, wholeLog(ran.replicas)), testCase.name).toEqual(testCase.converged);
  }
});

test("📏️ the corpus covers a finalize that leaves the other replica where it is, local switches and checkouts, both reloads and both arrival orders", () => {
  const steps = cases.flatMap((testCase) => testCase.steps.map((step, index) => ({ step, before: index === 0 ? null : testCase.steps[index - 1].expect })));
  const covered = (predicate: (entry: (typeof steps)[number]) => boolean) => steps.some(predicate);
  const other = (name: Name): Name => (name === "a" ? "b" : "a");
  const head = (shown: View) => [shown.line, shown.checkpoint];
  expect(covered(({ step }) => step.do.kind === "alternative" && step.expect[step.at].line === step.do.name && step.expect[other(step.at)].line !== step.do.name)).toBe(true);
  expect(covered(({ step, before }) => step.do.kind === "receive" && before !== null && step.expect[step.at].alternatives.length > before[step.at].alternatives.length && JSON.stringify(head(step.expect[step.at])) === JSON.stringify(head(before[step.at])) && JSON.stringify(step.expect[step.at].state) === JSON.stringify(before[step.at].state))).toBe(true);
  expect(covered(({ step }) => step.do.kind === "switch" && step.do.to !== TRUNK && step.expect[step.at].line === step.do.to && step.expect[other(step.at)].line !== step.do.to)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "switch" && step.do.to === TRUNK)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "checkout" && step.expect[step.at].checkpoint !== null && step.expect[other(step.at)].checkpoint === null)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "reload" && step.do.through === "spr" && step.expect[step.at].line !== TRUNK)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "reload" && step.do.through === "spr" && step.expect[step.at].checkpoint !== null)).toBe(true);
  expect(covered(({ step, before }) => step.do.kind === "reload" && step.do.through === "ops" && before !== null && before[step.at].line !== TRUNK && step.expect[step.at].line === TRUNK)).toBe(true);
  for (const scope of ["document", "line"]) expect(covered(({ step }) => step.do.kind === "supersede" && step.do.scope === scope)).toBe(true);
  for (const order of ["authored", "reversed"]) expect(covered(({ step }) => step.do.kind === "receive" && step.do.order === order)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "edit" && step.expect[step.at].line !== TRUNK)).toBe(true);
  expect(covered(({ step }) => step.do.kind === "commit" && step.expect[step.at].line !== TRUNK)).toBe(true);
  expect(cases.some((testCase) => Object.keys(testCase.converged.lines).length > 2)).toBe(true);
});
//#endregion 🧪️Corpus

//#region 🎲️Properties
test("🎲️ a replica holding the whole log shows every alternative as the corpus says, whatever order the events arrived in", () => {
  for (const testCase of cases) {
    const log = wholeLog(run(testCase).replicas);
    fc.assert(
      fc.property(fc.shuffledSubarray(log, { minLength: log.length, maxLength: log.length }), (arrival) => {
        expect(converged(testCase.initial, arrival)).toEqual(testCase.converged);
      }),
      { numRuns: 100 },
    );
  }
});

const operation: fc.Arbitrary<DemoOperation> = fc.oneof(fc.record({ operation: fc.constant("setN" as const), n: fc.integer({ min: -50, max: 50 }) }), fc.record({ operation: fc.constant("addN" as const), delta: fc.integer({ min: -5, max: 5 }) }));
const replacement: fc.Arbitrary<Replacement> = fc.oneof(operation, fc.constant("withdrawn" as const));

test("🎲️ receiving another replica's new alternative, in any order, changes nothing a replica shows but the list of alternatives", () => {
  fc.assert(
    fc.property(
      fc.array(fc.array(operation, { minLength: 1, maxLength: 3 }), { minLength: 1, maxLength: 12 }),
      fc.array(fc.integer({ min: -5, max: 5 }), { maxLength: 4 }),
      fc.nat(),
      fc.nat(),
      replacement,
      fc.boolean(),
      fc.nat(),
      (edits, concurrent, pickEdit, pickOp, replaced, committedFirst, seed) => {
        const initial: Snapshot = { n: 0 };
        const replicas: Record<Name, Replica> = { a: replica("a"), b: replica("b") };
        for (const edit of edits) act(initial, replicas, replicas.a, { kind: "edit", edit });
        if (committedFirst) act(initial, replicas, replicas.a, { kind: "commit" });
        act(initial, replicas, replicas.b, { kind: "receive", from: "a", order: "authored" });
        for (const delta of concurrent) act(initial, replicas, replicas.b, { kind: "edit", edit: [{ operation: "addN", delta }] });
        const before = view(initial, replicas.b);
        const edit = pickEdit % edits.length;
        act(initial, replicas, replicas.a, { kind: "alternative", name: "variant", target: { by: "a", edit, op: pickOp % edits[edit].length }, replacement: replaced });
        expect(view(initial, replicas.a).line).toBe("variant");
        const missing = [...replicas.a.events.values()].filter((event) => !replicas.b.events.has(event.id)).sort(byStamp);
        const rotation = seed % missing.length;
        receive(replicas.b, [...missing.slice(rotation), ...missing.slice(0, rotation)].reverse());
        const after = view(initial, replicas.b);
        expect({ ...after, alternatives: before.alternatives }).toEqual(before);
        expect(after.alternatives).toEqual(["", "variant"]);
        act(initial, replicas, replicas.b, { kind: "switch", to: "variant" });
        expect(view(initial, replicas.a).line).toBe("variant");
        act(initial, replicas, replicas.b, { kind: "switch", to: TRUNK });
        expect({ ...view(initial, replicas.b), alternatives: before.alternatives }).toEqual(before);
      },
    ),
    { numRuns: 300 },
  );
});
//#endregion 🎲️Properties
