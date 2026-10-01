/** ⏪️ TS reducer against the lifecycle-law fixture; oracles: ajv (schema), xstate (statechart built from the law matrix with an independent context model), fast-check (random event sequences, reducer vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, createMachine, initialTransition, transition, type AnyMachineSnapshot } from "xstate";
import lifecycle from "../../🧫️fixtures/🧫️lifecycle-law/🔣️.json";
import replayReportSchema from "../../../📡️replication/⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json";
import { replayReportBlocksFinalize, type InputReplacement } from "@semio-tech/framework-replication";
import schema from "../../🧬️schema/🔣️.json";
import * as M from "../../🟦️.ts";

const law = lifecycle as any;
const contextOf = (name: string): M.TimeTravelSession => M.timeTravelSessionFromJson(law.contexts[name]);
const eventOf = (key: string): M.TimeTravelEvent => M.timeTravelEventFromJson(law.events[key]);
const rowEvent = (row: any): M.TimeTravelEvent => M.timeTravelEventFromJson(row.input ?? law.events[row.event]);
const kinds = (effects: readonly M.TimeTravelEffect[]): string[] => effects.map((effect) => effect.type);

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(replayReportSchema);
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

  test("the fixture validates against LifecycleLawFixture", () => {
    const validate = validator("LifecycleLawFixture");
    expect(validate(law), JSON.stringify(validate.errors)).toBe(true);
  });

  test("every reducer output re-serializes to schema-valid sessions and effects", () => {
    const session = validator("TimeTravelSession");
    const effect = validator("TimeTravelEffect");
    for (const row of law.matrix) {
      const result = M.applyTimeTravel(contextOf(row.context), eventOf(row.event));
      if (!result.ok) continue;
      expect(session(M.timeTravelSessionToJson(result.session)), JSON.stringify(session.errors)).toBe(true);
      for (const value of result.effects) expect(effect(M.timeTravelEffectToJson(value)), JSON.stringify(effect.errors)).toBe(true);
    }
  });

  test("hostile fixture mutations are rejected by the schema", () => {
    const validate = validator("LifecycleLawFixture");
    expect(validate({ ...law, extra: true })).toBe(false);
    expect(validate({ ...law, matrix: law.matrix.slice(1) })).toBe(false);
    expect(validate({ ...law, labels: [{ ...law.labels[0], de: "" }, ...law.labels.slice(1)] })).toBe(false);
    expect(validate({ ...law, events: { ...law.events, exit: { type: "exit", generation: 1 } } })).toBe(false);
    const session = validator("TimeTravelSession");
    const context = law.contexts["reviewing.ready"];
    expect(session(context)).toBe(true);
    expect(session({ ...context, stage: "paused" })).toBe(false);
    expect(session({ ...context, base: { ...context.base, contentRevision: "00" } })).toBe(false);
    expect(session({ ...context, accepted: [{ ...context.accepted[0], replacement: { kind: "input", schema: "s", payload: [256] } }] })).toBe(false);
    expect(validator("TimeTravelEffect")({ type: "startReplay", drafts: [], from: "a" })).toBe(false);
    expect(session({ ...context, fault: "" })).toBe(false);
    expect(session({ ...context, fault: "vcs\u0085rejected" })).toBe(false);
  });

  test("a schema-invalid event is never well formed, and the byte ceiling is the reducer's own law", () => {
    const strict = validator("TimeTravelEvent");
    const input = validator("TimeTravelEventInput");
    const events = [
      ...Object.values(law.events),
      ...law.cases.map((row: any) => row.event),
      ...law.matrix.flatMap((row: any) => (row.input === undefined ? [] : [row.input])),
      ...law.scenarios.flatMap((row: any) => row.steps.map((step: any) => step.event)),
    ];
    let malformed = 0;
    for (const json of events) {
      expect(input(json), JSON.stringify(json)).toBe(true);
      const wellFormed = M.timeTravelEventWellFormed(M.timeTravelEventFromJson(json));
      if (!strict(json)) expect(wellFormed, JSON.stringify(json).slice(0, 120)).toBe(false);
      if (!wellFormed) malformed += 1;
    }
    expect(malformed).toBeGreaterThanOrEqual(10);
    const tooLong = { type: "choose", generation: 5, choice: { kind: "alternative", name: "ä".repeat(129) } };
    expect([strict(tooLong), M.timeTravelEventWellFormed(M.timeTravelEventFromJson(tooLong))]).toEqual([true, false]);
  });
});

describe("lifecycle law", () => {
  test("vocabularies mirror the fixture", () => {
    expect([...M.TIME_TRAVEL_STAGES]).toEqual(law.stages);
    expect([...M.TIME_TRAVEL_EVENT_KEYS]).toEqual(law.eventKeys);
    expect([...M.TIME_TRAVEL_EFFECT_KINDS]).toEqual(law.effectKinds);
    expect([...M.TIME_TRAVEL_REFUSALS]).toEqual(law.refusals);
    expect([...M.TIME_TRAVEL_REVIEWS]).toEqual(law.reviewKinds);
    expect(M.TIME_TRAVEL_FROZEN_CODE).toBe(law.frozenCode);
    expect(M.TIME_TRAVEL_CANCELLED_CODE).toBe(law.cancelledCode);
    expect(M.TIME_TRAVEL_TEXT_MAX_BYTES).toBe(law.limits.textMaxBytes);
    for (const key of M.TIME_TRAVEL_EVENT_KEYS) {
      expect(M.timeTravelEventKey(eventOf(key))).toBe(key);
      expect(M.timeTravelEventWellFormed(eventOf(key)), key).toBe(true);
    }
  });

  test("every (stage, event) pair is covered with distinct known guards", () => {
    for (const stage of M.TIME_TRAVEL_STAGES) {
      for (const key of M.TIME_TRAVEL_EVENT_KEYS) {
        const guards = law.matrix.filter((row: any) => row.from === stage && row.event === key).map((row: any) => row.when);
        expect(guards.length, `${stage} × ${key}`).toBeGreaterThan(0);
        expect(new Set(guards).size, `${stage} × ${key}`).toBe(guards.length);
        for (const guard of guards) expect(law.guards).toContain(guard);
      }
    }
  });

  test("reducer matches every matrix row", () => {
    for (const row of law.matrix) {
      const label = `${row.from} × ${row.event} [${row.when}]`;
      const before = contextOf(row.context);
      expect(before.stage, label).toBe(row.from);
      expect(M.timeTravelInvariantViolation(before), label).toBeNull();
      const snapshot = M.timeTravelSessionToJson(before);
      const event = rowEvent(row);
      expect(M.timeTravelEventKey(event), label).toBe(row.event);
      const result = M.applyTimeTravel(before, event);
      expect(M.timeTravelSessionToJson(before), `${label}: input untouched`).toEqual(snapshot);
      if ("rejection" in row) {
        expect(result.ok ? "accepted" : result.rejection, label).toBe(row.rejection);
        continue;
      }
      expect(result.ok, label).toBe(true);
      if (!result.ok) continue;
      expect(result.session.stage, label).toBe(row.to);
      expect(kinds(result.effects), label).toEqual(row.effects);
      expect(result.session.generation, label).toBe(row.generation === "increment" ? before.generation + 1 : before.generation);
      expect(result.session.id, label).toBe(row.session === "next" ? before.id + 1n : before.id);
      expect(M.timeTravelInvariantViolation(result.session), label).toBeNull();
    }
  });

  test("stale generations are silent no-ops in every context", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const before = M.timeTravelSessionFromJson(json);
      for (const key of M.TIME_TRAVEL_EVENT_KEYS) {
        const canonical = law.events[key];
        if (!("generation" in canonical)) continue;
        for (const generation of [before.generation - 1, before.generation + 1]) {
          const result = M.applyTimeTravel(before, M.timeTravelEventFromJson({ ...canonical, generation }));
          expect(result.ok ? "accepted" : result.rejection, `${name} × ${key} @ ${generation}`).toBe("timeTravel.stale");
        }
      }
    }
  });

  test("reducer matches every case", () => {
    for (const row of law.cases) {
      const result = M.applyTimeTravel(M.timeTravelSessionFromJson(row.session), M.timeTravelEventFromJson(row.event));
      if ("rejection" in row.expect) {
        expect(result.ok ? "accepted" : result.rejection, row.name).toBe(row.expect.rejection);
        continue;
      }
      expect(result.ok, row.name).toBe(true);
      if (!result.ok) continue;
      expect({ session: M.timeTravelSessionToJson(result.session), effects: result.effects.map(M.timeTravelEffectToJson) }, row.name).toEqual(row.expect.transition);
    }
  });

  test("reducer replays every scenario", () => {
    for (const scenario of law.scenarios) {
      let session = M.timeTravelSessionFromJson(scenario.initial);
      scenario.steps.forEach((step: any, index: number) => {
        const result = M.applyTimeTravel(session, M.timeTravelEventFromJson(step.event));
        const label = `${scenario.name} step ${index}`;
        if ("rejection" in step.expect) {
          expect(result.ok ? "accepted" : result.rejection, label).toBe(step.expect.rejection);
          return;
        }
        expect(result.ok, label).toBe(true);
        if (!result.ok) return;
        expect([result.session.stage, kinds(result.effects)], label).toEqual([step.expect.stage, step.expect.effects]);
        expect(M.timeTravelInvariantViolation(result.session), label).toBeNull();
        session = result.session;
      });
      expect(M.timeTravelSessionToJson(session), scenario.name).toEqual(scenario.final);
    }
  });

  test("every context reviews as the fixture says, and rerun is legal exactly when a replay is needed or a fault is shown", () => {
    expect(law.reviews.map((row: any) => row.context)).toEqual(Object.keys(law.contexts));
    for (const row of law.reviews) {
      const context = contextOf(row.context);
      expect(M.timeTravelReview(context), row.context).toBe(row.review);
      if (context.stage === "reviewing") expect(M.timeTravelRerunRefusal(context) === null, row.context).toBe(row.review === "needsReplay" || (context.fault !== null && context.accepted.length > 0));
    }
    expect(M.TIME_TRAVEL_REVIEWS.map(M.timeTravelReviewLabel)).toEqual(["noChanges", "needsReplay", "reportBlocking", "readyToFinalize"]);
    expect([M.timeTravelCodeLabel(M.TIME_TRAVEL_CANCELLED_CODE), M.timeTravelCodeLabel("vcs.rejected")]).toEqual(["replayCancelled", undefined]);
  });

  test("text limits hold at their edges with Unicode White_Space", () => {
    expect([M.isTimeTravelFaultCode("x".repeat(256)), M.isTimeTravelFaultCode("x".repeat(257)), M.isTimeTravelFaultCode(""), M.isTimeTravelFaultCode("vcs\u0085rejected"), M.isTimeTravelFaultCode("vcs\ufeffrejected")]).toEqual([
      true,
      false,
      false,
      false,
      true,
    ]);
    expect([
      M.isTimeTravelAlternativeName("ä".repeat(128)),
      M.isTimeTravelAlternativeName("ä".repeat(129)),
      M.isTimeTravelAlternativeName(""),
      M.isTimeTravelAlternativeName("\u00a0\u0085\u2003\t"),
      M.isTimeTravelAlternativeName(" Edited history "),
    ]).toEqual([true, false, false, false, true]);
  });

  test("invariants hold for every context and fail for their counterexample", () => {
    for (const [name, json] of Object.entries(law.contexts)) expect(M.timeTravelInvariantViolation(M.timeTravelSessionFromJson(json)), name).toBeNull();
    for (const row of law.invariants) expect(M.timeTravelInvariantViolation(M.timeTravelSessionFromJson(row.violation))).toBe(row.id);
  });

  test("every timeTravel code names the fixture label, and every session refusal its own", () => {
    expect(M.TIME_TRAVEL_CODE_LABELS.map(([code, key]) => ({ code, key }))).toEqual(law.codeLabels);
    for (const [code, key] of M.TIME_TRAVEL_CODE_LABELS) expect([M.isTimeTravelFaultCode(code), code.startsWith("timeTravel."), M.timeTravelCodeLabel(code)]).toEqual([true, true, key]);
    for (const refusal of M.TIME_TRAVEL_REFUSALS) expect(M.timeTravelCodeLabel(refusal)).toBe(M.timeTravelRefusalLabel(refusal));
    expect(new Set(M.TIME_TRAVEL_CODE_LABELS.map(([code]) => code)).size).toBe(M.TIME_TRAVEL_CODE_LABELS.length);
  });

  test("labels mirror the fixture with both locales", () => {
    expect(Object.entries(M.TIME_TRAVEL_LABELS).map(([key, text]) => ({ key, ...text }))).toEqual(law.labels);
    for (const stage of M.TIME_TRAVEL_STAGES) expect(M.TIME_TRAVEL_LABELS[M.timeTravelStageLabel(stage)]).toBeDefined();
    for (const refusal of M.TIME_TRAVEL_REFUSALS) expect(M.TIME_TRAVEL_LABELS[M.timeTravelRefusalLabel(refusal)]).toBeDefined();
  });

  test("history order is UTF-8 byte order, as Rust's String ordering", () => {
    const ids = ["b", "a", "ab", "\u{e000}", "\u{1f600}", "Z", ""];
    const sorted = [...ids].sort(M.compareCodePoints);
    const bytes = [...ids].sort((left, right) => Buffer.compare(Buffer.from(left, "utf8"), Buffer.from(right, "utf8")));
    expect(sorted).toEqual(bytes);
  });
});

describe("state machine oracle (xstate + fast-check)", () => {
  type Key = string;
  type Draft = { mutation: string; position: number; value: Key };
  type Pending = { mutation: string; position: number; original: Key; value: Key; returnStage: string };
  type Model = { id: bigint; generation: number; base: Key; accepted: Draft[]; pending: Pending | null; report: "clean" | "blocking" | null; fault: string | null; progress: string | null };
  type OracleEvent = { type: string; generation?: number; mutation?: string; position?: number; value?: Key; original?: Key; report?: "clean" | "blocking"; code?: string; name?: string; base?: Key; positions?: [string, number][] };

  const keyOf = (value: InputReplacement): Key => (value.kind === "withdrawn" ? "withdrawn" : `${value.schema}:${M.timeTravelBytesToHex(value.payload)}`);
  const order = (drafts: Draft[]): Draft[] => [...drafts].sort((left, right) => left.position - right.position || M.compareCodePoints(left.mutation, right.mutation));
  const startOf = (model: Model, pending: Pending): Key => model.accepted.find((draft) => draft.mutation === pending.mutation)?.value ?? pending.original;
  const unchanged = (model: Model): boolean => model.pending !== null && model.pending.value === startOf(model, model.pending);
  const acceptedAfter = (model: Model): Draft[] => {
    const pending = model.pending!;
    const kept = model.accepted.filter((draft) => draft.mutation !== pending.mutation);
    return pending.value === pending.original ? kept : order([...kept, { mutation: pending.mutation, position: pending.position, value: pending.value }]);
  };
  const reportLost = (model: Model): boolean => model.report === null && model.accepted.length > 0;
  const WHITE_SPACE = new Set([0x9, 0xa, 0xb, 0xc, 0xd, 0x20, 0x85, 0xa0, 0x1680, ...Array.from({ length: 11 }, (_, index) => 0x2000 + index), 0x2028, 0x2029, 0x202f, 0x205f, 0x3000]);
  const codePoints = (text: string): number[] => Array.from(text, (character) => character.codePointAt(0)!);
  const fitsBytes = (text: string): boolean => Buffer.byteLength(text, "utf8") <= 256;
  const guards: Record<string, (model: Model, event: OracleEvent) => boolean> = {
    always: () => true,
    newBase: (model, event) => event.base !== model.base && model.accepted.length > 0,
    newBaseEmpty: (model, event) => event.base !== model.base && model.accepted.length === 0,
    pendingUnchanged: (model) => unchanged(model),
    unchangedReturnInactive: (model) => unchanged(model) && model.pending!.returnStage === "inactive",
    unchangedReturnReviewing: (model) => unchanged(model) && model.pending!.returnStage === "reviewing" && !reportLost(model),
    unchangedReturnReplaying: (model) => unchanged(model) && model.pending!.returnStage === "reviewing" && reportLost(model),
    changed: (model) => !unchanged(model) && acceptedAfter(model).length > 0,
    changedToEmpty: (model) => !unchanged(model) && acceptedAfter(model).length === 0,
    returnInactive: (model) => model.pending!.returnStage === "inactive",
    returnReviewing: (model) => model.pending!.returnStage === "reviewing" && !reportLost(model),
    returnReplaying: (model) => model.pending!.returnStage === "reviewing" && reportLost(model),
    ready: (model) => model.accepted.length > 0 && model.report === "clean",
    needsReplay: (model) => model.accepted.length > 0 && (model.report === null || model.fault !== null),
    validCode: (_, event) => event.code!.length > 0 && fitsBytes(event.code!) && codePoints(event.code!).every((point) => !WHITE_SPACE.has(point)),
    validName: (_, event) => fitsBytes(event.name!) && codePoints(event.name!).some((point) => !WHITE_SPACE.has(point)),
  };
  const baseMovedGuard = (row: any) => (model: Model, event: OracleEvent) => (row.from === "inactive" || row.from === "editing" || row.from === "finalizing" ? event.base !== model.base : guards[row.when]!(model, event));

  const cleared = (model: Model): Model => ({ ...model, generation: model.generation + 1, accepted: [], pending: null, report: null, fault: null, progress: null });
  const replayed = (model: Model): Model => (model.accepted.length === 0 ? { ...model, report: null, progress: null, fault: null } : { ...model, generation: model.generation + 1, report: null, progress: null, fault: null });
  const moved = (model: Model, event: OracleEvent): Model => {
    const position = (mutation: string, current: number) => event.positions!.reduce((found, [candidate, value]) => (candidate === mutation ? value : found), current);
    return {
      ...model,
      base: event.base!,
      accepted: order(model.accepted.map((draft) => ({ ...draft, position: position(draft.mutation, draft.position) }))),
      pending: model.pending === null ? null : { ...model.pending, position: position(model.pending.mutation, model.pending.position) },
    };
  };
  const begun = (model: Model, event: OracleEvent, returnStage: string, id: bigint): Model => ({
    ...model,
    id,
    generation: model.generation + 1,
    pending: { mutation: event.mutation!, position: event.position!, original: event.original!, value: model.accepted.find((draft) => draft.mutation === event.mutation)?.value ?? event.original!, returnStage },
  });
  const resumed = (model: Model, returnStage: string): Model => {
    const next = { ...model, pending: null };
    if (returnStage !== "reviewing") return cleared(next);
    return reportLost(next) ? replayed(next) : next;
  };

  const assignments: Record<string, (model: Model, event: OracleEvent, row: any) => Model> = {
    begin: (model, event, row) => begun(model, event, row.from === "editing" ? model.pending!.returnStage : row.from, row.from === "inactive" ? model.id + 1n : model.id),
    draft: (model, event) => ({ ...model, pending: { ...model.pending!, value: event.value! } }),
    withdraw: (model) => ({ ...model, pending: { ...model.pending!, value: "withdrawn" } }),
    accept: (model) => (unchanged(model) ? resumed(model, model.pending!.returnStage) : replayed({ ...model, accepted: acceptedAfter(model), pending: null })),
    discard: (model) => resumed(model, model.pending!.returnStage),
    replayProgressed: (model, event) => ({ ...model, progress: event.value! }),
    replayCompleted: (model, event) => ({ ...model, report: event.report!, progress: null }),
    replayCancelled: (model) => ({ ...model, progress: null, fault: "timeTravel.cancelled" }),
    rerun: (model) => replayed(model),
    replayFaulted: (model, event) => ({ ...model, progress: null, fault: event.code! }),
    requestFinalize: (model) => model,
    chooseOverwrite: (model) => ({ ...model, generation: model.generation + 1 }),
    chooseAlternative: (model) => ({ ...model, generation: model.generation + 1 }),
    back: (model) => model,
    finalized: (model) => cleared(model),
    finalizeFaulted: (model, event) => ({ ...replayed(model), fault: event.code! }),
    baseMoved: (model, event, row) => {
      const next = moved(model, event);
      if (row.from === "editing") return { ...next, report: null };
      return row.from === "inactive" || row.from === "finalizing" ? next : replayed(next);
    },
    exit: (model) => cleared(model),
  };

  const states: Record<string, { on: Record<string, unknown[]> }> = {};
  for (const stage of law.stages) states[stage] = { on: {} };
  for (const row of law.matrix.filter((row: any) => "to" in row)) {
    const when = row.event === "baseMoved" ? baseMovedGuard(row) : guards[row.when]!;
    const guard = ({ context, event }: { context: Model; event: OracleEvent }) => (event.generation === undefined || event.generation === context.generation) && when(context, event);
    const action = assign(({ context, event }: { context: Model; event: OracleEvent }) => assignments[row.event]!(context, event, row));
    (states[row.from]!.on[row.event] ??= []).push({ target: `#timeTravel.${row.to}`, guard, actions: action });
  }
  const initialModel = (session: M.TimeTravelSession): Model => ({
    id: session.id,
    generation: session.generation,
    base: M.timeTravelBytesToHex(session.base.contentRevision) + `@${session.base.storeGeneration}`,
    accepted: session.accepted.map((draft) => ({ mutation: draft.target.mutation, position: draft.target.position, value: keyOf(draft.replacement) })),
    pending:
      session.pending === null
        ? null
        : { mutation: session.pending.target.mutation, position: session.pending.target.position, original: keyOf(session.pending.original), value: keyOf(session.pending.replacement), returnStage: session.pending.returnStage },
    report: session.report === null ? null : replayReportBlocksFinalize(session.report) ? "blocking" : "clean",
    fault: session.fault,
    progress: session.progress === null ? null : `${session.progress.done}/${session.progress.total}`,
  });
  const machine = createMachine({ id: "timeTravel", initial: "inactive", context: initialModel(contextOf("inactive")), states } as any);

  const oracleEvent = (event: M.TimeTravelEvent): OracleEvent => {
    const type = M.timeTravelEventKey(event);
    switch (event.type) {
      case "begin":
        return { type, mutation: event.target.mutation, position: event.target.position, original: keyOf(event.original) };
      case "draft":
        return { type, generation: event.generation, value: keyOf(event.replacement) };
      case "replayProgressed":
        return { type, generation: event.generation, value: `${event.done}/${event.total}` };
      case "replayCompleted":
        return { type, generation: event.generation, report: replayReportBlocksFinalize(event.report) ? "blocking" : "clean" };
      case "replayFaulted":
      case "finalizeFaulted":
        return { type, generation: event.generation, code: event.code };
      case "baseMoved":
        return { type, base: M.timeTravelBytesToHex(event.base.contentRevision) + `@${event.base.storeGeneration}`, positions: event.positions.map((target) => [target.mutation, target.position]) };
      case "choose":
        return { type, generation: event.generation, ...(event.choice.kind === "alternative" ? { name: event.choice.name } : {}) };
      case "exit":
        return { type };
      default:
        return { type, generation: event.generation };
    }
  };
  const project = (session: M.TimeTravelSession): Model => initialModel(session);
  const snapshotOf = (session: M.TimeTravelSession) => machine.resolveState({ value: session.stage, context: initialModel(session) } as any) as AnyMachineSnapshot;
  const firstLegalRow = (model: Model, stage: string, event: OracleEvent) =>
    law.matrix.find(
      (row: any) => "to" in row && row.from === stage && row.event === event.type && (event.generation === undefined || event.generation === model.generation) && (row.event === "baseMoved" ? baseMovedGuard(row) : guards[row.when]!)(model, event),
    );

  test("the xstate machine agrees with the reducer on every matrix row", () => {
    const [initial] = initialTransition(machine);
    expect(initial.value).toBe("inactive");
    for (const row of law.matrix) {
      const session = contextOf(row.context);
      const event = rowEvent(row);
      const result = M.applyTimeTravel(session, event);
      const snapshot = snapshotOf(session);
      const label = `${row.from} × ${row.event} [${row.when}]`;
      expect(snapshot.can(oracleEvent(event) as any), label).toBe(result.ok);
      if (!result.ok) continue;
      const [next] = transition(machine, snapshot, oracleEvent(event) as any);
      expect(next.value, label).toBe(result.session.stage);
      expect(next.context, label).toEqual(project(result.session));
    }
  });

  const SCHEMA = "s.demo.op";
  const SEED = 20_260_930;
  const RUNS = ["quick", "long", "exhaustive"].includes(process.env.SEMIO_TEST_LEVEL ?? "fundamental") ? 3_000 : 400;
  const starts = fc.oneof({ arbitrary: fc.constant("fresh"), weight: 1 }, { arbitrary: fc.constantFrom(...Object.keys(law.contexts)), weight: 2 });
  const value = (payload: string): InputReplacement => ({ kind: "input", schema: SCHEMA, payload: Array.from(M.timeTravelHexToBytes(payload)) });
  const originals: Record<string, InputReplacement> = { a: value("0a01"), b: value("0b01"), c: value("0c01") };
  const values: InputReplacement[] = [value("0a01"), value("0b01"), value("0c01"), value("0f01"), value("0f02"), { kind: "withdrawn" }];
  const reports = { clean: law.contexts["reviewing.ready"].report, blocking: law.contexts["reviewing.blocking"].report };
  const codes = ["replay.targetMissing", "vcs.rejected", "", "vcs rejected", "x".repeat(257), "vcs\u0085rejected"];
  const names = ["Edited history", " padded ", "", " \u00a0", "\u0085", "ä".repeat(129), "ä".repeat(128)];
  const bases = [0, 1, 2].map((index) => ({ storeGeneration: BigInt(10 + index), contentRevision: new Uint8Array(32).fill(index + 1) }));

  const pick = fc.record({
    key: fc.oneof(...M.TIME_TRAVEL_EVENT_KEYS.map((key) => ({ arbitrary: fc.constant(key), weight: key === "baseMoved" ? 3 : 1 }))),
    stale: fc.oneof({ arbitrary: fc.constant(0), weight: 8 }, { arbitrary: fc.constant(-1), weight: 1 }, { arbitrary: fc.constant(1), weight: 1 }),
    mutation: fc.oneof({ arbitrary: fc.constant("accepted"), weight: 2 }, { arbitrary: fc.constantFrom("a", "b", "c"), weight: 3 }),
    value: fc.oneof({ arbitrary: fc.constant(-1), weight: 2 }, { arbitrary: fc.nat(values.length - 1), weight: 4 }),
    report: fc.constantFrom<"clean" | "blocking">("clean", "clean", "blocking"),
    base: fc.nat(2),
    positions: fc.shuffledSubarray([2, 5, 9, 12], { minLength: 3, maxLength: 3 }),
    done: fc.nat(4),
    code: fc.oneof({ arbitrary: fc.nat(1), weight: 6 }, { arbitrary: fc.integer({ min: 2, max: codes.length - 1 }), weight: 1 }),
    name: fc.oneof({ arbitrary: fc.nat(1), weight: 6 }, { arbitrary: fc.integer({ min: 2, max: names.length - 1 }), weight: 1 }),
  });
  const agreement = (visited: Set<string>) =>
    fc.property(starts, fc.array(pick, { minLength: 40, maxLength: 200, size: "max" }), (start, picks) => {
      let session = start === "fresh" ? M.timeTravelSession(bases[0]!) : contextOf(start);
      let snapshot = snapshotOf(session);
      let positions: Record<string, number> = {
        a: 2,
        b: 5,
        c: 9,
        ...Object.fromEntries([...session.accepted.map((draft) => draft.target), ...(session.pending === null ? [] : [session.pending.target])].map((target) => [target.mutation, target.position])),
      };
      for (const choice of picks) {
        const generation = session.generation + choice.stale;
        const event: M.TimeTravelEvent = (() => {
          switch (choice.key) {
            case "begin": {
              const mutation = choice.mutation === "accepted" ? (session.accepted[0]?.target.mutation ?? "a") : choice.mutation;
              return { type: "begin", target: { mutation, position: positions[mutation]! }, original: originals[mutation]! };
            }
            case "draft":
              return { type: "draft", generation, replacement: choice.value < 0 ? originals[session.pending?.target.mutation ?? "a"]! : values[choice.value]! };
            case "replayProgressed":
              return { type: "replayProgressed", generation, done: choice.done, total: 4 };
            case "replayCompleted":
              return { type: "replayCompleted", generation, report: reports[choice.report] };
            case "replayFaulted":
            case "finalizeFaulted":
              return { type: choice.key, generation, code: codes[choice.code]! };
            case "chooseOverwrite":
              return { type: "choose", generation, choice: { kind: "overwrite" } };
            case "chooseAlternative":
              return { type: "choose", generation, choice: { kind: "alternative", name: names[choice.name]! } };
            case "baseMoved": {
              const next = { a: choice.positions[0]!, b: choice.positions[1]!, c: choice.positions[2]! };
              return { type: "baseMoved", base: bases[choice.base]!, positions: Object.entries(next).map(([mutation, position]) => ({ mutation, position })) };
            }
            case "exit":
              return { type: "exit" };
            default:
              return { type: choice.key, generation } as M.TimeTravelEvent;
          }
        })();
        const oracle = oracleEvent(event);
        const result = M.applyTimeTravel(session, event);
        if (snapshot.can(oracle as any) !== result.ok) return false;
        if (!result.ok) continue;
        const row = firstLegalRow(snapshot.context as Model, snapshot.value as string, oracle);
        if (row === undefined || JSON.stringify(kinds(result.effects)) !== JSON.stringify(row.effects)) return false;
        visited.add(`${row.from} × ${row.event} [${row.when}]`);
        [snapshot] = transition(machine, snapshot, oracle as any);
        session = result.session;
        if (event.type === "baseMoved") positions = Object.fromEntries(event.positions.map((target) => [target.mutation, target.position]));
        if (snapshot.value !== session.stage) return false;
        if (!Bun.deepEquals(snapshot.context, project(session))) return false;
        if (M.timeTravelInvariantViolation(session) !== null) return false;
        const start = result.effects.find((effect) => effect.type === "startReplay");
        if (start !== undefined && start.type === "startReplay" && (start.from !== session.accepted[0]!.target.mutation || start.drafts.map((draft) => draft.target).join() !== session.accepted.map((draft) => draft.target.mutation).join()))
          return false;
      }
      return true;
    });

  test("random event sequences: reducer and xstate agree on acceptance, stage, model and effects", () => {
    fc.assert(agreement(new Set()), { numRuns: RUNS });
  }, 120_000);

  test("a pinned seed visits every legal law row, so the random oracle is not vacuous", () => {
    const visited = new Set<string>();
    fc.assert(agreement(visited), { numRuns: RUNS, seed: SEED });
    const legal = law.matrix.filter((row: any) => "to" in row).map((row: any) => `${row.from} × ${row.event} [${row.when}]`);
    expect(legal.filter((key: string) => !visited.has(key))).toEqual([]);
  }, 120_000);
});
