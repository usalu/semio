import { sharpPrintRasterEngineV1 } from "./🖼️raster/🔌️sharp/🟦️.ts";
import limits from "../📋️intent/🧬️schema/🔣️.json" with { type: "json" };

/** 🖼️ Renders one self-contained authored SVG into bounded PNG bytes behind the Print resource interface. */
export async function renderPrintSvgResourceV1(source: Uint8Array, width: number, signal: AbortSignal): Promise<Uint8Array> {
  const policy = limits.definitions.Policy.const;
  signal.throwIfAborted();
  if (!Number.isSafeInteger(width) || width < 1 || width > 4096 || source.byteLength > policy.maximumResourceFileBytes) throw Error("Bounded SVG resource required");
  const text = new TextDecoder("utf-8", { fatal: true }).decode(source);
  if (!/<svg\b/u.test(text) || /<!DOCTYPE|<!ENTITY|(?:href|src)\s*=|url\s*\(/iu.test(text)) throw Error("Self-contained SVG resource required");
  const root = text.match(/<svg\b[^>]*>/u)?.[0], viewBox = root?.match(/\bviewBox\s*=\s*["']([^"']+)["']/u)?.[1]?.trim().split(/[\s,]+/u).map(Number);
  if (!root || viewBox?.length !== 4 || !viewBox.every(Number.isFinite) || !(viewBox[2]! > 0) || !(viewBox[3]! > 0)) throw Error("Finite SVG viewBox required");
  const height = Math.ceil(width * viewBox[3]! / viewBox[2]!);
  if (!Number.isSafeInteger(height) || height < 1 || width * height > policy.maximumPagePixels) throw Error("Bounded SVG resource dimensions required");
  const viewport = root.replace(/\s(?:width|height)\s*=\s*(?:"[^"]*"|'[^']*')/gu, "").replace(/>$/u, ` width="${width}px" height="${height}px">`);
  const projected = text.replace(root, viewport);
  const bytes = await sharpPrintRasterEngineV1.svgToPng({ svg: new TextEncoder().encode(projected), width, maximumPixels: policy.maximumPagePixels }, signal);
  if (bytes.byteLength > policy.maximumPngBytes) throw Error("Bounded PNG resource required");
  return bytes;
}
