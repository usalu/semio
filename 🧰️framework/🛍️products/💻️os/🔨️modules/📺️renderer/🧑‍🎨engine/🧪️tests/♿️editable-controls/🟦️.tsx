/** ♿️ Authored input labels, disability and blur commits survive the retained React interpreter. */
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { computeAccessibleName, computeAccessibleDescription } from "dom-accessibility-api";
import { afterEach, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/♿️editable-controls/🔣️.json";
import { createInputLedgerV1, inputActionWindowV1, inputActionWithWindowV1 } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import ownership from "../../🧫️fixtures/🪟️input-window/🔣️.json";
import ownershipSchema from "../../🧬️schema/🪟️input-window/🔣️.json";
import { InterpretedUiNode, UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";

afterEach(cleanup);

import crossInputs from "../../🧫️fixtures/↔️input-commits/🔣️.json";
import { inputCommitLaneV1 } from "../../🧱️elements/🗣️Interpreter/🎯️commit-lane/🟦️.ts";
import { LocalDocumentOwnerRegistryV1 } from "../../🧱️elements/🗣️Interpreter/🧭️local-document-owner/🟦️.ts";

for (const order of ["publication-first", "completion-first"] as const) it(`serializes sibling windows and awaits their own publication with ${order}`, async () => {
  const law = crossInputs;
  const owners = new LocalDocumentOwnerRegistryV1();
  const owner = owners.acquire({ pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: law.owner, clientInstanceId: "client-one" });
  const surfaces = [{}, {}];
  const published = law.fields.map((field) => ({ target: field.target, value: field.base, revision: law.baseRevision }));
  const listeners = [new Set<() => void>(), new Set<() => void>()];
  const sent: { index: number; revision: string }[] = [];
  const completions: ((outcome: InputOutcomeV1) => void)[] = [];
  const requests = surfaces.map((surface, index) => inputCommitLaneV1(owner).submit({ read: () => published[index]!, expected: law.fields[index]!.draft, active: () => true, subscribe: (listener) => { listeners[index]!.add(listener); return () => { listeners[index]!.delete(listener); }; }, send: () => { sent.push({ index, revision: published[index]!.revision }); return new Promise<InputOutcomeV1>((resolve) => completions.push(resolve)); } }));
  const done = Promise.all(requests);
  expect(sent).toHaveLength(law.expected.pending);
  const first = () => completions.shift()!({ kind: "applied", inputSeq: 1, commit: { operation: "1", revision: law.firstRevision } });
  const publishFirst = () => { published[0] = { ...published[0]!, value: law.fields[0]!.draft, revision: law.firstRevision }; for (const listener of listeners[0]!) listener(); };
  if (order === "publication-first") { publishFirst(); first(); } else { first(); await Promise.resolve(); publishFirst(); }
  await new Promise((resolve) => queueMicrotask(() => queueMicrotask(() => resolve(undefined))));
  expect(sent).toHaveLength(law.expected.pending);
  expect(listeners[0]!.size).toBe(0);
  expect(listeners[1]!.size).toBe(1);
  published[1] = { ...published[1]!, revision: law.firstRevision };
  for (const listener of listeners[1]!) listener();
  await new Promise((resolve) => queueMicrotask(() => queueMicrotask(() => resolve(undefined))));
  expect(sent).toEqual([{ index: 0, revision: law.baseRevision }, { index: 1, revision: law.firstRevision }]);
  published[1] = { ...published[1]!, value: law.fields[1]!.draft, revision: law.secondRevision };
  for (const listener of listeners[1]!) listener();
  completions.shift()!({ kind: "applied", inputSeq: 2, commit: { operation: "2", revision: law.secondRevision } });
  expect((await done).every((outcome) => outcome.kind === "applied")).toBe(true);
  expect(listeners.map((group) => group.size)).toEqual([0, 0]);
  owners.retireAll();
});

for (const ending of ["retired", "hidden", "foreign", "changed-target", "retired-after-publication", "hidden-after-publication", "foreign-after-publication"] as const) it(`stops a sibling window awaiting publication when ${ending}`, async () => {
  const law = crossInputs;
  const owners = new LocalDocumentOwnerRegistryV1();
  const identity = { pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: law.owner, clientInstanceId: "client-one" };
  const owner = owners.acquire(identity);
  const listeners = [new Set<() => void>(), new Set<() => void>()];
  const published = law.fields.map((field) => ({ target: field.target, value: field.base, revision: law.baseRevision }));
  let visible = true;
  let sent = 0;
  let first: (outcome: InputOutcomeV1) => void = () => {};
  const results = Promise.allSettled(law.fields.map((field, index) => inputCommitLaneV1(owner).submit({ read: () => published[index]!, expected: field.draft, active: () => visible, subscribe: (listener) => { listeners[index]!.add(listener); return () => { listeners[index]!.delete(listener); }; }, send: () => { sent += 1; return new Promise<InputOutcomeV1>((resolve) => { first = resolve; }); } })));
  published[0] = { ...published[0]!, value: law.fields[0]!.draft, revision: law.firstRevision };
  first({ kind: "applied", inputSeq: 1, commit: { operation: "1", revision: law.firstRevision } });
  await new Promise((resolve) => queueMicrotask(() => queueMicrotask(() => resolve(undefined))));
  expect(listeners[1]!.size).toBe(1);
  if (ending.endsWith("-after-publication")) {
    published[1] = { ...published[1]!, revision: law.firstRevision };
    for (const listener of listeners[1]!) listener();
  }
  if (ending.startsWith("retired")) owners.retire(identity);
  else {
    if (ending.startsWith("hidden")) visible = false;
    else if (ending.startsWith("foreign")) published[1] = { ...published[1]!, revision: law.foreignRevision };
    else published[1] = { ...published[1]!, target: "replaced", revision: law.firstRevision };
    for (const listener of listeners[1]!) listener();
  }
  await new Promise((resolve) => queueMicrotask(() => queueMicrotask(() => resolve(undefined))));
  expect(sent).toBe(law.expected.pending);
  expect((await results).map((result) => result.status)).toEqual(["fulfilled", "rejected"]);
  expect(listeners.map((group) => group.size)).toEqual([0, 0]);
  owners.retireAll();
});

it("bounds sibling commits and releases every pending publication subscription on retirement", async () => {
  const law = crossInputs;
  const owners = new LocalDocumentOwnerRegistryV1();
  const identity = { pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: law.owner, clientInstanceId: "client-one" };
  const owner = owners.acquire(identity);
  const surface = {};
  const lane = inputCommitLaneV1(owner);
  expect(inputCommitLaneV1(owner)).toBe(lane);
  let sent = 0;
  const listeners = new Set<() => void>();
  const requests = Array.from({ length: law.expected.maxPending + 1 }, () => lane.submit({ read: () => ({ target: law.fields[0]!.target, value: law.fields[0]!.base, revision: law.baseRevision }), expected: law.fields[0]!.draft, active: () => true, subscribe: (listener) => { listeners.add(listener); return () => { listeners.delete(listener); }; }, send: () => { sent += 1; return new Promise<InputOutcomeV1>(() => {}); } }));
  const finished = Promise.allSettled(requests);
  expect(sent).toBe(law.expected.pending);
  expect(listeners.size).toBe(1);
  owners.retire(identity);
  const results = await finished;
  expect(results.every((result) => result.status === "rejected")).toBe(true);
  expect((results.at(-1) as PromiseRejectedResult).reason.message).toBe("Input commit queue is full");
  expect(listeners.size).toBe(0);
  const next = owners.acquire(identity);
  expect(next).not.toBe(owner);
  expect(inputCommitLaneV1(next)).not.toBe(lane);
  owners.retireAll();
});

for (const mode of ["own", "foreign", "changed-sibling", "refused", "retired", "discarded-sibling"] as const) for (const order of ["publication-first", "completion-first"] as const) it(`serializes different retained input targets with ${order} and ${mode}`, async () => {
  const law = crossInputs;
  const store = new UiDocumentStore("cross-inputs");
  const intents: any[] = [];
  const completions: ((outcome: InputOutcomeV1) => void)[] = [];
  const retirements = new Set<() => void>();
  const owner = { id: law.owner, identity: { pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: law.owner, clientInstanceId: "client-one" }, active: true, subscribeRetirement: (listener: () => void) => { retirements.add(listener); return () => { retirements.delete(listener); }; } };
  const context = { store, localDocumentOwner: owner, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<InputOutcomeV1>((resolve) => completions.push(resolve)); } };
  const publish = (values: readonly string[], revision: string, ordinal: number) => store.loadSnapshot({ surface: "cross-inputs", revision: ordinal, root: 0, layoutEpoch: 0n, nodes: [
    { id: 0, key: "fields", component: { type: "container", role: "section", label: "Fields", defaultOpen: true }, children: [1, 2], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: "Fields" } },
    ...law.fields.map((field, index) => ({ id: index + 1, key: field.target, component: { type: "input", kind: "text", value: values[index], commit: "blur", draftTarget: field.target, publicationRevision: revision }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: field.label }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: index, revision } }] })),
  ] } as any);
  publish(law.fields.map((field) => field.base), law.baseRevision, 1);
  const view = render(createElement(UiNodeView, { store, id: 0, context }));
  const controls = law.fields.map((field) => view.getByRole("textbox", { name: field.label }) as HTMLInputElement);
  controls.forEach((control, index) => { fireEvent.change(control, { target: { value: law.fields[index]!.draft } }); fireEvent.blur(control); });
  expect(intents).toHaveLength(law.expected.pending);
  if (mode === "retired") {
    await act(async () => { owner.active = false; for (const retire of retirements) retire(); });
    expect(intents).toHaveLength(law.expected.retired);
    expect(retirements.size).toBe(0);
    await act(async () => completions.shift()!({ kind: "applied", inputSeq: 1, commit: { operation: "1", revision: law.firstRevision } }));
    expect(intents).toHaveLength(law.expected.retired);
    return;
  }
  if (mode === "refused") {
    await act(async () => completions.shift()!({ kind: "refused", inputSeq: 1, reason: "mutation-rejected", retryable: false }));
    expect(intents).toHaveLength(law.expected.foreign);
    controls.forEach((control, index) => { expect(control.value).toBe(law.fields[index]!.draft); expect(control.getAttribute("aria-invalid")).toBe("true"); });
    return;
  }
  const revision = mode === "foreign" ? law.foreignRevision : law.firstRevision;
  const publishedSibling = mode === "changed-sibling" ? "changed by the first operation" : law.fields[1]!.base;
  if (mode === "discarded-sibling") {
    fireEvent.keyDown(controls[1]!, { key: "Escape" });
    expect(controls[1]!.value).toBe(law.fields[1]!.base);
  }
  const completeFirst = () => completions.shift()!({ kind: "applied", inputSeq: 1, commit: { operation: "1", revision: law.firstRevision } });
  if (order === "publication-first") {
    act(() => publish([law.fields[0]!.draft, publishedSibling], revision, 2));
    expect(intents).toHaveLength(law.expected.pending);
    await act(async () => completeFirst());
  } else {
    await act(async () => completeFirst());
    expect(intents).toHaveLength(law.expected.pending);
    await act(async () => publish([law.fields[0]!.draft, publishedSibling], revision, 2));
  }
  if (mode === "discarded-sibling") {
    expect(intents).toHaveLength(law.expected.cancelled);
    expect(controls[1]!.value).toBe(law.fields[1]!.base);
    expect(controls[1]!.hasAttribute("aria-invalid")).toBe(false);
    expect(retirements.size).toBe(0);
    return;
  }
  if (mode !== "own") {
    expect(intents).toHaveLength(law.expected.foreign);
    expect(controls[1]!.value).toBe(law.fields[1]!.draft);
    expect(controls[1]!.getAttribute("aria-invalid")).toBe("true");
    expect(retirements.size).toBe(0);
    return;
  }
  expect(intents).toHaveLength(law.expected.complete);
  expect(intents[1].args.revision).toBe(law.firstRevision);
  expect(intents[1].args.column).toBe(1);
  expect(intents[1].input).toBe(law.fields[1]!.draft);
  await act(async () => {
    publish(law.fields.map((field) => field.draft), law.secondRevision, 3);
    completions.shift()!({ kind: "applied", inputSeq: 2, commit: { operation: "2", revision: law.secondRevision } });
  });
  controls.forEach((control, index) => { expect(control.value).toBe(law.fields[index]!.draft); expect(control.hasAttribute("aria-invalid")).toBe(false); expect(computeAccessibleName(control)).toBe(law.fields[index]!.label); });
});

for (const kind of ["refused", "superseded"] as const) it(`preserves a draft and stops queued edits when the first input is ${kind}`, async () => {
  const law = fixture.queuedAcknowledgements;
  const store = new UiDocumentStore("refused-draft");
  const intents: unknown[] = [];
  let complete: (value: any) => void = () => {};
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<any>((resolve) => { complete = resolve; }); } };
  store.loadSnapshot({ surface: "refused-draft", revision: 1, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: "text", value: law.base, commit: "blur", draftTarget: law.target }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: "Cell", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: 0, revision: law.revisionBefore } }] }] } as any);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.getByRole("textbox", { name: "Cell" }) as HTMLInputElement;
  fireEvent.change(control, { target: { value: law.first } });
  fireEvent.blur(control);
  fireEvent.change(control, { target: { value: law.second } });
  fireEvent.blur(control);
  await act(async () => complete(kind === "refused" ? { kind, inputSeq: 1, reason: "mutation-rejected", retryable: false } : { kind, inputSeq: 1, by: 2 }));
  expect(intents).toHaveLength(law.commandsAfterRefusal);
  expect(control.value).toBe(law.second);
  expect(control.getAttribute("aria-invalid")).toBe("true");
  expect(computeAccessibleName(control)).toBe("Cell");
  fireEvent.keyDown(control, { key: "Escape" });
  expect(control.value).toBe(law.base);
  expect(control.hasAttribute("aria-invalid")).toBe(false);
});

for (const order of ["publication-first", "settlement-first"] as const) it(`queues explicit commits across changing revision guards with ${order}`, async () => {
  const law = fixture.queuedAcknowledgements;
  const store = new UiDocumentStore("queued-draft");
  const intents: any[] = [];
  const completions: (() => void)[] = [];
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<void>((resolve) => completions.push(resolve)); } };
  const publish = (value: string, revision: string, ordinal: number) => store.loadSnapshot({ surface: "queued-draft", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: "text", value, commit: "blur", draftTarget: law.target }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: "Cell", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: 0, revision } }] }] } as any);
  publish(law.base, law.revisionBefore, 1);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.getByRole("textbox", { name: "Cell" }) as HTMLInputElement;
  fireEvent.change(control, { target: { value: law.first } });
  fireEvent.blur(control);
  fireEvent.change(control, { target: { value: law.second } });
  fireEvent.blur(control);
  expect(intents).toHaveLength(law.commandsWhileFirstPending);
  if (order === "settlement-first") {
    await act(async () => completions.shift()!());
    expect(intents).toHaveLength(law.commandsWhileFirstPending);
    act(() => publish(law.first, law.revisionAfterFirst, 2));
  } else {
    act(() => publish(law.first, law.revisionAfterFirst, 2));
    await act(async () => completions.shift()!());
  }
  expect(control.value).toBe(law.second);
  expect(intents).toHaveLength(law.commandsAfterFirstCompletes);
  expect(intents[1].input).toBe(law.second);
  expect(intents[1].args.revision).toBe(law.revisionAfterFirst);
  act(() => publish(law.second, law.revisionAfterSecond, 3));
  await act(async () => completions.shift()!());
  expect(control.value).toBe(law.second);
  fireEvent.blur(control);
  expect(intents).toHaveLength(law.commandsAfterFirstCompletes);
  expect(computeAccessibleName(control)).toBe("Cell");
});

for (const mode of ["external-guard", "normalized-number"] as const) it(`retains the draft correctly across ${mode}`, async () => {
  const law = fixture.queuedAcknowledgements;
  const numeric = mode === "normalized-number";
  const store = new UiDocumentStore("guarded-draft");
  const intents: any[] = [];
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } };
  const publish = (value: string, revision: string, ordinal: number) => store.loadSnapshot({ surface: "guarded-draft", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: numeric ? "number" : "text", value, commit: "blur", draftTarget: law.target }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: "Cell", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: 0, revision } }] }] } as any);
  publish(numeric ? law.normalizedNumber.base : law.base, law.revisionBefore, 1);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.getByRole(numeric ? "spinbutton" : "textbox", { name: "Cell" }) as HTMLInputElement;
  fireEvent.change(control, { target: { value: numeric ? law.normalizedNumber.typed : law.first } });
  if (numeric) {
    fireEvent.blur(control);
    expect(intents[0].input).toBe(Number(law.normalizedNumber.typed));
    await act(async () => publish(law.normalizedNumber.published, law.revisionAfterFirst, 2));
    expect(control.hasAttribute("aria-invalid")).toBe(false);
    fireEvent.blur(control);
    expect(intents).toHaveLength(1);
  } else {
    act(() => publish(law.base, law.revisionAfterFirst, 2));
    expect(control.value).toBe(law.first);
    expect(control.getAttribute("aria-invalid")).toBe("true");
    fireEvent.blur(control);
    expect(intents).toHaveLength(0);
  }
  expect(computeAccessibleName(control)).toBe("Cell");
});

for (const test of fixture.cases) for (const locale of ["en","de"] as const) for (const disabled of [false,true]) for (const gesture of fixture.commitGestures) {
  it(`${test.kind} preserves its ${locale} accessible label and disabled=${disabled} on ${gesture}`, () => {
    const component = test.kind === "select" ? { type: "select", value: test.value, items: [{ value: "normal",label: "Normal" }] } : { type: "input",kind: test.kind,value: test.value,commit: "blur" };
    const record = { id: 1,key: "control",component,disabled,children: [],layout: { kind: "leaf",width: "hug",height: "hug" },style: { variant: "plain",size: "md",density: "standard",tone: "neutral",emphasis: "regular" },accessibility: { label: test.label[locale],description: null,live: "off",shortcut: null,hidden: false },bindings: [{ trigger: test.kind === "select" ? "change" : "commit",action: { scope: "drawing-play",name: "patchLayers",version: 1 } }] };
    const store = new UiDocumentStore("editable-controls");
    store.loadSnapshot({ surface: "editable-controls",revision: 1,root: 1,nodes: [record],layoutEpoch: 0n } as any);
    const intents: any[] = [];
    const view = render(createElement(UiNodeView,{ store,id: 1,context: { store,onAction: () => {},onIntent: (intent: unknown) => { intents.push(intent); } } }));
    const control = view.container.querySelector("input, textarea, [role=combobox]") as HTMLInputElement;
    expect(computeAccessibleName(control)).toBe(test.label[locale]);
    expect(control.disabled).toBe(disabled);
    if (!disabled && test.draft) {
      fireEvent.blur(control);
      expect(intents).toHaveLength(fixture.commitCounts.untouched);
      fireEvent.change(control,{ target: { value: test.draft } });
      expect(intents).toHaveLength(0);
      if (gesture === "enter") fireEvent.keyDown(control, { key: "Enter", ctrlKey: test.kind === "longText" });
      else fireEvent.blur(control);
      expect(intents).toHaveLength(fixture.commitCounts.edited);
      fireEvent.blur(control);
      expect(intents).toHaveLength(fixture.commitCounts.repeatedBlur);
      expect(intents[0].trigger).toBe("commit");
      expect(intents[0].input).toEqual(test.kind === "number" ? Number(test.draft) : test.draft);
    }
  });
}

it("discards an uncommitted draft when the command target changes at the same published value", () => {
  const test = fixture.selectionChange;
  const store = new UiDocumentStore("selection-draft");
  const intents: unknown[] = [];
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } };
  const publish = (layer: string, revision: number) => store.loadSnapshot({ surface: "selection-draft", revision, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "position", component: { type: "input", kind: "number", value: test.value, commit: "blur" }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: "Mask X position", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: "raster-play", name: "patchLayers", version: 1 }, args: { field: "maskX", layerIds: [layer] } }] }] } as any);
  publish(test.first, 1);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.container.querySelector("input")!;
  fireEvent.change(control, { target: { value: test.draft } });
  expect(control.value).toBe(test.draft);
  act(() => publish(test.next, 2));
  expect(control.value).toBe(test.value);
  fireEvent.blur(control);
  expect(intents).toHaveLength(0);
});

import receiptLaw from "../../../../../🧫️fixtures/⌨️input-publication-receipt.json";
import type { InputOutcomeV1 } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";

for (const locale of ["en", "de"] as const) for (const order of ["publication-first", "completion-first"] as const) for (const foreign of [false, true]) it(`binds queued input to its native receipt with ${order}, foreign=${foreign} and locale=${locale}`, async () => {
  const law = receiptLaw;
  const store = new UiDocumentStore("causal-draft");
  const intents: any[] = [];
  let complete: (outcome: InputOutcomeV1) => void = () => {};
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<InputOutcomeV1>((resolve) => { complete = resolve; }); } };
  const publish = (value: string, revision: string, ordinal: number) => store.loadSnapshot({ surface: "causal-draft", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: "text", value, commit: "blur", draftTarget: law.target, publicationRevision: revision }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: "Cell", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: 0, revision } }] }] } as any);
  publish(law.base.value, law.base.revision, 1);
  const i18n = createShellI18nInstance(locale);
  receiptLocales.push(i18n);
  const view = render(createElement(I18nextProvider, { i18n }, createElement(UiNodeView, { store, id: 1, context })));
  const control = view.getByRole("textbox", { name: "Cell" }) as HTMLInputElement;
  for (const value of law.queued) { fireEvent.change(control, { target: { value } }); fireEvent.blur(control); }
  const publication = foreign ? law.foreignPublication : law.localPublication;
  const applied = { kind: "applied", inputSeq: 1, commit: law.localCompletion } as const;
  if (order === "completion-first") {
    await act(async () => complete(applied));
    expect(intents).toHaveLength(law.expected.dispatchesBeforeLocalPublication);
    if (!foreign) {
      fireEvent.keyDown(control, { key: "Escape" });
      expect(control.value).toBe(law.base.value);
      fireEvent.change(control, { target: { value: law.queued[1] } });
      fireEvent.blur(control);
      expect(intents).toHaveLength(law.expected.dispatchesBeforeLocalPublication);
    }
    act(() => publish(publication.value, publication.revision, 2));
  } else {
    act(() => publish(publication.value, publication.revision, 2));
    expect(control.hasAttribute("aria-invalid")).toBe(false);
    await act(async () => complete(applied));
  }
  expect(intents).toHaveLength(foreign ? law.expected.dispatchesBeforeLocalPublication : law.expected.dispatchesAfterLocalPublication);
  expect(control.value).toBe(law.queued[1]);
  expect(control.hasAttribute("aria-invalid")).toBe(foreign);
  expect(computeAccessibleName(control)).toBe("Cell");
  if (foreign) {
    fireEvent.blur(control);
    expect(intents).toHaveLength(1);
    const labels = fixture.draftRecovery[locale];
    expect(computeAccessibleDescription(control)).toBe(labels.inputDraftConflict);
    fireEvent.click(view.getByRole("button", { name: labels.inputDraftConflict }));
    expect(view.getByRole("dialog").textContent).toContain(labels.inputDraftConflict);
    fireEvent.click(view.getByRole("button", { name: labels.inputDraftDiscard }));
    expect(control.value).toBe(publication.value);
    expect(control.hasAttribute("aria-invalid")).toBe(false);
  } else expect(intents[1].args.revision).toBe(law.localCompletion.revision);
});

import { I18nextProvider } from "react-i18next";
import { createShellI18nInstance, disposeShellI18nInstance } from "@semio-tech/ui-react";
const receiptLocales: ReturnType<typeof createShellI18nInstance>[] = [];
afterEach(() => { for (const instance of receiptLocales.splice(0)) disposeShellI18nInstance(instance); });

it("retained input commits carry their own window even when another window has focus", () => {
  expect(new Ajv2020({ strict: true }).compile(ownershipSchema)(ownership)).toBe(true);
  const store = new UiDocumentStore("window-owner");
  store.loadSnapshot({ surface: "window-owner", revision: 1, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: "text", value: ownership.value, commit: "blur", draftTarget: "cell:0:0" }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: ownership.label, description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: { scope: ownership.controller, name: ownership.action, version: 1 }, args: { row: 0, column: 0 } }] }] } as any);
  const commits: { intent: any; windowId?: string }[] = [];
  const view = render(createElement("div", {}, createElement("input", { "aria-label": ownership.focused }), createElement(InterpretedUiNode, { store, windowId: ownership.owner, onAction: () => {}, onIntent: (intent: any, windowId?: string) => { commits.push({ intent, windowId }); } } as any)));
  const input = view.getByRole("textbox", { name: ownership.label }) as HTMLInputElement;
  input.focus();
  fireEvent.change(input, { target: { value: ownership.draft } });
  (view.getByRole("textbox", { name: ownership.focused }) as HTMLInputElement).focus();
  expect(commits).toHaveLength(1);
  expect(commits[0]!.windowId).toBe(ownership.owner);
  expect(commits[0]!.intent.input).toBe(ownership.draft);
});

it("window provenance routes scene and retained commands without rewriting their domain arguments", () => {
  const args = { row: 0, column: 0, windowId: ownership.focused };
  const action = { controllerId: ownership.controller, action: ownership.action, args, provenance: { windowId: ownership.focused, origin: "gesture" as const, causedBy: 7 } };
  const owned = inputActionWithWindowV1(action, ownership.owner);
  expect(inputActionWindowV1(owned, ownership.focused)).toBe(ownership.owner);
  expect(owned.args).toBe(args);
  expect(action.provenance.windowId).toBe(ownership.focused);
  const entry = createInputLedgerV1().issue(owned, { windowId: ownership.focused });
  expect(entry.provenance).toMatchObject({ windowId: ownership.owner, origin: "gesture", causedBy: 7 });
  expect(inputActionWindowV1({ controllerId: ownership.controller, action: ownership.action, args }, ownership.owner)).toBe(ownership.focused);
  expect(inputActionWindowV1({ controllerId: ownership.controller, action: ownership.action }, ownership.focused)).toBe(ownership.focused);
});

for (const owned of [false, true]) for (const ending of ["applied", "refused"] as const) it(`bounds same-control explicit commits and preserves the newest draft with owned=${owned}, ending=${ending}`, async () => {
  const law = fixture.queuedAcknowledgements;
  const store = new UiDocumentStore("bounded-local-draft");
  const registry = new LocalDocumentOwnerRegistryV1();
  const localDocumentOwner = owned ? registry.acquire({ pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: "bounded-local-draft", clientInstanceId: "one" }) : null;
  const intents: any[] = [];
  const completions: ((value: InputOutcomeV1) => void)[] = [];
  const context = { store, localDocumentOwner, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<InputOutcomeV1>((resolve) => completions.push(resolve)); } };
  const publish = (value: string, revision: string, ordinal: number) => store.loadSnapshot({ surface: "bounded-local-draft", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: "text", value, commit: "blur", draftTarget: law.target, publicationRevision: revision }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: "Cell" }, bindings: [{ trigger: "commit", action: { scope: "csv", name: "set-cell", version: 1 }, args: { row: 0, column: 0, revision } }] }] } as any);
  publish(law.base, crossInputs.baseRevision, 1);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.getByRole("textbox", { name: "Cell" }) as HTMLInputElement;
  for (let index = 0; index < law.admission.maxPending; index += 1) { fireEvent.change(control, { target: { value: `${law.first}${index}` } }); fireEvent.blur(control); }
  expect(intents).toHaveLength(1);
  expect(control.hasAttribute("aria-invalid")).toBe(false);
  fireEvent.change(control, { target: { value: law.admission.overflowDraft } });
  fireEvent.blur(control);
  expect(control.getAttribute("aria-invalid")).toBe("true");
  expect(control.value).toBe(law.admission.overflowDraft);
  await act(async () => {
    if (ending === "applied") publish(`${law.first}0`, crossInputs.firstRevision, 2);
    completions.shift()!(ending === "applied" ? { kind: "applied", inputSeq: 1, commit: { operation: "1", revision: crossInputs.firstRevision } } : { kind: "refused", inputSeq: 1, reason: "mutation-rejected", retryable: false });
  });
  expect(intents).toHaveLength(1);
  expect(control.value).toBe(law.admission.overflowDraft);
  expect(control.getAttribute("aria-invalid")).toBe("true");
  fireEvent.keyDown(control, { key: "Escape" });
  expect(control.value).toBe(ending === "applied" ? `${law.first}0` : law.base);
  expect(control.hasAttribute("aria-invalid")).toBe(false);
  fireEvent.change(control, { target: { value: law.admission.recoveredDraft } });
  fireEvent.blur(control);
  expect(intents).toHaveLength(2);
  expect(intents[1].input).toBe(law.admission.recoveredDraft);
  await act(async () => registry.retireAll());
});

for (const ending of ["applied", "refused", "rejected"] as const) it(`keeps a retained button busy through ${ending} and prevents duplicate stale dispatch`, async () => {
  const law = fixture.queuedAcknowledgements;
  const store = new UiDocumentStore("button-admission");
  const intents: any[] = [];
  let complete: (outcome: InputOutcomeV1) => void = () => {};
  let reject: (reason: Error) => void = () => {};
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<InputOutcomeV1>((resolve, fail) => { complete = resolve; reject = fail; }); } };
  const publish = (revision: string, ordinal: number) => store.loadSnapshot({ surface: "button-admission", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "add-row", component: { type: "button", label: law.button.label, icon: "plus" }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: law.button.label }, bindings: [{ trigger: "activate", action: { scope: "csv", name: "add-row", version: 1 }, args: { revision } }] }] } as any);
  publish(crossInputs.baseRevision, 1);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const button = view.getByRole("button", { name: law.button.label }) as HTMLButtonElement;
  fireEvent.click(button);
  fireEvent.click(button);
  expect(intents).toHaveLength(law.button.pendingDispatches);
  expect(button.disabled).toBe(true);
  expect(button.getAttribute("aria-busy")).toBe("true");
  act(() => publish(crossInputs.firstRevision, 2));
  expect(button.disabled).toBe(true);
  await act(async () => {
    if (ending === "rejected") reject(new Error("transport ended"));
    else complete(ending === "applied" ? { kind: "applied", inputSeq: 1, commit: { operation: "1", revision: crossInputs.firstRevision } } : { kind: "refused", inputSeq: 1, reason: "mutation-rejected", retryable: false });
  });
  expect(button.disabled).toBe(false);
  expect(button.hasAttribute("aria-busy")).toBe(false);
  fireEvent.click(button);
  expect(intents).toHaveLength(law.button.settledDispatches);
  expect(intents[1].args.revision).toBe(crossInputs.firstRevision);
});

it("a retired button completion cannot release the replacement owner's pending action", async () => {
  const law = fixture.queuedAcknowledgements.button;
  const store = new UiDocumentStore("button-owner");
  store.loadSnapshot({ surface: "button-owner", revision: 1, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "add-row", component: { type: "button", label: law.label, icon: "plus" }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: law.label }, bindings: [{ trigger: "activate", action: { scope: "csv", name: "add-row", version: 1 } }] }] } as any);
  const owners = new LocalDocumentOwnerRegistryV1();
  const identity = { pluginId: "stdio", appId: "csv", sessionInstanceId: 1, runtimeKey: "button-owner", clientInstanceId: "one" };
  const first = owners.acquire(identity);
  const completions: (() => void)[] = [];
  const context = { store, localDocumentOwner: first, onAction: () => {}, onIntent: () => new Promise<void>((resolve) => completions.push(resolve)) };
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const button = view.getByRole("button", { name: law.label }) as HTMLButtonElement;
  fireEvent.click(button);
  owners.retire(identity);
  view.rerender(createElement(UiNodeView, { store, id: 1, context: { ...context, localDocumentOwner: owners.acquire(identity) } }));
  expect(button.disabled).toBe(false);
  fireEvent.click(button);
  expect(completions).toHaveLength(law.settledDispatches);
  await act(async () => completions[0]!());
  expect(button.disabled).toBe(true);
  await act(async () => completions[1]!());
  expect(button.disabled).toBe(false);
  owners.retireAll();
});

for (const disabled of [false, true]) it(`retained buttons read the current publication at activation before React commits, disabled=${disabled}`, () => {
  const law = fixture.queuedAcknowledgements.button;
  const store = new UiDocumentStore("button-fresh-binding");
  const intents: any[] = [];
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } };
  const publish = (revision: string, ordinal: number, disabled: boolean) => store.loadSnapshot({ surface: "button-fresh-binding", revision: ordinal, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "add-row", component: { type: "button", label: law.label, icon: "plus" }, disabled, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: law.label }, bindings: [{ trigger: "activate", action: { scope: "csv", name: "add-row", version: 1 }, args: { revision } }] }] } as any);
  publish(crossInputs.baseRevision, 1, false);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const button = view.getByRole("button", { name: law.label });
  act(() => { publish(crossInputs.firstRevision, 2, disabled); fireEvent.click(button); });
  expect(intents).toHaveLength(disabled ? 0 : 1);
  if (!disabled) expect(intents[0].args.revision).toBe(crossInputs.firstRevision);
});

import dispositionLaw from "../../🧫️fixtures/🚦️input-draft-disposition/🔣️.json";

for (const law of dispositionLaw.cases) it(law.name, async () => {
  const store = new UiDocumentStore("history-refusal");
  const intents: unknown[] = [];
  let complete: (value: any) => void = () => {};
  const context = { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); return new Promise<any>((resolve) => { complete = resolve; }); } };
  store.loadSnapshot({ surface: "history-refusal", revision: 1, root: 1, layoutEpoch: 0n, nodes: [{ id: 1, key: "cell", component: { type: "input", kind: law.kind, value: law.base, commit: "blur", draftTarget: "cell" }, disabled: false, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: "Cell" }, bindings: [{ trigger: "commit", action: { scope: "draw", name: "rename", version: 1 }, args: {} }] }] } as any);
  const view = render(createElement(UiNodeView, { store, id: 1, context }));
  const control = view.container.querySelector<HTMLInputElement | HTMLTextAreaElement>("input,textarea")!;
  fireEvent.change(control, { target: { value: law.submitted } });
  fireEvent.blur(control);
  if ("cancelled" in law && law.cancelled) fireEvent.keyDown(control, { key: "Escape" });
  if (law.draft !== law.submitted) fireEvent.change(control, { target: { value: law.draft } });
  await act(async () => complete({ kind: "refused", inputSeq: 1, reason: "dispatch-failed", retryable: false, draftDisposition: law.disposition, diagnostic: { code: "timeTravel.frozen", message: "unparsed diagnostic" } }));
  expect(control.value).toBe(law.expected);
  expect(control.hasAttribute("aria-invalid")).toBe(law.conflicted);
  expect(computeAccessibleName(control)).toBe("Cell");
  expect(intents).toHaveLength(1);
});

import inspectorControls from "../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧫️fixtures/🔣️.json";
import widgetEdits from "../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json";
import inspectorTerminology from "../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗣️terminology.json";
import Ajv from "ajv";
import widgetInputSchema from "../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧬️schema/🔣️.json";
import { editInputValue, editCollectionValue } from "../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🟦️.ts";
 
for (const locale of ["en", "de"] as const) for (const gesture of inspectorControls.reactCommits.gestures) it(`retains inspector scalar and selected BRep labels and commands in ${locale} on ${gesture}`, () => {
  const law = inspectorControls.reactCommits;
  const scalarRows = inspectorControls.controls.map((control) => ({ ...control, widgetId: "typed", channel: control.port, draft: law.drafts[control.type === "toggle" ? "toggle" : control.kind as "number" | "longText"], current: inspectorControls.ports.find((port) => port.name === control.port)!.default, types: inspectorControls.ports.find((port) => port.name === control.port)!.valueTypes }));
  const selectedRows = inspectorControls.selectedBrepControls.cases.flatMap((entry) => entry.controls.map((control) => ({ ...control, type: "input", component: null, widgetId: entry.id, draft: String(control.committedValue), current: { $schema: "number", value: control.value }, types: ["number"] })));
  for (const control of [...scalarRows, ...selectedRows]) {
    const args = { widgetId: control.widgetId, channel: control.channel, ...(control.component ? { component: control.component } : {}) };
    const component = control.type === "toggle" ? { type: "toggle", appearance: "checkbox", on: true } : { type: "input", kind: control.kind, value: String(control.value), commit: "blur" };
    const record = { id: 1, key: "inspector-control", component, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label: control[locale] }, bindings: [{ trigger: control.trigger, action: { scope: law.scope, name: law.action, version: 1 }, args }] };
    const store = new UiDocumentStore("inspector-controls");
    store.loadSnapshot({ surface: "inspector-controls", revision: 1, root: 1, layoutEpoch: 0n, nodes: [record] } as any);
    const intents: any[] = [];
    const view = render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } } }));
    const element = view.getByRole(control.type === "toggle" ? "checkbox" : control.kind === "number" ? "spinbutton" : "textbox", { name: control[locale] }) as HTMLInputElement;
    expect(computeAccessibleName(element)).toBe(control[locale]);
    element.focus();
    expect(document.activeElement).toBe(element);
    if (control.type === "toggle") fireEvent.click(element);
    else {
      fireEvent.blur(element);
      expect(intents).toHaveLength(law.counts.untouched);
      fireEvent.change(element, { target: { value: control.draft } });
      expect(intents).toHaveLength(law.counts.draft);
      if (gesture === "enter") fireEvent.keyDown(element, { key: "Enter", ctrlKey: control.kind === "longText" });
      else fireEvent.blur(element);
    }
    expect(intents).toHaveLength(law.counts.committed);
    expect(intents[0].trigger).toBe(control.trigger);
    expect(intents[0].action).toEqual({ scope: law.scope, name: law.action, version: 1 });
    expect(intents[0].args).toEqual(args);
    expect(intents[0].input).toBe(control.type === "toggle" ? control.draft : control.kind === "number" ? Number(control.draft) : control.draft);
    const edited = editInputValue(control.types, control.current, String(intents[0].input), control.component ?? undefined);
    expect(JSON.parse(JSON.stringify(edited))).toEqual(control.component ? { ...control.current, [control.component]: Number(control.draft) } : { $schema: control.type === "toggle" ? "boolean" : control.kind === "number" ? "number" : "text", value: control.type === "toggle" ? control.draft : control.kind === "number" ? JSON.parse(String(control.draft)) : control.draft });
    fireEvent.blur(element);
    expect(intents).toHaveLength(law.counts.repeated);
    view.unmount();
  }
  console.log(`[DEBUG] inspector scalar ${locale}/${gesture}: controls=${scalarRows.length + selectedRows.length} actualUiNodeView=true independentAccessibleNames=true`);
});

for (const locale of ["en", "de"] as const) it(`retains ordered inspector collection commands and independent expected values in ${locale}`, () => {
  const validate = new Ajv({ strict: false }).compile(widgetInputSchema);
  const law = inspectorControls.reactCommits;
  const cases = widgetEdits.collections.filter((entry) => !("error" in entry) && (entry.command.operation === "set" || entry.command.value === "") && (entry.command.operation !== "move" || Math.abs(entry.command.destination! - entry.command.index) === 1));
  for (const entry of cases) {
    const port = inspectorControls.ports.find((port) => port.itemTypes?.[0] === entry.types[0])!;
    const editable = entry.command.operation === "set";
    const labelRow = inspectorControls.collectionControls.find((control) => control.port === port.name && (!editable || (control.component ?? undefined) === ("component" in entry.command ? entry.command.component : undefined)))!;
    const itemLabel = (editable ? labelRow[locale] : labelRow[locale].replace(/ [xyz]$/, "")).replace(" 1", ` ${entry.command.index + 1}`);
    const termKey = entry.command.operation === "move" ? entry.command.destination! < entry.command.index ? "input_list_up" : "input_list_down" : entry.command.operation === "add" ? "input_list_add" : "input_list_remove";
    const title = itemLabel.split(` ${inspectorTerminology.labels.input_list_item[locale === "en" ? "nativeEn" : "nativeDe"]} `)[0];
    const label = editable ? itemLabel : `${entry.command.operation === "add" ? title : itemLabel}: ${inspectorTerminology.labels[termKey][locale === "en" ? "nativeEn" : "nativeDe"]}`;
    const kind = labelRow.type === "toggle" ? "toggle" : labelRow.kind;
    const current = (entry.current as any)?.[String(entry.command.index)];
    const args = { widgetId: "typed", channel: port.name, ...entry.command };
    const component = editable ? kind === "toggle" ? { type: "toggle", appearance: "checkbox", on: current.value } : { type: "input", kind, value: String(labelRow.component ? current[labelRow.component] : current.value), commit: "blur" } : { type: "button", label, icon: "" };
    const record = { id: 1, key: entry.id, component, children: [], layout: { kind: "leaf", width: "hug", height: "hug" }, style: {}, accessibility: { label }, bindings: [{ trigger: editable ? kind === "toggle" ? "change" : "commit" : "activate", action: { scope: law.scope, name: law.action, version: 1 }, args }] };
    const store = new UiDocumentStore("inspector-collection");
    store.loadSnapshot({ surface: "inspector-collection", revision: 1, root: 1, layoutEpoch: 0n, nodes: [record] } as any);
    const intents: any[] = [];
    const view = render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } } }));
    const element = view.getByRole(!editable ? "button" : kind === "toggle" ? "checkbox" : kind === "number" ? "spinbutton" : "textbox", { name: label });
    expect(computeAccessibleName(element)).toBe(label);
    element.focus();
    expect(document.activeElement).toBe(element);
    if (!editable || kind === "toggle") fireEvent.click(element);
    else { fireEvent.change(element, { target: { value: entry.command.value } }); fireEvent.keyDown(element, { key: "Enter", ctrlKey: kind === "longText" }); }
    expect(intents).toHaveLength(1);
    expect(intents[0].args).toEqual(args);
    const command = { ...intents[0].args, value: editable ? String(intents[0].input) : entry.command.value };
    expect(validate(command), JSON.stringify(validate.errors)).toBe(true);
    expect(editCollectionValue(entry.types, entry.current, command, entry.cardinality)).toEqual(entry.expected);
    expect(JSON.parse(JSON.stringify(entry.expected))).toEqual(entry.expected);
    view.unmount();
  }
  console.log(`[DEBUG] inspector collections ${locale}: cases=${cases.length} actualUiNodeView=true independentAjvJson=true`);
});
