/** 🧷️ The overlay scrollbar thumb of a scroller that hides its native scrollbar so no gutter eats the last column of its cards: a glass chip on the right edge of the scroller, between the bands that lie over it, as high as the visible share of the content and at the share of the way the content has scrolled. It lies over the content and takes no room, can be dragged, and is absent while nothing overflows.
 *
 * @see ../../../../../🧫️fixtures/🧷️scroll-thumb/🔣️.json — the geometry as language-neutral vectors
 * @see https://www.w3.org/TR/css-overflow-4/#scrollbar-gutter-property — the gutter a native scrollbar reserves
 */

import { useLevel, windowFrameChipLevel } from "@semio-tech/ui-react/chrome";
import { useEffect, useRef, type CSSProperties, type ReactElement } from "react";

/** 📐️ What the thumb is placed from: the scroller's own measures and the clearance of the bands over it, in pixels. */
export interface ScrollThumbInput {
  readonly scrollTop: number;
  readonly scrollHeight: number;
  readonly clientHeight: number;
  readonly top: number;
  readonly bottom: number;
  readonly minimum: number;
}

/** 📏️ The thumb's height and its distance from the top of the track, or `null` while the content fits. */
export interface ScrollThumbGeometry {
  readonly size: number;
  readonly offset: number;
}

/** 📐️ The thumb of a scroller: the track is the client height without the band clearances, the thumb is the visible share of the content (never below `minimum`, never above the track), and its offset is the scrolled share of the track that is left over. */
export function scrollThumbGeometry(input: ScrollThumbInput): ScrollThumbGeometry | null {
  const { scrollTop, scrollHeight, clientHeight, top, bottom, minimum } = input;
  const track = clientHeight - top - bottom;
  const overflow = scrollHeight - clientHeight;
  if (overflow <= 0 || track <= 0) return null;
  const size = Math.min(track, Math.max(minimum, (track * clientHeight) / scrollHeight));
  const share = Math.min(1, Math.max(0, scrollTop / overflow));
  return { size, offset: (track - size) * share };
}

/** 🎚️ The scroll position that puts the thumb at `offset` of its track: the inverse of {@link scrollThumbGeometry}. */
export function scrollTopForThumb(input: ScrollThumbInput, offset: number): number {
  const geometry = scrollThumbGeometry(input);
  if (geometry === null) return 0;
  const room = input.clientHeight - input.top - input.bottom - geometry.size;
  const overflow = input.scrollHeight - input.clientHeight;
  return room <= 0 ? 0 : Math.min(overflow, Math.max(0, (offset / room) * overflow));
}

const MINIMUM_PX = 24;

const SPACER: CSSProperties = { position: "sticky", top: "var(--course-band-top, 0px)", height: 0, width: "100%", overflow: "visible", pointerEvents: "none", zIndex: 1 };

const THUMB: CSSProperties = { position: "absolute", insetInlineEnd: 0, top: 0, width: "0.5rem", display: "none", pointerEvents: "auto", touchAction: "none", cursor: "default", borderRadius: 0, boxShadow: "none" };

function pixels(value: string): number {
  return Number.parseFloat(value) || 0;
}

/** 🧷️ The overlay thumb, to be the first child of the scroller it belongs to (`overflow: auto`, native scrollbar hidden): a zero-height sticky row holding the thumb, so it stays in the scroller's view and takes no room. */
export function ScrollThumb(): ReactElement {
  const spacer = useRef<HTMLDivElement>(null);
  const thumb = useRef<HTMLDivElement>(null);
  const level = windowFrameChipLevel(useLevel());
  useEffect(() => {
    const row = spacer.current;
    const handle = thumb.current;
    const scroller = row?.parentElement;
    if (!row || !handle || !scroller) return;
    const view = scroller.ownerDocument.defaultView;
    if (!view) return;
    const input = (): ScrollThumbInput => {
      const style = view.getComputedStyle(row);
      const top = pixels(style.top);
      const app = scroller.closest<HTMLElement>(".course-app");
      const bottom = app === null ? 0 : pixels(view.getComputedStyle(app).getPropertyValue("--course-band-bottom"));
      return { scrollTop: scroller.scrollTop, scrollHeight: scroller.scrollHeight, clientHeight: scroller.clientHeight, top, bottom, minimum: MINIMUM_PX };
    };
    let frame = 0;
    const place = (): void => {
      frame = 0;
      const geometry = scrollThumbGeometry(input());
      handle.style.display = geometry === null ? "none" : "block";
      if (geometry === null) return;
      handle.style.height = `${geometry.size}px`;
      handle.style.transform = `translateY(${geometry.offset}px)`;
    };
    const queue = (): void => {
      if (frame === 0) frame = view.requestAnimationFrame(place);
    };
    scroller.addEventListener("scroll", queue, { passive: true });
    const resize = typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(queue);
    resize?.observe(scroller);
    for (const child of Array.from(scroller.children)) if (child !== row) resize?.observe(child);
    const content = new MutationObserver(queue);
    content.observe(scroller, { childList: true });
    place();
    let grip: { readonly pointer: number; readonly y: number; readonly offset: number } | undefined;
    const down = (event: PointerEvent): void => {
      const geometry = scrollThumbGeometry(input());
      if (geometry === null || event.button !== 0) return;
      grip = { pointer: event.pointerId, y: event.clientY, offset: geometry.offset };
      handle.setPointerCapture(event.pointerId);
      event.preventDefault();
    };
    const move = (event: PointerEvent): void => {
      if (grip === undefined || event.pointerId !== grip.pointer) return;
      const next = input();
      scroller.scrollTop = scrollTopForThumb(next, grip.offset + event.clientY - grip.y);
    };
    const up = (event: PointerEvent): void => {
      if (grip === undefined || event.pointerId !== grip.pointer) return;
      grip = undefined;
      handle.releasePointerCapture(event.pointerId);
    };
    handle.addEventListener("pointerdown", down);
    handle.addEventListener("pointermove", move);
    handle.addEventListener("pointerup", up);
    handle.addEventListener("pointercancel", up);
    return () => {
      if (frame !== 0) view.cancelAnimationFrame(frame);
      scroller.removeEventListener("scroll", queue);
      resize?.disconnect();
      content.disconnect();
      handle.removeEventListener("pointerdown", down);
      handle.removeEventListener("pointermove", move);
      handle.removeEventListener("pointerup", up);
      handle.removeEventListener("pointercancel", up);
    };
  }, []);
  return (
    <div ref={spacer} data-course-scroll-thumb-row="" aria-hidden="true" style={SPACER}>
      <div ref={thumb} data-course-scroll-thumb="" data-glass-chip="" data-level={level} className="ui-glass" style={THUMB} />
    </div>
  );
}
