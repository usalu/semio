import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerContextmenuAndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { CanvasPickMenu, ContextMenu, ContextMenuController, TextSelectionContextMenuHost, buildTextSelectionContextMenuItems, isContextMenuPointerTarget, isPointerEventOnDomTextSelection, readDomTextSelection, uiDataLabel } = dependencies;
  type CanvasPickTarget = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

  describe("ContextMenu", () => {
    it("prevents the native context menu when no items are registered", () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      const target = screen.getByRole("button", { name: "Target" });
      const event = new MouseEvent("contextmenu", { bubbles: true, cancelable: true });
      const preventDefaultSpy = vi.spyOn(event, "preventDefault");
      target.dispatchEvent(event);
      expect(preventDefaultSpy).toHaveBeenCalled();
      expect(screen.queryByRole("menu")).toBeNull();
    });

    it("buildTextSelectionContextMenuItems exposes copy with a mod shortcut and gates cut on selection", () => {
      const cut = vi.fn();
      const copy = vi.fn();
      const paste = vi.fn();
      const selectAll = vi.fn();
      const items = buildTextSelectionContextMenuItems(
        { editable: true, hasSelection: false },
        { cut: uiDataLabel("Cut"), copy: uiDataLabel("Copy"), paste: uiDataLabel("Paste"), selectAll: uiDataLabel("Select All") },
        { cut, copy, paste, selectAll },
      );
      const copyItem = items.find((item) => item.id === "text-copy");
      const cutItem = items.find((item) => item.id === "text-cut");
      expect(copyItem?.shortcut === "⌘️C" || copyItem?.shortcut === "Ctrl+C").toBe(true);
      expect(cutItem?.disabled).toBe(true);
      copyItem?.onSelect?.(new Event("select"));
      expect(copy).toHaveBeenCalled();
    });

    it("isPointerEventOnDomTextSelection is true only for nodes inside the live selection", () => {
      const host = document.createElement("div");
      host.textContent = "Selectable label";
      document.body.appendChild(host);
      const outside = document.createElement("div");
      outside.textContent = "Outside";
      document.body.appendChild(outside);
      const range = document.createRange();
      range.selectNodeContents(host);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
      expect(readDomTextSelection()).toBe("Selectable label");
      expect(isPointerEventOnDomTextSelection(host.firstChild)).toBe(true);
      expect(isPointerEventOnDomTextSelection(outside)).toBe(false);
      selection?.removeAllRanges();
      host.remove();
      outside.remove();
    });

    it("TextSelectionContextMenuHost opens copy on right-click over selected text", async () => {
      const writeText = vi.fn().mockResolvedValue(undefined);
      Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText, readText: vi.fn() } });
      render(
        <>
          <TextSelectionContextMenuHost />
          <span data-testid="selectable">Copy me please</span>
        </>,
      );
      const node = screen.getByTestId("selectable");
      const range = document.createRange();
      range.selectNodeContents(node);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
      fireEvent.contextMenu(node, { clientX: 40, clientY: 60 });
      const copyItem = await waitFor(() => screen.getByRole("menuitem", { name: "Copy" }));
      expect(copyItem).toBeTruthy();
      expect(screen.getByRole("menuitem", { name: "Select all" })).toBeTruthy();
      expect(screen.queryByRole("menuitem", { name: "Cut" })).toBeNull();
      fireEvent.click(copyItem);
      await waitFor(() => expect(writeText).toHaveBeenCalledWith("Copy me please"));
      selection?.removeAllRanges();
    });

    it("opens the custom menu when items are registered", async () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")} items={[{ id: "demo", label: uiDataLabel("Demo action") }]}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(screen.getByRole("menuitem", { name: "Demo action" })).toBeTruthy();
      });
    });

    it("anchors the menu title chip bottom-left at the pointer coordinates", async () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")} items={[{ id: "demo", label: uiDataLabel("Demo action") }]}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }), { clientX: 123, clientY: 87 });
      const menu = await waitFor(() => screen.getByRole("menu"));
      const chrome = menu.closest<HTMLElement>('[data-window-silhouette][data-slot="context-menu-content"]');
      expect(chrome?.parentElement).toBe(document.body);
      expect(chrome?.style.position).toBe("fixed");
      expect(chrome?.style.left).toBe("123px");
      expect(chrome?.style.top).toBe("calc(87px - var(--size-medium))");
      expect(document.querySelector('[data-slot="context-menu-title-chip"] [data-icon="list"]')).toBeTruthy();
    });

    it("wraps context menu rows in window chrome with a title chip and no enlarge control", async () => {
      render(
        <ContextMenu items={[{ id: "demo", label: uiDataLabel("Demo action") }]} title={uiDataLabel("Actions")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(document.querySelector('[data-slot="context-menu-content"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="context-menu-title-chip"]')?.textContent).toContain("Actions");
        expect(document.querySelector('[data-slot="context-menu-title-chip"] [data-icon="list"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="mode-dock-maximize"]')).toBeNull();
        expect(document.querySelector('[data-slot="window-chrome-controls"]')).toBeNull();
      });
    });

    it("keeps the U-gap transparent so the cutout shows the background instead of a filled rectangle", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
      expect(css).toContain("[data-window-silhouette-gap]");
      expect(css).toMatch(/\[data-window-silhouette-gap\][\s\S]*backdrop-filter:\s*none/);
      render(
        <ContextMenu items={[{ id: "demo", label: uiDataLabel("Demo action") }]} title={uiDataLabel("Actions")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        const stack = document.querySelector('[data-slot="context-menu-content"]') as HTMLElement;
        const gap = stack?.querySelector('[data-slot="window-chrome-gap"]') as HTMLElement;
        const chip = stack?.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
        expect(gap).toBeTruthy();
        expect(gap.className).toContain("bg-transparent");
        expect(gap.className).not.toContain("ui-glass");
        expect(stack.querySelector('[class*="ui-glass"][class*="absolute"][class*="inset-0"]')).toBeNull();
        expect(chip?.className).toContain("ui-glass");
        const titleChip = stack?.querySelector('[data-slot="context-menu-title-chip"]') as HTMLElement;
        expect(titleChip?.className).not.toContain("ui-surface");
      });
    });

    it("renders catalog and shortcode menu icons instead of raw labels", async () => {
      render(
        <ContextMenu
          title={uiDataLabel("Menu")}
          items={[
            { id: "delete", label: uiDataLabel("Delete"), icon: "trash" },
            { id: "suggest", label: uiDataLabel("Suggest"), icon: "sparkles" },
            { id: "copy", label: uiDataLabel("Copy"), icon: "copy" },
          ]}
        >
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(screen.getByRole("menuitem", { name: "Delete" }).querySelector("[data-icon='trash-2'] svg")).toBeTruthy();
        expect(screen.getByRole("menuitem", { name: "Suggest" }).querySelector("[data-icon='sparkles'] svg")).toBeTruthy();
        expect(screen.getByRole("menuitem", { name: "Copy" }).querySelector("[data-icon='copy'] svg")).toBeTruthy();
      });
      expect(screen.queryByText("trash")).toBeNull();
      expect(screen.queryByText("sparkles")).toBeNull();
    });

    it("renders a color swatch and fires hover callbacks on controller items", async () => {
      const onHover = vi.fn();
      const onHoverEnd = vi.fn();
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 12, y: 24 }} items={[{ id: "kind", label: uiDataLabel("Capsule"), icon: "box", color: "#aabbcc", onHover, onHoverEnd }]} onOpenChange={vi.fn()} />);
      const item = await waitFor(() => screen.getByRole("menuitem", { name: "Capsule" }));
      const swatch = Array.from(item.querySelectorAll("[aria-hidden]")).find((node) => {
        const style = (node as HTMLElement).getAttribute("style") ?? "";
        return style.includes("background");
      }) as HTMLElement | undefined;
      expect(swatch?.getAttribute("style") ?? "").toMatch(/#aabbcc|rgb\(170,\s*187,\s*204\)/);
      fireEvent.pointerEnter(item);
      expect(onHover).toHaveBeenCalled();
      fireEvent.pointerLeave(item);
      expect(onHoverEnd).toHaveBeenCalled();
    });

    it("highlights checked items without a tick or checkmark", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 8, y: 16 }}
          items={[
            { id: "a", label: uiDataLabel("Capsule · port"), icon: "box", color: "#112233", checked: true },
            { id: "b", label: uiDataLabel("Box · port"), icon: "box", color: "#445566", checked: false },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const active = await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · port" }));
      const idle = screen.getByRole("menuitemcheckbox", { name: "Box · port" });
      expect(active.getAttribute("aria-checked")).toBe("true");
      expect(active.getAttribute("data-selected")).toBe("true");
      expect(active.className.split(/\s+/)).toContain("bg-active-base");
      expect(active.className.split(/\s+/)).toContain("hover:bg-active-base/90");
      expect(active.textContent).not.toContain("✓️");
      expect(idle.getAttribute("aria-checked")).toBe("false");
      expect(idle.getAttribute("data-selected")).toBeNull();
      expect(idle.className.split(/\s+/)).not.toContain("bg-active-base");
      expect(idle.textContent).not.toContain("✓️");
    });

    it("omits the color swatch when suggestion-style rows have icon only", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 8, y: 16 }}
          items={[
            { id: "suggestion-0", label: uiDataLabel("Capsule · vortex 0"), icon: "box", checked: true },
            { id: "suggestion-1", label: uiDataLabel("Box · vortex 1"), icon: "box", checked: false },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const active = await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · vortex 0" }));
      const idle = screen.getByRole("menuitemcheckbox", { name: "Box · vortex 1" });
      const swatch = (item: HTMLElement) =>
        Array.from(item.querySelectorAll("[aria-hidden]")).find((node) => {
          const style = (node as HTMLElement).getAttribute("style") ?? "";
          return style.includes("background") || /#|rgb\(/i.test(style);
        });
      expect(swatch(active)).toBeUndefined();
      expect(swatch(idle)).toBeUndefined();
      expect(active.querySelector("[data-icon]")).toBeTruthy();
      expect(active.className.split(/\s+/)).toContain("gap-single");
      expect(active.className.split(/\s+/)).toContain("px-single");
      expect(active.className.split(/\s+/)).toContain("py-half");
    });

    it("ignores outside dismiss when pointerdown targets a sibling menu", async () => {
      const onOpenChange = vi.fn();
      render(
        <>
          <ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 10, y: 10 }} items={[{ id: "a", label: uiDataLabel("Alpha"), onSelect: vi.fn() }]} onOpenChange={onOpenChange} />
          <div role="menu" data-testid="sibling-menu">
            <button type="button">Sibling</button>
          </div>
        </>,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: "Alpha" }));
      await new Promise((resolve) => setTimeout(resolve, 0));
      fireEvent.pointerDown(screen.getByTestId("sibling-menu"));
      expect(onOpenChange).not.toHaveBeenCalled();
      fireEvent.pointerDown(document.body);
      expect(onOpenChange).toHaveBeenCalledWith(false);
    });

    it("previews numbered rows on digit keys and accepts the active row on Enter", async () => {
      const onHover = vi.fn();
      const onSelect = vi.fn();
      const onOpenChange = vi.fn();
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          closeOnSelect={false}
          position={{ x: 8, y: 16 }}
          items={[
            { id: "suggestion-0", label: uiDataLabel("Capsule · port"), icon: "box", checked: true, onSelect, onHover },
            { id: "suggestion-1", label: uiDataLabel("Box · port"), icon: "box", checked: false, onSelect, onHover },
          ]}
          onOpenChange={onOpenChange}
        />,
      );
      await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · port" }));
      fireEvent.keyDown(window, { key: "2" });
      expect(onHover).toHaveBeenCalledTimes(1);
      expect(onSelect).not.toHaveBeenCalled();
      fireEvent.keyDown(window, { key: "Enter" });
      expect(onSelect).toHaveBeenCalledTimes(1);
      expect(onOpenChange).not.toHaveBeenCalled();
    });

    it("renders ordinal badges for the first nine enabled rows", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "a", label: uiDataLabel("Alpha") },
            { id: "b", label: uiDataLabel("Beta") },
            { id: "sep", label: uiDataLabel(""), separator: true },
            { id: "c", label: uiDataLabel("Gamma") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const alpha = await waitFor(() => screen.getByRole("menuitem", { name: "Alpha" }));
      expect(alpha.textContent?.trim().startsWith("1")).toBe(true);
      expect(screen.getByRole("menuitem", { name: "Beta" }).textContent?.trim().startsWith("2")).toBe(true);
      expect(screen.getByRole("menuitem", { name: "Gamma" }).textContent?.trim().startsWith("3")).toBe(true);
    });

    it("moves the active row with arrow keys and wasd", async () => {
      const onHover = vi.fn();
      const onHoverEnd = vi.fn();
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "a", label: uiDataLabel("Alpha"), onHover, onHoverEnd },
            { id: "b", label: uiDataLabel("Beta"), onHover, onHoverEnd },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: /Alpha/ }));
      fireEvent.keyDown(window, { key: "ArrowDown" });
      expect(onHover).toHaveBeenCalledTimes(1);
      expect(screen.getByRole("menuitem", { name: /Alpha/ }).getAttribute("data-active")).toBe("true");
      fireEvent.keyDown(window, { key: "ArrowDown" });
      expect(screen.getByRole("menuitem", { name: /Beta/ }).getAttribute("data-active")).toBe("true");
      fireEvent.keyDown(window, { key: "w" });
      expect(onHoverEnd).toHaveBeenCalled();
      expect(screen.getByRole("menuitem", { name: /Alpha/ }).getAttribute("data-active")).toBe("true");
    });

    it("opens nested submenus with ArrowRight and closes with ArrowLeft", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            {
              id: "transform",
              label: uiDataLabel("Transform"),
              children: [
                { id: "move", label: uiDataLabel("Move") },
                { id: "rotate", label: uiDataLabel("Rotate") },
              ],
            },
            { id: "delete", label: uiDataLabel("Delete") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      fireEvent.keyDown(window, { key: "ArrowDown" });
      fireEvent.keyDown(window, { key: "ArrowRight" });
      await waitFor(() => screen.getByRole("menuitem", { name: "Move" }));
      fireEvent.keyDown(window, { key: "ArrowLeft" });
      await waitFor(() => expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull());
    });

    it("does not close on select when closeOnSelect is false", async () => {
      const onOpenChange = vi.fn();
      const onSelect = vi.fn();
      render(<ContextMenuController title={uiDataLabel("Menu")} open closeOnSelect={false} position={{ x: 4, y: 4 }} items={[{ id: "place", label: uiDataLabel("Place"), onSelect }]} onOpenChange={onOpenChange} />);
      fireEvent.click(await waitFor(() => screen.getByRole("menuitem", { name: "Place" })));
      expect(onSelect).toHaveBeenCalled();
      expect(onOpenChange).not.toHaveBeenCalled();
    });

    it("isContextMenuPointerTarget detects menu surfaces", () => {
      const menu = document.createElement("div");
      menu.setAttribute("role", "menu");
      const child = document.createElement("button");
      menu.appendChild(child);
      document.body.appendChild(menu);
      expect(isContextMenuPointerTarget(child)).toBe(true);
      expect(isContextMenuPointerTarget(document.body)).toBe(false);
      menu.remove();
    });

    it("renders a non-interactive header row for a labeled separator, leaving a bare separator unlabeled", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "recent-header", label: uiDataLabel("Recent"), separator: true },
            { id: "a", label: uiDataLabel("Alpha") },
            { id: "sep", label: uiDataLabel(""), separator: true },
            { id: "b", label: uiDataLabel("Beta") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const header = await waitFor(() => screen.getByText("Recent"));
      expect(header.getAttribute("role")).toBe("separator");
      expect(header.getAttribute("aria-label")).toBe("Recent");
      expect(header.tagName).not.toBe("BUTTON");
      const bareSeparators = document.querySelectorAll('[role="separator"]:not([aria-label])');
      expect(bareSeparators.length).toBe(1);
    });

    it("toggles a parent row's submenu open and closed on click", async () => {
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 4, y: 4 }} items={[{ id: "transform", label: uiDataLabel("Transform"), children: [{ id: "move", label: uiDataLabel("Move") }] }]} onOpenChange={vi.fn()} />);
      const parent = await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull();
      fireEvent.click(parent);
      await waitFor(() => expect(screen.getByRole("menuitem", { name: "Move" })).toBeTruthy());
      fireEvent.click(parent);
      await waitFor(() => expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull());
    });

    it("renders a parent row's shortcut before the submenu chevron", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[{ id: "transform", label: uiDataLabel("Transform"), shortcut: "⌘️T", children: [{ id: "move", label: uiDataLabel("Move") }] }]}
          onOpenChange={vi.fn()}
        />,
      );
      const parent = await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      expect(parent.textContent).toContain("⌘️T");
    });

    it("caps the menu surface height with a scrolling overflow class", async () => {
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 4, y: 4 }} items={[{ id: "a", label: uiDataLabel("Alpha") }]} onOpenChange={vi.fn()} />);
      const menu = await waitFor(() => screen.getByRole("menu"));
      const chrome = menu.closest<HTMLElement>('[data-slot="context-menu-content"]');
      expect(chrome?.className).toContain("max-h-layout-command");
      expect(chrome?.className).toContain("overflow-y-auto");
    });
  });

  describe("CanvasPickMenu", () => {
    const targets = [
      { domain: "group", id: "g1", generality: 0, label: "Group 1" },
      { domain: "path", id: "p1", generality: 2, label: "Path 1" },
    ] satisfies readonly CanvasPickTarget[];

    it("renders general-first rows and highlights hovered item", async () => {
      const onHoverKey = vi.fn();
      render(<CanvasPickMenu request={{ targets, client: { x: 10, y: 20 } }} hoveredKey="path:p1" onHoverKey={onHoverKey} onPick={vi.fn()} onDismiss={vi.fn()} />);
      const items = await waitFor(() => screen.getAllByRole("menuitem"));
      expect(items[0]?.textContent).toContain("Group 1");
      expect(items[1]?.getAttribute("aria-selected")).toBe("true");
    });

    it("dismisses on outside pointerdown", async () => {
      const onDismiss = vi.fn();
      render(<CanvasPickMenu request={{ targets, client: { x: 10, y: 20 } }} hoveredKey={null} onHoverKey={vi.fn()} onPick={vi.fn()} onDismiss={onDismiss} />);
      await waitFor(() => expect(screen.getAllByRole("menuitem").length).toBe(2));
      window.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
      expect(onDismiss).toHaveBeenCalled();
    });
  });
}
