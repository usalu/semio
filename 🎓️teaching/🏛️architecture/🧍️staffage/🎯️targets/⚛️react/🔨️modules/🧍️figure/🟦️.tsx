/** 🧍️ `StaffageFigure`: one evaluated asset drawn as inline SVG in true paper size or fitted to its parent, with hover and keyboard inspection and a table alternative to colour.
 *
 * `fit="paper"` sizes the element to the scene in CSS millimetres times `zoom` (zoom 1 is print size on screen); `fit="contain"` fills the parent box and never reports the scene's size back to it, so a walking figure cannot resize the card around it.
 * Only analytic views solve thermal or structural analyses; results are memoised by (asset, query, time, environment) across all figures.
 *
 * @see ../⏱️clock/🟦️.ts — the shared time source
 * @see ../🧮️pipeline/🟦️.ts — the memoised evaluator
 * @see ../🗣️labels/🟦️.ts — the spoken description
 */
import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import type { CSSProperties, KeyboardEvent, PointerEvent, ReactElement } from "react";
import type { SceneFrame } from "../🧮️pipeline/🟦️.ts";
import type { Asset, DepictionOptions, DisplayRange, DrawingScale, DrawingScene, Environment, EvaluatedState, EvaluationQuery, ForceQuantity, ItemValue, Language, StructuralResult, ThermalField, ThermalQuantity, View, Viewpoint } from "@teaching/staffage";
import type { StaffageClock } from "../⏱️clock/🧱️core/🟦️.ts";
import { describeScene, valueText, valuedItems, word } from "../🗣️labels/🟦️.ts";
import { defaultQuery, figureFrame, inlineSvg, queryAt } from "../🧮️pipeline/🟦️.ts";

/** ⚙️ The props of {@link StaffageFigure}. */
export type StaffageFigureProps = {
  asset: Asset;
  query?: EvaluationQuery;
  clock?: StaffageClock;
  view: View;
  scale: DrawingScale;
  viewpoint?: Viewpoint;
  theme?: "light" | "dark";
  frame?: SceneFrame;
  language: Language;
  zoom?: number;
  fit?: "paper" | "contain";
  transparent?: boolean;
  annotations?: Partial<DepictionOptions["annotations"]>;
  thermalQuantity?: ThermalQuantity;
  forceQuantity?: ForceQuantity;
  range?: DisplayRange;
  patterns?: boolean;
  diagrams?: Partial<DepictionOptions["diagrams"]>;
  environment?: Environment;
  onEvaluated?: (frame: { state: EvaluatedState; thermal: ThermalField; structure: StructuralResult; scene: DrawingScene }) => void;
  onInspect?: (value: ItemValue | null, item: { bone?: string; region?: string } | null) => void;
  className?: string;
  title?: string;
  live?: boolean;
};

const HIDDEN: CSSProperties = { position: "absolute", width: 1, height: 1, margin: -1, padding: 0, overflow: "hidden", clip: "rect(0 0 0 0)", whiteSpace: "nowrap", border: 0 };

/** ♿️ The visually hidden table of the analytic values of a scene, the accessible alternative to colour. */
export function StaffageValueTable({ scene, language, id, className }: { scene: DrawingScene; language: Language; id?: string; className?: string }): ReactElement | null {
  const rows = useMemo(() => valuedItems(scene), [scene]);
  if (rows.length === 0) return null;
  return (
    <table id={id} className={className} lang={language} style={HIDDEN}>
      <caption>{word("values", language)}</caption>
      <thead>
        <tr>
          <th scope="col">{word("item", language)}</th>
          <th scope="col">{word("quantity", language)}</th>
          <th scope="col">{word("value", language)}</th>
        </tr>
      </thead>
      <tbody>
        {rows.map((row) => (
          <tr key={row.id}>
            <th scope="row">{row.value.label[language]}</th>
            <td>{row.value.quantity}</td>
            <td>{valueText(row.value, language)}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function useVisible(live: boolean): [React.RefObject<HTMLDivElement | null>, boolean] {
  const ref = useRef<HTMLDivElement | null>(null);
  const [visible, setVisible] = useState(true);
  useEffect(() => {
    const element = ref.current;
    if (live || element === null || typeof IntersectionObserver !== "function") return;
    const observer = new IntersectionObserver((entries) => setVisible(entries.at(-1)?.isIntersecting ?? true));
    observer.observe(element);
    return () => observer.disconnect();
  }, [live]);
  return [ref, live || visible];
}

function useFrameTime(clock: StaffageClock | undefined, active: boolean): number | null {
  const last = useRef<number | null>(null);
  const snapshot = (): number | null => {
    if (clock === undefined) return null;
    if (active || last.current === null) last.current = clock.state.frame / clock.state.fps;
    return last.current;
  };
  return useSyncExternalStore(clock?.subscribe ?? ((): (() => void) => () => undefined), snapshot, snapshot);
}

const svgId = (id: string): string => id.replace(/[^A-Za-z0-9_.-]/gu, "-");

function match(target: EventTarget | null, ids: ReadonlySet<string>, root: HTMLElement): string | null {
  for (let node = target instanceof Element ? target : null; node !== null && node !== root; node = node.parentElement) {
    const id = node.getAttribute("id");
    if (id !== null && ids.has(id)) return id;
  }
  return null;
}

/** 🧍️ Renders one staffage asset in one view at one scale. */
export function StaffageFigure(props: StaffageFigureProps): ReactElement {
  const { asset, clock, language, zoom = 1, fit = "paper", live = false, onEvaluated, onInspect, className, title } = props;
  const [ref, visible] = useVisible(live);
  const timeS = useFrameTime(clock, visible);
  const request = {
    view: props.view,
    scale: props.scale,
    language,
    ...(props.viewpoint === undefined ? {} : { viewpoint: props.viewpoint }),
    ...(props.theme === undefined ? {} : { theme: props.theme }),
    ...(props.transparent === undefined ? {} : { transparent: props.transparent }),
    ...(props.annotations === undefined ? {} : { annotations: props.annotations }),
    ...(props.thermalQuantity === undefined ? {} : { thermalQuantity: props.thermalQuantity }),
    ...(props.forceQuantity === undefined ? {} : { forceQuantity: props.forceQuantity }),
    ...(props.range === undefined ? {} : { range: props.range }),
    ...(props.patterns === undefined ? {} : { patterns: props.patterns }),
    ...(props.diagrams === undefined ? {} : { diagrams: props.diagrams }),
    ...(props.environment === undefined ? {} : { environment: props.environment }),
  };
  const requestKey = JSON.stringify(request);
  const query = useMemo(() => queryAt(props.query ?? defaultQuery(asset, timeS), timeS), [asset, props.query, timeS]);
  const frame = useMemo(() => figureFrame(asset, query, request), [asset, query, requestKey]);
  const { scene } = frame;
  const frameKey = props.frame === undefined ? "" : JSON.stringify(props.frame);
  const markup = useMemo(() => inlineSvg(scene, props.frame), [scene, frameKey]);
  const widthMm = props.frame === undefined ? scene.widthMm : props.frame.maxX - props.frame.minX;
  const heightMm = props.frame === undefined ? scene.heightMm : props.frame.maxY - props.frame.minY;
  const showValues = props.annotations?.legend !== false || onInspect !== undefined;
  const items = useMemo(() => (showValues ? valuedItems(scene) : []), [scene, showValues]);
  const ids = useMemo(() => new Set(items.map((item) => svgId(item.id))), [items]);
  const [focused, setFocused] = useState(-1);
  const tableId = useRef(`staffage-values-${Math.random().toString(36).slice(2, 9)}`).current;

  useEffect(() => {
    onEvaluated?.(frame);
  }, [frame]);

  const inspect = (index: number): void => {
    const item = items[index];
    setFocused(index);
    onInspect?.(item?.value ?? null, item === undefined ? null : { ...(item.bone === undefined ? {} : { bone: item.bone }), ...(item.region === undefined ? {} : { region: item.region }) });
  };
  const onPointer = (event: PointerEvent<HTMLDivElement>): void => {
    if (onInspect === undefined) return;
    const id = match(event.target, ids, event.currentTarget);
    inspect(id === null ? -1 : items.findIndex((item) => svgId(item.id) === id));
  };
  const onKey = (event: KeyboardEvent<HTMLDivElement>): void => {
    if (items.length === 0) return;
    const step = event.key === "ArrowRight" || event.key === "ArrowDown" ? 1 : event.key === "ArrowLeft" || event.key === "ArrowUp" ? -1 : 0;
    if (step !== 0) {
      event.preventDefault();
      inspect((focused + step + items.length) % items.length);
    } else if (event.key === "Escape") inspect(-1);
  };

  const size: CSSProperties =
    fit === "paper"
      ? { position: "relative", width: `${widthMm * zoom}mm`, height: `${heightMm * zoom}mm`, overflow: "hidden", flex: "none" }
      : { position: "absolute", inset: 0, width: "100%", height: "100%", minWidth: 0, minHeight: 0, overflow: "hidden", flex: "none" };
  const label = title ?? describeScene(scene, language);
  const current = items[focused];

  return (
    <>
    <div
      ref={ref}
      className={className}
      role="img"
      tabIndex={items.length > 0 ? 0 : undefined}
      lang={language}
      aria-label={label}
      aria-describedby={items.length > 0 ? tableId : undefined}
      data-staffage-figure=""
      data-fit={fit}
      data-asset={asset.id}
      data-view={props.view}
      data-time={scene.metadata.timeS}
      data-width-mm={widthMm}
      data-height-mm={heightMm}
      style={{ display: "block", ...(props.frame !== undefined && scene.background !== null ? { background: scene.background } : {}), ...size }}
      onPointerMove={onPointer}
      onPointerLeave={() => onInspect !== undefined && inspect(-1)}
      onKeyDown={onKey}
      onBlur={() => focused >= 0 && inspect(-1)}
    >
      <div style={{ position: "absolute", inset: 0 }} dangerouslySetInnerHTML={{ __html: markup }} />
    </div>
    {showValues ? <StaffageValueTable scene={scene} language={language} id={tableId} /> : null}
    <span role="status" aria-live="polite" style={HIDDEN}>
      {current === undefined ? "" : `${current.value.label[language]}: ${valueText(current.value, language)}`}
    </span>
    </>
  );
}
