/** 📏 Owns the window measure rail sizes, the cleared top offset and the pane icons shared by window chrome. */
import { domSizePx } from "@semio-tech/ui-styling";
import { type IconName } from "@semio-tech/assets";
import { windowChromeScrollClearanceVar } from "../../🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx";

/** @emoji 📐️ Default unfolded width of window panes and the options rail (token-derived; matches panel default 300px). */
export const windowMeasuresDefaultWidthPx = domSizePx("layoutPanelRailUiSpacing");

/** @emoji 📐️ Minimum unfolded width of the window options rail (token-derived). */
export const windowMeasuresMinWidthPx = domSizePx("layoutPanelMinUiSpacing");

/** @emoji 📐️ Maximum unfolded width of the window options rail (token-derived). */
export const windowMeasuresMaxWidthPx = domSizePx("layoutPanelMaxUiSpacing");

/** @emoji 🚧️ Block offset that clears a window's floating chrome control row — the single rule for
 * anything a window's CONTENT anchors to a top corner (a scene overlay button, a status chip, the folded
 * engagement's quick-action rail). Reads the live clearance the enclosing {@link Window} publishes
 * ({@link windowChromeScrollClearanceVar}, measured off the mounted engagement/search/measures overlays)
 * and falls back to the chrome row's own token height, so content can never paint over a pane toggle. */
export const windowChromeClearedTopOffset = `calc(var(${windowChromeScrollClearanceVar}, calc(var(--size-medium) + var(--spacing-single))) + var(--spacing-single))`;

/** @emoji 🪟️ Pane chrome toggle — same layout as {@link panelAnchorTabButtonClass}: leading semantic icon, label, trailing {@link DragHandle}. */

/** @emoji 🪟️ Built-in window pane icons — fixed semantic affordances (never fold-direction chevrons). */
export const WINDOW_PANE_MEASURES_ICON = "settings-2" as const satisfies IconName;

export const WINDOW_PANE_ACTIONS_ICON = "play" as const satisfies IconName;

export const WINDOW_PANE_SEARCH_ICON = "search" as const satisfies IconName;

export const WINDOW_PANE_UTILITIES_ICON = "hammer" as const satisfies IconName;
