/** 🗂️ A treeSection outside a tree root renders as an accessible disclosure wired to the window open state. */
import type { AnyRecord, TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 🗂️ Registers the detached tree section tests. */
export function registerDetachedTreeSectionTests(kit: TreeWindowsTestKit): void {
  const { afterEach, cleanup, describe, expect, fireEvent, it, TREE_WINDOW_PATH_SEPARATOR, node, renderWindowedTree, treeItem } = kit;
  describe("🗂️ detached tree sections", () => {
    afterEach(() => cleanup());
    const nodes = (): AnyRecord[] => [
      node(1, "xlsx-sheets", { type: "treeSection", label: "", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 2, offset: 0 } }, [2, 3]),
      treeItem(2, "xlsx-sheet-0", "Data", { defaultOpen: true, window: { rowExtent: "standard", total: 1, offset: 0 } }, [4, 6]),
      treeItem(3, "xlsx-sheet-1", "Notes", { window: { rowExtent: "standard", total: 1, offset: 0 } }, [5]),
      treeItem(4, "data-row", "Data row"),
      node(6, "xlsx-sheet-0-grid", { type: "container" }, []),
      treeItem(5, "notes-row", "Notes row"),
    ];
    function mountDetached(openStates: Record<string, boolean>, setOpen: (key: string, open: boolean) => void) {
      return renderWindowedTree(nodes(), () => {}, (tree: any) => tree, { openStates, setOpen });
    }
    it("shows each worksheet label as a disclosure and requests the children when toggled", () => {
      const requests: [string, boolean][] = [];
      const view = mountDetached({}, (key, open) => requests.push([key, open]));
      expect(view.container.textContent).toContain("Notes");
      expect(view.container.textContent).toContain("Data");
      const toggle = view.container.querySelector<HTMLElement>('[aria-expanded="false"]');
      expect(toggle).toBeTruthy();
      fireEvent.click(toggle!);
      expect(requests.some(([key, open]) => key.endsWith("xlsx-sheet-1") && open)).toBe(true);
    });
    it("renders the children of a worksheet the host reports open", () => {
      const path = ["xlsx-sheets", "xlsx-sheet-1"].join(TREE_WINDOW_PATH_SEPARATOR);
      const closed = mountDetached({}, () => {});
      expect(closed.container.textContent).not.toContain("Notes row");
      cleanup();
      const open = mountDetached({ [path]: true }, () => {});
      expect(open.container.textContent).toContain("Notes row");
    });
    it("renders non-tree children as block content below the header, not in the trailing control slot", () => {
      const view = mountDetached({}, () => {});
      const grid = view.container.querySelector('[data-ui-node-key="xlsx-sheet-0-grid"]');
      expect(grid).not.toBeNull();
      const row = view.container.querySelector('[role="treeitem"][id$="xlsx-sheet-0"]')!;
      expect(row.querySelector('[data-slot="tree-row-layout"] [data-ui-node-key="xlsx-sheet-0-grid"]')).toBeNull();
    });
  });
}
