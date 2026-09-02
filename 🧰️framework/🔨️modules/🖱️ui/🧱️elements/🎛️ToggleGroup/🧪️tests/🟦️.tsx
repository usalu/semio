// #region 🔌️Adapters
import { act } from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ToggleGroup } from "../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🎛️Fixture
const items = [
  { value: "alpha", id: "toggle-alpha", icon: "x" as const, text: "Alpha" },
  { value: "disabled", id: "toggle-disabled", icon: "x" as const, text: "Disabled", disabled: true },
  { value: "beta", id: "toggle-beta", icon: "x" as const, text: "Beta" },
  { value: "gamma", id: "toggle-gamma", icon: "x" as const, text: "Gamma" },
];
// #endregion 🎛️Fixture

// #region 🎛️ToggleGroupMatrix
describe("ToggleGroup", () => {
  it("owns uncontrolled single selection and exact pressed state", () => {
    const changes = vi.fn();
    const { getByRole } = render(<ToggleGroup defaultValue="alpha" onValueChange={changes} items={items} />);
    const alpha = getByRole("button", { name: "Alpha" });
    const beta = getByRole("button", { name: "Beta" });
    expect(alpha.getAttribute("aria-pressed")).toBe("true");
    expect(alpha.getAttribute("data-state")).toBe("on");
    fireEvent.click(beta);
    expect(changes).toHaveBeenCalledWith("beta");
    expect(alpha.getAttribute("aria-pressed")).toBe("false");
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith("");
    expect(beta.getAttribute("aria-pressed")).toBe("false");
  });

  it("emits controlled single lag proposals without duplicate callbacks or local mutation", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(<ToggleGroup value="alpha" onValueChange={changes} items={items} />);
    const beta = getByRole("button", { name: "Beta" });
    fireEvent.click(beta);
    fireEvent.click(beta);
    fireEvent.click(beta);
    expect(changes.mock.calls).toEqual([["beta"], ["beta"], ["beta"]]);
    expect(beta.getAttribute("aria-pressed")).toBe("false");
    rerender(<ToggleGroup value="beta" onValueChange={changes} items={items} />);
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith("");
  });

  it("owns uncontrolled and controlled multiple arrays", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(<ToggleGroup kind="multiple" defaultValue={["alpha"]} onValueChange={changes} items={items} />);
    const beta = getByRole("button", { name: "Beta" });
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith(["alpha", "beta"]);
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    rerender(<ToggleGroup kind="multiple" value={[]} onValueChange={changes} items={items} />);
    fireEvent.click(beta);
    fireEvent.click(beta);
    expect(changes.mock.calls.slice(-2)).toEqual([[["beta"]], [["beta"]]]);
    expect(beta.getAttribute("aria-pressed")).toBe("false");
  });

  it("suppresses group/item disabled activation and owns generated distinct IDs across groups", () => {
    const changes = vi.fn();
    const { container, rerender } = render(
      <div>
        <ToggleGroup
          id="group-one"
          onValueChange={changes}
          items={[
            { value: "alpha", icon: "x", text: "Alpha" },
            { value: "beta", icon: "x", text: "Beta" },
          ]}
        />
        <ToggleGroup
          id="group-two"
          items={[
            { value: "alpha", icon: "x", text: "Other Alpha" },
            { value: "beta", icon: "x", text: "Other Beta" },
          ]}
        />
      </div>,
    );
    const alpha = container.querySelector<HTMLButtonElement>("#group-one-alpha")!;
    const beta = container.querySelector<HTMLButtonElement>("#group-one-beta")!;
    expect(alpha.id).toBe("group-one-alpha");
    expect(beta.id).toBe("group-one-beta");
    expect(container.querySelector<HTMLButtonElement>("#group-two-alpha")?.id).toBe("group-two-alpha");
    expect(new Set(Array.from(container.querySelectorAll<HTMLButtonElement>('[data-slot="toggle-group-item"]')).map((item) => item.id)).size).toBe(4);
    rerender(
      <ToggleGroup
        id="group-one"
        disabled
        onValueChange={changes}
        items={[
          { value: "alpha", icon: "x", text: "Alpha" },
          { value: "beta", icon: "x", text: "Beta" },
        ]}
      />,
    );
    const disabledAlpha = container.querySelector<HTMLButtonElement>("#group-one-alpha")!;
    fireEvent.click(disabledAlpha);
    expect(disabledAlpha.disabled).toBe(true);
    expect(disabledAlpha.tabIndex).toBe(-1);
    expect(changes).not.toHaveBeenCalled();
  });

  it("roves horizontally with RTL, skips disabled items, and honors loop false", () => {
    const { getByRole, rerender } = render(<ToggleGroup value="alpha" dir="rtl" items={items} />);
    const alpha = getByRole("button", { name: "Alpha" });
    const beta = getByRole("button", { name: "Beta" });
    const gamma = getByRole("button", { name: "Gamma" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(beta);
    fireEvent.keyDown(beta, { key: "End" });
    expect(document.activeElement).toBe(gamma);
    rerender(<ToggleGroup value="alpha" dir="ltr" loop={false} items={items} />);
    const ltrAlpha = getByRole("button", { name: "Alpha" });
    ltrAlpha.focus();
    fireEvent.keyDown(ltrAlpha, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(ltrAlpha);
  });

  it("roves vertically and keeps the action as a keyboard-focusable sibling", () => {
    const changes = vi.fn();
    const action = vi.fn();
    const { getByRole } = render(
      <ToggleGroup
        orientation="vertical"
        onValueChange={changes}
        items={[
          {
            value: "alpha",
            icon: "x",
            text: "Alpha",
            action: (
              <button type="button" onClick={action}>
                Action
              </button>
            ),
          },
          { value: "beta", icon: "x", text: "Beta" },
        ]}
      />,
    );
    const alpha = document.querySelector<HTMLButtonElement>('[data-toggle-value="alpha"]')!;
    const beta = getByRole("button", { name: "Beta" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowDown" });
    expect(document.activeElement).toBe(beta);
    const actionButton = getByRole("button", { name: "Action" });
    expect(alpha.contains(actionButton)).toBe(false);
    expect(document.querySelector("button button, button a, button input")).toBeNull();
    actionButton.focus();
    expect(document.activeElement).toBe(actionButton);
    fireEvent.keyDown(actionButton, { key: "Enter" });
    fireEvent.keyUp(actionButton, { key: "Enter" });
    act(() => actionButton.click());
    expect(action).toHaveBeenCalledTimes(1);
    expect(changes).not.toHaveBeenCalled();
  });
});
// #endregion 🎛️ToggleGroupMatrix
