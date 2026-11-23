import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { uiDataLabel } from "../🎗️UiLabel/🟦️.tsx";
import { Engagement } from "../🎮️Engagement/🟦️.tsx";
import { Tree } from "../🌳️Tree/🟦️.tsx";
import { ControlTree } from "../🌳️Tree/🕹️control-tree/🟦️.tsx";
import { dockedAnchorRect } from "../🗨️Popover/🟦️.tsx";

const read = (relative: string) => readFileSync(resolve(import.meta.dirname, relative), "utf8");
const css = read("../../🎨️styling/🖌️ui/🎨️.css");

describe("round 3 framework shared-border findings", () => {
  it("derives native border sides from data-shared-border-edges", () => {
    for (const side of ["top", "right", "bottom", "left"]) expect(css).toContain(`:where([data-shared-border-frame][data-shared-border-edges]:not([data-shared-border-edges~="${side}"])) { border-${side}-width: 0; }`);
    expect(css).toMatch(/\[data-shared-border-frame\] > :is\(\[data-slot="input"\], \[data-slot="textarea"\]\) \{\s*border-width: 0;/);
  });
  it("drops call-site border hacks", () => {
    expect(read("../🪟️Window/🟦️.tsx")).not.toContain("border-t-0");
    expect(read("../🎮️Engagement/🟦️.tsx")).not.toContain("border-t-0");
    for (const file of ["../🎮️Engagement/🟦️.tsx", "../🔍️WindowSearch/🟦️.tsx"]) expect(read(file)).not.toMatch(/ring-1/);
  });
  it("renders the engagement column as one vertical shared frame with an edge-restricted options-only variant", () => {
    const options = [{ id: "a", icon: "check", label: "A" } as never];
    const full = render(<Engagement options={options} status={[{ id: "s", content: "Ready" }]} sessionActive />);
    const column = full.container.querySelector<HTMLElement>('[data-slot="engagement"]')!;
    expect(column.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
    expect(column.getAttribute("data-orientation")).toBe("vertical");
    expect(column.getAttribute("data-shared-border-state")).toBe("active");
    expect(column.className).not.toMatch(/ring-|gap-/);
    full.unmount();
    const only = render(<Engagement options={options} />);
    expect(only.container.querySelector('[data-slot="engagement"]')!.getAttribute("data-shared-border-edges")).toBe("right bottom left");
  });
  it("renders file-tree cards as frames with cell rows", () => {
    const view = render(<Tree.Files title={uiDataLabel("Files")} nodes={[{ title: "a.ts", path: "a.ts" } as never]} />);
    const card = view.container.querySelector<HTMLElement>('[data-slot="tree-files"]')!;
    expect(card.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(card.querySelectorAll(":scope > [data-shared-border-cell]").length).toBe(2);
    expect(css).not.toContain('[data-slot="tree-files"] + [data-slot="tree-files"]');
  });
  it("replaces rings, dashes and negative margins", () => {
    expect(read("../📻️TableAvatar/🟦️.tsx")).not.toContain("ring-1");
    expect(read("../🖼️Panel/🟦️.tsx")).not.toContain("border-dashed");
    expect(read("../📊️Table/🟦️.tsx")).not.toContain("ring-2");
    expect(read("../🧭️PanelTabBar/🟦️.tsx")).not.toContain("ring-2");
    expect(read("../🥞️LayeredOverview/🟢️overview-button/🟦️.tsx")).toContain("right-0 top-0");
    expect(css).not.toMatch(/\.introduction-demo-callout-chip \{[^}]*(margin|border:)/);
  });
  it("docks popovers, selects and toggle dropdowns flush by default", () => {
    expect(read("../🗨️Popover/🟦️.tsx")).toContain("sideOffset ?? 0");
    expect(read("../🔽️Select/🟦️.tsx")).toContain("sideOffset ?? 0");
    expect(read("../🔀️Toggle/🟦️.tsx")).toContain("dropdownSideOffset = 0");
  });
  it("docks a popover to the outer rect of the frame that hugs its trigger", () => {
    const frame = document.createElement("div"), trigger = document.createElement("button");
    frame.setAttribute("data-shared-border-frame", ""); frame.append(trigger); document.body.append(frame);
    frame.getBoundingClientRect = () => new DOMRect(120, 120, 70, 22.4);
    trigger.getBoundingClientRect = () => new DOMRect(121, 121, 68, 20.4);
    const rect = dockedAnchorRect(trigger);
    expect([rect.left, rect.top, rect.right, rect.bottom]).toEqual([120, 120, 190, 142.4]);
    frame.remove();
  });
  it("keeps the engagement body free of padding and gaps over framed children", () => {
    expect(read("../../🔨️modules/🎨️chrome-class-names/🟦️.ts")).toMatch(/windowEngagementBodyClass = "[^"]*"/);
    expect(read("../../🔨️modules/🎨️chrome-class-names/🟦️.ts")).not.toMatch(/windowEngagementBodyClass = "[^"]*(px-single|gap-half)/);
  });
  it("keeps native scrollbars out of window and pane bodies so rows reach the frame edge", () => {
    const names = read("../../🔨️modules/🎨️chrome-class-names/🟦️.ts");
    for (const name of ["windowEngagementBodyClass", "utilityBarBodyClass", "windowMeasuresBodyClass"]) expect(names).toMatch(new RegExp(`${name} = "ui-scrollbar-hidden `));
    expect(read("../📌️WindowPane/🟦️.tsx")).toContain('bodyClassName = "ui-scrollbar-hidden overflow-y-auto p-single"');
    expect(css).toMatch(/\.ui-scrollbar-hidden \{\s*scrollbar-width: none;\s*&::-webkit-scrollbar \{\s*display: none;/);
  });
  it("sizes unanchored panel tab strips like the chip row so they share the chrome frame line", () => {
    expect(css).toMatch(/:root \{\s*--panel-header-height: var\(--chip-row-height\);/);
  });
  it("keeps element stories gapless: ribbon zones touch, panel bands carry the band slot, layered cards tile", () => {
    expect(read("../🎀️Ribbon/📖️stories/🧪️.story.tsx")).not.toMatch(/flex gap-single items-stretch/);
    const panel = read("../🖼️Panel/📖️stories/🧪️.story.tsx");
    expect(panel).toContain('data-slot="navbar"');
    expect(panel).toContain('data-slot="footer"');
    const layered = read("../🥞️LayeredOverview/📖️stories/🧪️.story.tsx");
    expect(layered).not.toMatch(/place-items-center|max-w-xs|grid-cols-3 grid-rows-3 place/);
  });
  it("composes the popover and layout stories from shared-border rows and framework inputs, never raw boxed inputs or stacked gaps", () => {
    const popover = read("../🗨️Popover/📖️stories/🧪️.story.tsx");
    expect(popover).toContain("SharedBorderRow");
    expect(popover).not.toMatch(/space-y-|gap-double/);
    expect(read("../📐️Layout/📖️stories/🧪️.story.tsx")).not.toMatch(/<input[^>]*border/);
  });
  it("offsets the window quick-action rail and the context menu chrome by the chip row so no seam sits 1.6px inside another", () => {
    expect(read("../🪟️Window/🟦️.tsx")).toContain('block: "var(--chip-row-height)"');
    expect(read("../🖱️ContextMenu/🟦️.tsx")).not.toContain("var(--size-medium)");
  });
  it("makes every control tree row a bottom-edge frame so the filter and the first control share one line", () => {
    const row = render(<ControlTree controls={[{ path: "A/x", controlKind: "number", value: 1, onChange: () => {} }]} />).container.querySelector('[data-slot="control-tree-row"]')!;
    expect(row.getAttribute("data-shared-border-frame")).toBe("");
    expect(row.getAttribute("data-shared-border-edges")).toBe("bottom");
  });
  it("keeps layout stories filled, reserves window control width for the measures pane, and parks the Layout panel folded", () => {
    for (const story of ["../🪟️Window/📖️stories/🧪️.story.tsx", "../🎨️Canvas/📖️stories/🧪️.story.tsx", "../📐️Layout/📖️stories/🧪️.story.tsx"]) expect(read(story)).not.toMatch(/<Window id="[^"]+" defaultSize/);
    expect(read("../🪟️Window/🟦️.tsx")).toContain("inlineEdgeReservePx={controlsWidthPx}");
    expect(read("../📐️Layout/📖️stories/🧪️.story.tsx")).toContain("useState(false);\n    const [topLeftPath");
  });
  it("paints every cell divider in the owner's overlay: the stylesheet carries no cell border, flow variable or divider colour", () => {
    expect(css).not.toContain("@property --shared-border-flow");
    expect(css).not.toContain("border-block-end: calc(var(--stroke-hairline)");
    expect(css).not.toMatch(/border-inline-end: var\(--stroke-hairline\) solid var\(--border-normal-color\);\n\}\n:is\(\.shared-border-cell/);
  });
  it("paints the staged action dialog footer through the row's top edge, not a raw border", () => {
    expect(read("../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/🟦️.tsx")).toContain('<SharedBorderRow edges={["top"]} className="w-full">');
  });
});
