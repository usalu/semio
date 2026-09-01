// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Checkbox, type CheckboxState } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region ☑️CheckboxMatrix
describe("Checkbox", () => {
  it.each([
    { state: true, checked: true, indeterminate: false, ariaChecked: "true", submitted: "enabled" },
    { state: false, checked: false, indeterminate: false, ariaChecked: "false", submitted: null },
    { state: "indeterminate", checked: false, indeterminate: true, ariaChecked: "mixed", submitted: null },
  ] as const)("synchronizes the $state state with DOM, ARIA, and form data", ({ state, checked, indeterminate, ariaChecked, submitted }) => {
    const ref = React.createRef<HTMLInputElement>();
    const { container } = render(
      <form>
        <Checkbox ref={ref} name="feature" value="enabled" checked={state as CheckboxState} onChange={() => undefined} />
      </form>,
    );
    const form = container.querySelector("form")!;

    expect(ref.current?.checked).toBe(checked);
    expect(ref.current?.indeterminate).toBe(indeterminate);
    expect(ref.current?.getAttribute("aria-checked")).toBe(ariaChecked);
    expect(new FormData(form).get("feature")).toBe(submitted);
  });

  it("forwards native change events and its input ref", () => {
    const checkedValues: boolean[] = [];
    const onChange = vi.fn((event: React.ChangeEvent<HTMLInputElement>) => checkedValues.push(event.target.checked));
    const ref = React.createRef<HTMLInputElement>();
    const { getByRole } = render(<Checkbox ref={ref} checked={false} onChange={onChange} aria-label="Feature" />);
    const checkbox = getByRole("checkbox") as HTMLInputElement;

    fireEvent.click(checkbox);
    expect(ref.current).toBe(checkbox);
    expect(onChange).toHaveBeenCalledOnce();
    expect(checkedValues).toEqual([true]);
  });
});
// #endregion ☑️CheckboxMatrix
