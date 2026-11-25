import type { WindowLayoutDropZone, WindowLayoutPath, WindowStackCorner } from "@semio-tech/ui-layout";
import * as React from "react";
import { dropPlaceAttributes } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";
import { computeModeSplitPreviewInBody } from "../🎯️drop/🟦️.ts";

/** 🧲️ The stacks a window can be dropped on, as the elements the canvas registered for them. */
export type ModeDropStacks = { readonly current: ReadonlyMap<string, { readonly path: WindowLayoutPath; readonly corners: Partial<Record<WindowStackCorner, HTMLElement | null>>; readonly body: HTMLElement | null }> };

const place = (left: number, top: number, width: number, height: number, active: boolean, key: string): React.ReactNode => (
  <div key={key} data-slot={active ? "mode-dock-drop-indicator" : undefined} {...dropPlaceAttributes(active ? "active" : "available")} className="absolute" style={{ left, top, width, height }} />
);

/**
 * 🫳️ The places of a window drag on the canvas, laid over its content inside the canvas rectangle: every stack is a place a window can be split off or docked into, and the half of the stack or of the canvas the drop would take is the one under the pointer. A drop among the tabs is shown by the tab rows and the slot in them.
 * The rectangle of the active place of a split is `mode-dock-drop-indicator`, as the acceptance of the dock reads it.
 * @see ../🎯️drop/🟦️.ts `computeModeSplitPreviewInBody`
 */
export const ModeDropPlaces: React.FC<{ readonly stacks: ModeDropStacks; readonly host: { readonly current: HTMLElement | null }; readonly zone: WindowLayoutDropZone | null }> = ({ stacks, host, zone }) => {
  const mode = host.current?.getBoundingClientRect();
  if (!mode) return null;
  const bodies = [...stacks.current.entries()].flatMap(([key, { body }]) => (body ? [{ key, bounds: body.getBoundingClientRect() }] : []));
  const children: React.ReactNode[] = bodies.map(({ key, bounds }) => place(bounds.left - mode.left, bounds.top - mode.top, bounds.width, bounds.height, false, `stack-${key}`));
  if (zone?.kind === "rootSplit") {
    const half = computeModeSplitPreviewInBody(mode.width, mode.height, zone.side);
    children.push(place(half.left, half.top, half.width, half.height, true, "active"));
  } else if (zone?.kind === "split") {
    const target = bodies.find(({ key }) => stacks.current.get(key)?.path.join(".") === zone.stackPath.join("."));
    if (target) {
      const half = computeModeSplitPreviewInBody(target.bounds.width, target.bounds.height, zone.side);
      children.push(place(target.bounds.left - mode.left + half.left, target.bounds.top - mode.top + half.top, half.width, half.height, true, "active"));
    }
  }
  return (
    <div data-slot="mode-dock-drop-places" aria-hidden="true" className="pointer-events-none absolute inset-0 z-panel">
      {children}
    </div>
  );
};
