// @vitest-environment jsdom
/** 🪟️ Mounted source editors preserve corrections and exact parent routing across refusals in both locales. */
import { act, cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { getByRole, queryByRole } from "@testing-library/dom";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { UiI18nProvider, createShellI18nInstance, disposeShellI18nInstance } from "@semio-tech/ui-react/source";
import type { ComponentSceneHostProps } from "@semio-tech/framework";
import * as nodeGraph from "@semio-tech/framework-os-node-graph-rs/index";
import { LocalDocumentOwnerContext, LocalDocumentOwnerRegistryV1 } from "../../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { useEditorSurfaceState } from "../../🧰️surface-state-and-delivery/🟦️.ts";
import type { FrameworkEditorSession } from "../../🏷️editor-types/🟦️.ts";
import { textEditorValueSourceActionV1 } from "../../📄️value-source/🟦️.ts";
import { TextEditorHost } from "../../🟦️.tsx";
import fixture from "../🧫️fixtures/📄️value-source/🔣️.json";

beforeEach(() => { vi.spyOn(nodeGraph, "useClient").mockReturnValue(false); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const applied = { kind: "applied", inputSeq: 1 };
const edit = (control: HTMLElement, value: string) => { fireEvent.change(control, { target: { value } }); fireEvent.blur(control); };

for (const row of fixture.refusalLocales) test(`mounted source refusal and correction: ${row.locale}`, async () => {
  if (row.locale !== "en" && row.locale !== "de") throw new Error("Explicit supported locale required");
  const i18n = createShellI18nInstance(row.locale), law = fixture.encodingBoundaries, valid = law.unit.repeat(law.repeat), invalid = valid + "x";
  const settings = { ...fixture.settings, label: row.sourceLabel, editArgument: "valueJsonChunks", editEncoding: "json-chunks", editMaximumBytes: law.maximumBytes, editArguments: { ...fixture.settings.editArguments, windowId: "source-first" } };
  const node = (buffer: string, label = settings.label): ComponentSceneHostProps["node"] => ({ type: "componentScene", componentKind: "text-editor", controllerId: "child", surfaceId: "source-first", textEditor: { language: "json", buffer, settingsJson: JSON.stringify({ ...settings, label }) } });
  const completions: ((value: unknown) => void)[] = [], onAction = vi.fn((_action: unknown) => new Promise<unknown>(resolve => completions.push(resolve)));
  const tree = (buffer: string, label = settings.label) => createElement(UiI18nProvider, { instance: i18n }, createElement(TextEditorHost, { node: node(buffer, label), onAction }));
  try {
    const view = render(tree(fixture.source)), control = view.getByRole("textbox") as HTMLTextAreaElement;
    edit(control, invalid);
    await waitFor(() => expect(view.getByRole("alert").textContent).toBe(row.message));
    expect(control.value).toBe(invalid);
    expect(getByRole(view.container, "textbox", { description: row.message })).toBe(control);
    expect(control.getAttribute("aria-invalid")).toBe("true");
    expect(onAction).not.toHaveBeenCalled();
    const other = fixture.refusalLocales.find(value => value.locale !== row.locale)!;
    await act(async () => i18n.changeLanguage(other.locale));
    view.rerender(tree(fixture.source, other.sourceLabel));
    expect(control.value).toBe(invalid);
    expect(view.getByRole("alert").textContent).toBe(other.message);
    await act(async () => i18n.changeLanguage(row.locale));
    view.rerender(tree(fixture.source));
    expect(control.value).toBe(invalid);
    expect(view.getByRole("alert").textContent).toBe(row.message);
    edit(control, valid);
    await waitFor(() => expect(onAction).toHaveBeenCalledTimes(1));
    expect(view.getByRole("alert").textContent).toBe(row.message);
    const command = onAction.mock.calls[0]![0] as { controllerId: string; action: string; args: { valueJsonChunks: string[]; windowId: string } };
    expect(command.controllerId).toBe(settings.editControllerId);
    expect(command.action).toBe(settings.editAction);
    expect(command.args.windowId).toBe("source-first");
    expect(JSON.parse(command.args.valueJsonChunks.join(""))).toBe(valid);
    edit(control, invalid + "y");
    await act(async () => completions[0]!(applied));
    expect(control.value).toBe(invalid + "y");
    expect(view.getByRole("alert").textContent).toBe(row.message);
    expect(onAction).toHaveBeenCalledTimes(1);
    edit(control, valid);
    await waitFor(() => expect(onAction).toHaveBeenCalledTimes(2));
    view.rerender(tree(valid));
    expect(control.value).toBe(valid);
    expect(view.getByRole("alert").textContent).toBe(row.message);
    await act(async () => completions[1]!(applied));
    await waitFor(() => expect(queryByRole(view.container, "alert")).toBeNull());
    expect(control.hasAttribute("aria-invalid")).toBe(false);
    view.rerender(tree("foreign publication"));
    expect(control.value).toBe("foreign publication");
    edit(control, invalid);
    await waitFor(() => expect(view.getByRole("alert").textContent).toBe(row.message));
    fireEvent.click(view.getByRole("button", { name: row.discard, exact: true }));
    expect(control.value).toBe("foreign publication");
    expect(queryByRole(view.container, "alert")).toBeNull();
    expect(onAction).toHaveBeenCalledTimes(2);
  } finally { disposeShellI18nInstance(i18n); }
});


test("exact mounted document retirement rejects an older source completion", async () => {
  const registry = new LocalDocumentOwnerRegistryV1(), identity = { pluginId: "fixture", appId: "source", sessionInstanceId: 1, runtimeKey: "first", clientInstanceId: "client" }, first = registry.acquire(identity), second = registry.acquire({ ...identity, runtimeKey: "second" });
  const i18n = createShellI18nInstance("en"), law = fixture.encodingBoundaries, valid = law.unit.repeat(law.repeat), invalid = valid + "x";
  const settings = { ...fixture.settings, editEncoding: "json-chunks", editArgument: "valueJsonChunks", editMaximumBytes: law.maximumBytes };
  const node = (buffer: string): ComponentSceneHostProps["node"] => ({ type: "componentScene", componentKind: "text-editor", controllerId: "same-child", surfaceId: "same-source", textEditor: { language: "json", buffer, settingsJson: JSON.stringify(settings) } });
  let complete: (outcome: unknown) => void = () => {};
  const onAction = vi.fn((_action: unknown) => new Promise<unknown>(resolve => { complete = resolve; }));
  const tree = (owner: typeof first, buffer: string) => createElement(UiI18nProvider, { instance: i18n }, createElement(LocalDocumentOwnerContext.Provider, { value: owner }, createElement(TextEditorHost, { node: node(buffer), onAction })));
  try {
    const view = render(tree(first, fixture.source)), control = view.getByRole("textbox") as HTMLTextAreaElement;
    edit(control, valid);
    await waitFor(() => expect(onAction).toHaveBeenCalledTimes(1));
    view.rerender(tree(second, "second publication"));
    expect(control.value).toBe("second publication");
    edit(control, invalid);
    await waitFor(() => expect(view.getByRole("alert").textContent).toBe(fixture.refusalLocales[0]!.message));
    await act(async () => complete(applied));
    expect(control.value).toBe(invalid);
    expect(view.getByRole("alert").textContent).toBe(fixture.refusalLocales[0]!.message);
    await act(async () => registry.retireId(second.id));
    expect(control.value).toBe("second publication");
    expect(queryByRole(view.container, "alert")).toBeNull();
    expect(onAction).toHaveBeenCalledTimes(1);
  } finally { registry.retireAll(); disposeShellI18nInstance(i18n); }
});

test("source outbox retains latest local edits without false typing or invalid parent dispatch", async () => {
  const i18n = createShellI18nInstance("en"), law = fixture.encodingBoundaries, valid = law.unit.repeat(law.repeat), invalid = valid + "x", corrected = law.unit.repeat(law.repeat - 1) + "x";
  const settings = { ...fixture.settings, commit: "change" as const, editArgument: "valueJsonChunks", editEncoding: "json-chunks" as const, editMaximumBytes: law.maximumBytes };
  const completions: (() => void)[] = [], parent = vi.fn((_action: unknown) => new Promise<void>(resolve => completions.push(resolve))), onAction = vi.fn(), local = vi.fn();
  let text = fixture.source, core: ReturnType<typeof useEditorSurfaceState> | undefined;
  const session = { text: () => text, anchor: () => text.length, caret: () => text.length } as unknown as FrameworkEditorSession;
  const commit = async (value: string) => {
    const result = textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textEdit", args: { text: value } });
    if (result.kind === "dispatch") await parent(result.action);
  };
  function Surface() {
    core = useEditorSurfaceState({ scene: { buffer: fixture.source, language: "json" }, controllerId: "child", surfaceId: "first", onAction, onSourceCommit: commit, onDraftChange: local });
    core.sessionRef.current = session;
    return createElement("button", { onClick: core.deliver }, "Deliver source");
  }
  try {
    const view = render(createElement(UiI18nProvider, { instance: i18n }, createElement(Surface)));
    const send = (value: string) => { text = value; fireEvent.click(view.getByRole("button", { name: "Deliver source" })); };
    await act(async () => send(invalid));
    expect(parent).not.toHaveBeenCalled();
    send(valid);
    await waitFor(() => expect(parent).toHaveBeenCalledTimes(1));
    send(invalid);
    send(corrected);
    expect(local.mock.calls.at(-1)?.[0]).toBe(corrected);
    await act(async () => completions[0]!());
    await waitFor(() => expect(parent).toHaveBeenCalledTimes(2));
    expect(JSON.parse((parent.mock.calls[1]![0] as { args: { valueJsonChunks: string[] } }).args.valueJsonChunks.join(""))).toBe(corrected);
    await act(async () => completions[1]!());
    expect(parent).toHaveBeenCalledTimes(2);
    expect(onAction).not.toHaveBeenCalled();
    expect(core!.typingRef.current!.preview(corrected)).toBeNull();
  } finally { disposeShellI18nInstance(i18n); }
});

test("a local revert is not lost behind an older admitted source edit", async () => {
  const i18n = createShellI18nInstance("en"), settings = { ...fixture.settings, editEncoding: "json-chunks", editArgument: "valueJsonChunks", editMaximumBytes: fixture.encodingBoundaries.maximumBytes }, later = "later source";
  const completions: ((outcome: unknown) => void)[] = [], onAction = vi.fn((_action: unknown) => new Promise<unknown>(resolve => completions.push(resolve)));
  const tree = (buffer: string) => createElement(UiI18nProvider, { instance: i18n }, createElement(TextEditorHost, { node: { type: "componentScene", componentKind: "text-editor", controllerId: "child", surfaceId: "revert-source", textEditor: { language: "json", buffer, settingsJson: JSON.stringify(settings) } }, onAction }));
  try {
    const view = render(tree(fixture.source)), control = view.getByRole("textbox") as HTMLTextAreaElement;
    edit(control, later);
    await waitFor(() => expect(onAction).toHaveBeenCalledTimes(1));
    edit(control, fixture.source);
    await waitFor(() => expect(onAction).toHaveBeenCalledTimes(2));
    expect(control.value).toBe(fixture.source);
    expect(JSON.parse((onAction.mock.calls[1]![0] as { args: { valueJsonChunks: string[] } }).args.valueJsonChunks.join(""))).toBe(fixture.source);
    view.rerender(tree(fixture.source));
    await act(async () => completions[1]!(applied));
    await act(async () => completions[0]!(applied));
    expect(control.value).toBe(fixture.source);
    expect(queryByRole(view.container, "alert")).toBeNull();
  } finally { disposeShellI18nInstance(i18n); }
});
