/** 🔤️ The room the texts of a drawing take as the browser sets them, beyond the room the drawing reserved for them.
 *
 * A drawing reserves the Helvetica width of every text (`textWidthMm` times `TEXT_SLACK`) and names a font stack whose first face may be wider; on a page that has that face, a caption at the edge of the paper is longer than its room and is cut there.
 * {@link textOverhang} measures every text in the face the browser uses and states how far the widest one reaches past its room to the left and to the right, so the window around the drawing can take it in. A digit counts as the widest digit, so a caption that states a running time has the same overhang on every frame.
 * Nothing is measured without a document and a 2D canvas (a worker, a test in jsdom): the overhang is none there. A face that finishes loading changes the measure: {@link subscribeTextEpoch} tells.
 *
 * @see ../🟦️.ts — `sceneExtent` and `sceneWindow`, which add the overhang
 * @see ./⚛️hook/🟦️.tsx — the React reader of the epoch
 * @see ../../../../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts — the reserved room
 * @see https://developer.mozilla.org/docs/Web/API/CanvasRenderingContext2D/measureText — the measure
 */
import { TEXT_SLACK } from "../../../../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { writeSvg } from "../../../../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { textWidthMm } from "../../../../../🔨️modules/📐️units/🟦️.ts";
import type { DrawingScene } from "@teaching/staffage";

/** 📏️ The width of `text` in ems, set regular or bold in the font stack `family`. */
export type TextMeasure = (text: string, bold: boolean, family: string) => number;

/** ↔️ How far the texts of a scene reach past their reserved room, in paper millimetres. */
export type TextOverhang = { readonly left: number; readonly right: number };

type Layer = DrawingScene["layers"][number];
type TextItem = Extract<Layer["items"][number], { readonly kind: "text" }>;

const NONE: TextOverhang = { left: 0, right: 0 };
const EM_PX = 100;
const DIGIT = /\d/gu;
const KEPT_WIDTHS = 4096;
const listeners = new Set<() => void>();
const widths = new WeakMap<TextMeasure, Map<string, number>>();
let overhangs = new WeakMap<DrawingScene, TextOverhang>();
let family: string | undefined;
let canvas: TextMeasure | null | undefined;
let epoch = 0;
let watching = false;

/** 🔡️ The font stack the drawing writer names for a text. */
function familyOf(scene: DrawingScene, layer: Layer, item: TextItem): string {
  const written = /font-family="([^"]*)"/u.exec(writeSvg({ ...scene, layers: [{ ...layer, items: [item] }], legends: [] }))?.[1] ?? "sans-serif";
  return written.replaceAll("&apos;", "'").replaceAll("&quot;", '"').replaceAll("&amp;", "&");
}

/** 🖌️ The measure of the document's own faces, or none where there is no document or no 2D canvas. */
function canvasMeasure(): TextMeasure | undefined {
  if (canvas !== undefined) return canvas ?? undefined;
  const context = typeof document === "undefined" || typeof OffscreenCanvas !== "function" ? null : new OffscreenCanvas(1, 1).getContext("2d");
  canvas = context === null ? null : (text, bold, stack) => {
    context.font = `${bold ? "bold " : ""}${EM_PX}px ${stack}`;
    return context.measureText(text).width / EM_PX;
  };
  if (canvas !== null && !watching && document.fonts?.addEventListener !== undefined) {
    watching = true;
    document.fonts.addEventListener("loadingdone", () => {
      const own = canvas ?? undefined;
      if (own !== undefined) widths.delete(own);
      overhangs = new WeakMap();
      epoch += 1;
      for (const listener of listeners) listener();
    });
  }
  return canvas ?? undefined;
}

function ems(measure: TextMeasure, text: string, bold: boolean, stack: string): number {
  let kept = widths.get(measure);
  if (kept === undefined || kept.size > KEPT_WIDTHS) widths.set(measure, (kept = new Map()));
  const key = `${bold ? "b" : "r"}${text}`;
  let width = kept.get(key);
  if (width === undefined) kept.set(key, (width = measure(text, bold, stack)));
  return width;
}

/** 🔢️ The width in ems of a text whose every digit is as wide as the widest digit of the face. */
function steadyEms(measure: TextMeasure, text: string, bold: boolean, stack: string): number {
  const digits = text.match(DIGIT)?.length ?? 0;
  if (digits === 0) return ems(measure, text, bold, stack);
  const widest = Math.max(...Array.from("0123456789", (digit) => ems(measure, digit, bold, stack)));
  const blank = text.replace(DIGIT, "0");
  return ems(measure, blank, bold, stack) + digits * (widest - ems(measure, "0", bold, stack));
}

/** ↔️ How far the widest text of `scene` reaches past its reserved room on each side; none when every text fits or nothing can be measured. */
export function textOverhang(scene: DrawingScene, measure: TextMeasure | undefined = canvasMeasure()): TextOverhang {
  if (measure === undefined) return NONE;
  const own = measure === canvas;
  const known = own ? overhangs.get(scene) : undefined;
  if (known !== undefined) return known;
  let left = 0;
  let right = 0;
  for (const layer of scene.layers) {
    for (const item of layer.items) {
      if (item.kind !== "text") continue;
      const bold = item.weight === "bold";
      const extra = steadyEms(measure, item.text, bold, (family ??= familyOf(scene, layer, item))) * item.sizeMm - textWidthMm(item.text, item.sizeMm, bold) * TEXT_SLACK;
      if (extra <= 0) continue;
      if (item.anchor !== "start") left = Math.max(left, item.anchor === "end" ? extra : extra / 2);
      if (item.anchor !== "end") right = Math.max(right, item.anchor === "start" ? extra : extra / 2);
    }
  }
  const overhang = left === 0 && right === 0 ? NONE : { left, right };
  if (own) overhangs.set(scene, overhang);
  return overhang;
}

/** 🕰️ Counts the times a face finished loading since the page started: a measure taken before is stale. */
export const textEpoch = (): number => epoch;

/** 📣️ Calls `listener` whenever {@link textEpoch} changes; returns the unsubscribe. */
export function subscribeTextEpoch(listener: () => void): () => void {
  canvasMeasure();
  listeners.add(listener);
  return () => void listeners.delete(listener);
}
