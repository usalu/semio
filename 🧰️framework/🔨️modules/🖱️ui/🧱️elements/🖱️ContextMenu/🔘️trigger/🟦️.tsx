/** 🔘️ A visible, keyboard- and touch-reachable trigger for a menu that otherwise opens only on a right click: activating it dispatches the same `contextmenu` event the right click raises, anchored at the target, so each host keeps its one menu path. */
import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { useLabel } from "../../🏷️Label/🟦️.tsx";

/** 📍️ Where a triggered menu anchors on its target: just below its trailing edge. */
export function contextMenuAnchorPointV1(rect: Readonly<{ left: number; top: number; right: number; bottom: number }>): Readonly<{ x: number; y: number }> {
  return { x: Math.max(rect.left, rect.right - 4), y: Math.min(rect.bottom, rect.top + 24) };
}

/** 🖱️ Raises a `contextmenu` event on `target` at `point`, exactly as a secondary click would. Returns whether a handler took it. */
export function dispatchContextMenuAtV1(target: Element, point: Readonly<{ x: number; y: number }>): boolean {
  return !target.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true, button: 2, buttons: 2, clientX: point.x, clientY: point.y }));
}

export interface ContextMenuTriggerButtonProps {
  /** 🎯️ The element whose context menu opens; resolved at activation so a re-rendered target is never stale. */
  resolveTarget: (button: HTMLButtonElement) => Element | null;
  className?: string;
}

/** 🔘️ The visible "More" button (`ui.contextMenu.more`, English and German) that opens a right-click-only menu. */
export const ContextMenuTriggerButton: React.FC<ContextMenuTriggerButtonProps> = ({ resolveTarget, className }) => {
  const label = useLabel("ui.contextMenu.more");
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      aria-haspopup="menu"
      data-context-menu-trigger=""
      className={cn("inline-flex size-6 shrink-0 items-center justify-center rounded text-muted-foreground hover:bg-accent focus-visible:outline focus-visible:outline-2 [@media(pointer:coarse)]:size-10", className)}
      onClick={(event) => {
        event.stopPropagation();
        const button = event.currentTarget;
        const target = resolveTarget(button);
        if (target !== null) dispatchContextMenuAtV1(target, contextMenuAnchorPointV1(target.getBoundingClientRect()));
      }}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <span aria-hidden="true">⋯</span>
    </button>
  );
};
