// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { CommandDialog } from "../⌨️Command/🟦️component.tsx";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger } from "./🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 💬️DialogMatrix
describe("Dialog", () => {
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

  it("dismisses nested and sibling dialogs strictly topmost while retaining the outer scroll lock", async () => {
    const outerChange = vi.fn();
    const innerChange = vi.fn();
    const { rerender } = render(
      <Dialog defaultOpen onOpenChange={outerChange}>
        <DialogContent showCloseButton={false}>
          <DialogTitle>Outer</DialogTitle>
          <Dialog defaultOpen onOpenChange={innerChange}>
            <DialogTrigger>Inner trigger</DialogTrigger>
            <DialogContent showCloseButton={false}>
              <DialogTitle>Inner</DialogTitle>
              <button type="button">Inner action</button>
            </DialogContent>
          </Dialog>
        </DialogContent>
      </Dialog>,
    );
    expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(2);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(innerChange).toHaveBeenCalledWith(false);
    expect(outerChange).not.toHaveBeenCalled();
    expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(1);
    expect(document.body.style.overflow).toBe("hidden");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(outerChange).toHaveBeenCalledWith(false);
    await waitFor(() => expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(0));

    const firstChange = vi.fn();
    const secondChange = vi.fn();
    rerender(
      <>
        <Dialog key="first" defaultOpen onOpenChange={firstChange}>
          <DialogContent showCloseButton={false}>
            <DialogTitle>First sibling</DialogTitle>
          </DialogContent>
        </Dialog>
        <Dialog key="second" defaultOpen onOpenChange={secondChange}>
          <DialogContent showCloseButton={false}>
            <DialogTitle>Second sibling</DialogTitle>
          </DialogContent>
        </Dialog>
      </>,
    );
    fireEvent.keyDown(document, { key: "Escape" });
    expect(secondChange).toHaveBeenCalledWith(false);
    expect(firstChange).not.toHaveBeenCalled();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(firstChange).toHaveBeenCalledWith(false);
  });

  it("restores pre-existing isolation and scroll styles and honors prevented autofocus", () => {
    document.body.style.overflow = "clip";
    document.body.style.paddingRight = "7px";
    const opened = vi.fn();
    const closed = vi.fn();
    const { container, getByRole, rerender, unmount } = render(
      <Dialog open={false}>
        <DialogContent
          onOpenAutoFocus={(event) => {
            opened();
            event.preventDefault();
          }}
          onCloseAutoFocus={(event) => {
            closed();
            event.preventDefault();
          }}
        >
          <DialogTitle>Lifecycle</DialogTitle>
          <button type="button">Inside</button>
        </DialogContent>
      </Dialog>,
    );
    container.setAttribute("aria-hidden", "false");
    container.setAttribute("inert", "legacy");
    rerender(
      <Dialog open>
        <DialogContent
          onOpenAutoFocus={(event) => {
            opened();
            event.preventDefault();
          }}
          onCloseAutoFocus={(event) => {
            closed();
            event.preventDefault();
          }}
        >
          <DialogTitle>Lifecycle</DialogTitle>
          <button type="button">Inside</button>
        </DialogContent>
      </Dialog>,
    );
    expect(opened).toHaveBeenCalledTimes(1);
    expect(document.activeElement).not.toBe(getByRole("button", { name: "Inside" }));
    unmount();
    expect(closed).toHaveBeenCalledTimes(1);
    expect(container.getAttribute("aria-hidden")).toBe("false");
    expect(container.getAttribute("inert")).toBe("legacy");
    expect(document.body.style.overflow).toBe("clip");
    expect(document.body.style.paddingRight).toBe("7px");
    document.body.style.overflow = "";
    document.body.style.paddingRight = "";
  });

  it("composes an explicit Portal and Overlay in a custom container without nesting portals", () => {
    const host = document.createElement("section");
    const sibling = document.createElement("button");
    sibling.textContent = "Host sibling";
    host.appendChild(sibling);
    document.body.appendChild(host);
    const { unmount } = render(
      <Dialog defaultOpen>
        <DialogPortal container={host}>
          <DialogOverlay />
          <DialogContent showCloseButton={false}>
            <DialogTitle>Custom portal</DialogTitle>
            <button type="button">Portal action</button>
          </DialogContent>
        </DialogPortal>
      </Dialog>,
    );
    const portal = host.querySelector<HTMLElement>('[data-slot="dialog-portal"]');
    expect(portal).not.toBeNull();
    expect(portal?.parentElement).toBe(host);
    expect(portal?.querySelectorAll('[data-slot="dialog-portal"]')).toHaveLength(0);
    expect(portal?.querySelectorAll('[data-slot="dialog-overlay"]')).toHaveLength(1);
    expect(sibling.hasAttribute("inert")).toBe(true);
    expect(document.body.style.overflow).toBe("hidden");
    unmount();
    expect(sibling.hasAttribute("inert")).toBe(false);
    expect(document.body.style.overflow).toBe("");
    host.remove();
  });

  it("unmounts closed portal descendants and runs their cleanup exactly once", () => {
    const lifecycle: string[] = [];
    function Effect(): React.ReactElement {
      React.useEffect(() => {
        lifecycle.push("mount");
        return () => lifecycle.push("cleanup");
      }, []);
      return <span>Effect</span>;
    }
    const { getByRole } = render(
      <Dialog>
        <DialogTrigger>Open lifecycle</DialogTrigger>
        <DialogContent>
          <DialogTitle>Lifecycle dialog</DialogTitle>
          <Effect />
        </DialogContent>
      </Dialog>,
    );
    expect(lifecycle).toEqual([]);
    fireEvent.click(getByRole("button", { name: "Open lifecycle" }));
    expect(lifecycle).toEqual(["mount"]);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(lifecycle).toEqual(["mount", "cleanup"]);
  });

  it("keeps CommandDialog's accessible labels inside the active portal", () => {
    const { getByRole } = render(
      <CommandDialog open title={"Commands" as never} description="Choose a command" showCloseButton={false}>
        <div>Command body</div>
      </CommandDialog>,
    );
    const content = getByRole("dialog", { name: "Commands" });
    const title = getByRole("heading", { name: "Commands" });
    expect(content.contains(title)).toBe(true);
    expect(content.contains(document.querySelector('[data-slot="dialog-description"]'))).toBe(true);
  });
});
// #endregion 💬️DialogMatrix
