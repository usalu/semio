/** 🔹️ The shared-border padding tests of the Dialog suite. */
import { render } from "@testing-library/react";
import { expect, it } from "vitest";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "../../../🟦️.tsx";
import { Button } from "../../../../🔘️Button/🟦️.tsx";

/** 🔷 Registers the shared-border padding tests of the Dialog suite. */
export function registerSharedBorderPaddingTests(): void {
  it("insets only text, never framed children: the frame has no padding or gap and the footer is an attached row of cells", () => {
    const { getByRole } = render(
      <Dialog open>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Title</DialogTitle>
            <DialogDescription>Text</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button icon="x" text="Cancel" />
            <Button icon="check" text="Save" />
          </DialogFooter>
        </DialogContent>
      </Dialog>,
    );
    const content = getByRole("dialog");
    expect(content.className).toMatch(/(^|\s)p-0(\s|$)/);
    expect(content.className).toMatch(/(^|\s)gap-0(\s|$)/);
    expect(content.className).not.toMatch(/(^|\s)(p|gap)-medium(\s|$)/);
    const header = content.querySelector<HTMLElement>('[data-slot="dialog-header"]')!;
    expect(header.className).toMatch(/(^|\s)p-medium(\s|$)/);
    expect(header.hasAttribute("data-shared-border-frame")).toBe(false);
    const footer = content.querySelector<HTMLElement>('[data-slot="dialog-footer"]')!;
    expect(footer.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(footer.dataset.sharedBorderEdges).toBe("top");
    expect(footer.className).not.toMatch(/\b(p|px|py|gap|m|mt|mb)-/);
    expect(footer.querySelectorAll(":scope > [data-shared-border-frame]")).toHaveLength(2);
  });
}
