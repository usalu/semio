import { chipGapReadsAsSliver } from "../🃏️CardStack/bands.ts";
import { sharedBorderAxisScale } from "../🔲️WindowFrame/shared-border.ts";

/** 🫸️ One chip of a cap or footer row in the row's own px: its painted extent, the width its content needs, and the neighbour it may stretch toward (`next` grows to the right, `prev` to the left). */
export interface ChipRowChip {
  readonly left: number;
  readonly right: number;
  readonly natural: number;
  readonly reach: "none" | "next" | "prev";
}

/** 🫸️ Which chips of one row must stretch to abut their neighbour because the open space between them would be a sliver under {@link chipGapReadsAsSliver}, so the two share one seam instead of a narrow strip of content. Gaps are read from the natural extents (a stretched chip does not hide the room it reopens), so the decision is stable. Pure and language-agnostic.
 *
 * @see ./chip-abut.cases.json */
export function resolveChipRowAbuts(chips: readonly ChipRowChip[]): readonly boolean[] {
  const abuts = chips.map(() => false);
  for (let index = 0; index + 1 < chips.length; index++) {
    const a = chips[index]!, b = chips[index + 1]!;
    const aRight = a.reach === "next" ? a.left + a.natural : a.right;
    const bLeft = b.reach === "prev" ? b.right - b.natural : b.left;
    if (!chipGapReadsAsSliver(bLeft - aRight)) continue;
    if (a.reach === "next") abuts[index] = true;
    else if (b.reach === "prev") abuts[index + 1] = true;
  }
  return abuts;
}

/** 📏️ The width a chip marked by {@link resolveChipRowAbuts} must take to close its sliver: from its own edge to the neighbour's natural edge. Pure and language-agnostic.
 *
 * @see ./chip-abut.cases.json */
export function chipAbutSize(chips: readonly ChipRowChip[], index: number): number {
  const chip = chips[index]!;
  if (chip.reach === "next") return chips[index + 1]!.left - chip.left;
  const previous = chips[index - 1]!;
  return chip.right - (previous.reach === "next" ? previous.left + previous.natural : previous.right);
}

const CHIP_SELECTOR = "[data-window-frame-chip]";
const PANE_ANCHOR_REACH = { "top-left": "next", "bottom-left": "next", "top-right": "prev", "bottom-right": "prev" } as const;

/** 🫸️ The cross-pane twin of {@link settleChipRowAbuts}: the folded anchored panes of one window body (left, middle, right of one edge) are separate frames, so their chips are ordered by position and a left-anchored chip reaches next, a right-anchored one prev. A marked chip gets `data-chip-abut` and its stack the width that closes the sliver (`--chip-abut-size`). Expanded panes keep their own cap row and only act as neighbours.
 *
 * @see ./chip-abut.cases.json */
export function settlePaneChipAbuts(body: HTMLElement): boolean {
  const rect = body.getBoundingClientRect();
  const scale = sharedBorderAxisScale(rect.width, body.offsetWidth);
  let changed = false;
  for (const edge of ["top", "bottom"]) {
    const panes = [...body.querySelectorAll<HTMLElement>(`[data-anchor^="${edge}-"]`)].filter(overlay => overlay.closest("[data-slot='window-body']") === body).flatMap(overlay => {
      const stack = overlay.querySelector<HTMLElement>(":scope > [data-window-frame]");
      const folded = stack?.querySelector<HTMLElement>(":scope > [data-window-frame-chip]") ?? null;
      const chip = folded ?? stack?.querySelector<HTMLElement>(":scope > [data-slot='window-chrome-cap'] [data-window-frame-chip]") ?? null;
      return stack && chip ? [{ stack, chip, owned: folded !== null, anchor: overlay.dataset.anchor as keyof typeof PANE_ANCHOR_REACH }] : [];
    });
    const measured = panes.map(({ chip, owned, anchor }) => {
      const box = chip.getBoundingClientRect();
      const natural = [...chip.children].reduce((sum, child) => sum + child.getBoundingClientRect().width, 0) / scale;
      return { left: (box.left - rect.left) / scale, right: (box.right - rect.left) / scale, natural, reach: owned ? (PANE_ANCHOR_REACH[anchor] ?? "none") : "none" } as const;
    });
    const order = measured.map((_, index) => index).sort((a, b) => measured[a]!.left - measured[b]!.left);
    const sorted = order.map(index => measured[index]!);
    const abuts = resolveChipRowAbuts(sorted);
    order.forEach((index, position) => {
      const { stack, chip } = panes[index]!;
      const size = abuts[position] ? `${chipAbutSize(sorted, position)}px` : "";
      if (chip.hasAttribute("data-chip-abut") !== abuts[position] || stack.style.getPropertyValue("--chip-abut-size") !== size) {
        chip.toggleAttribute("data-chip-abut", abuts[position]);
        if (size) stack.style.setProperty("--chip-abut-size", size);
        else stack.style.removeProperty("--chip-abut-size");
        changed = true;
      }
    });
  }
  return changed;
}

/** 🪟️ Reads the chips of `row` (its own chrome, not nested frames), marks the ones that must stretch with `data-chip-abut` and returns whether any mark changed. The chips declare their reach with `data-chip-reach`; the stylesheet stretches a marked chip over the free row space beside it.
 *
 * @see ./chip-abut.cases.json */
export function settleChipRowAbuts(row: HTMLElement): boolean {
  const owner = row.closest("[data-window-frame]");
  const chips = [...row.querySelectorAll<HTMLElement>(CHIP_SELECTOR)].filter(chip => chip.closest("[data-window-frame]") === owner);
  const rect = row.getBoundingClientRect();
  const scale = sharedBorderAxisScale(rect.width, row.offsetWidth);
  const measured = chips.map(chip => {
    const box = chip.getBoundingClientRect();
    const natural = [...chip.children].reduce((sum, child) => sum + child.getBoundingClientRect().width, 0) / scale;
    const reach = chip.dataset.chipReach;
    return { left: (box.left - rect.left) / scale, right: (box.right - rect.left) / scale, natural, reach: reach === "next" || reach === "prev" ? reach : "none" } as const;
  });
  const abuts = resolveChipRowAbuts(measured);
  let changed = false;
  chips.forEach((chip, index) => {
    if (chip.hasAttribute("data-chip-abut") === abuts[index]) return;
    chip.toggleAttribute("data-chip-abut", abuts[index]);
    changed = true;
  });
  return changed;
}

/** 🔭️ Keeps the chip marks of `row` settled while it, its chips or its children resize or change. */
export function observeChipRowAbuts(row: HTMLElement, settle: (row: HTMLElement) => boolean = settleChipRowAbuts): () => void {
  let frame = 0;
  const run = () => {
    frame = 0;
    settle(row);
  };
  const schedule = () => {
    if (frame) return;
    if (typeof requestAnimationFrame === "function") frame = requestAnimationFrame(run);
    else run();
  };
  const resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(schedule);
  const refresh = () => {
    resizeObserver?.disconnect();
    resizeObserver?.observe(row);
    for (const chip of row.querySelectorAll(CHIP_SELECTOR)) resizeObserver?.observe(chip);
  };
  const mutationObserver = typeof MutationObserver === "undefined" ? null : new MutationObserver(() => { refresh(); schedule(); });
  refresh();
  mutationObserver?.observe(row, { childList: true, subtree: true });
  settle(row);
  return () => {
    if (frame && typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
    resizeObserver?.disconnect();
    mutationObserver?.disconnect();
  };
}
