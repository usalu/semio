/** 🌊️ The shared streamed-gesture runner (`driveGesture`) against `🧫️fixtures/🧫️gesture-drive-law`, authored by an independent Python model of its counting tool (`🧪️s4-tools-a-gesture-drive-law.py` of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); oracles: ajv (schema and hostile mutations), xstate (the window's gesture slot as an idle/open/unrestorable machine, judged on every fixture row too), fast-check (random dispatch sequences with unrestorable gestures, runner vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, initialTransition, setup, transition, type SnapshotFrom } from "xstate";
import * as T from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🧫️gesture-drive-law/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

type Gesture = { readonly verb: string; readonly base: string; readonly ticks: readonly number[]; readonly corrupt?: true };
type Dispatch = { readonly verb: string; readonly phase?: string; readonly reason?: string; readonly tick: number | null; readonly base: string };
type Outcome = { readonly aborted: T.ToolAbortReason | null; readonly committed: readonly number[] | null; readonly next: "unchanged" | "cleared" | Gesture; readonly refused: T.ToolRefusal | null };
type Row = { readonly name: string; readonly persisted: Gesture | null; readonly dispatch: Dispatch; readonly expected: Outcome };
type Law = {
  readonly schema: string;
  readonly tool: { readonly refuseStartVerb: string; readonly startRefusal: T.ToolRefusal; readonly resumeRefusal: T.ToolRefusal; readonly sendRefusal: T.ToolRefusal };
  readonly phases: ReadonlyArray<{ readonly args: { readonly phase?: string; readonly reason?: string }; readonly phase: T.GesturePhase | null }>;
  readonly rows: readonly Row[];
};

const law = fixture as unknown as Law;

/** 🧾️ What the tools of one dispatch did beyond the drive's report: the reasons they were aborted with and the transactions they minted. */
type Trace = { readonly aborted: T.ToolAbortReason[]; minted: number };

/** 🧮️ The fixture's counting tool: a stream tick accumulates (open while it has ticks), a one-shot or commit folds its tick in and commits the ticks, an abort clears them; `send` refuses a negative tick. */
class CountingTool implements T.GestureTool<Gesture, number, number> {
  #ticks: number[];

  constructor(
    readonly verb: string,
    readonly baseRevision: string,
    ticks: readonly number[],
    readonly trace: Trace,
  ) {
    this.#ticks = [...ticks];
  }

  abort(reason: T.ToolAbortReason): void {
    this.trace.aborted.push(reason);
    this.#ticks = [];
  }

  send(phase: T.GesturePhase, tick: number | undefined): T.ToolStepResult<number> {
    if (tick !== undefined && tick < 0) return { ok: false, refusal: law.tool.sendRefusal };
    if (tick !== undefined) this.#ticks.push(tick);
    if (phase.kind === "stream") return { ok: true, step: { kind: this.#ticks.length > 0 ? "open" : "idle" } };
    const mutations = this.#ticks;
    this.#ticks = [];
    return { ok: true, step: mutations.length > 0 ? { kind: "committed", transaction: { id: `${this.verb}#${++this.trace.minted}`, tool: this.verb }, mutations } : { kind: "idle" } };
  }

  persist(): Gesture | undefined {
    return this.#ticks.length > 0 ? { verb: this.verb, base: this.baseRevision, ticks: this.#ticks } : undefined;
  }
}

/** 🪪️ Starts (refusing `refuseStartVerb`) and resumes (refusing an unrestorable gesture) counting tools that report into `trace`. */
function counting(trace: Trace): T.GestureToolKind<Gesture, number, number> {
  return {
    start: (verb, _authoringSeed, baseRevision) => (verb === law.tool.refuseStartVerb ? { ok: false, refusal: law.tool.startRefusal } : { ok: true, tool: new CountingTool(verb, baseRevision, [], trace) }),
    resume: (gesture) => (gesture.corrupt ? { ok: false, refusal: law.tool.resumeRefusal } : { ok: true, tool: new CountingTool(gesture.verb, gesture.base, gesture.ticks, trace) }),
    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),
  };
}

/** 🏃️ One dispatch through `driveGesture` from the window's persisted gesture, reported in the fixture's outcome shape. */
function drive(persisted: Gesture | null, dispatch: Dispatch): Outcome {
  const trace: Trace = { aborted: [], minted: 0 };
  const phase = T.parseGesturePhase(dispatch.phase, dispatch.reason);
  if (!phase) throw new Error(`no gesture phase: ${JSON.stringify(dispatch)}`);
  const result = T.driveGesture(counting(trace), persisted ?? undefined, dispatch.verb, phase, dispatch.tick ?? undefined, "seed", dispatch.base);
  if (trace.aborted.length > 1) throw new Error(`one dispatch aborted ${trace.aborted.length} gestures`);
  const aborted = trace.aborted[0] ?? null;
  if (!result.ok) return { aborted, committed: null, next: "unchanged", refused: result.refusal };
  const { committed, next } = result.drive;
  if (committed && (committed.transaction.tool !== dispatch.verb || trace.minted !== 1)) throw new Error("a commit is the dispatching verb's one transaction");
  return { aborted, committed: committed ? committed.mutations : null, next: next.kind === "persist" ? next.gesture : next.kind, refused: null };
}

/** 📡️ A typed phase as the wire arguments a gesture verb carries. */
function wire(verb: string, phase: T.GesturePhase, tick: number | undefined, base: string): Dispatch {
  return { verb, ...(phase.kind === "once" ? {} : { phase: phase.kind }), ...(phase.kind === "abort" ? { reason: phase.reason } : {}), tick: tick ?? null, base };
}

//#region 🎛️Oracle
type SlotState = "idle" | "open" | "unrestorable";
type Dispatched = { readonly type: "dispatch"; readonly verb: string; readonly phase: T.GesturePhase; readonly tick: number | undefined; readonly base: string };
type SlotEvent = Dispatched | { readonly type: "corrupt" };
type SlotContext = { readonly slot: Gesture | undefined; readonly last: Outcome | undefined };
type Judged = { readonly context: SlotContext; readonly event: SlotEvent };

const settled = (fields: Partial<Outcome>): Outcome => ({ aborted: null, committed: null, next: "unchanged", refused: null, ...fields });
const dispatched = (event: SlotEvent): Dispatched => {
  if (event.type !== "dispatch") throw new Error("a dispatch transition judged a host event");
  return event;
};
const slotOf = (context: SlotContext): Gesture => {
  if (!context.slot) throw new Error("an open slot without its gesture");
  return context.slot;
};
const tickOf = ({ event }: Judged): readonly number[] | null => {
  const tick = dispatched(event).tick;
  return tick === undefined ? null : [tick];
};
const kind = (...kinds: T.GesturePhase["kind"][]) => ({ event }: Judged) => kinds.includes(dispatched(event).phase.kind);
const ticked = ({ event }: Judged) => dispatched(event).tick !== undefined;
const startRefused = ({ event }: Judged) => dispatched(event).verb === law.tool.refuseStartVerb;
const tickRefused = ({ event }: Judged) => (dispatched(event).tick ?? 0) < 0;
const moved = ({ context, event }: Judged) => slotOf(context).base !== dispatched(event).base;
const otherVerb = ({ context, event }: Judged) => slotOf(context).verb !== dispatched(event).verb;
const interrupted = (judged: Judged) => moved(judged) || otherVerb(judged) || kind("once")(judged);
const opened = ({ event }: Judged): Gesture => {
  const { verb, base, tick } = dispatched(event);
  return { verb, base, ticks: tick === undefined ? [] : [tick] };
};
const record = (update: (judged: Judged) => SlotContext) => assign<SlotContext, SlotEvent, undefined, SlotEvent, never>(({ context, event }) => update({ context, event }));
const stay = (outcome: (judged: Judged) => Outcome) => record((judged) => ({ slot: judged.context.slot, last: outcome(judged) }));
const clear = (outcome: (judged: Judged) => Outcome) => record((judged) => ({ slot: undefined, last: outcome(judged) }));
const keep = (slot: (judged: Judged) => Gesture) => record((judged) => ({ slot: slot(judged), last: settled({ next: slot(judged) }) }));

/** 🎛️ The window's gesture slot as an xstate machine, written from the law independently of `driveGesture`: at rest, holding an open gesture, or holding one its tool cannot restore (dropped with zero trace by the next dispatch, which runs from rest); each transition records the dispatch's outcome. */
const slotMachine = setup({ types: { context: {} as SlotContext, events: {} as SlotEvent } }).createMachine({
  id: "gesture-slot",
  initial: "idle",
  context: { slot: undefined, last: undefined },
  states: {
    idle: {
      on: {
        corrupt: {},
        dispatch: [
          { guard: kind("abort"), actions: stay(() => settled({})) },
          { guard: startRefused, actions: stay(() => settled({ refused: law.tool.startRefusal })) },
          { guard: tickRefused, actions: stay(() => settled({ refused: law.tool.sendRefusal })) },
          { guard: (judged) => kind("stream")(judged) && ticked(judged), target: "open", actions: keep(opened) },
          { guard: kind("stream"), actions: stay(() => settled({})) },
          { actions: stay((judged) => settled({ committed: tickOf(judged) })) },
        ],
      },
    },
    open: {
      on: {
        corrupt: { target: "unrestorable", actions: record(({ context }) => ({ slot: { ...slotOf(context), corrupt: true }, last: context.last })) },
        dispatch: [
          { guard: kind("abort"), target: "idle", actions: clear(({ event }) => { const phase = dispatched(event).phase; return settled({ aborted: phase.kind === "abort" ? phase.reason : null, next: "cleared" }); }) },
          { guard: (judged) => moved(judged) && !kind("once")(judged), target: "idle", actions: clear(() => settled({ aborted: "baseMoved", next: "cleared" })) },
          { guard: (judged) => interrupted(judged) && startRefused(judged), actions: stay(() => settled({ refused: law.tool.startRefusal })) },
          { guard: tickRefused, actions: stay(() => settled({ refused: law.tool.sendRefusal })) },
          { guard: moved, target: "idle", actions: clear((judged) => settled({ aborted: "baseMoved", committed: tickOf(judged), next: "cleared" })) },
          { guard: kind("once"), target: "idle", actions: clear((judged) => settled({ aborted: "captureLost", committed: tickOf(judged), next: "cleared" })) },
          { guard: (judged) => otherVerb(judged) && kind("stream")(judged) && ticked(judged), actions: record((judged) => ({ slot: opened(judged), last: settled({ aborted: "captureLost", next: opened(judged) }) })) },
          { guard: (judged) => otherVerb(judged) && kind("stream")(judged), target: "idle", actions: clear(() => settled({ aborted: "captureLost", next: "cleared" })) },
          { guard: otherVerb, target: "idle", actions: clear((judged) => settled({ aborted: "captureLost", committed: tickOf(judged), next: "cleared" })) },
          { guard: (judged) => kind("stream")(judged) && ticked(judged), actions: keep((judged) => ({ ...slotOf(judged.context), ticks: [...slotOf(judged.context).ticks, ...(tickOf(judged) ?? [])] })) },
          { guard: kind("stream"), actions: stay(() => settled({})) },
          { target: "idle", actions: clear((judged) => settled({ committed: [...slotOf(judged.context).ticks, ...(tickOf(judged) ?? [])], next: "cleared" })) },
        ],
      },
    },
    unrestorable: {
      on: {
        corrupt: {},
        dispatch: [
          { guard: kind("abort"), target: "idle", actions: clear(() => settled({ next: "cleared" })) },
          { guard: startRefused, actions: stay(() => settled({ refused: law.tool.startRefusal })) },
          { guard: tickRefused, actions: stay(() => settled({ refused: law.tool.sendRefusal })) },
          { guard: (judged) => kind("stream")(judged) && ticked(judged), target: "open", actions: keep(opened) },
          { guard: kind("stream"), target: "idle", actions: clear(() => settled({ next: "cleared" })) },
          { target: "idle", actions: clear((judged) => settled({ committed: tickOf(judged), next: "cleared" })) },
        ],
      },
    },
  },
});

type SlotSnapshot = SnapshotFrom<typeof slotMachine>;

/** 🗂️ The slot state a persisted gesture puts the window in. */
const slotState = (gesture: Gesture | null | undefined): SlotState => (!gesture ? "idle" : gesture.corrupt ? "unrestorable" : "open");

/** 🎯️ The oracle snapshot holding `gesture`, reached by xstate's own resolution of that state. */
const holding = (gesture: Gesture | null): SlotSnapshot => slotMachine.resolveState({ value: slotState(gesture), context: { slot: gesture ?? undefined, last: undefined } });

/** 🔮️ One dispatch through the oracle. */
function judge(snapshot: SlotSnapshot, dispatch: Dispatch): SlotSnapshot {
  const phase = T.parseGesturePhase(dispatch.phase, dispatch.reason);
  if (!phase) throw new Error(`no gesture phase: ${JSON.stringify(dispatch)}`);
  return transition(slotMachine, snapshot, { type: "dispatch", verb: dispatch.verb, phase, tick: dispatch.tick ?? undefined, base: dispatch.base })[0];
}
//#endregion 🎛️Oracle

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;
  const mutated = (mutate: (copy: any) => void) => {
    const copy = structuredClone(fixture) as any;
    mutate(copy);
    return copy;
  };

  test("the fixture validates against its schema definition", () => {
    const validate = validator("GestureDriveLawFixture");
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("hostile fixture mutations are rejected by the schema", () => {
    const validate = validator("GestureDriveLawFixture");
    expect(validate(mutated((copy) => (copy.rows[0].dispatch.phase = "once")))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[7].dispatch.reason = "sideways")))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[3].persisted.ticks = [])))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[3].persisted.corrupt = false)))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[0].dispatch.tick = "1")))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[0].expected.next = "kept")))).toBe(false);
    expect(validate(mutated((copy) => delete copy.rows[0].expected.refused))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[0].expected.committed = [])))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[0].expected.refused = "toolTransaction.stale")))).toBe(false);
    expect(validate(mutated((copy) => (copy.rows[0].seed = "seed")))).toBe(false);
    expect(validate(mutated((copy) => (copy.schema = "semio.framework.tool-machine.gesture-drive-law.v0")))).toBe(false);
    expect(validator("GesturePhase")({ kind: "abort" })).toBe(false);
    expect(validator("GesturePhase")({ kind: "once", reason: "blur" })).toBe(false);
    expect(validator("GesturePhase")({ kind: "abort", reason: "blur" })).toBe(true);
  });

  test("the phase words and abort reasons mirror the TypeScript twin", () => {
    expect(schema.$defs.GesturePhaseWord.enum).toEqual(["stream", "commit", "abort"]);
    expect(schema.$defs.ToolAbortReason.enum).toEqual([...T.TOOL_ABORT_REASONS]);
    expect(schema.$defs.ToolRefusal.enum).toEqual([...T.TOOL_REFUSALS]);
  });
});

describe("gesture drive law", () => {
  test("the phase arguments parse like the fixture", () => {
    for (const row of law.phases) expect({ args: row.args, phase: T.parseGesturePhase(row.args.phase, row.args.reason) ?? null }).toEqual({ args: row.args, phase: row.phase });
  });

  test("every fixture row drives to its expected outcome", () => {
    for (const row of law.rows) expect({ name: row.name, ...drive(row.persisted, row.dispatch) }).toEqual({ name: row.name, ...row.expected });
  });

  test("the fixture covers every phase from every slot state and every surfaced refusal", () => {
    const covered = new Set(law.rows.map((row) => `${slotState(row.persisted)}/${row.dispatch.phase ?? "once"}`));
    for (const state of ["idle", "open", "unrestorable"]) for (const phase of ["once", "stream", "commit", "abort"]) expect(covered).toContain(`${state}/${phase}`);
    const refusals = new Set(law.rows.flatMap((row) => (row.expected.refused ? [`${slotState(row.persisted)}/${row.expected.refused}`] : [])));
    expect([...refusals].sort()).toEqual([`idle/${law.tool.startRefusal}`, `idle/${law.tool.sendRefusal}`, `open/${law.tool.startRefusal}`, `open/${law.tool.sendRefusal}`, `unrestorable/${law.tool.sendRefusal}`].sort());
    expect(law.tool.startRefusal).not.toBe(law.tool.sendRefusal);
    expect(law.rows.filter((row) => row.expected.refused).every((row) => row.expected.aborted === null && row.expected.committed === null && row.expected.next === "unchanged")).toBe(true);
  });

  test("the xstate slot oracle agrees with every fixture row", () => {
    for (const row of law.rows) {
      const after = judge(holding(row.persisted), row.dispatch);
      const next = row.expected.next === "unchanged" ? row.persisted : row.expected.next === "cleared" ? null : row.expected.next;
      expect({ name: row.name, outcome: after.context.last, state: after.value, slot: after.context.slot ?? null }).toEqual({ name: row.name, outcome: row.expected, state: slotState(next), slot: next });
    }
  });
});

describe("xstate oracle (fast-check)", () => {
  const phase = fc.oneof(
    { weight: 5, arbitrary: fc.constant<T.GesturePhase>({ kind: "stream" }) },
    { weight: 2, arbitrary: fc.constant<T.GesturePhase>({ kind: "commit" }) },
    { weight: 2, arbitrary: fc.constant<T.GesturePhase>({ kind: "once" }) },
    { weight: 2, arbitrary: fc.constantFrom(...T.TOOL_ABORT_REASONS).map((reason): T.GesturePhase => ({ kind: "abort", reason })) },
  );
  const dispatch = fc.record({
    verb: fc.constantFrom("drag", "drag", "drag", "turn", law.tool.refuseStartVerb),
    phase,
    tick: fc.option(fc.integer({ min: -2, max: 9 }), { nil: undefined, freq: 4 }),
    base: fc.constantFrom("r1", "r1", "r1", "r2"),
  });
  const operation = fc.oneof({ weight: 14, arbitrary: dispatch.map((event) => ({ type: "dispatch" as const, ...event })) }, { weight: 3, arbitrary: fc.constant({ type: "corrupt" as const }) });

  test("random dispatch sequences drive the runner and the slot oracle alike", () => {
    const reached = new Set<string>();
    fc.assert(
      fc.property(fc.array(operation, { maxLength: 40 }), (operations) => {
        let persisted = null as Gesture | null;
        let snapshot = initialTransition(slotMachine)[0];
        for (const operation of operations) {
          if (operation.type === "corrupt") {
            if (persisted) persisted = { ...persisted, corrupt: true };
            snapshot = transition(slotMachine, snapshot, operation)[0];
          } else {
            const before = slotState(persisted);
            const outcome = drive(persisted, wire(operation.verb, operation.phase, operation.tick, operation.base));
            persisted = outcome.next === "unchanged" ? persisted : outcome.next === "cleared" ? null : outcome.next;
            snapshot = transition(slotMachine, snapshot, operation)[0];
            expect(snapshot.context.last).toEqual(outcome);
            reached.add(`${before}/${operation.phase.kind}/${outcome.refused ?? outcome.aborted ?? (outcome.committed ? "commit" : "-")}`);
          }
          expect({ state: snapshot.value, slot: snapshot.context.slot ?? null }).toEqual({ state: slotState(persisted), slot: persisted });
        }
      }),
      { numRuns: 600 },
    );
    for (const path of ["idle/stream/-", "idle/once/commit", "open/stream/captureLost", "open/commit/baseMoved", "open/once/baseMoved", "open/once/captureLost", "open/commit/commit", "open/abort/blur", `open/stream/${law.tool.sendRefusal}`, `open/stream/${law.tool.startRefusal}`, `idle/stream/${law.tool.startRefusal}`, "unrestorable/stream/-", "unrestorable/commit/commit", "unrestorable/once/commit", "unrestorable/abort/-"]) expect(reached).toContain(path);
  });
});
