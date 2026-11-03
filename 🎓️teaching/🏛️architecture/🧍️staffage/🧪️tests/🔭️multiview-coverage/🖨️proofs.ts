/** 🖨️ Generates the proof sheets: `bun 🖨️proofs.ts --out <dir> [--categories humans,animals] [--assets a,b] [--scales 500,200,100,50]`.
 *
 * - `contact-<category>.svg`: one row per asset, six cells (side elevation, front elevation, plan) of the neutral pose and of the extreme instant of the default clip, each fitted to its cell; a view that the resolver reports unavailable prints its code in red, it is never replaced.
 * - `print-<denominator>-<category>.svg`: the three views of the neutral pose at true print scale (millimetres on A3 width pages) with a one metre scale bar and the print-profile minimum stroke noted; unavailable scales print their reason.
 *
 * All sheets come from `resolveViews`, so every cell of one row is projected from one evaluated state.
 *
 * @see ./🧰️.ts — the instants
 * @see ../../🔨️modules/🔭️multiview/🟦️.ts — the resolver
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Asset, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene, EvaluationQuery } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { resolveViews, type MultiView } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { svgMarkup } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { CATEGORY_ORDER, VIEWPOINT_LIST, clipInstants, drawOptions, loadCatalog, probeChannels, scaleSelector } from "./🧰️.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const out = resolve(flag("out") ?? ".");
const wantedCategories = flag("categories")?.split(",");
const wantedAssets = flag("assets")?.split(",");
const scales = (flag("scales") ?? "500,200,100,50").split(",").map(Number);
const assets = loadCatalog().filter((asset) => (wantedAssets === undefined || wantedAssets.includes(asset.id)) && (wantedCategories === undefined || wantedCategories.includes(asset.category)));
const esc = (text: string): string => text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");
const num = (n: number): string => (Math.round(n * 1000) / 1000).toString();
const VIEW_LABEL: Record<Viewpoint, string> = { elevation: "side elevation", front: "front elevation", plan: "plan (top view)" };

const text = (x: number, y: number, size: number, content: string, fill = "#222"): string => `<text x="${num(x)}" y="${num(y)}" font-family="Helvetica, Arial, sans-serif" font-size="${num(size)}" fill="${fill}">${esc(content)}</text>`;

function place(scene: DrawingScene, x: number, y: number, w: number, h: number): string {
  const fit = Math.min(w / scene.widthMm, h / scene.heightMm);
  const sw = scene.widthMm * fit;
  const sh = scene.heightMm * fit;
  return `<svg x="${num(x + (w - sw) / 2)}" y="${num(y + (h - sh) / 2)}" width="${num(sw)}" height="${num(sh)}" viewBox="0 0 ${num(scene.widthMm)} ${num(scene.heightMm)}">${svgMarkup(scene)}</svg>`;
}

function neutralQuery(asset: Asset): EvaluationQuery {
  return { kind: "pose", pose: asset.presentation.defaultPose };
}

function extremeQuery(asset: Asset, denominator: number): EvaluationQuery | undefined {
  const clip = asset.clips.find((candidate) => candidate.id === asset.presentation.defaultClip) ?? asset.clips[0];
  if (clip === undefined) return undefined;
  const probe = (t: number): readonly number[] | undefined => {
    try {
      return probeChannels(evaluate(asset, { kind: "clip", clip: clip.id, timeS: t }, scaleSelector(denominator)));
    } catch {
      return undefined;
    }
  };
  const extreme = clipInstants(asset, clip.id, probe).filter((instant) => instant.tag === "extreme");
  const widest = extreme.map((instant) => ({ instant, width: (probe(instant.t)?.[1] ?? 0) - (probe(instant.t)?.[0] ?? 0) })).sort((a, b) => b.width - a.width)[0]?.instant;
  return { kind: "clip", clip: clip.id, timeS: widest?.t ?? 0 };
}

function cellMulti(asset: Asset, query: EvaluationQuery, denominator: number): MultiView | string {
  try {
    return resolveViews(asset, query, { kind: "scale", denominator }, "light-line", drawOptions("light-line", denominator, true));
  } catch (error) {
    return `error: ${(error as Error).message.split("\n")[0]}`;
  }
}

function contactSheet(category: string, group: readonly Asset[]): string {
  const cellW = 52;
  const cellH = 52;
  const labelW = 44;
  const gap = 2;
  const headH = 14;
  const width = labelW + 6 * (cellW + gap) + gap;
  const height = headH + group.length * (cellH + gap) + gap;
  const parts: string[] = [`<rect width="${width}" height="${height}" fill="#fff"/>`];
  const heads = ["neutral", "extreme"].flatMap((moment) => VIEWPOINT_LIST.map((view) => `${moment} · ${VIEW_LABEL[view]}`));
  heads.forEach((label, i) => parts.push(text(labelW + i * (cellW + gap) + 1, 9, 2.6, label)));
  group.forEach((asset, row) => {
    const y = headH + row * (cellH + gap);
    parts.push(text(1, y + 5, 3, asset.id), text(1, y + 9, 2.2, `${asset.dimensions.heightM} × ${asset.dimensions.widthM} × ${asset.dimensions.depthM} m`, "#666"));
    const scale = [asset.presentation.defaultScale, 100, 50, 200, 500].find((d) => asset.representations[`${d}` as keyof Asset["representations"]]?.status === "authored") ?? asset.presentation.defaultScale;
    const queries = [neutralQuery(asset), extremeQuery(asset, scale)];
    queries.forEach((query, moment) => {
      const multi = query === undefined ? "no clip" : cellMulti(asset, query, scale);
      VIEWPOINT_LIST.forEach((view, col) => {
        const x = labelW + (moment * 3 + col) * (cellW + gap);
        parts.push(`<rect x="${x}" y="${y}" width="${cellW}" height="${cellH}" fill="none" stroke="#ccc" stroke-width="0.15"/>`);
        const found = typeof multi === "string" ? undefined : multi.views[view];
        if (found?.kind === "drawing") parts.push(place(found.scene, x + 1, y + 1, cellW - 2, cellH - 2));
        else parts.push(text(x + 2, y + 6, 2.4, found?.kind === "unavailable" ? `${found.code}` : `${multi}`, "#c00000"));
      });
    });
  });
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" width="${width}mm" height="${height}mm" viewBox="0 0 ${width} ${height}"><title>${esc(category)} contact sheet</title>${parts.join("")}</svg>\n`;
}

function printSheet(category: string, denominator: number, group: readonly Asset[]): string {
  const margin = 8;
  const k = 1000 / denominator;
  const blocks = group.map((asset) => {
    const decision = asset.representations[`${denominator}` as keyof Asset["representations"]];
    const multi = decision?.status === "authored" ? cellMulti(asset, neutralQuery(asset), denominator) : undefined;
    const drawings = multi === undefined || typeof multi === "string" ? [] : VIEWPOINT_LIST.flatMap((view) => (multi.views[view].kind === "drawing" ? [{ view, scene: (multi.views[view] as { scene: DrawingScene }).scene }] : []));
    const width = drawings.length === 0 ? 70 : drawings.reduce((sum, d) => sum + d.scene.widthMm + 4, 0);
    const height = drawings.length === 0 ? 12 : Math.max(...drawings.map((d) => d.scene.heightMm)) + 7;
    return { asset, decision, drawings, width, height };
  });
  const pageW = Math.max(420, ...blocks.map((block) => block.width + 2 * margin));
  const parts: string[] = [];
  let x = margin;
  let y = margin + 8;
  let rowH = 0;
  for (const { asset, decision, drawings, width, height } of blocks) {
    if (x + width > pageW - margin && x > margin) {
      x = margin;
      y += rowH + 4;
      rowH = 0;
    }
    parts.push(text(x, y + 3, 2.4, asset.id));
    if (drawings.length === 0) {
      const reason = decision?.status === "unavailable" ? `1:${denominator} unavailable (${decision.cause}): ${decision.reason.en}` : "no representation";
      parts.push(text(x, y + 8, 2, reason.slice(0, 120), "#c00000"));
    } else {
      let dx = x;
      for (const { view, scene } of drawings) {
        parts.push(`<svg x="${num(dx)}" y="${num(y + 5)}" width="${num(scene.widthMm)}" height="${num(scene.heightMm)}" viewBox="0 0 ${num(scene.widthMm)} ${num(scene.heightMm)}">${svgMarkup(scene)}</svg>`, text(dx, y + 5 + scene.heightMm + 2, 1.6, view, "#666"));
        dx += scene.widthMm + 4;
      }
    }
    rowH = Math.max(rowH, height);
    x += width + 6;
  }
  const height = y + rowH + 20;
  parts.unshift(`<rect width="${pageW}" height="${height}" fill="#fff"/>`, text(margin, margin, 4, `${category} · 1:${denominator} · true print scale (1 m = ${num(k)} mm)`));
  parts.push(`<rect x="${margin}" y="${height - 8}" width="${num(k)}" height="1.5" fill="#000"/>`, text(margin + k + 2, height - 6.5, 2.4, "1 m"));
  return `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" width="${pageW}mm" height="${num(height)}mm" viewBox="0 0 ${pageW} ${num(height)}"><title>${esc(category)} 1:${denominator}</title>${parts.join("")}</svg>
`;
}

mkdirSync(out, { recursive: true });
for (const category of CATEGORY_ORDER) {
  const group = assets.filter((asset) => asset.category === category);
  if (group.length === 0) continue;
  console.error(`[proofs] ${category}: ${group.length} assets`);
  writeFileSync(join(out, `contact-${category}.svg`), contactSheet(category, group));
  for (const denominator of scales) writeFileSync(join(out, `print-${denominator}-${category}.svg`), printSheet(category, denominator, group));
}
console.error("[done]");
