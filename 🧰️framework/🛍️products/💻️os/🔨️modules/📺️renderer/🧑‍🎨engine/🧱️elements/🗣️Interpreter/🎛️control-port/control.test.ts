import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import lodash from "lodash";
import { compare } from "fast-json-patch";
import type { ActionBinding, UiIntent, UiNodeRecord, Trigger as UiTrigger, UiValue } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import cases from "./cases.json";
import schema from "./cases.schema.json";
import { documentControlContext, type UiControlDocument } from "./🟦️.ts";

function fixture() {
  let current: UiNodeRecord | undefined;
  let revision = cases.revision;
  let sequence = 0n;
  const args = { count: "-", nested: { untouched: true } };
  const sent: UiIntent[] = [];
  const listeners = new Set<() => void>();
  const source: UiControlDocument = {
    getState: () => ({ surface: cases.surface, revision }),
    getNodeSnapshot: (id) => current?.id === id ? current : undefined,
    subscribeRevision: (listener) => { listeners.add(listener); return () => { listeners.delete(listener); }; },
    buildIntent: (record, binding, input) => ({ surface: cases.surface, revision, node: record.id, nodeKey: record.key, trigger: binding.trigger, action: binding.action, args: binding.args, input: input ?? null, seq: ++sequence }),
  };
  const context = documentControlContext({ store: source, onIntent: (intent) => { sent.push(intent); } });
  const record = (triggers: string[] = ["change"], disabled = false): UiNodeRecord => ({ id: 7, key: "count", component: { type: "input", value: "0", kind: "text" }, disabled, bindings: triggers.map((trigger) => ({ trigger, action: { scope: "ifc", name: "stage", version: 1 }, args, capability: null })) } as unknown as UiNodeRecord);
  return { context, source, sent, args, listeners, record, set: (value: UiNodeRecord | undefined) => { current = value; }, revision: (value: number) => { revision = value; for (const listener of listeners) listener(); } };
}

test("closed language neutral control vectors", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, cases)).toBe(true);
  expect(ajv.validate(schema, { ...cases, unexpected: true })).toBe(false);
});

for (const row of cases.cases) test("document control authority: " + row.id, () => {
  const f = fixture();
  const record = f.record(row.bindings, row.state === "disabled");
  const current = row.state === "removed" ? undefined : row.state === "replaced" ? f.record(row.bindings) : record;
  f.set(current);
  f.context.controls.dispatch(record, row.trigger as UiTrigger, "-");
  const bindings = lodash.filter(record.bindings, { trigger: row.trigger });
  const oracle = lodash.eq(current, record) && !record.disabled && bindings.length === 1;
  expect(oracle).toBe(row.sent);
  expect(f.sent.length).toBe(Number(oracle));
  if (oracle) {
    expect(f.sent[0].args).toBe(f.args);
    expect(f.sent[0].revision).toBe(cases.revision);
    expect(f.sent[0].seq).toBe(1n);
    expect(compare(f.sent[0].args as object, f.args)).toEqual([]);
  }
});

test("source identity and original callbacks survive a context refresh", () => {
  const f = fixture();
  const record = f.record();
  f.set(record);
  const context = documentControlContext({ store: f.source, onIntent: (intent) => { f.sent.push(intent); }, domScope: "row" });
  expect(context.controls.identity).toBe(f.context.controls.identity);
  expect(context.controls.surface()).toBe(cases.surface);
  expect(context.controls.read(record.id)).toBe(record);
  expect(context.controls.value(record)).toBeUndefined();
  expect(context.controls.disabled(record)).toBe(false);
  const revisions: number[] = [];
  const stop = context.controls.subscribe(() => revisions.push(f.source.getState().revision));
  f.revision(42);
  stop();
  f.revision(43);
  context.controls.dispatch(record, "change", "raw");
  expect(revisions).toEqual([42]);
  expect(f.sent[0].revision).toBe(43);
  expect(f.sent[0].args).toBe(f.args);
  expect(context.domScope).toBe("row");
  f.set(undefined);
  expect(context.controls.disabled(record)).toBe(true);
});

test("completion and failure retain the sink's exact identity", () => {
  const f = fixture();
  const record = f.record(["activate"]);
  f.set(record);
  const completion = Promise.resolve();
  const context = documentControlContext({ store: f.source, onIntent: () => completion });
  expect(context.controls.dispatch(record, "activate")).toBe(completion);
  const error = new Error("sink");
  const failed = documentControlContext({ store: f.source, onIntent: () => { throw error; } });
  expect(() => failed.controls.dispatch(record, "activate")).toThrow(error);
});

test("the adapter neither walks nor clones resident binding arguments", () => {
  const f = fixture();
  const record = f.record();
  const args = Object.defineProperty({}, "unread", { get() { throw new Error("walked args"); } });
  (record.bindings as ActionBinding[])[0] = { ...record.bindings![0], args: args as UiValue };
  f.set(record);
  f.context.controls.dispatch(record, "change", "-");
  expect(f.sent[0].args).toBe(args);
});
