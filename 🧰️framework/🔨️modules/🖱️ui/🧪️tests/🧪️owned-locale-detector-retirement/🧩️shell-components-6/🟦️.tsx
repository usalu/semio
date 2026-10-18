import type { WindowLayoutNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents6Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { COMPOSE_WINDOW_TEMPLATE_MIME, Engagement, Mode, Search, applyModeDrop, beginWindowTemplateDrag, beginWindowTemplatePointerDrag, cancelWindowTemplatePointerDrag, computeModeDropZone, endWindowTemplateDrag, filterSearchPossibles, insertWindowAsTabAtCorner, insertWindowAtDropZone, isSearchSuggestionActionTarget, modeCollectWindowIds, modeDockOutLayout, modeStackTabsByCorner, readActiveWindowTemplateDragSession, reconcileWindows, removeWindowFromLayout, resolveTranslationLabel, searchActiveInlineCompletion, searchControlledLineV1, searchInlineCompletion, shouldActivateSearchPossibleOnConfirm, splitWithWindow, uiDataLabel, uiI18n, windowTemplatePaletteTreeDragController, windowTemplatePointerDragRef } = dependencies;
  type WindowLayoutNode = any;
  const { describe, expect, it } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

describe("Shell components", () => {

    it("computeModeDropZone treats tab bar hits as tab drops not body splits", () => {
      const tabBar = { left: 0, top: 0, right: 200, bottom: 24, width: 200, height: 24 } as DOMRect;
      const body = { left: 0, top: 24, right: 200, bottom: 224, width: 200, height: 200 } as DOMRect;
      const targets = new Map([
        [
          "1",
          {
            corners: { topLeft: { rect: tabBar, element: null as unknown as HTMLElement } },
            body,
          },
        ],
      ]);
      expect(computeModeDropZone(100, 12, targets, null)).toEqual({ kind: "tab", stackPath: "1", corner: "topLeft", index: 0 });
      expect(computeModeDropZone(100, 30, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "top" });
      expect(computeModeDropZone(100, 200, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "bottom" });
      expect(computeModeDropZone(50, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "left" });
      expect(computeModeDropZone(150, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "right" });
    });
    it("modeStackTabsByCorner groups windows by corner defaulting to topLeft", () => {
      const groups = modeStackTabsByCorner([
        { kind: "window", id: "a" },
        { kind: "window", id: "b", corner: "topRight" },
        { kind: "window", id: "c", corner: "bottomLeft" },
        { kind: "window", id: "d", corner: "bottomRight" },
        { kind: "window", id: "e", corner: "topLeft" },
      ]);
      expect(groups.topLeft.map((child) => child.id)).toEqual(["a", "e"]);
      expect(groups.topRight.map((child) => child.id)).toEqual(["b"]);
      expect(groups.bottomLeft.map((child) => child.id)).toEqual(["c"]);
      expect(groups.bottomRight.map((child) => child.id)).toEqual(["d"]);
    });

    it("applyModeDrop moves a tab to another corner of the same stack", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        activeId: "a",
        children: [
          { kind: "window", id: "a", corner: "topLeft" },
          { kind: "window", id: "b", corner: "topLeft" },
        ],
      };
      const drag = {
        dragKind: "tab" as const,
        windowId: "b",
        stackPath: "",
        tabIndex: 1,
        pointerId: 1,
        ghostLabel: "B",
        x: 0,
        y: 0,
      };
      const next = applyModeDrop(layout, drag, { kind: "tab", stackPath: "", corner: "bottomRight", index: 0 });
      expect(next.kind).toBe("stack");
      if (next.kind !== "stack") return;
      expect(next.activeId).toBe("b");
      expect(next.children.map((child) => ({ id: child.id, corner: child.corner }))).toEqual([
        { id: "a", corner: "topLeft" },
        { id: "b", corner: "bottomRight" },
      ]);
    });

    it("insertWindowAsTabAtCorner preserves one activeId across corners", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        activeId: "a",
        children: [{ kind: "window", id: "a", corner: "topLeft" }],
      };
      const next = insertWindowAsTabAtCorner(layout, "", "b", "topRight");
      expect(next.kind).toBe("stack");
      if (next.kind !== "stack") return;
      expect(next.activeId).toBe("b");
      expect(modeStackTabsByCorner(next.children).topRight.map((child) => child.id)).toEqual(["b"]);
      expect(modeStackTabsByCorner(next.children).topLeft.map((child) => child.id)).toEqual(["a"]);
    });

    it("computeModeDropZone resolves distinct corner tab bars", () => {
      const topLeft = { left: 0, top: 0, right: 80, bottom: 24, width: 80, height: 24 } as DOMRect;
      const topRight = { left: 120, top: 0, right: 200, bottom: 24, width: 80, height: 24 } as DOMRect;
      const body = { left: 0, top: 24, right: 200, bottom: 224, width: 200, height: 200 } as DOMRect;
      const targets = new Map([
        [
          "0",
          {
            corners: {
              topLeft: { rect: topLeft, element: null as unknown as HTMLElement },
              topRight: { rect: topRight, element: null as unknown as HTMLElement },
            },
            body,
          },
        ],
      ]);
      expect(computeModeDropZone(40, 12, targets, null)).toEqual({ kind: "tab", stackPath: "0", corner: "topLeft", index: 0 });
      expect(computeModeDropZone(160, 12, targets, null)).toEqual({ kind: "tab", stackPath: "0", corner: "topRight", index: 0 });
    });

    it("beginWindowTemplateDrag records a session for mode dock preview", () => {
      beginWindowTemplateDrag({ payload: { windowKindId: "main", templateId: "top" }, label: "Top" });
      expect(readActiveWindowTemplateDragSession()?.label).toBe("Top");
      endWindowTemplateDrag();
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("windowTemplatePaletteTreeDragController starts pointer and native template drags", () => {
      const controller = windowTemplatePaletteTreeDragController();
      const encoded = JSON.stringify({ windowKindId: "puzzle-3d-main", templateId: "perspective" });
      controller.pointerPaletteDrag?.begin(encoded, [COMPOSE_WINDOW_TEMPLATE_MIME]);
      expect(windowTemplatePointerDragRef.active).toBe(true);
      controller.onDragStart?.({
        items: [],
        sourceItem: {
          id: "framework.display.windows.puzzle-3d-main.perspective",
          label: "Perspective",
          dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: encoded },
        },
        section: { id: "framework.display.windows.puzzle-3d-main", items: [] },
      });
      expect(readActiveWindowTemplateDragSession()?.label).toBe("Perspective");
      cancelWindowTemplatePointerDrag();
      expect(windowTemplatePointerDragRef.active).toBe(false);
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("cancelWindowTemplatePointerDrag clears an active template drag session", () => {
      beginWindowTemplatePointerDrag(JSON.stringify({ windowKindId: "gis-map-main" }));
      beginWindowTemplateDrag({ payload: { windowKindId: "gis-map-main" }, label: "Map" });
      cancelWindowTemplatePointerDrag();
      expect(windowTemplatePointerDragRef.active).toBe(false);
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("reconcileWindows drops closed windows instead of re-adding them", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [
          { kind: "window", id: "a" },
          { kind: "window", id: "b" },
        ],
        activeId: "a",
      };
      const next = reconcileWindows(layout, ["a"]);
      expect(modeCollectWindowIds(next)).toEqual(["a"]);
    });

    it("resolveTranslationLabel exposes the empty shell notice", () => {
      const label = resolveTranslationLabel(uiI18n.t("ui.display.emptyShell"));
      expect(label).toBeTruthy();
      expect(label).toContain("Display");
    });

    it("insertWindowAtDropZone adds a window on root-split", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [{ kind: "window", id: "a" }],
      };
      const next = insertWindowAtDropZone(layout, "b", { kind: "root-split", side: "right" });
      expect(next.kind).toBe("row");
      const ids = next.kind === "row" ? next.children.flatMap((child) => (child.kind === "stack" ? child.children.map((leaf) => leaf.id) : [])) : [];
      expect(ids.sort()).toEqual(["a", "b"]);
    });

    it("computeModeDropZone root-split uses half of the mode when pointer is outside stack bodies", () => {
      const modeRect = { left: 0, top: 0, right: 400, bottom: 300, width: 400, height: 300 } as DOMRect;
      expect(computeModeDropZone(80, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "left" });
      expect(computeModeDropZone(320, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "right" });
      expect(computeModeDropZone(200, 40, new Map(), modeRect)).toEqual({ kind: "root-split", side: "top" });
      expect(computeModeDropZone(200, 260, new Map(), modeRect)).toEqual({ kind: "root-split", side: "bottom" });
    });

    it("modeDockOutLayout removes the dragged window without mutating drop targets", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "stack",
            children: [
              { kind: "window", id: "a" },
              { kind: "window", id: "b" },
            ],
            activeId: "a",
          },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const dockedOut = modeDockOutLayout(layout, { dragKind: "tab", windowId: "b", stackPath: "0" });
      expect(modeCollectWindowIds(dockedOut)).toEqual(["a", "c"]);
      expect(modeCollectWindowIds(layout)).toEqual(["a", "b", "c"]);
    });

    it("removeWindowFromLayout and splitWithWindow mutate the layout tree", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "stack",
            children: [
              { kind: "window", id: "a" },
              { kind: "window", id: "b" },
            ],
            activeId: "a",
          },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const removed = removeWindowFromLayout(layout, "b");
      expect(removed?.kind).toBe("row");
      const split = splitWithWindow(layout, "1", "b", "left");
      expect(split.kind).toBe("row");
      if (split.kind === "row") {
        const target = split.children[1];
        expect(target?.kind === "row" || target?.kind === "column").toBe(true);
      }
    });

    it("modeDockOutLayout removes an entire stack for stack drag", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "stack",
            children: [
              { kind: "window", id: "a" },
              { kind: "window", id: "b" },
            ],
            activeId: "a",
          },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const dockedOut = modeDockOutLayout(layout, { dragKind: "stack", windowId: "a", stackPath: "0" });
      expect(modeCollectWindowIds(dockedOut)).toEqual(["c"]);
      expect(modeCollectWindowIds(layout)).toEqual(["a", "b", "c"]);
    });

    it("applyModeDrop relocates a stack when dropping on another stack body edge", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "stack",
            children: [
              { kind: "window", id: "a" },
              { kind: "window", id: "b" },
            ],
            activeId: "a",
          },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const drag = {
        dragKind: "stack" as const,
        windowId: "a",
        stackPath: "0",
        tabIndex: -1,
        pointerId: 1,
        ghostLabel: "Stack",
        x: 0,
        y: 0,
      };
      const next = applyModeDrop(layout, drag, { kind: "split", stackPath: "1", side: "left" });
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      expect(next.children).toHaveLength(2);
      const left = next.children[0];
      const right = next.children[1];
      expect(left?.kind).toBe("stack");
      expect(right?.kind).toBe("stack");
      if (left?.kind === "stack") expect(left.children.map((child) => child.id)).toEqual(["a", "b"]);
      if (right?.kind === "stack") expect(right.children.map((child) => child.id)).toEqual(["c"]);
    });

    it("applyModeDrop splits within the same stack when dropping on a body edge zone", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [
          { kind: "window", id: "a" },
          { kind: "window", id: "b" },
        ],
        activeId: "a",
      };
      const drag = {
        dragKind: "tab" as const,
        windowId: "a",
        stackPath: "",
        tabIndex: 0,
        pointerId: 1,
        ghostLabel: "A",
        x: 0,
        y: 0,
      };
      const zone = { kind: "split" as const, stackPath: "", side: "right" as const };
      const next = applyModeDrop(layout, drag, zone);
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      expect(next.children).toHaveLength(2);
      const leftStack = next.children[0];
      const rightStack = next.children[1];
      expect(leftStack?.kind).toBe("stack");
      expect(rightStack?.kind).toBe("stack");
      if (leftStack?.kind === "stack") expect(leftStack.children.map((c) => c.id)).toEqual(["b"]);
      if (rightStack?.kind === "stack") expect(rightStack.children.map((c) => c.id)).toEqual(["a"]);
    });

    it("Mode maximize shows only one stack", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "a", title: uiDataLabel("A"), iconId: "app-window", children: <div>A Body</div> },
              { id: "b", title: uiDataLabel("B"), iconId: "app-window", children: <div>B Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
                { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
              ],
            }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      fireEvent.click(container.querySelector("[data-stack-path='0'] [data-slot='mode-dock-tab-focus']")!);
      expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe("0");
      expect(screen.getByText("A Body")).toBeTruthy();
      expect(screen.queryByText("B Body")).toBeNull();
    });

    it("Mode hides the Focus enlarge control when only one window is on the canvas", () => {
      const { container, rerender } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode windows={[{ id: "solo", title: uiDataLabel("Solo"), iconId: "app-window", children: <div>Solo Body</div> }]} activeWindowId="solo" onActiveWindowChange={() => {}} />
        </div>,
      );
      expect(container.querySelector('[data-slot="mode-dock-tab-focus"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-close"]')).toBeTruthy();
      rerender(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "solo", title: uiDataLabel("Solo"), iconId: "app-window", children: <div>Solo Body</div> },
              { id: "peer", title: uiDataLabel("Peer"), iconId: "app-window", children: <div>Peer Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "solo" }], activeId: "solo" },
                { kind: "stack", children: [{ kind: "window", id: "peer" }], activeId: "peer" },
              ],
            }}
            activeWindowId="solo"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(container.querySelectorAll('[data-slot="mode-dock-tab-focus"]').length).toBeGreaterThan(0);
    });

    it("Engagement renders options and status lines; Search renders the input", () => {
      const { container } = render(
        <>
          <Engagement options={[{ id: "opt-a", label: uiDataLabel("Option A"), icon: "circle-dot", onPress: () => {} }]} status={[{ id: "status-a", content: "Ready" }]} />
          <Search input={{ placeholder: uiDataLabel("Type here") }} />
        </>,
      );
      expect(screen.getByRole("button", { name: "OptionA" })).toBeTruthy();
      expect(screen.getByPlaceholderText("Type here")).toBeTruthy();
      expect(screen.getByText("Ready")).toBeTruthy();
      expect(container.querySelector('[data-slot="engagement"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="search"]')).toBeTruthy();
    });

    it("Engagement option buttons size to label text without clipping", () => {
      const longLabel = uiDataLabel("C Confirm selection");
      const { container } = render(<Engagement options={[{ id: "engagement-transition-confirm-c", label: longLabel, icon: "circle-dot", onPress: () => {} }]} />);
      const item = container.querySelector('[data-slot="button-group-item"]') as HTMLElement;
      expect(item?.textContent).toContain("CConfirmSelection");
      expect(item?.className).toContain("aspect-auto");
      expect(item?.className).not.toContain("aspect-square");
    });

    it("Search focuses its input when active", async () => {
      const { rerender } = render(<Search active={false} input={{ id: "search-input", placeholder: uiDataLabel("Action") }} />);
      const field = () => screen.getByPlaceholderText("Action") as HTMLInputElement;
      expect(document.activeElement).not.toBe(field());
      rerender(<Search active input={{ id: "search-input", placeholder: uiDataLabel("Action") }} />);
      await waitFor(() => expect(document.activeElement).toBe(field()));
      expect(field().tabIndex).toBe(0);
    });

    it("Search input is removed from tab order when inactive", () => {
      render(<Search active={false} input={{ placeholder: uiDataLabel("Action") }} />);
      expect((screen.getByPlaceholderText("Action") as HTMLInputElement).tabIndex).toBe(-1);
    });

    it("filterSearchPossibles matches label, detail, and id", () => {
      const items = [
        { id: "primitive.box", label: "Box", detail: "b" },
        { id: "primitive.sphere", label: "Sphere", detail: "s" },
      ];
      expect(filterSearchPossibles("", items)).toHaveLength(2);
      expect(filterSearchPossibles("sph", items).map((row) => row.id)).toEqual(["primitive.sphere"]);
    });

    it("filterSearchPossibles ranks shorter label prefix matches first", () => {
      const items = [
        { id: "feature.extrudeWire", label: "ExtrudeWire", detail: "e" },
        { id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e" },
      ];
      expect(filterSearchPossibles("Extr", items).map((row) => row.id)).toEqual(["surface.extrudeCrv", "feature.extrudeWire"]);
    });

    it("isSearchSuggestionActionTarget accepts text nodes inside action rows", () => {
      const row = document.createElement("div");
      row.setAttribute("data-slot", "command-item");
      const label = document.createTextNode("ExtrudeCrv");
      row.appendChild(label);
      document.body.appendChild(row);
      expect(isSearchSuggestionActionTarget({ target: label })).toBe(true);
      row.remove();
    });

    it("searchInlineCompletion uses label casing for matched name prefix", () => {
      const box = { id: "primitive.box", label: "Box", detail: "b" };
      const sphere = { id: "primitive.sphere", label: "Sphere", detail: "s" };
      expect(searchInlineCompletion("b", box)).toEqual({ prefix: "B", suffix: "ox" });
      expect(searchInlineCompletion("Sp", sphere)).toEqual({ prefix: "Sp", suffix: "here" });
      expect(searchInlineCompletion("sph", sphere)).toEqual({ prefix: "Sph", suffix: "ere" });
      expect(searchActiveInlineCompletion("Sp", [sphere], 0)).toEqual({ prefix: "Sp", suffix: "here" });
    });

    it("shouldActivateSearchPossibleOnConfirm accepts typed draft or expanded list", () => {
      expect(shouldActivateSearchPossibleOnConfirm("", false, 2)).toBe(false);
      expect(shouldActivateSearchPossibleOnConfirm("", true, 2)).toBe(true);
      expect(shouldActivateSearchPossibleOnConfirm("f", false, 2)).toBe(true);
      expect(shouldActivateSearchPossibleOnConfirm("f", false, 0)).toBe(false);
    });

    it("a controlled search line keeps the typed draft while the program's published value stands still", async () => {
      expect(searchControlledLineV1("", null)).toBe("");
      expect(searchControlledLineV1("", { text: "Brush", base: "" })).toBe("Brush");
      expect(searchControlledLineV1("Fill3", { text: "Brush", base: "" })).toBe("Fill3");
      expect(searchControlledLineV1("Brush", { text: "Brush", base: "Brush" })).toBe("Brush");
      // 🔁️ A program that echoes each keystroke one round trip behind publishes lines this edit already
      // sent — an echo of "Pl" while the draft reads "PlaceCol" is not the program authoring the line.
      expect(searchControlledLineV1("Pl", { text: "PlaceCol", base: "", sent: ["P", "Pl", "Pla", "Plac", "Place", "PlaceC", "PlaceCo", "PlaceCol"] })).toBe("PlaceCol");
      expect(searchControlledLineV1("Place", { text: "Pla", base: "", sent: ["P", "Pl", "Pla", "Plac", "Place"] })).toBe("Pla");
      expect(searchControlledLineV1("SetHeight3", { text: "PlaceCol", base: "", sent: ["P", "Pl"] })).toBe("SetHeight3");

      const sent: string[] = [];
      const submitted: string[] = [];
      // 🗣️ The puzzle3d window engagement: the guest stores the line and does NOT republish it, so
      // `value` stays "" across the whole exchange. Before this rule every keystroke snapped the field
      // back to "" and `onSubmit` carried an empty verb.
      const { rerender } = render(<Search active input={{ id: "line", placeholder: uiDataLabel("brush, fill <n>"), value: "", onChange: (text: string) => sent.push(text), onSubmit: (text: string) => submitted.push(text) }} />);
      const field = screen.getByPlaceholderText("brush, fill <n>") as HTMLInputElement;
      fireEvent.change(field, { target: { value: "brush" } });
      rerender(<Search active input={{ id: "line", placeholder: uiDataLabel("brush, fill <n>"), value: "", onChange: (text: string) => sent.push(text), onSubmit: (text: string) => submitted.push(text) }} />);
      // ⌨️ The line reaches the program verbatim; the guest's `strip_engagement_prefix` matches
      // alphanumerics case-insensitively, so `brush` is the verb.
      await waitFor(() => expect((screen.getByPlaceholderText("brush, fill <n>") as HTMLInputElement).value).toBe("brush"));
      expect(sent).toEqual(["brush"]);
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(submitted).toEqual(["brush"]));
      // 🧹️ The confirm RELEASES the line — the guest's own `engagement_submit` empties
      // `engagement_input`, and its published "" leads again from here.
      await waitFor(() => expect((screen.getByPlaceholderText("brush, fill <n>") as HTMLInputElement).value).toBe(""));
      // 📢️ A program that DOES author the line wins it back: a published value other than the one that
      // stood when the edit began replaces the draft.
      rerender(<Search active input={{ id: "line", placeholder: uiDataLabel("brush, fill <n>"), value: "fill 3", onChange: (text: string) => sent.push(text), onSubmit: (text: string) => submitted.push(text) }} />);
      await waitFor(() => expect((screen.getByPlaceholderText("brush, fill <n>") as HTMLInputElement).value).toBe("fill 3"));
    });
});
}
