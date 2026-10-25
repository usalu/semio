/** 📑️ The pdf.js loader, the default pdf canvas port and its ephemeral box, pdf page navigation and cover scale math and the pdf container size hook.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
// #endregion 🔌️Adapters
import type { DispositionPosition, PdfEmbodiment } from "@semio-tech/presentation";
import { useEffect, useState, type RefObject } from "@semio-tech/ui-react";
import { type PdfCanvasLoadingTask, type PdfCanvasPort } from "../🔌️pdf-canvas-port/🟦️.ts";
import { parsePresentationSlideCssSize } from "../🏷️morph-context/🟦️.ts";
import { figureCoverOverflowAxis } from "../🖼️figure-scroll/🟦️.tsx";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME } from "../🧮️interaction-model/🟦️.ts";

//#region 🔖️MorphView
//#region 🔖️SlideEpoch
//#region 🔌️PdfCanvasPort
interface PdfJsModule {
  readonly GlobalWorkerOptions: { workerSrc: string };
  getDocument(source: string): PdfCanvasLoadingTask;
}

const pdfWorkerReady = ephemeralBox<Promise<void> | null>("framework.products.presentation.targets.react.pdfWorkerReady", null);

async function loadPdfJsModule(): Promise<PdfJsModule> {
  const module = (await import("pdfjs-dist")) as unknown as PdfJsModule;
  if (!pdfWorkerReady.current) {
    pdfWorkerReady.current = Promise.resolve().then(() => {
      module.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
    });
  }
  await pdfWorkerReady.current;
  return module;
}

const defaultPdfCanvasPort: PdfCanvasPort = {
  load(source) {
    let disposed = false;
    let externalTask: PdfCanvasLoadingTask | null = null;
    const promise = loadPdfJsModule().then(async (module) => {
      if (disposed) {
        throw new Error("PDF load cancelled");
      }
      externalTask = module.getDocument(source);
      if (disposed) {
        await externalTask.destroy();
        throw new Error("PDF load cancelled");
      }
      return externalTask.promise;
    });
    return {
      promise,
      destroy() {
        disposed = true;
        return externalTask?.destroy();
      },
    };
  },
};

export const pdfCanvasPort = ephemeralBox<PdfCanvasPort>("framework.products.presentation.targets.react.pdfCanvasPort", defaultPdfCanvasPort);

/** 🧪️ Replaces the PDF canvas implementation through the owned port. */
export function setPdfCanvasPort(port: PdfCanvasPort): void {
  pdfCanvasPort.current = port;
}

/** 🧹️ Restores the production PDF.js canvas implementation. */
export function resetPdfCanvasPort(): void {
  pdfCanvasPort.current = defaultPdfCanvasPort;
}
//#endregion 🔌️PdfCanvasPort

/** 📐️ Uniform PDF canvas scale so the page covers the disposition frame without distortion. */
export function pdfCoverScale(containerWidth: number, containerHeight: number, pageWidth: number, pageHeight: number): number | null {
  if (containerWidth <= 0 || containerHeight <= 0 || pageWidth <= 0 || pageHeight <= 0) {
    return null;
  }
  return Math.max(containerWidth / pageWidth, containerHeight / pageHeight);
}

/** ⚖️ Uniform PDF canvas scale for one-axis cover scroll (fit width or height, overflow the other). */
export function pdfScrollCoverScale(containerWidth: number, containerHeight: number, pageWidth: number, pageHeight: number): number | null {
  if (containerWidth <= 0 || containerHeight <= 0 || pageWidth <= 0 || pageHeight <= 0) {
    return null;
  }
  const pageAspect = pageWidth / pageHeight;
  const axis = figureCoverOverflowAxis(containerWidth, containerHeight, pageAspect);
  if (axis === "y") {
    return containerWidth / pageWidth;
  }
  if (axis === "x") {
    return containerHeight / pageHeight;
  }
  return Math.min(containerWidth / pageWidth, containerHeight / pageHeight);
}

/** 📋️ Ordered PDF page numbers for navigation; empty means all document pages. */
export function pdfEmbodimentPageList(embodiment: PdfEmbodiment): readonly number[] {
  return embodiment.pages ?? [];
}

/** 🆕️ Starting page for a pdf embodiment (subset or single page). */
export function pdfEmbodimentInitialPage(embodiment: PdfEmbodiment): number {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    const declared = embodiment.page ?? pages[0];
    return pages.includes(declared) ? declared : pages[0];
  }
  return embodiment.page ?? 1;
}

/** 🧭️ Whether pdf prev/next controls apply for this embodiment. */
export function pdfPageNavEnabled(embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 1) {
    return true;
  }
  return pages.length === 0 && numPages !== null && numPages > 1;
}

/** ◀️ True when pdf page nav can move to an earlier page. */
export function pdfCanGoToPreviousPage(currentPage: number, embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    return pages.indexOf(currentPage) > 0;
  }
  return currentPage > 1;
}

/** ▶️ True when pdf page nav can move to a later page. */
export function pdfCanGoToNextPage(currentPage: number, embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    const index = pages.indexOf(currentPage);
    return index >= 0 && index < pages.length - 1;
  }
  return numPages !== null && currentPage < numPages;
}

/** 📄️ Target page for prev/next within a subset or the full document. */
export function pdfAdjacentPage(currentPage: number, direction: "prev" | "next", embodiment: PdfEmbodiment, numPages: number | null): number {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    let index = pages.indexOf(currentPage);
    if (index < 0) {
      index = direction === "next" ? -1 : pages.length;
    }
    const nextIndex = direction === "prev" ? index - 1 : index + 1;
    return pages[nextIndex] ?? currentPage;
  }
  const step = direction === "prev" ? -1 : 1;
  const next = currentPage + step;
  if (direction === "prev" && next < 1) {
    return currentPage;
  }
  if (direction === "next" && numPages !== null && next > numPages) {
    return currentPage;
  }
  return next;
}

/** 📦️ Measures the PDF canvas viewport from the disposition frame or enlarged slide content box. */
export function usePdfContainerSize(anchorRef: RefObject<HTMLDivElement | null>, position: DispositionPosition | undefined, slideEpoch: number, enlarged: boolean): { readonly width?: number; readonly height?: number } {
  const [size, setSize] = useState<{ readonly width?: number; readonly height?: number }>({});
  useEffect(() => {
    const el = anchorRef.current;
    if (!el) {
      return;
    }
    const measureTarget = (): HTMLElement | null => {
      if (enlarged) {
        return el.closest(".presentation-interactive-disposition--enlarged")?.querySelector<HTMLElement>(".presentation-interactive-disposition__content") ?? null;
      }
      const frame = el.closest<HTMLElement>(".presentation-disposition-frame");
      const scrollViewport = frame?.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
      return scrollViewport ?? frame;
    };
    const measure = (): void => {
      const target = measureTarget();
      if (target && target.clientWidth > 8 && target.clientHeight > 8) {
        setSize({
          width: target.clientWidth,
          height: target.clientHeight,
        });
        return;
      }
      const slide = parsePresentationSlideCssSize(el.closest(".reveal"));
      const frame = enlarged ? SLIDE_INTERACTIVE_ENLARGE_FRAME : position;
      const width = frame ? Math.floor(slide.width * frame.width) : Math.floor(slide.width * 0.8);
      const height = frame ? Math.floor(slide.height * frame.height) : Math.floor(slide.height * 0.4);
      if (width > 0 && height > 0) {
        setSize({ width, height });
      }
    };
    measure();
    const observed = measureTarget();
    const observer = observed && typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
    if (observed && observer) {
      observer.observe(observed);
    }
    const visibility =
      typeof IntersectionObserver !== "undefined"
        ? new IntersectionObserver((entries) => {
            if (entries[0]?.isIntersecting) {
              measure();
            }
          })
        : null;
    visibility?.observe(el);
    return () => {
      observer?.disconnect();
      visibility?.disconnect();
    };
  }, [anchorRef, enlarged, position?.height, position?.width, position?.x, position?.y, slideEpoch]);
  return size;
}
//#endregion 🔖️SlideEpoch
//#endregion 🔖️MorphView
