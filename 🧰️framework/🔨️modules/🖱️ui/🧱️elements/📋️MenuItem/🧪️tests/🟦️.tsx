// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { MenuItem, menuItemClassName } from "../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 📋️MenuItemMatrix
describe("MenuItem", () => {
  it.each([false, true])("owns button-like menu-row semantics when disabled is %s", (disabled) => {
    const onClick = vi.fn();
    const ref = React.createRef<HTMLButtonElement>();
    const { getByRole } = render(
      <div role="menu">
        <MenuItem ref={ref} disabled={disabled} className="consumer-class" onClick={onClick}>
          Run
        </MenuItem>
      </div>,
    );
    const item = getByRole("menuitem") as HTMLButtonElement;

    expect(ref.current).toBe(item);
    expect(item.type).toBe("button");
    expect(item.disabled).toBe(disabled);
    expect(item.getAttribute("aria-disabled")).toBe(disabled ? "true" : null);
    expect(item.className).toContain(menuItemClassName);
    expect(item.className).toContain("consumer-class");
    fireEvent.click(item);
    expect(onClick).toHaveBeenCalledTimes(disabled ? 0 : 1);
  });
});
// #endregion 📋️MenuItemMatrix
