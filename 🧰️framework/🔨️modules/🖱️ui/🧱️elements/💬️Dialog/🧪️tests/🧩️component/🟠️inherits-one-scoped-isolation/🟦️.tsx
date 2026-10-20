/** 🟠️ The inherits one scoped isolation tests of the Dialog suite. */
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { CommandDialog } from "../../../../⌨️Command/🟦️.tsx";
import { Dialog, DialogContent, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger } from "../../../🟦️.tsx";

/** 🟡 Registers the inherits one scoped isolation tests of the Dialog suite. */
export function registerInheritsOneScopedIsolationTests(): void {
  it("inherits one scoped isolation root across nested dialogs and dismisses only its topmost owner", () => {
    const root = document.createElement("section"), app = document.createElement("div"), layer = document.createElement("div");
    root.style.position = "relative";
    layer.style.position = "absolute";
    root.append(app, layer);
    document.body.appendChild(root);
    const outerChange = vi.fn(), innerChange = vi.fn();
    const view = render(
      <Dialog defaultOpen isolationRoot={root} onOpenChange={outerChange}>
        <DialogPortal container={layer}>
          <DialogOverlay />
          <DialogContent showCloseButton={false}>
            <DialogTitle>Scoped outer</DialogTitle>
            <Dialog defaultOpen onOpenChange={innerChange}>
              <DialogContent showCloseButton={false}>
                <DialogTitle>Scoped inner</DialogTitle>
                <button type="button">Inner action</button>
              </DialogContent>
            </Dialog>
          </DialogContent>
        </DialogPortal>
      </Dialog>,
      { container: app, baseElement: root },
    );
    try {
      const inner = view.getByRole("button", { name: "Inner action" });
      expect(root.querySelectorAll('[data-dialog-isolation="scoped"]')).toHaveLength(2);
      fireEvent.keyDown(inner, { key: "Escape" });
      expect(innerChange).toHaveBeenCalledWith(false);
      expect(outerChange).not.toHaveBeenCalled();
      const outer = view.getByRole("dialog", { name: "Scoped outer" });
      fireEvent.keyDown(outer, { key: "Escape" });
      expect(outerChange).toHaveBeenCalledWith(false);
    } finally {
      view.unmount();
      root.remove();
    }
  });
  it("unmounts closed portal descendants and runs their cleanup exactly once", () => {
    const lifecycle: string[] = [];
    function Effect(): React.ReactElement {
      React.useEffect(() => {
        lifecycle.push("mount");
        return () => { lifecycle.push("cleanup"); };
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
}
