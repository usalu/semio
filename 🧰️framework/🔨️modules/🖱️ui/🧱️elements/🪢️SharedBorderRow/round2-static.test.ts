import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { modeDockTabInsertPreviewClass } from "../🎨️Canvas/🧲️drag-preview/🟦️.tsx";

const read = (relative: string) => readFileSync(resolve(import.meta.dirname, relative), "utf8");

describe("round 2 static border findings", () => {
  const css = read("../../🎨️styling/🖌️ui/🎨️.css");
  it("keeps the intro-demo callout chips on one shared hairline", () => {
    expect(css).toMatch(/\.introduction-demo-callout-chips \{[^}]*gap: 0;/);
    expect(css).not.toMatch(/\.introduction-demo-callout-chip \{[^}]*margin-block-start/);
  });
  it("keeps a button-group wrapper flush with the border row that owns its seams", () => {
    expect(css).toMatch(/\[data-shared-border-cells\] > \[data-slot="button-group"\] \{\s*padding: 0;/u);
  });
  it("stacks file-tree cards on one seam", () => {
    expect(css).not.toMatch(/\[data-slot="tree-files"\] \+ \[data-slot="tree-files"\]/);
  });
  it("draws the dock insert preview as an unbordered, unmargined cell", () => {
    expect(modeDockTabInsertPreviewClass).not.toMatch(/(^|\s)(m[xy]?|border|rounded)-/);
  });
  it("paints menu colour swatches without a border box and avatar stacks without a second arc", () => {
    expect(read("../🖱️ContextMenu/🎨️item-render/🟦️.tsx")).not.toMatch(/size-small shrink-0 rounded-sm border/);
    expect(read("../🕰️HistoryTable/🟦️.tsx")).toMatch(/className="border-0 ring-2 ring-background"/);
  });
});
