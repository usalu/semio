// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ActionDropdown } from "../../⚡️ActionGroup/🟦️component.tsx";
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger, resolvePopoverPlacement } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗨️PopoverMatrix
describe("Popover", () => {
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

  it("routes sibling Escape by focus and last activation instead of portal order", () => {
    const firstChange = vi.fn();
    const secondChange = vi.fn();
    const { getByRole } = render(
      <>
        <Popover open onOpenChange={firstChange}>
          <PopoverTrigger>First trigger</PopoverTrigger>
          <PopoverContent>
            <button type="button">First action</button>
          </PopoverContent>
        </Popover>
        <Popover open onOpenChange={secondChange}>
          <PopoverTrigger>Second trigger</PopoverTrigger>
          <PopoverContent>
            <button type="button">Second action</button>
          </PopoverContent>
        </Popover>
      </>,
    );
    const firstAction = getByRole("button", { name: "First action" });
    firstAction.focus();
    firstChange.mockClear();
    secondChange.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(firstChange).toHaveBeenCalledWith(false);
    expect(secondChange).not.toHaveBeenCalled();

    const secondTrigger = getByRole("button", { name: "Second trigger" });
    fireEvent.pointerDown(secondTrigger);
    fireEvent.click(secondTrigger);
    firstChange.mockClear();
    secondChange.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(secondChange).toHaveBeenCalledWith(false);
    expect(firstChange).not.toHaveBeenCalled();
  });

  it("unmounts closed content so hidden descendants cannot run effects", () => {
    const lifecycle: string[] = [];
    function Effect(): React.ReactElement {
      React.useEffect(() => {
        lifecycle.push("mount");
        return () => lifecycle.push("cleanup");
      }, []);
      return <span>Effect</span>;
    }
    const { getByRole } = render(
      <Popover>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent>
          <Effect />
        </PopoverContent>
      </Popover>,
    );
    expect(lifecycle).toEqual([]);
    fireEvent.click(getByRole("button", { name: "Open" }));
    expect(lifecycle).toEqual(["mount"]);
    fireEvent.click(getByRole("button", { name: "Open" }));
    expect(lifecycle).toEqual(["mount", "cleanup"]);
  });

  it("resolves every used side and alignment with offsets", () => {
    const anchor = { top: 100, right: 140, bottom: 120, left: 100, width: 40, height: 20 };
    const content = { width: 60, height: 30 };
    const expected: Record<string, [number, number]> = {
      "top-start": [103, 65],
      "top-center": [93, 65],
      "top-end": [83, 65],
      "bottom-start": [103, 125],
      "bottom-center": [93, 125],
      "bottom-end": [83, 125],
      "left-start": [35, 103],
      "left-center": [35, 98],
      "left-end": [35, 93],
      "right-start": [145, 103],
      "right-center": [145, 98],
      "right-end": [145, 93],
    };
    for (const side of ["top", "right", "bottom", "left"] as const) {
      for (const align of ["start", "center", "end"] as const) {
        const result = resolvePopoverPlacement(anchor, content, { width: 500, height: 400 }, side, align, 5, 3, 8, false, false);
        expect([result.left, result.top], `${side}-${align}`).toEqual(expected[`${side}-${align}`]);
      }
    }
    expect(resolvePopoverPlacement(anchor, content, { width: 500, height: 400 }, "bottom", "start", 5, 3, 8, true, false).left).toBe(83);
  });

  it("flips and clamps at viewport collisions and remeasures anchor updates", async () => {
    const flipped = resolvePopoverPlacement({ top: 180, right: 35, bottom: 200, left: 5, width: 30, height: 20 }, { width: 90, height: 60 }, { width: 120, height: 220 }, "bottom", "start", 4, 0, 8, false, true);
    expect(flipped.side).toBe("top");
    expect(flipped.left).toBe(8);

    let resize: (() => void) | undefined;
    const OriginalObserver = globalThis.ResizeObserver;
    globalThis.ResizeObserver = class {
      constructor(callback: ResizeObserverCallback) {
        resize = () => callback([], this as ResizeObserver);
      }
      observe() {}
      unobserve() {}
      disconnect() {}
    };
    try {
      const anchorRect = { current: { top: 10, right: 30, bottom: 30, left: 10, width: 20, height: 20 } };
      render(
        <Popover defaultOpen>
          <PopoverAnchor asChild>
            <div data-testid="anchor" />
          </PopoverAnchor>
          <PopoverContent avoidCollisions={false}>Body</PopoverContent>
        </Popover>,
      );
      const anchorElement = document.querySelector('[data-testid="anchor"]') as HTMLElement;
      const contentElement = document.querySelector('[role="dialog"]') as HTMLElement;
      anchorElement.getBoundingClientRect = () => anchorRect.current as DOMRect;
      contentElement.getBoundingClientRect = () => ({ width: 40, height: 20 }) as DOMRect;
      anchorRect.current = { top: 50, right: 80, bottom: 70, left: 60, width: 20, height: 20 };
      resize?.();
      await waitFor(() => expect(contentElement.style.left).toBe("50px"));
      expect(contentElement.style.top).toBe("74px");
    } finally {
      globalThis.ResizeObserver = OriginalObserver;
    }
  });

  it("renders the real ActionDropdown with a button trigger and no nested interactive control", () => {
    const changed = vi.fn();
    const { container, getByRole } = render(
      <ActionDropdown
        id="ui.test.action-dropdown"
        value="first"
        onValueChange={changed}
        options={[
          { value: "first", icon: "x", label: "First" as never },
          { value: "second", icon: "plus", label: "Second" as never },
        ]}
      />,
    );
    expect(container.querySelector("button button, button a, button input")).toBeNull();
    fireEvent.click(container.querySelector('[data-slot="popover-trigger"]')!);
    fireEvent.click(getByRole("button", { name: "Second" }));
    expect(changed).toHaveBeenCalledWith("second");
  });
});
// #endregion 🗨️PopoverMatrix
