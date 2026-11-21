// #region 🔌️Adapters
import * as React from "react";
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { Command, CommandItem, CommandList } from "../⌨️Command/🟦️.tsx";
import { MenuItem } from "../📋️MenuItem/🟦️.tsx";
import { ContextMenuTriggerButton } from "../🖱️ContextMenu/🔘️trigger/🟦️.tsx";
import { Tabs, TabsList, TabsTrigger } from "../📑️Tabs/🟦️.tsx";
import { Table } from "../📊️Table/🟦️.tsx";
import { Slider } from "../🎚️Slider/🟦️.tsx";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../↔️Resizable/🟦️.tsx";
import { formControlResetClass } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";
import { interactiveActiveFillClass, interactiveOnClass, interactiveTabActiveClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { menuListItemClassName } from "../../🔨️modules/📋️menu-item-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔲️StateBoxes
const statePresets = [interactiveActiveFillClass, interactiveOnClass, interactiveTabActiveClass, menuListItemClassName, formControlResetClass];
const stateLine = /(?:^|\s)(?:[a-z\-\[\]=:]+:)*(?:border-(?:active-base|accent|destructive)|ring-(?:accent|destructive|ring)|outline-accent)(?:\s|$)/;

describe("🔲️ State lines belong to the border compositor", () => {
  afterEach(cleanup);

  it("keeps shared presets free of state border, ring and outline colours", () => {
    for (const preset of statePresets) expect(preset).not.toMatch(stateLine);
    expect(interactiveActiveFillClass).toContain("bg-active-base");
    expect(menuListItemClassName).toContain("data-[selected=true]:bg-active-base");
  });

  it("turns borderless controls into edge-less boxes", () => {
    const { container } = render(
      <div>
        <MenuItem>Run</MenuItem>
        <ContextMenuTriggerButton resolveTarget={() => null} />
        <Command>
          <CommandList>
            <CommandItem value="a">A</CommandItem>
          </CommandList>
        </Command>
        <Slider min={0} max={10} defaultValue={[3]} aria-label="Level" />
      </div>,
    );
    for (const selector of ['[data-slot="menu-item"]', "[data-context-menu-trigger]", '[data-slot="command-item"]', '[data-slot="slider-value"]']) {
      const box = container.querySelector(selector)!;
      expect(box.getAttribute("data-shared-border-frame"), selector).toBe("");
      expect(box.getAttribute("data-shared-border-edges"), selector).toBe("");
      expect(box.className, selector).not.toMatch(/focus-visible:(?:outline|ring)-\d|focus-visible:outline(?:\s|$)/);
    }
  });

  it("keeps every tab a frame whose edges close only while selected", () => {
    const { getAllByRole } = render(
      <Tabs defaultValue="b">
        <TabsList>
          <TabsTrigger value="a">A</TabsTrigger>
          <TabsTrigger value="b">B</TabsTrigger>
        </TabsList>
      </Tabs>,
    );
    const [unselected, selected] = getAllByRole("tab");
    expect(unselected!.getAttribute("data-shared-border-frame")).toBe("");
    expect(unselected!.getAttribute("data-shared-border-edges")).toBe("");
    expect(unselected!.getAttribute("data-shared-border-state")).toBeNull();
    expect(selected!.getAttribute("aria-selected")).toBe("true");
    expect(selected!.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
    expect(selected!.getAttribute("data-shared-border-state")).toBe("active");
    expect(selected!.className).not.toContain("focus-visible:border-active-base");
  });

  it("makes the resize handle a borderless box without its own focus ring", () => {
    const { container } = render(
      <ResizablePanelGroup orientation="horizontal">
        <ResizablePanel id="a" />
        <ResizableHandle />
        <ResizablePanel id="b" />
      </ResizablePanelGroup>,
    );
    const handle = container.querySelector('[data-slot="resizable-handle"]')!;
    expect(handle.getAttribute("data-shared-border-edges")).toBe("");
    expect(handle.className).not.toContain("focus-visible:ring");
  });

  it("turns clickable table rows into interactive boxes marked by selection", () => {
    const columns = [{ id: "name", header: "Name", accessor: (row: { name: string }) => row.name }] as never;
    const { container } = render(<Table columns={columns} data={[{ id: "a", name: "Alpha" }, { id: "b", name: "Beta" }]} getRowId={(row: { id: string }) => row.id} selectedRows={["b"]} onRowClick={() => {}} />);
    const [first, second] = container.querySelectorAll("tbody tr");
    for (const row of [first!, second!]) {
      expect(row.getAttribute("data-shared-border-edges")).toBe("bottom");
      expect(row.hasAttribute("data-shared-border-interactive")).toBe(true);
    }
    expect(first!.getAttribute("data-active")).toBeNull();
    expect(second!.getAttribute("data-active")).toBe("true");
    expect(second!.className).not.toContain("border-active-base");
  });

  it("recolours the slider thumb hairline on keyboard focus", () => {
    const { container } = render(<Slider min={0} max={10} defaultValue={[3]} aria-label="Level" />);
    const thumb = container.querySelector('[role="slider"]')!;
    expect(thumb.className.split(" ")).toEqual(expect.arrayContaining(["border", "border-transparent", "focus-visible:border-emphasized"]));
    expect(thumb.className).not.toContain("ring-");
    expect(thumb.hasAttribute("data-shared-border-frame")).toBe(false);
  });
});
// #endregion 🔲️StateBoxes
