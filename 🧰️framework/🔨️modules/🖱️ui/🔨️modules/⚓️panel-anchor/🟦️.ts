/** ⚓ Owns the panel anchor vocabulary and the positioning styles derived from it. */
import * as React from "react";
import { type Flow } from "../🧭️flow-direction-context/🟦️.tsx";

export const ANCHORS = ["top-left", "top-middle", "top-right", "right-middle", "bottom-right", "bottom-middle", "bottom-left", "left-middle"] as const;

/** 🧭️ One of the eight anchors a panel or pane can grow from. */
export type Anchor = (typeof ANCHORS)[number];

/** ↕️ `"top"`/`"middle"`/`"bottom"` row of an {@link Anchor} — `left-middle`/`right-middle` sit in the middle row. */
export function anchorVertical(anchor: Anchor): "top" | "middle" | "bottom" {
  if (anchor.startsWith("top")) return "top";
  if (anchor.startsWith("bottom")) return "bottom";
  return "middle";
}

/** ↔️ `"left"`/`"middle"`/`"right"` column of an {@link Anchor} — `top-middle`/`bottom-middle` sit in the middle column. */
export function anchorHorizontal(anchor: Anchor): "left" | "middle" | "right" {
  switch (anchor) {
    case "top-left":
    case "bottom-left":
    case "left-middle":
      return "left";
    case "top-right":
    case "bottom-right":
    case "right-middle":
      return "right";
    default:
      return "middle";
  }
}

/** 🌊️ The mirrored {@link Flow} a {@link Panel} or {@link Pane} grows into — right anchors flip inline, bottom anchors flip block; middle anchors (row or column) never mirror. */
export function flowFromAnchor(anchor: Anchor): Flow {
  return { inline: anchorHorizontal(anchor) === "right" ? "rtl" : "ltr", block: anchorVertical(anchor) === "bottom" ? "up" : "down" };
}

/** 📍️ Edge insets for an {@link Anchor} plus responsive width/height clamps to its containing region — corners inset on both axes, an edge-middle anchor centers along its middle axis via a translate. Shared by {@link Panel} and {@link Pane} so both float from identical math. */
export function anchorPositionStyle(anchor: Anchor): React.CSSProperties {
  const horizontal = anchorHorizontal(anchor);
  const vertical = anchorVertical(anchor);
  const style: React.CSSProperties = {
    maxWidth: "calc(100% - (var(--panel-inset) * 2))",
    maxHeight: "calc(100% - (var(--panel-inset) * 2))",
  };
  if (horizontal === "middle") {
    style.left = "50%";
    style.transform = "translateX(-50%)";
  } else {
    style[horizontal] = "var(--panel-inset)";
  }
  if (vertical === "middle") {
    style.top = "50%";
    style.transform = style.transform ? `${style.transform} translateY(-50%)` : "translateY(-50%)";
  } else {
    style[vertical] = "var(--panel-inset)";
  }
  return style;
}

/**
 * 🔓️ Open chrome-hosted {@link Panel} position — pulls the window-chrome cap into the navbar/footer
 * band where {@link PanelChromeTabBar} parked the folded toggles, so opening unfolds in place instead of
 * jumping the chips into the canvas. Navbar/footer are `h-large` with centered `h-medium` items; the offset
 * aligns the panel's `min-h-medium` cap to that centered row. Max height grows by the same overhang so the
 * body still fills the canvas region.
 * @see {@link anchorPositionStyle}
 */
export function chromeHostedOpenPanelPositionStyle(anchor: Anchor): React.CSSProperties {
  const style = anchorPositionStyle(anchor);
  const vertical = anchorVertical(anchor);
  const shellHeight = vertical === "bottom" ? "var(--footer-height)" : "var(--navbar-height)";
  const intoShellChrome = `calc(-1 * (${shellHeight} + var(--panel-header-height)) / 2)`;
  const maxHeightWithChrome = `calc(100% + (${shellHeight} + var(--panel-header-height)) / 2)`;
  if (vertical === "top") {
    style.top = intoShellChrome;
    style.maxHeight = maxHeightWithChrome;
  } else if (vertical === "bottom") {
    style.bottom = intoShellChrome;
    style.maxHeight = maxHeightWithChrome;
  }
  return style;
}

// Window-level floating chrome (measures, engagement, search, utility bar, and any app-contributed floating
// control) — anchored and draggable across the same eight anchors a Panel docks to (see 🧭️Panel above), so panels
// and panes share one positioning mechanism end to end. Naming follows the repo convention (see ticket
// `26/07/15/RENAME-WINDOW-PANELS-TO-PANES-AND-CORNER-PANELS-TO-PANELS`): "panel" is the shell-edge dock, "pane" is
// chrome floating inside one window.

/** 🎯️ Nearest anchor to a pointer position within a host's bounding rect — a 3×3 zone grid over the box.
 * There is no center anchor, so the dead-center zone resolves to whichever edge-middle the pointer has drifted
 * closer to (compares distance from the vertical vs. horizontal midline). */
export function nearestAnchor(pointerX: number, pointerY: number, hostRect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): Anchor {
  const x = hostRect.width > 0 ? (pointerX - hostRect.left) / hostRect.width : 0.5;
  const y = hostRect.height > 0 ? (pointerY - hostRect.top) / hostRect.height : 0.5;
  const col: "left" | "middle" | "right" = x < 1 / 3 ? "left" : x > 2 / 3 ? "right" : "middle";
  const row: "top" | "middle" | "bottom" = y < 1 / 3 ? "top" : y > 2 / 3 ? "bottom" : "middle";
  if (row !== "middle" && col !== "middle") return `${row}-${col}` as Anchor;
  if (row !== "middle") return `${row}-middle` as Anchor;
  if (col !== "middle") return `${col}-middle` as Anchor;
  const distanceFromVerticalMidline = Math.abs(x - 0.5);
  const distanceFromHorizontalMidline = Math.abs(y - 0.5);
  if (distanceFromVerticalMidline >= distanceFromHorizontalMidline) return x < 0.5 ? "left-middle" : "right-middle";
  return y < 0.5 ? "top-middle" : "bottom-middle";
}
