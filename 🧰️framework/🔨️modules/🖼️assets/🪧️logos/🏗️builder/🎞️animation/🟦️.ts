import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { exportAnimatedSvgToMp4, repoToolCacheEnv } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

type OwnedSvgElement = { getAttribute(name: string): string | null; querySelector(selector: string): OwnedSvgElement | null };
type OwnedSvgDocument = { querySelectorAll(selector: string): Iterable<OwnedSvgElement> };
type OwnedJsDomConstructor = new (markup: string, options: { contentType: string }) => { window: { document: OwnedSvgDocument } };
const { JSDOM } = createRequire(import.meta.url)("jsdom") as { JSDOM: OwnedJsDomConstructor };
const assetsRoot = (): string => join(import.meta.dir, "..", "..", "..");

//#region ⚙️Kinds
interface LogoTransformData {
  translate: { x: number; y: number };
  rotate: { angle: number; cx: number; cy: number };
  scale: { x: number; y: number };
}

interface LogoGroupData {
  id: string;
  transform: LogoTransformData;
  path: { d: string; fill: string; stroke: string; strokeWidth: string };
}

interface LogoKeyframeData {
  groups: LogoGroupData[];
}
//#endregion ⚙️Kinds

//#region 🔢️Helpers
function normalizeLogoNumber(value: number): number {
  if (!Number.isFinite(value)) return 0;
  return Number(value.toFixed(6));
}

function escapeLogoXmlAttribute(value: string): string {
  return value.replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function createLogoIdentityTransform(): LogoTransformData {
  return { translate: { x: 0, y: 0 }, rotate: { angle: 0, cx: 0, cy: 0 }, scale: { x: 1, y: 1 } };
}
//#endregion 🔢️Helpers

//#region 🧮️TransformParsing
function logoTransformToMatrix(translate: { x: number; y: number }, rotate: { angle: number; cx: number; cy: number }, scale: { x: number; y: number }): string {
  const angleRadians = (rotate.angle * Math.PI) / 180;
  const cosine = Math.cos(angleRadians);
  const sine = Math.sin(angleRadians);
  const scaleX = scale.x === 0 ? 1 : scale.x;
  const scaleY = scale.y === 0 ? 1 : scale.y;
  const a = normalizeLogoNumber(cosine * scaleX);
  const b = normalizeLogoNumber(sine * scaleX);
  const c = normalizeLogoNumber(-sine * scaleY);
  const d = normalizeLogoNumber(cosine * scaleY);
  const e = normalizeLogoNumber(translate.x + rotate.cx - rotate.cx * a - rotate.cy * c);
  const f = normalizeLogoNumber(translate.y + rotate.cy - rotate.cx * b - rotate.cy * d);
  return `${a} ${b} ${c} ${d} ${e} ${f}`;
}

function parseLogoMatrixTransform(valuesText: string): LogoTransformData {
  const values = valuesText.split(/[,\s]+/).filter(Boolean).map(Number);
  if (values.length !== 6 || values.some((value) => !Number.isFinite(value))) {
    return createLogoIdentityTransform();
  }
  const [a, b, c, d, e, f] = values;
  const scaleX = Math.hypot(a, b) || 1;
  const determinant = a * d - b * c;
  const scaleY = determinant === 0 ? 1 : determinant / scaleX;
  const angle = Math.atan2(b, a) * (180 / Math.PI);
  return {
    translate: { x: normalizeLogoNumber(e), y: normalizeLogoNumber(f) },
    rotate: { angle: normalizeLogoNumber(angle), cx: 0, cy: 0 },
    scale: { x: normalizeLogoNumber(scaleX), y: normalizeLogoNumber(scaleY) },
  };
}

function parseLogoTransform(transformText: string | null): LogoTransformData {
  const result = createLogoIdentityTransform();
  if (!transformText) return result;
  const matrixMatch = transformText.match(/matrix\(([^)]+)\)/);
  if (matrixMatch) return parseLogoMatrixTransform(matrixMatch[1]);
  const translateMatch = transformText.match(/translate\(([^)]+)\)/);
  if (translateMatch) {
    const values = translateMatch[1].split(/[,\s]+/).filter(Boolean).map(Number);
    result.translate.x = normalizeLogoNumber(values[0] ?? 0);
    result.translate.y = normalizeLogoNumber(values[1] ?? 0);
  }
  const rotateMatch = transformText.match(/rotate\(([^)]+)\)/);
  if (rotateMatch) {
    const values = rotateMatch[1].split(/[,\s]+/).filter(Boolean).map(Number);
    result.rotate.angle = normalizeLogoNumber(values[0] ?? 0);
    result.rotate.cx = normalizeLogoNumber(values[1] ?? 0);
    result.rotate.cy = normalizeLogoNumber(values[2] ?? 0);
  }
  const scaleMatch = transformText.match(/scale\(([^)]+)\)/);
  if (scaleMatch) {
    const values = scaleMatch[1].split(/[,\s]+/).filter(Boolean).map(Number);
    result.scale.x = normalizeLogoNumber(values[0] ?? 1) || 1;
    result.scale.y = normalizeLogoNumber(values[1] ?? values[0] ?? 1) || 1;
  }
  return result;
}
//#endregion 🧮️TransformParsing

//#region 🎈️SvgParsing
function parseLogoSvgFile(filePath: string): LogoKeyframeData {
  const svgContent = readFileSync(filePath, "utf-8");
  const dom = new JSDOM(svgContent, { contentType: "image/svg+xml" });
  const document = dom.window.document;
  const groups: LogoGroupData[] = [];
  for (const groupElement of document.querySelectorAll("g[id]")) {
    const id = groupElement.getAttribute("id");
    const pathElement = groupElement.querySelector("path");
    if (!id || !pathElement) continue;
    groups.push({
      id,
      transform: parseLogoTransform(groupElement.getAttribute("transform")),
      path: {
        d: pathElement.getAttribute("d") ?? "",
        fill: pathElement.getAttribute("fill") ?? "none",
        stroke: pathElement.getAttribute("stroke") ?? "none",
        strokeWidth: pathElement.getAttribute("stroke-width") ?? "0",
      },
    });
  }
  return { groups };
}
//#endregion 🎈️SvgParsing

//#region 📮️SequenceGeneration
function generateLogoKeyframeSequence(keyframes: LogoKeyframeData[]): LogoKeyframeData[] {
  if (keyframes.length <= 1) return [...keyframes];
  const sequence: LogoKeyframeData[] = [];
  for (const keyframe of keyframes) sequence.push(keyframe, keyframe, keyframe);
  for (let index = keyframes.length - 2; index > 0; index -= 1) sequence.push(keyframes[index], keyframes[index]);
  sequence.push(keyframes[0]);
  return sequence;
}
//#endregion 📮️SequenceGeneration

//#region 📻️AnimatedSvgOutput
function createLogoAnimatedSVG(keyframes: LogoKeyframeData[], outputPath: string): void {
  const sequence = generateLogoKeyframeSequence(keyframes);
  if (sequence.length === 0) {
    throw new Error("Cannot create animated SVG without keyframes.");
  }
  const totalFrames = sequence.length;
  const totalDurationSeconds = Math.max(keyframes.length * 4, 1);
  const keyTimes = sequence.map((_, index) => normalizeLogoNumber(index / Math.max(totalFrames - 1, 1)).toString());
  const keyTimesText = keyTimes.join(";");
  const keySplinesText = Array.from({ length: Math.max(totalFrames - 1, 1) }, (_, index) => {
    const currentFrame = JSON.stringify(sequence[index]);
    const nextFrame = JSON.stringify(sequence[Math.min(index + 1, totalFrames - 1)]);
    return currentFrame === nextFrame ? "0 0 1 1" : "0.25 0.1 0.75 0.9";
  }).join(";");
  const allGroupIds = new Set<string>();
  for (const keyframe of keyframes) {
    for (const group of keyframe.groups) allGroupIds.add(group.id);
  }
  const lines: string[] = [
    '<?xml version="1.0" encoding="UTF-8" standalone="no"?>',
    '<svg viewBox="0 0 410 140" style="background: #001117;" version="1.1" xmlns="http://www.w3.org/2000/svg">',
    "  <title>compose</title>",
    '  <rect id="background" width="100%" height="100%" fill="#001117" />',
  ];
  for (const groupId of allGroupIds) {
    const groupFrames = sequence.map((keyframe) => keyframe.groups.find((group) => group.id === groupId) ?? null);
    const firstGroup = groupFrames.find((group): group is LogoGroupData => group !== null);
    if (!firstGroup) continue;
    const matrixValues = groupFrames
      .map((group) => logoTransformToMatrix(group?.transform.translate ?? firstGroup.transform.translate, group?.transform.rotate ?? firstGroup.transform.rotate, group?.transform.scale ?? firstGroup.transform.scale))
      .join(";");
    const fillValues = groupFrames.map((group) => group?.path.fill ?? firstGroup.path.fill).join(";");
    const strokeValues = groupFrames.map((group) => group?.path.stroke ?? firstGroup.path.stroke).join(";");
    const strokeWidthValues = groupFrames.map((group) => group?.path.strokeWidth ?? firstGroup.path.strokeWidth).join(";");
    lines.push(`  <g id="${escapeLogoXmlAttribute(groupId)}">`);
    lines.push(`    <path d="${escapeLogoXmlAttribute(firstGroup.path.d)}" fill="${escapeLogoXmlAttribute(firstGroup.path.fill)}" stroke="${escapeLogoXmlAttribute(firstGroup.path.stroke)}" stroke-width="${escapeLogoXmlAttribute(firstGroup.path.strokeWidth)}">`);
    lines.push(`      <animateTransform attributeName="transform" type="matrix" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${matrixValues}" calcMode="spline" keySplines="${keySplinesText}" />`);
    lines.push(`      <animate attributeName="fill" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${fillValues}" calcMode="spline" keySplines="${keySplinesText}" />`);
    lines.push(`      <animate attributeName="stroke" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${strokeValues}" calcMode="spline" keySplines="${keySplinesText}" />`);
    lines.push(`      <animate attributeName="stroke-width" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${strokeWidthValues}" calcMode="spline" keySplines="${keySplinesText}" />`);
    lines.push("    </path>");
    lines.push("  </g>");
  }
  lines.push("</svg>");
  writeFileSync(outputPath, `${lines.join("\n")}\n`);
  console.log(`Logo animated SVG created: ${outputPath}`);
}
//#endregion 📻️AnimatedSvgOutput

//#region 🚀️LogoCommands
/** 🎞️Resolves the six handpicked animation keyframes in their authored order. */
export function logoKeyframePaths(logoDir: string): string[] {
  return [
    "🎞️animation/1️⃣one/🖋️vector.svg",
    "🎞️animation/2️⃣two/🖋️vector.svg",
    "🎞️animation/3️⃣three/🖋️vector.svg",
    "🎞️animation/4️⃣four/🖋️vector.svg",
    "🎞️animation/5️⃣five/🖋️vector.svg",
    "🎞️animation/6️⃣six/🖋️vector.svg",
  ].map((path) => join(logoDir, path));
}

export function generateLogoAnimation(): void {
  const logoDir = join(assetsRoot(), "🪧️logos");
  const keyframes: LogoKeyframeData[] = [];
  for (const filePath of logoKeyframePaths(logoDir)) {
    console.log(`Parsing ${filePath}...`);
    keyframes.push(parseLogoSvgFile(filePath));
  }
  console.log(`Found ${keyframes.length} keyframes`);
  console.log(`Will generate ${generateLogoKeyframeSequence(keyframes).length} animation frames`);
  createLogoAnimatedSVG(keyframes, join(logoDir, "🎞️animation/⚡️animated.svg"));
}

export async function exportLogoAnimation(repoRoot: string): Promise<void> {
  process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
  const logoDir = join(assetsRoot(), "🪧️logos");
  const inputPath = join(logoDir, "🎞️animation/⚡️animated.svg");
  const outputPath = join(logoDir, "🎞️animation/🎬️animation.mp4");
  console.log(`Exporting ${inputPath} to ${outputPath}...`);
  await exportAnimatedSvgToMp4(inputPath, outputPath, { progress: ({ completed, total }) => {
    if (completed % 60 === 0 || completed === total) console.log(`[logo] encoded ${completed}/${total} frames`);
  } });
  console.log(`Successfully exported logo to MP4: ${outputPath}`);
}
//#endregion 🚀️LogoCommands
