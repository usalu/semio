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

//#region 🔖️Scrub
import scrubFixture from "../../🧫️fixtures/🧫️scrub-law/🔣️.json";

const scrubLaw = scrubFixture as any;
type Leaf = { readonly set: string; readonly target: string; readonly value: number };

function scrubStepJson(step: T.ToolStep<unknown>, open: T.ScrubState<unknown> | undefined): Json {
  return step.kind === "open" ? { kind: "open", transaction: open!.transaction } : step;
}

function scrubOpenJson(ledger: T.ScrubLedger<unknown>): Json {
  return Object.fromEntries(ledger.windows().map((window) => {
    const state = ledger.open(window)!;
    return [window, { gesture: state.gesture, base: state.baseRevision, tool: state.tool, transaction: state.transaction, entries: entriesJson(state.entries) }];
  }));
}

function scrubInput(json: Json): T.ScrubInput<unknown> {
  return json.kind === "abort" ? { kind: "abort", reason: json.reason } : { kind: json.kind, gesture: json.gesture, leaves: json.leaves };
}

const scrubClock = (physical: number): T.ToolClock => ({ actor: 0, physical_ms: physical, logical: 0 });

/** 🎛️ The xstate oracle of the scrub chart: guard `sameGesture`, actions logging yields, an independent `Map` transaction, the rest law, host cancels re-entering the initial snapshot, and the capture of another press as a host cancel first. */
class ScrubOracle {
  readonly #machine;
  #snapshot: AnyMachineSnapshot;
  #open: { reference: TransactionRef; model: MapTransaction } | undefined;

  constructor(readonly tool: string, readonly actor: string) {
    const replace = (context: any, leaves: readonly unknown[]) => [...leaves.map((mutation, index) => ({ kind: "upsert", key: String(index), mutation })), ...Array.from({ length: Math.max(0, context.keys - leaves.length) }, (_, offset) => ({ kind: "retract", key: String(leaves.length + offset) }))];
    this.#machine = setup({
      guards: { sameGesture: ({ context, event }: any) => context.gesture === event.gesture } as any,
      actions: {
        follow: assign(({ context, event }: any) => ({ log: [...context.log, ...replace(context, event.leaves)], gesture: event.gesture, keys: event.leaves.length })),
        settle: assign(({ context, event }: any) => ({ log: [...context.log, ...replace(context, event.leaves), { kind: "commit" }], gesture: undefined, keys: 0 })),
      } as any,
    }).createMachine({
      id: "scrub-oracle",
      initial: scrubLaw.chart.initial,
      context: { gesture: undefined, keys: 0, log: [] },
      states: Object.fromEntries(scrubLaw.chart.states.map((state: string) => [state, { on: Object.fromEntries(scrubLaw.chart.events.map((event: string) => [event, scrubLaw.chart.transitions.filter((row: Json) => row.from === state && row.event === event).map((row: Json) => ({ target: row.to, actions: [row.action], ...(row.guard ? { guard: row.guard } : {}) }))])) }])),
    } as any);
    this.#snapshot = initialTransition(this.#machine)[0];
  }

  get state(): string {
    return this.#snapshot.value as string;
  }

  #rest(): TransactionRef | undefined {
    this.#snapshot = initialTransition(this.#machine)[0];
    const dropped = this.#open?.reference;
    this.#open = undefined;
    return dropped;
  }

  send(input: T.ScrubInput<unknown>, clock: T.ToolClock): Json {
    if (input.kind === "abort") {
      const dropped = this.#rest();
      return dropped ? { kind: "aborted", transaction: dropped, reason: input.reason } : { kind: "idle" };
    }
    const open = this.#snapshot.context.gesture;
    if (open !== undefined && open !== input.gesture) this.#rest();
    const before = this.#snapshot.context.log.length;
    this.#snapshot = transition(this.#machine, this.#snapshot, { type: input.kind === "tick" ? "Tick" : "Commit", gesture: input.gesture, leaves: input.leaves } as any)[0];
    for (const yielded of this.#snapshot.context.log.slice(before) as T.ToolYield<unknown>[]) {
      if (this.#open) this.#open.model.apply(yielded);
      else if (yielded.kind === "upsert") {
        this.#open = { reference: mintTransactionRef(this.actor, clock, this.tool), model: new MapTransaction() };
        this.#open.model.apply(yielded);
      }
    }
    const current = this.#open;
    if (!current) return { kind: "idle" };
    if (current.model.state === "open") return { kind: "open", transaction: current.reference, entries: [...current.model.entries].map(([key, mutation]) => ({ key, mutation })) };
    this.#open = undefined;
    return current.model.entries.size === 0 ? { kind: "empty", transaction: current.reference } : { kind: "committed", transaction: current.reference, mutations: [...current.model.entries.values()] };
  }
}

describe("scrub machine", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

  test("the scrub fixture validates and hostile mutations are rejected (ajv)", () => {
    const validate = validator("ScrubLawFixture");
    expect(validate(scrubLaw), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...scrubLaw, extra: 1 })).toBe(false);
    expect(validate({ ...scrubLaw, chart: { ...scrubLaw.chart, fingerprint: "0x1" } })).toBe(false);
    expect(validator("ScrubPhase")({ kind: "abort", gesture: "g", reason: "sideways" })).toBe(false);
    expect(validator("ScrubPhase")({ kind: "tick", gesture: "" })).toBe(false);
    expect(validator("ScrubInput")({ kind: "commit", gesture: "g" })).toBe(false);
    expect(validator("ScrubStep")({ kind: "committed", transaction: { id: "tx-0000000000000000", tool: "a#b" }, mutations: [] })).toBe(false);
  });

  test("the protocol arguments parse like the fixture", () => {
    expect(scrubLaw.args).toEqual({ gesture: T.SCRUB_GESTURE_ARG, commit: T.SCRUB_COMMIT_ARG, abort: T.SCRUB_ABORT_ARG });
    for (const row of scrubLaw.phases) expect(T.parseScrubPhase(row.args.gesture, row.args.commit, row.args.abort) ?? null, JSON.stringify(row.args)).toEqual(row.phase);
  });

  test("the kernel tables are the fixture chart with the Rust fingerprint", () => {
    const { definition } = T.scrubMachine<unknown>();
    expect([definition.id, definition.fingerprint.toString(), definition.manifestJson]).toEqual([scrubLaw.chart.id, scrubLaw.chart.fingerprint, scrubLaw.chart.manifestJson]);
    expect(definition.nodes.slice(1).map((node) => node.stableId)).toEqual(scrubLaw.chart.states);
    expect(definition.nodes[definition.nodes[0]!.initial!]!.stableId).toBe(scrubLaw.chart.initial);
    const rows = definition.transitions.map((row) => ({ from: definition.nodes[row.source]!.stableId, event: scrubLaw.chart.events[(row.trigger as { event: number }).event], guard: row.guard === undefined ? null : "sameGesture", to: definition.nodes[row.targets[0]!]!.stableId, action: ["follow", "settle"][row.actions[0]!] }));
    expect(rows).toEqual(scrubLaw.chart.transitions);
  });

  test("the ledger replays every fixture scenario with the independently minted ids", () => {
    for (const scenario of scrubLaw.scenarios) {
      const ledger = new T.ScrubLedger<unknown>();
      scenario.steps.forEach((row: Json, index: number) => {
        const label = `${scenario.name} #${index}`;
        if (row.abortAll !== undefined) expect(ledger.abortAll(row.abortAll), label).toEqual(row.expect.steps);
        else if (row.retain !== undefined) expect(ledger.retainWindows((window) => row.retain.includes(window)), label).toEqual(row.expect.steps);
        else if (row.abort !== undefined) expect(ledger.abort(row.window, row.abort.gesture ?? undefined, row.abort.reason), label).toEqual(row.expect.step);
        else {
          const result = ledger.send(row.window, row.tool, scrubLaw.actor, row.base, scrubInput(row.input), scrubClock(row.clock));
          if (!result.ok) throw new Error(result.refusal);
          expect(scrubStepJson(result.step, ledger.open(row.window)), label).toEqual(row.expect.step);
        }
        expect(scrubOpenJson(ledger), label).toEqual(row.expect.open);
        expect(ledger.provisional(), label).toEqual(row.expect.provisional);
      });
    }
  });

  test("random presses with captures, host cancels and resumes: the scrub and the xstate oracle agree on steps, entries and ids", () => {
    const leaves = fc.array(fc.record({ set: fc.constant("opacity"), target: fc.constantFrom("a", "b", "c"), value: fc.integer({ min: 0, max: 9 }) }), { maxLength: 3 });
    type Operation = { readonly input: T.ScrubInput<Leaf> } | { readonly resume: true };
    const operation: fc.Arbitrary<Operation> = fc.oneof(
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("tick" as const), gesture: fc.constantFrom("g1", "g2"), leaves }) }), weight: 6 },
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("commit" as const), gesture: fc.constantFrom("g1", "g2"), leaves }) }), weight: 2 },
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("abort" as const), reason: fc.constantFrom(...T.TOOL_ABORT_REASONS) }) }), weight: 1 },
      { arbitrary: fc.constant({ resume: true as const }), weight: 2 },
    );
    fc.assert(
      fc.property(fc.array(operation, { minLength: 1, maxLength: 50 }), (operations) => {
        let scrub = T.Scrub.start<Leaf>("law#scrub", scrubLaw.actor, "r1");
        const oracle = new ScrubOracle("law#scrub", scrubLaw.actor);
        operations.forEach((item, index) => {
          if ("resume" in item) {
            const state = scrub.persist();
            if (!state) {
              expect(scrub.transaction()?.state === "open").toBe(false);
              scrub = T.Scrub.start<Leaf>("law#scrub", scrubLaw.actor, "r1");
              if (oracle.state !== scrubLaw.chart.initial) oracle.send({ kind: "abort", reason: "tool" }, scrubClock(0));
              return;
            }
            const resumed = T.Scrub.resume(state);
            if (!resumed.ok) throw new Error(resumed.refusal);
            scrub = resumed.scrub;
            return;
          }
          const clock = scrubClock(7000 + index);
          const result = scrub.send(item.input, clock);
          if (!result.ok) throw new Error(result.refusal);
          const open = scrub.transaction();
          const step = result.step.kind === "open" ? { kind: "open", transaction: open!.reference, entries: entriesJson(open!.entries()) } : result.step;
          expect(step).toEqual(oracle.send(item.input, clock));
          expect(open === undefined || scrub.runner.atRest() === false).toBe(true);
        });
      }),
      { numRuns: 400 },
    );
  });

  test("random ledger inputs across windows, tools and revisions are never refused, and a released press stays silent (CLOSURE-3)", () => {
    const leaves = fc.array(fc.integer({ min: 0, max: 9 }), { maxLength: 2 });
    const press = { window: fc.constantFrom("w1", "w2"), tool: fc.constantFrom("t1", "t2"), base: fc.constantFrom("r1", "r2") };
    const input: fc.Arbitrary<T.ScrubInput<number>> = fc.oneof(
      { arbitrary: fc.record({ kind: fc.constant("tick" as const), gesture: fc.constantFrom("g1", "g2"), leaves }), weight: 5 },
      { arbitrary: fc.record({ kind: fc.constant("commit" as const), gesture: fc.constantFrom("g1", "g2"), leaves }), weight: 2 },
      { arbitrary: fc.record({ kind: fc.constant("abort" as const), reason: fc.constantFrom(...T.TOOL_ABORT_REASONS) }), weight: 1 },
    );
    fc.assert(
      fc.property(fc.array(fc.record({ ...press, input }), { minLength: 1, maxLength: 40 }), (rows) => {
        const ledger = new T.ScrubLedger<number>();
        rows.forEach((row, index) => {
          const result = ledger.send(row.window, row.tool, scrubLaw.actor, row.base, row.input, scrubClock(index));
          expect(result.ok).toBe(true);
          if (row.input.kind !== "commit") return;
          expect(ledger.open(row.window)?.gesture === row.input.gesture).toBe(false);
          const before = [scrubOpenJson(ledger), ledger.provisional()];
          for (const late of [{ ...row.input, kind: "tick" as const }, row.input]) expect(ledger.send(row.window, row.tool, scrubLaw.actor, row.base, late, scrubClock(index))).toEqual({ ok: true, step: { kind: "idle" } });
          expect([scrubOpenJson(ledger), ledger.provisional()]).toEqual(before);
        });
      }),
      { numRuns: 400 },
    );
  });
});
//#endregion 🔖️Scrub

//#region 🔖️Typing
import typingFixture from "../../🧫️fixtures/🧫️typing-law/🔣️.json";

const typingLaw = typingFixture as any;
type Run = { readonly at: number; readonly text: string } | { readonly text: string };

/** 🔗️ The fixture's typing algebra: an insertion folds into an open one that ends where it starts, a whole buffer replaces a whole buffer, an edit without leaves cancels the net, anything else splits. */
function fixtureFold(net: readonly Json[], next: readonly Json[]): T.TypingFold<Json> {
  if (next.length === 0) return { kind: "net", leaves: [] };
  if (net.length !== 1 || next.length !== 1) return { kind: "split" };
  const [open, typed] = [net[0], next[0]];
  if ("at" in open && "at" in typed) return typed.at === open.at + Array.from(open.text as string).length ? { kind: "net", leaves: [{ at: open.at, text: open.text + typed.text }] } : { kind: "split" };
  if (!("at" in open) && !("at" in typed)) return { kind: "net", leaves: [typed] };
  return { kind: "split" };
}

const typingClock = (physical: number, logical = 0): T.ToolClock => ({ actor: 0, physical_ms: physical, logical });

function typingStepJson(ledger: T.TypingLedger<Json>, window: string, step: T.ToolStep<Json>): Json {
  return step.kind === "open" ? { kind: "open", transaction: ledger.open(window)!.transaction } : step;
}

function typingOpenJson(ledger: T.TypingLedger<Json>): Json {
  return Object.fromEntries(ledger.windows().map((window) => {
    const state = ledger.open(window)!;
    return [window, { buffer: state.buffer, tool: state.tool, deadline: state.deadlineMs, transaction: state.transaction, entries: entriesJson(state.entries) }];
  }));
}

function typingWindowed(ledger: T.TypingLedger<Json>, all: readonly (readonly [string, T.ToolStepResult<Json>])[]): Json {
  return all.map(([window, result]) => {
    if (!result.ok) throw new Error(result.refusal);
    return [window, typingStepJson(ledger, window, result.step)];
  });
}

/** ⌨️ The xstate oracle of the typing chart: guard `sameBuffer`, actions logging yields, an explicit `Lapse` event the oracle sends from its own deadline bookkeeping (the chart's `after` timer), an independent `Map` transaction, the split rule (another buffer or a split fold commits first) and host aborts re-entering the initial snapshot. */
class TypingOracle {
  readonly #machine;
  #snapshot: AnyMachineSnapshot;
  #open: { reference: TransactionRef; model: MapTransaction } | undefined;
  #deadline: number | undefined;

  constructor(readonly tool: string, readonly actor: string) {
    const replace = (context: any, leaves: readonly unknown[]) => [...leaves.map((mutation, index) => ({ kind: "upsert", key: String(index), mutation })), ...Array.from({ length: Math.max(0, context.keys - leaves.length) }, (_, offset) => ({ kind: "retract", key: String(leaves.length + offset) }))];
    const rows = typingLaw.chart.transitions as Json[];
    const on = (state: string) => Object.fromEntries(["Edit", "Commit"].map((event) => [event, rows.filter((row) => row.from === state && row.trigger.event === event).map((row) => ({ target: row.to, actions: [row.action], ...(row.guard ? { guard: row.guard } : {}) }))]));
    const lapse = (state: string) => rows.filter((row) => row.from === state && row.trigger.afterMs !== undefined).map((row) => ({ target: row.to, actions: [row.action] }));
    this.#machine = setup({
      guards: { sameBuffer: ({ context, event }: any) => context.buffer === event.buffer } as any,
      actions: {
        follow: assign(({ context, event }: any) => ({ log: [...context.log, ...replace(context, event.leaves)], buffer: event.buffer, keys: event.leaves.length })),
        settle: assign(({ context }: any) => ({ log: [...context.log, { kind: "commit" }], buffer: undefined, keys: 0 })),
      } as any,
    }).createMachine({
      id: "typing-oracle",
      initial: typingLaw.chart.initial,
      context: { buffer: undefined, keys: 0, log: [] },
      states: Object.fromEntries(typingLaw.chart.states.filter((state: string) => state !== "root").map((state: string) => [state, { on: { ...on(state), Lapse: lapse(state) } }])),
    } as any);
    this.#snapshot = initialTransition(this.#machine)[0];
  }

  #run(event: Json, clock: T.ToolClock): Json {
    const before = this.#snapshot.context.log.length;
    this.#snapshot = transition(this.#machine, this.#snapshot, event)[0];
    for (const yielded of this.#snapshot.context.log.slice(before) as T.ToolYield<unknown>[]) {
      if (this.#open) this.#open.model.apply(yielded);
      else if (yielded.kind === "upsert") {
        this.#open = { reference: mintTransactionRef(this.actor, clock, this.tool), model: new MapTransaction() };
        this.#open.model.apply(yielded);
      }
    }
    this.#deadline = this.#snapshot.value === "typing" ? (event.type === "Edit" ? clock.physical_ms + typingLaw.idleMs : this.#deadline) : undefined;
    const current = this.#open;
    if (!current) return { kind: "idle" };
    if (current.model.state === "open") return { kind: "open", transaction: current.reference, entries: [...current.model.entries].map(([key, mutation]) => ({ key, mutation })) };
    this.#open = undefined;
    return current.model.entries.size === 0 ? { kind: "empty", transaction: current.reference } : { kind: "committed", transaction: current.reference, mutations: [...current.model.entries.values()] };
  }

  send(input: T.TypingInput<Json>, clock: T.ToolClock): Json[] {
    if (input.kind === "abort") {
      this.#snapshot = initialTransition(this.#machine)[0];
      const dropped = this.#open?.reference;
      this.#open = undefined;
      this.#deadline = undefined;
      return [dropped ? { kind: "aborted", transaction: dropped, reason: input.reason } : { kind: "idle" }];
    }
    if (input.kind === "commit") return [this.#run({ type: "Commit" }, clock)];
    const steps: Json[] = [];
    if (this.#deadline !== undefined && this.#deadline <= clock.physical_ms) steps.push(this.#run({ type: "Lapse" }, clock));
    const buffer = this.#snapshot.context.buffer;
    const fold = buffer === undefined ? { kind: "net", leaves: input.leaves } : buffer !== input.buffer ? { kind: "split" } : this.#open ? fixtureFold([...this.#open.model.entries.values()], input.leaves) : { kind: "net", leaves: input.leaves };
    if (fold.kind === "split") steps.push(this.#run({ type: "Commit" }, clock));
    steps.push(this.#run({ type: "Edit", buffer: input.buffer, leaves: fold.kind === "net" ? fold.leaves : input.leaves }, clock));
    return steps;
  }

  lapse(clock: T.ToolClock): Json | undefined {
    return this.#deadline !== undefined && this.#deadline <= clock.physical_ms ? this.#run({ type: "Lapse" }, clock) : undefined;
  }
}

describe("typing machine", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

  test("the typing fixture validates and hostile mutations are rejected (ajv)", () => {
    const validate = validator("TypingLawFixture");
    expect(validate(typingLaw), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...typingLaw, extra: 1 })).toBe(false);
    expect(validate({ ...typingLaw, idleMs: 2000 })).toBe(false);
    expect(validate({ ...typingLaw, chart: { ...typingLaw.chart, fingerprint: "0x1" } })).toBe(false);
    expect(validator("TypingPhase")({ kind: "commit", buffer: "s", reason: "paste" })).toBe(false);
    expect(validator("TypingPhase")({ kind: "edit", buffer: "" })).toBe(false);
    expect(validator("TypingInput")({ kind: "edit", buffer: "s" })).toBe(false);
    expect(validator("TypingOpen")({ buffer: "s", tool: "a#b", transaction: { id: "tx-0000000000000000", tool: "a#b" }, entries: [] })).toBe(false);
  });

  test("the protocol arguments and reasons parse like the fixture", () => {
    expect(typingLaw.args).toEqual({ buffer: T.TYPING_BUFFER_ARG, commit: T.TYPING_COMMIT_ARG });
    expect([typingLaw.idleMs, typingLaw.reasons]).toEqual([T.TYPING_IDLE_MS, [...T.TYPING_COMMITS]]);
    for (const row of typingLaw.phases) expect(T.parseTypingPhase(row.args.typing, row.args.typingCommit) ?? null, JSON.stringify(row.args)).toEqual(row.phase);
  });

  test("the kernel tables are the fixture chart with the Rust fingerprint", () => {
    const { definition } = T.typingMachine<unknown>();
    expect([definition.id, definition.fingerprint.toString(), definition.manifestJson]).toEqual([typingLaw.chart.id, typingLaw.chart.fingerprint, typingLaw.chart.manifestJson]);
    expect(definition.nodes.map((node) => node.stableId)).toEqual(typingLaw.chart.states);
    expect(definition.nodes[definition.nodes[0]!.initial!]!.stableId).toBe(typingLaw.chart.initial);
    const rows = definition.transitions.map((row) => ({
      from: definition.nodes[row.source]!.stableId,
      trigger: row.trigger.kind === "event" ? { event: ["Edit", "Commit"][row.trigger.event] } : { afterMs: definition.nodes[row.source]!.timers.find(([timer]) => row.trigger.kind === "timer" && timer === row.trigger.timer)![1] },
      guard: row.guard === undefined ? null : "sameBuffer",
      to: definition.nodes[row.targets[0]!]!.stableId,
      action: ["follow", "settle"][row.actions[0]!],
    }));
    expect(rows).toEqual(typingLaw.chart.transitions);
  });

  test("the fixture algebra folds like the fixture", () => {
    for (const row of typingLaw.algebra.folds) expect(fixtureFold(row.net, row.next)).toEqual(row.fold);
  });

  test("the ledger replays every fixture scenario with the independently minted ids", () => {
    for (const scenario of typingLaw.scenarios) {
      const ledger = new T.TypingLedger<Json>();
      scenario.steps.forEach((row: Json, index: number) => {
        const label = `${scenario.name} #${index}`;
        if (row.lapse !== undefined) expect(typingWindowed(ledger, ledger.lapse(typingClock(row.lapse))), label).toEqual(row.expect.lapsed);
        else if (row.abortAll !== undefined) expect(ledger.abortAll(row.abortAll), label).toEqual(row.expect.all);
        else if (row.commitAll !== undefined) expect(typingWindowed(ledger, ledger.commitAll(row.commitAll, typingClock(row.clock))), label).toEqual(row.expect.all);
        else if (row.retain !== undefined) expect(typingWindowed(ledger, ledger.retainWindows((window) => row.retain.includes(window), typingClock(row.clock))), label).toEqual(row.expect.all);
        else if (row.abort !== undefined) expect(ledger.abort(row.window, row.abort), label).toEqual(row.expect.step);
        else if (row.commit !== undefined) {
          const result = ledger.commit(row.window, row.commit, typingClock(row.clock));
          if (!result.ok) throw new Error(result.refusal);
          expect(typingStepJson(ledger, row.window, result.step), label).toEqual(row.expect.step);
        } else {
          const result = ledger.send(row.window, row.tool, typingLaw.actor, row.input, fixtureFold, typingClock(row.clock));
          if (!result.ok) throw new Error(result.refusal);
          expect(result.steps.map((step) => typingStepJson(ledger, row.window, step)), label).toEqual(row.expect.steps);
        }
        expect(typingOpenJson(ledger), label).toEqual(row.expect.open);
        expect(ledger.provisional(), label).toEqual(row.expect.provisional);
      });
    }
  });

  test("random typing with pauses, jumps, buffers, commits, aborts and resumes: the run and the xstate oracle agree on steps, entries and ids", () => {
    type Operation = { readonly input: T.TypingInput<Run>; readonly pause: number } | { readonly resume: true };
    const typed = fc.oneof(fc.record({ at: fc.integer({ min: 0, max: 6 }), text: fc.constantFrom("a", "b", "😀") }), fc.record({ text: fc.constantFrom("x", "xy") }));
    const operation: fc.Arbitrary<Operation> = fc.oneof(
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("edit" as const), buffer: fc.constantFrom("s1", "s2"), leaves: fc.array(typed, { maxLength: 1 }) }), pause: fc.integer({ min: 1, max: 1200 }) }), weight: 8 },
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("commit" as const), reason: fc.constantFrom(...T.TYPING_COMMITS) }), pause: fc.integer({ min: 1, max: 900 }) }), weight: 2 },
      { arbitrary: fc.record({ input: fc.record({ kind: fc.constant("abort" as const), reason: fc.constantFrom("baseMoved" as const, "frozen" as const) }), pause: fc.integer({ min: 1, max: 900 }) }), weight: 1 },
      { arbitrary: fc.constant({ resume: true as const }), weight: 2 },
    );
    fc.assert(
      fc.property(fc.array(operation, { minLength: 1, maxLength: 60 }), (operations) => {
        let typing = T.Typing.start<Json>("law#textSplice", typingLaw.actor);
        const oracle = new TypingOracle("law#textSplice", typingLaw.actor);
        let now = 10_000;
        operations.forEach((item, index) => {
          if ("resume" in item) {
            const state = typing.persist();
            if (!state) return;
            const resumed = T.Typing.resume<Json>(state);
            if (!resumed.ok) throw new Error(resumed.refusal);
            typing = resumed.typing;
            return;
          }
          now += item.pause;
          const clock = typingClock(now, index);
          const result = typing.send(item.input, fixtureFold, clock);
          if (!result.ok) throw new Error(result.refusal);
          const open = typing.transaction();
          const steps = result.steps.map((step) => (step.kind === "open" ? { kind: "open", transaction: open!.reference, entries: entriesJson(open!.entries()) } : step));
          expect(steps).toEqual(oracle.send(item.input, clock));
          expect(typing.deadlineMs === undefined || typing.deadlineMs > now - item.pause).toBe(true);
        });
        now += T.TYPING_IDLE_MS;
        const lapsed = typing.lapse(typingClock(now, 999));
        const expected = oracle.lapse(typingClock(now, 999));
        expect(lapsed?.ok ? lapsed.step : undefined).toEqual(expected);
      }),
      { numRuns: 400 },
    );
  });
});
//#endregion 🔖️Typing

//#region 🔖️NodeDrag
import nodeDragFixture from "../../🧫️fixtures/🧫️node-drag-law/🔣️.json";

describe("node drag record (design §13.3)", () => {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(schema as Json);
  const validateRow = ajv.getSchema(`${(schema as Json).$id}#/$defs/NodeDragRow`)!;
  const validateFixture = ajv.getSchema(`${(schema as Json).$id}#/$defs/NodeDragLawFixture`)!;
  const law = nodeDragFixture as Json;

  test("the fixture satisfies its schema", () => {
    expect(validateFixture(law), JSON.stringify(validateFixture.errors)).toBe(true);
  });

  test("every valid row decodes to its record, round trips and moves as stated; the schema agrees", () => {
    for (const valid of law.valid) {
      expect(validateRow(valid.row), `${valid.id}: ${JSON.stringify(validateRow.errors)}`).toBe(true);
      const decoded = T.nodeDragRecordFromRow(valid.row);
      expect(decoded, valid.id).toEqual({ ok: true, record: valid.record });
      if (!decoded.ok) continue;
      expect(T.nodeDragRecordFromRow(T.nodeDragRow(decoded.record)), valid.id).toEqual(decoded);
      expect(T.nodeDragMoves(decoded.record), valid.id).toBe(valid.moves);
    }
  });

  test("every invalid row is refused by the decoder and by the schema", () => {
    for (const invalid of law.invalid) {
      expect(T.nodeDragRecordFromRow(invalid.row).ok, `${invalid.id}: ${invalid.reason}`).toBe(false);
      expect(validateRow(invalid.row), `${invalid.id}: ${invalid.reason}`).toBe(false);
    }
  });

  test("a released node drag commits one transaction of its leaves, minted like every tool transaction", () => {
    const commit = law.commit;
    const committed = T.nodeDragCommit(commit.tool, commit.actor, commit.gesture, commit.leaves, toClock(commit.clock));
    expect(committed).toEqual({ transaction: mintTransactionRef(commit.actor, toClock(commit.clock), commit.tool), mutations: commit.leaves });
    expect(T.nodeDragCommit(commit.tool, commit.actor, commit.gesture, [], toClock(commit.clock))).toBeUndefined();
  });
});
//#endregion 🔖️NodeDrag
