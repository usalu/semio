import type { CanvasHoverFocus, CanvasPickTarget } from "../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";

/** 🎯️ Stable identity of one canvas pick target: its domain and id. */
export function canvasPickTargetKey(target: CanvasPickTarget): string {
  return `${target.domain}:${target.id}`;
}

/** 🪪️ Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
  const colon = key.indexOf(":");
  if (colon < 0) return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

/** 🔽️ Orders overlapping pick targets from the most general to the most specific. */
export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

/** 🔼️ Picks the most specific of several overlapping targets. */
export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
  if (targets.length === 0) return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

/** 👁️ Hover focus a surface publishes for one pick target. */
export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
  return { sourceId, target };
}
