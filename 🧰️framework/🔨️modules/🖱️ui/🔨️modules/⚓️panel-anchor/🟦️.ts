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

/** 📍️ Anchor positioning with matched bounds clamps: Panels and contained Panes meet their host edges (zero inset) so their outline shares the host's line. Edge-middle anchors remain centered by a translate. */
export function anchorPositionStyle(anchor: Anchor, inset = "0px"): React.CSSProperties {
  const horizontal = anchorHorizontal(anchor);
  const vertical = anchorVertical(anchor);
  const style: React.CSSProperties = {
    maxWidth: `calc(100% - (${inset} * 2))`,
    maxHeight: `calc(100% - (${inset} * 2))`,
  };
  if (horizontal === "middle") {
    style.left = "50%";
    style.transform = "translateX(-50%)";
  } else {
    style[horizontal] = inset;
  }
  if (vertical === "middle") {
    style.top = "50%";
    style.transform = style.transform ? `${style.transform} translateY(-50%)` : "translateY(-50%)";
  } else {
    style[vertical] = inset;
  }
  return style;
}

/**
 * 🔓️ Open chrome-hosted {@link Panel} position — a top panel pulls the window-chrome cap into the navbar
 * band where {@link PanelChromeTabBar} parked the folded toggles, so opening unfolds in place instead of
 * jumping the chips into the canvas. The cap is exactly as tall as the band (`--panel-header-height`
 * is set to the band height) and starts on the band's outer edge, so its tab frame shares the band's lines. Max height
 * grows by the same overhang and stops one footer band short, so the body fills the canvas region without covering the folded bottom
 * panel chips (`Canvas` `bottomChips`) that share the same column. A bottom panel has no band to overhang: its
 * folded toggles are glass chips inside the canvas frame's bottom edge (`Canvas` `bottomChips`), so the open cap lands
 * on the same row inside the canvas rectangle.
 * @see {@link anchorPositionStyle}
 */
export function chromeHostedOpenPanelPositionStyle(anchor: Anchor): React.CSSProperties {
  const style: Record<string, unknown> = { ...anchorPositionStyle(anchor) };
  const vertical = anchorVertical(anchor);
  style["--panel-header-height"] = vertical === "bottom" ? "var(--footer-band-height)" : "var(--navbar-band-height)";
  if (vertical === "top") {
    style.top = "calc(-1 * var(--navbar-band-height))";
    style.maxHeight = "calc(100% + var(--navbar-band-height) - var(--footer-band-height))";
  }
  return style as React.CSSProperties;
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

/** 🔲️ The cell of the 3×3 grid over a host that an anchor stands for: the corner anchors take the corner cells, the edge middles the middle cell of their edge. The dead centre belongs to no anchor, {@link nearestAnchor} assigns it to an edge middle. */
export function anchorPlaceRect(anchor: Anchor, hostRect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number } {
  const column = { left: 0, middle: 1, right: 2 }[anchorHorizontal(anchor)];
  const row = { top: 0, middle: 1, bottom: 2 }[anchorVertical(anchor)];
  const cellWidth = hostRect.width / 3;
  const cellHeight = hostRect.height / 3;
  return { left: hostRect.left + column * cellWidth, top: hostRect.top + row * cellHeight, right: hostRect.left + (column + 1) * cellWidth, bottom: hostRect.top + (row + 1) * cellHeight };
}

/** 🎯️ The eight places a pane can be dropped on over a host, one per anchor. */
export function anchorPlaces(hostRect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): readonly { readonly id: Anchor; readonly rect: ReturnType<typeof anchorPlaceRect> }[] {
  return ANCHORS.map((anchor) => ({ id: anchor, rect: anchorPlaceRect(anchor, hostRect) }));
}
