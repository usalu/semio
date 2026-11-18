/** 🔹️ The shared-border cells tests of the Popover suite. */
import { render } from "@testing-library/react";
import { expect, it } from "vitest";
import { Popover, PopoverContent, PopoverTrigger } from "../../../🟦️.tsx";
import { Button } from "../../../../🔘️Button/🟦️.tsx";
import { Input } from "../../../../✏️Input/🟦️.tsx";

/** 🔷 Registers the shared-border cells tests of the Popover suite. */
export function registerSharedBorderCellsTests(): void {
  it("is an unpadded cells owner so framed children share its edges instead of sitting inset", () => {
    const { getByRole } = render(
      <Popover defaultOpen>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent>
          <Input id="name" aria-label="Name" />
          <Button icon="check" text="Apply" />
        </PopoverContent>
      </Popover>,
    );
    const content = getByRole("dialog");
    expect(content.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(content.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(content.dataset.orientation).toBe("vertical");
    expect(content.className.split(" ")).toContain("flex-col");
    expect(content.className).toMatch(/(^|\s)p-0(\s|$)/);
    expect(content.className).not.toMatch(/(^|\s)p-1(\s|$)/);
    expect(content.querySelectorAll(":scope > [data-shared-border-frame]")).toHaveLength(2);
  });
}
