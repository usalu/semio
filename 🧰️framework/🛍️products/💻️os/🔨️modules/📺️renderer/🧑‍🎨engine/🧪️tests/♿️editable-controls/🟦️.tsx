/** ♿️ Authored input labels, disability and blur commits survive the retained React interpreter. */
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { computeAccessibleName } from "dom-accessibility-api";
import { afterEach, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/♿️editable-controls/🔣️.json";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";

afterEach(cleanup);
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
      if (gesture === "enter") fireEvent.keyDown(control, { key: "Enter" });
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
