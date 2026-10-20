/** 🔶️ The tree helpers phase of registerTreeHelpers4AndMoreTests. */
import { renderToStaticMarkup } from "react-dom/server";
import type { Tests2Vitest, Tests2Dependencies } from "../../🧰️harness/🟦️.ts";

/** 🔷 Registers the tree helpers tests of registerTreeHelpers4AndMoreTests. */
export function registerTreeHelpersTests({ dependencies, vitest }: { readonly dependencies: Tests2Dependencies; readonly vitest: Tests2Vitest }) {
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
  return { describe, it, buildVirtualFileSystemVisibleRows, expect, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, uiDataLabel, buildVirtualFileSystemDescriptorColumns, renderToStaticMarkup, formatVirtualFileSystemTime, VirtualFileSystem, resolveVirtualFileSystemSchemaIcon, vi, getVirtualFileSystemNextSelectionState, SCENE_GIZMO_LABELS, resolveSceneGizmoSnapTarget, THREE, resolveSceneGizmoViewportPlacement, resetElementsSurfaceChromeForTests, applyElementsSurfaceChrome, DEFAULT_UI_DRIVER, COMPACT_UI_DRIVER, bootstrapElementsSurfaceChromeDocument, applyChromeRevealAtPoint, useCanvasAppearanceSync };
}
