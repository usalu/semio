/** 🧷️ Grouped buttons and segmented choices use the shared border mechanism: one frame, one cell divider per seam.
 * @see ./🥒️.feature — the scenarios
 */
import { cleanup, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { Button, ButtonGroup, Notice, Segmented } from "../../🔨️modules/🪟️chrome/🟦️.tsx";

const source = (path: string): string => readFileSync(resolve(import.meta.dirname, "../../🔨️modules", path), "utf8");

const tokens = (element: Element): string[] => element.className.toString().split(/\s+/u).filter((token) => token !== "");

describe("🧷️ shared borders", () => {
  afterEach(cleanup);

  it("draws a button group as one frame with cell buttons", () => {
    const { container } = render(
      <ButtonGroup label="Actions">
        <Button grouped>A</Button>
        <Button grouped>B</Button>
        <Button grouped>C</Button>
      </ButtonGroup>,
    );
    const group = container.querySelector('[role="group"]')!;
    expect(group.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(group.hasAttribute("data-shared-border-cells")).toBe(true);
    const cells = [...group.querySelectorAll("[data-shared-border-cell]")];
    expect(cells).toHaveLength(3);
    for (const cell of cells) {
      expect(cell.parentElement).toBe(group);
      expect(tokens(cell)).toContain("border-0");
    }
  });

  it("keeps an ungrouped button as its own framed chip", () => {
    const { container } = render(<Button>Alone</Button>);
    const button = container.querySelector("button")!;
    expect(button.hasAttribute("data-shared-border-cell")).toBe(false);
    expect(tokens(button)).not.toContain("border-0");
  });

  it("draws a segmented choice as a shared border group", () => {
    const { container } = render(<Segmented label="Pick" value={1} onChange={() => undefined} options={[{ value: 1, label: "One" }, { value: 2, label: "Two" }, { value: 3, label: "Three" }]} />);
    const group = container.querySelector('[role="radiogroup"]')!;
    expect(group.hasAttribute("data-shared-border-frame")).toBe(true);
    const radios = [...group.querySelectorAll('[role="radio"]')];
    expect(radios).toHaveLength(3);
    for (const radio of radios) {
      expect(radio.hasAttribute("data-shared-border-cell")).toBe(true);
      expect(tokens(radio)).toContain("border-0");
      expect(tokens(radio)).not.toContain("border-l");
    }
  });

  it("draws plain notice actions as cells of one row, not gap-separated buttons", () => {
    const { container } = render(
      <Notice kind="error" plain title="Failed" actions={<><Button>Retry</Button><Button>Close</Button></>}>
        Words
      </Notice>,
    );
    const notice = container.querySelector("[data-notice]")!;
    expect(notice.hasAttribute("data-shared-border-group")).toBe(true);
    const cells = [...notice.querySelectorAll("button[data-shared-border-cell]")];
    expect(cells).toHaveLength(2);
    expect(cells[0]!.parentElement).toBe(cells[1]!.parentElement);
    expect(notice.querySelector(".gap-single.flex-wrap")).toBeNull();
    expect(notice.className).not.toMatch(/\bborder\b/u);
  });

  it("keeps the 3D stage toolbar one row of cells without gap, group frames or first-child rules", () => {
    const style = source("🧊️spatial/🖥️stage/🟦️.ts");
    expect(style).not.toMatch(/\.semio-stage-bar\{[^}]*gap:/u);
    expect(style).not.toMatch(/\.semio-stage-group\{[^}]*border:/u);
    expect(style).not.toContain("first-child{border-inline-start:0}");
    expect(style).not.toMatch(/border-width:2px/u);
  });

  it("states unavailable figures without a dashed box and leaves collection edges to the shared owner", () => {
    for (const path of ["🔭️viewpoints/🪟️sheet/🟦️.tsx", "🧩️element/🟦️.ts", "🧍️figure/🟦️.tsx", "🧊️spatial/🚪️gate/🟦️.tsx"]) expect(source(path), path).not.toMatch(/dashed/u);
    expect(source("⭐️collections/🟦️.tsx")).not.toContain("borderBlockEndWidth");
    expect(source("🔬️detail/🧱️tables/🟦️.tsx")).not.toContain("last:border-b-0");
  });

  it("lays the playground timeline bars flush on their row, never inset from the row border", () => {
    const source = readFileSync(resolve(import.meta.dirname, "../../🔨️modules/🧪️playground/🎞️timeline/🖼️panel/🟦️.tsx"), "utf8");
    expect(source).not.toMatch(/absolute[^"]*\btop-1\b/u);
    expect(source).toMatch(/border-l border-current bg-\[var\(--accent\)\]\/30/u);
    expect(source).toMatch(/className="absolute inset-y-0 w-2/u);
    expect(source).toMatch(/aria-hidden="true" data-border-probe-art=""/u);
  });

  it("shows the navbar search only where it has room, never squeezed to zero width beside the language choice", () => {
    const app = readFileSync(resolve(import.meta.dirname, "../../📦️packages/🟦️typescript/📚️app/🟦️.tsx"), "utf8");
    expect(app).toMatch(/key: "search",\s*className: "hidden min-w-0 shrink lg:flex"/u);
  });
});
