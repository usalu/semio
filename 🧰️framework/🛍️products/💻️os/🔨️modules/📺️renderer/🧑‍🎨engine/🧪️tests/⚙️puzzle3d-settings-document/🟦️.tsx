/**
 * ⚙️ React's independent acceptance of the same language-neutral Puzzle3D Settings document
 * the retained wgpu frame law consumes. Testing Library supplies the browser-role oracle; the live
 * Interpreter supplies the NumberStepper behavior and semantic intent.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const law = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/⚙️puzzle3d-settings-document/🔣️.json"), "utf8")) as {
  readonly document: { readonly surface: string; readonly revision: number; readonly root: number; readonly nodes: readonly Record<string, unknown>[] };
  readonly windowId: string;
  readonly controls: readonly { readonly key: string; readonly value: number; readonly step: number; readonly action: string; readonly label: string }[];
};
const stepperFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🪜️stepper-pointer-commit/🔣️.json"), "utf8")) as { readonly cases: readonly { id: string; segment: "minus" | "value" | "plus"; terminal: "release-inside" | "release-outside" | "cancel"; pressActions: number; terminalActions: number; deltaSteps: number }[] };
const stepperSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🪜️stepper-pointer-commit/🔣️.json"), "utf8"));

describe("Puzzle3D Settings Component document", () => {
  afterEach(() => cleanup());

  it("validates the shared pointer commit contract", () => {
    const validate = new Ajv2020().compile(stepperSchema);
    expect(validate(stepperFixture), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const gesture of stepperFixture.cases) it(`commits ${gesture.id} at press and only cleans up at release`, () => {
    const store = new UiDocumentStore(law.document.surface);
    store.loadSnapshot({ surface: law.document.surface, revision: law.document.revision, root: law.document.root, nodes: [...law.document.nodes] } as any);
    const intents: any[] = [];
    render(createElement(UiNodeView, { store, id: law.document.root, context: { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } } }));
    const input = screen.getAllByRole("spinbutton")[0];
    const target = gesture.segment === "value" ? input : input.closest('[data-slot="stepper-group"]')!.querySelector(`[data-slot="stepper-${gesture.segment}"]`)!;
    fireEvent.mouseDown(target);
    expect(intents).toHaveLength(gesture.pressActions);
    if (intents.length) {
      expect(intents[0].action.name).toBe(law.controls[0].action);
      expect(intents[0].input).toBeCloseTo(law.controls[0].value + gesture.deltaSteps * law.controls[0].step, 9);
    }
    if (gesture.terminal !== "release-inside") fireEvent.mouseLeave(target);
    fireEvent.mouseUp(gesture.terminal === "release-inside" ? target : document.body);
    expect(intents).toHaveLength(gesture.pressActions + gesture.terminalActions);
    console.log(`[DEBUG] stepper pointer ${gesture.id}: press=${gesture.pressActions} terminal=${gesture.terminalActions}`);
  });

  it("mounts four uniform steppers and each increment dispatches its authored Change intent", () => {
    const store = new UiDocumentStore(law.document.surface);
    store.loadSnapshot({ surface: law.document.surface, revision: law.document.revision, root: law.document.root, nodes: [...law.document.nodes] } as any);
    const intents: any[] = [];
    render(createElement(UiNodeView, { store, id: law.document.root, context: { store, onAction: () => {}, onIntent: (intent: unknown) => { intents.push(intent); } } }));

    const inputs = screen.getAllByRole("spinbutton");
    expect(inputs).toHaveLength(law.controls.length);
    law.controls.forEach((control, index) => {
      const input = inputs[index] as HTMLInputElement;
      expect(input.id).toBe(`${law.document.surface}/${control.key}`);
      expect(input.value).toBe(String(control.value));
      expect(input.getAttribute("data-mixed")).toBeNull();
      const group = input.closest('[data-slot="stepper-group"]');
      expect(group).not.toBeNull();
      const plus = group!.querySelector('[data-slot="stepper-plus"]');
      expect(plus).not.toBeNull();
      fireEvent.mouseDown(plus!);
      fireEvent.mouseUp(plus!);
      const intent = intents[index];
      expect(intent.nodeKey).toBe(control.key);
      expect(intent.trigger).toBe("change");
      expect(intent.action).toEqual({ scope: "puzzle3d-play", name: control.action, version: 1 });
      expect(intent.args).toEqual({ windowId: law.windowId });
      expect(intent.input).toBeCloseTo(control.value + control.step, 9);
    });
  });
});
