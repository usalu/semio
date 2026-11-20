import * as React from "react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, waitFor } from "@testing-library/react";
import { SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, SharedBorderRowLabel, useSharedBorderFrameRef } from "./🟦️.tsx";
import { Button } from "../🔘️Button/🟦️.tsx";
import { Input } from "../✏️Input/🟦️.tsx";
import { Textarea } from "../🔤️Textarea/🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

const WIDTH = 300;
const HEIGHT = 30;

function rectOf(element: HTMLElement): DOMRect {
  const slot = element.dataset.slot;
  if (slot === "shared-border-row") return new DOMRect(0, 0, WIDTH, HEIGHT);
  if (slot === "button-group" || slot === "input-root" || slot === "textarea-root") return new DOMRect(0, 0, WIDTH / 3, HEIGHT);
  return new DOMRect();
}

function mockRects() {
  vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockImplementation(function (this: HTMLElement) { return rectOf(this).width; });
  vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockImplementation(function (this: HTMLElement) { return rectOf(this).height; });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) { return rectOf(this); });
}

const overlays = (element: Element) => [...element.querySelectorAll("svg[data-shared-border-overlay]")];

describe("controls placed directly in a shared-border row are cells of the row", () => {
  it("marks the row as owner and frame, and a group-only row as owner without frame", () => {
    const { container, rerender } = render(<div className="semio-scope"><SharedBorderRow orientation="vertical" edges={["top", "bottom"]} /></div>);
    const row = container.querySelector<HTMLElement>('[data-slot="shared-border-row"]')!;
    expect(row.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(row.getAttribute("data-shared-border-edges")).toBe("top bottom");
    expect(row.getAttribute("data-orientation")).toBe("vertical");
    expect(row.hasAttribute("data-shared-border-frame")).toBe(true);
    rerender(<div className="semio-scope"><SharedBorderRow framed={false} /></div>);
    const group = container.querySelector<HTMLElement>('[data-slot="shared-border-row"]')!;
    expect(group.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(group.hasAttribute("data-shared-border-group")).toBe(true);
    expect(group.hasAttribute("data-shared-border-edges")).toBe(false);
  });

  it("registers one overlay for the row, one for the nested row that owns cells of its own, and none for its Button, Input or Textarea cells", async () => {
    mockRects();
    const { container } = render(
      <div className="semio-scope">
        <SharedBorderRow>
          <Button text="Archive" icon="x" />
          <Input id="a" defaultValue="a" />
          <Textarea id="b" defaultValue="b" />
          <SharedBorderRow><Button text="Inner" icon="x" /></SharedBorderRow>
          <SharedBorderRowCell>Text</SharedBorderRowCell>
        </SharedBorderRow>
      </div>,
    );
    const row = container.querySelector<HTMLElement>('[data-slot="shared-border-row"]')!;
    await waitFor(() => expect(row.querySelectorAll(":scope > svg[data-shared-border-overlay] [data-shared-border-segment]").length).toBeGreaterThan(0));
    expect(overlays(container)).toHaveLength(2);
    expect(row.querySelector(":scope > svg[data-shared-border-overlay]")).not.toBeNull();
    for (const cell of row.querySelectorAll<HTMLElement>(":scope > [data-shared-border-frame]")) {
      expect(cell.hasAttribute("data-shared-border-attached")).toBe(false);
      expect(cell.querySelector(":scope > svg[data-shared-border-overlay]") === null).toBe(!cell.hasAttribute("data-shared-border-cells"));
    }
    expect(row.querySelectorAll(":scope > [data-shared-border-frame]").length).toBe(4);
    const paths = [...row.querySelectorAll(":scope > svg [data-shared-border-segment]")].map((path) => path.getAttribute("d"));
    for (const edge of [`M 1 1 L 1 ${HEIGHT - 1}`, `M ${WIDTH - 1} 1 L ${WIDTH - 1} ${HEIGHT - 1}`]) expect(paths).toContain(edge);
    expect(paths.every(d => /^M [\d.]+ [\d.]+ L [\d.]+ [\d.]+$/.test(d ?? ""))).toBe(true);
  });

  it("registers a framed control again as soon as it is no longer a direct child of the owner", async () => {
    mockRects();
    const { container, rerender } = render(<div className="semio-scope"><SharedBorderRow><Button text="Go" icon="x" /></SharedBorderRow></div>);
    await waitFor(() => expect(overlays(container).length).toBeGreaterThan(0));
    expect(overlays(container)).toHaveLength(1);
    rerender(<div className="semio-scope"><SharedBorderRow><div><Button text="Go" icon="x" /></div></SharedBorderRow></div>);
    await waitFor(() => expect(overlays(container)).toHaveLength(2));
    expect(container.querySelector('[data-slot="button-group"]')!.hasAttribute("data-shared-border-attached")).toBe(true);
  });

  it("styles frame cells in the stylesheet: no frame, no inner border, one divider, stretched across the owner", () => {
    const css = readFileSync(resolve(import.meta.dirname, "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
    const own = css.match(/\[data-shared-border-cells\] > \[data-shared-border-frame\] \{([^}]*)\}/)![1]!;
    for (const declaration of ["border-width: 0;", "border-radius: 0;", "align-self: stretch;", "height: auto;", "max-height: none;"]) expect(own).toContain(declaration);
    expect(css).toMatch(/\[data-shared-border-cells\] > \[data-shared-border-frame\] > :is\(\[data-slot="input"\], \[data-slot="textarea"\]\) \{\s*border-width: 0;/);
    expect(css).not.toContain("border-inline-end: var(--stroke-hairline) solid var(--border-normal-color);\n}\n:is(.shared-border-cell");
    const priority = css.slice(css.indexOf("@layer shared-border-priority {"), css.indexOf("@layer shared-border-ownership {"));
    expect(priority).not.toContain(":has(+");
    expect(priority).not.toContain("border-color: var(--active-base) !important");
  });
});

const stylesheet = () => readFileSync(resolve(import.meta.dirname, "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");

describe("row primitives for lists, labels, native fields and tones", () => {
  it("renders as ul/li and forwards refs to the element", async () => {
    mockRects();
    const rowRef = React.createRef<HTMLElement>();
    const cellRef = React.createRef<HTMLElement>();
    const { container } = render(<div className="semio-scope"><SharedBorderRow as="ul" ref={rowRef} orientation="vertical"><SharedBorderRowCell as="li" ref={cellRef}>a</SharedBorderRowCell></SharedBorderRow></div>);
    expect(rowRef.current?.tagName).toBe("UL");
    expect(cellRef.current?.tagName).toBe("LI");
    expect(container.querySelector("ul > li[data-shared-border-cell]")).not.toBeNull();
    await waitFor(() => expect(overlays(container)).toHaveLength(1));
    expect(typeof useSharedBorderFrameRef).toBe("function");
  });

  it("gives labels a fixed or auto column and keeps the label associated with its control", () => {
    const { container } = render(<div className="semio-scope"><SharedBorderRow><SharedBorderRowLabel htmlFor="a">Fixed</SharedBorderRowLabel><SharedBorderRowLabel htmlFor="b" width="auto">Auto</SharedBorderRowLabel></SharedBorderRow></div>);
    const [fixed, auto] = [...container.querySelectorAll<HTMLElement>('[data-slot="shared-border-row-label"]')];
    expect(fixed!.getAttribute("data-label-width")).toBe("fixed");
    expect(fixed!.className).toContain("shared-border-label-width");
    expect(auto!.className).not.toContain("shared-border-label-width");
    expect(fixed!.querySelector("label")!.getAttribute("for")).toBe("a");
  });

  it("makes native select, input and contentEditable cells of a row without registering frames", async () => {
    mockRects();
    const { container, getByRole } = render(
      <div className="semio-scope">
        <SharedBorderRow>
          <SharedBorderRowField label="Mode"><select defaultValue="b"><option value="a">A</option><option value="b">B</option></select></SharedBorderRowField>
          <SharedBorderRowField label="Colour"><input type="color" defaultValue="#ff0000" /></SharedBorderRowField>
          <select data-shared-border-cell="" aria-label="Direct"><option>x</option></select>
          <div data-shared-border-cell="" contentEditable suppressContentEditableWarning>edit</div>
        </SharedBorderRow>
      </div>,
    );
    expect(getByRole("combobox", { name: /^Mode/ })).toBeTruthy();
    await waitFor(() => expect(overlays(container)).toHaveLength(1));
    expect(container.querySelectorAll("[data-shared-border-field]")).toHaveLength(2);
    expect(container.querySelectorAll(":scope [data-shared-border-cell]").length).toBe(4);
    const css = stylesheet();
    expect(css).toMatch(/\[data-shared-border-cell\]:where\(select, textarea, input:not\([^)]*\), \[contenteditable\]:not\(\[contenteditable="false"\]\)\) \{\s*border-width: 0;\s*border-radius: 0;/);
    expect(css).toMatch(/\[data-shared-border-field\] > :where\(select, textarea[^{]*\) \{\s*border-width: 0;\s*border-radius: 0;/);
    expect(css.indexOf('[data-shared-border-cell]:where(select')).toBeLessThan(css.indexOf("Cells have no native border"));
  });

  it("marks destructive rows and cells and paints a destructive row's normal strokes destructive", () => {
    const { container } = render(<div className="semio-scope"><SharedBorderRow tone="destructive"><SharedBorderRowCell tone="destructive">x</SharedBorderRowCell></SharedBorderRow></div>);
    expect(container.querySelector('[data-slot="shared-border-row"]')!.getAttribute("data-tone")).toBe("destructive");
    expect(container.querySelector('[data-slot="shared-border-row-cell"]')!.getAttribute("data-tone")).toBe("destructive");
    const css = stylesheet();
    expect(css).toContain('[data-shared-border-state="normal"] { stroke: var(--destructive-border); }');
  });

  it("lets Input and Textarea roots take className and style", () => {
    const { container } = render(<div className="semio-scope"><Input id="rooted-input" defaultValue="a" rootClassName="absolute" rootStyle={{ left: 4 }} /><Textarea id="rooted-textarea" defaultValue="b" rootClassName="fixed" rootStyle={{ top: 2 }} /></div>);
    const input = container.querySelector<HTMLElement>('[data-slot="input-root"]')!;
    const textarea = container.querySelector<HTMLElement>('[data-slot="textarea-root"]')!;
    expect(input.className).toContain("absolute");
    expect(input.style.left).toBe("4px");
    expect(input.style.transition).toContain("opacity");
    expect(textarea.className).toContain("fixed");
    expect(textarea.style.top).toBe("2px");
  });
});
