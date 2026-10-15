/** 🔍️ Zoomed figure bodies: image, video, pdf page and crop scroll content.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  useEffect,
  useRef,
  useState,
  type CSSProperties,
  type ReactNode
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import {
  PdfCanvasResourceOwner,
  pdfCanvasBitmapSize,
  pdfCanvasStatusAnnouncement,
  type PdfCanvasDocument,
  type PdfCanvasPage,
  type PdfCanvasStatus
} from "../🔌️pdf-canvas-port/🟦️.ts";
import { pdfCanvasPort } from "../📑️pdf-canvas-view/🟦️.ts";
import { resolvePresentationAssetUrl } from "../📐️figure-frames/🟦️.ts";
import {
  figureBackgroundSizeScrollAxis,
  figureBackgroundSizeZoomed,
  figureCoverOverflowAxis,
  figureCoverScrollContentSize,
  figureCoverScrollElementStyle,
  figureCropScrollBackgroundSize,
  figureCropScrollContentSize,
  useFigureZoom
} from "../🖼️figure-scroll/🟦️.tsx";

//#region 🔖️MorphView
export function FigureZoomedImage({
  className,
  portWidth,
  portHeight,
  sourceAspect,
  scrollEnabled,
  src,
  alt,
  onLoad,
}: {
  readonly className: string;
  readonly portWidth: number;
  readonly portHeight: number;
  readonly sourceAspect: number;
  readonly scrollEnabled: boolean;
  readonly src: string;
  readonly alt: string;
  readonly onLoad: (event: React.SyntheticEvent<HTMLImageElement>) => void;
}): ReactNode {
  const zoom = useFigureZoom();
  const elementStyle = scrollEnabled
    ? figureCoverScrollElementStyle(portWidth, portHeight, sourceAspect, zoom)
    : zoom > 1
      ? {
          width: `${zoom * 100}%`,
          height: "auto",
          aspectRatio: String(sourceAspect),
          maxWidth: "none",
          maxHeight: "none",
        }
      : undefined;
  const mergedClassName = [className, scrollEnabled || zoom > 1 ? "presentation-figure-scroll-media" : undefined].filter(Boolean).join(" ");
  return <img className={mergedClassName} style={elementStyle} src={src} alt={alt} onLoad={onLoad} />;
}

export function FigureZoomedVideo({
  className,
  portWidth,
  portHeight,
  sourceAspect,
  scrollEnabled,
  src,
  poster,
  loop,
  muted,
  controls,
  onLoadedMetadata,
}: {
  readonly className: string;
  readonly portWidth: number;
  readonly portHeight: number;
  readonly sourceAspect: number;
  readonly scrollEnabled: boolean;
  readonly src: string;
  readonly poster?: string;
  readonly loop?: boolean;
  readonly muted?: boolean;
  readonly controls?: boolean;
  readonly onLoadedMetadata: (event: React.SyntheticEvent<HTMLVideoElement>) => void;
}): ReactNode {
  const zoom = useFigureZoom();
  const elementStyle = scrollEnabled
    ? figureCoverScrollElementStyle(portWidth, portHeight, sourceAspect, zoom)
    : zoom > 1
      ? {
          width: `${zoom * 100}%`,
          height: "auto",
          aspectRatio: String(sourceAspect),
          maxWidth: "none",
          maxHeight: "none",
        }
      : undefined;
  const mergedClassName = [className, scrollEnabled || zoom > 1 ? "presentation-figure-scroll-media" : undefined].filter(Boolean).join(" ");
  return <video className={mergedClassName} style={elementStyle} src={src} poster={poster} autoPlay={false} loop={loop ?? false} muted={muted ?? true} controls={controls ?? true} playsInline preload="metadata" onLoadedMetadata={onLoadedMetadata} />;
}

export function FigureZoomedPdfPage({
  source,
  currentPage,
  coverScale,
  ready,
  ariaLabel,
  onDocumentLoadSuccess,
  onPageLoadSuccess,
}: {
  readonly source: string;
  readonly currentPage: number;
  readonly coverScale: number | null;
  readonly ready: boolean;
  readonly ariaLabel: string;
  readonly onDocumentLoadSuccess: (document: { readonly numPages: number }) => void;
  readonly onPageLoadSuccess: (page: PdfCanvasPage) => void;
}): ReactNode {
  const zoom = useFigureZoom();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const resourceOwnerRef = useRef<PdfCanvasResourceOwner | null>(null);
  resourceOwnerRef.current ??= new PdfCanvasResourceOwner();
  const [documentState, setDocumentState] = useState<{
    readonly source: string;
    readonly document: PdfCanvasDocument;
  } | null>(null);
  const [status, setStatus] = useState<PdfCanvasStatus>("loading");
  useEffect(() => {
    let active = true;
    const owner = resourceOwnerRef.current!;
    setDocumentState(null);
    setStatus("loading");
    const loadingTask = pdfCanvasPort.current.load(resolvePresentationAssetUrl(source));
    owner.beginLoad(loadingTask);
    void loadingTask.promise.then(
      (document) => {
        if (!active || !owner.acceptDocument(loadingTask, document)) {
          void document.destroy();
          return;
        }
        setDocumentState({ source, document });
        onDocumentLoadSuccess({ numPages: document.numPages });
      },
      () => {
        if (active) {
          setStatus("error");
        }
      },
    );
    return () => {
      active = false;
      owner.disposeDocument();
    };
  }, [onDocumentLoadSuccess, source]);
  useEffect(() => {
    const document = documentState?.source === source ? documentState.document : null;
    if (!document || !ready) {
      return;
    }
    let active = true;
    const owner = resourceOwnerRef.current!;
    owner.beginPage();
    setStatus("loading");
    void document
      .getPage(currentPage)
      .then(async (page) => {
        if (!active) {
          page.cleanup();
          return;
        }
        owner.acceptPage(page);
        onPageLoadSuccess(page);
        const canvas = canvasRef.current;
        const context = canvas?.getContext("2d") ?? null;
        if (!canvas || !context) {
          throw new Error("PDF canvas 2d context unavailable");
        }
        const scale = (coverScale ?? 1) * zoom;
        const pixelRatio = Math.max(1, globalThis.devicePixelRatio || 1);
        const viewport = page.getViewport({ scale });
        const renderViewport = page.getViewport({ scale: scale * pixelRatio });
        const bitmap = pdfCanvasBitmapSize(viewport, pixelRatio);
        canvas.width = bitmap.width;
        canvas.height = bitmap.height;
        canvas.style.width = `${viewport.width}px`;
        canvas.style.height = `${viewport.height}px`;
        canvas.dataset.page = String(currentPage);
        canvas.dataset.scale = String(scale);
        const renderTask = page.render({ canvas, canvasContext: context, viewport: renderViewport });
        owner.acceptRender(renderTask);
        await renderTask.promise;
        if (active) {
          setStatus("ready");
        }
      })
      .catch(() => {
        if (active) {
          setStatus("error");
        }
      });
    return () => {
      active = false;
      owner.disposePage();
    };
  }, [coverScale, currentPage, documentState, onPageLoadSuccess, ready, source, zoom]);
  const announcement = pdfCanvasStatusAnnouncement(status);
  return (
    <div className="presentation-media-pdf" data-pdf-status={status}>
      {announcement ? (
        <span className={`presentation-media-pdf-${status}`} role={announcement.role} aria-live={announcement.role === "status" ? "polite" : undefined}>
          {announcement.text}
        </span>
      ) : null}
      <canvas ref={canvasRef} role="img" aria-label={ariaLabel} style={{ visibility: status === "ready" ? "visible" : "hidden" }} />
    </div>
  );
}

export function FigureZoomedCropScrollContent({
  className,
  portWidth,
  portHeight,
  sourceAspect,
  backgroundSize,
  backgroundPosition,
  backgroundVars,
  ariaLabel,
  morphCropData,
}: {
  readonly className: string;
  readonly portWidth: number;
  readonly portHeight: number;
  readonly sourceAspect: number;
  readonly backgroundSize: string;
  readonly backgroundPosition: string;
  readonly backgroundVars: CSSProperties;
  readonly ariaLabel: string;
  readonly morphCropData?: string;
}): ReactNode {
  const zoom = useFigureZoom();
  const scrollContentStyle: CSSProperties = {
    ...(backgroundSize === "cover" ? figureCoverScrollContentSize(portWidth, portHeight, sourceAspect, zoom).style : figureCropScrollContentSize(figureBackgroundSizeZoomed(backgroundSize, zoom))),
    backgroundImage: backgroundVars.backgroundImage,
    ["--presentation-figure-bg-size" as string]: figureCropScrollBackgroundSize(backgroundSize, figureBackgroundSizeScrollAxis(backgroundSize) ?? figureCoverOverflowAxis(portWidth, portHeight, sourceAspect), zoom),
    ["--presentation-figure-bg-position" as string]: backgroundPosition,
    ["--presentation-figure-bg-size-morph" as string]: backgroundVars["--presentation-figure-bg-size-morph" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-position-morph" as string]: backgroundVars["--presentation-figure-bg-position-morph" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-grid-size" as string]: backgroundVars["--presentation-figure-bg-grid-size" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-grid-position" as string]: backgroundVars["--presentation-figure-bg-grid-position" as keyof typeof backgroundVars],
  };
  return <div className={className} style={scrollContentStyle} {...(morphCropData !== undefined ? { "data-presentation-morph-crop": morphCropData } : {})} role="img" aria-label={ariaLabel} />;
}
//#endregion 🔖️MorphView
