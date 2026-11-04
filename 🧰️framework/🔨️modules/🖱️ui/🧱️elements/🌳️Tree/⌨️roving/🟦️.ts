/** 🌳️ One visible tree row as the keyboard pattern sees it. */
export type TreeNavRowV1 = Readonly<{ level: number; expanded: boolean | undefined }>;

/** ⌨️ What a key does in the WAI-ARIA tree pattern: move focus to a row, or activate (click) a row to fold, unfold or open it. */
export type TreeNavIntentV1 = Readonly<{ kind: "focus"; index: number }> | Readonly<{ kind: "click"; index: number }>;

export function treeNavigateV1(key: string, rows: readonly TreeNavRowV1[], current: number): TreeNavIntentV1 | null {
  if (rows.length === 0 || current < 0 || current >= rows.length) return null;
  const row = rows[current]!;
  switch (key) {
    case "ArrowDown": return current + 1 < rows.length ? { kind: "focus", index: current + 1 } : null;
    case "ArrowUp": return current > 0 ? { kind: "focus", index: current - 1 } : null;
    case "Home": return { kind: "focus", index: 0 };
    case "End": return { kind: "focus", index: rows.length - 1 };
    case "ArrowRight":
      if (row.expanded === false) return { kind: "click", index: current };
      if (row.expanded === true && current + 1 < rows.length && rows[current + 1]!.level > row.level) return { kind: "focus", index: current + 1 };
      return null;
    case "ArrowLeft": {
      if (row.expanded === true) return { kind: "click", index: current };
      for (let index = current - 1; index >= 0; index--) if (rows[index]!.level < row.level) return { kind: "focus", index };
      return null;
    }
    case "Enter": return { kind: "click", index: current };
    default: return null;
  }
}

const ROW_SELECTOR = '[role="treeitem"]';

/** 🌳️ Visible rows of a tree root in document order: rows inside hidden or inert subtrees are skipped. */
export function visibleTreeRowsV1(root: Element): HTMLElement[] {
  return [...root.querySelectorAll<HTMLElement>(ROW_SELECTOR)].filter((row) => row.closest('[hidden],[inert],[aria-hidden="true"]') === null);
}

function rowOf(element: HTMLElement): TreeNavRowV1 {
  const expanded = element.getAttribute("aria-expanded");
  return { level: Number(element.getAttribute("aria-level") ?? "1") || 1, expanded: expanded === null ? undefined : expanded === "true" };
}

/** 🌳️ Applies the roving tabindex: the active row is the one tab stop (`0`), every other row is `-1`. */
export function applyRovingTabindexV1(root: Element, active: HTMLElement | null): void {
  const rows = visibleTreeRowsV1(root);
  const stop = active !== null && rows.includes(active) ? active : (rows.find((row) => row.getAttribute("aria-selected") === "true") ?? rows[0] ?? null);
  for (const row of rows) {
    const wanted = row === stop ? "0" : "-1";
    if (row.getAttribute("tabindex") !== wanted) row.setAttribute("tabindex", wanted);
  }
}

/** ⌨️ Handles one keydown on a tree root; only keys that land on the row itself count, so controls inside a row keep their own keys. Returns whether it handled the key. */
export function handleTreeKeyDownV1(root: Element, event: Pick<KeyboardEvent, "key" | "target" | "ctrlKey" | "metaKey" | "altKey" | "shiftKey" | "preventDefault">): boolean {
  const target = event.target;
  if (!(target instanceof HTMLElement) || target.getAttribute("role") !== "treeitem" || event.ctrlKey || event.metaKey || event.altKey || event.shiftKey) return false;
  const elements = visibleTreeRowsV1(root);
  const current = elements.indexOf(target);
  const intent = treeNavigateV1(event.key, elements.map(rowOf), current);
  if (intent === null) return false;
  event.preventDefault();
  const element = elements[intent.index]!;
  if (intent.kind === "click") {
    element.click();
    return true;
  }
  applyRovingTabindexV1(root, element);
  element.focus();
  return true;
}
