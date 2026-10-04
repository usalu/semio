/** 🌳️ The retained React interpreter mounts native Details collections as expandable tree items with live editors. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, expect, it } from "vitest";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";

type RendererInput = { readonly key: string; readonly label: string; readonly value: string; readonly action: string; readonly draftTargetContains: string };
type RendererEntry = { readonly key: string; readonly label: string; readonly defaultOpen: boolean; readonly value: { readonly key: string; readonly label: string; readonly input: RendererInput } };
type RendererHierarchy = { readonly key: string; readonly label: string; readonly defaultOpen: boolean; readonly window: { readonly total: number; readonly offset: number }; readonly entries: readonly RendererEntry[] };

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(
  readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️fixtures/🪟️collection-tree-shape/🔣️.json"), "utf8"),
) as { readonly collectionKey: string; readonly rendererHierarchy: RendererHierarchy };
const layout = { kind: "leaf", width: "hug", height: "hug" } as const;
const style = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;
const accessibility = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

function record(id: number, key: string, component: Record<string, unknown>, children: readonly number[] = [], bindings: readonly Record<string, unknown>[] = [], label: string | null = null) {
  return { id, key, component, layout, style, activity: "idle", disabled: false, transition: null, accessibility: { ...accessibility, label }, bindings: [...bindings], menu: null, children: [...children] };
}

function treeItem(id: number, key: string, label: string, children: readonly number[], extra: Record<string, unknown> = {}) {
  return record(id, key, { type: "treeItem", label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null, ...extra }, children);
}

function documentFromNativeHierarchy(hierarchy: RendererHierarchy) {
  let nextId = 4;
  const entryIds: number[] = [];
  const nodes: Record<string, unknown>[] = [
    record(1, "stdio-snapshot-details", { type: "tree", interactionDomain: null }, [2]),
    record(2, "stdio-snapshot-details-fields", { type: "treeSection", label: "Details", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 1, offset: 0 } }, [3]),
  ];
  for (const [index, entry] of hierarchy.entries.entries()) {
    const entryId = nextId++;
    const valueId = nextId++;
    const inputId = nextId++;
    entryIds.push(entryId);
    nodes.push(treeItem(entryId, entry.key, entry.label, [valueId], { defaultOpen: entry.defaultOpen }));
    nodes.push(treeItem(valueId, entry.value.key, entry.value.label, [inputId]));
    nodes.push(
      record(
        inputId,
        entry.value.input.key,
        { type: "input", kind: "text", value: entry.value.input.value, placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null, precision: null, snaps: [], displayFactor: null, limits: null, draftTarget: `${entry.value.input.draftTargetContains}:${entry.value.key}` },
        [],
        [{ trigger: "commit", action: { scope: "test.collection-tree-shape", name: entry.value.input.action, version: 1 }, args: { path: `/${fixture.collectionKey}/${index}/value` }, capability: null }],
        entry.value.input.label,
      ),
    );
  }
  nodes.push(treeItem(3, hierarchy.key, hierarchy.label, entryIds, { defaultOpen: hierarchy.defaultOpen, window: { rowExtent: "standard", ...hierarchy.window } }));
  return { surface: "stdio-details-tree", revision: 1, root: 1, layoutEpoch: 0n, nodes };
}

afterEach(cleanup);

it("mounts native collection rows as expandable tree descendants with editable existing values", () => {
  const hierarchy = fixture.rendererHierarchy;
  const store = new UiDocumentStore("stdio-details-tree");
  store.loadSnapshot(documentFromNativeHierarchy(hierarchy) as any);
  const intents: any[] = [];
  render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } } }));

  const collection = screen.getByRole("treeitem", { name: hierarchy.label });
  expect(collection.getAttribute("aria-expanded")).toBe("true");
  const editors = screen.getAllByRole("textbox", { name: hierarchy.entries[0]!.value.input.label }) as HTMLInputElement[];
  expect(editors.map((editor) => editor.value)).toEqual(hierarchy.entries.map((entry) => entry.value.input.value));
  for (const entry of hierarchy.entries) expect(screen.getByRole("treeitem", { name: entry.label })).toBeTruthy();

  fireEvent.change(editors[0]!, { target: { value: "Gamma" } });
  fireEvent.blur(editors[0]!);
  expect(intents).toHaveLength(1);
  expect(intents[0].action.name).toBe(hierarchy.entries[0]!.value.input.action);
  expect(intents[0].input).toBe("Gamma");

  const editorWithValue = (value: string) => Array.from(document.querySelectorAll<HTMLInputElement>("input, textarea")).find((editor) => editor.value === value);
  fireEvent.click(collection.querySelector("button[aria-expanded=true]") as HTMLButtonElement);
  expect(collection.getAttribute("aria-expanded")).toBe("false");
  expect(editorWithValue(hierarchy.entries[1]!.value.input.value)).toBeUndefined();
  fireEvent.click(collection.querySelector("button[aria-expanded=false]") as HTMLButtonElement);
  expect(collection.getAttribute("aria-expanded")).toBe("true");
  expect(editorWithValue(hierarchy.entries[1]!.value.input.value)?.disabled).toBe(false);
});
