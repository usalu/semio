import { useUiTranslation, resolveTranslationLabel } from "../../🏷️Label/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";

/** ♿️ The four directions arrow-key navigation walks a node graph in. */
export type DiagramArrowDirection = "up" | "down" | "left" | "right";

/** ⌨️ Arrow keys mapped to graph directions — the ONE place the mapping lives, so the keyboard handler
 * and its test cannot disagree. */
export const DIAGRAM_ARROW_KEYS: Readonly<Record<string, DiagramArrowDirection>> = { ArrowUp: "up", ArrowDown: "down", ArrowLeft: "left", ArrowRight: "right" };

/** 🧭️ The only node facts keyboard navigation reads: identity and canvas position. */
export type DiagramNavigableNode = { readonly id: string; readonly position: { readonly x: number; readonly y: number } };

/**
 * 🏹️ The node an arrow press moves to: the nearest node in that half-plane, measured with the
 * ALONG-axis distance weighted over the across-axis one, so `ArrowRight` prefers the node to the right
 * over one that is merely further away diagonally. Ties break on id so the walk is deterministic.
 *
 * With no focused node (the first arrow press after the canvas takes focus) the extreme node in the
 * OPPOSITE direction is entered — pressing Right enters at the leftmost node, which is where a reader
 * would expect to start. Returns `null` only when there is nowhere to go.
 *
 * `🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:108-112` records that this surface carries its whole
 * interaction inside a texture no assistive technology can walk, and that its `application` role is the
 * ARIA promise the widget handles its own arrow/Enter keys. This function is that promise.
 */
export function diagramArrowNavigationTarget(nodes: readonly DiagramNavigableNode[], focusedId: string | null | undefined, direction: DiagramArrowDirection): string | null {
  if (nodes.length === 0) return null;
  const horizontal = direction === "left" || direction === "right";
  const sign = direction === "right" || direction === "down" ? 1 : -1;
  const along = (node: DiagramNavigableNode) => (horizontal ? node.position.x : node.position.y);
  const across = (node: DiagramNavigableNode) => (horizontal ? node.position.y : node.position.x);
  const current = focusedId == null ? undefined : nodes.find((node) => node.id === focusedId);
  if (!current) {
    let entry = nodes[0]!;
    for (const node of nodes) {
      const better = sign > 0 ? along(node) < along(entry) : along(node) > along(entry);
      if (better || (along(node) === along(entry) && node.id < entry.id)) entry = node;
    }
    return entry.id;
  }
  let best: DiagramNavigableNode | null = null;
  let bestCost = Number.POSITIVE_INFINITY;
  for (const node of nodes) {
    if (node.id === current.id) continue;
    const forward = (along(node) - along(current)) * sign;
    if (forward <= 0) continue;
    const cost = forward + Math.abs(across(node) - across(current)) * 2;
    if (cost < bestCost || (cost === bestCost && best !== null && node.id < best.id)) {
      best = node;
      bestCost = cost;
    }
  }
  return best?.id ?? null;
}

/** ✅️ The selection an Enter press produces: plain Enter replaces, Shift+Enter toggles membership —
 * the same replace/additive vocabulary `🕹️interaction`'s `MergeMode` names. */
export function diagramActivateSelection(selectedIds: readonly string[], focusedId: string, additive: boolean): string[] {
  if (!additive) return [focusedId];
  return selectedIds.includes(focusedId) ? selectedIds.filter((id) => id !== focusedId) : [...selectedIds, focusedId];
}

/** 🪜️ What ONE key press does to a keyboard-walked node graph — `null` when the key is not the graph's.
 * `select.added` says whether the focused node ended up IN the selection (Shift+Enter can remove it). */
export type DiagramKeyboardStep =
  | { readonly kind: "focus"; readonly focusedId: string }
  | { readonly kind: "select"; readonly focusedId: string; readonly selectedIds: readonly string[]; readonly added: boolean }
  | { readonly kind: "clear" }
  | null;

/**
 * 👣️ The whole keyboard law of every node-graph surface — the React Flow {@link Diagram} and the dag/flow
 * canvas hosts (`🕸️NodeGraph`) run THIS function, so arrows, Enter, Shift+Enter and Escape mean one thing
 * wherever a graph is drawn. Arrows move focus ({@link diagramArrowNavigationTarget}); Enter/Space select
 * the focused node ({@link diagramActivateSelection}); Escape clears. Selection keys are inert on a
 * read-only surface, navigation is not (reading a graph is not editing it).
 */
export function diagramKeyboardStep(nodes: readonly DiagramNavigableNode[], focusedId: string | null, selectedIds: readonly string[], key: string, shift: boolean, editable: boolean): DiagramKeyboardStep {
  const direction = DIAGRAM_ARROW_KEYS[key];
  if (direction) {
    const next = diagramArrowNavigationTarget(nodes, focusedId, direction);
    return next ? { kind: "focus", focusedId: next } : null;
  }
  if (!editable) return null;
  if (key === "Escape") return selectedIds.length === 0 && focusedId === null ? null : { kind: "clear" };
  if ((key !== "Enter" && key !== " ") || focusedId === null || !nodes.some((node) => node.id === focusedId)) return null;
  const next = diagramActivateSelection(selectedIds, focusedId, shift);
  return { kind: "select", focusedId, selectedIds: next, added: next.includes(focusedId) };
}

/** 📖️ A node's 1-based position in reading order (top-to-bottom, then left-to-right, then id) — the
 * "3 of 7" a screen reader speaks, independent of the order the graph happens to store its nodes in. */
export function diagramReadingOrderPosition(nodes: readonly DiagramNavigableNode[], id: string): number {
  const ordered = [...nodes].sort((a, b) => a.position.y - b.position.y || a.position.x - b.position.x || (a.id < b.id ? -1 : a.id > b.id ? 1 : 0));
  return ordered.findIndex((node) => node.id === id) + 1;
}

/** ⭕️ The visible keyboard-focus ring on the focused node — the sighted twin of what the live region
 * speaks. (React Flow reserves a node's DOM `id`, so there is no `aria-activedescendant` target: the
 * {@link DiagramLiveRegion} sentence is the assistive-technology channel.) */
export const DIAGRAM_KEYBOARD_FOCUS_CLASS = "outline-solid outline-2 outline-offset-2 outline-accent";

/** 🔑️ Resolves a translation key with interpolation outside render (keyboard handlers run there). */
export type DiagramTranslate = (key: "ui.diagram.focusedNode" | "ui.diagram.selectedNode" | "ui.diagram.deselectedNode" | "ui.diagram.selectionCleared", options?: Record<string, unknown>) => string;

/** 📢️ The localized sentence a {@link DiagramLiveRegion} speaks for one {@link diagramKeyboardStep}: the
 * focused node's label with its reading-order position, the (de)selection with the new selection size,
 * or "selection cleared". Pure — the locale lives entirely in `translate`. */
export function diagramKeyboardAnnouncement(step: NonNullable<DiagramKeyboardStep>, nodes: readonly DiagramNavigableNode[], labelOf: (id: string) => string, translate: DiagramTranslate): string {
  if (step.kind === "clear") return translate("ui.diagram.selectionCleared");
  if (step.kind === "focus") return translate("ui.diagram.focusedNode", { node: labelOf(step.focusedId), position: diagramReadingOrderPosition(nodes, step.focusedId), count: nodes.length });
  return translate(step.added ? "ui.diagram.selectedNode" : "ui.diagram.deselectedNode", { node: labelOf(step.focusedId), count: step.selectedIds.length });
}

/** 🌐️ {@link DiagramTranslate} bound to the active shell locale and label tier (no default language:
 * whatever the scope's i18n instance stands at is what is spoken). */
export function useDiagramTranslate(): DiagramTranslate {
  const { t } = useUiTranslation();
  return reactHostPort.useCallback<DiagramTranslate>((key, options) => resolveTranslationLabel(t(key, options)) ?? "", [t]);
}

/** 🕸️ The polite live region every node-graph surface announces keyboard steps through. Visually
 * hidden; `aria-atomic` so the whole sentence is re-read when it changes. */
export function DiagramLiveRegion({ text }: { readonly text: string }) {
  return (
    <span role="status" aria-live="polite" aria-atomic="true" className="sr-only" data-diagram-announcement="">
      {text}
    </span>
  );
}
