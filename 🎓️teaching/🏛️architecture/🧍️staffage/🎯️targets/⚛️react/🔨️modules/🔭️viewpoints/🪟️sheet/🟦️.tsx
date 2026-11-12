/** 🪟️ The three-view sheet: top view above the side view, front view beside it, in aligned projection, every cell a window of the same drawing at the same zoom.
 *
 * {@link ThreeViewSheet} draws a given layout; all three figures share one asset, query, clock, view mode, scale and environment, so scrubbing moves them on the same frame and a metre measures the same in each. A cell whose
 * viewpoint cannot be drawn keeps its place and shows the reason in words. {@link StaffageViews} is the reusable component that measures its box and fits the sheet (or takes a fixed zoom, 1 = print size) and finds its own layout.
 *
 * @see ../🟦️.ts — the layout and the resolutions
 * @see ../../🧍️figure/🟦️.tsx — the figure of each cell
 */
import { useId, useMemo, type ReactElement } from "react";
import { CSS_PX_PER_MM, formatNumber, mmPerMetre } from "@teaching/staffage";
import type { Asset, DrawingScale, EvaluationQuery, Language, PlanarViewpoint, View } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { StaffageFigure, type StaffageFigureProps } from "../../🧍️figure/🟦️.tsx";
import { useSize } from "../../🥞️levels/🟦️.tsx";
import { Notice, cn } from "../../🪟️chrome/🟦️.tsx";
import { FigureBoundary } from "../../🖼️previews/🟦️.tsx";
import { defaultQuery } from "../../🧮️pipeline/🟦️.ts";
import { SHEET_ORDER, VIEWPOINT_NAMES, VIEWPOINT_PHRASES, sheetOf, viewpointWord, type Sheet } from "../🟦️.ts";

/** 🎛️ What the three figures share beyond asset, instant and scale. */
export type SheetFigureOptions = Pick<StaffageFigureProps, "theme" | "transparent" | "annotations" | "thermalQuantity" | "forceQuantity" | "range" | "patterns" | "diagrams" | "environment" | "onInspect">;

/** ⚙️ The props of {@link ThreeViewSheet}. */
export type ThreeViewSheetProps = {
  readonly asset: Asset;
  readonly query?: EvaluationQuery;
  readonly clock?: StaffageClock;
  readonly view: View;
  readonly scale: DrawingScale;
  readonly language: Language;
  readonly sheet: Sheet;
  readonly zoom: number;
  readonly focus: PlanarViewpoint;
  readonly onFocus?: (viewpoint: PlanarViewpoint) => void;
  readonly onEvaluated?: StaffageFigureProps["onEvaluated"];
  readonly figure?: SheetFigureOptions;
  readonly className?: string;
};

/** 🔭️ Draws the given sheet at a zoom. */
export function ThreeViewSheet(props: ThreeViewSheetProps): ReactElement {
  const { asset, language, sheet, zoom, focus, onFocus, onEvaluated } = props;
  const { layout, resolutions } = sheet;
  const mm = (value: number): string => `${value * zoom}mm`;
  const column = (viewpoint: PlanarViewpoint): number => layout.cells[viewpoint].column;
  const helpId = useId();
  const missing = SHEET_ORDER.filter((viewpoint) => !resolutions[viewpoint].available).map((viewpoint) => VIEWPOINT_PHRASES[viewpoint][language]);
  return (
    <div
      role="group"
      lang={language}
      aria-label={viewpointWord("viewpoints.sheet", language, { title: asset.title[language] })}
      aria-describedby={helpId}
      data-three-views=""
      data-zoom={zoom.toFixed(4)}
      data-width-mm={layout.widthMm.toFixed(3)}
      data-height-mm={layout.heightMm.toFixed(3)}
      className={cn("grid", props.className)}
      style={{ display: "grid", gridTemplateColumns: `${mm(layout.cells.elevation.widthMm)} ${mm(layout.cells.front.widthMm)}`, gridTemplateRows: `${mm(layout.cells.plan.heightMm)} ${mm(layout.cells.elevation.heightMm)}`, gap: mm(layout.gapMm) }}
    >
      <span id={helpId} className="sr-only">{viewpointWord("viewpoints.sheetHelp", language)}</span>
      {missing.length === 0 ? null : <span role="status" aria-live="polite" className="sr-only">{viewpointWord("viewpoints.sheetUnavailable", language, { views: missing.join(", ") })}</span>}
      {SHEET_ORDER.map((viewpoint) => {
        const cell = layout.cells[viewpoint];
        const resolution = resolutions[viewpoint];
        const focused = focus === viewpoint;
        const phrase = VIEWPOINT_PHRASES[viewpoint][language];
        const name = VIEWPOINT_NAMES[viewpoint][language];
        return (
          <figure
            key={viewpoint}
            data-viewpoint={viewpoint}
            data-focus={focused ? "" : undefined}
            data-available={resolution.available ? "true" : "false"}
            data-width-mm={cell.widthMm.toFixed(3)}
            data-height-mm={cell.heightMm.toFixed(3)}
            aria-label={name}
            className="relative m-0 min-w-0 overflow-hidden"
            style={{ gridColumn: column(viewpoint), gridRow: cell.row, width: mm(cell.widthMm), height: mm(cell.heightMm) }}
          >
            {resolution.available ? (
              <FigureBoundary resetKey={asset} fallback={(message) => <Notice kind="error" compact title={phrase}>{message}</Notice>}>
                <StaffageFigure
                  asset={asset}
                  {...(props.query === undefined ? {} : { query: props.query })}
                  {...(props.clock === undefined ? {} : { clock: props.clock })}
                  view={props.view}
                  scale={props.scale}
                  viewpoint={viewpoint}
                  language={language}
                  frame={cell.frame}
                  zoom={zoom}
                  fit="paper"
                  live
                  {...props.figure}
                  {...(focused && onEvaluated !== undefined ? { onEvaluated } : {})}
                />
              </FigureBoundary>
            ) : (
              <div data-notice="unavailable" className="flex h-full min-w-0 flex-col gap-single overflow-auto border border-dashed border-border bg-muted/40 p-double pt-6 text-xs text-foreground">
                <p className="m-0 text-sm font-semibold">{viewpointWord("viewpoints.unavailable", language, { view: phrase })}</p>
                <p className="m-0">{resolution.message[language]}</p>
                {resolution.details === undefined ? null : (
                  <details>
                    <summary>{viewpointWord("viewpoints.failedDetails", language)}</summary>
                    <code className="font-mono">{resolution.details}</code>
                  </details>
                )}
                <p className="m-0 text-muted-foreground">{viewpointWord("viewpoints.nothingInstead", language)}</p>
              </div>
            )}
            <button
              type="button"
              aria-pressed={focused}
              onClick={() => onFocus?.(viewpoint)}
              onPointerDown={(event) => event.stopPropagation()}
              className={cn("absolute left-1 top-1 border border-border px-single text-[0.65rem] uppercase tracking-wide", focused ? "bg-active-base text-active-foreground" : "bg-panel text-muted-foreground hover:bg-hover-panel")}
              aria-label={focused ? viewpointWord("viewpoints.focusPressed", language, { name }) : viewpointWord("viewpoints.focusAction", language, { name })}
              title={viewpointWord("viewpoints.focus", language)}
            >
              {name}
            </button>
          </figure>
        );
      })}
    </div>
  );
}

/** 📏️ The props of {@link StaffageViews}. */
export type StaffageViewsProps = Omit<ThreeViewSheetProps, "sheet" | "zoom" | "focus" | "scale"> & { readonly scale: DrawingScale; readonly zoom?: "fit" | number; readonly focus?: PlanarViewpoint };

/** 📐️ The reusable three-view component: finds the layout of the asset at the scale and shows the aligned sheet. `zoom` is a number (1 = print size on screen) or `"fit"` to the parent box; either way all three views share it. */
export function StaffageViews(props: StaffageViewsProps): ReactElement {
  const { asset, language, view, scale, zoom = "fit" } = props;
  const [attach, size] = useSize({ width: 800, height: 600 });
  const queries = useMemo(() => [props.query ?? defaultQuery(asset, 0)], [asset, props.query]);
  const sheet = useMemo(() => sheetOf(asset, queries, { view, denominator: scale.denominator, language, ...(props.figure?.theme === undefined ? {} : { theme: props.figure.theme }) }), [asset, queries, view, scale.denominator, language, props.figure?.theme]);
  const fit = Math.min((size.width - 8) / (sheet.layout.widthMm * CSS_PX_PER_MM), (size.height - 8) / (sheet.layout.heightMm * CSS_PX_PER_MM));
  const effective = zoom === "fit" ? Math.max(0.05, fit) : zoom;
  const note = viewpointWord("viewpoints.sheetScale", language, { mm: formatNumber(mmPerMetre(scale), 1, language), scale: scale.denominator });
  return (
    <div ref={attach} data-staffage-views="" className="flex w-full flex-col items-center gap-single" title={note}>
      <ThreeViewSheet {...props} sheet={sheet} zoom={effective} focus={props.focus ?? "elevation"} />
    </div>
  );
}
