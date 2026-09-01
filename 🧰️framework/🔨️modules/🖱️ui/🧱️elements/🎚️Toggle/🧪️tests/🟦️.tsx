// #region 🔌️Adapters
import * as React from "react";
import { act } from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Toggle } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗡️ToggleMatrix
describe("Toggle", () => {
  it("updates uncontrolled pressed state and proposes controlled changes", () => {
    const uncontrolledChange = vi.fn();
    const { getByRole, rerender } = render(<Toggle id="test.uncontrolled" icon="x" defaultPressed onPressedChange={uncontrolledChange} />);
    const button = getByRole("button") as HTMLButtonElement;

    expect(button.getAttribute("aria-pressed")).toBe("true");
    expect(button.getAttribute("data-state")).toBe("on");
    fireEvent.click(button);
    expect(button.getAttribute("aria-pressed")).toBe("false");
    expect(uncontrolledChange).toHaveBeenLastCalledWith(false);

    const controlledChange = vi.fn();
    rerender(<Toggle id="test.controlled" icon="x" pressed={false} onPressedChange={controlledChange} />);
    fireEvent.click(button);
    expect(controlledChange).toHaveBeenCalledWith(true);
    expect(button.getAttribute("aria-pressed")).toBe("false");
    rerender(<Toggle id="test.controlled" icon="x" pressed onPressedChange={controlledChange} />);
    expect(button.getAttribute("aria-pressed")).toBe("true");
  });

  it("activates once for Enter and once for Space", () => {
    const changes = vi.fn();
    const { getByRole } = render(<Toggle id="test.keyboard" icon="x" onPressedChange={changes} />);
    const button = getByRole("button");

    fireEvent.keyDown(button, { key: "Enter" });
    expect(button.getAttribute("aria-pressed")).toBe("true");
    expect(changes).toHaveBeenLastCalledWith(true);

    fireEvent.keyDown(button, { key: " " });
    expect(button.getAttribute("aria-pressed")).toBe("true");
    fireEvent.keyUp(button, { key: " " });
    expect(button.getAttribute("aria-pressed")).toBe("false");
    expect(changes).toHaveBeenLastCalledWith(false);
    expect(changes).toHaveBeenCalledTimes(2);
  });

  it("uses a native non-submitting button and suppresses disabled activation", () => {
    const onPressedChange = vi.fn();
    const { getByRole } = render(<Toggle id="test.disabled" icon="x" disabled onPressedChange={onPressedChange} />);
    const button = getByRole("button") as HTMLButtonElement;

    expect(button.tagName).toBe("BUTTON");
    expect(button.type).toBe("button");
    expect(button.disabled).toBe(true);
    fireEvent.click(button);
    fireEvent.keyDown(button, { key: "Enter" });
    fireEvent.keyDown(button, { key: " " });
    fireEvent.keyUp(button, { key: " " });
    expect(onPressedChange).not.toHaveBeenCalled();
    expect(button.getAttribute("aria-pressed")).toBe("false");
  });

  it("isolates the action branch from toggle selection", () => {
    const onPressedChange = vi.fn();
    const onActionClick = vi.fn();
    const primaryRef = React.createRef<HTMLButtonElement>();
    const { container } = render(<Toggle ref={primaryRef} id="test.action" kind="withAction" icon="x" actionIcon="plus" actionId="test.action.secondary" onActionClick={onActionClick} onPressedChange={onPressedChange} />);
    const item = container.querySelector('[data-slot="toggle-group-item"]') as HTMLButtonElement;
    const action = container.querySelector('[data-slot="action"]') as HTMLButtonElement;

    expect(item).toBeTruthy();
    expect(primaryRef.current).toBe(item);
    expect(action.tagName).toBe("BUTTON");
    expect(item.contains(action)).toBe(false);
    expect(container.querySelector("button button, button a, button input")).toBeNull();
    action.focus();
    expect(document.activeElement).toBe(action);
    fireEvent.keyDown(action, { key: "Enter" });
    fireEvent.keyUp(action, { key: "Enter" });
    act(() => action.click());
    expect(onActionClick).toHaveBeenCalledTimes(1);
    expect(onPressedChange).not.toHaveBeenCalled();
    expect(item.getAttribute("aria-pressed")).toBe("false");
    item.focus();
    expect(document.activeElement).toBe(item);
    act(() => item.click());
    expect(onPressedChange).toHaveBeenCalledWith(true);
  });

  it("isolates the dropdown trigger from toggle selection", () => {
    const onPressedChange = vi.fn();
    const onOpenChange = vi.fn();
    const onValueChange = vi.fn();
    const { container, getByRole } = render(
      <Toggle
        id="test.dropdown"
        kind="dropdown"
        dropdownId="test.dropdown.trigger"
        defaultValue="first"
        onPressedChange={onPressedChange}
        onOpenChange={onOpenChange}
        onValueChange={onValueChange}
        items={[
          { value: "first", icon: "x", text: "First" },
          { value: "second", icon: "plus", text: "Second" },
        ]}
      />,
    );

    expect(container.querySelector('[data-slot="toggle-group-item"]')).toBeTruthy();
    const item = container.querySelector('[data-slot="toggle-group-item"]') as HTMLButtonElement;
    const trigger = document.getElementById("test.dropdown.trigger") as HTMLButtonElement;
    expect(trigger).toBeTruthy();
    expect(trigger.tagName).toBe("BUTTON");
    expect(item.contains(trigger)).toBe(false);
    expect(container.querySelector("button button, button a, button input")).toBeNull();
    trigger.focus();
    expect(document.activeElement).toBe(trigger);
    fireEvent.keyDown(trigger, { key: "Enter" });
    fireEvent.keyUp(trigger, { key: "Enter" });
    act(() => trigger.click());
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(onPressedChange).not.toHaveBeenCalled();
    expect(item.getAttribute("aria-pressed")).toBe("false");
    const second = getByRole("button", { name: "Second" });
    second.focus();
    expect(document.activeElement).toBe(second);
    act(() => second.click());
    expect(onValueChange).toHaveBeenCalledWith("second");
    expect(onPressedChange).not.toHaveBeenCalled();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
  });

  it("disables both primary and secondary controls in action branches", () => {
    const onPressedChange = vi.fn();
    const onActionClick = vi.fn();
    const { container } = render(<Toggle disabled id="test.action.disabled" kind="withAction" icon="x" actionIcon="plus" onActionClick={onActionClick} onPressedChange={onPressedChange} />);
    const buttons = Array.from(container.querySelectorAll<HTMLButtonElement>("button"));
    expect(buttons).toHaveLength(2);
    expect(buttons.every((button) => button.disabled)).toBe(true);
    buttons.forEach((button) => button.click());
    expect(onActionClick).not.toHaveBeenCalled();
    expect(onPressedChange).not.toHaveBeenCalled();
  });
});
// #endregion 🗡️ToggleMatrix
