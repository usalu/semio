/** 🗺 Owns the window layout node shapes and the even layout builder used by canvases. */
import { type UiLabel } from "../../🧱️elements/🎗️UiLabel/🟦️.tsx";

export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";

export interface WindowLayoutWindowNode {
  kind: "window";
  id: string;
  title?: UiLabel;
  size?: number;
  corner?: WindowStackCorner;
}

export interface WindowLayoutStackNode {
  kind: "stack";
  size?: number;
  activeId?: string;
  children: readonly WindowLayoutWindowNode[];
}

export interface WindowLayoutAxisNode {
  kind: "row" | "column";
  size?: number;
  children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

/** @emoji 🪟️ Recursive resizable window layout tree for {@link Mode}. */
export type WindowLayoutNode = WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode;

/** @emoji 🪟️ Builds an even horizontal split layout for the given window ids. */
export function createEvenWindowLayout(windowIds: readonly string[]): WindowLayoutNode {
  if (windowIds.length === 0) return { kind: "stack", children: [] };
  if (windowIds.length === 1) return { kind: "stack", children: [{ kind: "window", id: windowIds[0]! }] };
  return {
    kind: "row",
    children: windowIds.map((id) => ({
      kind: "stack" as const,
      children: [{ kind: "window" as const, id }],
    })),
  };
}
