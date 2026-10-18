import type { IntroductionStepDefinition, ModeWindowDescriptor, UiDriver } from "@semio-tech/ui-react/source";
import type { WindowLayoutNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import * as ui from "@semio-tech/ui-react/source";
import { createBrowserStoragePort } from "../../../🔨️modules/🗄️storage-port/🟦️.ts";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerUidriverAndMoreTests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { Button, CheckIcon, cn, COMPACT_UI_DRIVER, composeControlKeybindings, createBrowserStoragePort, DEFAULT_UI_DRIVER, deriveTreeDragRoles, treeRowDragPayloadAttributes, DragHandle, Icon, Mode, parseUiDriver, readStoredUiChromeLayout, renderToStaticMarkup, resolveUiDriver, Search, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, serializeUiDriver, Slider, Table, Toggle, TreeCheckbox, TreeContext, TreeItem, UI_CHROME_LAYOUT_STORAGE_KEY, uiDataLabel, UiDriverProvider, UIIntroduction, UiKeybindingsProvider, Window, WindowMeasuresTree, windowMeasureToggleClass, windowMeasureToggleCompactClass, WindowMeasureTreeGroup, WindowMeasureTreeLeaf, writeStoredUiChromeLayout } = dependencies;
  const { describe, expect, it } = vitest;

  
    describe("UiDriver", () => {
      it("round-trips a custom driver through parseUiDriver/serializeUiDriver", () => {
        const custom: UiDriver = { id: "custom.mine", label: "Mine", labels: "icons", labelTier: "beginner", drag: "surface", chrome: "hover", gumball: "always", tooltips: "minimal", hotkeys: "tooltip" };
        const parsed = parseUiDriver(JSON.parse(serializeUiDriver(custom)));
        expect(parsed).toEqual(custom);
      });
  
      it("shows introduction footer hotkey badges under the default driver", async () => {
        const { render } = await import("@testing-library/react");
        const steps: IntroductionStepDefinition[] = [
          { id: "a", title: "A", body: "One", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
          { id: "b", title: "B", body: "Two", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        ];
        const { container } = render(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <UIIntroduction introduction={{ title: "T", steps }} stepIndex={1} onStepIndexChange={() => undefined} onDismiss={() => undefined} />
          </UiDriverProvider>,
        );
        expect(container.querySelector('[data-slot="control-hotkey"]')).toBeTruthy();
      });
  
      it("throws on an invalid axis value", () => {
        expect(() => parseUiDriver({ ...DEFAULT_UI_DRIVER, labels: "both" })).toThrow();
        expect(() => parseUiDriver({ ...DEFAULT_UI_DRIVER, tooltips: "verbose" })).toThrow();
      });
  
      it("resolveUiDriver prefers a custom driver over a builtin id, then falls back to default", () => {
        const custom: UiDriver = { ...COMPACT_UI_DRIVER, id: "compact", label: "My Compact", tooltips: "full" };
        expect(resolveUiDriver("compact", { compact: custom })).toEqual(custom);
        expect(resolveUiDriver("compact", {})).toEqual(COMPACT_UI_DRIVER);
        expect(resolveUiDriver("unknown.id", {})).toEqual(DEFAULT_UI_DRIVER);
      });
  
      it("deriveTreeDragRoles separates catalogue transfer from in-tree sort", () => {
        expect(deriveTreeDragRoles({ draggable: true, dragData: { "application/x-test": "{}" } }, true)).toEqual(["transfer"]);
        expect(deriveTreeDragRoles({ draggable: true }, false)).toEqual(["sort"]);
        expect(deriveTreeDragRoles({ draggable: true, dragData: { "application/x-test": "{}" }, isDragHandle: true }, true)).toEqual(["sort", "transfer"]);
      });
  
      it("a transfer row mirrors its drag payload onto the DOM", () => {
        expect(treeRowDragPayloadAttributes(undefined)).toEqual({});
        expect(treeRowDragPayloadAttributes({ "application/x-semio-catalogue-item": "" })).toEqual({});
        expect(treeRowDragPayloadAttributes({ "application/x-semio-catalogue-item": '{"objectKind":"b-l"}' })).toEqual({
          "data-drag-mime": "application/x-semio-catalogue-item",
          "data-drag-payload": '{"objectKind":"b-l"}',
        });
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
              <TreeItem id="tooltip.manual" label="Kind" draggable dragRoles={["transfer"]} dragInitiation="handle" dragData={{ "application/x-semio-catalogue-item": '{"objectKind":"b-l"}' }} />
            </TreeContext.Provider>
          </UiDriverProvider>,
        );
        // 📮️ A native HTML5 drag cannot be driven by synthetic pointer moves, so the bytes a real
        // `dragstart` would put on the `DataTransfer` have to be readable from the row itself.
        expect(markup).toContain('data-drag-mime="application/x-semio-catalogue-item"');
        expect(markup).toContain("data-drag-payload=");
        expect(markup).toContain('data-draggable="true"');
      });

      it("default driver keeps catalogue transfer on the move handle only", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
              <TreeItem id="tooltip.manual" label="Kind" draggable dragRoles={["transfer"]} dragInitiation="handle" />
            </TreeContext.Provider>
          </UiDriverProvider>,
        );
        expect(markup).toContain('data-drag-role="transfer"');
        expect(markup).not.toContain('data-drag-role="sort"');
        expect(markup).toContain('data-slot="tree-item-row"');
        expect(markup).toMatch(/data-slot="tree-item-row"[^>]*draggable="false"/);
        expect(markup).toContain('data-slot="drag-handle"');
      });
  
      it("compact driver collapses drag roles onto the row surface", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
              <TreeItem id="tooltip.manual" label="Kind" draggable dragRoles={["sort", "transfer"]} dragInitiation="handle" />
            </TreeContext.Provider>
          </UiDriverProvider>,
        );
        expect(markup).not.toContain('data-slot="drag-handle"');
        expect(markup).toContain('draggable="true"');
      });
  
      it("table HTML5 drag rows expose a transfer handle under the default driver", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Table
              columns={[{ id: "name", header: "Name", accessor: (row: { name: string }) => row.name }]}
              data={[{ id: "r1", name: "Row", _drag: { kind: "x" } }]}
              getRowId={(row) => row.id}
              rowDragProps={(row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: () => undefined,
                    }
                  : {}
              }
            />
          </UiDriverProvider>,
        );
        expect(markup).toContain('data-slot="drag-handle"');
        expect(markup).toContain('data-drag-role="transfer"');
        expect(markup).not.toMatch(/<tr[^>]*draggable="true"/);
        expect(markup).not.toMatch(/<tr[^>]*cursor-grab/);
      });
  
      it("compact driver keeps table rows surface-draggable without a transfer handle", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <Table
              columns={[{ id: "name", header: "Name", accessor: (row: { name: string }) => row.name }]}
              data={[{ id: "r1", name: "Row", _drag: { kind: "x" } }]}
              getRowId={(row) => row.id}
              rowDragProps={(row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: () => undefined,
                    }
                  : {}
              }
            />
          </UiDriverProvider>,
        );
        expect(markup).not.toContain('data-slot="drag-handle"');
        expect(markup).toContain('draggable="true"');
      });
  
      it("useControlInlineText hides labels only under an icons-only driver", () => {
        const iconsMarkup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <Button id="settings.driver.compact" icon={<CheckIcon />} />
          </UiDriverProvider>,
        );
        expect(iconsMarkup).not.toContain(">Compact<");
        const fullMarkup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Button id="settings.driver.compact" icon={<CheckIcon />} />
          </UiDriverProvider>,
        );
        expect(fullMarkup).toContain(">Compact<");
      });
  
      it("icon-only controls expose tooltip titles with hotkeys when the driver shows them", () => {
        const tooltipDriver: UiDriver = { ...COMPACT_UI_DRIVER, tooltips: "minimal", hotkeys: "tooltip" };
        const markup = renderToStaticMarkup(
          <UiKeybindingsProvider bindings={composeControlKeybindings(new Map(), {})}>
            <UiDriverProvider driver={tooltipDriver}>
              <Button id="ui.search.toggle" icon={<CheckIcon />} />
            </UiDriverProvider>
          </UiKeybindingsProvider>,
        );
        expect(markup).toMatch(/title="Search.*\(/);
      });
  
      it("drag handles expose glass chrome tooltips instead of native title attributes", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <DragHandle subject="Perspective Window" />
          </UiDriverProvider>,
        );
        expect(markup).toContain('data-slot="chrome-control-hint"');
        expect(markup).not.toMatch(/data-slot="drag-handle"[^>]*\btitle=/);
      });
    });

  
    describe("UiChromeLayout storage", () => {
      const storage = createBrowserStoragePort();
  
      it("defaults to desktop when nothing is stored", () => {
        storage.remove(UI_CHROME_LAYOUT_STORAGE_KEY);
        expect(readStoredUiChromeLayout(storage)).toBe("desktop");
      });
  
      it("round-trips a written tablet preference", () => {
        writeStoredUiChromeLayout(storage, "tablet");
        expect(readStoredUiChromeLayout(storage)).toBe("tablet");
        writeStoredUiChromeLayout(storage, "desktop");
        expect(readStoredUiChromeLayout(storage)).toBe("desktop");
      });
  
      it("falls back to desktop for a garbage stored value", () => {
        storage.set(UI_CHROME_LAYOUT_STORAGE_KEY, "giant-monitor");
        expect(readStoredUiChromeLayout(storage)).toBe("desktop");
        storage.remove(UI_CHROME_LAYOUT_STORAGE_KEY);
      });
    });

  
    describe("Mode mobile", () => {
      const windows: ModeWindowDescriptor[] = [
        { id: "a", title: uiDataLabel("A"), iconId: "app-window", children: <div>A body</div> },
        { id: "b", title: uiDataLabel("B"), iconId: "app-window", children: <div>B body</div> },
      ];
      const splitLayout: WindowLayoutNode = {
        kind: "row",
        children: [
          { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
          { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
        ],
      };
  
      it("collapses split panes into one tab stack with the regular window chrome, dropping Focus but keeping Close", () => {
        const markup = renderToStaticMarkup(<Mode mobile windows={windows} activeWindowId="b" layout={splitLayout} />);
        expect(markup).toContain('data-slot="mode-dock-tabbar"');
        expect(markup).toContain('data-slot="mode-dock-tab"');
        expect(markup).not.toContain('data-slot="mode-dock-tab-focus"');
        expect(markup).toContain('data-slot="mode-dock-tab-close"');
        expect(markup).toContain("A");
        expect(markup).toContain("B body");
        expect(markup).not.toContain("A body");
      });
  
      it("falls back to the first ordered window when activeWindowId is stale", () => {
        const markup = renderToStaticMarkup(<Mode mobile windows={windows} activeWindowId="missing" layout={splitLayout} />);
        expect(markup).toContain("A body");
        expect(markup).not.toContain("B body");
      });
  
      it("drops the Focus control out of the unshrinkable per-tab chrome grid even with many windows — windows always take the full space on mobile", () => {
        const manyWindows: ModeWindowDescriptor[] = ["a", "b", "c", "d"].map((id) => ({ id, title: uiDataLabel(`semio · cad · ${id}`), iconId: "app-window", children: <div>{id} body</div> }));
        const manyLayout: WindowLayoutNode = {
          kind: "row",
          children: manyWindows.map((window) => ({ kind: "stack", children: [{ kind: "window", id: window.id }], activeId: window.id })),
        };
        const markup = renderToStaticMarkup(<Mode mobile windows={manyWindows} activeWindowId="a" layout={manyLayout} />);
        expect(markup).not.toContain('data-slot="mode-dock-chrome-column"');
        expect(markup).toContain('data-slot="mode-dock-tab-cap"');
        expect(markup).not.toContain('data-slot="mode-dock-tab-focus"');
        expect(markup.match(/data-slot="mode-dock-tab"/g)?.length).toBe(4);
      });
  
      it("keeps the Focus control on desktop for the same layout", () => {
        const markup = renderToStaticMarkup(<Mode windows={windows} activeWindowId="b" layout={splitLayout} />);
        expect(markup).toContain('data-slot="mode-dock-tab-focus"');
        expect(markup).toContain('data-slot="mode-dock-tab-close"');
      });
    });

  
    describe("WindowMeasuresTree", () => {
      it("renders inline label and control on one measure row", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeLeaf label={uiDataLabel("LOD")}>
              <Select id="lod-mode.select" defaultValue="automatic">
                <SelectTrigger id="lod-mode">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="automatic">Auto</SelectItem>
                </SelectContent>
              </Select>
            </WindowMeasureTreeLeaf>
          </WindowMeasuresTree>,
        );
        expect(markup).toContain("LOD");
        expect(markup).toContain('data-slot="select-trigger"');
        expect(markup).toContain('data-slot="window-measure-tree-row-right"');
        expect(markup).not.toContain("ring-[3px]");
        expect(markup).not.toContain("border-emphasized");
        expect(markup).not.toContain('data-slot="tree-section-row"');
      });
  
      it("puts the toggle icon before the label and a checkbox on the right", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeLeaf label={uiDataLabel("Grid")} icon={<Icon icon="layout-grid" size={12} />}>
              <TreeCheckbox id="grid-visible" checked title={uiDataLabel("Grid")} onCheckedChange={() => undefined} />
            </WindowMeasureTreeLeaf>
          </WindowMeasuresTree>,
        );
        const iconIdx = markup.indexOf('data-slot="tree-icon"');
        const labelIdx = markup.indexOf('data-slot="tree-label"');
        const checkboxIdx = markup.indexOf('data-slot="tree-action-checkbox"');
        const rightIdx = markup.indexOf('data-slot="window-measure-tree-row-right"');
        expect(iconIdx).toBeGreaterThan(-1);
        expect(labelIdx).toBeGreaterThan(iconIdx);
        expect(checkboxIdx).toBeGreaterThan(labelIdx);
        expect(rightIdx).toBeGreaterThan(-1);
        expect(checkboxIdx).toBeGreaterThan(rightIdx);
        expect(markup).toContain('type="checkbox"');
        expect(markup).toContain('id="grid-visible"');
        expect(markup).toContain("Grid");
        expect(markup).toContain('data-icon="layout-grid"');
      });
  
      it("keeps measure-row hover fill and puts slider loading on the track wrap only", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeGroup id="fill" label="Fill" defaultOpen>
              <WindowMeasureTreeLeaf label={uiDataLabel("Count")}>
                <Slider id="puzzle3d-fill-count" value={[0]} min={0} max={1000} ready={120} loading />
              </WindowMeasureTreeLeaf>
            </WindowMeasureTreeGroup>
          </WindowMeasuresTree>,
        );
        expect(markup).toContain('data-slot="window-measure-tree-row"');
        expect(markup).toContain("hover:bg-hover-interactive-fill");
        expect(markup).toContain('data-slot="slider-thumb"');
        expect(markup).toContain("group-hover:bg-emphasized");
        expect(markup).toContain('data-slot="slider-track-wrap"');
        expect(markup).toContain('data-loading="true"');
        expect(markup).toContain("border-loading");
        expect(markup).not.toMatch(/data-slot="window-measure-tree-row"[^>]*border-loading/);
        expect(markup).toContain('data-tree-guide-line=""');
        expect(markup).toMatch(/data-tree-guide-line="" class="w-px h-full bg-muted-foreground\/40 group-hover\/tree-row:bg-emphasized/);
      });
  
      it("puts a waiting ring on a window measure tree leaf's own row", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeGroup id="fill" label="Fill" defaultOpen>
              <WindowMeasureTreeLeaf label={uiDataLabel("Count")} waiting>
                <span>0</span>
              </WindowMeasureTreeLeaf>
            </WindowMeasureTreeGroup>
          </WindowMeasuresTree>,
        );
        expect(markup).toMatch(/data-slot="window-measure-tree-row"[^>]*data-waiting="true"/);
        expect(markup).toMatch(/data-slot="window-measure-tree-row"[^>]*border-waiting/);
      });
  
      it("renders nested measure groups with branch guide lines", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeGroup id="display" label="Display" defaultOpen={true}>
              <WindowMeasureTreeLeaf label={uiDataLabel("LOD")}>
                <span>Auto</span>
              </WindowMeasureTreeLeaf>
            </WindowMeasureTreeGroup>
          </WindowMeasuresTree>,
        );
        expect(markup).toContain('data-slot="window-measure-tree-content"');
        expect(markup).toContain('data-slot="tree-guide"');
        expect(markup.match(/data-tree-guide-line="" class="w-px h-full bg-muted-foreground\/40 group-hover\/tree-row:bg-emphasized/g)?.length ?? 0).toBeGreaterThanOrEqual(1);
        expect(markup).toContain('data-slot="tree-branch-elbow"');
      });
  
      it("renders bottom-pane measure trees upward with children above their group row", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree direction="up">
            <WindowMeasureTreeGroup id="fill" label="Fill" defaultOpen>
              <WindowMeasureTreeLeaf label={uiDataLabel("Count")}>
                <span>1000</span>
              </WindowMeasureTreeLeaf>
              <WindowMeasureTreeLeaf label={uiDataLabel("Mode")}>
                <span>Voxel</span>
              </WindowMeasureTreeLeaf>
            </WindowMeasureTreeGroup>
          </WindowMeasuresTree>,
        );
        expect(markup).toContain('data-direction="up"');
        expect(markup.indexOf("Voxel")).toBeLessThan(markup.indexOf("Count"));
        expect(markup.indexOf("Count")).toBeLessThan(markup.indexOf("Fill"));
        expect(markup).toContain('data-icon="chevron-up"');
        const stemStyle = markup.match(/data-slot="tree-branch-stem"[^>]*style="([^"]*)"/)?.[1] ?? "";
        expect(stemStyle).toContain("top:0");
        expect(stemStyle).toContain("bottom:var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
      });
  
      it("stretches full-width measure toggles so active fill spans the tree row", () => {
        const markup = renderToStaticMarkup(
          <WindowMeasuresTree>
            <WindowMeasureTreeLeaf fullWidth>
              <Toggle id="move-axes" className={cn(windowMeasureToggleClass, windowMeasureToggleCompactClass)} pressed icon={<CheckIcon className="size-small" />} text="Move Axes" />
            </WindowMeasureTreeLeaf>
          </WindowMeasuresTree>,
        );
        expect(markup).toContain('data-slot="toggle-group"');
        expect(markup).toContain("!w-full");
        expect(markup).toContain("[&amp;_[data-slot=toggle-group-item]]:!flex-1");
        expect(markup).toContain("[&amp;_[data-slot=toggle-group-item]]:!shrink");
        expect(markup).toContain('data-state="on"');
      });
    });
}
