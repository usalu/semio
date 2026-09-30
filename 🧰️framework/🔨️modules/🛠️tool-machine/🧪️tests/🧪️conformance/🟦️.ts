/** 🧪️ TS reducer, id minting and runner against the transaction-law fixture; oracles: ajv (schema), JS `Map` (keyed first-insertion order), xstate (each fixture chart as an xstate machine), fast-check (random yield sequences for reducer vs `Map`; random events, host aborts and resets for runner vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, initialTransition, setup, transition, type AnyMachineSnapshot } from "xstate";
import { ActionId, EventId, InvokeId, NodeId, ROOT, TestHost, TimerId, type ActionFn, type Machine, type MachineSpec, type NodeDef, type StatechartEvent, type TransitionDef } from "@semio-tech/machine";
import { mintTransactionRef, type TransactionRef } from "@semio-tech/framework-replication";
import fixture from "../../🧫️fixtures/🧫️transaction-law/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import * as T from "../../🟦️.ts";

const law = fixture as any;
type Json = any;

function toClock(json: T.ToolClock): T.ToolClock {
  return { actor: json.actor, physical_ms: json.physical_ms, logical: json.logical };
}

function entriesJson(entries: ReadonlyArray<readonly [string, unknown]>): Json[] {
  return entries.map(([key, mutation]) => ({ key, mutation }));
}

//#region 🔖️MapOracle
/** 🗺️ Independent keyed-order model: `Map.set` keeps an existing key's slot, `Map.delete` + `Map.set` re-appends. */
class MapTransaction {
  state: T.ToolTransactionState = "open";
  readonly entries = new Map<string, unknown>();

  apply(yielded: T.ToolYield<unknown>): boolean {
    if (this.state !== "open") return false;
    if (yielded.kind === "upsert") this.entries.set(yielded.key, yielded.mutation);
    else if (yielded.kind === "retract") this.entries.delete(yielded.key);
    else if (yielded.kind === "commit") this.state = "committed";
    else {
      this.state = "aborted";
      this.entries.clear();
    }
    return true;
  }
}
//#endregion 🔖️MapOracle

//#region 🔖️Gesture
type GestureInput = { readonly magnet: { readonly x: number; readonly y: number }; readonly radius: number };
type GestureMutation = { readonly kind: "translate"; readonly dx: number; readonly dy: number } | { readonly kind: "connect"; readonly x: number; readonly y: number };
type GestureContext = { readonly input: GestureInput; origin: readonly [number, number] };
const EVENT_NAMES = ["pointerDown", "pointerMove", "pointerUp", "escape"] as const;
type GestureEventJson = { readonly type: (typeof EVENT_NAMES)[number]; readonly x?: number; readonly y?: number };
type GestureEvent = StatechartEvent & GestureEventJson;
interface GestureSpec extends MachineSpec {
  Context: GestureContext;
  Event: GestureEvent;
  Input: GestureInput;
  Output: never;
  Effect: T.ToolYield<GestureMutation>;
}

function gestureEvent(json: GestureEventJson): GestureEvent {
  const index = EVENT_NAMES.indexOf(json.type);
  return { ...json, eventCount: EVENT_NAMES.length, eventId: () => EventId(index), eventName: (id) => EVENT_NAMES[id]! };
}

/** 🧲️ The drag action of the fixture chart: translate from the origin (retracted at zero offset), connect within the magnet radius. */
function dragYields(origin: readonly [number, number], x: number, y: number, input: GestureInput): T.ToolYield<GestureMutation>[] {
  const [dx, dy] = [x - origin[0], y - origin[1]];
  const near = (x - input.magnet.x) ** 2 + (y - input.magnet.y) ** 2 <= input.radius ** 2;
  return [
    dx === 0 && dy === 0 ? { kind: "retract", key: "translate" } : { kind: "upsert", key: "translate", mutation: { kind: "translate", dx, dy } },
    near ? { kind: "upsert", key: "connect", mutation: { kind: "connect", x: input.magnet.x, y: input.magnet.y } } : { kind: "retract", key: "connect" },
  ];
}

const GESTURE_ACTIONS: Record<string, ActionFn<GestureSpec>> = {
  press: (context, event) => {
    context.origin = [event!.x!, event!.y!];
  },
  drag: (context, event, sink) => {
    for (const effect of dragYields(context.origin, event!.x!, event!.y!, context.input)) sink.push({ kind: "effect", effect });
  },
  release: (_context, _event, sink) => sink.push({ kind: "effect", effect: { kind: "commit" } }),
  cancel: (_context, _event, sink) => sink.push({ kind: "effect", effect: { kind: "abort" } }),
};
const ACTION_NAMES = Object.keys(GESTURE_ACTIONS);

function atomicNodes(states: readonly string[]): NodeDef[] {
  return states.map((stableId, index) => ({ stableId, kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: index + 1 }));
}

/** 📐️ The kernel tables of the fixture chart (TS has no `statechart!`, so the chart is the table source). */
function chartMachine(chart: Json): Machine<GestureSpec> {
  const node = (state: string) => NodeId(chart.states.indexOf(state) + 1);
  const nodes: NodeDef[] = [{ stableId: "root", kind: "compound", initial: node(chart.initial), children: chart.states.map(node), entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 }, ...atomicNodes(chart.states)];
  const transitions: TransitionDef[] = chart.transitions.map((row: Json, index: number) => ({ source: node(row.from), trigger: { kind: "event", event: EventId(EVENT_NAMES.indexOf(row.event)) }, targets: [node(row.to)], kind: "external", actions: row.action === null ? [] : [ActionId(ACTION_NAMES.indexOf(row.action))], docIndex: index }));
  return { definition: { id: chart.id, nodes, transitions, contextFromInput: (input) => ({ input, origin: [0, 0] }), guards: [], actions: ACTION_NAMES.map((name) => GESTURE_ACTIONS[name]!), fingerprint: 0n, manifestJson: "{}" } };
}

const gesture = law.gesture;
type Variant = { readonly chart: Json; readonly scenarios: Json[]; readonly machine: Machine<GestureSpec> };
const variants: Variant[] = gesture.variants.map((variant: Json) => ({ ...variant, machine: chartMachine(variant.chart) }));
const variant = (id: string): Variant => variants.find((candidate) => candidate.chart.id === id)!;

function startRunner(machine: Machine<GestureSpec>): T.ToolMachineRunner<GestureSpec> {
  const started = T.ToolMachineRunner.start(machine, gesture.tool, gesture.actor, gesture.input, new TestHost<GestureSpec>());
  if (!started.ok) throw new Error(started.refusal);
  return started.runner;
}

function activeState(runner: T.ToolMachineRunner<GestureSpec>, chart: Json): string {
  return chart.states.find((state: string) => runner.snapshot.matches(state));
}

function stepJson(step: T.ToolStep<unknown>, open: T.ToolTransaction<unknown> | undefined): Json {
  if (step.kind === "open") return { kind: "open", transaction: open!.reference, entries: entriesJson(open!.entries()) };
  return step;
}

type Operation = GestureEventJson | { readonly host: "abort"; readonly reason: T.ToolAbortReason } | { readonly host: "reset" } | { readonly host: "resume" };
type Slot = { runner: T.ToolMachineRunner<GestureSpec> };

function resumed(machine: Machine<GestureSpec>, snapshot: T.ToolMachineRunner<GestureSpec>["snapshot"], transaction: T.ToolTransaction<GestureMutation> | undefined): ReturnType<typeof T.ToolMachineRunner.resume<GestureSpec>> {
  return T.ToolMachineRunner.resume(machine, gesture.tool, gesture.actor, gesture.input, snapshot, transaction, new TestHost<GestureSpec>());
}

/** 🧳️ Persists the slot's runner and resumes it with a fresh host. */
function persistAndResume(slot: Slot): void {
  const [snapshot, transaction] = slot.runner.intoParts();
  const result = resumed(slot.runner.machine, snapshot, transaction);
  if (!result.ok) throw new Error(result.refusal);
  slot.runner = result.runner;
}

/** 🎚️ What a fixture row expects: the settled state plus the step, the refusal or the dropped ref. */
function expectedOutcome(row: Json): Json {
  const { event: _event, clock: _clock, host: _host, reason: _reason, ...outcome } = row;
  return outcome;
}

function rowOperation(row: Json): Operation {
  return row.host === "abort" ? { host: "abort", reason: row.reason } : row.host === "reset" || row.host === "resume" ? { host: row.host } : row.event;
}

/** 🏃‍♀️ Applies one operation to the runner and reports it in the fixture row shape. */
function runRunner(slot: Slot, chart: Json, operation: Operation, clock: T.ToolClock): Json {
  let outcome: Json = {};
  if ("host" in operation && operation.host === "resume") persistAndResume(slot);
  const { runner } = slot;
  if ("host" in operation) {
    if (operation.host === "abort") outcome = { step: stepJson(runner.abort(operation.reason), runner.transaction()) };
    if (operation.host === "reset") outcome = { dropped: runner.reset() ?? null };
  } else {
    const result = runner.send(gestureEvent(operation), clock);
    outcome = result.ok ? { step: stepJson(result.step, runner.transaction()) } : { refusal: result.refusal };
  }
  expect(runner.transaction() === undefined || !runner.atRest()).toBe(true);
  return { state: activeState(runner, chart), ...outcome };
}

/** 🎛️ The xstate oracle: a fixture chart as an xstate machine whose actions log yields, driving an independent `Map` transaction slot, the rest law (the chart's initial state) and host cancels by re-entering the initial snapshot. */
class OracleRunner {
  readonly #chart: Json;
  readonly #machine;
  #snapshot: AnyMachineSnapshot;
  #open: { reference: TransactionRef; model: MapTransaction } | undefined;

  constructor(chart: Json) {
    this.#chart = chart;
    this.#machine = setup({
      actions: {
        press: assign(({ event }: any) => ({ origin: [event.x, event.y] })),
        drag: assign(({ context, event }: any) => ({ log: [...context.log, ...dragYields(context.origin, event.x, event.y, gesture.input)] })),
        release: assign(({ context }: any) => ({ log: [...context.log, { kind: "commit" }] })),
        cancel: assign(({ context }: any) => ({ log: [...context.log, { kind: "abort" }] })),
      } as any,
    }).createMachine({
      id: "oracle",
      initial: chart.initial,
      context: { origin: [0, 0], log: [] },
      states: Object.fromEntries(chart.states.map((state: string) => [state, { on: Object.fromEntries(chart.transitions.filter((row: Json) => row.from === state).map((row: Json) => [row.event, { target: row.to, actions: row.action === null ? [] : [row.action] }])) }])),
    } as any);
    this.#snapshot = initialTransition(this.#machine)[0];
  }

  get state(): string {
    return this.#snapshot.value as string;
  }

  apply(operation: Operation, clock: T.ToolClock): Json {
    if (!("host" in operation)) return { ...this.#send(operation, clock), state: this.state };
    if (operation.host === "resume") return { state: this.state };
    const dropped = this.#rest();
    const outcome = operation.host === "reset" ? { dropped: dropped ?? null } : { step: dropped ? { kind: "aborted", transaction: dropped, reason: operation.reason } : { kind: "idle" } };
    return { state: this.state, ...outcome };
  }

  #rest(): TransactionRef | undefined {
    this.#snapshot = initialTransition(this.#machine)[0];
    const dropped = this.#open?.reference;
    this.#open = undefined;
    return dropped;
  }

  #send(event: GestureEventJson, clock: T.ToolClock): Json {
    const before = this.#snapshot.context.log.length;
    this.#snapshot = transition(this.#machine, this.#snapshot, event as any)[0];
    let refused = false;
    for (const yielded of this.#snapshot.context.log.slice(before) as T.ToolYield<unknown>[]) {
      if (refused) continue;
      if (this.#open) refused = !this.#open.model.apply(yielded);
      else if (yielded.kind === "upsert") {
        this.#open = { reference: mintTransactionRef(gesture.actor, clock, gesture.tool), model: new MapTransaction() };
        this.#open.model.apply(yielded);
      }
    }
    const open = this.#open;
    const unclosed = open?.model.state === "open" && this.state === this.#chart.initial;
    if (refused || unclosed) {
      this.#open = undefined;
      return { refusal: refused ? "toolTransaction.closed" : "toolTransaction.unclosed" };
    }
    if (!open) return { step: { kind: "idle" } };
    if (open.model.state === "open") return { step: { kind: "open", transaction: open.reference, entries: [...open.model.entries].map(([key, mutation]) => ({ key, mutation })) } };
    this.#open = undefined;
    if (open.model.state === "aborted") return { step: { kind: "aborted", transaction: open.reference, reason: "tool" } };
    return { step: open.model.entries.size === 0 ? { kind: "empty", transaction: open.reference } : { kind: "committed", transaction: open.reference, mutations: [...open.model.entries.values()] } };
  }
}
//#endregion 🔖️Gesture

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

  test("the fixture validates against its schema definition", () => {
    const validate = validator("TransactionLawFixture");
    expect(validate(law), JSON.stringify(validate.errors)).toBe(true);
  });

  test("hostile fixture mutations are rejected by the schema", () => {
    const validate = validator("TransactionLawFixture");
    expect(validate({ ...law, extra: true })).toBe(false);
    expect(validate({ ...law, matrix: law.matrix.slice(1) })).toBe(false);
    expect(validate({ ...law, ids: [{ ...law.ids[0], id: "tx-XYZ" }] })).toBe(false);
    expect(validate({ ...law, refusals: ["toolTransaction.closed", "toolTransaction.stale"] })).toBe(false);
    expect(validate({ ...law, abortReasons: [...law.abortReasons.slice(1), "sleep"] })).toBe(false);
    expect(validator("ToolYield")({ kind: "upsert", mutation: 1 })).toBe(false);
    expect(validator("ToolYield")({ kind: "commit", key: "a" })).toBe(false);
    const reference = { id: law.ids[0].id, tool: law.ids[0].tool };
    expect(validator("ToolStep")({ kind: "committed", transaction: reference, mutations: [] })).toBe(false);
    expect(validator("ToolStep")({ kind: "committed", transaction: reference, mutations: [1] })).toBe(true);
    expect(validator("GestureEvent")({ type: "pointerMove", x: 1 })).toBe(false);
    expect(validator("ToolStep")({ kind: "aborted", transaction: reference })).toBe(false);
    expect(validator("ToolStep")({ kind: "aborted", transaction: reference, reason: "blur" })).toBe(true);
    expect(validator("GestureScenario")({ name: "reset", steps: [{ host: "reset", state: "idle", dropped: reference, reason: "blur" }] })).toBe(false);
  });
});

describe("transaction law", () => {
  test("states, yield kinds, refusals and abort reasons mirror the fixture", () => {
    expect([...T.TOOL_TRANSACTION_STATES]).toEqual(law.states);
    expect([...T.TOOL_YIELD_KINDS]).toEqual(law.yieldKinds);
    expect([...T.TOOL_REFUSALS]).toEqual(law.refusals);
    expect([...T.TOOL_ABORT_REASONS]).toEqual(law.abortReasons);
  });

  test("the reducer matches every matrix row", () => {
    const sample = (kind: T.ToolYieldKind): T.ToolYield<number> => (kind === "upsert" ? { kind, key: "k", mutation: 1 } : kind === "retract" ? { kind, key: "k" } : { kind });
    for (const row of law.matrix) {
      const transaction = new T.ToolTransaction<number>({ id: "tx-0000000000000000", tool: "law#matrix" });
      if (row.from !== "open") transaction.apply({ kind: row.from === "committed" ? "commit" : "abort" });
      const before = { state: transaction.state, entries: entriesJson(transaction.entries()) };
      const result = transaction.apply(sample(row.yield));
      const label = `${row.from} × ${row.yield}`;
      if ("refusal" in row) {
        expect(result.ok ? "accepted" : result.refusal, label).toBe(row.refusal);
        expect({ state: transaction.state, entries: entriesJson(transaction.entries()) }, label).toEqual(before);
      } else {
        expect(result.ok, label).toBe(true);
        expect(transaction.state, label).toBe(row.to);
      }
    }
  });

  test("resume upserts entries in order", () => {
    const resumed = T.ToolTransaction.resume({ id: "tx-0000000000000000", tool: "law#resume" }, [["a", 1], ["b", 2], ["a", 3]]);
    expect(resumed.state).toBe("open");
    expect(entriesJson(resumed.entries())).toEqual([{ key: "a", mutation: 3 }, { key: "b", mutation: 2 }]);
  });

  test("the reducer matches every case", () => {
    for (const row of law.cases) {
      const transaction = new T.ToolTransaction<unknown>(row.transaction);
      const refused: number[] = [];
      row.yields.forEach((yielded: T.ToolYield<unknown>, index: number) => {
        if (!transaction.apply(yielded).ok) refused.push(index);
      });
      expect(transaction.state, row.name).toBe(row.expect.state);
      expect(entriesJson(transaction.entries()), row.name).toEqual(row.expect.entries);
      expect(refused, row.name).toEqual(row.expect.refused);
      expect(transaction.mutations(), row.name).toEqual(row.expect.entries.map((entry: Json) => entry.mutation));
      if (transaction.state === "open") {
        const rebuilt = T.ToolTransaction.resume(transaction.reference, transaction.entries());
        expect({ state: rebuilt.state, entries: entriesJson(rebuilt.entries()) }, row.name).toEqual({ state: "open", entries: row.expect.entries });
      }
    }
  });

  test("random yield sequences: the reducer and the Map oracle agree on state, order and refusals", () => {
    const yielded = fc.oneof(
      { arbitrary: fc.record({ kind: fc.constant("upsert" as const), key: fc.constantFrom("a", "b", "c", "d"), mutation: fc.integer({ min: 0, max: 9 }) }), weight: 6 },
      { arbitrary: fc.record({ kind: fc.constant("retract" as const), key: fc.constantFrom("a", "b", "c", "d", "z") }), weight: 3 },
      { arbitrary: fc.constant({ kind: "commit" as const }), weight: 1 },
      { arbitrary: fc.constant({ kind: "abort" as const }), weight: 1 },
    );
    fc.assert(
      fc.property(fc.array(yielded, { maxLength: 40 }), (yields) => {
        const transaction = new T.ToolTransaction<number>({ id: "tx-0000000000000000", tool: "law#random" });
        const oracle = new MapTransaction();
        for (const item of yields) expect(transaction.apply(item).ok).toBe(oracle.apply(item));
        expect(transaction.state).toBe(oracle.state);
        expect(entriesJson(transaction.entries())).toEqual([...oracle.entries].map(([key, mutation]) => ({ key, mutation })));
      }),
      { numRuns: 500 },
    );
  });

  test("the replication mint reproduces every fixture id", () => {
    for (const row of law.ids) expect(mintTransactionRef(row.actor, toClock(row.clock), row.tool)).toEqual({ id: row.id, tool: row.tool });
  });
});

describe("tool machine runner", () => {
  test("the kernel tables are the fixture charts", () => {
    expect(variants.map((candidate) => candidate.chart.id)).toEqual(["drag_gesture", "leaky_gesture"]);
    for (const { chart, machine } of variants) {
      const { nodes, transitions } = machine.definition;
      expect(nodes.slice(1).map((node) => node.stableId)).toEqual(chart.states);
      expect(nodes[nodes[0]!.initial!]!.stableId).toBe(chart.initial);
      const rows = transitions.map((row) => ({ from: nodes[row.source]!.stableId, event: EVENT_NAMES[(row.trigger as { event: number }).event], to: nodes[row.targets[0]!]!.stableId, action: row.actions.length === 0 ? null : ACTION_NAMES[row.actions[0]!] }));
      expect(rows).toEqual(chart.transitions);
    }
  });

  test("the runner replays every gesture scenario", () => {
    for (const { chart, scenarios, machine } of variants) {
      for (const scenario of scenarios) {
        for (const everywhere of [false, true]) {
          const slot: Slot = { runner: startRunner(machine) };
          scenario.steps.forEach((row: Json, index: number) => {
            expect(runRunner(slot, chart, rowOperation(row), row.clock), `${chart.id}: ${scenario.name} #${index} (resume everywhere: ${everywhere})`).toEqual(expectedOutcome(row));
            if (everywhere) persistAndResume(slot);
          });
        }
      }
    }
  });

  test("the xstate oracle agrees with every gesture scenario", () => {
    for (const { chart, scenarios } of variants) {
      for (const scenario of scenarios) {
        const oracle = new OracleRunner(chart);
        scenario.steps.forEach((row: Json, index: number) => expect(oracle.apply(rowOperation(row), row.clock), `${chart.id}: ${scenario.name} #${index}`).toEqual(expectedOutcome(row)));
      }
    }
  });

  test("random gestures with host cancels and resumes: the runner and the xstate oracle agree on state, steps, refusals and ids", () => {
    const point = (x: [number, number], y: [number, number]) => ({ x: fc.integer({ min: x[0], max: x[1] }), y: fc.integer({ min: y[0], max: y[1] }) });
    const operation: fc.Arbitrary<Operation> = fc.oneof(
      { arbitrary: fc.record({ type: fc.constant("pointerDown" as const), ...point([98, 102], [-1, 1]) }), weight: 2 },
      { arbitrary: fc.record({ type: fc.constant("pointerMove" as const), ...point([98, 102], [-1, 1]) }), weight: 3 },
      { arbitrary: fc.record({ type: fc.constant("pointerMove" as const), ...point([70, 112], [-6, 6]) }), weight: 3 },
      { arbitrary: fc.constant({ type: "pointerUp" as const }), weight: 2 },
      { arbitrary: fc.constant({ type: "escape" as const }), weight: 1 },
      { arbitrary: fc.record({ host: fc.constant("abort" as const), reason: fc.constantFrom(...T.TOOL_ABORT_REASONS) }), weight: 1 },
      { arbitrary: fc.constant({ host: "reset" as const }), weight: 1 },
      { arbitrary: fc.constant({ host: "resume" as const }), weight: 2 },
    );
    for (const id of ["drag_gesture", "leaky_gesture"]) {
      const { chart, machine } = variant(id);
      fc.assert(
        fc.property(fc.array(operation, { minLength: 1, maxLength: 60 }), (operations) => {
          const slot: Slot = { runner: startRunner(machine) };
          const oracle = new OracleRunner(chart);
          operations.forEach((item, index) => {
            const clock = toClock({ actor: 7, physical_ms: 5000 + index, logical: 0 });
            expect(runRunner(slot, chart, item, clock)).toEqual(oracle.apply(item, clock));
          });
        }),
        { numRuns: 300 },
      );
    }
  });

  test("the scenarios cover every step kind, host operation and the rest refusal", () => {
    const seen = new Set<string>();
    for (const { chart, scenarios } of variants) for (const scenario of scenarios) for (const row of scenario.steps) seen.add(row.refusal ?? (row.host === "reset" || row.host === "resume" ? row.host : `${chart.id === "drag_gesture" ? "" : "leaky:"}${row.step.kind}${row.host ? `:${row.host}` : ""}`));
    for (const kind of ["idle", "open", "committed", "empty", "aborted", "aborted:abort", "idle:abort", "reset", "resume", "toolTransaction.unclosed", "leaky:aborted"]) expect(seen.has(kind), kind).toBe(true);
  });

  test("resume admits exactly the fixture rows", () => {
    const { machine } = variant("drag_gesture");
    const clock = toClock({ actor: 7, physical_ms: 1, logical: 0 });
    const paths: Record<string, GestureEventJson[]> = { idle: [], pressed: [{ type: "pointerDown", x: 0, y: 0 }], dragging: [{ type: "pointerDown", x: 0, y: 0 }, { type: "pointerMove", x: 1, y: 0 }] };
    for (const row of gesture.resume) {
      const runner = startRunner(machine);
      for (const event of paths[row.state]!) expect(runner.send(gestureEvent(event), clock).ok).toBe(true);
      const [snapshot] = runner.intoParts();
      const open = () => T.ToolTransaction.resume<GestureMutation>(mintTransactionRef(gesture.actor, clock, gesture.tool), [["translate", { kind: "translate", dx: 1, dy: 0 }]]);
      const closed = (kind: "commit" | "abort") => {
        const transaction = open();
        transaction.apply({ kind });
        return transaction;
      };
      const transaction = row.transaction === "none" ? undefined : row.transaction === "open" ? open() : closed(row.transaction === "committed" ? "commit" : "abort");
      const result = resumed(machine, snapshot, transaction);
      expect(result.ok ? "ok" : result.refusal, JSON.stringify(row)).toBe(row.expect);
    }
  });

  interface ProbeSpec extends MachineSpec {
    Context: undefined;
    Event: StatechartEvent & { readonly type: "twice" | "arm" };
    Input: undefined;
    Output: never;
    Effect: T.ToolYield<number>;
  }
  const PROBE_EVENTS = ["twice", "arm"] as const;
  const probeEvent = (type: "twice" | "arm"): ProbeSpec["Event"] => ({ type, eventCount: 2, eventId: () => EventId(PROBE_EVENTS.indexOf(type)), eventName: (id) => PROBE_EVENTS[id]! });
  function probeMachine(entryYields: boolean): Machine<ProbeSpec> {
    const [resting, armed] = [NodeId(1), NodeId(2)];
    const nodes: NodeDef[] = [
      { stableId: "root", kind: "compound", initial: resting, children: [resting, armed], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
      { stableId: "resting", kind: "atomic", parent: ROOT, children: [], entryActions: entryYields ? [ActionId(3)] : [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
      { stableId: "armed", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [InvokeId(0)], timers: [[TimerId(0), 500]], docIndex: 2 },
    ];
    const transitions: TransitionDef[] = [
      { source: resting, trigger: { kind: "event", event: EventId(0) }, targets: [resting], kind: "external", actions: [ActionId(0)], docIndex: 0 },
      { source: resting, trigger: { kind: "event", event: EventId(1) }, targets: [armed], kind: "external", actions: [ActionId(1)], docIndex: 1 },
      { source: armed, trigger: { kind: "timer", timer: TimerId(0) }, targets: [resting], kind: "external", actions: [ActionId(2)], docIndex: 2 },
    ];
    const actions: ActionFn<ProbeSpec>[] = [
      (_context, _event, sink) => {
        sink.push({ kind: "effect", effect: { kind: "upsert", key: "a", mutation: 1 } });
        sink.push({ kind: "effect", effect: { kind: "commit" } });
        sink.push({ kind: "effect", effect: { kind: "upsert", key: "b", mutation: 2 } });
      },
      (_context, _event, sink) => sink.push({ kind: "effect", effect: { kind: "upsert", key: "hold", mutation: 1 } }),
      (_context, _event, sink) => sink.push({ kind: "effect", effect: { kind: "commit" } }),
      (_context, _event, sink) => sink.push({ kind: "effect", effect: { kind: "upsert", key: "early", mutation: 0 } }),
    ];
    return { definition: { id: "probe", nodes, transitions, contextFromInput: () => undefined, guards: [], actions, fingerprint: 0n, manifestJson: "{}" } };
  }
  const probeClock = (physical_ms: number) => toClock({ actor: 1, physical_ms, logical: 0 });

  test("a yield while entering the initial configuration is refused", () => {
    expect(T.ToolMachineRunner.start(probeMachine(true), "law#eager", "actor-1", undefined, new TestHost<ProbeSpec>())).toEqual({ ok: false, refusal: "toolTransaction.closed" });
  });

  test("a yield after the close within one event publishes nothing", () => {
    const started = T.ToolMachineRunner.start(probeMachine(false), "law#probe", "actor-1", undefined, new TestHost<ProbeSpec>());
    if (!started.ok) throw new Error(started.refusal);
    expect(started.runner.send(probeEvent("twice"), probeClock(10))).toEqual({ ok: false, refusal: "toolTransaction.closed" });
    expect(started.runner.transaction()).toBeUndefined();
    expect(started.runner.send(probeEvent("arm"), probeClock(10))).toEqual({ ok: true, step: { kind: "open" } });
  });

  test("timers route to the host and settle like events", () => {
    const host = new TestHost<ProbeSpec>();
    const started = T.ToolMachineRunner.start(probeMachine(false), "law#probe", "actor-1", undefined, host);
    if (!started.ok) throw new Error(started.refusal);
    expect(started.runner.send(probeEvent("arm"), probeClock(20))).toEqual({ ok: true, step: { kind: "open" } });
    const due = host.advance(500);
    expect(due).toEqual([[T.TOOL_MACHINE_ACTOR, TimerId(0)]]);
    expect(started.runner.timerElapsed(due[0]![1], probeClock(520))).toEqual({ ok: true, step: { kind: "committed", transaction: mintTransactionRef("actor-1", probeClock(20), "law#probe"), mutations: [1] } });
    expect(started.runner.snapshot.matches("resting")).toBe(true);
  });

  test("a host abort cancels the left timers and invokes and rests the tool", () => {
    const host = new TestHost<ProbeSpec>();
    const started = T.ToolMachineRunner.start(probeMachine(false), "law#probe", "actor-1", undefined, host);
    if (!started.ok) throw new Error(started.refusal);
    const { runner } = started;
    expect(runner.send(probeEvent("arm"), probeClock(30))).toEqual({ ok: true, step: { kind: "open" } });
    expect(host.startedTasks()).toEqual([[T.TOOL_MACHINE_ACTOR, InvokeId(0)]]);
    const reference = runner.transaction()!.reference;
    expect(runner.abort("frozen")).toEqual({ kind: "aborted", transaction: reference, reason: "frozen" });
    expect(runner.transaction()).toBeUndefined();
    expect(runner.atRest() && runner.snapshot.matches("resting")).toBe(true);
    expect(host.advance(500)).toEqual([]);
    expect(host.cancelledTasks()).toEqual([[T.TOOL_MACHINE_ACTOR, InvokeId(0)]]);
    expect(host.startedTasks()).toEqual([]);
    expect(runner.timerElapsed(TimerId(0), probeClock(30))).toEqual({ ok: true, step: { kind: "idle" } });
    expect(runner.abort("retired")).toEqual({ kind: "idle" });
    expect(runner.reset()).toBeUndefined();
  });
});
