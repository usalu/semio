import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerTreeHelpers3Tests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { Button, CheckIcon, COLLAPSED_FIELD_ELLIPSIS, COMPACT_UI_DRIVER, ControlTree, fitCollapsedFieldText, FlowProvider, getElementById, Input, Label, NavbarExampleSelect, renderToStaticMarkup, resolveCollapsedFieldDisplayState, Ring, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Slider, Textarea, Toggle, TreeAlignedRow, TreeContent, TreeContext, TreeItem, TreeRowAlignmentContext, TreeSection, UiDriverProvider } = dependencies;
  const { describe, expect, it, vi } = vitest;

describe("tree helpers", () => {

  
      it("renders sliders with element gray range and thumb at rest", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.demo" value={[42]} min={0} max={100} />);
  
        expect(markup).toContain('data-slot="slider-range"');
        expect(markup).toContain("bg-element");
        expect(markup).toContain("group-hover:bg-emphasized");
        expect(markup).toContain("data-[dragging=true]:bg-active-base");
        expect(markup).toContain("has-[[data-slot=slider-thumb]:hover]:[&amp;_[data-slot=slider-range]]:bg-emphasized");
        expect(markup).toContain("h-full");
        expect(markup).not.toContain("bg-foreground");
        expect(markup).toContain('data-slot="slider-thumb"');
        expect(markup).toContain("rounded-full");
        expect(markup).toContain("hover:bg-emphasized");
        expect(markup).toContain("group-hover:bg-emphasized");
        expect(markup).toContain('data-slot="slider-value"');
        expect(markup).toContain("text-element");
        expect(markup).not.toContain('data-slot="slider-ready"');
        expect(markup).toContain('data-slot="slider-track-wrap"');
        expect(markup).not.toContain("border-loading");
      });
  
      it("can omit the numeric readout for track-only graph overlays", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.track-only" className="h-full w-full min-w-0" value={[42]} min={0} max={100} showValue={false} />);
  
        expect(markup).toContain('data-slot="slider-track"');
        expect(markup).toContain('data-slot="slider-thumb"');
        expect(markup).toContain("h-full w-full min-w-0");
        expect(markup).not.toContain('data-slot="slider-value"');
        expect(markup).not.toContain('data-slot="slider-row"');
      });
  
      it("renders a ready extent highlight to the right of the knob on a fixed range", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.ready" value={[20]} min={0} max={100} ready={55} />);
  
        expect(markup).toContain('data-slot="slider-ready"');
        expect(markup).toContain("bg-[var(--accent-secondary)]");
        expect(markup).not.toContain('data-slot="slider-ready" class="bg-emphasized');
        expect(markup).toMatch(/left:\s*20%/);
        expect(markup).toMatch(/width:\s*35%/);
        expect(markup).toContain('data-slot="slider-thumb"');
        expect(markup).toContain("group-hover:bg-emphasized");
      });
  
      it("keeps the full fixed range interactive beyond the ready extent", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const onValueChange = vi.fn();
        const { container } = render(<Slider id="ui.slider.ready-interactive" value={[55]} min={0} max={100} ready={55} step={1} onValueChange={onValueChange} />);
        const thumb = container.querySelector('[data-slot="slider-thumb"]');
        expect(thumb).not.toBeNull();
  
        fireEvent.keyDown(thumb!, { key: "ArrowRight" });
        expect(onValueChange).toHaveBeenLastCalledWith([56]);
      });
  
      it("clampToReady suppresses a no-op attempt at the ready extent", async () => {
        // 🪣️ Opt-in: most `ready` consumers (the test above) want the highlight without a hard limit;
        // a background-planned count (e.g. puzzle3d's fill slider) needs `clampToReady` to make the
        // un-planned tail physically unreachable.
        const { fireEvent, render } = await import("@testing-library/react");
        const onValueChange = vi.fn();
        const { container } = render(<Slider id="ui.slider.ready-clamped" value={[55]} min={0} max={100} ready={55} step={1} clampToReady onValueChange={onValueChange} />);
        const thumb = container.querySelector('[data-slot="slider-thumb"]');
        expect(thumb).not.toBeNull();
  
        fireEvent.keyDown(thumb!, { key: "ArrowRight" });
        expect(onValueChange).not.toHaveBeenCalled();
      });
  
      it("forwards fractional step to the owned root so 0–1 probability sliders are not stuck at 0/1", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const onValueChange = vi.fn();
        const { container } = render(<Slider id="ui.slider.probability-step" value={[0.5]} min={0} max={1} step={0.01} onValueChange={onValueChange} />);
        const thumb = container.querySelector('[data-slot="slider-thumb"]');
        expect(thumb).not.toBeNull();
  
        fireEvent.keyDown(thumb!, { key: "ArrowRight" });
        expect(onValueChange).toHaveBeenLastCalledWith([0.51]);
      });
  
      it("puts the loading ring on the track wrap only so the knob and hover chrome stay visible", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.loading" value={[10]} min={0} max={100} ready={40} loading />);
  
        expect(markup).toContain('data-slot="slider-track-wrap"');
        expect(markup).toContain('data-loading="true"');
        expect(markup).toContain("border-loading");
        expect(markup).toContain('data-slot="slider-thumb"');
        expect(markup).toContain("group-hover:bg-emphasized");
        expect(markup).toContain("hover:bg-emphasized");
        expect(markup).toContain('data-slot="slider-ready"');
        const trackWrapIdx = markup.indexOf('data-slot="slider-track-wrap"');
        const thumbIdx = markup.indexOf('data-slot="slider-thumb"');
        expect(trackWrapIdx).toBeGreaterThan(-1);
        expect(thumbIdx).toBeGreaterThan(trackWrapIdx);
        expect(markup.indexOf("border-loading")).toBeGreaterThan(trackWrapIdx);
        expect(markup.indexOf("border-loading")).toBeLessThan(thumbIdx);
      });
  
      it("puts the waiting ring on the track wrap only so the knob and hover chrome stay visible", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.waiting" value={[10]} min={0} max={100} ready={40} waiting />);
  
        expect(markup).toContain('data-slot="slider-track-wrap"');
        expect(markup).toContain('data-waiting="true"');
        expect(markup).toContain("border-waiting");
        expect(markup).toContain('data-slot="slider-thumb"');
        const trackWrapIdx = markup.indexOf('data-slot="slider-track-wrap"');
        const thumbIdx = markup.indexOf('data-slot="slider-thumb"');
        expect(markup.indexOf("border-waiting")).toBeGreaterThan(trackWrapIdx);
        expect(markup.indexOf("border-waiting")).toBeLessThan(thumbIdx);
      });
  
      it("prefers the loading ring over waiting on the slider track wrap when both are set", () => {
        const markup = renderToStaticMarkup(<Slider id="ui.slider.both" value={[10]} min={0} max={100} ready={40} loading waiting />);
  
        expect(markup).toContain("border-loading");
        expect(markup).not.toContain("border-waiting");
      });
  
      it("renders shared field roots that stretch within the property value column", () => {
        const inputMarkup = renderToStaticMarkup(<Input id="tooltip.manual" value="value" />);
        const textareaMarkup = renderToStaticMarkup(<Textarea id="tooltip.manual" value="value" />);
  
        expect(inputMarkup).toContain('data-slot="input-root"');
        expect(inputMarkup).toContain('data-detail-panel-control="fill"');
        expect(inputMarkup).toContain("flex min-w-0 w-full flex-1 items-stretch");
        expect(inputMarkup).toContain('autoComplete="off"');
        expect(inputMarkup).toContain('spellCheck="false"');
        expect(textareaMarkup).toContain('data-slot="textarea-root"');
        expect(textareaMarkup).toContain('data-detail-panel-control="fill"');
        expect(textareaMarkup).toContain("flex min-w-0 w-full flex-1 items-stretch");
        expect(textareaMarkup).toContain('autoComplete="off"');
        expect(textareaMarkup).toContain('spellCheck="false"');
      });
  
      it("anchors fit-content button and toggle controls to the shared property edge", () => {
        const buttonMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Button text="Apply" icon="check" />
          </Label>,
        );
        const toggleMarkup = renderToStaticMarkup(<Toggle id="tooltip.manual" icon={<CheckIcon />} showLabel />);
  
        expect(buttonMarkup).toContain('data-slot="property-control"');
        expect(buttonMarkup).toContain("justify-end");
        expect(buttonMarkup).toContain('data-slot="button-group"');
        expect(buttonMarkup).toContain('data-detail-panel-control="fit"');
        expect(buttonMarkup).toContain("w-fit shrink-0");
        expect(toggleMarkup).toContain('data-slot="property-control"');
        expect(toggleMarkup).toContain("justify-end");
        expect(toggleMarkup).toContain('data-slot="toggle-group"');
        expect(toggleMarkup).toContain('data-detail-panel-control="fit"');
        expect(toggleMarkup).toContain("w-fit shrink-0");
      });
  
      it("renders ring inside tree-aligned property row with label and fit control", () => {
        const ringMarkup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeRowAlignmentContext.Provider value={true}>
              <div data-slot="tree-row">
                <TreeAlignedRow level={1} isLastAtLevel={[true]} showLines={true} connectCurrentLevel={true} contentClassName="min-w-0">
                  <Ring id="ui.ring.demo" orbs={[{ id: "connector-1", t: 0.25, selected: true }]} showLabel />
                </TreeAlignedRow>
              </div>
            </TreeRowAlignmentContext.Provider>
          </TreeContext.Provider>,
        );
  
        expect(ringMarkup).toContain('data-slot="tree-row-layout"');
        expect(ringMarkup).toContain('data-slot="tree-gutter"');
        expect(ringMarkup).toContain('data-slot="property-row"');
        expect(ringMarkup).toContain('data-slot="property-label"');
        expect(ringMarkup).toContain('data-slot="property-control"');
        expect(ringMarkup).toContain('data-slot="ring"');
        expect(ringMarkup).toContain('data-detail-panel-control="fit"');
        expect(ringMarkup).toContain("w-fit shrink-0");
        expect(ringMarkup).toContain('id="ui.ring.demo-label"');
        expect(ringMarkup).toContain(">Ring<");
      });
  
      it("marks select triggers as fill-width detail controls", () => {
        const selectMarkup = renderToStaticMarkup(
          <Select id="tooltip.manual" showLabel defaultValue="alpha">
            <SelectTrigger>
              <SelectValue placeholder="Select" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="alpha">Alpha</SelectItem>
            </SelectContent>
          </Select>,
        );
  
        expect(selectMarkup).toContain('data-slot="select-trigger"');
        expect(selectMarkup).toContain('data-detail-panel-control="fill"');
      });
  
      it("keeps select chevrons and mounts playground example options only while open", async () => {
        const { render } = await import("@testing-library/react");
        const userEvent = (await import("@testing-library/user-event")).default;
        const user = userEvent.setup();
        const selectMarkup = renderToStaticMarkup(
          <Select id="tooltip.manual" defaultValue="alpha">
            <SelectTrigger className="w-32">
              <SelectValue placeholder="Select" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="alpha">A very long option label that must not clip the chevron</SelectItem>
            </SelectContent>
          </Select>,
        );
        render(
          <NavbarExampleSelect
            id="playground.navbar.fixture"
            value="nakagin"
            options={[
              { id: "nakagin", label: "Nakagin Capsule Tower with an intentionally long label", icon: "building" },
              { id: "villa", label: "Villa Savoye", icon: "landmark" },
            ]}
            onValueChange={() => undefined}
            includeNoExample={false}
          />,
        );
        expect(document.querySelector('[data-slot="select-content"]')).toBeNull();
        // 🆔️ The component's own id names the TRIGGER: it is the control a press addresses and the one
        // element whose text is the active example, so `#playground.navbar.fixture` must resolve to the
        // combobox itself and read exactly that label — never the visually-hidden "Example" caption.
        const exampleTrigger = document.getElementById("playground.navbar.fixture")!;
        expect(exampleTrigger.getAttribute("role")).toBe("combobox");
        expect(exampleTrigger.textContent).toContain("Nakagin Capsule Tower with an intentionally long label");
        expect(exampleTrigger.textContent).not.toContain("Example");
        await user.click(exampleTrigger);
        const exampleMarkup = document.body.innerHTML;
  
        expect(selectMarkup).toContain('data-slot="select-chevron"');
        expect(selectMarkup).toContain('data-icon="chevron-down"');
        expect(selectMarkup).toContain("*:data-[slot=select-value]:min-w-0");
        expect(selectMarkup).toContain("*:data-[slot=select-value]:flex-1");
        expect(selectMarkup).toContain("shrink-0");
        expect(exampleMarkup).toContain('data-slot="select-trigger"');
        expect(exampleMarkup).toContain('data-slot="select-chevron"');
        expect(exampleMarkup).toContain('data-icon="chevron-down"');
        expect(exampleMarkup).toContain('data-slot="navbar-example-icon"');
        expect(exampleMarkup.match(/data-slot="navbar-example-icon"/g)?.length).toBe(1);
        expect(exampleMarkup).toContain('data-slot="select-item-icon"');
        expect(exampleMarkup).not.toMatch(/data-slot="select-item"[\s\S]*?<span className="flex items-center gap-single">[\s\S]*?data-icon-kind/);
      });
  
      it("renders section and item content slots flush under their headers", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" defaultOpen={true}>
              <TreeItem id="tooltip.tutorial" defaultOpen={true}>
                <Label id="tooltip.manual">
                  <span>Control</span>
                </Label>
              </TreeItem>
            </TreeSection>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-item-content"');
        expect(markup).toContain('data-slot="property-row"');
        expect(markup).toContain('data-slot="tree-item-content" data-tree-owner-kind="group" data-tree-owner-expanded="true" class="relative flex w-full min-w-0 flex-col"');
        expect(markup).not.toContain("padding-top:6px");
        expect(markup).not.toContain("padding-top:2px");
        expect(markup).not.toContain("margin-bottom:12px");
      });
  
      it("keeps guide wrappers continuous and pushes labels farther from the guide stroke", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" defaultOpen={true}>
              <TreeItem id="tooltip.tutorial" defaultOpen={true}>
                <TreeContent>
                  <span>Nested content</span>
                </TreeContent>
                <TreeItem id="tooltip.docs">
                  <TreeContent>
                    <span>Leaf content</span>
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            </TreeSection>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-content" data-tree-row-kind="content" class="relative w-full min-w-0"');
        expect(markup).toContain('data-slot="tree-gutter"');
        expect(markup).toContain('data-slot="tree-branch-elbow"');
        expect(markup).toContain('data-slot="tree-gutter-slot"');
        expect(markup).toContain("grid-template-columns:calc(");
        expect(markup).toContain("minmax(0, 1fr)");
        expect(markup).toContain("var(--spacing-double)");
        expect(markup).not.toMatch(/data-slot="tree-gutter"[^>]*><div class="absolute left-0 top-0 bottom-0 pointer-events-none"/);
        expect(markup).not.toContain('data-slot="tree-gutter-slot" class="absolute inset-y-0 left-0 flex items-center justify-center"');
        expect(markup).toContain('data-slot="tree-gutter-slot"');
        expect(markup).toContain('data-slot="tree-gutter-slot" class="absolute flex -translate-y-1/2 items-center justify-center"');
        expect(markup).toContain("var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
        expect(markup).toContain('data-slot="tree-branch-elbow"');
        expect(markup).not.toContain('style="top:50%;left:7px;width:3px"');
        expect(markup).toContain('data-slot="tree-branch-stem"');
        expect(markup.match(/data-tree-guide-line="" class="w-px h-full bg-muted-foreground\/40 group-hover\/tree-row:bg-emphasized/g)?.length ?? 0).toBeGreaterThanOrEqual(1);
        expect(markup).not.toContain('data-slot="tree-content" class="relative" style="padding-top:3px;padding-bottom:3px;padding-left:');
        expect(markup).not.toContain('data-slot="tree-property-label" class="relative min-w-0" style="padding-left:');
        expect(markup).toContain('data-slot="tree-item-content" data-tree-owner-kind="group" data-tree-owner-expanded="true" class="relative flex w-full min-w-0 flex-col"');
      });
  
      it("keeps tree gutter/guide geometry identical under an rtl FlowProvider — logical insets, not a direction-conditional physical side", () => {
        const build = () => (
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [false], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem id="rtl-check">
              <TreeContent>
                <span>Content</span>
              </TreeContent>
            </TreeItem>
          </TreeContext.Provider>
        );
        const ltrMarkup = renderToStaticMarkup(build());
        const rtlMarkup = renderToStaticMarkup(<FlowProvider inline="rtl">{build()}</FlowProvider>);
        expect(ltrMarkup).toBe(rtlMarkup);
        expect(rtlMarkup).toContain('data-slot="tree-gutter"');
        expect(rtlMarkup).toContain("inset-inline-start:");
        expect(rtlMarkup).not.toContain("left:");
        expect(rtlMarkup).not.toContain("right:");
      });
  
      it("renders sortable drag handles without bordered action chrome", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem id="tooltip.manual" sortable={true} sortableId="sortable-manual" isDragHandle={true} />
          </TreeContext.Provider>,
        );
  
        const handleClassName = markup.match(/data-slot="drag-handle"[^>]*class="([^"]+)"/)?.[1] ?? "";
        expect(markup).toContain('data-slot="drag-handle"');
        expect(handleClassName).toContain("cursor-grab");
        expect(handleClassName).not.toContain("hover:bg-hover");
        expect(markup).not.toContain('data-slot="drag-handle" class="text-foreground');
      });
  
      it("under the compact driver, sortable tree items render no drag handle (whole row is the surface)", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
              <TreeItem id="tooltip.manual" sortable={true} sortableId="sortable-manual" isDragHandle={true} />
            </TreeContext.Provider>
          </UiDriverProvider>,
        );
        expect(markup).not.toContain('data-slot="drag-handle"');
        expect(markup).toContain("cursor-grab");
      });
  
      it("renders control-tree folder branches inside the same continuous guide wrapper", () => {
        const markup = renderToStaticMarkup(
          <ControlTree
            controls={[
              {
                path: "Folder/Value",
                controlKind: "number",
                value: 3,
                onChange: () => undefined,
              },
            ]}
          />,
        );
  
        expect(markup).toContain('data-slot="control-tree-folder-content" data-tree-owner-expanded="false" class="relative flex w-full min-w-0 flex-col"');
        expect(markup).toContain('data-slot="control-tree-folder-label"');
        expect(markup).toContain('data-slot="control-tree-control-label"');
        expect(markup).toContain('data-slot="tree-row-layout"');
        expect(markup).toContain('data-slot="tree-gutter"');
        expect(markup).toContain("grid-template-columns:calc(");
        expect(markup).toContain("minmax(0, 1fr)");
        expect(markup).toContain("var(--spacing-double)");
        expect(markup).not.toContain("margin-left:13px");
      });
  
      it("truncates collapsed field text on word boundaries before falling back to characters", () => {
        const measureText = (value: string) => value.length * 8;
  
        expect(
          fitCollapsedFieldText({
            value: "Alpha beta gamma delta",
            maxWidth: measureText("Alpha beta..."),
            measureText,
          }),
        ).toBe("Alpha beta...");
  
        expect(
          fitCollapsedFieldText({
            value: "Supercalifragilisticexpialidocious",
            maxWidth: measureText("Supercali..."),
            measureText,
          }),
        ).toBe("Supercali...");
      });
  
      it("uses stacked overflow when enabled and inline ellipsis when disabled", () => {
        const measureText = (value: string) => value.length * 8;
        const stackedState = resolveCollapsedFieldDisplayState({
          allowStackedOverflow: true,
          value: "Alpha beta gamma delta",
          maxWidth: measureText("Alpha beta gamma"),
          measureText,
        });
        const inlineState = resolveCollapsedFieldDisplayState({
          value: "Alpha beta gamma delta",
          maxWidth: measureText("Alpha beta gamma"),
          measureText,
        });
  
        expect(stackedState.value).toBe("Alpha beta gamma");
        expect(stackedState.isOverflowing).toBe(true);
        expect(stackedState.layoutKind).toBe("stacked-overflow");
        expect(stackedState.value.endsWith(COLLAPSED_FIELD_ELLIPSIS)).toBe(false);
  
        expect(inlineState.value).toBe("Alpha beta...");
        expect(inlineState.isOverflowing).toBe(true);
        expect(inlineState.layoutKind).toBe("single-line");
        expect(inlineState.value.endsWith(COLLAPSED_FIELD_ELLIPSIS)).toBe(true);
      });
  
      it("keeps single-line text fields in the normal state when the text still fits", () => {
        const measureText = (value: string) => value.length * 8;
        const fittingState = resolveCollapsedFieldDisplayState({
          allowStackedOverflow: true,
          value: "Nakagin Capsule Tower",
          maxWidth: measureText("Nakagin Capsule Tower"),
          measureText,
        });
  
        expect(fittingState.isOverflowing).toBe(false);
        expect(fittingState.layoutKind).toBe("single-line");
        expect(fittingState.value).toBe("Nakagin Capsule Tower");
      });
  
      it("enables stacked overflow only after the rendered value exceeds the inner field width", () => {
        const measureText = (value: string) => value.length * 8;
        const exactFitState = resolveCollapsedFieldDisplayState({
          allowStackedOverflow: true,
          value: "Nakagin Capsule Tower",
          maxWidth: measureText("Nakagin Capsule Tower"),
          measureText,
        });
        const overflowingState = resolveCollapsedFieldDisplayState({
          allowStackedOverflow: true,
          value: "Nakagin Capsule Tower",
          maxWidth: measureText("Nakagin Capsule Towe"),
          measureText,
        });
  
        expect(exactFitState.isOverflowing).toBe(false);
        expect(exactFitState.layoutKind).toBe("single-line");
        expect(overflowingState.isOverflowing).toBe(true);
        expect(overflowingState.layoutKind).toBe("stacked-overflow");
        expect(overflowingState.value).toBe("Nakagin Capsule");
      });
  
      it("keeps tree section actions inline with the header row when isTree is true", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" defaultOpen={false} actions={[{ icon: <span data-testid="add-icon" />, onClick: () => undefined }]} />
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('class="flex h-full items-center gap-double min-w-0 w-full"');
        expect(markup).toContain('data-slot="tree-header-actions"');
        expect(markup).not.toContain('data-slot="property-control"');
        const rowContentIdx = markup.indexOf('data-slot="tree-row-content"');
        const actionsIdx = markup.indexOf('data-testid="add-icon"');
        expect(rowContentIdx).toBeGreaterThan(-1);
        expect(actionsIdx).toBeGreaterThan(-1);
        expect(actionsIdx).toBeGreaterThan(rowContentIdx);
      });
});
}
