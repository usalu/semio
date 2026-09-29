/** 🕸️ Spider (radar) diagrams of category profiles as owned SVG.
 *
 * Every axis normalises its value to `(value − min) / (max − min)`, clamped to the diagram, and spokes start at twelve
 * o'clock and run clockwise. Axis labels come from content and can be long, so {@link radarLayout} lays each diagram
 * out for the width it actually gets: labels wrap onto as many lines as they need, the circle yields room to them and
 * the height grows to hold them. The SVG draws one user unit per CSS pixel, so labels keep the reading size of the text
 * around them and are never scaled down or clipped. The SVG is one `role="img"` with a text name; the exact values,
 * units and axis ranges are always available as a table in a disclosure next to it — the text alternative screen
 * readers and learners who prefer numbers use.
 *
 * @see ../../../../🧬️schema/🔣️.json — `Axis`, `Profile`
 * @see https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText
 * @see https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver
 */

import { useId, useLayoutEffect, useMemo, useRef, useState, type ReactElement, type RefObject } from "react";
import type { QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import { formatNumber, withUnit } from "../📏️quantity/🟦️.ts";

//#region 📐️Geometry
/** 📐️ The circle a diagram is drawn in, in SVG user units. */
export interface RadarFrame {
  readonly cx: number;
  readonly cy: number;
  readonly radius: number;
}

/** 📍️ A point in SVG user units. */
export interface RadarPoint {
  readonly x: number;
  readonly y: number;
}

/** 📏️ The share of an axis `value` covers between `min` (centre) and `max` (rim), clamped to [0, 1]. */
export function radarFraction(value: number, min: number, max: number): number {
  if (max === min) return 0;
  return Math.min(1, Math.max(0, (value - min) / (max - min)));
}

/** 🧭️ The angle of spoke `index` of `count` in radians, zero at twelve o'clock, growing clockwise. */
export function radarAngle(index: number, count: number): number {
  return (2 * Math.PI * index) / count;
}

/** 📍️ The point at `fraction` of the radius along spoke `index` of `count`. */
export function radarPoint(frame: RadarFrame, index: number, count: number, fraction: number): RadarPoint {
  const angle = radarAngle(index, count);
  return { x: frame.cx + frame.radius * fraction * Math.sin(angle), y: frame.cy - frame.radius * fraction * Math.cos(angle) };
}

/** 🔷️ The polygon of one profile: one point per axis, in axis order. */
export function radarPolygon(frame: RadarFrame, axes: readonly Pick<RadarAxis, "id" | "min" | "max">[], values: Readonly<Record<string, number>>): readonly RadarPoint[] {
  return axes.map((axis, index) => radarPoint(frame, index, axes.length, radarFraction(values[axis.id] ?? axis.min, axis.min, axis.max)));
}

function pointList(points: readonly RadarPoint[]): string {
  return points.map((point) => `${point.x.toFixed(2)},${point.y.toFixed(2)}`).join(" ");
}
//#endregion 📐️Geometry

//#region 🏷️Labels
/** 📏️ The advance width of a text in SVG user units. */
export type TextMeasure = (text: string) => number;

/** ▭️ An axis-aligned box in SVG user units. */
export interface RadarBox {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** 🏷️ One wrapped axis label: its lines, the box they fill, and where and how every line anchors horizontally. */
export interface RadarLabel {
  readonly lines: readonly string[];
  readonly box: RadarBox;
  readonly x: number;
  readonly anchor: "start" | "middle" | "end";
}

/** 🔢️ One entry of the numbered legend a diagram shows when its labels do not fit beside the circle. */
export interface RadarLegendEntry {
  readonly key: RadarLabel;
  readonly text: RadarLabel;
}

/** 🗺️ A diagram laid out for one width: the view box, the circle, the text at every spoke end (the label, or its number
 * when the labels form the `legend` below), with the font size and line height of all of them. */
export interface RadarLayout {
  readonly width: number;
  readonly height: number;
  readonly frame: RadarFrame;
  readonly fontSize: number;
  readonly lineHeight: number;
  readonly labels: readonly RadarLabel[];
  readonly legend: readonly RadarLegendEntry[];
}

/** 🔠️ The label font size as a share of the surrounding text size. */
export const RADAR_LABEL_EM = 0.85;

/** 📐️ The layout's proportions in label font sizes: view box padding, spoke end to label, line height, space between
 * labels, the radius range, the smallest radius that still keeps a side label's longest word whole, the room side
 * labels get before the circle grows, and the least room they keep. */
export const RADAR_METRICS = { pad: 0.4, gap: 0.5, line: 1.3, spacing: 0.3, minRadius: 3, maxRadius: 7.5, wordRadius: 2, side: 6, minSide: 3 } as const;

const HYPHEN = "-";
const MIN_FRAGMENT = 3;
const SYLLABLE_REACH = 4;
const LETTER = /\p{L}/u;
const VOWEL = /[aeiouyàáâãäåæèéêëìíîïòóôõöøùúûüýÿœ]/iu;
const FIT_TOLERANCE = 1e-6;
const HYPHEN_BREAK = /(?<=-)(?=[^-])/u;
const NARROW = new Set("iljI|.,:;'!·ı¡");
const SLIM = new Set('frt()[]{}°¹²³"*`‘’‚');
const WIDE = new Set("mwæMW");
const BROAD = new Set("ÆŒœ¼½¾");
const MEDIUM = new Set("/&€$£#+=<>_~?–-");
const charWidths = new Map<string, number>();

function charEm(char: string): number {
  const known = charWidths.get(char);
  if (known !== undefined) return known;
  const width = /\p{M}/u.test(char)
    ? 0
    : /\s/u.test(char) || NARROW.has(char)
      ? 0.3
      : SLIM.has(char)
        ? 0.45
        : WIDE.has(char)
          ? 0.95
          : BROAD.has(char)
            ? 1.2
            : /\p{Ll}/u.test(char)
              ? 0.62
              : /\p{Lu}/u.test(char)
                ? 0.8
                : /\p{Nd}/u.test(char)
                  ? 0.68
                  : MEDIUM.has(char)
                    ? 0.72
                    : 1.05;
  charWidths.set(char, width);
  return width;
}

/** 🔤️ A width no smaller than common sans-serif faces (semio's Anta, Arial, Segoe UI and the like) need for `text` at
 * `fontSize`, from per-character classes — for where no canvas can measure (tests, server rendering, the first pass). */
export function estimateTextWidth(text: string, fontSize: number): number {
  let em = 0;
  for (const char of text) em += charEm(char);
  return em * fontSize;
}

let canvas: OffscreenCanvasRenderingContext2D | null | undefined;

/** 📏️ Measures text at `fontSize` in the CSS font `family` with a 2D canvas, which shapes text like the SVG around it;
 * falls back to {@link estimateTextWidth} where no canvas exists. */
export function textMeasure(fontSize: number, family: string): TextMeasure {
  canvas ??= typeof OffscreenCanvas === "undefined" ? null : new OffscreenCanvas(1, 1).getContext("2d");
  const context = canvas;
  if (context === null) return (text) => estimateTextWidth(text, fontSize);
  const font = `${fontSize}px ${family}`;
  return (text) => {
    context.font = font;
    return context.measureText(text).width;
  };
}

function fittingPrefix(chars: readonly string[], width: number, measure: TextMeasure): number {
  let low = 1;
  let high = chars.length - 1;
  while (low < high) {
    const middle = Math.ceil((low + high) / 2);
    if (measure(`${chars.slice(0, middle).join("")}${HYPHEN}`) <= width + FIT_TOLERANCE) low = middle;
    else high = middle - 1;
  }
  return syllableCut(chars, chars.length >= 2 * MIN_FRAGMENT ? Math.min(low, chars.length - MIN_FRAGMENT) : low);
}

function syllableCut(chars: readonly string[], cut: number): number {
  for (let at = cut; at >= Math.max(MIN_FRAGMENT, cut - SYLLABLE_REACH); at -= 1) {
    const [before = "", letter = "", after = ""] = chars.slice(at - 1, at + 2);
    if (LETTER.test(before) && LETTER.test(letter) && !VOWEL.test(letter) && VOWEL.test(after)) return at;
  }
  return cut;
}

/** ↩️ `label` in lines no wider than `width`: breaks at white space, then after hyphens, and splits a word only when it
 * is wider than `width` on its own, marking the split with a hyphen, carrying at least three characters over to the
 * next line when the word has six or more, and moving the split back by up to four characters to just before a
 * consonant that starts a syllable (a consonant followed by a vowel). A line keeps at least one character. */
export function wrapLabel(label: string, width: number, measure: TextMeasure): readonly string[] {
  const lines: string[] = [];
  let line = "";
  for (const word of label.split(/\s+/u)) {
    if (word === "") continue;
    word.split(HYPHEN_BREAK).forEach((piece, index) => {
      const joined = line === "" ? piece : `${line}${index === 0 ? " " : ""}${piece}`;
      if (measure(joined) <= width + FIT_TOLERANCE) {
        line = joined;
        return;
      }
      if (line !== "") lines.push(line);
      let rest = [...piece];
      while (rest.length > 1 && measure(rest.join("")) > width + FIT_TOLERANCE) {
        const cut = fittingPrefix(rest, width, measure);
        lines.push(`${rest.slice(0, cut).join("")}${HYPHEN}`);
        rest = rest.slice(cut);
      }
      line = rest.join("");
    });
  }
  if (line !== "") lines.push(line);
  return lines;
}

interface Placement {
  readonly lines: readonly string[];
  readonly side: number;
  readonly x: number;
  readonly width: number;
  readonly height: number;
  y: number;
}

function shares(left: number, right: number, room: number): readonly [number, number] {
  if (left + right <= room) return [left + (room - left - right) / 2, right + (room - left - right) / 2];
  if (right <= room / 2) return [room - right, right];
  if (left <= room / 2) return [left, room - left];
  return [room / 2, room / 2];
}

function centre(box: Placement): number {
  return box.y + box.height / 2;
}

function clearCircle(box: Placement, cx: number, clearance: number, downwards: boolean): void {
  const dx = Math.max(box.x - cx, cx - box.x - box.width, 0);
  if (dx >= clearance) return;
  const dy = Math.sqrt(clearance * clearance - dx * dx);
  if (box.y >= dy || box.y + box.height <= -dy) return;
  box.y = downwards ? dy : -dy - box.height;
}

function separate(boxes: readonly Placement[], spacing: number): void {
  for (let round = 0; round < 64; round += 1) {
    let moved = false;
    boxes.forEach((a, index) => {
      for (const b of boxes.slice(index + 1)) {
        if (a.height === 0 || b.height === 0 || a.x >= b.x + b.width || b.x >= a.x + a.width) continue;
        if (a.y >= b.y + b.height + spacing || b.y >= a.y + a.height + spacing) continue;
        const [inner, outer] = Math.abs(centre(a)) <= Math.abs(centre(b)) ? [a, b] : [b, a];
        const upwards = centre(outer) < centre(inner) || (centre(outer) === centre(inner) && centre(outer) <= 0);
        outer.y = upwards ? inner.y - spacing - outer.height : inner.y + inner.height + spacing;
        moved = true;
      }
    });
    if (!moved) return;
  }
}

function sideOf(index: number, count: number): number {
  const sin = Math.sin(radarAngle(index, count));
  return Math.abs(sin) < 1e-9 ? 0 : Math.sign(sin);
}

function piecesOf(text: string): readonly string[] {
  return text.split(/\s+/u).flatMap((word) => (word === "" ? [] : word.split(HYPHEN_BREAK)));
}

function spokeLayout(texts: readonly string[], width: number, fontSize: number, measure: TextMeasure): Omit<RadarLayout, "legend"> {
  const pad = Math.max(2, fontSize * RADAR_METRICS.pad);
  const gap = fontSize * RADAR_METRICS.gap;
  const lineHeight = fontSize * RADAR_METRICS.line;
  const room = Math.max(0, width - 2 * pad);
  const spokes = texts.map((text, index) => {
    const angle = radarAngle(index, texts.length);
    return { text, sin: Math.sin(angle), cos: Math.cos(angle), side: sideOf(index, texts.length), natural: measure(text) };
  });
  const sideSpokes = spokes.filter((spoke) => spoke.side !== 0);
  const widest = Math.max(0, ...sideSpokes.map((spoke) => spoke.natural));
  const longestPiece = Math.max(0, ...sideSpokes.flatMap((spoke) => piecesOf(spoke.text).map(measure)));
  const wanted = Math.max(Math.min(widest, fontSize * RADAR_METRICS.side), longestPiece);
  const lowest = Math.max(0, Math.min(fontSize * RADAR_METRICS.minRadius, room / 2 - gap - (sideSpokes.length === 0 ? 0 : fontSize * RADAR_METRICS.minSide)));
  const unbroken = room / 2 - gap - longestPiece;
  const smallest = unbroken >= fontSize * RADAR_METRICS.wordRadius ? Math.min(lowest, unbroken) : lowest;
  const radius = Math.min(room / 2, fontSize * RADAR_METRICS.maxRadius, Math.max(smallest, room / 2 - gap - wanted));
  const reach = radius + gap;
  const need = (side: number): number => Math.max(radius, ...spokes.map((spoke) => (spoke.side === side ? reach * Math.abs(spoke.sin) + spoke.natural : spoke.side === 0 ? spoke.natural / 2 : 0)));
  const [left, right] = shares(need(-1), need(1), room);
  const cx = pad + left;
  const placed: Placement[] = spokes.map((spoke) => {
    const px = cx + reach * spoke.sin;
    const lines = wrapLabel(spoke.text, spoke.side === 0 ? room : (spoke.side > 0 ? right : left) - reach * Math.abs(spoke.sin), measure);
    const boxWidth = Math.max(0, ...lines.map(measure));
    const height = lines.length * lineHeight;
    const x = spoke.side > 0 ? px : spoke.side < 0 ? px - boxWidth : Math.max(pad, Math.min(px - boxWidth / 2, width - pad - boxWidth));
    return { lines, side: spoke.side, x, width: boxWidth, height, y: -reach * spoke.cos - (height * (1 + spoke.cos)) / 2 };
  });
  placed.forEach((box, index) => clearCircle(box, cx, radius + gap / 2, (spokes[index]?.cos ?? 0) < 0));
  separate(placed, fontSize * RADAR_METRICS.spacing);
  const filled = placed.filter((box) => box.height > 0);
  const top = Math.min(-radius, ...filled.map((box) => box.y));
  const bottom = Math.max(radius, ...filled.map((box) => box.y + box.height));
  const cy = pad - top;
  return {
    width,
    height: bottom - top + 2 * pad,
    frame: { cx, cy, radius },
    fontSize,
    lineHeight,
    labels: placed.map((box) => ({
      lines: box.lines,
      box: { x: box.x, y: box.y + cy, width: box.width, height: box.height },
      x: box.side > 0 ? box.x : box.side < 0 ? box.x + box.width : box.x + box.width / 2,
      anchor: box.side > 0 ? "start" : box.side < 0 ? "end" : "middle",
    })),
  };
}

function startLabel(lines: readonly string[], x: number, y: number, lineHeight: number, measure: TextMeasure): RadarLabel {
  return { lines, box: { x, y, width: Math.max(0, ...lines.map(measure)), height: lines.length * lineHeight }, x, anchor: "start" };
}

/** 🗺️ Lays out a diagram with one label per spoke for `width` user units, with labels at `fontSize`.
 *
 * Proportions come from {@link RADAR_METRICS}. The side labels first get room for up to `side` font sizes (at least
 * their longest word), the circle takes the rest within `minRadius` and `maxRadius` — down to `wordRadius` when that
 * keeps the longest word whole, and less only when the width leaves side labels under `minSide`. The circle then
 * shifts sideways so the side whose labels need more room gets it; every label wraps into the room on its side (at
 * twelve and six o'clock into the whole width), stacks outward from its spoke end and stays clear of the circle,
 * overlapping labels are pushed apart away from the centre, and the height grows to hold everything.
 *
 * Where even a circle of `wordRadius` leaves a side label too little room for its longest word — long labels in a
 * narrow column or at a large text size — the spokes carry their numbers instead and the labels follow below the
 * diagram as a numbered legend across the whole width. Every box then lies inside `[0, width] × [0, height]` unless a
 * single character is wider than the room its label gets. */
export function radarLayout(labels: readonly string[], width: number, fontSize: number, measure: TextMeasure): RadarLayout {
  const texts = labels.map((label) => label.trim());
  const pad = Math.max(2, fontSize * RADAR_METRICS.pad);
  const gap = fontSize * RADAR_METRICS.gap;
  const room = Math.max(0, width - 2 * pad);
  const longest = Math.max(0, ...texts.flatMap((text, index) => (sideOf(index, texts.length) === 0 ? [] : piecesOf(text).map(measure))));
  if (room / 2 - gap - longest >= fontSize * RADAR_METRICS.wordRadius) return { ...spokeLayout(texts, width, fontSize, measure), legend: [] };
  const keys = texts.map((_, index) => String(index + 1));
  const chart = spokeLayout(keys, width, fontSize, measure);
  const indent = Math.max(0, ...keys.map(measure)) + gap;
  const spacing = fontSize * RADAR_METRICS.spacing;
  let y = chart.height - pad + gap;
  const legend = texts.map((text, index) => {
    const entry = { key: startLabel([keys[index] ?? ""], pad, y, chart.lineHeight, measure), text: startLabel(wrapLabel(text, room - indent, measure), pad + indent, y, chart.lineHeight, measure) };
    y += Math.max(1, entry.text.lines.length) * chart.lineHeight + spacing;
    return entry;
  });
  return { ...chart, height: y - spacing + pad, legend };
}
//#endregion 🏷️Labels

//#region 🕸️Chart
/** 🕸️ One localized spoke of a diagram. */
export interface RadarAxis {
  readonly id: string;
  readonly label: string;
  readonly unit: string;
  readonly min: number;
  readonly max: number;
}

interface RadarSpace {
  readonly width: number;
  readonly em: number;
  readonly family: string;
  readonly fonts: number;
}

const RULER_EM = 10;
const FALLBACK_WIDTH_EM = 18;
const FALLBACK_SPACE: RadarSpace = { width: FALLBACK_WIDTH_EM * 16, em: 16, family: "sans-serif", fonts: 0 };
const RINGS = [0.25, 0.5, 0.75, 1];
const VALUE_CELL = "quiz-nowrap border-b border-normal px-single py-single text-left align-top tabular-nums";
const VALUE_HEAD = "quiz-nowrap border-b-2 border-normal px-single py-single text-left align-top font-semibold";
const VALUE_ROW_HEAD = "border-b border-normal px-single py-single text-left align-top font-semibold";

function sameSpace(a: RadarSpace, b: RadarSpace): boolean {
  return a.width === b.width && a.em === b.em && a.family === b.family && a.fonts === b.fonts;
}

function useRadarSpace(figure: RefObject<HTMLElement | null>, ruler: RefObject<HTMLElement | null>): RadarSpace {
  const [space, setSpace] = useState(FALLBACK_SPACE);
  useLayoutEffect(() => {
    const element = figure.current;
    const probe = ruler.current;
    if (element === null || probe === null) return undefined;
    let live = true;
    let fonts = 0;
    const update = (): void => {
      if (!live) return;
      const style = getComputedStyle(element);
      const em = probe.getBoundingClientRect().width / RULER_EM || Number.parseFloat(style.fontSize) || FALLBACK_SPACE.em;
      const measured = Math.floor(element.getBoundingClientRect().width);
      const next: RadarSpace = { width: measured > 0 ? measured : FALLBACK_WIDTH_EM * em, em, family: style.fontFamily || FALLBACK_SPACE.family, fonts };
      setSpace((current) => (sameSpace(current, next) ? current : next));
    };
    const loaded = (): void => {
      fonts += 1;
      update();
    };
    update();
    const observer = typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(update);
    observer?.observe(element);
    observer?.observe(probe);
    const faces = (document as { readonly fonts?: FontFaceSet }).fonts;
    faces?.addEventListener("loadingdone", loaded);
    void faces?.ready.then(loaded);
    return () => {
      live = false;
      observer?.disconnect();
      faces?.removeEventListener("loadingdone", loaded);
    };
  }, [figure, ruler]);
  return space;
}

function fixed(value: number): number {
  return Math.round(value * 100) / 100;
}

function RadarText(props: { readonly className: string; readonly label: RadarLabel; readonly lineHeight: number }): ReactElement {
  const { className, label, lineHeight } = props;
  return (
    <text className={className} x={fixed(label.x)} textAnchor={label.anchor} dominantBaseline="central">
      {label.lines.map((line, row) => (
        <tspan key={row} x={fixed(label.x)} y={fixed(label.box.y + (row + 0.5) * lineHeight)} dominantBaseline="central">
          {line}
        </tspan>
      ))}
    </text>
  );
}

/** 🕸️ The spider diagram of `values` over `axes`, named `name`, laid out for the width it gets, with its value table. */
export function RadarChart(props: { readonly name: string; readonly axes: readonly RadarAxis[]; readonly values: Readonly<Record<string, number>>; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { name, axes, values, text, locale } = props;
  const titleId = useId();
  const figure = useRef<HTMLElement>(null);
  const ruler = useRef<HTMLSpanElement>(null);
  const space = useRadarSpace(figure, ruler);
  const labelKey = JSON.stringify(axes.map((axis) => axis.label));
  const layout = useMemo(() => {
    const fontSize = space.em * RADAR_LABEL_EM;
    return radarLayout(JSON.parse(labelKey) as string[], space.width, fontSize, textMeasure(fontSize, space.family));
  }, [labelKey, space]);
  const { frame } = layout;
  const count = axes.length;
  return (
    <figure ref={figure} className="quiz-radar m-0 flex flex-col gap-single">
      <span ref={ruler} className="quiz-radar-ruler" style={{ width: `${RULER_EM}em` }} aria-hidden="true" />
      <svg className="quiz-radar-chart" viewBox={`0 0 ${fixed(layout.width)} ${fixed(layout.height)}`} width={fixed(layout.width)} height={fixed(layout.height)} role="img" aria-labelledby={titleId}>
        <title id={titleId}>{text("quiz.radar.label", { name })}</title>
        {RINGS.map((ring) => (
          <polygon key={ring} className="quiz-radar-ring" points={pointList(axes.map((_, index) => radarPoint(frame, index, count, ring)))} />
        ))}
        {axes.map((axis, index) => {
          const end = radarPoint(frame, index, count, 1);
          return <line key={axis.id} className="quiz-radar-spoke" x1={fixed(frame.cx)} y1={fixed(frame.cy)} x2={fixed(end.x)} y2={fixed(end.y)} />;
        })}
        <polygon className="quiz-radar-area" points={pointList(radarPolygon(frame, axes, values))} />
        <g className="quiz-radar-labels" fontSize={fixed(layout.fontSize)}>
          {layout.labels.map((label, index) => (
            <RadarText key={axes[index]?.id ?? index} className={layout.legend.length === 0 ? "quiz-radar-label" : "quiz-radar-key"} label={label} lineHeight={layout.lineHeight} />
          ))}
          {layout.legend.map((entry, index) => (
            <g key={axes[index]?.id ?? index} className="quiz-radar-legend">
              <RadarText className="quiz-radar-key" label={entry.key} lineHeight={layout.lineHeight} />
              <RadarText className="quiz-radar-label" label={entry.text} lineHeight={layout.lineHeight} />
            </g>
          ))}
        </g>
      </svg>
      <details className="text-sm">
        <summary className="quiz-target flex cursor-pointer items-center">{text("quiz.radar.table")}</summary>
        <div className="max-w-full overflow-x-auto">
          <table className="w-full border-collapse">
            <caption className="sr-only">{text("quiz.radar.label", { name })}</caption>
            <thead>
              <tr>
                <th scope="col" className={VALUE_HEAD}>
                  {text("quiz.radar.axis")}
                </th>
                <th scope="col" className={VALUE_HEAD}>
                  {text("quiz.radar.value")}
                </th>
                <th scope="col" className={VALUE_HEAD}>
                  {text("quiz.radar.minimum")}
                </th>
                <th scope="col" className={VALUE_HEAD}>
                  {text("quiz.radar.maximum")}
                </th>
              </tr>
            </thead>
            <tbody>
              {axes.map((axis) => (
                <tr key={axis.id}>
                  <th scope="row" className={VALUE_ROW_HEAD}>
                    {axis.label}
                  </th>
                  <td className={VALUE_CELL}>{axis.id in values ? withUnit(formatNumber(values[axis.id] ?? 0, locale), axis.unit) : "–"}</td>
                  <td className={VALUE_CELL}>{withUnit(formatNumber(axis.min, locale), axis.unit)}</td>
                  <td className={VALUE_CELL}>{withUnit(formatNumber(axis.max, locale), axis.unit)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </details>
    </figure>
  );
}
//#endregion 🕸️Chart
