/** 🔶️ The actual react stepper matches tests of the ⚙️ General Settings Tree and bottom-panel flow suite. */
import { act, fireEvent, render } from "@testing-library/react";
import { createElement as h } from "react";
import { expect, test, vi } from "vitest";
import { Stepper } from "@semio-tech/ui-react";
import { Slider } from "@semio-tech/ui-react";
import { bottomGeometry, fixture, sliderEditingFixture, stepperEditingFixture } from "../🟢️shared/🟦️.ts";

/** 🔷 Registers the actual react stepper matches tests of the ⚙️ General Settings Tree and bottom-panel flow suite. */
export function registerActualReactStepperMatchesTests(): void {
  test("the actual React Stepper matches bounded editing, key, focus and held-button behavior", () => {
    const uniform = stepperEditingFixture.uniform;
    const changes: number[] = [];
    const view = render(h(Stepper, { id: "stepper.edit", ...uniform, onChange: (value: number) => changes.push(value) }));
    const input = view.container.querySelector<HTMLInputElement>("[data-stepper-input='true']")!;
    fireEvent.focus(input);
    expect(input.value).toBe(uniform.focusDisplay);
    fireEvent.change(input, { target: { value: stepperEditingFixture.validEdit.input } });
    expect(changes).toEqual([stepperEditingFixture.validEdit.expectedValue]);
    expect(input.value).toBe(stepperEditingFixture.validEdit.expectedDisplay);
    fireEvent.keyDown(input, { key: "Escape" });
    expect(input.value).toBe(String(uniform.value));
    expect(document.activeElement).not.toBe(input);
    view.unmount();

    const refusedChanges: number[] = [];
    const refused = render(h(Stepper, { id: "stepper.refused", ...uniform, onChange: (value: number) => refusedChanges.push(value) }));
    const refusedInput = refused.container.querySelector<HTMLInputElement>("[data-stepper-input='true']")!;
    fireEvent.focus(refusedInput);
    fireEvent.change(refusedInput, { target: { value: stepperEditingFixture.refusedEdit.input } });
    expect(refusedChanges).toEqual([]);
    expect(refusedInput.value).toBe(stepperEditingFixture.refusedEdit.expectedDisplay);
    expect(refusedInput.getAttribute("aria-invalid")).toBe("true");
    fireEvent.blur(refusedInput);
    expect(refusedInput.value).toBe(stepperEditingFixture.refusedEdit.expectedDisplay);
    refused.unmount();

    const mixed = render(h(Stepper, { id: "stepper.mixed", ...stepperEditingFixture.mixed, mixed: true, value: undefined }));
    const mixedInput = mixed.container.querySelector<HTMLInputElement>("[data-stepper-input='true']")!;
    fireEvent.focus(mixedInput);
    expect(mixedInput.value).toBe(stepperEditingFixture.mixed.focusDisplay);
    mixed.unmount();

    for (const keyCase of stepperEditingFixture.boundedKeys) {
      const actions: { field: string; value: number }[] = [];
      const props = {
        id: `stepper.${keyCase.id}`,
        value: keyCase.value,
        min: uniform.min,
        max: uniform.max,
        step: uniform.step,
        ...(keyCase.binding === "delta"
          ? { onDelta: (value: number) => actions.push({ field: "delta", value }) }
          : { onChange: (value: number) => actions.push({ field: "value", value }) }),
      };
      const keyed = render(h(Stepper, props));
      const keyedInput = keyed.container.querySelector<HTMLInputElement>("[data-stepper-input='true']")!;
      fireEvent.focus(keyedInput);
      fireEvent.keyDown(keyedInput, { key: keyCase.key });
      expect(actions).toEqual([{ field: keyCase.expectedField, value: keyCase.expectedActionValue }]);
      expect(keyedInput.value).toBe(keyCase.expectedDisplay);
      fireEvent.keyDown(keyedInput, { key: "Enter" });
      expect(document.activeElement).not.toBe(keyedInput);
      keyed.unmount();
    }

    const pointerCase = stepperEditingFixture.boundedPointer;
    const boundedActions: number[] = [];
    const bounded = render(h(Stepper, { id: "stepper.bounded", value: pointerCase.value, min: uniform.min, max: uniform.max, step: uniform.step, onChange: (value: number) => boundedActions.push(value) }));
    const boundedSide = bounded.container.querySelector<HTMLButtonElement>(`[data-slot='stepper-${pointerCase.side}']`)!;
    expect(boundedSide.disabled).toBe(pointerCase.disabled);
    fireEvent.mouseDown(boundedSide);
    expect(boundedActions).toHaveLength(pointerCase.actionCount);
    bounded.unmount();

    vi.useFakeTimers();
    try {
      const hold = stepperEditingFixture.hold;
      const heldActions: number[] = [];
      const held = render(h(Stepper, { id: "stepper.hold", value: hold.start, min: uniform.min, max: uniform.max, step: uniform.step, onChange: (value: number) => heldActions.push(value) }));
      const heldInput = held.container.querySelector<HTMLInputElement>("[data-stepper-input='true']")!;
      const plus = held.container.querySelector<HTMLButtonElement>("[data-slot='stepper-plus']")!;
      fireEvent.mouseDown(plus);
      expect(heldInput.value).toBe(String(hold.atDelay));
      act(() => vi.advanceTimersByTime(hold.delayMs));
      expect(heldInput.value).toBe(String(hold.atDelay));
      act(() => vi.advanceTimersByTime(hold.intervalMs));
      expect(heldInput.value).toBe(String(hold.atFirstInterval));
      act(() => vi.advanceTimersByTime(hold.intervalMs));
      expect(heldInput.value).toBe(String(hold.atSecondInterval));
      expect(heldActions).toHaveLength(hold.actionCount);
      fireEvent.mouseUp(plus);
      act(() => vi.advanceTimersByTime(hold.intervalMs * 4));
      expect(heldInput.value).toBe(String(hold.atSecondInterval));
      held.unmount();
    } finally {
      vi.useRealTimers();
    }
  });
  test("the actual React Slider matches readout editing, keyboard and accessibility behavior", () => {
    const contract = sliderEditingFixture;
    const changes: number[][] = [];
    const commits: number[][] = [];
    const props = {
      id: "slider.edit",
      "aria-label": contract.accessibility.label,
      min: contract.slider.min,
      max: contract.slider.max,
      step: contract.slider.step,
      value: [contract.slider.value],
      onValueChange: (values: number[]) => changes.push(values),
      onValueCommit: (values: number[]) => commits.push(values),
    };
    const view = render(h(Slider, props));
    const readout = view.container.querySelector<HTMLElement>("[data-slot='slider-value']")!;
    const thumb = view.getByRole(contract.accessibility.thumbRole, { name: contract.accessibility.label });
    expect(readout.getAttribute("role")).toBe(contract.accessibility.readoutRole);
    expect(thumb.getAttribute("aria-valuemin")).toBe(String(contract.slider.min));
    expect(thumb.getAttribute("aria-valuemax")).toBe(String(contract.slider.max));
    expect(thumb.getAttribute("aria-valuenow")).toBe(String(contract.slider.value));
    fireEvent.doubleClick(readout);
    const editor = view.getByRole(contract.accessibility.editorRole) as HTMLInputElement;
    expect(editor.value).toBe(String(contract.slider.value));
    expect(editor.min).toBe(String(contract.slider.min));
    expect(editor.max).toBe(String(contract.slider.max));
    fireEvent.change(editor, { target: { value: contract.enter.input } });
    fireEvent.keyDown(editor, { key: "Enter" });
    expect(changes).toEqual([[contract.enter.expectedValue]]);
    expect(commits).toEqual([[contract.enter.expectedValue]]);
    expect(view.container.querySelector("[data-slot='slider-value']")?.textContent).toBe(contract.enter.expectedDisplay);
    view.rerender(h(Slider, props));
    expect(view.container.querySelector("[data-slot='slider-value']")?.textContent).toBe(contract.enter.staleDeclaredDisplay);
    view.rerender(h(Slider, { ...props, value: [contract.enter.expectedValue] }));
    expect(view.container.querySelector("[data-slot='slider-value']")?.textContent).toBe(contract.enter.expectedDisplay);
    view.unmount();

    for (const rejected of contract.rejected) {
      const rejectedChanges: number[][] = [];
      const rejectedView = render(h(Slider, { ...props, id: `slider.${rejected.id}`, onValueChange: (values: number[]) => rejectedChanges.push(values), onValueCommit: undefined }));
      fireEvent.doubleClick(rejectedView.container.querySelector("[data-slot='slider-value']")!);
      const rejectedEditor = rejectedView.getByRole(contract.accessibility.editorRole) as HTMLInputElement;
      fireEvent.change(rejectedEditor, { target: { value: rejected.input } });
      if (rejected.exit === "blur") fireEvent.blur(rejectedEditor);
      else fireEvent.keyDown(rejectedEditor, { key: rejected.exit });
      expect(rejectedChanges, rejected.id).toHaveLength(rejected.actionCount);
      if (rejected.editing) {
        expect([rejectedEditor.value, rejectedEditor.getAttribute("aria-invalid")], rejected.id).toEqual([rejected.expectedDisplay, "true"]);
      } else {
        expect(rejectedView.container.querySelector("[data-slot='slider-value']")?.textContent, rejected.id).toBe(rejected.expectedDisplay);
      }
      rejectedView.unmount();
    }

    for (const keyCase of contract.keyboard) {
      const keyedChanges: number[][] = [];
      const keyedCommits: number[][] = [];
      const keyed = render(h(Slider, { ...props, id: `slider.${keyCase.key}`, onValueChange: (values: number[]) => keyedChanges.push(values), onValueCommit: (values: number[]) => keyedCommits.push(values) }));
      const keyedThumb = keyed.getByRole(contract.accessibility.thumbRole, { name: contract.accessibility.label });
      fireEvent.focus(keyedThumb);
      fireEvent.keyDown(keyedThumb, { key: keyCase.key });
      expect(keyedChanges).toEqual([[keyCase.expectedValue]]);
      fireEvent.keyUp(keyedThumb, { key: keyCase.key });
      expect(keyedCommits).toEqual([[keyCase.expectedValue]]);
      keyed.unmount();
    }
  });
  test("an independent upward-flow oracle reserves bottom tabs outside retained content", () => {
    expect(bottomGeometry(fixture.bottomPanel)).toEqual(fixture.bottomPanel.expected);
    const contentBottom = fixture.bottomPanel.expected.content.y + fixture.bottomPanel.expected.content.h;
    expect(contentBottom).toBeLessThanOrEqual(fixture.bottomPanel.expected.leafTabY);
    expect(fixture.bottomPanel.expected.rootTabY).toBeGreaterThan(fixture.bottomPanel.expected.leafTabY);
  });
}
