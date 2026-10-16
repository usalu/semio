/** 🧫️ Typed admission of the neutral JSON fixtures: plain JSON numbers (the schema's `Binary64Transport` number branch) become Binary64 words exactly once, then the owning parser admits the model. */
import type { ValidateFunction } from "ajv";
import { binary64 } from "@semio-tech/framework/value/ieee754";
import { parseBlendMode, parseDrawingArtifact, parseDrawingImageAsset, parseDrawingLayerNode, parsePathGeometrySegment, type DrawingArtifact, type DrawingImageAsset, type DrawingLayerNode, type PathGeometrySegment } from "../../🧬️schema/🟦️.ts";
import type { RasterSceneAsset, RasterSceneGroup } from "../../🧬️schema/🎬️scene/📷️raster/🟦️.ts";
import type { FramingNode } from "../../🧬️schema/🧮️geometry/📷️framing/🟦️.ts";
import type { Matrix } from "../../🧬️schema/🧮️geometry/🟦️.ts";

/** 🔢️ Capture every JSON number below `value` as its Binary64 word; strings, booleans and null stay as authored. */
export function drawingWords(value: unknown): unknown {
  if (typeof value === "number") return binary64(value);
  if (Array.isArray(value)) return value.map(drawingWords);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, drawingWords(item)]));
  return value;
}

/** 🗂️ Layers of a neutral fixture document, admitted by the owning layer parser. */
export function drawingLayersFixture(layers: unknown): DrawingLayerNode[] {
  if (!Array.isArray(layers)) throw new TypeError("fixture layers must be an array");
  return layers.map((layer, index) => parseDrawingLayerNode(drawingWords(layer), `$.layers[${index}]`));
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
  return parseDrawingArtifact({ ...document, layers: drawingWords(document.layers), assets: document.assets ?? {}, artboard: drawingWords(document.artboard) });
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
