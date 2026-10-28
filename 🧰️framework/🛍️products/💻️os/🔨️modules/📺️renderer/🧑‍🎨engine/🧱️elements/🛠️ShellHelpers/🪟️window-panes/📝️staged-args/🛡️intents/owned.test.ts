import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { applyPatch } from "fast-json-patch";
import cases from "./cases.json";
import ownedCases from "./owned-cases.json";
import schema from "./owned-cases.schema.json";
import { StagedFormSession } from "./🟦️.ts";
import { OwnedStagedControls, type StagedOwnedSurface, type StagedOwnedSubscription, type StagedOwnedSnapshot } from "./owned.ts";
import { STAGED_FORM_STATUS_EXTENSION, STAGED_FORM_EVENT_ACTION, STAGED_FORM_PAGE_ACTION, type StagedFormStatus } from "@semio-tech/framework/manifest/staging";

function fixture() {
  const token = { ...cases.token };
  const args = structuredClone(cases.args);
  let view = { revision: 1, root: 0, hash: "one" };
  let status: StagedFormStatus = { token, ready: true, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0" };
  const records = new Map<number, any>();
  const subscriptions = new Map<StagedOwnedSubscription, { snapshot: StagedOwnedSnapshot; notify: () => void }>();
  const viewSubscriptions = new Map<StagedOwnedSubscription, () => void>();
  const surface: StagedOwnedSurface = {
    get view() { return view; },
    subscribeNode: (id, notify) => {
      const snapshot = { version: view.revision, get record() { return records.get(id); } };
      const subscription: StagedOwnedSubscription = { get snapshot(): StagedOwnedSnapshot | null { return subscriptions.get(subscription)?.snapshot ?? null; } };
      subscriptions.set(subscription, { snapshot, notify });
      return subscription;
    },
    acknowledgeRead: () => {},
    subscribeView: (notify) => { const subscription = { snapshot: null }; viewSubscriptions.set(subscription, notify); return subscription; },
    unsubscribeNode: (subscription) => { subscriptions.delete(subscription); viewSubscriptions.delete(subscription); },
  };
  records.set(0, { id: 0, key: "status", component: { type: "extension", extension: STAGED_FORM_STATUS_EXTENSION, get props() { return status; } } });
  const record = { id: 7, key: "count", disabled: false, component: { type: "input", kind: "text", value: "0", commit: null }, bindings: [{ trigger: "change", action: { scope: "ifc", name: STAGED_FORM_EVENT_ACTION, version: 1 }, args: { token, layout: "preserve", edits: [{ source: "text", path: ["count"] }] } }] };
  records.set(7, record);
  const session = new StagedFormSession<number, number>({ token, locale: "de", controllerId: "ifc" }, args);
  const staged: unknown[] = [];
  const owner = new OwnedStagedControls(surface, "form", session, (result) => { staged.push(result); });
  const statusRead = owner.reads.subscribeNode(0, () => {});
  expect(owner.publishStatus(statusRead, statusRead.snapshot!, view)).toBe(true);
  const read = owner.reads.subscribeNode(7, () => {});
  const snapshot = read.snapshot!;
  const port = owner.control(read, snapshot, view);
  return { owner, surface, session, staged, statusRead, read, snapshot, port, record, records, args, token, subscriptions, viewSubscriptions, view: () => view,
    replace: () => { subscriptions.get(read)!.snapshot = { version: 1, record: { ...record } }; },
    publication: () => { view = { revision: 2, root: 0, hash: "two" }; for (const notify of viewSubscriptions.values()) notify(); },
    status: (value: StagedFormStatus) => { status = value; },
  };
}

test("closed language neutral owned control cases", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, ownedCases)).toBe(true);
  expect(ajv.validate(schema, { ...ownedCases, forged: true })).toBe(false);
});

for (const row of ownedCases.cases) test("owned control current custody: " + row.id, () => {
  const f = fixture();
  let port = f.port;
  if (row.state === "replaced") f.replace();
  if (row.state === "publication") f.publication();
  if (row.state === "unsubscribed") f.owner.reads.unsubscribeNode(f.read);
  if (row.state === "closed") f.owner.close();
  if (row.state === "unsupported") f.record.bindings[0].action.name = "ordinary.async.action";
  if (row.state === "disabled") f.record.disabled = true;
  if (row.state === "foreign") port = f.owner.control(f.surface.subscribeNode(7, () => {}), f.snapshot, f.view());
  port.dispatch(f.record as any, "change", "-");
  expect(f.staged.length).toBe(Number(row.sent));
  const expected = row.state === "closed" ? undefined : row.sent ? applyPatch(structuredClone(f.args), [{ op: "replace", path: "/count", value: "-" }], true, false, true).newDocument : f.args;
  expect(f.session.request()?.args).toEqual(expected);
});

test("text preserving control displays rapid drafts synchronously without a promise sink", () => {
  const f = fixture();
  expect(f.port.identity).toBe(f.owner);
  expect(f.port.disabled(f.record as any)).toBe(false);
  expect(f.port.dispatch(f.record as any, "change", "-")).toBeUndefined();
  expect(f.port.value(f.record as any)).toBe("-");
  f.port.dispatch(f.record as any, "change", "-1e");
  expect(f.port.value(f.record as any)).toBe("-1e");
  expect(f.staged.length).toBe(2);
  expect(f.session.canExecute()).toBe(false);
  expect(f.port.disabled(f.record as any)).toBe(false);
});

test("structural control disables old children and preserves page args identity", () => {
  const f = fixture();
  const record = { id: 8, key: "next", disabled: false, component: { type: "button" }, bindings: [{ trigger: "activate", action: { scope: "ifc", name: STAGED_FORM_PAGE_ACTION, version: 1 }, args: { token: f.token, page: { fieldOffset: "16", diagnosticOffset: "0" } } }] };
  f.records.set(8, record);
  const read = f.owner.reads.subscribeNode(8, () => {});
  const port = f.owner.control(read, read.snapshot!, f.view());
  const before = f.session.request()!.args;
  port.dispatch(record as any, "activate");
  expect(f.session.request()!.args).toBe(before);
  expect(f.session.request()!.page?.fieldOffset).toBe("16");
  expect(f.port.disabled(f.record as any)).toBe(true);
  expect(port.disabled(record as any)).toBe(true);
});

test("resident binding values are owned before asynchronous host follow-up", () => {
  const f = fixture();
  const fixed = { nested: { value: "original" } };
  const record = { id: 8, key: "create", disabled: false, component: { type: "button" }, bindings: [{ trigger: "activate", action: { scope: "ifc", name: STAGED_FORM_EVENT_ACTION, version: 1 }, args: { token: f.token, layout: "replace", edits: [{ source: "fixed", edit: { kind: "set", path: ["created"], value: fixed } }] } }] };
  f.records.set(8, record);
  const read = f.owner.reads.subscribeNode(8, () => {});
  f.owner.control(read, read.snapshot!, f.view()).dispatch(record as any, "activate");
  fixed.nested.value = "changed after borrow";
  expect(f.session.request()?.args.created).toEqual({ nested: { value: "original" } });
});

test("status and read failures disable dispatch; close is idempotent", () => {
  const f = fixture();
  const original = f.statusRead.snapshot!;
  f.status({ token: f.token, ready: true, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0", extra: true } as any);
  expect(f.owner.publishStatus(f.statusRead, original, f.view())).toBe(false);
  expect(f.port.disabled(f.record as any)).toBe(true);
  f.owner.close();
  f.owner.close();
  expect(f.subscriptions.size).toBe(0);
  expect(f.port.read(7)).toBeUndefined();
});

test("a closing notification retires the old staged follow-up", () => {
  const f = fixture();
  f.owner.subscribe(() => f.owner.close());
  f.port.dispatch(f.record as any, "change", "-");
  expect(f.staged).toEqual([]);
  expect(f.session.request()).toBeUndefined();
});

test("a newer synchronous edit supersedes the older notification follow-up", () => {
  const f = fixture();
  let replaced = false;
  f.owner.subscribe(() => { if (!replaced) { replaced = true; f.port.dispatch(f.record as any, "change", "newer"); } });
  f.port.dispatch(f.record as any, "change", "older");
  expect(f.staged).toHaveLength(1);
  expect((f.staged[0] as any).args.count).toBe("newer");
  expect(f.session.request()?.args.count).toBe("newer");
});

test("a notification replacing the publication retires the old follow-up", () => {
  const f = fixture();
  f.owner.subscribe(() => f.publication());
  f.port.dispatch(f.record as any, "change", "-");
  expect(f.staged).toEqual([]);
  expect(f.port.disabled(f.record as any)).toBe(true);
});

test("notification faults cannot interrupt argument follow-up or owned cleanup", () => {
  const f = fixture();
  f.owner.subscribe(() => { throw new Error("listener"); });
  let reached = false;
  f.owner.subscribe(() => { reached = true; });
  expect(() => f.port.dispatch(f.record as any, "change", "-")).not.toThrow();
  expect(reached).toBe(true);
  expect(f.staged).toHaveLength(1);
  expect(() => f.owner.close()).not.toThrow();
  expect(f.subscriptions.size).toBe(0);
});

test("failed source unsubscribe remains owned for a later close attempt", () => {
  const f = fixture();
  const unsubscribe = f.surface.unsubscribeNode;
  const error = new Error("retry source unsubscribe");
  let fail = true;
  f.surface.unsubscribeNode = (read) => { if (read === f.read && fail) throw error; unsubscribe(read); };
  expect(() => f.owner.close()).toThrow(error);
  expect(f.subscriptions.size).toBe(1);
  fail = false;
  f.owner.close();
  expect(f.subscriptions.size).toBe(0);
});

test("body metadata subscribes through the real view port and retains its original publication", () => {
  const f = fixture();
  const publications: unknown[] = [];
  const read = f.owner.reads.subscribeView(() => publications.push(f.owner.view()));
  expect(f.owner.view()).toBe(f.view());
  expect(f.subscriptions.size).toBe(2);
  expect(f.viewSubscriptions.size).toBe(1);
  f.publication();
  expect(publications).toEqual([f.view()]);
  expect(publications[0]).toBe(f.view());
  f.owner.close();
  expect(f.owner.view()).toBeUndefined();
  expect(f.subscriptions.size).toBe(0);
  expect(f.viewSubscriptions.size).toBe(0);
  f.owner.reads.unsubscribeNode(read);
});
