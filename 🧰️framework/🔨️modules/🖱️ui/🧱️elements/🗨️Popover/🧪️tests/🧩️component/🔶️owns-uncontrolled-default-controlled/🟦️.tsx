/** 🔶️ The owns uncontrolled default controlled tests of the Popover suite. */
import * as React from "react";
import { fireEvent, render, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger } from "../../../🟦️.tsx";

/** 🔷 Registers the owns uncontrolled default controlled tests of the Popover suite. */
export function registerOwnsUncontrolledDefaultControlledTests(): void {
  it("owns uncontrolled, default, and controlled-lag state and trigger associations", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(
      <Popover defaultOpen onOpenChange={changes}>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent>Body</PopoverContent>
      </Popover>,
    );
    const trigger = getByRole("button", { name: "Open" });
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(getByRole("dialog").id);
    expect(trigger.getAttribute("data-state")).toBe("open");
    fireEvent.click(trigger);
    expect(changes).toHaveBeenLastCalledWith(false);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(document.querySelector('[data-slot="popover-content"]')).toBeNull();

    rerender(
      <Popover open={false} onOpenChange={changes}>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent>Body</PopoverContent>
      </Popover>,
    );
    fireEvent.click(trigger);
    expect(changes).toHaveBeenLastCalledWith(true);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(document.querySelector('[data-slot="popover-content"]')).toBeNull();
  });
  it("composes asChild refs and events and honors prevented child activation", () => {
    const ref = React.createRef<HTMLButtonElement>();
    const childClick = vi.fn((event: React.MouseEvent) => event.preventDefault());
    const change = vi.fn();
    const { getByRole } = render(
      <Popover onOpenChange={change}>
        <PopoverTrigger asChild ref={ref} onClick={vi.fn()}>
          <button type="button" onClick={childClick}>
            Child
          </button>
        </PopoverTrigger>
        <PopoverContent>Body</PopoverContent>
      </Popover>,
    );
    const child = getByRole("button", { name: "Child" });
    expect(ref.current).toBe(child);
    fireEvent.click(child);
    expect(childClick).toHaveBeenCalledTimes(1);
    expect(change).not.toHaveBeenCalled();
    expect(document.querySelector("button button, button a, button input")).toBeNull();
  });
  it("suppresses disabled trigger activation", () => {
    const change = vi.fn();
    const { getByRole } = render(
      <Popover onOpenChange={change}>
        <PopoverTrigger disabled>Disabled</PopoverTrigger>
        <PopoverContent>Body</PopoverContent>
      </Popover>,
    );
    const trigger = getByRole("button", { name: "Disabled" }) as HTMLButtonElement;
    expect(trigger.disabled).toBe(true);
    fireEvent.click(trigger);
    expect(change).not.toHaveBeenCalled();
  });
  it("focuses content, supports prevented autofocus, dismisses on Escape, and returns focus", async () => {
    const closeFocus = vi.fn();
    const { getByRole } = render(
      <Popover>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent onCloseAutoFocus={closeFocus}>
          <button type="button">Inside</button>
        </PopoverContent>
      </Popover>,
    );
    const trigger = getByRole("button", { name: "Open" });
    trigger.focus();
    fireEvent.click(trigger);
    expect(document.activeElement).toBe(getByRole("button", { name: "Inside" }));
    fireEvent.keyDown(document, { key: "Escape" });
    await waitFor(() => expect(document.querySelector('[role="dialog"]')).toBeNull());
    expect(closeFocus).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(trigger);

    const prevented = render(
      <Popover defaultOpen>
        <PopoverAnchor asChild>
          <input aria-label="Search" />
        </PopoverAnchor>
        <PopoverContent onOpenAutoFocus={(event) => event.preventDefault()}>
          <button type="button">Suggestion</button>
        </PopoverContent>
      </Popover>,
    );
    const search = prevented.getByRole("textbox", { name: "Search" });
    search.focus();
    expect(document.activeElement).toBe(search);
  });
  it("dismisses pointer and focus outside, allows prevention, and ignores its logical nested boundary", () => {
    const parentChange = vi.fn();
    const childChange = vi.fn();
    render(
      <Popover defaultOpen onOpenChange={parentChange}>
        <PopoverTrigger>Parent</PopoverTrigger>
        <PopoverContent>
          <Popover defaultOpen onOpenChange={childChange}>
            <PopoverTrigger>Child</PopoverTrigger>
            <PopoverContent>
              <button type="button">Nested action</button>
            </PopoverContent>
          </Popover>
        </PopoverContent>
      </Popover>,
    );
    fireEvent.pointerDown(document.querySelector("[data-popover-boundary] button")!);
    expect(parentChange).not.toHaveBeenCalled();
    expect(childChange).not.toHaveBeenCalled();
    fireEvent.pointerDown(document.body);
    expect(parentChange).toHaveBeenCalledWith(false);
    expect(childChange).toHaveBeenCalledWith(false);

    const focusChange = vi.fn();
    const outside = document.createElement("button");
    document.body.appendChild(outside);
    render(
      <Popover defaultOpen onOpenChange={focusChange}>
        <PopoverTrigger>Focus owner</PopoverTrigger>
        <PopoverContent onPointerDownOutside={(event) => event.preventDefault()}>Prevented</PopoverContent>
      </Popover>,
    );
    fireEvent.pointerDown(outside);
    expect(focusChange).not.toHaveBeenCalled();
    fireEvent.focusIn(outside);
    expect(focusChange).toHaveBeenCalledWith(false);
  });
  it("dismisses only the topmost nested popover on each Escape", () => {
    const parentChange = vi.fn();
    const childChange = vi.fn();
    render(
      <Popover defaultOpen onOpenChange={parentChange}>
        <PopoverTrigger>Parent</PopoverTrigger>
        <PopoverContent>
          <Popover defaultOpen onOpenChange={childChange}>
            <PopoverTrigger>Child</PopoverTrigger>
            <PopoverContent>Nested</PopoverContent>
          </Popover>
        </PopoverContent>
      </Popover>,
    );
    expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(2);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(childChange).toHaveBeenCalledWith(false);
    expect(parentChange).not.toHaveBeenCalled();
    expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(1);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(parentChange).toHaveBeenCalledWith(false);
  });
}
