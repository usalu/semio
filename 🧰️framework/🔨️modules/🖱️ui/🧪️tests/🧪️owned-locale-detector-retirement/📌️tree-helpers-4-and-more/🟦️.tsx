import type { VirtualFileSystemNode, VirtualFileSystemRow, VirtualFileSystemSchema } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerTreeHelpers4AndMoreTests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { applyChromeRevealAtPoint, applyElementsSurfaceChrome, bootstrapElementsSurfaceChromeDocument, buildVirtualFileSystemDescriptorColumns, buildVirtualFileSystemVisibleRows, COMPACT_UI_DRIVER, DEFAULT_UI_DRIVER, formatVirtualFileSystemTime, getVirtualFileSystemNextSelectionState, Input, Label, renderToStaticMarkup, resetElementsSurfaceChromeForTests, resolveSceneGizmoSnapTarget, resolveSceneGizmoViewportPlacement, resolveVirtualFileSystemSchemaIcon, SCENE_GIZMO_LABELS, Stepper, Textarea, THREE, Toggle, TreeContext, TreeItem, uiDataLabel, useCanvasAppearanceSync, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VirtualFileSystem } = dependencies;
  const { describe, expect, it, vi } = vitest;

describe("tree helpers", () => {

  
      it("keeps tree item actions inline with the header row when isTree is true", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem id="tooltip.manual" actions={[{ icon: <span data-testid="remove-icon" />, onClick: () => undefined }]} />
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('class="flex h-full items-center gap-double min-w-0 w-full"');
        expect(markup).toContain('data-slot="tree-header-actions"');
        expect(markup).not.toContain('data-slot="property-control"');
        expect(markup).toContain('data-testid="remove-icon"');
      });
  
      it("keeps row-placement actions visible and routes menu-placement actions to the context menu", async () => {
        const { render } = await import("@testing-library/react");
        const { container } = render(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem
              id="tooltip.manual"
              actions={[
                { id: "persistent", icon: <span data-testid="persistent-icon" />, onClick: () => undefined },
                { id: "menu-only", icon: <span data-testid="menu-icon" />, onClick: () => undefined, placement: "menu" },
              ]}
            />
          </TreeContext.Provider>,
        );
  
        const actions = container.querySelector<HTMLElement>('[data-slot="tree-header-actions"]');
        expect(actions).toBeTruthy();
        expect(container.querySelector('[data-slot="tree-header-reveal-actions"]')).toBeNull();
        expect(actions?.querySelector('[data-testid="persistent-icon"]')).toBeTruthy();
        expect(actions?.querySelector('[data-testid="menu-icon"]')).toBeNull();
        expect(actions?.getAttribute("data-ui-reveal-region")).toBe("");
      });
  
      it("uses the same inline tree header actions when isTree is false", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: false, indentMultiplier: 1 }}>
            <TreeItem id="tooltip.manual" actions={[{ icon: <span data-testid="add-icon" />, onClick: () => undefined }]} />
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-header-actions"');
        expect(markup).not.toContain('data-slot="property-control"');
        expect(markup).toContain('data-testid="add-icon"');
      });
  
      it("renders checkbox actions inline with tree headers", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem
              id="tooltip.manual"
              actions={[
                {
                  kind: "checkbox",
                  id: "tree-checkbox-action",
                  checked: true,
                  title: uiDataLabel("Toggle item"),
                  onCheckedChange: () => undefined,
                },
              ]}
            />
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-header-actions"');
        expect(markup).toContain('data-slot="tree-action-checkbox-wrapper"');
        expect(markup).toContain('data-slot="tree-action-checkbox"');
        expect(markup).toContain('id="tree-checkbox-action"');
        expect(markup).toContain('type="checkbox"');
        expect(markup).toContain('checked=""');
        expect(markup).toContain('aria-label="Toggle item"');
      });
  
      it("renders empty Input inside a Label property row with muted opacity and full opacity when value is present", () => {
        const emptyMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Input id="tooltip.manual" value="" />
          </Label>,
        );
        const filledMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Input id="tooltip.manual" value="hello" />
          </Label>,
        );
        const standaloneMarkup = renderToStaticMarkup(<Input id="tooltip.manual" value="" />);
  
        expect(emptyMarkup).toContain('data-slot="input-root"');
        expect(emptyMarkup).toContain("opacity:0.6");
        expect(filledMarkup).toContain("opacity:1");
        expect(standaloneMarkup).not.toContain("opacity:0.6");
      });
  
      it("renders empty Textarea inside a Label property row with muted opacity and full opacity when value is present", () => {
        const emptyMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Textarea id="tooltip.manual" value="" />
          </Label>,
        );
        const filledMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Textarea id="tooltip.manual" value="some text" />
          </Label>,
        );
        const standaloneMarkup = renderToStaticMarkup(<Textarea id="tooltip.manual" value="" />);
  
        expect(emptyMarkup).toContain('data-slot="textarea-root"');
        expect(emptyMarkup).toContain("opacity:0.6");
        expect(filledMarkup).toContain("opacity:1");
        expect(standaloneMarkup).not.toContain("opacity:0.6");
      });
  
      it("renders Stepper with undefined value inside a Label property row with muted opacity and full opacity when value is defined", () => {
        const emptyMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Stepper id="ui.stepper.demo" value={undefined} />
          </Label>,
        );
        const filledMarkup = renderToStaticMarkup(
          <Label id="tooltip.manual">
            <Stepper id="ui.stepper.demo" value={5} />
          </Label>,
        );
        const standaloneMarkup = renderToStaticMarkup(<Stepper id="ui.stepper.demo" value={undefined} />);
  
        expect(emptyMarkup).toContain('data-slot="stepper-group"');
        expect(emptyMarkup).toContain("opacity:0.6");
        expect(filledMarkup).toContain("opacity:1");
        expect(standaloneMarkup).not.toContain("opacity:0.6");
      });
});

  
    describe("VirtualFileSystem", () => {
      it("buildVirtualFileSystemVisibleRows only includes children of expanded parents", () => {
        const root: VirtualFileSystemNode = { id: "root", fileNodeKindId: "root", name: "Root", hasChildren: true };
        const childrenByParentId = new Map<string, readonly VirtualFileSystemNode[]>([
          [
            "root",
            [
              { id: "f1", fileNodeKindId: "branch", name: "Models", parentId: "root", hasChildren: true },
              { id: "d1", fileNodeKindId: "leaf", name: "Tower", parentId: "root", hasChildren: false },
            ],
          ],
          ["f1", [{ id: "t1", fileNodeKindId: "leaf", name: "Capsule", parentId: "f1", hasChildren: false }]],
        ]);
        const collapsed = buildVirtualFileSystemVisibleRows("root", childrenByParentId, new Set(["root"]), root);
        expect(collapsed.map((row) => row.id)).toEqual(["f1", "d1"]);
        const expanded = buildVirtualFileSystemVisibleRows("root", childrenByParentId, new Set(["root", "f1"]), root);
        expect(expanded.map((row) => row.id)).toEqual(["f1", "t1", "d1"]);
      });
  
      it("buildVirtualFileSystemDescriptorColumns renders avatar and time cells", () => {
        const schemaWithMeta: VirtualFileSystemSchema = {
          ...VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
          descriptorColumnIds: ["updated", "createdBy", "path", "fileNodeKind"],
          fileNodeKinds: {
            ...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
            root: {
              ...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.root,
              descriptors: [...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.root.descriptors, { id: "updated", descriptorKindId: "time", label: uiDataLabel("Updated") }, { id: "createdBy", descriptorKindId: "avatar", label: uiDataLabel("Created by") }],
            },
          },
        };
        const columns = buildVirtualFileSystemDescriptorColumns(schemaWithMeta, "en");
        const createdBy = columns.find((column) => column.id === "createdBy");
        expect(createdBy?.header).toBe("Created by");
        const updated = columns.find((column) => column.id === "updated");
        expect(updated?.header).toBe("Updated");
        const row: VirtualFileSystemRow = {
          id: "root:1",
          fileNodeKindId: "root",
          name: "Alpha",
          level: 0,
          descriptorValues: {
            updated: { presentation: "time", iso: "2026-05-01T12:00:00.000Z" },
            createdBy: { presentation: "avatar", name: "Ada", icon: "https://example.com/a.png" },
          },
        };
        const updatedMarkup = renderToStaticMarkup(<>{updated?.accessor(row)}</>);
        expect(updatedMarkup).toContain("2026");
        const avatarColumn = buildVirtualFileSystemDescriptorColumns(
          {
            ...schemaWithMeta,
            descriptorColumnIds: ["createdBy"],
          },
          "en",
        )[0];
        const avatarMarkup = renderToStaticMarkup(<>{avatarColumn?.accessor(row)}</>);
        expect(avatarMarkup).toContain("avatar-fallback");
        expect(avatarMarkup).toContain(">A<");
      });
  
      it("formats calendar and relative timestamps through the owned locale-explicit formatter", () => {
        const calendar = new Date(2026, 4, 1, 12, 34);
        expect(formatVirtualFileSystemTime(calendar, "date", "en")).toBe(new Intl.DateTimeFormat("en", { year: "numeric", month: "2-digit", day: "2-digit" }).format(calendar));
        expect(formatVirtualFileSystemTime(calendar, "datetime", "de")).toBe(new Intl.DateTimeFormat("de", { year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit" }).format(calendar));
        const now = new Date("2026-05-01T12:00:00.000Z");
        const future = new Date("2026-05-01T14:00:00.000Z");
        const past = new Date("2026-05-01T10:00:00.000Z");
        expect(formatVirtualFileSystemTime(future, "relative", "en", now)).toContain("2 hours");
        expect(formatVirtualFileSystemTime(past, "relative", "de", now)).toContain("2 Stunden");
      });
  
      it("renders expand affordance only for rows with children", () => {
        const markup = renderToStaticMarkup(
          <VirtualFileSystem
            schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA}
            rows={[
              { id: "root", fileNodeKindId: "root", name: "Root", level: 0, hasChildren: true, isExpanded: true },
              { id: "file", fileNodeKindId: "leaf", name: "readme.md", level: 1, hasChildren: false },
            ]}
          />,
        );
        expect(markup).toContain("data-vfs-expand");
        expect(markup).toContain("readme.md");
        expect(markup).toContain("cursor-selectable");
        expect(markup).toContain("text-element");
        expect(markup).not.toContain("text-muted-foreground");
      });
  
      it("renders file node kind vendored icons instead of avatars for schema icon ids", () => {
        const markup = renderToStaticMarkup(<VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "root", fileNodeKindId: "root", name: "Alpha", level: 0, hasChildren: false }]} />);
        expect(markup).toContain('data-icon="layout-grid"');
        expect(markup).not.toContain("avatar-fallback");
      });
  
      it("resolveVirtualFileSystemSchemaIcon maps sketchpad vfs icon ids", () => {
        expect(resolveVirtualFileSystemSchemaIcon("component")).toBe("component");
        expect(resolveVirtualFileSystemSchemaIcon("circle-dot")).toBe("circle-dot");
        expect(resolveVirtualFileSystemSchemaIcon("type")).toBe("component");
      });
  
      it("resolveVirtualFileSystemSchemaIcon maps file extension ids", () => {
        expect(resolveVirtualFileSystemSchemaIcon("glb")).toBe("box");
        expect(resolveVirtualFileSystemSchemaIcon("pdf")).toBe("file-type");
        expect(resolveVirtualFileSystemSchemaIcon("json")).toBe("file-json");
      });
  
      it("renders per-row extension icons for kit files", () => {
        const markup = renderToStaticMarkup(<VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "f1", fileNodeKindId: "leaf", name: "Tower", icon: "glb", level: 0, hasChildren: false }]} />);
        expect(markup).toContain('data-icon="box"');
        expect(markup).not.toContain("avatar-fallback");
      });
  
      it("invokes onRowDoubleClick on double-click", async () => {
        const { render } = await import("@testing-library/react");
        const userEvent = (await import("@testing-library/user-event")).default;
        const user = userEvent.setup();
        const onRowDoubleClick = vi.fn();
        const { container } = render(
          <VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={[{ id: "leaf-a", fileNodeKindId: "leaf", name: "Alpha", level: 0, hasChildren: false, navigateUri: "/alpha" }]} onRowDoubleClick={onRowDoubleClick} />,
        );
        const leafRow = container.querySelector('tr[data-row-id="leaf-a"]');
        expect(leafRow).toBeTruthy();
        await user.dblClick(leafRow!);
        expect(onRowDoubleClick).toHaveBeenCalledWith(expect.objectContaining({ id: "leaf-a", navigateUri: "/alpha" }), 0);
      });
  
      it("computes shift range and ctrl toggle selection for visible rows", () => {
        const orderedRowIds = ["root", "branch", "leaf-a", "leaf-b"];
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "multiple",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-b",
            anchorRowId: "root",
            additiveKey: false,
            rangeKey: true,
          }).selectedRowIds,
        ).toEqual(["root", "branch", "leaf-a", "leaf-b"]);
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "multiple",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-b",
            anchorRowId: "root",
            additiveKey: true,
            rangeKey: false,
          }).selectedRowIds,
        ).toEqual(["root", "leaf-b"]);
        expect(
          getVirtualFileSystemNextSelectionState({
            selectionMode: "single",
            selectedRowIds: ["root"],
            orderedRowIds,
            targetRowId: "leaf-a",
            additiveKey: true,
            rangeKey: true,
          }).selectedRowIds,
        ).toEqual(["leaf-a"]);
      });
    });

  
    describe("scene helpers", () => {
      it("keeps the 3d gizmo label-free", () => {
        expect(SCENE_GIZMO_LABELS).toEqual(["", "", ""]);
      });
  
      it("maps dominant gizmo axes to blender-style orthographic snap targets", () => {
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(1, 0.2, 0.1))).toEqual({
          axis: "x",
          sign: 1,
          view: "side",
          cameraDirection: { x: 1, y: 0, z: 0 },
          up: { x: 0, y: 1, z: 0 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 1, 0.2))).toEqual({
          axis: "y",
          sign: 1,
          view: "top",
          cameraDirection: { x: 0, y: 1, z: 0 },
          up: { x: 0, y: 0, z: -1 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 0.2, -1))).toEqual({
          axis: "z",
          sign: -1,
          view: "back",
          cameraDirection: { x: 0, y: 0, z: -1 },
          up: { x: 0, y: 1, z: 0 },
        });
      });
  
      it("preserves the complementary blender views for negative axis clicks", () => {
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(-1, 0, 0))).toEqual({
          axis: "x",
          sign: -1,
          view: "opposite-side",
          cameraDirection: { x: -1, y: 0, z: 0 },
          up: { x: 0, y: 1, z: 0 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, -1, 0))).toEqual({
          axis: "y",
          sign: -1,
          view: "bottom",
          cameraDirection: { x: 0, y: -1, z: 0 },
          up: { x: 0, y: 0, z: 1 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, 0, 1))).toEqual({
          axis: "z",
          sign: 1,
          view: "front",
          cameraDirection: { x: 0, y: 0, z: 1 },
          up: { x: 0, y: 1, z: 0 },
        });
      });
  
      it("keeps the gizmo in the bottom-right corner above the folded projection pane chrome", () => {
        expect(resolveSceneGizmoViewportPlacement({ width: 1280, height: 720 })).toEqual({
          alignment: "bottom-right",
          margin: [32, 58],
        });
  
        expect(resolveSceneGizmoViewportPlacement({ width: 120, height: 160 })).toEqual({
          alignment: "bottom-right",
          margin: [32, 53],
        });
  
        expect(resolveSceneGizmoViewportPlacement({ width: 40, height: 48 })).toEqual({
          alignment: "bottom-right",
          margin: [22, 22],
        });
      });
    });

  
    describe("ElementsSurfaceChrome", () => {
      it("keeps dark class while nested leases are active", () => {
        resetElementsSurfaceChromeForTests();
        const outer = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        const inner = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("dark")).toBe(false);
        inner();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        outer();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        resetElementsSurfaceChromeForTests();
        expect(document.documentElement.classList.contains("dark")).toBe(false);
      });
  
      it("reapplies the previous lease after the top lease releases", () => {
        resetElementsSurfaceChromeForTests();
        const outer = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        const inner = applyElementsSurfaceChrome({ appearance: "dark", device: "tablet", driver: COMPACT_UI_DRIVER });
        expect(document.documentElement.dataset.uiDevice).toBe("tablet");
        expect(document.documentElement.dataset.uiDriver).toBe("compact");
        inner();
        expect(document.documentElement.dataset.uiDevice).toBe("desktop");
        expect(document.documentElement.dataset.uiDriver).toBe("default");
        outer();
        resetElementsSurfaceChromeForTests();
      });
  
      it("defers clearing dark until the next animation frame when the last lease releases", () => {
        resetElementsSurfaceChromeForTests();
        let scheduled: FrameRequestCallback | undefined;
        const raf = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation((callback: FrameRequestCallback) => {
          scheduled = callback;
          return 1;
        });
        const release = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        release();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        expect(raf).toHaveBeenCalled();
        scheduled?.(0);
        expect(document.documentElement.classList.contains("dark")).toBe(false);
        raf.mockRestore();
        resetElementsSurfaceChromeForTests();
      });
  
      it("bootstrapElementsSurfaceChromeDocument applies explicit dark before leases exist", () => {
        resetElementsSurfaceChromeForTests();
        bootstrapElementsSurfaceChromeDocument("dark");
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        expect(document.documentElement.dataset.uiAppearance).toBe("dark");
        resetElementsSurfaceChromeForTests();
      });
  
      it("applies touch chrome and the mobile device attribute for the mobile device", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "mobile", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("touch")).toBe(true);
        expect(document.documentElement.dataset.uiDevice).toBe("mobile");
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("clears touch chrome for the desktop device", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("touch")).toBe(false);
        expect(document.documentElement.dataset.uiDevice).toBe("desktop");
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("applies every driver axis attribute and clears them on release", async () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const root = document.documentElement;
        expect(root.dataset.uiDriver).toBe("compact");
        expect(root.dataset.uiLabels).toBe("icons");
        expect(root.dataset.uiDrag).toBe("surface");
        expect(root.dataset.uiChromeReveal).toBe("hover");
        expect(root.dataset.uiGumballReveal).toBe("hover");
        expect(root.dataset.uiTooltips).toBe("none");
        release();
        await new Promise((resolve) => requestAnimationFrame(resolve));
        expect(root.dataset.uiDriver).toBeUndefined();
        expect(root.dataset.uiLabels).toBeUndefined();
        expect(root.dataset.uiDrag).toBeUndefined();
        expect(root.dataset.uiChromeReveal).toBeUndefined();
        expect(root.dataset.uiGumballReveal).toBeUndefined();
        expect(root.dataset.uiTooltips).toBeUndefined();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a hover-reveal chrome region only while the pointer is near its edge band", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const region = document.createElement("nav");
        region.dataset.uiRevealRegion = "navbar";
        region.getBoundingClientRect = () => ({ left: 0, right: 100, top: 200, bottom: 240, width: 100, height: 40 }) as DOMRect;
        document.body.appendChild(region);
        applyChromeRevealAtPoint(document.documentElement, 50, 2);
        expect(region.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 50, 400);
        expect(region.dataset.uiRevealed).toBeUndefined();
        applyChromeRevealAtPoint(document.documentElement, 50, 220);
        expect(region.dataset.uiRevealed).toBe("true");
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a footer hover-reveal region near the bottom screen edge", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const innerHeight = window.innerHeight;
        Object.defineProperty(window, "innerHeight", { value: 800, configurable: true });
        const region = document.createElement("footer");
        region.dataset.uiRevealRegion = "footer";
        region.getBoundingClientRect = () => ({ left: 0, right: 100, top: 760, bottom: 800, width: 100, height: 40 }) as DOMRect;
        document.body.appendChild(region);
        applyChromeRevealAtPoint(document.documentElement, 50, 795);
        expect(region.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 50, 400);
        expect(region.dataset.uiRevealed).toBeUndefined();
        Object.defineProperty(window, "innerHeight", { value: innerHeight, configurable: true });
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a window-cap region when the pointer is near the stack top edge", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const stack = document.createElement("div");
        stack.setAttribute("data-slot", "window-chrome-stack");
        stack.getBoundingClientRect = () => ({ left: 100, right: 400, top: 50, bottom: 350, width: 300, height: 300 }) as DOMRect;
        const cap = document.createElement("div");
        cap.dataset.uiRevealRegion = "window-cap";
        cap.getBoundingClientRect = () => ({ left: 100, right: 400, top: 50, bottom: 50, width: 300, height: 0 }) as DOMRect;
        stack.appendChild(cap);
        document.body.appendChild(stack);
        applyChromeRevealAtPoint(document.documentElement, 250, 55);
        expect(cap.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 250, 100);
        expect(cap.dataset.uiRevealed).toBeUndefined();
        applyChromeRevealAtPoint(document.documentElement, 250, 58);
        expect(cap.dataset.uiRevealed).toBe("true");
        document.body.removeChild(stack);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a hover-reveal region on focus and clears on blur", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const region = document.createElement("nav");
        region.dataset.uiRevealRegion = "navbar";
        const button = document.createElement("button");
        region.appendChild(button);
        document.body.appendChild(region);
        button.dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
        expect(region.dataset.uiRevealed).toBe("true");
        button.dispatchEvent(new FocusEvent("focusout", { bubbles: true, relatedTarget: null }));
        expect(region.dataset.uiRevealed).toBeUndefined();
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("useCanvasAppearanceSync does not re-run sync when the callback identity changes each render", async () => {
        const { render } = await import("@testing-library/react");
        const sync = vi.fn();
        function Harness({ n }: { readonly n: number }) {
          useCanvasAppearanceSync(() => {
            sync(n);
          });
          return <span data-testid="harness">{n}</span>;
        }
        const { rerender } = render(<Harness n={1} />);
        expect(sync).toHaveBeenCalledTimes(1);
        rerender(<Harness n={2} />);
        rerender(<Harness n={3} />);
        expect(sync).toHaveBeenCalledTimes(1);
        document.documentElement.dataset.uiAppearance = "dark";
        await vi.waitFor(() => expect(sync.mock.calls.length).toBeGreaterThan(1));
        delete document.documentElement.dataset.uiAppearance;
      });
    });
}
