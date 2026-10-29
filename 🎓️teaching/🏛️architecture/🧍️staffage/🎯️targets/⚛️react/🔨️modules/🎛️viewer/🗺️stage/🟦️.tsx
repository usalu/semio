/** 🗺️ The stage of the viewer: one figure in true paper size times a zoom, panned by drag, wheel and keys, on the ground the reader chose.
 *
 * Zoom is a screen transform only (zoom 1 = print size, `1 mm` = 96/25.4 px) and never changes the scene or its scale; pan is stored in paper millimetres so it survives a zoom. Every view shares one window around the model origin,
 * so switching views never moves the figure. Keys on the focused stage: `+` `-` zoom, `0` fit, `1` print size, Alt+arrows pan.
 *
 * @see ../../🧍️figure/🟦️.tsx — the figure
 * @see ../../🧮️pipeline/🟦️.ts — `SceneFrame`
 */
import { useCallback, useEffect, useId, useRef, useState, type KeyboardEvent, type PointerEvent, type ReactElement } from "react";
import { CSS_PX_PER_MM } from "@teaching/staffage";
import type { Asset, DepictionOptions, DisplayRange, DrawingScene, EvaluatedState, StructuralResult, ThermalField, EvaluationQuery, ForceQuantity, ItemValue, Language, ThermalQuantity, View, Environment } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🟦️.ts";
import { StaffageFigure } from "../../🧍️figure/🟦️.tsx";
import { useSize } from "../../🥞️levels/🟦️.tsx";
import type { SceneFrame } from "../../🧮️pipeline/🟦️.ts";
import type { Background } from "../../💾️persistence/🟦️.ts";
import { cn } from "../../🪟️chrome/🟦️.tsx";
import { FigureBoundary } from "../../🖼️previews/🟦️.tsx";
import { Notice } from "../../🪟️chrome/🟦️.tsx";

/** 🎨️ The ground colours of the stage. */
export const GROUND_COLOURS: Readonly<Record<Exclude<Background, "transparent">, string>> = { dark: "#001117", light: "#f7f3e3", paper: "#ffffff" };

/** ⚙️ The props of {@link Stage}. */
export type StageProps = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly clock: StaffageClock;
  readonly view: View;
  readonly scale: number;
  readonly language: Language;
  readonly theme: "light" | "dark";
  readonly ground: Background;
  readonly transparent: boolean;
  readonly frame: SceneFrame | undefined;
  readonly anchor: SceneFrame | undefined;
  readonly zoom: "fit" | number;
  readonly pan: { readonly xMm: number; readonly yMm: number };
  readonly onZoom: (zoom: "fit" | number) => void;
  readonly onPan: (pan: { readonly xMm: number; readonly yMm: number }) => void;
  readonly label: string;
  readonly help: string;
  readonly thermalQuantity: ThermalQuantity;
  readonly forceQuantity: ForceQuantity;
  readonly range: DisplayRange;
  readonly patterns: boolean;
  readonly diagrams: DepictionOptions["diagrams"];
  readonly environment: Environment | undefined;
  readonly onEvaluated: (frame: { state: EvaluatedState; thermal: ThermalField; structure: StructuralResult; scene: DrawingScene }) => void;
  readonly onInspect: (value: ItemValue | null, item: { bone?: string; region?: string } | null) => void;
  readonly onFit?: (zoom: number) => void;
  readonly className?: string;
};

const MIN_ZOOM = 0.05;
const MAX_ZOOM = 64;

/** 🗺️ The pannable, zoomable stage. */
export function Stage(props: StageProps): ReactElement {
  const { frame, zoom, pan, onZoom, onPan, ground, transparent } = props;
  const [attach, size] = useSize({ width: 800, height: 520 });
  const helpId = useId();
  const [drag, setDrag] = useState<{ readonly xMm: number; readonly yMm: number } | null>(null);
  const start = useRef<{ x: number; y: number; pan: { xMm: number; yMm: number } } | null>(null);
  const element = useRef<HTMLDivElement | null>(null);
  const widthMm = frame === undefined ? 100 : frame.maxX - frame.minX;
  const heightMm = frame === undefined ? 100 : frame.maxY - frame.minY;
  const fit = Math.max(MIN_ZOOM, Math.min((size.width - 24) / (widthMm * CSS_PX_PER_MM), (size.height - 24) / (heightMm * CSS_PX_PER_MM), MAX_ZOOM));
  const effective = zoom === "fit" ? fit : zoom;
  const centre = (box: SceneFrame | undefined): { readonly x: number; readonly y: number } => (box === undefined ? { x: 0, y: 0 } : { x: (box.minX + box.maxX) / 2, y: (box.minY + box.maxY) / 2 });
  const shift = zoom === "fit" ? { x: 0, y: 0 } : { x: centre(frame).x - centre(props.anchor).x, y: centre(frame).y - centre(props.anchor).y };
  const shown = drag ?? pan;
  const px = CSS_PX_PER_MM * effective;
  const latest = useRef({ effective, onZoom });
  latest.current = { effective, onZoom };
  useEffect(() => props.onFit?.(fit), [fit]);
  useEffect(() => {
    const node = element.current;
    if (node === null) return;
    const wheel = (event: WheelEvent): void => {
      if (!event.ctrlKey && !event.metaKey) return;
      event.preventDefault();
      const next = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, latest.current.effective * Math.exp(-event.deltaY * 0.0015)));
      latest.current.onZoom(Number(next.toFixed(3)));
    };
    node.addEventListener("wheel", wheel, { passive: false });
    return () => node.removeEventListener("wheel", wheel);
  }, []);
  const attachBoth = useCallback(
    (node: HTMLDivElement | null): void => {
      element.current = node;
      attach(node);
    },
    [attach],
  );
  const down = (event: PointerEvent<HTMLDivElement>): void => {
    if (event.button !== 0) return;
    event.currentTarget.setPointerCapture(event.pointerId);
    start.current = { x: event.clientX, y: event.clientY, pan };
  };
  const move = (event: PointerEvent<HTMLDivElement>): void => {
    const origin = start.current;
    if (origin === null) return;
    setDrag({ xMm: origin.pan.xMm + (event.clientX - origin.x) / px, yMm: origin.pan.yMm + (event.clientY - origin.y) / px });
  };
  const up = (): void => {
    if (start.current !== null && drag !== null) onPan({ xMm: drag.xMm, yMm: drag.yMm });
    start.current = null;
    setDrag(null);
  };
  const key = (event: KeyboardEvent<HTMLDivElement>): void => {
    const step = (event.shiftKey ? 40 : 10) / px;
    if (event.altKey && event.key.startsWith("Arrow")) {
      event.preventDefault();
      event.stopPropagation();
      onPan({ xMm: pan.xMm + (event.key === "ArrowLeft" ? step : event.key === "ArrowRight" ? -step : 0), yMm: pan.yMm + (event.key === "ArrowUp" ? step : event.key === "ArrowDown" ? -step : 0) });
    } else if (event.key === "+" || event.key === "=") onZoom(Math.min(MAX_ZOOM, Number((effective * 1.25).toFixed(3))));
    else if (event.key === "-") onZoom(Math.max(MIN_ZOOM, Number((effective / 1.25).toFixed(3))));
    else if (event.key === "0") onZoom("fit");
    else if (event.key === "1") onZoom(1);
  };
  const colour = transparent ? undefined : GROUND_COLOURS[ground === "transparent" ? "paper" : ground];
  return (
    <div
      ref={attachBoth}
      role="group"
      tabIndex={0}
      aria-label={props.label}
      aria-describedby={helpId}
      aria-keyshortcuts="+ - 0 1 Alt+ArrowLeft Alt+ArrowRight Alt+ArrowUp Alt+ArrowDown"
      onPointerDown={down}
      onPointerMove={move}
      onPointerUp={up}
      onPointerCancel={up}
      onKeyDown={key}
      data-stage=""
      data-zoom={effective.toFixed(3)}
      className={cn("relative h-[clamp(22rem,64vh,42rem)] w-full touch-pan-y select-none overflow-hidden border border-border", ground === "transparent" && "staffage-checker", start.current === null ? "cursor-grab" : "cursor-grabbing", props.className)}
      style={colour === undefined ? undefined : { backgroundColor: colour }}
    >
      <span id={helpId} className="sr-only">
        {props.help}
      </span>
      <div className="absolute left-1/2 top-1/2" style={{ transform: `translate(-50%, -50%) translate(${(shown.xMm + shift.x) * px}px, ${(shown.yMm + shift.y) * px}px)` }}>
        <FigureBoundary resetKey={props.asset} fallback={(message) => <Notice kind="error" title={props.asset.title[props.language]}>{message}</Notice>}>
          <StaffageFigure
            asset={props.asset}
            query={props.query}
            clock={props.clock}
            view={props.view}
            scale={{ denominator: props.scale }}
            language={props.language}
            theme={props.theme}
            zoom={effective}
            fit="paper"
            transparent={transparent}
            {...(frame === undefined ? {} : { frame })}
            thermalQuantity={props.thermalQuantity}
            forceQuantity={props.forceQuantity}
            range={props.range}
            patterns={props.patterns}
            diagrams={props.diagrams}
            {...(props.environment === undefined ? {} : { environment: props.environment })}
            live
            onEvaluated={props.onEvaluated}
            onInspect={props.onInspect}
          />
        </FigureBoundary>
      </div>
    </div>
  );
}
