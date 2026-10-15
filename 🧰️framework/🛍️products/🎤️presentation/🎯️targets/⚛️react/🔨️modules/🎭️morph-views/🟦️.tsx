/** 🎭️ Per-embodiment morph views: text, authors, affiliations, bullets, figures, video, iframe, markdown, json and pdf, and the disposition frame.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type {
  AffiliationEntry,
  AffiliationsEmbodiment,
  AuthorPerson,
  AuthorsEmbodiment,
  BulletEmbodiment,
  DispositionPosition,
  DispositionStyle,
  FigureEmbodiment,
  IframeEmbodiment,
  JsonEmbodiment,
  MarkdownEmbodiment,
  ParticipantEmphasis,
  PdfEmbodiment,
  TextEmbodiment,
  VideoEmbodiment
} from "@semio-tech/presentation";
import { abbreviateAuthorFirstName, affiliationLineName, resolveMediaScrollOrigin, resolveTextMorphRoot } from "@semio-tech/presentation";
// #region 🔌️Adapters
import {
  Scrollable,
  Fragment,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { type PdfCanvasPage } from "../🔌️pdf-canvas-port/🟦️.ts";
import { compileMarkdownToHtml } from "../🎚️deck-chrome/🟦️.ts";
import { type RevealMorphCompanionKind, type RevealResolvedDisposition } from "../🔀️reveal-morph/🟦️.ts";
import {
  emphasisClass,
  morphAnchorClass,
  MorphAnchorOnWrapperContext,
  morphTextClass,
  PresentationDispositionEnlargeContext,
  PresentationFigureCropFrameContext,
  PresentationInteractiveDispositionIdContext,
  PresentationSlideAspectContext,
  PresentationSlideEpochContext
} from "../🏷️morph-context/🟦️.ts";
import {
  pdfAdjacentPage,
  pdfCanGoToNextPage,
  pdfCanGoToPreviousPage,
  pdfCoverScale,
  pdfEmbodimentInitialPage,
  pdfPageNavEnabled,
  pdfScrollCoverScale,
  usePdfContainerSize
} from "../📑️pdf-canvas-view/🟦️.ts";
import { figureCropBackgroundVars, figureCropBackgroundVarsTargetGhost, resolvePresentationAssetUrl } from "../📐️figure-frames/🟦️.ts";
import {
  estimateDispositionFramePixels,
  figureBackgroundSizeScrollAxis,
  figureCoverOverflowAxis,
  figureCoverScrollContentSizeAtZoom,
  figureEmbodimentScrollEnabled,
  FigureScrollViewport,
  MediaTeaserWrap,
  pdfEmbodimentScrollEnabled,
  presentationSlidePixelSize,
  videoEmbodimentScrollEnabled
} from "../🖼️figure-scroll/🟦️.tsx";
import { FigureZoomedCropScrollContent, FigureZoomedImage, FigureZoomedPdfPage, FigureZoomedVideo } from "../🔍️figure-zoom/🟦️.tsx";
import { PresentationInteractionContext } from "../🧮️interaction-model/🟦️.ts";
import { renderJsonTree } from "../🌳️json-tree/🟦️.tsx";

//#region 🔖️MorphView
function lineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string | undefined {
  return [morphTextClass(morphId), embodiment.fit ? "r-fit-text" : undefined, emphasisClass(emphasis)].filter(Boolean).join(" ") || undefined;
}

function centeredLineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string {
  return [lineClass(morphId, embodiment, emphasis), "text-center"].filter(Boolean).join(" ");
}

/** 🎯️ Renders {@link TextEmbodiment}; `data-id` on leaf text unless the intro wrapper owns the morph anchor. */
/** 🎯️ Stable reveal.js `data-id` for one text line; single-line blocks use the morph id, multi-line use `--index`. */
function textMorphAnchorId(anchorId: string, lineIndex: number, lineCount: number, _root: ReturnType<typeof resolveTextMorphRoot>): string {
  return lineCount === 1 ? anchorId : `${anchorId}--${lineIndex}`;
}

function morphLeafDataId(anchorOnWrapper: boolean, id: string): { readonly "data-id"?: string } {
  return anchorOnWrapper ? {} : { "data-id": id };
}

function TextMorphView({ morphId: anchorId, embodiment, emphasis, anchorOnWrapper = false }: { readonly morphId: string; readonly embodiment: TextEmbodiment; readonly emphasis: ParticipantEmphasis; readonly anchorOnWrapper?: boolean }): ReactNode {
  const root = resolveTextMorphRoot(embodiment);
  const centeredHeadingClass = centeredLineClass(anchorId, embodiment, emphasis);
  const lineCount = embodiment.lines.length;

  switch (root) {
    case "title":
      return (
        <h1 {...morphLeafDataId(anchorOnWrapper, anchorId)} className={centeredLineClass(anchorId, embodiment, emphasis)}>
          {embodiment.lines[0]}
        </h1>
      );
    case "body":
      return (
        <div className="w-full text-center">
          {embodiment.lines.map((line, lineIndex) => (
            <p
              key={`${anchorId}--${lineIndex}`}
              {...morphLeafDataId(anchorOnWrapper, textMorphAnchorId(anchorId, lineIndex, lineCount, root))}
              className={[lineClass(anchorId, embodiment, emphasis), "text-center"].filter(Boolean).join(" ") || "text-center"}
            >
              {line}
            </p>
          ))}
        </div>
      );
    case "heading-line":
    case "subheading-line":
      return (
        <h2 {...morphLeafDataId(anchorOnWrapper, anchorId)} className={centeredHeadingClass}>
          {embodiment.lines[0]}
        </h2>
      );
    case "heading-block":
      return (
        <div className="w-full text-center">
          {embodiment.lines.map((line, lineIndex) => (
            <h2 key={`${anchorId}--${lineIndex}`} {...morphLeafDataId(anchorOnWrapper, textMorphAnchorId(anchorId, lineIndex, lineCount, root))} className={centeredHeadingClass}>
              {line}
            </h2>
          ))}
        </div>
      );
    default: {
      const _exhaustive: never = root;
      return _exhaustive;
    }
  }
}

function authorRows(embodiment: AuthorsEmbodiment): readonly (readonly AuthorPerson[])[] {
  if (embodiment.lines && embodiment.lines.length > 0) {
    return embodiment.lines;
  }
  if (embodiment.people && embodiment.people.length > 0) {
    return [embodiment.people];
  }
  return [];
}

function authorDisplayName(person: AuthorPerson, embodiment: AuthorsEmbodiment): string {
  return embodiment.abbreviateFirstName ? abbreviateAuthorFirstName(person.name) : person.name;
}

function AuthorsMorphView({
  morphId: anchorId,
  embodiment,
  emphasis,
  anchorOnWrapper = false,
}: {
  readonly morphId: string;
  readonly embodiment: AuthorsEmbodiment;
  readonly emphasis: ParticipantEmphasis;
  readonly anchorOnWrapper?: boolean;
}): ReactNode {
  const namesMuted = embodiment.abbreviateFirstName === true || emphasis === "muted";
  const rows = authorRows(embodiment);
  return (
    <div className="presentation-intro-rows presentation-intro-authors flex w-full max-w-full flex-col items-center text-center">
      {rows.map((line, lineIndex) => (
        <div key={`${anchorId}-line-${lineIndex}`} className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]">
          {line.map((person) => {
            const displayName = authorDisplayName(person, embodiment);
            return (
              <h4 key={person.name} {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${person.name}`)} className={morphTextClass(anchorId, "m-0 shrink-0 text-center")}>
                {namesMuted ? <span className="opacity-20">{displayName}</span> : displayName}
                {person.markEntries && person.markEntries.length > 0 ? (
                  <sup className="ms-[0.35em]">
                    {person.markEntries.map((entry, markIndex) => (
                      <Fragment key={entry.mark}>
                        {markIndex > 0 ? <span className="opacity-20">,</span> : null}
                        <span className={emphasisClass(entry.emphasis)}>{entry.mark}</span>
                      </Fragment>
                    ))}
                  </sup>
                ) : person.marks && person.marks.length > 0 ? (
                  <sup className="ms-[0.35em]">{person.marks.join(",")}</sup>
                ) : null}
              </h4>
            );
          })}
        </div>
      ))}
    </div>
  );
}

function affiliationLineLabel(entry: AffiliationEntry, part: "line" | "suffix"): string {
  if (part === "suffix" && entry.suffix) {
    return entry.suffix.name;
  }
  return affiliationLineName(entry);
}

function affiliationLineMuted(partEmphasis: ParticipantEmphasis | undefined): boolean {
  return partEmphasis === "muted";
}

function affiliationLineContent(entry: AffiliationEntry, part: "line" | "suffix", label: string): ReactNode {
  const muted = part === "suffix" ? affiliationLineMuted(entry.suffixEmphasis) : affiliationLineMuted(entry.lineEmphasis);
  const mark = part === "suffix" && entry.suffix ? entry.suffix.mark : entry.mark;
  if (muted) {
    return (
      <span className="opacity-20">
        <sup>{mark}</sup>
        {label}
      </span>
    );
  }
  return (
    <>
      <sup>{mark}</sup>
      {label}
    </>
  );
}

function AffiliationsMorphView({ morphId: anchorId, embodiment, anchorOnWrapper = false }: { readonly morphId: string; readonly embodiment: AffiliationsEmbodiment; readonly anchorOnWrapper?: boolean }): ReactNode {
  const rowClass = morphTextClass(anchorId, "presentation-affiliation-row m-0 inline-flex max-w-full shrink-0 flex-row flex-nowrap items-center justify-center gap-x-[0.35em] text-center");
  return (
    <div className="presentation-intro-rows presentation-intro-affiliations flex w-full max-w-full flex-col items-center text-center">
      {embodiment.entries.map((entry) => {
        const displayLabel = affiliationLineLabel(entry, "line");
        return (
          <div key={entry.mark} className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]">
            <h4 {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.mark}`)} className={rowClass}>
              {affiliationLineContent(entry, "line", displayLabel)}
              {entry.suffix ? (
                <span {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.suffix.mark}`)} className="inline-flex shrink-0 items-center justify-center text-center">
                  {affiliationLineContent(entry, "suffix", affiliationLineLabel(entry, "suffix"))}
                </span>
              ) : null}
            </h4>
          </div>
        );
      })}
    </div>
  );
}

function BulletMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: BulletEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  return (
    <div data-id={anchorId} className={emphasisClass(emphasis)}>
      <ul className={morphTextClass(anchorId)}>
        {embodiment.items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </div>
  );
}

function FigureImageMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: FigureEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
  const scrollEnabled = figureEmbodimentScrollEnabled(embodiment);
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const slideAspect = useContext(PresentationSlideAspectContext);
  const anchorRef = useRef<HTMLDivElement>(null);
  const [portSize, setPortSize] = useState({ width: 0, height: 0 });
  const [sourceAspect, setSourceAspect] = useState(embodiment.sourceAspect ?? 1);
  useLayoutEffect(() => {
    const anchor = anchorRef.current;
    if (!anchor) {
      return;
    }
    const measure = (): void => {
      const viewport = anchor.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
      const target = viewport ?? anchor;
      setPortSize({ width: target.clientWidth, height: target.clientHeight });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(anchor);
    return () => observer.disconnect();
  }, [slideEpoch]);
  const estimatedPortSize = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
  }, [position, slideAspect]);
  const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
  const coverScroll = figureCoverScrollContentSizeAtZoom(effectivePortSize.width, effectivePortSize.height, sourceAspect, 1);
  const axis = coverScroll.axis;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  return (
    <div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <FigureScrollViewport enabled={scrollEnabled} axis={axis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <FigureZoomedImage
            className="presentation-media-figure"
            portWidth={effectivePortSize.width}
            portHeight={effectivePortSize.height}
            sourceAspect={sourceAspect}
            scrollEnabled={scrollEnabled}
            src={resolvePresentationAssetUrl(embodiment.src)}
            alt={embodiment.alt ?? ""}
            onLoad={(event) => {
              const image = event.currentTarget;
              if (embodiment.sourceAspect === undefined && image.naturalWidth > 0 && image.naturalHeight > 0) {
                setSourceAspect(image.naturalWidth / image.naturalHeight);
              }
            }}
          />
        </FigureScrollViewport>
      </MediaTeaserWrap>
    </div>
  );
}

function FigureCropMorphView({
  morphId: anchorId,
  embodiment,
  emphasis,
  position,
  style,
  dormantAnchor,
  anchorOnWrapper = false,
  revealMorphCompanion,
  morphFrame,
  morphToFrame,
  fromMorphToFrame,
}: {
  readonly morphId: string;
  readonly embodiment: FigureEmbodiment;
  readonly emphasis: ParticipantEmphasis;
  readonly position: DispositionPosition;
  readonly style?: DispositionStyle;
  readonly dormantAnchor?: boolean;
  readonly anchorOnWrapper?: boolean;
  readonly revealMorphCompanion?: RevealMorphCompanionKind;
  readonly morphFrame?: DispositionPosition;
  readonly morphToFrame?: DispositionPosition;
  readonly fromMorphToFrame?: DispositionPosition;
}): ReactNode {
  const cropFrame = useContext(PresentationFigureCropFrameContext) ?? position;
  const slideAspect = useContext(PresentationSlideAspectContext);
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const scrollEnabled = figureEmbodimentScrollEnabled(embodiment);
  const frameRef = useRef<HTMLDivElement>(null);
  const [portSize, setPortSize] = useState({ width: 0, height: 0 });
  const estimatedPortSize = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return estimateDispositionFramePixels(cropFrame, slide.width, slide.height);
  }, [cropFrame, slideAspect]);
  const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
  const dormant = dormantAnchor === true;
  const morphCropFrom = revealMorphCompanion === "target" && morphFrame !== undefined && position !== undefined;
  const morphCropTo = position !== undefined && (fromMorphToFrame !== undefined || morphToFrame !== undefined);
  const backgroundVars =
    morphCropFrom && morphFrame ? figureCropBackgroundVarsTargetGhost(embodiment, embodiment.crop!, morphFrame, position) : figureCropBackgroundVars(embodiment, embodiment.crop!, cropFrame, morphToFrame, fromMorphToFrame, slideAspect);
  const backgroundSize = String(backgroundVars["--presentation-figure-bg-size" as keyof typeof backgroundVars] ?? "cover");
  const backgroundPosition = String(backgroundVars["--presentation-figure-bg-position" as keyof typeof backgroundVars] ?? "50% 50%");
  const sourceAspect = embodiment.sourceAspect ?? 1;
  useLayoutEffect(() => {
    const frame = frameRef.current;
    if (!frame) {
      return;
    }
    const measure = (): void => {
      setPortSize({ width: frame.clientWidth, height: frame.clientHeight });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(frame);
    return () => observer.disconnect();
  }, [slideEpoch]);
  const scrollAxis = figureBackgroundSizeScrollAxis(backgroundSize) ?? figureCoverOverflowAxis(effectivePortSize.width, effectivePortSize.height, sourceAspect);
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  const frameStyle = anchorOnWrapper
    ? {
        position: "relative" as const,
        width: "100%",
        height: "100%",
        boxSizing: "border-box" as const,
        ...(style?.opacity !== undefined ? { opacity: style.opacity } : {}),
      }
    : dispositionFrameStyle(position, dormant ? undefined : style);
  const slotClassName = [
    "presentation-morph-slot",
    "presentation-morph-slot--figure",
    revealMorphCompanion === "target" ? "presentation-target-ghost" : undefined,
    revealMorphCompanion === "source" ? "presentation-source-ghost" : undefined,
    morphCropFrom ? "presentation-morph-crop-from" : undefined,
    morphCropTo ? "presentation-morph-crop-to" : undefined,
    dormant ? "presentation-morph-slot--dormant" : undefined,
    emphasisClass(emphasis),
  ]
    .filter(Boolean)
    .join(" ");
  if (!scrollEnabled) {
    return (
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <div
          {...(anchorOnWrapper ? {} : { "data-id": anchorId })}
          className={["presentation-disposition-frame", slotClassName].join(" ")}
          style={{ ...frameStyle, ...backgroundVars }}
          {...(morphCropFrom && embodiment.crop ? { "data-presentation-morph-crop": JSON.stringify(embodiment.crop) } : {})}
          role="img"
          aria-label={embodiment.alt ?? ""}
        />
      </MediaTeaserWrap>
    );
  }
  return (
    <MediaTeaserWrap teaser={embodiment.teaser}>
      <div ref={frameRef} {...(anchorOnWrapper ? {} : { "data-id": anchorId })} className={["presentation-disposition-frame", "presentation-disposition-frame--figure-scroll"].join(" ")} style={frameStyle}>
        <FigureScrollViewport enabled axis={scrollAxis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <FigureZoomedCropScrollContent
            className={[slotClassName, "presentation-figure-scroll-content"].join(" ")}
            portWidth={effectivePortSize.width}
            portHeight={effectivePortSize.height}
            sourceAspect={sourceAspect}
            backgroundSize={backgroundSize}
            backgroundPosition={scrollAxis === "x" ? `0% ${scrollOrigin.y}%` : scrollAxis === "y" ? `${scrollOrigin.x}% 0%` : backgroundPosition}
            backgroundVars={backgroundVars}
            ariaLabel={embodiment.alt ?? ""}
            morphCropData={morphCropFrom && embodiment.crop ? JSON.stringify(embodiment.crop) : undefined}
          />
        </FigureScrollViewport>
      </div>
    </MediaTeaserWrap>
  );
}

function FigureMorphView({
  morphId: anchorId,
  embodiment,
  emphasis,
  position,
  style,
  dormantAnchor,
  anchorOnWrapper = false,
  revealMorphCompanion,
  morphFrame,
  morphToFrame,
  fromMorphToFrame,
}: {
  readonly morphId: string;
  readonly embodiment: FigureEmbodiment;
  readonly emphasis: ParticipantEmphasis;
  readonly position?: DispositionPosition;
  readonly style?: DispositionStyle;
  readonly dormantAnchor?: boolean;
  readonly anchorOnWrapper?: boolean;
  readonly revealMorphCompanion?: RevealMorphCompanionKind;
  readonly morphFrame?: DispositionPosition;
  readonly morphToFrame?: DispositionPosition;
  readonly fromMorphToFrame?: DispositionPosition;
}): ReactNode {
  if (embodiment.crop && position) {
    return (
      <FigureCropMorphView
        morphId={anchorId}
        embodiment={embodiment}
        emphasis={emphasis}
        position={position}
        style={style}
        dormantAnchor={dormantAnchor}
        anchorOnWrapper={anchorOnWrapper}
        revealMorphCompanion={revealMorphCompanion}
        morphFrame={morphFrame}
        morphToFrame={morphToFrame}
        fromMorphToFrame={fromMorphToFrame}
      />
    );
  }
  return <FigureImageMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={position} />;
}

/** 🏷️ Positioned text morph slot: `data-id` on the frame so reveal.js can morph figure crops into labels. */
function PositionedTextMorphView({
  morphId: anchorId,
  embodiment,
  emphasis,
  position,
  style,
  anchorOnWrapper = false,
  receivesMorphFrom = false,
}: {
  readonly morphId: string;
  readonly embodiment: TextEmbodiment;
  readonly emphasis: ParticipantEmphasis;
  readonly position: DispositionPosition;
  readonly style?: DispositionStyle;
  readonly anchorOnWrapper?: boolean;
  readonly receivesMorphFrom?: boolean;
}): ReactNode {
  const frameStyle = anchorOnWrapper
    ? {
        position: "relative" as const,
        width: "100%",
        height: "100%",
        boxSizing: "border-box" as const,
        ...(style?.opacity !== undefined ? { opacity: style.opacity } : {}),
      }
    : dispositionFrameStyle(position, style);
  if (receivesMorphFrom) {
    const headingClass = centeredLineClass(anchorId, embodiment, emphasis);
    return (
      <div className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", "presentation-morph-target", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
        <h2 className={headingClass}>{embodiment.lines[0]}</h2>
      </div>
    );
  }
  if (resolveTextMorphRoot(embodiment) === "heading-block") {
    return (
      <div className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
        <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />
      </div>
    );
  }
  const headingClass = centeredLineClass(anchorId, embodiment, emphasis);
  return (
    <div {...(anchorOnWrapper ? {} : { "data-id": anchorId })} className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
      <h2 className={headingClass}>{embodiment.lines[0]}</h2>
    </div>
  );
}

function VideoMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: VideoEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
  const scrollEnabled = videoEmbodimentScrollEnabled(embodiment);
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const slideAspect = useContext(PresentationSlideAspectContext);
  const anchorRef = useRef<HTMLDivElement>(null);
  const [portSize, setPortSize] = useState({ width: 0, height: 0 });
  const [sourceAspect, setSourceAspect] = useState(16 / 9);
  useLayoutEffect(() => {
    const anchor = anchorRef.current;
    if (!anchor) {
      return;
    }
    const measure = (): void => {
      const viewport = anchor.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
      const target = viewport ?? anchor;
      setPortSize({ width: target.clientWidth, height: target.clientHeight });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(anchor);
    return () => observer.disconnect();
  }, [slideEpoch]);
  const estimatedPortSize = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
  }, [position, slideAspect]);
  const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
  const coverScroll = figureCoverScrollContentSizeAtZoom(effectivePortSize.width, effectivePortSize.height, sourceAspect, 1);
  const axis = coverScroll.axis;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  return (
    <div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <FigureScrollViewport enabled={scrollEnabled} axis={axis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <FigureZoomedVideo
            className="presentation-media-video"
            portWidth={effectivePortSize.width}
            portHeight={effectivePortSize.height}
            sourceAspect={sourceAspect}
            scrollEnabled={scrollEnabled}
            src={embodiment.src}
            poster={embodiment.poster}
            loop={embodiment.loop}
            muted={embodiment.muted}
            controls={embodiment.controls}
            onLoadedMetadata={(event) => {
              const video = event.currentTarget;
              if (video.videoWidth > 0 && video.videoHeight > 0) {
                setSourceAspect(video.videoWidth / video.videoHeight);
              }
            }}
          />
        </FigureScrollViewport>
      </MediaTeaserWrap>
    </div>
  );
}

function IframeMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: IframeEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  return (
    <div data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <iframe className="presentation-media-iframe" src={resolvePresentationAssetUrl(embodiment.src)} title={embodiment.title ?? ""} loading="eager" />
      </MediaTeaserWrap>
    </div>
  );
}

function MarkdownMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: MarkdownEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  const [html, setHtml] = useState("");
  useEffect(() => {
    let cancelled = false;
    void (async () => {
      const markdown: string = embodiment.markdown ?? (await fetch(resolvePresentationAssetUrl(embodiment.src)).then((response) => response.text()));
      const compiled = await compileMarkdownToHtml(markdown);
      if (!cancelled) {
        setHtml(compiled);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [embodiment.markdown, embodiment.src]);
  return (
    <div data-id={anchorId} className={[morphAnchorClass(emphasis), "presentation-markdown-morph", "h-full w-full min-h-0 min-w-0"].filter(Boolean).join(" ")} aria-label={embodiment.title}>
      <Scrollable orientation="both" className="h-full w-full p-small">
        <div className="prose prose-sm max-w-none dark:prose-invert presentation-markdown-prose presentation-markdown-prose--top-left" dangerouslySetInnerHTML={{ __html: html }} />
      </Scrollable>
    </div>
  );
}

function JsonMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: JsonEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  const [data, setData] = useState<unknown>(embodiment.json ?? null);
  useEffect(() => {
    let cancelled = false;
    if (embodiment.json !== undefined) {
      setData(embodiment.json);
      return;
    }
    void (async () => {
      const text = await fetch(resolvePresentationAssetUrl(embodiment.src)).then((response) => response.text());
      const parsed = JSON.parse(text) as unknown;
      if (!cancelled) {
        setData(parsed);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [embodiment.json, embodiment.src]);
  return (
    <div data-id={anchorId} className={[morphAnchorClass(emphasis), "presentation-json-morph", "h-full w-full min-h-0 min-w-0"].filter(Boolean).join(" ")} aria-label={embodiment.title}>
      <Scrollable orientation="both" className="h-full w-full p-small">
        {renderJsonTree(data)}
      </Scrollable>
    </div>
  );
}

function PdfMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: PdfEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
  const anchorRef = useRef<HTMLDivElement>(null);
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const enlarged = useContext(PresentationDispositionEnlargeContext);
  const dispositionId = useContext(PresentationInteractiveDispositionIdContext);
  const interaction = useContext(PresentationInteractionContext);
  const declaredPage = pdfEmbodimentInitialPage(embodiment);
  const currentPage = dispositionId !== undefined && interaction !== null ? interaction.getPdfPage(dispositionId, declaredPage) : declaredPage;
  const containerSize = usePdfContainerSize(anchorRef, position, slideEpoch, enlarged);
  const [pageViewport, setPageViewport] = useState<{
    readonly width: number;
    readonly height: number;
  } | null>(null);
  const [numPages, setNumPages] = useState<number | null>(null);
  useEffect(() => {
    setPageViewport(null);
  }, [currentPage, embodiment.src, slideEpoch]);
  const pdfSrcRef = useRef(embodiment.src);
  useEffect(() => {
    if (pdfSrcRef.current === embodiment.src) {
      return;
    }
    pdfSrcRef.current = embodiment.src;
    setNumPages(null);
  }, [embodiment.src]);
  const onDocumentLoadSuccess = useCallback(({ numPages: total }: { readonly numPages: number }) => {
    setNumPages(total);
  }, []);
  const onPageLoadSuccess = useCallback((page: PdfCanvasPage) => {
    const viewport = page.getViewport({ scale: 1 });
    setPageViewport({ width: viewport.width, height: viewport.height });
  }, []);
  const scrollEnabled = pdfEmbodimentScrollEnabled(embodiment);
  const slideAspect = useContext(PresentationSlideAspectContext);
  const estimatedContainer = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : { width: 960, height: 700 };
  }, [position, slideAspect]);
  const effectiveContainer = containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0 ? { width: containerSize.width, height: containerSize.height } : estimatedContainer;
  const pageAspect = pageViewport !== null && pageViewport.height > 0 ? pageViewport.width / pageViewport.height : 1;
  const scrollAxis = scrollEnabled && pageViewport !== null ? figureCoverOverflowAxis(effectiveContainer.width, effectiveContainer.height, pageAspect) : null;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  const coverScale = useMemo(() => {
    if (pageViewport === null) {
      return null;
    }
    if (effectiveContainer.width <= 0 || effectiveContainer.height <= 0) {
      return null;
    }
    if (scrollEnabled) {
      const port = position !== undefined ? estimatedContainer : containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0 ? effectiveContainer : estimatedContainer;
      return pdfScrollCoverScale(port.width, port.height, pageViewport.width, pageViewport.height);
    }
    if (containerSize.width === undefined || containerSize.height === undefined) {
      return null;
    }
    return pdfCoverScale(containerSize.width, containerSize.height, pageViewport.width, pageViewport.height);
  }, [containerSize.height, containerSize.width, effectiveContainer.height, effectiveContainer.width, estimatedContainer.height, estimatedContainer.width, pageViewport, position, scrollEnabled]);
  const ready = containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0;
  const selected = dispositionId !== undefined && interaction !== null && interaction.isSelected(dispositionId);
  const showPageNav = (enlarged || selected) && dispositionId !== undefined && interaction !== null && pdfPageNavEnabled(embodiment, numPages);
  const goToPage = useCallback(
    (nextPage: number) => {
      if (dispositionId === undefined || interaction === null) {
        return;
      }
      interaction.setPdfPage(dispositionId, nextPage, declaredPage);
    },
    [declaredPage, dispositionId, interaction],
  );
  const goToAdjacentPage = useCallback(
    (direction: "prev" | "next") => {
      goToPage(pdfAdjacentPage(currentPage, direction, embodiment, numPages));
    },
    [currentPage, embodiment, goToPage, numPages],
  );
  return (
    <div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <FigureScrollViewport enabled={scrollEnabled} axis={scrollAxis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <div className={["presentation-media-pdf-document", scrollEnabled ? "presentation-media-pdf-document--scroll" : undefined].filter(Boolean).join(" ")}>
            <FigureZoomedPdfPage source={embodiment.src} currentPage={currentPage} coverScale={coverScale} ready={ready} ariaLabel={`PDF page ${currentPage}`} onDocumentLoadSuccess={onDocumentLoadSuccess} onPageLoadSuccess={onPageLoadSuccess} />
          </div>
        </FigureScrollViewport>
      </MediaTeaserWrap>
      {showPageNav ? (
        <div className="presentation-pdf-page-nav" role="group" aria-label="PDF pages">
          <button
            type="button"
            className="presentation-pdf-page-nav__button presentation-pdf-page-nav__button--prev"
            title="Previous page"
            disabled={!pdfCanGoToPreviousPage(currentPage, embodiment, numPages)}
            onPointerDown={(event) => {
              event.stopPropagation();
            }}
            onClick={(event) => {
              event.stopPropagation();
              goToAdjacentPage("prev");
            }}
          >
            ‹
          </button>
          <button
            type="button"
            className="presentation-pdf-page-nav__button presentation-pdf-page-nav__button--next"
            title="Next page"
            disabled={!pdfCanGoToNextPage(currentPage, embodiment, numPages)}
            onPointerDown={(event) => {
              event.stopPropagation();
            }}
            onClick={(event) => {
              event.stopPropagation();
              goToAdjacentPage("next");
            }}
          >
            ›
          </button>
        </div>
      ) : null}
    </div>
  );
}

function dispositionFrameStyle(position: DispositionPosition | undefined, style: DispositionStyle | undefined): React.CSSProperties | undefined {
  if (!position && !style) {
    return undefined;
  }
  const frame: React.CSSProperties = position
    ? {
        position: "absolute",
        left: `${position.x * 100}%`,
        top: `${position.y * 100}%`,
        width: `${position.width * 100}%`,
        height: `${position.height * 100}%`,
        boxSizing: "border-box",
      }
    : {};
  if (style?.opacity !== undefined) {
    frame.opacity = style.opacity;
  }
  const transforms: string[] = [];
  if (style?.rotate !== undefined) {
    transforms.push(`rotate(${style.rotate}deg)`);
  }
  if (style?.scale !== undefined) {
    transforms.push(`scale(${style.scale})`);
  }
  if (transforms.length > 0) {
    frame.transform = transforms.join(" ");
  }
  return frame;
}

function DispositionFrame({ disposition, children, overlay }: { readonly disposition: RevealResolvedDisposition; readonly children: ReactNode; readonly overlay?: boolean }): ReactNode {
  const frameStyle = dispositionFrameStyle(disposition.position, disposition.style);
  if (!frameStyle) {
    return children;
  }
  return (
    <div className={["presentation-disposition-frame", overlay ? "presentation-disposition-frame--overlay" : undefined].filter(Boolean).join(" ")} style={frameStyle}>
      {children}
    </div>
  );
}

export function MorphDispositionView({ disposition }: { readonly disposition: RevealResolvedDisposition }): ReactNode {
  const { embodiment, emphasis, morphId: anchorId } = disposition;
  const anchorOnWrapper = useContext(MorphAnchorOnWrapperContext);
  let content: ReactNode;
  switch (embodiment.kind) {
    case "text":
      if (disposition.position !== undefined) {
        return (
          <PositionedTextMorphView
            morphId={anchorId}
            embodiment={embodiment}
            emphasis={emphasis}
            position={disposition.position}
            style={disposition.style}
            anchorOnWrapper={anchorOnWrapper}
            receivesMorphFrom={(disposition.morphFrom?.length ?? 0) > 0}
          />
        );
      }
      content = <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} anchorOnWrapper={anchorOnWrapper} />;
      break;
    case "authors":
      content = <AuthorsMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} anchorOnWrapper={anchorOnWrapper} />;
      break;
    case "affiliations":
      content = <AffiliationsMorphView morphId={anchorId} embodiment={embodiment} anchorOnWrapper={anchorOnWrapper} />;
      break;
    case "bullet":
      content = <BulletMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
      break;
    case "figure":
      if (embodiment.crop && disposition.position !== undefined) {
        return (
          <FigureMorphView
            morphId={anchorId}
            embodiment={embodiment}
            emphasis={emphasis}
            position={disposition.position}
            style={disposition.style}
            anchorOnWrapper={anchorOnWrapper}
            revealMorphCompanion={disposition.revealMorphCompanion}
            morphFrame={disposition.revealMorphFromFrame}
            morphToFrame={disposition.revealMorphToFrame}
            fromMorphToFrame={disposition.revealMorphFromMorphToFrame}
          />
        );
      }
      content = <FigureMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} anchorOnWrapper={anchorOnWrapper} revealMorphCompanion={disposition.revealMorphCompanion} />;
      break;
    case "video":
      content = <VideoMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} />;
      break;
    case "iframe":
      content = <IframeMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
      break;
    case "markdown":
      content = <MarkdownMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
      break;
    case "json":
      content = <JsonMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
      break;
    case "pdf":
      content = <PdfMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} />;
      break;
    default: {
      const _exhaustive: never = embodiment;
      content = _exhaustive;
    }
  }
  const overlay = disposition.embodiment.kind === "figure" && disposition.position !== undefined && !disposition.embodiment.crop;
  return (
    <DispositionFrame disposition={disposition} overlay={overlay}>
      {content}
    </DispositionFrame>
  );
}
//#endregion 🔖️MorphView
