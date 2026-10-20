/** 🔶️ The owns uncontrolled state exact tests of the Dialog suite. */
import * as React from "react";
import { fireEvent, render, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle, DialogTrigger } from "../../../🟦️.tsx";

/** 🔷 Registers the owns uncontrolled state exact tests of the Dialog suite. */
export function registerOwnsUncontrolledStateExactTests(): void {
  it("owns uncontrolled state, exact slots, stable associations, modal isolation, and focus return", async () => {
    const changes = vi.fn();
    const triggerRef = React.createRef<HTMLButtonElement>();
    const { container, getByRole } = render(
      <Dialog onOpenChange={changes}>
        <DialogTrigger asChild ref={triggerRef}>
          <button type="button">Open editor</button>
        </DialogTrigger>
        <DialogContent>
          <DialogTitle>Edit capsule</DialogTitle>
          <DialogDescription>Change the capsule settings.</DialogDescription>
          <input aria-label="Capsule name" />
        </DialogContent>
      </Dialog>,
    );
    const trigger = getByRole("button", { name: "Open editor" });
    expect(triggerRef.current).toBe(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    trigger.focus();
    fireEvent.click(trigger);
    const content = getByRole("dialog", { name: "Edit capsule" });
    expect(changes).toHaveBeenLastCalledWith(true);
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(content.id);
    expect(content.getAttribute("aria-modal")).toBe("true");
    expect(content.getAttribute("aria-labelledby")).toBe(getByRole("heading", { name: "Edit capsule" }).id);
    expect(content.getAttribute("aria-describedby")).toBe(document.querySelector('[data-slot="dialog-description"]')?.id);
    expect(document.activeElement).toBe(getByRole("textbox", { name: "Capsule name" }));
    expect(container.getAttribute("aria-hidden")).toBe("true");
    expect(container.hasAttribute("inert")).toBe(true);
    expect(document.body.style.overflow).toBe("hidden");
    expect(document.querySelectorAll('[data-slot="dialog-portal"]')).toHaveLength(1);
    fireEvent.click(getByRole("button", { name: /close/i }));
    await waitFor(() => expect(document.querySelector('[role="dialog"]')).toBeNull());
    expect(document.activeElement).toBe(trigger);
    expect(container.hasAttribute("aria-hidden")).toBe(false);
    expect(container.hasAttribute("inert")).toBe(false);
    expect(document.body.style.overflow).toBe("");
    expect(document.querySelector('[data-slot="dialog-portal"]')).toBeNull();
  });
  it("preserves controlled lag and composes preventable trigger and close actions", () => {
    const changes = vi.fn();
    const preventedTrigger = vi.fn((event: React.MouseEvent) => event.preventDefault());
    const preventedClose = vi.fn((event: React.MouseEvent) => event.preventDefault());
    const { getByRole, rerender } = render(
      <Dialog open={false} onOpenChange={changes}>
        <DialogTrigger asChild>
          <button type="button" onClick={preventedTrigger}>
            Controlled trigger
          </button>
        </DialogTrigger>
        <DialogContent showCloseButton={false}>
          <DialogTitle>Controlled dialog</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    fireEvent.click(getByRole("button", { name: "Controlled trigger" }));
    expect(preventedTrigger).toHaveBeenCalledTimes(1);
    expect(changes).not.toHaveBeenCalled();
    expect(document.querySelector('[role="dialog"]')).toBeNull();

    rerender(
      <Dialog open onOpenChange={changes}>
        <DialogTrigger>Controlled trigger</DialogTrigger>
        <DialogContent showCloseButton={false}>
          <DialogTitle>Controlled dialog</DialogTitle>
          <DialogClose asChild>
            <button type="button" onClick={preventedClose}>
              Keep open
            </button>
          </DialogClose>
        </DialogContent>
      </Dialog>,
    );
    fireEvent.click(getByRole("button", { name: "Keep open" }));
    expect(preventedClose).toHaveBeenCalledTimes(1);
    expect(changes).not.toHaveBeenCalled();
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();
    fireEvent.pointerDown(document.querySelector('[data-slot="dialog-overlay"]')!);
    expect(changes).toHaveBeenCalledWith(false);
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();
  });
  it("wraps Tab focus in both directions and traps programmatic focus outside", () => {
    const outside = document.createElement("button");
    outside.textContent = "Outside";
    document.body.appendChild(outside);
    const focusOutside = vi.fn();
    const { getByRole } = render(
      <Dialog defaultOpen>
        <DialogContent showCloseButton={false} onFocusOutside={focusOutside}>
          <DialogTitle>Focus dialog</DialogTitle>
          <button type="button">First</button>
          <button type="button">Last</button>
        </DialogContent>
      </Dialog>,
    );
    const first = getByRole("button", { name: "First" });
    const last = getByRole("button", { name: "Last" });
    expect(document.activeElement).toBe(first);
    last.focus();
    fireEvent.keyDown(document, { key: "Tab" });
    expect(document.activeElement).toBe(first);
    fireEvent.keyDown(document, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(last);
    outside.focus();
    expect(focusOutside).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(first);
    outside.remove();
  });
  it("orders preventable outside and Escape hooks before close proposals", () => {
    const calls: string[] = [];
    const changes = vi.fn((open: boolean) => calls.push(`change:${open}`));
    const { rerender } = render(
      <Dialog defaultOpen onOpenChange={changes}>
        <DialogContent
          onPointerDownOutside={(event) => {
            calls.push("pointer");
            event.preventDefault();
          }}
          onInteractOutside={() => calls.push("interact")}
          onEscapeKeyDown={(event) => {
            calls.push("escape");
            event.preventDefault();
          }}
        >
          <DialogTitle>Prevented dialog</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    fireEvent.pointerDown(document.body);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(calls).toEqual(["pointer", "interact", "escape"]);
    expect(changes).not.toHaveBeenCalled();
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();

    calls.length = 0;
    rerender(
      <Dialog defaultOpen onOpenChange={changes}>
        <DialogContent onPointerDownOutside={() => calls.push("pointer")} onInteractOutside={() => calls.push("interact")}>
          <DialogTitle>Dismissible dialog</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    fireEvent.pointerDown(document.body);
    expect(calls).toEqual(["pointer", "interact", "change:false"]);
  });
}
