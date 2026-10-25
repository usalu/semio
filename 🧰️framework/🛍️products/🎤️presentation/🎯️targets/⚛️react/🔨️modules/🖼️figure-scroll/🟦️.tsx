/** 🔎️ Figure scroll and wheel-zoom math, the zoom context, the scroll viewport with its overlay bar and the media teaser wrapper.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type {
  DispositionPosition,
  FigureEmbodiment,
  MediaTeaser,
  PdfEmbodiment,
  VideoEmbodiment
} from "@semio-tech/presentation";
// #region 🔌️Adapters
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type ReactNode,
  type RefObject
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { FIGURE_MOSAIC_ALIGN_EPSILON } from "../📐️figure-frames/🟦️.ts";

//#region 🔖️MorphView
//#region 🔖️FigureScroll
/** 📐️ Default slide pixel size from {@link PresentationSlideAspectContext} (960×700 when unknown). */
export function presentationSlidePixelSize(slideAspect?: number): { readonly width: number; readonly height: number } {
  const height = 700;
  return {
    width: slideAspect !== undefined ? slideAspect * height : 960,
    height,
  };
}
/** 🔳️ Estimated frame pixels from normalized disposition (before DOM measure on hidden slides). */
export function estimateDispositionFramePixels(frame: DispositionPosition, slideWidth = 960, slideHeight = 700): { readonly width: number; readonly height: number } {
  return {
    width: frame.width * slideWidth,
    height: frame.height * slideHeight,
  };
}

/** 📜️ True when a figure should use one-axis cover scroll (mosaic tiles always clip). */
export function figureEmbodimentScrollEnabled(embodiment: FigureEmbodiment): boolean {
  if (embodiment.scroll === false || embodiment.mosaic !== undefined) {
    return false;
  }
  return true;
}

/** 🎥️ True when a video should use one-axis cover scroll. */
export function videoEmbodimentScrollEnabled(embodiment: VideoEmbodiment): boolean {
  return embodiment.scroll !== false;
}

/** 📔️ True when a pdf page should use one-axis cover scroll. */
export function pdfEmbodimentScrollEnabled(embodiment: PdfEmbodiment): boolean {
  return embodiment.scroll !== false;
}

/** ✨️ True when a media embodiment should show the glassy teaser veil. */
export function mediaTeaserActive(teaser: MediaTeaser | undefined): teaser is MediaTeaser {
  return teaser !== undefined;
}

export function MediaTeaserWrap({ teaser, children }: { readonly teaser?: MediaTeaser; readonly children: ReactNode }): ReactNode {
  if (!mediaTeaserActive(teaser)) {
    return children;
  }
  return (
    <div className="presentation-media-teaser">
      <div className="presentation-media-teaser__content">{children}</div>
      <div className="presentation-media-teaser__veil" aria-hidden={teaser.label === undefined} {...(teaser.label !== undefined ? { role: "img", "aria-label": teaser.label } : {})}>
        {teaser.label !== undefined ? <span className="presentation-media-teaser__label">{teaser.label}</span> : null}
      </div>
    </div>
  );
}

/** ↔ Which axis overflows under uniform cover (frame vs source aspect). */
export function figureCoverOverflowAxis(frameWidth: number, frameHeight: number, sourceAspect: number): "x" | "y" | null {
  if (frameWidth <= 0 || frameHeight <= 0 || sourceAspect <= 0) {
    return null;
  }
  const frameAspect = frameWidth / frameHeight;
  if (frameAspect > sourceAspect + FIGURE_MOSAIC_ALIGN_EPSILON) {
    return "y";
  }
  if (frameAspect < sourceAspect - FIGURE_MOSAIC_ALIGN_EPSILON) {
    return "x";
  }
  return null;
}

/** 🪄️ Scroll axis implied by a crop background-size (`N% auto` → x, `auto N%` → y). */
export function figureBackgroundSizeScrollAxis(bgSize: string): "x" | "y" | null {
  if (bgSize === "cover" || bgSize === "contain") {
    return null;
  }
  if (/^[\d.]+% auto$/.test(bgSize)) {
    return "x";
  }
  if (/^auto [\d.]+%$/.test(bgSize)) {
    return "y";
  }
  return null;
}

/** ✂️ Inner scroll content size for a crop background-size string. */
export function figureCropScrollContentSize(bgSize: string): CSSProperties {
  const horizontal = bgSize.match(/^([\d.]+)% auto$/);
  if (horizontal) {
    return { width: bgSize, height: "100%", minHeight: "100%" };
  }
  const vertical = bgSize.match(/^auto ([\d.]+)%$/);
  if (vertical) {
    return { width: "100%", minWidth: "100%", height: `${vertical[1]}%` };
  }
  return { width: "100%", height: "100%", minWidth: "100%", minHeight: "100%" };
}

/** 📍️ Scroll offset matching CSS background-position along one axis. */
export function figureScrollOffsetForBackgroundPosition(axis: "x" | "y", positionPercent: number, scrollSize: number, clientSize: number): number {
  const max = Math.max(0, scrollSize - clientSize);
  return (positionPercent / 100) * max;
}

/** 🖼️ Crop background-size on a scroll inner (span one scroll axis). */
export function figureCropScrollBackgroundSize(bgSize: string, scrollAxis?: "x" | "y" | null, zoom = 1): string {
  const axis = figureBackgroundSizeScrollAxis(bgSize) ?? scrollAxis ?? null;
  if (axis === "x") {
    return zoom > 1 ? `${zoom * 100}% auto` : "100% auto";
  }
  if (axis === "y") {
    return zoom > 1 ? `auto ${zoom * 100}%` : "auto 100%";
  }
  return figureBackgroundSizeZoomed(bgSize, zoom);
}

/** 🔍️ Minimum ctrl+wheel zoom (cover baseline). */
export const FIGURE_WHEEL_ZOOM_MIN = 1;

/** 🔭️ Maximum ctrl+wheel zoom multiplier. */
export const FIGURE_WHEEL_ZOOM_MAX = 8;

const FIGURE_WHEEL_ZOOM_FACTOR = 1.1;

/** 🪜️ Next zoom level from a wheel delta and current multiplier. */
export function figureWheelZoomStep(deltaY: number, currentZoom: number): number {
  const factor = deltaY < 0 ? FIGURE_WHEEL_ZOOM_FACTOR : 1 / FIGURE_WHEEL_ZOOM_FACTOR;
  return Math.min(FIGURE_WHEEL_ZOOM_MAX, Math.max(FIGURE_WHEEL_ZOOM_MIN, currentZoom * factor));
}

/** ⚓️ Keep the pointer anchor fixed while zooming scrollable figure content. */
export function figureWheelZoomAdjustScroll(scroller: HTMLElement, clientX: number, clientY: number, prevZoom: number, nextZoom: number): void {
  if (prevZoom <= 0 || nextZoom === prevZoom) {
    return;
  }
  const rect = scroller.getBoundingClientRect();
  const anchorX = clientX - rect.left;
  const anchorY = clientY - rect.top;
  const ratio = nextZoom / prevZoom;
  scroller.scrollLeft = Math.max(0, (scroller.scrollLeft + anchorX) * ratio - anchorX);
  scroller.scrollTop = Math.max(0, (scroller.scrollTop + anchorY) * ratio - anchorY);
}

/** ⚖️ Scale a crop `background-size` string by a zoom multiplier. */
export function figureBackgroundSizeZoomed(bgSize: string, zoom: number): string {
  if (zoom <= 1) {
    return bgSize;
  }
  const horizontal = bgSize.match(/^([\d.]+)% auto$/);
  if (horizontal) {
    return `${Number.parseFloat(horizontal[1]) * zoom}% auto`;
  }
  const vertical = bgSize.match(/^auto ([\d.]+)%$/);
  if (vertical) {
    return `auto ${Number.parseFloat(vertical[1]) * zoom}%`;
  }
  return bgSize;
}

/** 🗜️ Apply ctrl+wheel zoom to cover scroll content sizing. */
export function figureCoverScrollContentSize(portWidth: number, portHeight: number, sourceAspect: number, zoom = 1): { readonly axis: "x" | "y" | "both" | null; readonly style: CSSProperties } {
  const base = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
  if (zoom <= 1) {
    return base;
  }
  if (base.axis === null) {
    const widthPercent = ((sourceAspect * portHeight * zoom) / portWidth) * 100;
    const heightPercent = ((portWidth * zoom) / sourceAspect / portHeight) * 100;
    return {
      axis: "both",
      style: { width: `${widthPercent}%`, height: `${heightPercent}%` },
    };
  }
  if (base.axis === "y") {
    const heightPercent = Number.parseFloat(String(base.style.height)) * zoom;
    return {
      axis: zoom > 1 ? "both" : "y",
      style: {
        width: zoom > 1 ? `${zoom * 100}%` : "100%",
        height: `${heightPercent}%`,
      },
    };
  }
  const widthPercent = Number.parseFloat(String(base.style.width)) * zoom;
  return {
    axis: zoom > 1 ? "both" : "x",
    style: {
      width: `${widthPercent}%`,
      height: zoom > 1 ? `${zoom * 100}%` : "100%",
    },
  };
}

export function figureCoverScrollContentSizeAtZoom(portWidth: number, portHeight: number, sourceAspect: number, _zoom: number): { readonly axis: "x" | "y" | null; readonly style: CSSProperties } {
  const axis = figureCoverOverflowAxis(portWidth, portHeight, sourceAspect);
  if (!axis) {
    return { axis: null, style: { width: "100%", height: "100%" } };
  }
  if (axis === "y") {
    const heightPercent = (portWidth / sourceAspect / portHeight) * 100;
    return { axis, style: { width: "100%", height: `${heightPercent}%` } };
  }
  const widthPercent = ((sourceAspect * portHeight) / portWidth) * 100;
  return { axis, style: { width: `${widthPercent}%`, height: "100%" } };
}

/** 🖌️ Apply ctrl+wheel zoom to img/video cover element sizing. */
export function figureCoverScrollElementStyle(portWidth: number, portHeight: number, sourceAspect: number, zoom = 1): CSSProperties {
  if (zoom <= 1) {
    const { axis } = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
    if (!axis) {
      return { width: "100%", height: "100%" };
    }
    if (axis === "y") {
      return {
        width: "100%",
        height: "auto",
        aspectRatio: String(sourceAspect),
        maxWidth: "none",
        maxHeight: "none",
      };
    }
    return {
      width: "auto",
      height: "100%",
      aspectRatio: String(sourceAspect),
      maxWidth: "none",
      maxHeight: "none",
    };
  }
  const { axis } = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
  if (!axis) {
    return {
      width: `${zoom * 100}%`,
      height: "auto",
      aspectRatio: String(sourceAspect),
      maxWidth: "none",
      maxHeight: "none",
    };
  }
  if (axis === "y") {
    return {
      width: `${zoom * 100}%`,
      height: "auto",
      aspectRatio: String(sourceAspect),
      maxWidth: "none",
      maxHeight: "none",
    };
  }
  return {
    width: "auto",
    height: `${zoom * 100}%`,
    aspectRatio: String(sourceAspect),
    maxWidth: "none",
    maxHeight: "none",
  };
}

const FigureZoomContext = createContext(1);

/** 🔬️ Ctrl+wheel zoom multiplier for the enclosing {@link FigureScrollViewport}. */
export function useFigureZoom(): number {
  return useContext(FigureZoomContext);
}

function figureScrollViewportClass(axis: "x" | "y" | "both" | null): string {
  if (axis === "x") {
    return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-x";
  }
  if (axis === "y") {
    return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-y";
  }
  if (axis === "both") {
    return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-both";
  }
  return "presentation-figure-scroll-viewport";
}

function figureScrollScrollerClass(axis: "x" | "y" | "both" | null): string {
  if (axis === "x") {
    return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-x";
  }
  if (axis === "y") {
    return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-y";
  }
  if (axis === "both") {
    return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-both";
  }
  return "presentation-figure-scroll-scroller";
}

/** 📏️ Overlay scrollbar thumb size and offset from native scroll metrics. */
export function figureScrollOverlayThumbMetrics(clientSize: number, scrollSize: number, scrollPos: number, minThumb = 24): { readonly thumbSize: number; readonly thumbOffset: number; readonly visible: boolean } {
  if (clientSize <= 0 || scrollSize <= clientSize + 1) {
    return { thumbSize: 0, thumbOffset: 0, visible: false };
  }
  const thumbSize = Math.max(minThumb, (clientSize / scrollSize) * clientSize);
  const maxThumbOffset = Math.max(0, clientSize - thumbSize);
  const maxScroll = scrollSize - clientSize;
  const thumbOffset = maxThumbOffset <= 0 || maxScroll <= 0 ? 0 : (scrollPos / maxScroll) * maxThumbOffset;
  return { thumbSize, thumbOffset, visible: true };
}

function observeFigureScrollScrollerLayout(scroller: HTMLElement, onChange: () => void): () => void {
  const observed = new Set<Element>();
  const observeNode = (node: Element): void => {
    if (!(node instanceof HTMLElement) || observed.has(node)) {
      return;
    }
    observed.add(node);
    resizeObserver.observe(node);
  };
  const walk = (root: Element): void => {
    observeNode(root);
    for (const child of root.children) {
      walk(child);
    }
  };
  const resizeObserver = new ResizeObserver(onChange);
  walk(scroller);
  const mutationObserver = new MutationObserver(() => {
    walk(scroller);
    onChange();
  });
  mutationObserver.observe(scroller, { childList: true, subtree: true, attributes: true });
  return () => {
    mutationObserver.disconnect();
    resizeObserver.disconnect();
    observed.clear();
  };
}

function observeFigureScrollScroller(scroller: HTMLElement, onChange: () => void): () => void {
  const stopLayout = observeFigureScrollScrollerLayout(scroller, onChange);
  scroller.addEventListener("scroll", onChange, { passive: true });
  return () => {
    scroller.removeEventListener("scroll", onChange);
    stopLayout();
  };
}

function useFigureScrollOverlayBar(scrollerRef: RefObject<HTMLElement | null>, barRef: RefObject<HTMLDivElement | null>, thumbRef: RefObject<HTMLDivElement | null>, axis: "x" | "y" | null, enabled: boolean): void {
  const syncBar = useCallback((): void => {
    const scroller = scrollerRef.current;
    const bar = barRef.current;
    const thumb = thumbRef.current;
    if (!scroller || !bar || !thumb) {
      return;
    }
    if (!enabled || axis === null) {
      bar.style.display = "none";
      return;
    }
    const clientSize = axis === "y" ? scroller.clientHeight : scroller.clientWidth;
    const scrollSize = axis === "y" ? scroller.scrollHeight : scroller.scrollWidth;
    const scrollPos = axis === "y" ? scroller.scrollTop : scroller.scrollLeft;
    const metrics = figureScrollOverlayThumbMetrics(clientSize, scrollSize, scrollPos);
    if (!metrics.visible) {
      bar.style.display = "none";
      return;
    }
    bar.style.display = "";
    if (axis === "y") {
      thumb.style.width = "";
      thumb.style.height = `${metrics.thumbSize}px`;
      thumb.style.transform = `translateY(${metrics.thumbOffset}px)`;
    } else {
      thumb.style.height = "";
      thumb.style.width = `${metrics.thumbSize}px`;
      thumb.style.transform = `translateX(${metrics.thumbOffset}px)`;
    }
  }, [axis, barRef, enabled, scrollerRef, thumbRef]);
  useLayoutEffect(() => {
    syncBar();
    const scroller = scrollerRef.current;
    if (!scroller) {
      return;
    }
    return observeFigureScrollScroller(scroller, syncBar);
  }, [axis, enabled, scrollerRef, syncBar]);
}

function useFigureScrollViewportSync(viewportRef: RefObject<HTMLElement | null>, axis: "x" | "y" | null, scrollOrigin: { readonly x: number; readonly y: number }, enabled: boolean): void {
  const syncedScrollSizeRef = useRef(0);
  const applyingRef = useRef(false);
  useLayoutEffect(() => {
    syncedScrollSizeRef.current = 0;
    if (!enabled || axis === null) {
      return;
    }
    const viewport = viewportRef.current;
    if (!viewport) {
      return;
    }
    const apply = (): void => {
      if (syncedScrollSizeRef.current === Number.POSITIVE_INFINITY) {
        return;
      }
      const clientSize = axis === "y" ? viewport.clientHeight : viewport.clientWidth;
      const scrollSize = axis === "y" ? viewport.scrollHeight : viewport.scrollWidth;
      if (scrollSize <= clientSize + 1 || scrollSize <= syncedScrollSizeRef.current) {
        return;
      }
      applyingRef.current = true;
      const positionPercent = axis === "x" ? scrollOrigin.x : scrollOrigin.y;
      if (axis === "x") {
        viewport.scrollLeft = figureScrollOffsetForBackgroundPosition("x", positionPercent, scrollSize, clientSize);
      } else {
        viewport.scrollTop = figureScrollOffsetForBackgroundPosition("y", positionPercent, scrollSize, clientSize);
      }
      syncedScrollSizeRef.current = scrollSize;
      applyingRef.current = false;
    };
    const onUserScroll = (): void => {
      if (applyingRef.current) {
        return;
      }
      syncedScrollSizeRef.current = Number.POSITIVE_INFINITY;
    };
    apply();
    viewport.addEventListener("scroll", onUserScroll, { passive: true });
    const stopLayout = observeFigureScrollScrollerLayout(viewport, apply);
    return () => {
      viewport.removeEventListener("scroll", onUserScroll);
      stopLayout();
    };
  }, [axis, enabled, scrollOrigin.x, scrollOrigin.y, viewportRef]);
}

export function FigureScrollViewport({
  enabled,
  axis,
  scrollOrigin,
  slideEpoch,
  className,
  style,
  children,
}: {
  readonly enabled: boolean;
  readonly axis: "x" | "y" | null;
  readonly scrollOrigin: { readonly x: number; readonly y: number };
  readonly slideEpoch?: number;
  readonly className?: string;
  readonly style?: CSSProperties;
  readonly children: ReactNode;
}): ReactNode {
  const [zoom, setZoom] = useState(1);
  const scrollerRef = useRef<HTMLDivElement>(null);
  const pendingZoomAnchorRef = useRef<{
    readonly clientX: number;
    readonly clientY: number;
    readonly prevZoom: number;
    readonly nextZoom: number;
  } | null>(null);
  const barRef = useRef<HTMLDivElement>(null);
  const thumbRef = useRef<HTMLDivElement>(null);
  const thumbDragRef = useRef<{ readonly startPointer: number; readonly startScroll: number } | null>(null);
  const scrollActive = enabled || zoom > 1;
  const scrollAxis: "x" | "y" | "both" | null = zoom > 1 ? "both" : axis;
  const overlayAxis: "x" | "y" | null = scrollAxis === "both" ? axis : scrollAxis;
  useEffect(() => {
    setZoom(1);
  }, [slideEpoch]);
  useLayoutEffect(() => {
    const pending = pendingZoomAnchorRef.current;
    const scroller = scrollerRef.current;
    if (!pending || !scroller) {
      return;
    }
    figureWheelZoomAdjustScroll(scroller, pending.clientX, pending.clientY, pending.prevZoom, pending.nextZoom);
    pendingZoomAnchorRef.current = null;
  }, [zoom]);
  useFigureScrollViewportSync(scrollerRef, overlayAxis, scrollOrigin, scrollActive && overlayAxis !== null);
  useFigureScrollOverlayBar(scrollerRef, barRef, thumbRef, overlayAxis, scrollActive && overlayAxis !== null);
  const onWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (event.ctrlKey) {
        event.preventDefault();
        event.stopPropagation();
        const nextZoom = figureWheelZoomStep(event.deltaY, zoom);
        if (nextZoom === zoom) {
          return;
        }
        pendingZoomAnchorRef.current = {
          clientX: event.clientX,
          clientY: event.clientY,
          prevZoom: zoom,
          nextZoom,
        };
        const scroller = scrollerRef.current;
        if (scroller) {
          figureWheelZoomAdjustScroll(scroller, event.clientX, event.clientY, zoom, nextZoom);
          pendingZoomAnchorRef.current = null;
        }
        setZoom(nextZoom);
        return;
      }
      if (!scrollActive || scrollAxis === null || scrollAxis === "both") {
        return;
      }
      const viewport = event.currentTarget;
      const delta = scrollAxis === "x" ? (Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY) : event.deltaY;
      if (delta === 0) {
        return;
      }
      const max = scrollAxis === "x" ? viewport.scrollWidth - viewport.clientWidth : viewport.scrollHeight - viewport.clientHeight;
      if (max <= 0) {
        return;
      }
      const next = scrollAxis === "x" ? viewport.scrollLeft + delta : viewport.scrollTop + delta;
      if (next > 0 && next < max) {
        event.stopPropagation();
      }
    },
    [scrollActive, scrollAxis, zoom],
  );
  const onThumbPointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const scroller = scrollerRef.current;
      const bar = barRef.current;
      if (!scrollActive || overlayAxis === null || scroller === null) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      event.currentTarget.setPointerCapture(event.pointerId);
      bar?.classList.add("presentation-figure-scroll-bar--dragging");
      thumbDragRef.current = {
        startPointer: overlayAxis === "y" ? event.clientY : event.clientX,
        startScroll: overlayAxis === "y" ? scroller.scrollTop : scroller.scrollLeft,
      };
    },
    [overlayAxis, barRef, scrollActive],
  );
  const onThumbPointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const drag = thumbDragRef.current;
      const scroller = scrollerRef.current;
      if (!scrollActive || overlayAxis === null || drag === null || scroller === null) {
        return;
      }
      const clientSize = overlayAxis === "y" ? scroller.clientHeight : scroller.clientWidth;
      const scrollSize = overlayAxis === "y" ? scroller.scrollHeight : scroller.scrollWidth;
      const maxScroll = scrollSize - clientSize;
      const metrics = figureScrollOverlayThumbMetrics(clientSize, scrollSize, overlayAxis === "y" ? scroller.scrollTop : scroller.scrollLeft);
      const maxThumbOffset = Math.max(0, clientSize - metrics.thumbSize);
      if (maxThumbOffset <= 0 || maxScroll <= 0) {
        return;
      }
      const pointer = overlayAxis === "y" ? event.clientY : event.clientX;
      const delta = pointer - drag.startPointer;
      const nextScroll = Math.max(0, Math.min(maxScroll, drag.startScroll + (delta / maxThumbOffset) * maxScroll));
      if (overlayAxis === "y") {
        scroller.scrollTop = nextScroll;
      } else {
        scroller.scrollLeft = nextScroll;
      }
    },
    [overlayAxis, scrollActive],
  );
  const onThumbPointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      thumbDragRef.current = null;
      barRef.current?.classList.remove("presentation-figure-scroll-bar--dragging");
      if (event.currentTarget.hasPointerCapture(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
    },
    [barRef],
  );
  if (!scrollActive) {
    return (
      <FigureZoomContext.Provider value={zoom}>
        <div className={className} style={style} onWheel={onWheel}>
          {children}
        </div>
      </FigureZoomContext.Provider>
    );
  }
  return (
    <FigureZoomContext.Provider value={zoom}>
      <div className={[figureScrollViewportClass(scrollAxis), "presentation-figure-scroll-viewport--overlay", className].filter(Boolean).join(" ")} style={style}>
        <div ref={scrollerRef} className={figureScrollScrollerClass(scrollAxis)} onWheel={onWheel}>
          {children}
        </div>
        {overlayAxis !== null ? (
          <div ref={barRef} className={`presentation-figure-scroll-bar presentation-figure-scroll-bar--axis-${overlayAxis}`} style={{ display: "none" }} aria-hidden="true">
            <div ref={thumbRef} className="presentation-figure-scroll-bar-thumb" onPointerDown={onThumbPointerDown} onPointerMove={onThumbPointerMove} onPointerUp={onThumbPointerUp} onPointerCancel={onThumbPointerUp} />
          </div>
        ) : null}
      </div>
    </FigureZoomContext.Provider>
  );
}
//#endregion 🔖️FigureScroll
//#endregion 🔖️MorphView
