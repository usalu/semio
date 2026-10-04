/**
 * ⚙️ React's independent acceptance of the same language-neutral Puzzle3D Settings document
 * the retained wgpu frame law consumes. Testing Library supplies the browser-role oracle; the live
 * Interpreter supplies the NumberStepper behavior and semantic intent.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { Field, Section, formatNumber, uiDataLabel } from "@semio-tech/ui-react";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";

const { computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const law = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/⚙️puzzle3d-settings-document/🔣️.json"), "utf8")) as {
  readonly document: { readonly surface: string; readonly revision: number; readonly root: number; readonly nodes: readonly Record<string, unknown>[] };
  readonly windowId: string;
  readonly controls: readonly { readonly key: string; readonly value: number; readonly step: number; readonly action: string; readonly label: string; readonly formatted: string }[];
};
const stepperFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🪜️stepper-pointer-commit/🔣️.json"), "utf8")) as { readonly cases: readonly { id: string; segment: "minus" | "value" | "plus"; terminal: "release-inside" | "release-outside" | "cancel"; pressActions: number; terminalActions: number; deltaSteps: number }[] };
const stepperSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🪜️stepper-pointer-commit/🔣️.json"), "utf8"));
const pressInputSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🎚️numeric-press-input/🔣️.json"), "utf8"));
const pressInputFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎚️numeric-press-input/🔣️.json"), "utf8")) as { readonly lifecycle: { readonly offeredCommit: false; readonly releasedCommit: true; readonly releasedActions: number; readonly sequenceStep: number }; readonly cases: readonly { readonly name: string; readonly input: unknown; readonly accepted: boolean }[] };
const validatePressInput = new Ajv2020().compile(pressInputSchema.$defs.input);
const sectionFieldFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️section-field-presentation/🔣️.json"), "utf8")) as {
  readonly density: { readonly gap: number; readonly fieldLabelLineHeight: number; readonly fieldDetailLineHeight: number; readonly sectionTitleLineHeight: number; readonly sectionTitleBodyGap: number; readonly sectionTrailingMargin: number };
  readonly section: { readonly width: number; readonly title: string; readonly titleLines: number; readonly titleHeight: number; readonly contentTop: number; readonly trailingMargin: number };
  readonly field: { readonly width: number; readonly label: string; readonly description: string; readonly error: string; readonly detailLines: number; readonly controlHeight: number; readonly controlTop: number; readonly errorTop: number; readonly totalHeight: number };
};
const sectionFieldSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/📐️section-field-presentation/🔣️.json"), "utf8"));

describe("Puzzle3D Settings Component document", () => {
  afterEach(() => cleanup());

  it("validates the shared pointer commit contract", () => {
    const validate = new Ajv2020().compile(stepperSchema);
    expect(validate(stepperFixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("admits the closed numeric press corpus through the independent wire oracle", () => {
    const validate = new Ajv2020().compile(pressInputSchema);
    expect(validate(pressInputFixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of pressInputFixture.cases) expect(validatePressInput(row.input), row.name).toBe(row.accepted);
    expect(validate({ ...pressInputFixture, foreign: true })).toBe(false);
  });

  it("matches the shared display precision and square-button presentation contract", () => {
    const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🪜️stepper-presentation/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🪜️stepper-presentation/🔣️.json"), "utf8"));
    const validate = new Ajv2020().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of fixture.numbers) expect(formatNumber(row.value)).toBe(row.text);
    const store = new UiDocumentStore(law.document.surface);
    store.loadSnapshot({ surface: law.document.surface, revision: law.document.revision, root: law.document.root, nodes: [...law.document.nodes] } as any);
    render(createElement(UiNodeView, { store, id: law.document.root, context: { store, onAction: () => {}, onIntent: () => {} } }));
    expect(screen.getByRole("region", { name: "Settings — pane-top" })).toBeTruthy();
    for (const input of screen.getAllByRole("spinbutton")) {
      expect(input.classList.contains("text-center")).toBe(true);
      expect(input.classList.contains("border-0")).toBe(true);
      const group = input.closest('[data-slot="stepper-group"]')!;
      expect(group.classList.contains("border")).toBe(true);
      expect(group.classList.contains("overflow-hidden")).toBe(true);
      for (const slot of ["minus", "plus"]) {
        const button = group.querySelector(`[data-slot="stepper-${slot}"]`)!;
        expect(button.classList.contains("w-medium")).toBe(true);
        expect(button.classList.contains("h-medium")).toBe(true);
        expect(button.classList.contains("shrink-0")).toBe(true);
        expect(button.querySelector("[data-icon]")?.getAttribute("data-icon")).toBe(slot);
        expect(button.querySelector("[data-icon]")?.classList.contains("size-tiny")).toBe(true);
      }
    }
  });

  it("uses React's actual Section and Field token/order contract for the neutral geometry fixture", () => {
    const validate = new Ajv2020().compile(sectionFieldSchema);
    expect(validate(sectionFieldFixture), JSON.stringify(validate.errors)).toBe(true);
    render(
      createElement(
        Section,
        {
          id: "settings-geometry",
          title: uiDataLabel(sectionFieldFixture.section.title),
          children: createElement(Field, {
            id: "settings-geometry.threshold",
            label: sectionFieldFixture.field.label,
            description: sectionFieldFixture.field.description,
            error: sectionFieldFixture.field.error,
            required: true,
            children: createElement("input", { id: "settings-geometry.threshold.control" }),
          }),
        },
      ),
    );
    const heading = screen.getByRole("heading", { name: sectionFieldFixture.section.title });
    expect(heading.classList.contains("text-2xl")).toBe(true);
    expect(heading.classList.contains("font-semibold")).toBe(true);
    expect(heading.classList.contains("mb-4")).toBe(true);
    expect(heading.closest("section")?.classList.contains("mb-8")).toBe(true);
    const field = document.querySelector('[data-slot="field"]')!;
    expect([...field.children].map((child) => child.querySelector("[data-slot]")?.getAttribute("data-slot") ?? child.getAttribute("data-slot"))).toEqual(["field-label", "field-description", "field-control", "field-error"]);
    expect(field.classList.contains("gap-single")).toBe(true);
    expect(field.querySelector('[data-slot="field-label"]')?.classList.contains("truncate")).toBe(true);
    expect(field.querySelector('[data-slot="field-label"]')?.classList.contains("font-medium")).toBe(true);
    expect(sectionFieldFixture.section.titleHeight).toBeCloseTo(sectionFieldFixture.section.titleLines * sectionFieldFixture.density.sectionTitleLineHeight, 6);
    expect(sectionFieldFixture.section.contentTop).toBeCloseTo(sectionFieldFixture.section.titleHeight + sectionFieldFixture.density.sectionTitleBodyGap, 6);
    expect(sectionFieldFixture.field.controlTop).toBeCloseTo(sectionFieldFixture.density.fieldLabelLineHeight + sectionFieldFixture.density.gap + sectionFieldFixture.field.detailLines * sectionFieldFixture.density.fieldDetailLineHeight + sectionFieldFixture.density.gap, 6);
    expect(sectionFieldFixture.field.errorTop).toBeCloseTo(sectionFieldFixture.field.controlTop + sectionFieldFixture.field.controlHeight + sectionFieldFixture.density.gap, 6);
    expect(sectionFieldFixture.field.totalHeight).toBeCloseTo(sectionFieldFixture.field.errorTop + sectionFieldFixture.field.detailLines * sectionFieldFixture.density.fieldDetailLineHeight, 6);
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
      expect(validatePressInput(intents[0].input), JSON.stringify(validatePressInput.errors)).toBe(true);
      expect(intents[0].input.commit).toBe(pressInputFixture.lifecycle.offeredCommit);
      expect(intents[0].input.gesture.split(":").slice(0, -2).join(":")).toBe(law.controls[0].key);
      expect(intents[0].input.value).toBeCloseTo(law.controls[0].value + gesture.deltaSteps * law.controls[0].step, 9);
    }
    if (gesture.terminal !== "release-inside") fireEvent.mouseLeave(target);
    fireEvent.mouseUp(gesture.terminal === "release-inside" ? target : document.body);
    const offered = intents.filter((intent) => intent.input.commit === pressInputFixture.lifecycle.offeredCommit);
    const committed = intents.filter((intent) => intent.input.commit === pressInputFixture.lifecycle.releasedCommit);
    expect(offered).toHaveLength(gesture.pressActions + gesture.terminalActions);
    expect(committed).toHaveLength(gesture.pressActions ? pressInputFixture.lifecycle.releasedActions : 0);
    expect(intents).toHaveLength(offered.length + committed.length);
    for (const intent of committed) {
      expect(validatePressInput(intent.input), JSON.stringify(validatePressInput.errors)).toBe(true);
      expect(intent.input).toEqual({ ...offered[0].input, commit: pressInputFixture.lifecycle.releasedCommit });
      expect(intent.seq).toBe(offered[0].seq + BigInt(pressInputFixture.lifecycle.sequenceStep));
      expect({ ...intent, input: offered[0].input, seq: offered[0].seq }).toEqual(offered[0]);
    }
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
      expect(input.value).toBe(control.formatted);
      expect(computeAccessibleName(input)).toBe(control.label);
      expect(input.getAttribute("data-mixed")).toBeNull();
      const group = input.closest('[data-slot="stepper-group"]');
      expect(group).not.toBeNull();
      const plus = group!.querySelector('[data-slot="stepper-plus"]');
      expect(plus).not.toBeNull();
      fireEvent.mouseDown(plus!);
      fireEvent.mouseUp(plus!);
      const controlIntents = intents.filter((intent) => intent.nodeKey === control.key);
      expect(controlIntents).toHaveLength(1 + pressInputFixture.lifecycle.releasedActions);
      const [intent, committed] = controlIntents;
      expect(intent.nodeKey).toBe(control.key);
      expect(intent.trigger).toBe("change");
      expect(intent.action).toEqual({ scope: "puzzle3d-play", name: control.action, version: 1 });
      expect(intent.args).toEqual({ windowId: law.windowId });
      expect(validatePressInput(intent.input), JSON.stringify(validatePressInput.errors)).toBe(true);
      expect(intent.input.commit).toBe(pressInputFixture.lifecycle.offeredCommit);
      expect(intent.input.gesture.split(":").slice(0, -2).join(":")).toBe(control.key);
      expect(intent.input.value).toBeCloseTo(control.value + control.step, 9);
      expect(committed.input).toEqual({ ...intent.input, commit: pressInputFixture.lifecycle.releasedCommit });
      expect(committed.seq).toBe(intent.seq + BigInt(pressInputFixture.lifecycle.sequenceStep));
      expect({ ...committed, input: intent.input, seq: intent.seq }).toEqual(intent);
    });
    expect(intents).toHaveLength(law.controls.length * (1 + pressInputFixture.lifecycle.releasedActions));
  });
});
