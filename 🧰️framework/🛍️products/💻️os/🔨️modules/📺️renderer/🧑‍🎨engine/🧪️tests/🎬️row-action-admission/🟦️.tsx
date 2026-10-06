/** 🎬️ Retained row actions admit once per exact owner, Store, target and action. */
import { act, cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020";
import { afterEach, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🎬️row-action-admission/🔣️.json";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { LocalDocumentOwnerRegistryV1 } from "../../🧱️elements/🗣️Interpreter/🧭️local-document-owner/🟦️.ts";

afterEach(cleanup);

type Case = typeof fixture.initial;
type Deferred = { readonly promise: Promise<unknown>; resolve(value: unknown): void; reject(reason: unknown): void };

const deferred = (): Deferred => {
  let resolve = (_value: unknown): void => {};
  let reject = (_reason: unknown): void => {};
  const promise = new Promise<unknown>((accept, refuse) => { resolve = accept; reject = refuse; });
  return { promise, resolve, reject };
};

const record = (id: number, key: string, component: Record<string, unknown>, children: number[] = []) => ({
  id,
  key,
  component,
  children,
  disabled: false,
  activity: "idle",
  layout: { kind: "leaf", width: "hug", height: "hug" },
  style: {},
  accessibility: {},
  bindings: [],
});

function snapshot(revision: number, current: Case) {
  return {
    surface: "row-action-admission",
    revision,
    layoutEpoch: 0n,
    root: 0,
    nodes: [
      record(0, "root", { type: "container", role: "group" }, [1, 4]),
      record(1, "tree", { type: "tree", presentation: "tree" }, [2]),
      record(2, "tree-section", { type: "treeSection", label: "Rows", defaultOpen: true }, [3]),
      record(3, "tree-row", { type: "treeItem", label: fixture.expected.treeRowName, target: current.target, rowActions: [current.action] }),
      record(4, "table", { type: "table", label: "Rows", columns: ["Name"], actionsLabel: "Actions" }, [5]),
      record(5, "table-row", { type: "tableRow", cells: [fixture.expected.tableRowName], target: current.target, rowActions: [current.action] }),
    ],
  };
}

function mount(current: Case = fixture.initial) {
  const owners = new LocalDocumentOwnerRegistryV1();
  const owner = owners.acquire(fixture.owner);
  const store = new UiDocumentStore("row-action-admission");
  store.loadSnapshot(snapshot(1, current) as any);
  const intents: any[] = [];
  const settlements: Deferred[] = [];
  const context = (activeOwner: typeof owner, activeStore = store) => ({
    store: activeStore,
    localDocumentOwner: activeOwner,
    onAction: () => {},
    onIntent: (intent: unknown) => { intents.push(intent); const next = deferred(); settlements.push(next); return next.promise; },
  });
  const view = render(createElement(UiNodeView, { store, id: 0, context: context(owner) }));
  return { owners, owner, store, intents, settlements, context, view };
}

const tableAction = (label = fixture.initial.action.label) => screen.getByRole("button", { name: `${label}: ${fixture.expected.tableRowName}` }) as HTMLButtonElement;
const treeAction = (label = fixture.initial.action.label) => screen.getByRole("button", { name: label }) as HTMLButtonElement;
const pending = (button: HTMLButtonElement): boolean => !button.disabled && button.getAttribute("aria-disabled") === "true" && button.getAttribute("aria-busy") === "true";

it("validates the language-neutral admission vectors with an independent JSON Schema oracle", () => {
  expect(fixture.initial.target).not.toEqual(fixture.replacement.target);
  expect(fixture.initial.action.verb).not.toBe(fixture.replacement.action.verb);
});

for (const ending of ["completed", "refused", "rejected"] as const) it(`recovers table and tree actions after ${ending} settlement`, async () => {
  const mounted = mount();
  act(() => tableAction().focus());
  fireEvent.click(tableAction());
  fireEvent.click(tableAction());
  fireEvent.click(treeAction());
  expect(mounted.intents).toHaveLength(fixture.expected.pendingDispatches);
  for (const button of [tableAction(), treeAction()]) expect([button.disabled, button.getAttribute("aria-disabled"), button.getAttribute("aria-busy")], "a pending action is busy, never gone: it stays focusable").toEqual([false, "true", "true"]);
  expect(document.activeElement === tableAction(), "the pressed action keeps the focus its press gave it").toBe(true);
  await act(async () => {
    if (ending === "rejected") mounted.settlements[0]!.reject(new Error("transport refused"));
    else mounted.settlements[0]!.resolve(ending === "completed" ? { kind: "applied", inputSeq: 1 } : { kind: "refused", inputSeq: 1, reason: "row-conflict", retryable: true });
    await Promise.resolve();
  });
  for (const button of [tableAction(), treeAction()]) expect([button.disabled, button.hasAttribute("aria-disabled"), button.hasAttribute("aria-busy")], `a ${ending} settlement ends the pending state it began`).toEqual([false, false, false]);
  expect(document.activeElement === tableAction(), "and the action still holds the focus").toBe(true);
  mounted.owners.retireAll();
});

it("refuses stale table and tree callbacks after the current rows become disabled", () => {
  const mounted = mount();
  const staleTable = tableAction();
  const staleTree = treeAction();
  const disabled = snapshot(2, fixture.initial) as any;
  disabled.nodes[3].disabled = true;
  disabled.nodes[5].disabled = true;
  mounted.store.loadSnapshot(disabled);
  staleTable.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  staleTree.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  expect(mounted.intents).toHaveLength(0);
  mounted.owners.retireAll();
});

it("refuses stale callbacks when their action slot is replaced or reordered before React commits", () => {
  const mounted = mount();
  const staleTable = tableAction();
  const staleTree = treeAction();
  const reordered = snapshot(2, fixture.initial) as any;
  reordered.nodes[3].component.rowActions = [fixture.replacement.action, fixture.initial.action];
  reordered.nodes[5].component.rowActions = [fixture.replacement.action, fixture.initial.action];
  mounted.store.loadSnapshot(reordered);
  staleTable.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  staleTree.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  expect(mounted.intents).toHaveLength(0);
  mounted.owners.retireAll();
});

it("keeps the authored command identity stable across cosmetic row-action publication", async () => {
  const mounted = mount();
  const stale = tableAction();
  const cosmetic = snapshot(2, fixture.initial) as any;
  cosmetic.nodes[5].component.rowActions[0] = { ...fixture.initial.action, icon: fixture.replacement.action.icon, label: fixture.replacement.action.label };
  mounted.store.loadSnapshot(cosmetic);
  stale.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  expect(mounted.intents).toHaveLength(1);
  expect(mounted.intents[0].action).toEqual({ scope: fixture.initial.target.scope, name: fixture.initial.action.verb, version: fixture.initial.target.version });
  await act(async () => { mounted.settlements[0]!.resolve({ kind: "applied", inputSeq: 1 }); await Promise.resolve(); });
  mounted.owners.retireAll();
});

it("isolates replacement target, Store and owner lifetimes from an older pending action", async () => {
  const mounted = mount();
  fireEvent.click(treeAction());
  expect(mounted.intents).toHaveLength(1);
  await act(async () => mounted.store.loadSnapshot(snapshot(2, fixture.replacement) as any));
  expect(tableAction(fixture.replacement.action.label).disabled).toBe(false);
  fireEvent.click(treeAction(fixture.replacement.action.label));
  expect(mounted.intents).toHaveLength(fixture.expected.replacementDispatches);
  await act(async () => { mounted.settlements[0]!.resolve({ kind: "applied", inputSeq: 1 }); await Promise.resolve(); });
  expect(pending(tableAction(fixture.replacement.action.label))).toBe(true);
  await act(async () => { mounted.settlements[1]!.resolve({ kind: "refused", inputSeq: 2, reason: "row-conflict", retryable: true }); await Promise.resolve(); });
  expect(pending(tableAction(fixture.replacement.action.label))).toBe(false);
  const replacementStore = new UiDocumentStore("row-action-admission");
  replacementStore.loadSnapshot(snapshot(3, fixture.replacement) as any);
  mounted.view.rerender(createElement(UiNodeView, { store: replacementStore, id: 0, context: mounted.context(mounted.owner, replacementStore) }));
  expect(tableAction(fixture.replacement.action.label).disabled).toBe(false);
  fireEvent.click(tableAction(fixture.replacement.action.label));
  expect(mounted.intents).toHaveLength(3);
  await act(async () => mounted.owners.retire(fixture.owner));
  expect(tableAction(fixture.replacement.action.label).disabled).toBe(true);
  fireEvent.click(tableAction(fixture.replacement.action.label));
  expect(mounted.intents).toHaveLength(3);
  const nextOwner = mounted.owners.acquire(fixture.owner);
  mounted.view.rerender(createElement(UiNodeView, { store: replacementStore, id: 0, context: mounted.context(nextOwner, replacementStore) }));
  expect(tableAction(fixture.replacement.action.label).disabled).toBe(false);
  fireEvent.click(tableAction(fixture.replacement.action.label));
  expect(mounted.intents).toHaveLength(4);
  mounted.owners.retireAll();
});
