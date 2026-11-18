import { WINDOW_SILHOUETTE_PATH_INSET } from "../🔲️WindowSilhouette/🟦️.tsx";
import { sharedBorderAxisScale } from "../🔲️WindowSilhouette/shared-border.ts";
import { deviceSnappedRect } from "./shared-border.ts";

const matchesFocusVisible = (element: Element): boolean => {
  try {
    return element.matches(":focus-visible");
  } catch {
    return false;
  }
};

const SNAP_REACH = 2;

const lineCoordinates = (stack: HTMLElement): { readonly xs: number[]; readonly ys: number[] } => {
  const xs: number[] = [], ys: number[] = [];
  for (const path of stack.querySelectorAll(':scope > svg[data-window-silhouette-border] path:not([data-shared-border-state="focus"])')) {
    for (const [, command, args] of (path.getAttribute("d") ?? "").matchAll(/([MLHV])([^MLHVZmlhvz]*)/g)) {
      const values = (args ?? "").match(/-?\d*\.?\d+/g)?.map(Number) ?? [];
      if (command === "H") xs.push(...values);
      else if (command === "V") ys.push(...values);
      else values.forEach((value, index) => (index % 2 === 0 ? xs : ys).push(value));
    }
  }
  return { xs, ys };
};

const snapToLine = (value: number, lines: readonly number[]): number => {
  let best = value, reach = SNAP_REACH + 1e-6;
  for (const line of lines) if (Math.abs(line - value) < reach) { best = line; reach = Math.abs(line - value); }
  return best;
};

/** 🎯️ The outline of the chip of `stack` that holds keyboard focus, in the stack's own px as a closed path on the silhouette's centreline, or `null`. The silhouette paints it in the focus colour over its own line, so the focus indication of a chip control is the recolour of the chip's seams (and of the window edge where the chip reaches it), never a ring inside the lines. Chips of nested silhouettes are ignored. An edge within 2 px of a line of the silhouette's own border (a chip of a larger window, whose interior seams are not inset) snaps onto that line, so the focus never sits beside the seam it recolours.
 *
 * @see ./chip-focus.test.ts */
export function focusedWindowChipPath(stack: HTMLElement): string | null {
  const active = stack.ownerDocument.activeElement;
  if (!active || !stack.contains(active) || !matchesFocusVisible(active)) return null;
  const chip = active.closest<HTMLElement>("[data-window-silhouette-chip]");
  if (!chip || !stack.contains(chip) || chip.closest("[data-window-silhouette]") !== stack) return null;
  const stackRect = deviceSnappedRect(stack);
  const rect = deviceSnappedRect(chip);
  const x = sharedBorderAxisScale(stackRect.width, stack.offsetWidth), y = sharedBorderAxisScale(stackRect.height, stack.offsetHeight);
  const inset = WINDOW_SILHOUETTE_PATH_INSET;
  const left = (rect.left - stackRect.left) / x + inset, right = (rect.right - stackRect.left) / x - inset;
  const top = (rect.top - stackRect.top) / y + inset, bottom = (rect.bottom - stackRect.top) / y - inset;
  if (!(right > left && bottom > top)) return null;
  const { xs, ys } = lineCoordinates(stack);
  const [l, r] = [snapToLine(left, xs), snapToLine(right, xs)], [t, b] = [snapToLine(top, ys), snapToLine(bottom, ys)];
  return `M ${l} ${t} L ${r} ${t} L ${r} ${b} L ${l} ${b} Z`;
}
