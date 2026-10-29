/** 🖨️ Preview CLI for content authors (Node-only): renders one asset pose or clip time in any of the seven views to SVG and PNG, or all poses × views into one contact sheet.
 *
 * Usage: `repo tools exec bun -- <this file> --file <category.json | asset.json> --asset <id> [--pose id | --clip id --time s] [--view v] [--scale d] [--language en|de] [--dpi 300] [--no-png] [--sheet] --out <dir>`
 *
 * `rsvg-convert` is not a registered repo tool, so the system binary is used behind the {@link Rasterizer} interface.
 *
 * @see ../🟦️.ts — `depict`
 * @see ../../📤️export/🖼️svg/🟦️.ts — `writeSvg`, `svgMarkup`
 */
import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { Asset, Language } from "../../../🧬️schema/🟦️.ts";
import { VIEWS, type DrawingScene, type EvaluationQuery, type View } from "../../🔗️contracts/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { solveThermal } from "../../🌡️thermal/🟦️.ts";
import { svgMarkup, writeSvg } from "../../📤️export/🖼️svg/🟦️.ts";
import { defaultDepictionOptions, depict } from "../🟦️.ts";

/** 🖼️ Turns an SVG file into a PNG at a resolution. */
export interface Rasterizer {
  rasterize(svgPath: string, pngPath: string, dpi: number): void;
}

/** 🖼️ The `rsvg-convert` implementation of {@link Rasterizer}. */
export const rsvgRasterizer: Rasterizer = {
  rasterize(svgPath, pngPath, dpi) {
    execFileSync("rsvg-convert", ["-d", String(dpi), "-p", String(dpi), "-f", "png", "-o", pngPath, svgPath]);
  },
};

type Args = { readonly flags: ReadonlyMap<string, string> };

function parse(argv: readonly string[]): Args {
  const flags = new Map<string, string>();
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]!;
    if (!a.startsWith("--")) continue;
    const next = argv[i + 1];
    if (next === undefined || next.startsWith("--")) flags.set(a.slice(2), "true");
    else {
      flags.set(a.slice(2), next);
      i++;
    }
  }
  return { flags };
}

function loadAssets(file: string): Asset[] {
  const doc = JSON.parse(readFileSync(file, "utf8")) as { assets?: Asset[]; rig?: unknown };
  return doc.assets ?? (doc.rig ? [doc as unknown as Asset] : []);
}

function sheet(rows: readonly { readonly label: string; readonly scenes: readonly DrawingScene[] }[]): string {
  const cellW = Math.max(...rows.flatMap((r) => r.scenes.map((s) => s.widthMm)));
  const cellH = Math.max(...rows.flatMap((r) => r.scenes.map((s) => s.heightMm)));
  const labelW = 30;
  const head = 8;
  const width = labelW + cellW * VIEWS.length;
  const height = head + cellH * rows.length;
  const cells = rows.map((row, r) => row.scenes.map((scene, c) => `<svg x="${labelW + c * cellW}" y="${head + r * cellH}" width="${scene.widthMm}" height="${scene.heightMm}" viewBox="0 0 ${scene.widthMm} ${scene.heightMm}">${svgMarkup(scene)}</svg>`).join("")).join("");
  const text = (x: number, y: number, t: string): string => `<text x="${x}" y="${y}" font-size="2.5" font-family="Helvetica, Arial, sans-serif" fill="#001117">${t}</text>`;
  const cols = VIEWS.map((v, c) => text(labelW + c * cellW + 2, 5, v)).join("");
  const labels = rows.map((row, r) => text(2, head + r * cellH + 6, row.label)).join("");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" version="1.1" width="${width}mm" height="${height}mm" viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" fill="#ffffff"/>${cols}${labels}${cells}</svg>\n`;
}

/** ▶️ Runs the preview CLI with the given arguments; returns the written file paths. */
export function runPreview(argv: readonly string[], rasterizer: Rasterizer = rsvgRasterizer): string[] {
  const { flags } = parse(argv);
  const file = flags.get("file");
  const out = flags.get("out");
  if (!file || !out) throw new Error("usage: --file <json> --asset <id> [--pose id | --clip id --time s] [--view v] [--scale d] [--language en|de] [--dpi 300] [--no-png] [--sheet] --out <dir>");
  const assets = loadAssets(file);
  const id = flags.get("asset");
  const asset = id ? assets.find((a) => a.id === id) : assets.length === 1 ? assets[0] : undefined;
  if (!asset) throw new Error(`asset not found; available: ${assets.map((a) => a.id).join(", ")}`);
  const denominator = Number(flags.get("scale") ?? 100);
  const language = (flags.get("language") ?? "en") as Language;
  const dpi = Number(flags.get("dpi") ?? 300);
  const png = !flags.has("no-png");
  const views = flags.has("view") ? [flags.get("view") as View] : [...VIEWS];
  mkdirSync(out, { recursive: true });
  const written: string[] = [];
  const emit = (name: string, svg: string): void => {
    const svgPath = join(out, `${name}.svg`);
    writeFileSync(svgPath, svg);
    written.push(svgPath);
    if (png) {
      const pngPath = join(out, `${name}.png`);
      rasterizer.rasterize(svgPath, pngPath, dpi);
      written.push(pngPath);
    }
  };
  const scenesFor = (query: EvaluationQuery, which: readonly View[], legend: boolean): DrawingScene[] => {
    const state = evaluate(asset, query);
    const analysis = { thermal: solveThermal(state), structure: solveStructure(state) };
    return which.map((view) => {
      const o = defaultDepictionOptions(view, language, denominator);
      return depict(state, { ...o, annotations: { ...o.annotations, legend } }, analysis);
    });
  };
  if (flags.has("sheet")) {
    const rows = asset.poses.map((pose) => ({ label: pose.id, scenes: scenesFor({ kind: "pose", pose: pose.id }, VIEWS, false) }));
    emit(`${asset.id}--sheet-1-${denominator}`, sheet(rows));
    return written;
  }
  const query: EvaluationQuery = flags.has("clip") ? { kind: "clip", clip: flags.get("clip")!, timeS: Number(flags.get("time") ?? 0) } : { kind: "pose", pose: flags.get("pose") ?? asset.presentation.defaultPose };
  const tag = query.kind === "clip" ? `${query.clip}@${query.timeS}` : query.kind === "pose" ? query.pose : "transition";
  scenesFor(query, views, true).forEach((scene, i) => emit(`${asset.id}--${tag}--${views[i]}-1-${denominator}`, writeSvg(scene)));
  return written;
}

if (import.meta.main) {
  for (const path of runPreview(process.argv.slice(2))) console.log(path);
}
