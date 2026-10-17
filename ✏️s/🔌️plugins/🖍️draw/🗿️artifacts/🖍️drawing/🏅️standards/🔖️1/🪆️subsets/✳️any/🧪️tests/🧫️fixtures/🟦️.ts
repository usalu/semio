/** 🧫️ Typed admission of the neutral JSON fixtures: plain JSON numbers (the schema's `Binary64Transport` number branch) become Binary64 words exactly once, then the owning parser admits the model. */
import { readFileSync } from "node:fs";
import type { ValidateFunction } from "ajv";
import { unicodeSourceUrls } from "@semio-tech/framework/2d/text/font/shape/unicode";
import type { FillStyle, StrokeStyle } from "@semio-tech/framework/2d";
import { binary64, binary64Value } from "@semio-tech/framework/value/ieee754";
import { parseBlendMode, parseDrawingArtifact, parseDrawingImageAsset, parseDrawingLayerNode, parsePathGeometrySegment, type DrawingArtifact, type DrawingImageAsset, type DrawingLayerNode, type PathGeometrySegment } from "../../🧬️schema/🟦️.ts";
import type { RasterSceneAsset, RasterSceneGroup } from "../../🧬️schema/🎬️scene/📷️raster/🟦️.ts";
import type { FramingNode } from "../../🧬️schema/🧮️geometry/📷️framing/🟦️.ts";
import type { PaintedPathQuery } from "../../🧬️schema/🧮️geometry/🎯️picking/🎨️paint/🟦️.ts";
import { parseStrokeCap, parseStrokeJoin } from "../../🧬️schema/🖊️stroke/🟦️.ts";
import type { Matrix } from "../../🧬️schema/🧮️geometry/🟦️.ts";
import type { DrawingSvgNode } from "../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🟦️.ts";

/** 🔢️ Capture every JSON number below `value` as its Binary64 word; strings, booleans and null stay as authored. */
export function drawingWords(value: unknown): unknown {
  if (typeof value === "number") return binary64(value);
  if (Array.isArray(value)) return value.map(drawingWords);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, drawingWords(item)]));
  return value;
}

/** 🧮️ Derive the numeric view of every Binary64 word below `value`: the single computational boundary of a test that walks imported models (the inverse of `drawingWords`). */
export function drawingNumbers(value: unknown): any {
  if (value !== null && typeof value === "object" && "bits" in value && typeof value.bits === "bigint") return binary64Value({ bits: value.bits });
  if (Array.isArray(value)) return value.map(drawingNumbers);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, drawingNumbers(item)]));
  return value;
}

/** 🕳️ Neutral fixtures spell an absent optional field as `null`; the Binary64 model has no nullable field, so absence is the omitted key. */
export function absentNulls(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(absentNulls);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([, item]) => item !== null).map(([key, item]) => [key, absentNulls(item)]));
  return value;
}

/** 🗂️ Layers of a neutral fixture document, admitted by the owning layer parser. */
export function drawingLayersFixture(layers: unknown): DrawingLayerNode[] {
  if (!Array.isArray(layers)) throw new TypeError("fixture layers must be an array");
  return layers.map((layer, index) => parseDrawingLayerNode(drawingWords(absentNulls(layer)), `$.layers[${index}]`));
}

/** 🖼️ Integer RGBA assets of a neutral fixture document, admitted by the owning asset parser (assets stay plain integers). */
export function drawingAssetsFixture(assets: unknown): Record<string, DrawingImageAsset> {
  if (assets === null || typeof assets !== "object" || Array.isArray(assets)) throw new TypeError("fixture assets must be an object");
  return Object.fromEntries(Object.entries(assets).map(([key, asset]) => [key, parseDrawingImageAsset(asset, `$.assets.${key}`)]));
}

/** 📐 Authored path geometry of a neutral fixture, admitted segment by segment (geometry stays numeric). */
export function pathSegmentsFixture(segments: unknown): PathGeometrySegment[] {
  if (!Array.isArray(segments)) throw new TypeError("fixture segments must be an array");
  return segments.map((segment, index) => parsePathGeometrySegment(segment, `$.segments[${index}]`));
}

/** 📄 A whole neutral fixture document: layers and artboard become Binary64 words, assets stay integers, and the artifact parser admits the result. */
export function drawingArtifactFixture(document: { readonly schema: string; readonly id: string; readonly title?: string; readonly layers: unknown; readonly assets?: unknown; readonly artboard?: unknown }): DrawingArtifact {
  return parseDrawingArtifact({ ...document, layers: drawingWords(absentNulls(document.layers)), assets: document.assets ?? {}, artboard: drawingWords(absentNulls(document.artboard)) });
}

/** 🛂️ Admit a neutral fixture through the compiled JSON Schema that owns its shape; the type guard narrows the value, so no cast is needed. */
export function admitted<T>(validate: ValidateFunction<T>, value: unknown, label = "fixture"): T {
  if (!validate(value)) throw new TypeError(`${label} violates its schema: ${JSON.stringify(validate.errors)}`);
  return value;
}

/** ☣️ Deliberately unchecked input for refusal tests: the unit under test is the runtime refusal of data the static type forbids, so no typed admission exists for it. */
export function hostile<T>(value: unknown): T {
  return value as T;
}

/** 🎞️ A neutral group record as the raster scene consumes it: the blend name is admitted by the owning blend parser. */
export function rasterSceneGroupFixture(group: { readonly id: string; readonly opacity: number; readonly blendMode: string }): RasterSceneGroup {
  return { id: group.id, opacity: group.opacity, blendMode: parseBlendMode(group.blendMode) };
}

/** 🖼️ A neutral asset record as the raster scene consumes it: the JSON sample list becomes the native byte array. */
export function rasterSceneAssetFixture(asset: { readonly id: string; readonly image: { readonly width: number; readonly height: number; readonly pixels: readonly number[] } }): RasterSceneAsset {
  return { id: asset.id, image: { width: asset.image.width, height: asset.image.height, pixels: Uint8Array.from(asset.image.pixels) } };
}

const fixtureRecord = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  return value as Record<string, unknown>;
};
const fixtureNumber = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be a finite number`);
  return value;
};

/** 📐 A six-element affine matrix of a neutral fixture, admitted element by element. */
export function matrixFixture(value: unknown, at = "$.transform"): Matrix {
  if (!Array.isArray(value) || value.length !== 6) throw new TypeError(`${at} must hold six numbers`);
  return [fixtureNumber(value[0], `${at}[0]`), fixtureNumber(value[1], `${at}[1]`), fixtureNumber(value[2], `${at}[2]`), fixtureNumber(value[3], `${at}[3]`), fixtureNumber(value[4], `${at}[4]`), fixtureNumber(value[5], `${at}[5]`)];
}

/** 📷️ Scene nodes of a neutral framing fixture, admitted field by field (geometry stays numeric). */
export function framingNodesFixture(value: unknown): FramingNode[] {
  if (!Array.isArray(value)) throw new TypeError("fixture nodes must be an array");
  return value.map((item, index) => {
    const at = `$.nodes[${index}]`, row = fixtureRecord(item, at);
    const stroke = row.stroke === undefined ? undefined : { width: fixtureNumber(fixtureRecord(row.stroke, `${at}.stroke`).width, `${at}.stroke.width`) };
    const image = row.image === undefined ? undefined : (({ width, height }) => ({ width: fixtureNumber(width, `${at}.image.width`), height: fixtureNumber(height, `${at}.image.height`) }))(fixtureRecord(row.image, `${at}.image`));
    const text = row.text === undefined ? undefined : (({ content, size }) => ({ content: String(content), size: fixtureNumber(size, `${at}.text.size`) }))(fixtureRecord(row.text, `${at}.text`));
    if (typeof row.visible !== "boolean") throw new TypeError(`${at}.visible must be a boolean`);
    return { transform: matrixFixture(row.transform, `${at}.transform`), segments: pathSegmentsFixture(row.segments), visible: row.visible, opacity: fixtureNumber(row.opacity, `${at}.opacity`), ...(stroke && { stroke }), ...(image && { image }), ...(text && { text }) };
  });
}

const fixturePoint = (value: unknown, at: string): readonly [number, number] => {
  if (!Array.isArray(value) || value.length !== 2) throw new TypeError(`${at} must hold two numbers`);
  return [fixtureNumber(value[0], `${at}[0]`), fixtureNumber(value[1], `${at}[1]`)];
};

/** 🎨️ A painted-path query of a neutral fixture, admitted field by field (the query is numeric geometry). */
export function paintedPathQueryFixture(value: unknown): PaintedPathQuery {
  const row = fixtureRecord(value, "$.query");
  if (!Array.isArray(row.transform) || typeof row.fill !== "boolean") throw new TypeError("painted path query needs a transform list and a fill flag");
  if (row.fillRule !== "nonzero" && row.fillRule !== "evenodd") throw new TypeError("painted path query needs a fill rule");
  const stroke = row.stroke === null ? null : (({ width, cap, join, dash }) => {
    if (!Array.isArray(dash)) throw new TypeError("$.query.stroke.dash must be an array");
    return { width: fixtureNumber(width, "$.query.stroke.width"), cap: parseStrokeCap(cap), join: parseStrokeJoin(join), dash: dash.map((item, index) => fixtureNumber(item, `$.query.stroke.dash[${index}]`)) };
  })(fixtureRecord(row.stroke, "$.query.stroke"));
  return { point: fixturePoint(row.point, "$.query.point"), transform: row.transform.map((item, index) => fixtureNumber(item, `$.query.transform[${index}]`)), tolerance: fixtureNumber(row.tolerance, "$.query.tolerance"), flatness: fixtureNumber(row.flatness, "$.query.flatness"), fill: row.fill, fillRule: row.fillRule, stroke };
}

const fixtureColor = (value: unknown, at: string): readonly [number, number, number, number] => {
  if (!Array.isArray(value) || value.length !== 4) throw new TypeError(`${at} must hold four numbers`);
  return [fixtureNumber(value[0], `${at}[0]`), fixtureNumber(value[1], `${at}[1]`), fixtureNumber(value[2], `${at}[2]`), fixtureNumber(value[3], `${at}[3]`)];
};
const fixtureStops = (value: unknown, at: string) => {
  if (!Array.isArray(value)) throw new TypeError(`${at} must be an array`);
  return value.map((item, index) => {
    const stop = fixtureRecord(item, `${at}[${index}]`);
    return { offset: fixtureNumber(stop.offset, `${at}[${index}].offset`), color: fixtureColor(stop.color, `${at}[${index}].color`) };
  });
};

/** 🎨️ A canvas-neutral fill of a neutral fixture, admitted field by field (numeric paint, including values the renderer must refuse). */
export function fillStyleFixture(value: unknown, at = "$.fill"): FillStyle {
  const row = fixtureRecord(value, at);
  if (row.kind === "solid") return { kind: "solid", color: fixtureColor(row.color, `${at}.color`) };
  if (row.kind === "linearGradient") return { kind: "linearGradient", x1: fixtureNumber(row.x1, `${at}.x1`), y1: fixtureNumber(row.y1, `${at}.y1`), x2: fixtureNumber(row.x2, `${at}.x2`), y2: fixtureNumber(row.y2, `${at}.y2`), stops: fixtureStops(row.stops, `${at}.stops`) };
  if (row.kind === "radialGradient") return { kind: "radialGradient", cx: fixtureNumber(row.cx, `${at}.cx`), cy: fixtureNumber(row.cy, `${at}.cy`), r: fixtureNumber(row.r, `${at}.r`), stops: fixtureStops(row.stops, `${at}.stops`) };
  throw new TypeError(`${at}.kind must name a fill`);
}

/** 🖊️ A canvas-neutral stroke of a neutral fixture, admitted field by field. */
export function strokeStyleFixture(value: unknown, at = "$.stroke"): StrokeStyle {
  const row = fixtureRecord(value, at);
  const dash = row.dash === undefined ? undefined : Array.isArray(row.dash) ? row.dash.map((item, index) => fixtureNumber(item, `${at}.dash[${index}]`)) : (() => { throw new TypeError(`${at}.dash must be an array`); })();
  return { color: fixtureColor(row.color, `${at}.color`), width: fixtureNumber(row.width, `${at}.width`), cap: parseStrokeCap(row.cap), join: parseStrokeJoin(row.join), ...(dash && { dash }) };
}

/** 🧩️ Scene nodes of a neutral SVG/compositing fixture, admitted field by field (the scene is numeric geometry). */
export function svgNodesFixture(value: unknown): DrawingSvgNode[] {
  if (!Array.isArray(value)) throw new TypeError("fixture nodes must be an array");
  return value.map((item, index) => {
    const at = `$.nodes[${index}]`, row = fixtureRecord(item, at);
    if (typeof row.id !== "string" || typeof row.blendMode !== "string" || typeof row.visible !== "boolean" || !Array.isArray(row.transform)) throw new TypeError(`${at} needs an id, blend mode, visibility flag and transform list`);
    const groups = row.groups === undefined ? undefined : Array.isArray(row.groups) ? row.groups.map((group, groupIndex) => {
      const entry = fixtureRecord(group, `${at}.groups[${groupIndex}]`);
      if (typeof entry.id !== "string" || typeof entry.blendMode !== "string") throw new TypeError(`${at}.groups[${groupIndex}] needs an id and a blend mode`);
      return { id: entry.id, opacity: fixtureNumber(entry.opacity, `${at}.groups[${groupIndex}].opacity`), blendMode: entry.blendMode };
    }) : (() => { throw new TypeError(`${at}.groups must be an array`); })();
    const text = row.text === undefined ? undefined : (({ content, size }) => ({ content: String(content), size: fixtureNumber(size, `${at}.text.size`) }))(fixtureRecord(row.text, `${at}.text`));
    const image = row.image === undefined ? undefined : (({ assetId, width, height }) => ({ assetId: String(assetId), width: fixtureNumber(width, `${at}.image.width`), height: fixtureNumber(height, `${at}.image.height`) }))(fixtureRecord(row.image, `${at}.image`));
    return { id: row.id, transform: row.transform.map((entry, entryIndex) => fixtureNumber(entry, `${at}.transform[${entryIndex}]`)), segments: pathSegmentsFixture(row.segments), opacity: fixtureNumber(row.opacity, `${at}.opacity`), blendMode: row.blendMode, visible: row.visible, ...(groups && { groups }), ...(row.fill !== undefined && { fill: fillStyleFixture(row.fill, `${at}.fill`) }), ...(row.stroke !== undefined && { stroke: strokeStyleFixture(row.stroke, `${at}.stroke`) }), ...(typeof row.fillRule === "string" && { fillRule: row.fillRule }), ...(text && { text }), ...(image && { image }) };
  });
}

/** 🌐️ The original Unicode record of the framework, joined from its two data parts in byte order exactly as the Rust crates join them. */
export function unicodeSourceBytes(): Uint8Array {
  return new Uint8Array(Buffer.concat(unicodeSourceUrls.map(url => readFileSync(url))));
}
