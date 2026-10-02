// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import numberControlsFixture from "../../../../🧬️contract/🧫️fixtures/🧫️number-controls/🔣️.json";
import { Stepper } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🪜️StepperMatrix
describe("Stepper", () => {
  it("answers every shared keyboard-law row of an unbounded or bounded field through the arrow, page, Home and End keys", () => {
    const physical = { increment: "ArrowUp", decrement: "ArrowDown", pageUp: "PageUp", pageDown: "PageDown", home: "Home", end: "End" } as const;
    expect(new Set(numberControlsFixture.keys.map((row) => row.key))).toEqual(new Set(Object.keys(physical)));
    for (const row of numberControlsFixture.keys) {
      const precision = "precision" in row ? row.precision : null;
      const factor = "factor" in row ? row.factor : null;
      const change = vi.fn();
      const { getByRole, unmount } = render(<Stepper id={`stepper.${row.case}`} aria-label={row.case} value={row.current} min={row.min ?? undefined} max={row.max ?? undefined} step={row.step} precision={precision ?? undefined} displayFactor={factor} snapValues={row.snaps} onChange={change} />);
      const caret = (row.key === "home" && row.min === null) || (row.key === "end" && row.max === null);
      const pressed = fireEvent.keyDown(getByRole("spinbutton"), { key: physical[row.key as keyof typeof physical], shiftKey: row.large });
      if (caret) expect([pressed, change.mock.calls.length, row.expected], `${row.case}: the caret keeps a Home/End without its bound`).toEqual([true, 0, row.current]);
      else expect(change, row.case).toHaveBeenLastCalledWith(row.expected);
      unmount();
    }
  });

  it("shows a radian value in degrees with its unit and reads a typed 90 back as π/2 exactly", () => {
    const change = vi.fn();
    const { getByRole, container } = render(<Stepper id="stepper.heading" aria-label="Heading" value={Math.PI / 4} step={Math.PI / 180} displayFactor={180 / Math.PI} unit="°" snapValues={[0, Math.PI / 2]} aria-valuetext="45 °" onChange={change} />);
    const field = getByRole("spinbutton") as HTMLInputElement;
    expect(field.value).toBe("45");
    expect(field.getAttribute("aria-valuetext")).toBe("45 °");
    expect(container.querySelector('[data-slot="stepper-unit"]')!.textContent).toBe("°");
    fireEvent.focus(field);
    fireEvent.change(field, { target: { value: "90" } });
    expect(change).toHaveBeenLastCalledWith(Math.PI / 2);
  });

  it("refuses a typed value beyond a hard bound visibly, keeping the draft and dispatching nothing", () => {
    const change = vi.fn();
    const { getByRole, queryByRole } = render(<Stepper id="stepper.gap" aria-label="Gap" value={2} step={1} precision={1} unit="mm" snapValues={[0, 5, 10]} limits={{ max: { value: 10, refusal: "Must be at most 10 mm" } }} onChange={change} />);
    const field = getByRole("spinbutton") as HTMLInputElement;
    fireEvent.focus(field);
    fireEvent.change(field, { target: { value: "12" } });
    expect(getByRole("alert").textContent).toBe("Must be at most 10 mm");
    expect(field.getAttribute("aria-invalid")).toBe("true");
    expect(field.getAttribute("aria-describedby")).toBe("stepper-gap-refusal");
    expect(field.value).toBe("12");
    expect(change).not.toHaveBeenCalled();
    fireEvent.change(field, { target: { value: "7" } });
    expect(queryByRole("alert")).toBeNull();
    expect(change).toHaveBeenLastCalledWith(7);
    fireEvent.keyDown(field, { key: "PageUp" });
    expect(change).toHaveBeenLastCalledWith(10);
  });
});
// #endregion 🪜️StepperMatrix
