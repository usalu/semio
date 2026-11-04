/** 📐️ How an open floating panel fits a viewport: it never takes more than a viewport fraction, and it folds when the column left for the editor would be too narrow to work in. */

/** 🔲️ The largest fraction of the viewport width one floating panel may take. */
export const PANEL_MAX_VIEWPORT_FRACTION = 0.45;

/** 📏️ The narrowest editor column, in CSS pixels, a floating panel may leave visible beside it. */
export const PANEL_MIN_EDITOR_WIDTH_PX = 420;

/** 🧮️ The width an open panel renders at and whether it must fold, for one viewport width and the width its caller asks for. */
export function panelFitV1(viewportWidthPx: number, requestedPx: number): Readonly<{ widthPx: number; collapsed: boolean }> {
  const widthPx = Math.max(0, Math.min(requestedPx, Math.floor(viewportWidthPx * PANEL_MAX_VIEWPORT_FRACTION)));
  return { widthPx, collapsed: viewportWidthPx - widthPx < PANEL_MIN_EDITOR_WIDTH_PX };
}
