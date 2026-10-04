/** 🧾️ Actual Source host controls preserve refused and foreign drafts; Testing Library and accessible descriptions are the independent DOM oracle. */
import { act, cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { createElement, useState } from "react";
import { getByRole, queryByRole } from "@testing-library/dom";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import type { ComponentSceneHostProps } from "@semio-tech/framework";
import * as sessionLoader from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import * as nodeGraph from "../../../🕸️NodeGraph/🟦️.tsx";
import fixture from "../../🧫️fixtures/📝️explicit-draft/🔣️.json";
import law from "../../🧫️fixtures/📝️explicit-draft/⚖️lifecycle.json";
import retention from "../../🧫️fixtures/📝️explicit-draft/🧭️retention.json";
import { LocalDocumentOwnerContext, LocalDocumentOwnerRegistryV1, LocalDocumentWindowContext, type LocalDocumentOwnerV1 } from "../../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { retainedTextEditorDraftCountV1, TextEditorHost } from "../../🟦️.tsx";

beforeEach(() => { vi.spyOn(nodeGraph, "useClient").mockReturnValue(false); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

const node = (settings: typeof fixture.cases[number]["settings"], buffer = law.base, revision = law.baseRevision, surfaceId = law.source): ComponentSceneHostProps["node"] => ({ type: "componentScene", componentKind: "text-editor", controllerId: "document", surfaceId, textEditor: { language: "json", buffer, settingsJson: JSON.stringify({ ...settings, publicationRevision: revision }) } });
const typeDraft = (control: HTMLTextAreaElement, text = law.draft) => { fireEvent.change(control, { target: { value: text } }); fireEvent.blur(control); };
const applied = { kind: "applied", inputSeq: 1, commit: { operation: law.operation, revision: law.ownRevision } };

function RetentionDisclosure({ owner, source, onAction }: { readonly owner: LocalDocumentOwnerV1; readonly source: ComponentSceneHostProps["node"]; readonly onAction: ComponentSceneHostProps["onAction"] }) {
  const [open, setOpen] = useState(true);
  return createElement(LocalDocumentOwnerContext.Provider, { value: owner },
    createElement(LocalDocumentWindowContext.Provider, { value: retention.windowId },
      createElement("button", { type: "button", onClick: () => setOpen(value => !value) }, open ? "Collapse source" : "Expand source"),
      open ? createElement(TextEditorHost, { node: source, onAction }) : null,
    ),
  );
}

for (const row of fixture.cases) {
  for (const failure of law.failures) test(`Source Apply preserves ${row.id} draft and offers accessible recovery after ${failure}`, async () => {
    const onAction = vi.fn(async () => {
      if (failure === "throw") throw new Error("transport details must stay out of end-user text");
      return failure === "refused" ? { kind: "refused", inputSeq: 1, reason: "dispatch-failed", retryable: true } : failure === "superseded" ? { kind: "superseded", inputSeq: 1, by: 2 } : { kind: "applied", inputSeq: 1 };
    });
    const view = render(createElement(TextEditorHost, { node: node(row.settings), onAction }));
    const control = view.getByRole("textbox") as HTMLTextAreaElement;
    typeDraft(control);
    await act(async () => { fireEvent.click(view.getByRole("button", { name: row.settings.applyLabel, exact: true })); });
    expect(onAction).toHaveBeenCalledTimes(1);
    expect(control.value).toBe(law.draft);
    expect(view.getByRole("alert").textContent).toBe(row.settings.failedLabel);
    expect(getByRole(view.container, "textbox", { description: row.settings.failedLabel })).toBe(control);
    expect(control.getAttribute("aria-invalid")).toBe("true");
    expect((view.getByRole("button", { name: row.settings.applyLabel, exact: true }) as HTMLButtonElement).disabled).toBe(false);
    fireEvent.click(view.getByRole("button", { name: row.settings.discardLabel, exact: true }));
    expect(control.value).toBe(law.base);
    expect(view.container.querySelector('[role="alert"]')).toBeNull();
    expect(control.hasAttribute("aria-invalid")).toBe(false);
  });

  for (const order of law.orders) for (const foreign of [false, true]) test(`Source Apply ${row.id} waits for exact native authority: ${order}, foreign=${foreign}`, async () => {
    let complete: (outcome: unknown) => void = () => {};
    const onAction = vi.fn(() => new Promise((resolve) => { complete = resolve; }));
    const view = render(createElement(TextEditorHost, { node: node(row.settings), onAction }));
    const control = view.getByRole("textbox") as HTMLTextAreaElement;
    const apply = () => view.getByRole("button", { name: row.settings.applyLabel, exact: true }) as HTMLButtonElement;
    typeDraft(control);
    fireEvent.click(apply());
    fireEvent.click(apply());
    expect(onAction).toHaveBeenCalledTimes(1);
    expect(view.getByRole("status").textContent).toBe(row.settings.applyingLabel);
    const published = foreign ? law.draft : law.normalized;
    const revision = foreign ? law.foreignRevision : law.ownRevision;
    const publish = () => view.rerender(createElement(TextEditorHost, { node: node(row.settings, published, revision), onAction }));
    if (order === "publication-first") {
      publish();
      expect(control.value).toBe(law.draft);
      expect(apply().disabled).toBe(true);
      await act(async () => complete(applied));
    } else {
      await act(async () => complete(applied));
      expect(apply().disabled).toBe(true);
      expect(control.value).toBe(law.draft);
      publish();
    }
    expect(view.container.querySelector('[role="status"]')).toBeNull();
    expect(control.value).toBe(foreign ? law.draft : law.normalized);
    expect(view.container.querySelector("[data-dirty]")?.getAttribute("data-dirty")).toBe(foreign ? "true" : "false");
    if (foreign) {
      expect(getByRole(view.container, "textbox", { description: row.settings.conflictLabel })).toBe(control);
      expect(apply().disabled).toBe(true);
      fireEvent.click(view.getByRole("button", { name: row.settings.discardLabel, exact: true }));
      expect(view.container.querySelector('[role="alert"]')).toBeNull();
    }
  });
}

test("Source Apply settlement cannot clear the next mounted document draft", async () => {
  const settings = fixture.cases[0]!.settings;
  let complete: (outcome: unknown) => void = () => {};
  const onAction = vi.fn(() => new Promise((resolve) => { complete = resolve; }));
  const view = render(createElement(TextEditorHost, { node: node(settings), onAction }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true }));
  view.rerender(createElement(TextEditorHost, { node: node(settings, law.normalized, law.foreignRevision, law.otherSource), onAction }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement, law.newerDraft);
  await act(async () => complete(applied));
  expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.newerDraft);
  expect(view.container.querySelector('[role="alert"]')).toBeNull();
  expect((view.getByRole("button", { name: settings.applyLabel, exact: true }) as HTMLButtonElement).disabled).toBe(false);
});

test("Source announces a localized syntax refusal with the native span and path", async () => {
  const settings = fixture.cases[0]!.settings;
  const onAction = vi.fn(async () => ({ kind: "refused", inputSeq: 1, reason: "dispatch-failed", retryable: true, diagnostic: { code: "snapshot-edit.invalid-source", message: "unexpected byte at byte 24", span: { line: 3, column: 3, length: 1 }, params: { path: "/rows/2/name" } } } as const));
  const view = render(createElement(TextEditorHost, { node: node(settings), onAction }));
  const control = view.getByRole("textbox") as HTMLTextAreaElement;
  typeDraft(control);
  await act(async () => { fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true })); });
  const message = "The source contains invalid syntax. Line 3, Column 3, Path /rows/2/name";
  expect(view.getByRole("alert").textContent).toBe(message);
  expect(getByRole(view.container, "textbox", { description: message })).toBe(control);
  expect(control.getAttribute("aria-invalid")).toBe("true");
});

test("Source Cancel targets its exact admitted operation once and preserves the draft", async () => {
  const settings = fixture.cases[0]!.settings;
  let complete: (outcome: unknown) => void = () => {};
  const cancel = vi.fn(async () => ({ kind: "applied", inputSeq: 2 } as const));
  const onAction = vi.fn((_action, lifecycle) => new Promise(resolve => {
    lifecycle?.started({ operationId: law.operation, generation: law.operationGeneration, cancel });
    complete = resolve;
  }));
  const view = render(createElement(TextEditorHost, { node: node(settings), onAction }));
  const control = view.getByRole("textbox") as HTMLTextAreaElement;
  typeDraft(control);
  fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true }));
  const cancelButton = view.getByRole("button", { name: settings.cancelLabel, exact: true }) as HTMLButtonElement;
  expect(cancelButton.disabled).toBe(false);
  await act(async () => { fireEvent.click(cancelButton); });
  expect(cancel).toHaveBeenCalledTimes(1);
  expect(cancelButton.disabled).toBe(true);
  await act(async () => complete(applied));
  expect(control.value).toBe(law.draft);
  expect(view.container.querySelector("[data-dirty]")?.getAttribute("data-dirty")).toBe("true");
  expect(queryByRole(view.container, "button", { name: settings.cancelLabel })).toBeNull();
});

test("Source re-enables exact cancellation after its cancellation dispatch is refused", async () => {
  const settings = fixture.cases[0]!.settings;
  const cancel = vi.fn(async () => ({ kind: "refused", inputSeq: 2, reason: "dispatch-failed", retryable: true } as const));
  const onAction = vi.fn((_action, lifecycle) => new Promise(() => lifecycle?.started({ operationId: law.operation, generation: law.operationGeneration, cancel })));
  const view = render(createElement(TextEditorHost, { node: node(settings), onAction }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true }));
  const cancellation = () => view.getByRole("button", { name: settings.cancelLabel, exact: true }) as HTMLButtonElement;
  await act(async () => { fireEvent.click(cancellation()); });
  expect(cancellation().disabled).toBe(false);
  expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.draft);
  await act(async () => { fireEvent.click(cancellation()); });
  expect(cancel).toHaveBeenCalledTimes(2);
});

test("Source disclosure retains a dirty local draft for the exact live document owner", () => {
  const registry = new LocalDocumentOwnerRegistryV1(retention.limits.owners);
  const owner = registry.acquire(retention.owner);
  const settings = fixture.cases[0]!.settings;
  const view = render(createElement(RetentionDisclosure, { owner, source: node(settings), onAction: () => {} }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  expect(retainedTextEditorDraftCountV1(owner)).toBe(1);
  fireEvent.click(view.getByRole("button", { name: "Collapse source" }));
  expect(queryByRole(view.container, "textbox")).toBeNull();
  fireEvent.click(view.getByRole("button", { name: "Expand source" }));
  expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.draft);
});

test("Source Apply settles while its disclosure is unmounted and acknowledges only its exact publication", async () => {
  const registry = new LocalDocumentOwnerRegistryV1();
  const owner = registry.acquire(retention.owner);
  const settings = fixture.cases[0]!.settings;
  let complete: (outcome: unknown) => void = () => {};
  const onAction = vi.fn(() => new Promise(resolve => { complete = resolve; }));
  const view = render(createElement(RetentionDisclosure, { owner, source: node(settings), onAction }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true }));
  fireEvent.click(view.getByRole("button", { name: "Collapse source" }));
  await act(async () => complete(applied));
  view.rerender(createElement(RetentionDisclosure, { owner, source: node(settings, law.normalized, law.ownRevision), onAction }));
  fireEvent.click(view.getByRole("button", { name: "Expand source" }));
  await waitFor(() => expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.normalized));
  expect(view.container.querySelector("[data-dirty]")?.getAttribute("data-dirty")).toBe("false");
  expect(retainedTextEditorDraftCountV1(owner)).toBe(0);
});

test("Source retains its exact Cancel control across disclosure unmount", async () => {
  const registry = new LocalDocumentOwnerRegistryV1();
  const owner = registry.acquire(retention.owner);
  const settings = fixture.cases[0]!.settings;
  let complete: (outcome: unknown) => void = () => {};
  const cancel = vi.fn(async () => ({ kind: "applied", inputSeq: 2 } as const));
  const onAction = vi.fn((_action, lifecycle) => new Promise(resolve => {
    lifecycle?.started({ operationId: law.operation, generation: law.operationGeneration, cancel });
    complete = resolve;
  }));
  const view = render(createElement(RetentionDisclosure, { owner, source: node(settings), onAction }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  fireEvent.click(view.getByRole("button", { name: settings.applyLabel, exact: true }));
  fireEvent.click(view.getByRole("button", { name: "Collapse source" }));
  expect(queryByRole(view.container, "button", { name: settings.cancelLabel })).toBeNull();
  fireEvent.click(view.getByRole("button", { name: "Expand source" }));
  await act(async () => { fireEvent.click(view.getByRole("button", { name: settings.cancelLabel, exact: true })); });
  expect(cancel).toHaveBeenCalledTimes(1);
  await act(async () => complete(applied));
  expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.draft);
});

test("a replacement document never inherits a retained Source draft and retirement clears its owner", () => {
  const registry = new LocalDocumentOwnerRegistryV1();
  const owner = registry.acquire(retention.owner);
  const replacement = registry.acquire(retention.replacementOwner);
  const settings = fixture.cases[0]!.settings;
  const view = render(createElement(RetentionDisclosure, { owner, source: node(settings), onAction: () => {} }));
  typeDraft(view.getByRole("textbox") as HTMLTextAreaElement);
  fireEvent.click(view.getByRole("button", { name: "Collapse source" }));
  registry.retire(retention.owner);
  expect(owner.active).toBe(false);
  expect(retainedTextEditorDraftCountV1(owner)).toBe(0);
  view.rerender(createElement(RetentionDisclosure, { owner: replacement, source: node(settings), onAction: () => {} }));
  fireEvent.click(view.getByRole("button", { name: "Expand source" }));
  expect((view.getByRole("textbox") as HTMLTextAreaElement).value).toBe(law.base);
});

for (const chord of law.selectAllChords) test(`Source canvas Select All accepts ${JSON.stringify(chord)}`, async () => {
  vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} });
  vi.mocked(nodeGraph.useClient).mockReturnValue(true);
  const selectAll = vi.fn();
  const attachCanvas = vi.fn(async () => {});
  const session = new Proxy({ attachCanvas, selectAll, text: () => law.base, caret: () => 0, anchor: () => 0, cameraJson: () => "{}", hoverTokenRangeJson: () => "null", pickTargetsAtScreenJson: () => "[]" }, { get: (target, key) => key === "then" ? undefined : key in target ? target[key as keyof typeof target] : () => {} }) as unknown as sessionLoader.EditorWasmSession;
  vi.spyOn(sessionLoader, "createEditorSession").mockResolvedValue(session);
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
  const view = render(createElement(TextEditorHost, { node: node(fixture.cases[0]!.settings), onAction: () => {} }));
  await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
  fireEvent.keyDown(view.getByRole("textbox"), chord);
  expect(selectAll).toHaveBeenCalledOnce();
});
