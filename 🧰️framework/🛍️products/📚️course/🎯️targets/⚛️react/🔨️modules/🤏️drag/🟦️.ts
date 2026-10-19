/** 🤏️ Pointer drag and drop for task interactions — mouse, pen and touch through one Pointer Events path.
 *
 * Drop zones are plain elements carrying `data-course-drop="<zone>"`; a drag starts on a grip, follows the pointer with
 * an inert ghost copy of its source, highlights the zone under the pointer and reports the zone it was released over.
 * Escape or a cancelled pointer abandons the drag. Every interaction also has a keyboard path in its task component;
 * dragging is the additional pointer path, so the ghost and highlights are hidden from assistive technology.

 * A finger on a grip must not pan the page at the same time. The task sits in a scroller (`overflow: auto`), and a
 * browser pans that scroller from the touch unless the grip refuses panning and the touch's move is cancelled. The
 * cancellation is a listener of its own, not the pointer move: on `window` a touch move is passive unless told
 * otherwise, and a passive listener cannot cancel it.
 * @see https://w3c.github.io/pointerevents/#determining-supported-direct-manipulation-behavior
 * @see https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#passive
 */

/** 📏️ Pointer travel in CSS pixels before a press becomes a drag. */
export const DRAG_THRESHOLD_PX = 4;

/** 🎯️ The drop zone under a viewport point, if any. */
export function dropZoneAt(x: number, y: number): string | undefined {
  const element = typeof document.elementFromPoint === "function" ? document.elementFromPoint(x, y) : null;
  return element?.closest<HTMLElement>("[data-course-drop]")?.dataset.courseDrop;
}

function zoneElement(zone: string | undefined, root: ParentNode): HTMLElement | undefined {
  if (zone === undefined) return undefined;
  return [...root.querySelectorAll<HTMLElement>("[data-course-drop]")].find((element) => element.dataset.courseDrop === zone);
}

/** 👻️ An inert copy of `source` that looks like it, in an empty copy of the list it lies in: the list is the room its
 * rows lay themselves out for (a container query), so the copy keeps the layout of its source — one line or two —
 * wherever it is carried. Cloning a node does not carry over what a select currently shows, so every select of the
 * copy is given the value of the one it copies.
 * @see https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element — no cloning steps
 * @see ../../🎨️.css — `.course-rows` */
function ghostOf(source: HTMLElement): HTMLElement {
  const copy = source.cloneNode(true) as HTMLElement;
  const shown = source.querySelectorAll("select");
  copy.querySelectorAll("select").forEach((select, index) => (select.value = shown[index]!.value));
  copy.style.width = `${source.getBoundingClientRect().width}px`;
  const room = source.parentElement;
  const ghost = room === null ? document.createElement("div") : (room.cloneNode(false) as HTMLElement);
  for (const own of ["id", "tabindex", "aria-label", "aria-labelledby"]) ghost.removeAttribute(own);
  ghost.classList.add("course-drag-ghost");
  ghost.setAttribute("aria-hidden", "true");
  if (room !== null) ghost.style.width = `${room.getBoundingClientRect().width}px`;
  ghost.append(copy);
  return ghost;
}

/** 📌️ Pins `id` to `target` so the drag keeps the pointer when the finger leaves the grip; a pointer the
 * browser does not know (a synthetic press) is passed over. */
function holdPointer(target: Element, id: number | undefined): void {
  if (id === undefined) return;
  const capture = target as Element & { setPointerCapture?: (pointerId: number) => void };
  if (typeof capture.setPointerCapture !== "function") return;
  try {
    capture.setPointerCapture(id);
  } catch {
    return;
  }
}

/** 📎️ Gives `id` back when the drag ends. */
function releasePointer(target: Element, id: number | undefined): void {
  if (id === undefined) return;
  const capture = target as Element & { hasPointerCapture?: (pointerId: number) => boolean; releasePointerCapture?: (pointerId: number) => void };
  if (typeof capture.releasePointerCapture !== "function") return;
  try {
    if (capture.hasPointerCapture?.(id) === false) return;
    capture.releasePointerCapture(id);
  } catch {
    return;
  }
}

/** 🤏️ Starts a pointer drag of the element around `event.currentTarget` (its nearest `[data-course-drag]` ancestor);
 * `onDrop` receives the zone the pointer is released over. A touch that starts here cannot scroll the page: the grip
 * refuses panning, and every touch move until the drag ends is cancelled. */
export function startPointerDrag(event: { readonly button: number; readonly clientX: number; readonly clientY: number; readonly pointerId?: number; readonly currentTarget: Element; preventDefault(): void }, onDrop: (zone: string) => void): void {
  if (event.button !== 0) return;
  event.preventDefault();
  const source = event.currentTarget.closest<HTMLElement>("[data-course-drag]") ?? (event.currentTarget as HTMLElement);
  const grip = event.currentTarget;
  holdPointer(grip, event.pointerId);
  const host = source.closest<HTMLElement>(".course-app") ?? document.body;
  const origin = { x: event.clientX, y: event.clientY };
  const bounds = source.getBoundingClientRect();
  let ghost: HTMLElement | undefined;
  let highlighted: HTMLElement | undefined;
  const highlight = (zone: string | undefined): void => {
    const element = zoneElement(zone, host);
    if (element === highlighted) return;
    highlighted?.classList.remove("course-drop-active");
    element?.classList.add("course-drop-active");
    highlighted = element;
  };
  const move = (moved: PointerEvent): void => {
    if (ghost === undefined) {
      if (Math.hypot(moved.clientX - origin.x, moved.clientY - origin.y) < DRAG_THRESHOLD_PX) return;
      ghost = ghostOf(source);
      host.append(ghost);
      source.classList.add("course-dragging");
    }
    ghost.style.transform = `translate(${bounds.left + moved.clientX - origin.x}px, ${bounds.top + moved.clientY - origin.y}px)`;
    highlight(dropZoneAt(moved.clientX, moved.clientY));
  };
  const blockScroll = (touch: Event): void => {
    if (touch.cancelable) touch.preventDefault();
  };
  const finish = (): void => {
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", release);
    window.removeEventListener("pointercancel", finish);
    window.removeEventListener("keydown", escape);
    window.removeEventListener("touchmove", blockScroll, true);
    releasePointer(grip, event.pointerId);
    highlight(undefined);
    ghost?.remove();
    source.classList.remove("course-dragging");
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
  window.addEventListener("touchmove", blockScroll, { passive: false, capture: true });
}
