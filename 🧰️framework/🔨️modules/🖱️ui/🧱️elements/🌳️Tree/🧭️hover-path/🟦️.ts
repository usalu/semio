/** 🧭 Owns the pure DOM resolution and marking of the hovered tree row path shared by the tree and the panel ghost. */

// 🌳️Branch containers that hold child rows and render IndentationLines.
export const treeBranchSlots = new Set(["tree-section-content", "tree-item-content", "tree-property-content", "control-tree-folder-content", "window-measure-tree-content"]);

// 🔷️Row-level elements that own an elbow connector.
export const treeRowSlots = new Set(["tree-item-row", "tree-section-row", "tree-property-item", "tree-row", "control-tree-row", "window-measure-tree-row"]);

const treeHoverPathRowSelector =
  '[data-slot="tree-item-row"], [data-slot="tree-section-row"], [data-slot="tree-property-item"], [data-slot="tree-row"], [data-slot="control-tree-row"], [data-slot="tree-content"], [data-slot="window-measure-tree-row"]';

const treeHoverPathBranchSelector = '[data-slot="tree-section-content"], [data-slot="tree-item-content"], [data-slot="tree-property-content"], [data-slot="control-tree-folder-content"], [data-slot="window-measure-tree-content"]';

/**
 * 📦️Derive the row element that owns a branch container.
 * Handles all DOM shapes: tree-item-row/control-tree-row/tree-property-item siblings (branch below for `down`, above for `up`),
 * tree-section-row beside collapsible-content.
 */
export const rowForBranch = (branch: Element): Element | null => {
  const prev = branch.previousElementSibling;
  if (prev) {
    const prevSlot = prev.getAttribute("data-slot");
    if (prevSlot && treeRowSlots.has(prevSlot)) return prev;
  }
  const next = branch.nextElementSibling;
  if (next) {
    const nextSlot = next.getAttribute("data-slot");
    if (nextSlot && treeRowSlots.has(nextSlot)) return next;
  }
  const parent = branch.parentElement;
  const parentSlot = parent?.getAttribute("data-slot");
  if (parentSlot === "collapsible-content") {
    const previousRow = parent!.previousElementSibling;
    if (previousRow?.getAttribute("data-slot") === "tree-section-row") return previousRow;
    const nextRow = parent!.nextElementSibling;
    if (nextRow?.getAttribute("data-slot") === "tree-section-row") return nextRow;
  }
  return null;
};

/**
 * 🎛️Resolve the conceptual tree row from a pointer target.
 * First tries matching a known row slot via closest(). When no row wrapper
 * exists (pass-through TreeRow, raw controls), falls back to the nearest
 * branch container and returns its owner row.
 */
export const resolveHoverRow = (target: HTMLElement, root: HTMLElement): Element | null => {
  const direct = target.closest(treeHoverPathRowSelector);
  if (direct && root.contains(direct)) return direct;
  const branch = target.closest(treeHoverPathBranchSelector);
  if (branch && root.contains(branch)) return rowForBranch(branch);
  return null;
};

export const markGhostTreeInteraction = (target: Element, region: Element): HTMLElement[] => {
  const row = resolveHoverRow(target as HTMLElement, region as HTMLElement);
  if (!row) return [];

  const marked = new Set<HTMLElement>();
  const mark = (element: Element | null, attribute: "data-active-interaction" | "data-active-ancestor") => {
    if (!(element instanceof HTMLElement)) return;
    element.setAttribute(attribute, "");
    marked.add(element);
  };
  const markRow = (element: Element, active: boolean) => {
    mark(element, active ? "data-active-interaction" : "data-active-ancestor");
    for (const gutter of Array.from(element.querySelectorAll('[data-slot="tree-gutter"]'))) {
      mark(gutter, "data-active-ancestor");
    }
  };

  markRow(row, true);
  let element: Element | null = row.parentElement;
  while (element && element !== region) {
    const slot = element.getAttribute("data-slot");
    if (slot && treeBranchSlots.has(slot)) {
      mark(element, "data-active-ancestor");
      for (const child of Array.from(element.children)) {
        if (child.getAttribute("data-slot") === "tree-guide") mark(child, "data-active-ancestor");
      }
      const ownerRow = rowForBranch(element);
      if (ownerRow) markRow(ownerRow, false);
    }
    element = element.parentElement;
  }
  return [...marked];
};
