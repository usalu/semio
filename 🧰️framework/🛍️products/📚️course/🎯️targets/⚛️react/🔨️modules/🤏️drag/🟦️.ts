/** 🤏️ Pointer drag and drop for task interactions — mouse, pen and touch through one Pointer Events path.
 *
 * Drop zones are plain elements carrying `data-course-drop="<zone>"`; a drag starts on a grip and detaches its source: the source leaves its place
 * (only its room stays, so nothing jumps under the pointer) and travels with the pointer as an inert glass copy one layer above the page. While it travels every
 * zone of its task is shown as a place to drop it (`data-course-dragging` on the task), the zone under the pointer more strongly, and the zone it is released over is reported.
 * Escape or a cancelled pointer abandons the drag. Every interaction also has a keyboard path in its task component;
 * dragging is the additional pointer path, so the ghost and highlights are hidden from assistive technology.

 * A finger on a grip controls the drag. Holding it at the top or bottom of the scroller brings distant drop zones
 * within reach while keeping the ghost at the finger.
 * @see https://w3c.github.io/pointerevents/#determining-supported-direct-manipulation-behavior
 * @see https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#passive
 */

/** 📏️ Pointer travel in CSS pixels before a press becomes a drag. */
export const DRAG_THRESHOLD_PX = 4;

const SCROLL_EDGE_PX = 72;
const SCROLL_SPEED_PX_PER_MS = 0.9;

function dropElementAt(x: number, y: number): HTMLElement | null {
  const element = typeof document.elementFromPoint === "function" ? document.elementFromPoint(x, y) : null;
  return element?.closest<HTMLElement>("[data-course-drop]") ?? null;
}

/** 🎯️ The drop zone under a viewport point, if any. */
export function dropZoneAt(x: number, y: number): string | undefined {
  return dropElementAt(x, y)?.dataset.courseDrop;
}

function dragScrollports(source: HTMLElement, host: HTMLElement): HTMLElement[] {
  const ports: HTMLElement[] = [];
  for (let element = source.parentElement; element !== null && element !== host; element = element.parentElement) {
    if (/^(auto|scroll)$/u.test(window.getComputedStyle(element).overflowY)) ports.push(element);
  }
  return [...ports, host];
}

function edgeScrollSpeed(host: HTMLElement, y: number): number {
  const remaining = host.scrollHeight - host.clientHeight;
  if (remaining <= 0) return 0;
  const bounds = host.getBoundingClientRect();
  const top = Math.max(0, bounds.top);
  const bottom = Math.min(window.innerHeight, bounds.bottom);
  const edge = Math.min(SCROLL_EDGE_PX, (bottom - top) / 4);
  if (edge <= 0) return 0;
  if (y < top + edge && host.scrollTop > 0) return -SCROLL_SPEED_PX_PER_MS * Math.min(1, (top + edge - y) / edge);
  if (y > bottom - edge && host.scrollTop < remaining) return SCROLL_SPEED_PX_PER_MS * Math.min(1, (y - bottom + edge) / edge);
  return 0;
}

/** 👻️ An inert copy of `source` that looks like it, in an empty copy of the list it lies in: the list is the room its
 * rows lay themselves out for (a container query), so the copy keeps the layout of its source — one line or two —
 * wherever it is carried. Of the list only its classes are kept: a list that is a shared-border group or cell is positioned by the border
 * compositor, which would put the copy back into the flow of the page instead of at the pointer. Cloning a node does not carry over what a select currently shows, so every select of the
 * copy is given the value of the one it copies.
 * @see https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element — no cloning steps
 * @see ../../🎨️.css — `.course-rows` */
function ghostOf(source: HTMLElement): HTMLElement {
  const copy = source.cloneNode(true) as HTMLElement;
  const shown = source.querySelectorAll("select");
  copy.querySelectorAll("select").forEach((select, index) => (select.value = shown[index]!.value));
  copy.removeAttribute("id");
  for (const element of copy.querySelectorAll("[id]")) element.removeAttribute("id");
  copy.style.width = `${source.getBoundingClientRect().width}px`;
  copy.classList.add("ui-glass");
  copy.setAttribute("data-level", "menu");
  copy.setAttribute("data-glass-chip", "");
  const room = source.parentElement;
  const ghost = room === null ? document.createElement("div") : (room.cloneNode(false) as HTMLElement);
  for (const own of ghost.getAttributeNames()) if (own !== "class") ghost.removeAttribute(own);
  ghost.classList.add("course-drag-ghost");
  ghost.setAttribute("aria-hidden", "true");
  ghost.setAttribute("inert", "");
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

/** 🖱️ Starts a pointer drag of the element around `event.currentTarget` (its nearest `[data-course-drag]` ancestor);
 * `onDrop` receives the zone the pointer is released over. The grip refuses native panning during its touch drag.
 * Returns the function that abandons the drag
 * — its listeners, ghost and highlight go, `onDrop` is never called — for whoever must end it before the pointer does. */
export function startPointerDrag(event: { readonly button: number; readonly clientX: number; readonly clientY: number; readonly pointerId?: number; readonly pointerType?: string; readonly currentTarget: Element; preventDefault(): void }, onDrop: (zone: string) => void): () => void {
  if (event.button !== 0) return () => undefined;
  event.preventDefault();
  const source = event.currentTarget.closest<HTMLElement>("[data-course-drag]") ?? (event.currentTarget as HTMLElement);
  const grip = event.currentTarget;
  holdPointer(grip, event.pointerId);
  const host = source.closest<HTMLElement>(".course-app") ?? document.body;
  const places = source.closest<HTMLElement>(".course-task") ?? host;
  const origin = { x: event.clientX, y: event.clientY };
  const bounds = source.getBoundingClientRect();
  const edgeScroll = event.pointerType === "touch" || event.pointerType === "pen";
  const scrollports = edgeScroll ? dragScrollports(source, host) : [];
  let point = origin;
  let ghost: HTMLElement | undefined;
  let highlighted: HTMLElement | null = null;
  let frame: number | undefined;
  let previousFrame: number | undefined;
  let finished = false;
  const owns = (pointer: PointerEvent): boolean => event.pointerId === undefined || event.pointerId === pointer.pointerId;
  const target = (x: number, y: number): HTMLElement | null => {
    const element = dropElementAt(x, y);
    return host.contains(element) ? element : null;
  };
  const highlight = (element: HTMLElement | null): void => {
    if (element === highlighted) return;
    highlighted?.classList.remove("course-drop-active");
    element?.classList.add("course-drop-active");
    highlighted = element;
  };
  const edgeTarget = () => {
    for (const port of scrollports) {
      const speed = edgeScrollSpeed(port, point.y);
      if (speed !== 0) return { port, speed };
    }
    return undefined;
  };
  const scroll = (at: number): void => {
    frame = undefined;
    const active = edgeTarget();
    if (active === undefined) {
      previousFrame = undefined;
      return;
    }
    const elapsed = previousFrame === undefined ? 16 : Math.min(at - previousFrame, 32);
    previousFrame = at;
    const { port, speed } = active;
    const before = port.scrollTop;
    port.scrollTo({ top: before + speed * elapsed, behavior: "instant" });
    highlight(target(point.x, point.y));
    if (port.scrollTop !== before) frame = window.requestAnimationFrame(scroll);
    else previousFrame = undefined;
  };
  const move = (moved: PointerEvent): void => {
    if (!owns(moved)) return;
    if (ghost === undefined) {
      if (Math.hypot(moved.clientX - origin.x, moved.clientY - origin.y) < DRAG_THRESHOLD_PX) return;
      ghost = ghostOf(source);
      host.append(ghost);
      source.classList.add("course-dragging");
      places.setAttribute("data-course-dragging", "");
    }
    point = { x: moved.clientX, y: moved.clientY };
    ghost.style.transform = `translate(${bounds.left + point.x - origin.x}px, ${bounds.top + point.y - origin.y}px)`;
    highlight(target(point.x, point.y));
    if (frame === undefined && edgeTarget() !== undefined) frame = window.requestAnimationFrame(scroll);
  };
  const blockScroll = (touch: Event): void => {
    if (touch.cancelable) touch.preventDefault();
  };
  const finish = (): void => {
    if (finished) return;
    finished = true;
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", release);
    window.removeEventListener("pointercancel", cancel);
    window.removeEventListener("blur", finish);
    grip.removeEventListener("lostpointercapture", cancel);
    window.removeEventListener("keydown", escape);
    grip.removeEventListener("touchmove", blockScroll);
    if (frame !== undefined) window.cancelAnimationFrame(frame);
    releasePointer(grip, event.pointerId);
    highlight(null);
    ghost?.remove();
    source.classList.remove("course-dragging");
    places.removeAttribute("data-course-dragging");
  };
  const cancel = (pointer: Event): void => {
    if (owns(pointer as PointerEvent)) finish();
  };
  const release = (released: PointerEvent): void => {
    if (!owns(released)) return;
    const zone = ghost === undefined ? undefined : target(released.clientX, released.clientY)?.dataset.courseDrop;
    finish();
    if (zone !== undefined) onDrop(zone);
  };
  const escape = (key: KeyboardEvent): void => {
    if (key.key === "Escape") finish();
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", release);
  window.addEventListener("pointercancel", cancel);
  window.addEventListener("blur", finish);
  grip.addEventListener("lostpointercapture", cancel);
  window.addEventListener("keydown", escape);
  if (event.pointerType === "touch") grip.addEventListener("touchmove", blockScroll, { passive: false });
  return finish;
}
