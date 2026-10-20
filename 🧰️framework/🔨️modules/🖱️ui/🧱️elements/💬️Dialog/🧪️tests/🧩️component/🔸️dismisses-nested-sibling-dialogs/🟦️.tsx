/** 🔸️ The dismisses nested sibling dialogs tests of the Dialog suite. */
import { fireEvent, render, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Dialog, DialogContent, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger } from "../../../🟦️.tsx";

/** 🔹 Registers the dismisses nested sibling dialogs tests of the Dialog suite. */
export function registerDismissesNestedSiblingDialogsTests(): void {
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
    expect(document.body.style.paddingRight).toBe(`${Math.max(0, window.innerWidth - document.documentElement.clientWidth)}px`);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(innerChange).toHaveBeenCalledWith(false);
    expect(outerChange).not.toHaveBeenCalled();
    expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(1);
    expect(document.body.style.overflow).toBe("hidden");
    expect(document.body.style.paddingRight).toBe(`${Math.max(0, window.innerWidth - document.documentElement.clientWidth)}px`);
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
}
