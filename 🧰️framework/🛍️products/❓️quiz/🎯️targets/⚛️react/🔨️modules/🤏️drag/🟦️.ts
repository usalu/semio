/** 🤏️ Pointer drag and drop for task interactions — mouse, pen and touch through one Pointer Events path.
 *
 * Drop zones are plain elements carrying `data-quiz-drop="<zone>"`; a drag starts on a grip, follows the pointer with
 * an inert ghost copy of its source, highlights the zone under the pointer and reports the zone it was released over.
 * Escape or a cancelled pointer abandons the drag. Every interaction also has a keyboard path in its task component;
 * dragging is the additional pointer path, so the ghost and highlights are hidden from assistive technology.
 */

/** 📏️ Pointer travel in CSS pixels before a press becomes a drag. */
export const DRAG_THRESHOLD_PX = 4;

/** 🎯️ The drop zone under a viewport point, if any. */
export function dropZoneAt(x: number, y: number): string | undefined {
  const element = typeof document.elementFromPoint === "function" ? document.elementFromPoint(x, y) : null;
  return element?.closest<HTMLElement>("[data-quiz-drop]")?.dataset.quizDrop;
}

function zoneElement(zone: string | undefined, root: ParentNode): HTMLElement | undefined {
  if (zone === undefined) return undefined;
  return [...root.querySelectorAll<HTMLElement>("[data-quiz-drop]")].find((element) => element.dataset.quizDrop === zone);
}

/** 🤏️ Starts a pointer drag of the element around `event.currentTarget` (its nearest `[data-quiz-drag]` ancestor);
 * `onDrop` receives the zone the pointer is released over. */
export function startPointerDrag(event: { readonly button: number; readonly clientX: number; readonly clientY: number; readonly currentTarget: Element; preventDefault(): void }, onDrop: (zone: string) => void): void {
  if (event.button !== 0) return;
  event.preventDefault();
  const source = event.currentTarget.closest<HTMLElement>("[data-quiz-drag]") ?? (event.currentTarget as HTMLElement);
  const host = source.closest<HTMLElement>(".quiz-app") ?? document.body;
  const origin = { x: event.clientX, y: event.clientY };
  const bounds = source.getBoundingClientRect();
  let ghost: HTMLElement | undefined;
  let highlighted: HTMLElement | undefined;
  const highlight = (zone: string | undefined): void => {
    const element = zoneElement(zone, host);
    if (element === highlighted) return;
    highlighted?.classList.remove("quiz-drop-active");
    element?.classList.add("quiz-drop-active");
    highlighted = element;
  };
  const move = (moved: PointerEvent): void => {
    if (ghost === undefined) {
      if (Math.hypot(moved.clientX - origin.x, moved.clientY - origin.y) < DRAG_THRESHOLD_PX) return;
      ghost = source.cloneNode(true) as HTMLElement;
      ghost.classList.add("quiz-drag-ghost");
      ghost.setAttribute("aria-hidden", "true");
      ghost.style.width = `${bounds.width}px`;
      host.append(ghost);
      source.classList.add("quiz-dragging");
    }
    ghost.style.transform = `translate(${bounds.left + moved.clientX - origin.x}px, ${bounds.top + moved.clientY - origin.y}px)`;
    highlight(dropZoneAt(moved.clientX, moved.clientY));
  };
  const finish = (): void => {
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", release);
    window.removeEventListener("pointercancel", finish);
    window.removeEventListener("keydown", escape);
    highlight(undefined);
    ghost?.remove();
    source.classList.remove("quiz-dragging");
  };
  const release = (released: PointerEvent): void => {
    const dragged = ghost !== undefined;
    finish();
    const zone = dragged ? dropZoneAt(released.clientX, released.clientY) : undefined;
    if (zone !== undefined) onDrop(zone);
  };
  const escape = (key: KeyboardEvent): void => {
    if (key.key === "Escape") finish();
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", release);
  window.addEventListener("pointercancel", finish);
  window.addEventListener("keydown", escape);
}
