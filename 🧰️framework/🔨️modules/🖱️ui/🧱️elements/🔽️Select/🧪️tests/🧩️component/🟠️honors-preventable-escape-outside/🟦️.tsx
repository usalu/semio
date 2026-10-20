/** 🟠️ The honors preventable escape outside tests of the Select suite. */
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../🟦️.tsx";
import { Dialog, DialogContent, DialogPortal, DialogTitle } from "../../../../💬️Dialog/🟦️.tsx";
import { BasicSelect } from "../🟣️shared/🟦️.tsx";

/** 🟡 Registers the honors preventable escape outside tests of the Select suite. */
export function registerHonorsPreventableEscapeOutsideTests(): void {
  it("honors preventable Escape and outside dismissal, then preserves outside focus", async () => {
    const escaped = vi.fn((event: { preventDefault(): void }) => event.preventDefault());
    render(
      <Select id="select-component-test-4" defaultOpen onOpenChange={vi.fn()}>
        <SelectTrigger aria-label="Dismiss">
          <SelectValue />
        </SelectTrigger>
        <SelectContent onEscapeKeyDown={escaped}>
          <SelectItem value="one">One</SelectItem>
        </SelectContent>
      </Select>,
    );
    await screen.findByRole("listbox");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(escaped).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("listbox")).toBeTruthy();
    const outside = document.createElement("button");
    document.body.appendChild(outside);
    fireEvent.pointerDown(outside);
    outside.focus();
    await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
    expect(document.activeElement).toBe(outside);
    outside.remove();
  });
  it("dismisses only the deepest or most recently active open surface", async () => {
    const first = vi.fn();
    const second = vi.fn();
    render(
      <>
        <Select id="select-component-test-5" open onOpenChange={first}>
          <SelectTrigger aria-label="First">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="one">One</SelectItem>
          </SelectContent>
        </Select>
        <Select id="select-component-test-6" open onOpenChange={second}>
          <SelectTrigger aria-label="Second">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="two">Two</SelectItem>
          </SelectContent>
        </Select>
      </>,
    );
    await waitFor(() => expect(screen.getAllByRole("listbox")).toHaveLength(2));
    first.mockClear();
    second.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(second).toHaveBeenCalledWith(false);
    expect(first).not.toHaveBeenCalled();
    expect(screen.getAllByRole("listbox")).toHaveLength(2);
  });
  it("does not let a page Select consume events owned by a scoped application dialog", async () => {
    const pageChange = vi.fn(), scopedChange = vi.fn();
    const page = render(<BasicSelect open onOpenChange={pageChange} />);
    const root = document.createElement("section"), app = document.createElement("div"), layer = document.createElement("div");
    root.style.position = "relative";
    layer.style.position = "absolute";
    root.append(app, layer);
    document.body.appendChild(root);
    const scoped = render(
      <Dialog defaultOpen isolationRoot={root} onOpenChange={scopedChange}>
        <DialogPortal container={layer}>
          <DialogContent showCloseButton={false}>
            <DialogTitle>Scoped application</DialogTitle>
            <button type="button">Scoped action</button>
          </DialogContent>
        </DialogPortal>
      </Dialog>,
      { container: app, baseElement: root },
    );
    try {
      await page.findByRole("listbox");
      const action = scoped.getByRole("button", { name: "Scoped action" });
      fireEvent.pointerDown(action);
      expect(pageChange).not.toHaveBeenCalled();
      fireEvent.keyDown(action, { key: "Escape" });
      expect(pageChange).not.toHaveBeenCalled();
      expect(scopedChange).toHaveBeenCalledWith(false);
    } finally {
      scoped.unmount();
      page.unmount();
      root.remove();
    }
  });
  it("routes Escape to a logically nested portal before its parent", async () => {
    const parentChange = vi.fn();
    const childChange = vi.fn();
    const Nested = ({ childOpen }: { childOpen: boolean }) => (
      <Select id="select-component-test-7" open onOpenChange={parentChange} value="parent">
        <SelectTrigger aria-label="Parent">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="parent">Parent option</SelectItem>
        </SelectContent>
        <Select id="select-component-test-8" open={childOpen} onOpenChange={childChange} value="child">
          <SelectTrigger aria-label="Child">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="child">Child option</SelectItem>
          </SelectContent>
        </Select>
      </Select>
    );
    const view = render(<Nested childOpen />);
    await waitFor(() => expect(screen.getAllByRole("listbox")).toHaveLength(2));
    parentChange.mockClear();
    childChange.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(childChange).toHaveBeenCalledWith(false);
    expect(parentChange).not.toHaveBeenCalled();
    view.rerender(<Nested childOpen={false} />);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(parentChange).toHaveBeenCalledWith(false);
  });
  it("does not hover-activate on touch and still commits the touch click once", async () => {
    const values = vi.fn();
    render(<BasicSelect defaultOpen value="alpha" onValueChange={values} />);
    const listbox = await screen.findByRole("listbox");
    const alpha = screen.getByRole("option", { name: "Alpha" });
    const gamma = screen.getByRole("option", { name: "Gamma" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(alpha.id);
    fireEvent.pointerMove(gamma, { pointerType: "touch" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(alpha.id);
    fireEvent.pointerDown(gamma, { pointerType: "touch" });
    fireEvent.click(gamma);
    expect(values).toHaveBeenCalledTimes(1);
    expect(values).toHaveBeenCalledWith("gamma");
  });
}
