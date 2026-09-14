/** 🧩️ TS reducer, ring, store and codecs against the shared fixtures; oracles: ajv (schema), xstate (machine built from the matrix), fast-check (random event sequences, reducer vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, createMachine, initialTransition, transition, type AnyMachineSnapshot } from "xstate";
import lifecycle from "../../🧫️fixtures/⚖️lifecycle-law.json";
import ticks from "../../🧫️fixtures/🎞️ticks.json";
import tracePages from "../../🧫️fixtures/📼️trace-pages.json";
import schema from "../../🧬️schema/🔣️.json";
import * as M from "../../🟦️.ts";

const law = lifecycle as any;
const pagesFixture = tracePages as any;
const ticksFixture = ticks as any;
const hex = M.toolRunBytesToHex;

function matrixEvent(key: M.ToolRunEventKey, run: bigint, generation: number): M.ToolRunEvent {
  switch (key) {
    case "start":
      return { type: "start", run: run + 1n };
    case "settingsChanged":
    case "baseChanged":
    case "dismiss":
      return { type: key, run };
    case "closed":
      return { type: "closed" };
    case "abort":
    case "abortWhilePublishing":
      return { type: "abort", run, generation, publishing: key === "abortWhilePublishing" };
    default:
      return { type: key, run, generation };
  }
}

function stateName(slot: M.ToolRunSlot | null): string {
  return slot === null ? "none" : slot.state;
}

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

  test("every fixture validates against its schema definition", () => {
    for (const [name, value] of [["LifecycleLawFixture", law], ["TracePageFixture", pagesFixture], ["TickFixture", ticksFixture]] as const) {
      const validate = validator(name);
      expect(validate(value), `${name}: ${JSON.stringify(validate.errors)}`).toBe(true);
    }
  });

  test("hostile fixture mutations are rejected by the schema", () => {
    const lifecycleValidate = validator("LifecycleLawFixture");
    expect(lifecycleValidate({ ...law, extra: true })).toBe(false);
    expect(lifecycleValidate({ ...law, matrix: law.matrix.slice(1) })).toBe(false);
    expect(lifecycleValidate({ ...law, definition: { ...law.definition, reasons: [{ ...law.definition.reasons[0], code: 65280 }] } })).toBe(false);
    expect(lifecycleValidate({ ...law, limits: { ...law.limits, stepRingCapacity: 12 } })).toBe(false);
    const pageValidate = validator("ToolRunTracePage");
    const page = pagesFixture.pages[0].page;
    expect(pageValidate(page)).toBe(true);
    expect(pageValidate({ ...page, ops: [{ op: "upsert", key: 1, verdict: "rejected", reason: 0, subject: { kind: "entity", entity: 1 } }] })).toBe(false);
    expect(pageValidate({ ...page, identity: { ...page.identity, baseRevision: "00" } })).toBe(false);
    const tickValidate = validator("ToolRunTick");
    expect(tickValidate({ ...ticksFixture.ticks[0].tick, steps: [{ ...ticksFixture.ticks[0].tick.steps[0], args: [{ unsigned: 1 }, { unsigned: 2 }, { unsigned: 3 }, { unsigned: 4 }, { unsigned: 5 }] }] })).toBe(false);
  });
});

describe("lifecycle law", () => {
  test("states and event keys mirror the fixture", () => {
    expect([...M.TOOL_RUN_STATES]).toEqual(law.states);
    expect([...M.TOOL_RUN_EVENT_KEYS]).toEqual(law.eventKeys);
    expect(new Set(law.matrix.map((row: any) => `${row.from}×${row.event}`)).size).toBe(180);
  });

  test("reducer matches every matrix row", () => {
    for (const row of law.matrix) {
      const slot: M.ToolRunSlot | null = row.from === "none" ? null : { run: 7n, generation: 2, state: row.from };
      const result = M.ToolRunMachine.apply(slot, matrixEvent(row.event, 7n, 2));
      const label = `${row.from} × ${row.event}`;
      if ("rejection" in row) {
        expect(result.ok ? "accepted" : result.rejection, label).toBe(row.rejection);
        continue;
      }
      expect(result.ok, label).toBe(true);
      if (!result.ok) continue;
      expect(stateName(result.transition.slot), label).toBe(row.to);
      expect(result.transition.effect, label).toBe(row.effect);
      expect(M.toolRunEffectCommits(result.transition.effect), label).toBe(row.commits);
      if (result.transition.slot !== null) {
        expect(result.transition.slot.generation, label).toBe(row.generation === "reset" ? 0 : row.generation === "increment" ? 3 : 2);
        expect(result.transition.slot.run, label).toBe(row.event === "start" ? 8n : 7n);
      }
    }
  });

  test("reducer matches every case", () => {
    for (const row of law.cases) {
      const result = M.ToolRunMachine.apply(M.toolRunSlotFromJson(row.slot), M.toolRunEventFromJson(row.event));
      if ("rejection" in row.expect) expect(result.ok ? "accepted" : result.rejection, row.name).toBe(row.expect.rejection);
      else {
        expect(result.ok, row.name).toBe(true);
        if (result.ok) expect({ slot: M.toolRunSlotToJson(result.transition.slot), effect: result.transition.effect }, row.name).toEqual(row.expect.transition);
      }
    }
  });

  test("reducer replays every scenario with its commit count", () => {
    for (const scenario of law.scenarios) {
      let slot: M.ToolRunSlot | null = null;
      let commits = 0;
      const observed: string[] = [];
      for (const json of scenario.events) {
        const result = M.ToolRunMachine.apply(slot, M.toolRunEventFromJson(json));
        if (result.ok) {
          commits += Number(M.toolRunEffectCommits(result.transition.effect));
          slot = result.transition.slot;
          observed.push(stateName(slot));
        } else observed.push(result.rejection);
      }
      expect(observed, scenario.name).toEqual(scenario.states);
      expect(commits, scenario.name).toBe(scenario.commits);
    }
  });

  test("actions, chords, labels, templates and limits mirror the fixture", () => {
    expect(M.TOOL_RUN_ACTIONS.map((action) => ({ id: action.id, chord: action.chord, label: action.label, args: action.args }))).toEqual(law.actions.map((row: any) => ({ id: row.id, chord: row.chord, label: row.label, args: row.args })));
    for (const row of law.actions) {
      for (const state of ["none", ...M.TOOL_RUN_STATES]) {
        const legal = M.isToolRunActionLegal(row.id, state === "none" ? null : (state as M.ToolRunState));
        expect(legal, `${row.id} in ${state}`).toBe(row.legalIn.includes(state));
      }
    }
    expect(Object.entries(M.TOOL_RUN_LABELS).map(([key, text]) => ({ key, ...text }))).toEqual(law.labels);
    for (const state of M.TOOL_RUN_STATES) expect(M.TOOL_RUN_LABELS[M.toolRunStateLabel(state)]).toBeDefined();
    for (const row of law.reservedReasons) expect(M.toolRunReasonLabel(row.code)).toBe(row.label);
    expect(M.toolRunReasonLabel(M.TOOL_RUN_RESERVED_REASON_FLOOR - 1)).toBeUndefined();
    for (const row of law.templates) expect(M.toolRunFormat(row.template, (name) => row.values[name])).toBe(row.expected);
    expect(law.limits).toEqual({
      stepRingCapacity: M.TOOL_RUN_STEP_RING_CAPACITY,
      stepArgsMax: M.TOOL_RUN_STEP_ARGS_MAX,
      countersMax: M.TOOL_RUN_COUNTERS_MAX,
      provisionalOpsMax: M.TOOL_RUN_PROVISIONAL_OPS_MAX,
      traceResidentRecords: M.TOOL_RUN_TRACE_RESIDENT_RECORDS,
      tracePageOpsMax: M.TOOL_RUN_TRACE_PAGE_OPS_MAX,
      tracePageBytesMax: M.TOOL_RUN_TRACE_PAGE_BYTES_MAX,
      tickBytesMax: M.TOOL_RUN_TICK_BYTES_MAX,
      traceLogCompactFloor: M.TOOL_RUN_TRACE_LOG_COMPACT_FLOOR,
      statusAnnounceIntervalMs: M.TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS,
      reservedReasonFloor: M.TOOL_RUN_RESERVED_REASON_FLOOR,
    });
  });
});

describe("state machine oracle (xstate + fast-check)", () => {
  type Context = { run: bigint; generation: number };
  type OracleEvent = { type: M.ToolRunEventKey; run?: bigint; generation?: number };
  const states: Record<string, { on: Record<string, unknown> }> = {};
  for (const from of ["none", ...law.states]) states[from] = { on: {} };
  for (const row of law.matrix.filter((row: any) => "to" in row)) {
    const guard = ({ context, event }: { context: Context; event: OracleEvent }) => {
      if (row.event === "closed") return true;
      if (row.event === "start") return row.from === "none" || event.run! > context.run;
      return event.run === context.run && (event.generation === undefined || event.generation === context.generation);
    };
    const assignContext = ({ context, event }: { context: Context; event: OracleEvent }): Context => (row.event === "start" ? { run: event.run!, generation: 0 } : row.generation === "increment" ? { run: context.run, generation: (context.generation + 1) >>> 0 } : context);
    states[row.from]!.on[row.event] = { target: `#toolRun.${row.to}`, guard, actions: assign(assignContext as any) };
  }
  const machine = createMachine({ id: "toolRun", initial: "none", context: { run: 0n, generation: 0 } as Context, states } as any);

  const oracleEvent = (event: M.ToolRunEvent): OracleEvent => ({ type: M.toolRunEventKey(event), ...("run" in event ? { run: event.run } : {}), ...("generation" in event ? { generation: event.generation } : {}) });

  test("the xstate machine built from the matrix agrees with the reducer on every row", () => {
    const [initial] = initialTransition(machine);
    expect(initial.value).toBe("none");
    for (const row of law.matrix) {
      const slot: M.ToolRunSlot | null = row.from === "none" ? null : { run: 7n, generation: 2, state: row.from };
      const snapshot = machine.resolveState({ value: row.from, context: { run: 7n, generation: 2 } } as any);
      const event = matrixEvent(row.event, 7n, 2);
      const result = M.ToolRunMachine.apply(slot, event);
      expect((snapshot as AnyMachineSnapshot).can(oracleEvent(event) as any), `${row.from} × ${row.event}`).toBe(result.ok);
      if (!result.ok) continue;
      const [next] = transition(machine, snapshot, oracleEvent(event) as any);
      expect(next.value, `${row.from} × ${row.event}`).toBe(stateName(result.transition.slot));
      if (result.transition.slot !== null) expect(next.context, `${row.from} × ${row.event}`).toEqual({ run: result.transition.slot.run, generation: result.transition.slot.generation });
    }
  });

  test("random event sequences: reducer and xstate agree on acceptance, state, run and generation", () => {
    const weighted = <T extends string>(same: T, ...others: T[]) => fc.oneof({ arbitrary: fc.constant(same), weight: 8 }, ...others.map((other) => ({ arbitrary: fc.constant(other), weight: 1 })));
    const choice = fc.record({ key: fc.constantFrom(...M.TOOL_RUN_EVENT_KEYS), run: weighted("same", "next", "older"), generation: weighted("same", "next", "zero") });
    fc.assert(
      fc.property(fc.array(choice, { minLength: 20, maxLength: 160 }), (choices) => {
        let slot: M.ToolRunSlot | null = null;
        let [snapshot] = initialTransition(machine);
        for (const pick of choices) {
          const baseRun = slot?.run ?? 1n;
          const run = pick.run === "same" ? baseRun : pick.run === "next" ? baseRun + 1n : baseRun > 0n ? baseRun - 1n : 0n;
          const generation = pick.generation === "same" ? (slot?.generation ?? 0) : pick.generation === "next" ? (slot?.generation ?? 0) + 1 : 0;
          const event = matrixEvent(pick.key, run - (pick.key === "start" ? 1n : 0n), generation);
          const oracle = oracleEvent(event);
          const result = M.ToolRunMachine.apply(slot, event);
          const accepted = (snapshot as AnyMachineSnapshot).can(oracle as any);
          if (result.ok !== accepted) return false;
          if (!result.ok) continue;
          [snapshot] = transition(machine, snapshot, oracle as any);
          slot = result.transition.slot;
          if (snapshot.value !== stateName(slot)) return false;
          if (slot !== null && (snapshot.context.run !== slot.run || snapshot.context.generation !== slot.generation)) return false;
        }
        return true;
      }),
      { numRuns: 3_000 },
    );
  });
});

describe("trace codec and store", () => {
  test("pages and deltas encode to and decode from the fixture bytes", () => {
    for (const row of pagesFixture.pages) {
      const page = M.toolRunTracePageFromJson(row.page);
      expect(hex(M.encodeToolRunTracePage(page)), row.name).toBe(row.hex);
      expect(M.toolRunTracePageToJson(M.decodeToolRunTracePage(M.toolRunHexToBytes(row.hex))), row.name).toEqual(row.page);
    }
    for (const row of pagesFixture.deltas) {
      const delta = M.toolRunTraceDeltaFromJson(row.delta);
      expect(hex(M.encodeToolRunTraceDelta(delta)), row.name).toBe(row.hex);
      expect(M.toolRunTraceDeltaToJson(M.decodeToolRunTraceDelta(M.toolRunHexToBytes(row.hex))), row.name).toEqual(row.delta);
    }
  });

  test("malformed wire values are rejected", () => {
    for (const row of pagesFixture.malformed) {
      const decode = row.target === "page" ? M.decodeToolRunTracePage : row.target === "delta" ? M.decodeToolRunTraceDelta : M.decodeToolRunTick;
      expect(() => decode(M.toolRunHexToBytes(row.hex)), row.name).toThrow(M.ToolRunCodecError);
    }
  });

  test("full u64 range and the op cap", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 0xffff_ffff, run: 0xffff_ffff_ffff_ffffn }, generation: 1, baseRevision: new Uint8Array(32).fill(0xab) };
    const page: M.ToolRunTracePage = { identity, page: 0xffff_ffff, ops: [{ op: "upsert", key: 0xffff_ffff_ffff_ffffn, verdict: "warning", reason: 0xffff, subject: { kind: "entity", entity: 0xffff_ffff_ffff_ffffn } }] };
    expect(M.decodeToolRunTracePage(M.encodeToolRunTracePage(page))).toEqual(page);
    expect(() => M.encodeToolRunTracePage({ identity, page: 0, ops: Array.from({ length: M.TOOL_RUN_TRACE_PAGE_OPS_MAX + 1 }, () => ({ op: "clear" }) as const) })).toThrow(M.ToolRunCodecError);
    expect(M.toolRunGroupId(identity.id)).toBe("toolRun:18446744073709551615");
  });

  const storeFor = (row: any) => {
    const store = new M.ToolRunTraceStore(M.toolRunIdentityFromJson(row.identity), row.capacity, row.compactFloor);
    const applied = row.pages.map((ops: any[]) => store.applyOps(ops.map(M.toolRunTraceOpFromJson)));
    return { store, applied };
  };

  test("residency matches the fixture", () => {
    for (const row of pagesFixture.residency) {
      const { store, applied } = storeFor(row);
      expect(applied, row.name).toEqual(row.applied);
      const resident = [...store.records()].sort((a, b) => (a[0] < b[0] ? -1 : 1)).map(([key, record]) => ({ key: Number(key), verdict: record.verdict, reason: record.reason }));
      expect(resident, row.name).toEqual(row.resident);
      expect(store.logBase, row.name).toBe(row.logBase);
      const logPages = Array.from({ length: store.nextPage - store.logBase }, (_, index) => store.logPage(store.logBase + index)!.map(M.toolRunTraceOpToJson));
      expect(logPages, row.name).toEqual(row.logPages);
    }
  });

  test("delivery matches the fixture", () => {
    for (const row of pagesFixture.delivery) {
      const { store } = storeFor(pagesFixture.residency.find((candidate: any) => candidate.name === row.residency));
      store.rebind({ ...store.identity, generation: row.generation });
      const cursor = row.cursor === null ? null : { run: BigInt(row.cursor.run), generation: row.cursor.generation, page: row.cursor.page };
      const delta = store.deltaAfter(cursor, row.byteBudget);
      expect({ clear: delta.clear, next: delta.next, pages: delta.pages.map((page) => page.page) }, row.name).toEqual(row.expect);
      expect(M.decodeToolRunTraceDelta(M.encodeToolRunTraceDelta(delta)), row.name).toEqual(delta);
    }
  });

  test("stale pages are rejected and a new run resets the store", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 1, run: 5n }, generation: 0, baseRevision: new Uint8Array(32) };
    const store = new M.ToolRunTraceStore(identity, 4, 4);
    const ops: M.ToolRunTraceOp[] = [{ op: "upsert", key: 1n, verdict: "testing", reason: 0, subject: { kind: "entity", entity: 1n } }];
    expect(store.applyPage({ identity: { ...identity, generation: 1 }, page: 0, ops })).toBe("toolRun.stale");
    expect(store.applyPage({ identity, page: 9, ops })).toEqual({ logged: 1, evicted: 0, overflowed: 0, compacted: false });
    store.rebind({ ...identity, id: { appInstanceId: 1, run: 6n } });
    expect([store.size, store.logBase, store.nextPage]).toEqual([0, 0, 0]);
  });
});

describe("tick codec and step ring", () => {
  test("ticks encode to and decode from the fixture bytes", () => {
    for (const row of ticksFixture.ticks) {
      const tick = M.toolRunTickFromJson(row.tick);
      expect(hex(M.encodeToolRunTick(tick)), row.name).toBe(row.hex);
      expect(M.toolRunTickToJson(M.decodeToolRunTick(M.toolRunHexToBytes(row.hex))), row.name).toEqual(row.tick);
    }
  });

  test("tick byte cap", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 0, run: 0n }, generation: 0, baseRevision: new Uint8Array(32) };
    expect(() => M.encodeToolRunTick({ identity, sequence: 0n, steps: [], trace: [], appendOps: [new Uint8Array(M.TOOL_RUN_TICK_BYTES_MAX)], appendEntities: [] })).toThrow(M.ToolRunCodecError);
  });

  test("step ring matches the fixture", () => {
    for (const row of ticksFixture.stepRing) {
      const ring = new M.ToolRunStepRing();
      for (const push of row.push) {
        if ("repeatSequence" in push) {
          for (let sequence = push.repeatSequence.from; sequence < push.repeatSequence.from + push.repeatSequence.count; sequence += 1) ring.push({ sequence: BigInt(sequence), kind: push.repeatSequence.kind, stage: 0, reason: sequence & 0xffff, repeat: 1, args: [] });
        } else ring.push(M.toolRunStepFromJson(push));
      }
      expect(ring.length, row.name).toBe(row.expect.length);
      expect(ring.oldest() === undefined ? null : M.toolRunStepToJson(ring.oldest()!), row.name).toEqual(row.expect.first);
      expect(ring.newest() === undefined ? null : M.toolRunStepToJson(ring.newest()!), row.name).toEqual(row.expect.last);
    }
  });
});

describe("start admission", () => {
  test("every admission row follows the lane law", () => {
    for (const row of law.admission) {
      const live = row.live.map((entry: any) => [entry.lane, entry.state] as const);
      const result = M.toolRunAdmit(row.lane, live);
      expect(result, row.name).toEqual(row.rejection === undefined ? { ok: true, replaces: row.replaces } : { ok: false, rejection: row.rejection });
    }
  });

  test("fast-check: a start is busy exactly when a non-terminal run on its lane exists, and never admits a second non-terminal mutating run", () => {
    const lane = fc.record({ mutating: fc.boolean(), toolId: fc.constantFrom("fill", "preview"), windowId: fc.constantFrom("a", "b") });
    fc.assert(
      fc.property(fc.array(fc.tuple(lane, fc.constantFrom(...M.TOOL_RUN_STATES)), { maxLength: 6 }), lane, (live, next) => {
        const result = M.toolRunAdmit(next, live);
        const blocking = live.some(([other, state]) => !M.isToolRunTerminal(state) && (next.mutating && other.mutating ? true : !next.mutating && !other.mutating && other.toolId === next.toolId && other.windowId === next.windowId));
        expect(result.ok).toBe(!blocking);
        if (result.ok && next.mutating) expect(live.filter(([other, state]) => other.mutating && !M.isToolRunTerminal(state)).length).toBe(0);
      }),
      { numRuns: 2000 },
    );
  });
});

describe("tool run panel groups", () => {
  test("group keys and reveals follow the fixture", () => {
    const cases = law.panelGroups;
    expect(M.TOOL_RUN_PANEL_ID).toBe(cases.rootId);
    for (const row of cases.keys) {
      expect(M.toolRunPanelGroupRun(row.key), row.key).toBe(row.run === null ? null : BigInt(row.run));
      if (row.run !== null) expect(M.toolRunPanelGroupId(BigInt(row.run))).toBe(row.key);
    }
    for (const row of cases.reveals) {
      const { current, added } = M.toolRunPanelNewRuns(new Set(row.previous.map(BigInt)), row.keys);
      expect([[...current].sort(), added], row.name).toEqual([row.current.map(BigInt), row.added.map(BigInt)]);
    }
  });
});
