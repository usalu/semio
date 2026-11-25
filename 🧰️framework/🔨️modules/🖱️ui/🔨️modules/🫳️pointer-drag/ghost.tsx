import * as React from "react";
import { createPortal } from "react-dom";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { type DropPlaceState, dragCancelKey, dragGhostPosition } from "./places.ts";

/** 🫳️ The one mechanism of every drag: the dragged element leaves its place (hidden, it keeps its room) and travels with the pointer as one floating rectangle of menu-level glass with the chip tint, on a layer above everything; while it travels every place it can be dropped is tinted, the one under the pointer more strongly.
 *
 * A host states the drag by attributes — `data-drag-active` on the page, `data-drag-hidden` on the source, `data-drop-place` and `data-drop-active` on the places — and the paint lives in `🎨️styling/🖌️ui/🎨️.css`. The pointer drags of panels and windows render {@link DragGhost}; the native drags of rows call {@link beginNativeDragGhost}. A drag is always accompanied by a keyboard path in its host; the copy and the tints are hidden from assistive technology.
 * @see https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API
 * @see https://w3c.github.io/pointerevents/ */

/** 🏷️ The page attribute that marks a running drag. */
export const DRAG_ACTIVE_ATTRIBUTE = "data-drag-active";

let runningDrags = 0;

function markDrag(delta: 1 | -1): void {
  runningDrags = Math.max(0, runningDrags + delta);
  if (runningDrags > 0) document.documentElement.setAttribute(DRAG_ACTIVE_ATTRIBUTE, "");
  else document.documentElement.removeAttribute(DRAG_ACTIVE_ATTRIBUTE);
}

/** 🚩️ Marks the page for as long as `active` holds, so every `data-drop-place` shows. */
export function useDragActive(active: boolean): void {
  reactHostPort.useEffect(() => {
    if (!active) return undefined;
    markDrag(1);
    return () => markDrag(-1);
  }, [active]);
}

/** ⎋ Calls `cancel` when Escape is pressed while `active` holds. */
export function useDragEscape(active: boolean, cancel: () => void): void {
  const cancelRef = reactHostPort.useRef(cancel);
  cancelRef.current = cancel;
  reactHostPort.useEffect(() => {
    if (!active) return undefined;
    const onKeyDown = (event: KeyboardEvent): void => {
      if (!dragCancelKey(event.key)) return;
      event.preventDefault();
      cancelRef.current();
    };
    document.addEventListener("keydown", onKeyDown, true);
    return () => document.removeEventListener("keydown", onKeyDown, true);
  }, [active]);
}

/** 🎯️ The attributes of a place: none outside a drag, `data-drop-place` while the element travels, `data-drop-active` under the pointer. */
export function dropPlaceAttributes(state: DropPlaceState | undefined): Record<string, string> {
  if (state === undefined) return {};
  return state === "active" ? { "data-drop-place": "", "data-drop-active": "" } : { "data-drop-place": "" };
}

/** 🫥️ The attributes of the source of a drag: hidden in place, it keeps its room. */
export function dragSourceAttributes(hidden: boolean): Record<string, string> {
  return hidden ? { "data-drag-hidden": "" } : {};
}

/** 👻️ The travelling copy: one rectangle of menu-level glass with the chip tint at the pointer, above everything, inert. */
export const DragGhost: React.FC<{ readonly x: number; readonly y: number; readonly children: React.ReactNode }> = ({ x, y, children }) => {
  const at = dragGhostPosition({ x, y });
  return createPortal(
    <div data-slot="drag-ghost" data-drag-ghost="" data-level="menu" data-glass-chip="" className="ui-glass" aria-hidden="true" inert style={{ transform: `translate(${at.x}px, ${at.y}px)` }}>
      {children}
    </div>,
    document.body,
  );
};

function blankDragImage(): HTMLElement {
  const blank = document.createElement("div");
  blank.style.cssText = "position:fixed;left:-1000px;top:0;width:1px;height:1px;opacity:0";
  document.body.append(blank);
  window.setTimeout(() => blank.remove(), 0);
  return blank;
}

/** 🫳️ Detaches a native HTML5 drag: the browser's own drag image is replaced by an empty one, a {@link DragGhost}-like copy follows the pointer, the source is hidden in place after the browser has taken the drag, and `dragend` — also after Escape — takes everything away again. Returns the function that ends it earlier. */
export function beginNativeDragGhost(event: { readonly dataTransfer: DataTransfer | null; readonly currentTarget: EventTarget; readonly clientX: number; readonly clientY: number }, label: string, options: { readonly hideSource?: boolean } = {}): () => void {
  const source = event.currentTarget instanceof HTMLElement && options.hideSource !== false ? event.currentTarget : null;
  const ghost = document.createElement("div");
  ghost.setAttribute("data-slot", "drag-ghost");
  ghost.setAttribute("data-drag-ghost", "");
  ghost.setAttribute("data-level", "menu");
  ghost.setAttribute("data-glass-chip", "");
  ghost.setAttribute("aria-hidden", "true");
  ghost.setAttribute("inert", "");
  ghost.className = "ui-glass";
  ghost.textContent = label;
  const place = (x: number, y: number): void => {
    const at = dragGhostPosition({ x, y });
    ghost.style.transform = `translate(${at.x}px, ${at.y}px)`;
  };
  place(event.clientX, event.clientY);
  document.body.append(ghost);
  event.dataTransfer?.setDragImage?.(blankDragImage(), 0, 0);
  markDrag(1);
  const hide = window.setTimeout(() => source?.setAttribute("data-drag-hidden", ""), 0);
  const move = (over: Event): void => {
    const { clientX, clientY } = over as DragEvent;
    if (clientX !== 0 || clientY !== 0) place(clientX, clientY);
  };
  let finished = false;
  const end = (): void => {
    if (finished) return;
    finished = true;
    window.clearTimeout(hide);
    document.removeEventListener("dragover", move, true);
    document.removeEventListener("drag", move, true);
    document.removeEventListener("dragend", end, true);
    document.removeEventListener("drop", end, true);
    ghost.remove();
    source?.removeAttribute("data-drag-hidden");
    markDrag(-1);
  };
  document.addEventListener("dragover", move, true);
  document.addEventListener("drag", move, true);
  document.addEventListener("dragend", end, true);
  document.addEventListener("drop", end, true);
  return end;
}

/** 🎯️ Whether a native drag is over a place: `over` holds from the first `dragenter` until the last `dragleave` of the place and its children, and ends at a drop or at the end of any drag. */
export function useNativeDropOver(): { readonly over: boolean; readonly bind: { readonly onDragEnter: React.DragEventHandler<HTMLElement>; readonly onDragLeave: React.DragEventHandler<HTMLElement> }; readonly reset: () => void } {
  const [over, setOver] = reactHostPort.useState(false);
  const depth = reactHostPort.useRef(0);
  const reset = reactHostPort.useCallback(() => {
    depth.current = 0;
    setOver(false);
  }, []);
  reactHostPort.useEffect(() => {
    if (!over) return undefined;
    document.addEventListener("dragend", reset, true);
    document.addEventListener("drop", reset, true);
    return () => {
      document.removeEventListener("dragend", reset, true);
      document.removeEventListener("drop", reset, true);
    };
  }, [over, reset]);
  const bind = reactHostPort.useMemo(
    () => ({
      onDragEnter: () => {
        depth.current += 1;
        setOver(true);
      },
      onDragLeave: () => {
        depth.current = Math.max(0, depth.current - 1);
        if (depth.current === 0) setOver(false);
      },
    }),
    [],
  );
  return { over, bind, reset };
}

/** 🧩️ The part of a dnd-kit drag event the ghost reads: the active element, the event that started the drag and the travel since. */
export interface DndGhostEvent {
  readonly active: { readonly id: string | number; readonly rect: { readonly current: { readonly initial: { readonly left: number; readonly top: number } | null } } };
  readonly activatorEvent: Event;
  readonly delta?: { readonly x: number; readonly y: number };
}

/** 🏷️ The text of a dragged element on one line: the label its copy shows. */
export function dragLabelOf(element: Element | null): string {
  return (element?.textContent ?? "").replace(/\s+/gu, " ").trim();
}

/** 🫳️ Detaches a dnd-kit drag: `activeId` names the source to hide in place, `ghost` is the copy to render once, `start`, `move` and `end` belong to `onDragStart`, `onDragMove` and `onDragEnd` / `onDragCancel`. The label is what `labelOf` names for the id of the dragged element. */
export function useDndGhost(labelOf: (id: string) => string): { readonly activeId: string | null; readonly ghost: React.ReactNode; readonly start: (event: DndGhostEvent) => void; readonly move: (event: DndGhostEvent) => void; readonly end: () => void } {
  const [drag, setDrag] = reactHostPort.useState<{ readonly id: string; readonly label: string; readonly x: number; readonly y: number } | null>(null);
  useDragActive(drag !== null);
  const pointer = (event: DndGhostEvent): { x: number; y: number } => {
    const origin = event.activatorEvent as Partial<PointerEvent>;
    const travel = event.delta ?? { x: 0, y: 0 };
    if (typeof origin.clientX === "number" && typeof origin.clientY === "number") return { x: origin.clientX + travel.x, y: origin.clientY + travel.y };
    const bounds = event.active.rect.current.initial;
    return { x: (bounds?.left ?? 0) + travel.x, y: (bounds?.top ?? 0) + travel.y };
  };
  const start = reactHostPort.useCallback(
    (event: DndGhostEvent) => {
      const id = String(event.active.id);
      const at = pointer(event);
      setDrag({ id, label: labelOf(id), ...at });
    },
    [labelOf],
  );
  const move = reactHostPort.useCallback((event: DndGhostEvent) => {
    const at = pointer(event);
    setDrag((previous) => (previous === null ? previous : { ...previous, ...at }));
  }, []);
  const end = reactHostPort.useCallback(() => setDrag(null), []);
  return { activeId: drag?.id ?? null, ghost: drag === null ? null : <DragGhost x={drag.x} y={drag.y}>{drag.label}</DragGhost>, start, move, end };
}
