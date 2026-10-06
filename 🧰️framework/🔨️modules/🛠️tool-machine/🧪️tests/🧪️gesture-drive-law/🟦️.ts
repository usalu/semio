/** 🌊️ The shared streamed-gesture runner (`driveGesture`) against `🧫️fixtures/🧫️gesture-drive-law`, authored by an independent Python model of its counting tool (`🧪️s4-tools-a-gesture-drive-law.py` of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); oracles: ajv (schema and hostile mutations), xstate (the window's gesture slot as an idle/open/unrestorable machine, judged on every fixture row too), fast-check (random dispatch sequences with unrestorable gestures, runner vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, initialTransition, setup, transition, type SnapshotFrom } from "xstate";
import * as T from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🧫️gesture-drive-law/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import models from "../../🧪️testing/🔢️counting-gesture/🧬️schema/🔣️.json";

type Gesture = { readonly verb: string; readonly base: string; readonly ticks: readonly number[]; readonly corrupt?: true; readonly press?: string };
type Dispatch = { readonly verb: string; readonly phase?: string; readonly reason?: string; readonly tick: number | null; readonly base: string };
type Outcome = { readonly aborted: T.ToolAbortReason | null; readonly committed: readonly number[] | null; readonly next: "unchanged" | "cleared" | Gesture; readonly refused: T.ToolRefusal | null };
type Row = { readonly name: string; readonly persisted: Gesture | null; readonly dispatch: Dispatch; readonly expected: Outcome };
type HostAbort = { readonly event: T.GestureHostEvent; readonly baseBound: boolean; readonly reason: T.ToolAbortReason | null };
type SlotOutcome = { readonly committed: readonly number[] | null; readonly refused: T.ToolRefusal | null; readonly aborted: readonly T.GestureEnd[]; readonly open: Readonly<Record<string, Gesture>>; readonly closed: Readonly<Record<string, string>> };
type SlotStep = { readonly expected: SlotOutcome } & ({ readonly drive: Dispatch & { readonly window: string; readonly press?: string } } | { readonly host: { readonly window: string; readonly event: T.GestureHostEvent } } | { readonly hostAll: { readonly event: T.GestureHostEvent } } | { readonly retain: readonly string[] } | { readonly corrupt: string });
type SlotScenario = { readonly name: string; readonly steps: readonly SlotStep[] };
type Law = {
  readonly schema: string;
  readonly tool: { readonly refuseStartVerb: string; readonly startRefusal: T.ToolRefusal; readonly resumeRefusal: T.ToolRefusal; readonly sendRefusal: T.ToolRefusal };
  readonly phases: ReadonlyArray<{ readonly args: { readonly phase?: string; readonly reason?: string }; readonly phase: T.GesturePhase | null }>;
  readonly rows: readonly Row[];
  readonly hostEvents: readonly HostAbort[];
  readonly slots: readonly SlotScenario[];
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
    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.press === right.press && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),
    baseRevision: (gesture) => gesture.base,
    press: (gesture) => gesture.press,
    withPress: (gesture, press) => ({ verb: gesture.verb, base: gesture.base, ticks: gesture.ticks, ...(gesture.corrupt ? { corrupt: true as const } : {}), ...(press === undefined ? {} : { press }) }),
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
type SlotEvent = Dispatched | { readonly type: "corrupt" } | { readonly type: "host"; readonly event: T.GestureHostEvent };
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
        host: { actions: stay(() => settled({})) },
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
        host: [
          { guard: ({ context, event }) => event.type === "host" && T.gestureHostAbortReason(event.event, slotOf(context).base !== "") !== undefined, target: "idle", actions: record(({ context, event }) => ({ slot: undefined, last: settled({ aborted: event.type === "host" ? (T.gestureHostAbortReason(event.event, slotOf(context).base !== "") ?? null) : null, next: "cleared" }) })) },
          { actions: stay(() => settled({})) },
        ],
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
        host: [
          { guard: ({ context, event }) => event.type === "host" && T.gestureHostAbortReason(event.event, slotOf(context).base !== "") !== undefined, target: "idle", actions: record(({ context, event }) => ({ slot: undefined, last: settled({ aborted: event.type === "host" ? (T.gestureHostAbortReason(event.event, slotOf(context).base !== "") ?? null) : null, next: "cleared" }) })) },
          { actions: stay(() => settled({})) },
        ],
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
  ajv.addSchema(models);
  const validator = (name: string) => ajv.getSchema(`${name.startsWith("Counting") ? models.$id : schema.$id}#/$defs/${name}`)!;
  const mutated = (mutate: (copy: any) => void) => {
    const copy = structuredClone(fixture) as any;
    mutate(copy);
    return copy;
  };

  test("validates actual counting-gesture input, persisted state and result payloads", () => {
    for (const row of fixture.rows) {
      expect(validator("CountingGestureDispatch")(row.dispatch)).toBe(true);
      if (row.persisted) expect(validator("CountingGestureState")(row.persisted)).toBe(true);
      expect(validator("CountingGestureOutcome")(row.expected)).toBe(true);
    }
  });

  test("rejects hostile domain payload mutations through the schema", () => {
    expect(validator("CountingGestureDispatch")(mutated(copy => copy.rows[0].dispatch.phase = "once").rows[0].dispatch)).toBe(false);
    expect(validator("CountingGestureDispatch")(mutated(copy => copy.rows[7].dispatch.reason = "sideways").rows[7].dispatch)).toBe(false);
    expect(validator("CountingGestureState")(mutated(copy => copy.rows[3].persisted.ticks = []).rows[3].persisted)).toBe(false);
    expect(validator("CountingGestureState")(mutated(copy => copy.rows[3].persisted.corrupt = false).rows[3].persisted)).toBe(false);
    expect(validator("CountingGestureDispatch")(mutated(copy => copy.rows[0].dispatch.tick = "1").rows[0].dispatch)).toBe(false);
    expect(validator("CountingGestureOutcome")(mutated(copy => copy.rows[0].expected.next = "kept").rows[0].expected)).toBe(false);
    expect(validator("CountingGestureOutcome")(mutated(copy => delete copy.rows[0].expected.refused).rows[0].expected)).toBe(false);
    expect(validator("CountingGestureOutcome")(mutated(copy => copy.rows[0].expected.committed = []).rows[0].expected)).toBe(false);
    expect(validator("CountingGestureOutcome")(mutated(copy => copy.rows[0].expected.refused = "toolTransaction.stale").rows[0].expected)).toBe(false);
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

//#region 🗂️Ledger
/** 👣️ One slot step through the ledger (`GestureLedger`), reported in the fixture's outcome shape. */
function ledgerStep(ledger: T.GestureLedger<Gesture, number, number>, step: SlotStep): SlotOutcome {
  let committed: readonly number[] | null = null;
  let refused: T.ToolRefusal | null = null;
  let aborted: T.GestureEnd[] = [];
  if ("drive" in step) {
    const phase = T.parseGesturePhase(step.drive.phase, step.drive.reason);
    if (!phase) throw new Error(`no gesture phase: ${JSON.stringify(step.drive)}`);
    const result = ledger.drive(step.drive.window, step.drive.press, step.drive.verb, phase, step.drive.tick ?? undefined, "seed", step.drive.base);
    if (result.ok) committed = result.committed?.mutations ?? null;
    else refused = result.refusal;
  } else if ("host" in step) {
    const ended = ledger.hostEvent(step.host.window, step.host.event);
    aborted = ended ? [ended] : [];
  } else if ("hostAll" in step) aborted = ledger.hostEventAll(step.hostAll.event);
  else if ("retain" in step) aborted = ledger.retainWindows((window) => step.retain.includes(window));
  else ledger.settle(step.corrupt, { ...ledger.open(step.corrupt)!, corrupt: true });
  return { committed, refused, aborted, open: Object.fromEntries(ledger.windows().map((window) => [window, ledger.open(window)!])), closed: ledger.closedPresses() };
}

describe("gesture ledger law (design §22.10)", () => {
  test("every host fact ends an open gesture with the fixture's reason", () => {
    expect(schema.$defs.GestureHostEvent.enum).toEqual([...T.GESTURE_HOST_EVENTS]);
    expect(law.hostEvents.map((row) => `${row.event}/${row.baseBound}`).sort()).toEqual(T.GESTURE_HOST_EVENTS.flatMap((event) => [`${event}/true`, `${event}/false`]).sort());
    for (const row of law.hostEvents) expect({ ...row, reason: T.gestureHostAbortReason(row.event, row.baseBound) ?? null }).toEqual(row);
  });

  test("every slot scenario steps the ledger to its expected outcome", () => {
    for (const scenario of law.slots) {
      const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));
      scenario.steps.forEach((step, index) => expect({ name: `${scenario.name}#${index}`, ...ledgerStep(ledger, step) }).toEqual({ name: `${scenario.name}#${index}`, ...step.expected }));
    }
  });

  test("the scenarios cover every host fact on an open gesture, the kept one and a frozen multi-window edit", () => {
    const facts = new Set(law.slots.flatMap((scenario) => scenario.steps.flatMap((step) => ("host" in step && step.expected.aborted.length > 0 ? [step.host.event] : []))));
    expect([...facts].sort()).toEqual([...T.GESTURE_HOST_EVENTS].sort());
    expect(law.slots.some((scenario) => scenario.steps.some((step) => "host" in step && step.host.event === "baseMoved" && step.expected.aborted.length === 0 && Object.keys(step.expected.open).length > 0))).toBe(true);
    expect(law.slots.some((scenario) => scenario.steps.some((step) => "hostAll" in step && step.hostAll.event === "timeTravelFrozen" && step.expected.aborted.length > 1 && step.expected.aborted.every((end) => end.reason === "frozen") && Object.keys(step.expected.open).length === 0))).toBe(true);
  });

  test("the xstate slot oracle judges every window of every scenario without a named press like the ledger", () => {
    for (const scenario of law.slots.filter((scenario) => scenario.steps.every((step) => !("drive" in step) || step.drive.press === undefined))) {
      const windows = new Map<string, SlotSnapshot>();
      const slot = (window: string) => windows.get(window) ?? initialTransition(slotMachine)[0];
      const ended = (window: string, before: SlotSnapshot, after: SlotSnapshot): T.GestureEnd[] => (before.context.slot && !after.context.slot && after.context.last?.aborted ? [{ window, reason: after.context.last.aborted }] : []);
      scenario.steps.forEach((step, index) => {
        let aborted: T.GestureEnd[] = [];
        const host = (window: string, event: T.GestureHostEvent) => {
          const before = slot(window);
          const after = transition(slotMachine, before, { type: "host", event })[0];
          windows.set(window, after);
          aborted = [...aborted, ...ended(window, before, after)];
        };
        if ("drive" in step) windows.set(step.drive.window, judge(slot(step.drive.window), step.drive));
        else if ("host" in step) host(step.host.window, step.host.event);
        else if ("hostAll" in step) {
          for (const window of [...windows.keys()].sort()) host(window, step.hostAll.event);
        } else if ("retain" in step) {
          for (const window of [...windows.keys()].sort().filter((window) => !step.retain.includes(window))) host(window, "retiring");
        } else windows.set(step.corrupt, transition(slotMachine, slot(step.corrupt), { type: "corrupt" })[0]);
        const open = Object.fromEntries([...windows.entries()].filter(([, snapshot]) => snapshot.context.slot).map(([window, snapshot]) => [window, snapshot.context.slot!]).sort(([left], [right]) => (left < right ? -1 : 1)));
        expect({ name: `${scenario.name}#${index}`, aborted, open }).toEqual({ name: `${scenario.name}#${index}`, aborted: [...step.expected.aborted], open: step.expected.open });
      });
    }
  });
});
//#endregion 🗂️Ledger

describe("gesture press identity law", () => {
  const named = law.slots.flatMap((scenario) => scenario.steps.flatMap((step, index) => ("drive" in step && step.drive.press !== undefined ? [{ scenario, step, before: index === 0 ? undefined : scenario.steps[index - 1]!.expected }] : [])));

  test("the scenarios cover a dropped late dispatch, an interrupting press, a refused one and a forgotten one", () => {
    const dropped = named.filter(({ step, before }) => before?.closed[step.drive.window] === step.drive.press);
    expect(dropped.length).toBeGreaterThan(3);
    expect(dropped.every(({ step, before }) => step.expected.committed === null && step.expected.refused === null && JSON.stringify(step.expected.open) === JSON.stringify(before!.open))).toBe(true);
    expect(dropped.some(({ step }) => step.drive.phase === "commit" && step.drive.tick !== null)).toBe(true);
    const interrupting = named.filter(({ step, before }) => before?.open[step.drive.window]?.press !== undefined && before.open[step.drive.window]!.press !== step.drive.press && before.closed[step.drive.window] !== step.drive.press);
    expect(interrupting.some(({ step, before }) => step.expected.refused === null && step.expected.closed[step.drive.window] === before!.open[step.drive.window]!.press && step.expected.open[step.drive.window]?.press === step.drive.press)).toBe(true);
    expect(interrupting.some(({ step, before }) => step.expected.refused !== null && JSON.stringify(step.expected.open) === JSON.stringify(before!.open) && JSON.stringify(step.expected.closed) === JSON.stringify(before!.closed))).toBe(true);
    expect(law.slots.some((scenario) => scenario.steps.some((step, index) => "retain" in step && index > 0 && Object.keys(scenario.steps[index - 1]!.expected.closed).length > 0 && Object.keys(step.expected.closed).length === 0))).toBe(true);
  });

  test("random named sequences keep the press invariants (fast-check)", () => {
    const phase = fc.oneof({ weight: 5, arbitrary: fc.constant<T.GesturePhase>({ kind: "stream" }) }, { weight: 2, arbitrary: fc.constant<T.GesturePhase>({ kind: "commit" }) }, { weight: 1, arbitrary: fc.constant<T.GesturePhase>({ kind: "once" }) }, { weight: 1, arbitrary: fc.constant<T.GesturePhase>({ kind: "abort", reason: "blur" }) });
    const dispatch = fc.record({ type: fc.constant("dispatch" as const), press: fc.constantFrom(undefined, "p1", "p1", "p2", "p3"), verb: fc.constantFrom("drag", "drag", "turn", law.tool.refuseStartVerb), phase, tick: fc.option(fc.integer({ min: -1, max: 9 }), { nil: undefined, freq: 5 }), base: fc.constantFrom("r1", "r1", "r1", "r2") });
    const operation = fc.oneof({ weight: 8, arbitrary: dispatch }, { weight: 1, arbitrary: fc.constantFrom(...T.GESTURE_HOST_EVENTS).map((event) => ({ type: "host" as const, event })) });
    const seen = new Set<string>();
    fc.assert(
      fc.property(fc.array(operation, { maxLength: 40 }), (operations) => {
        const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));
        for (const step of operations) {
          const before = { open: ledger.open("w"), closed: ledger.closed("w") };
          if (step.type === "host") {
            if (ledger.hostEvent("w", step.event) && before.open?.press !== undefined) expect(ledger.closed("w")).toBe(before.open.press);
            continue;
          }
          const result = ledger.drive("w", step.press, step.verb, step.phase, step.tick, "seed", step.base);
          const after = { open: ledger.open("w"), closed: ledger.closed("w") };
          if (!result.ok) {
            expect(after).toEqual(before);
            seen.add("refused");
          } else if (step.press !== undefined && before.closed === step.press) {
            expect({ ...after, committed: result.committed }).toEqual({ ...before, committed: undefined });
            seen.add("dropped");
          } else {
            const terminal = step.press !== undefined && step.phase.kind !== "stream";
            if (terminal) expect(after.closed).toBe(step.press);
            if (after.open?.press !== undefined) expect([before.open?.press, step.press]).toContain(after.open.press);
            if (before.open?.press !== undefined && after.open?.press !== before.open.press && !terminal) expect(after.closed).toBe(before.open.press);
            if (before.open?.press !== undefined && step.press !== undefined && before.open.press !== step.press) seen.add("interrupted");
          }
        }
      }),
      { numRuns: 400 },
    );
    for (const path of ["refused", "dropped", "interrupted"]) expect(seen).toContain(path);
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
  const operation = fc.oneof(
    { weight: 14, arbitrary: dispatch.map((event) => ({ type: "dispatch" as const, ...event })) },
    { weight: 3, arbitrary: fc.constant({ type: "corrupt" as const }) },
    { weight: 3, arbitrary: fc.constantFrom(...T.GESTURE_HOST_EVENTS).map((event) => ({ type: "host" as const, event })) },
  );

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
          } else if (operation.type === "host") {
            const before = slotState(persisted);
            const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));
            if (persisted) ledger.settle("w", persisted);
            const ended = ledger.hostEvent("w", operation.event);
            persisted = ledger.open("w") ?? null;
            snapshot = transition(slotMachine, snapshot, operation)[0];
            expect(snapshot.context.last?.aborted ?? null).toEqual(ended?.reason ?? null);
            reached.add(`${before}/host/${ended?.reason ?? "-"}`);
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
    for (const path of ["idle/stream/-", "idle/once/commit", "open/stream/captureLost", "open/commit/baseMoved", "open/once/baseMoved", "open/once/captureLost", "open/commit/commit", "open/abort/blur", `open/stream/${law.tool.sendRefusal}`, `open/stream/${law.tool.startRefusal}`, `idle/stream/${law.tool.startRefusal}`, "unrestorable/stream/-", "unrestorable/commit/commit", "unrestorable/once/commit", "unrestorable/abort/-", "open/host/frozen", "open/host/blur", "open/host/retired", "open/host/baseMoved", "idle/host/-"]) expect(reached).toContain(path);
  });
});
