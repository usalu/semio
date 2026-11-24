/** 🔎️ The authors' gate of the multiview coverage: checks single assets in seconds with the check functions of the matrix itself.
 *
 * Usage, from `🧪️tests/🔭️multiview-coverage` with the pinned bun:
 *
 *     bun 🔎️scan.ts --language en|de <asset id> [<asset id> …]      one or several assets, findings on stdout
 *     bun 🔎️scan.ts --language en|de --all --out scan.jsonl         the whole catalog, one JSON line per asset, resumable
 *     bun 🔎️scan.ts --language en|de --categories humans --json     a category, the rows as one JSON array on stdout
 *
 * - `--language en|de` (required): the language of the headings, progress and summary; no language is assumed. Codes and the messages of the engine are language-neutral data.
 * - `--out <file.jsonl>`: appends one row per asset (every finding) when it ends; a rerun skips the assets the file already holds (`--fresh` starts the file anew). `--report <file.json>`: writes at the end `{ version, rows }` with the rows of the selection in compact form: the summary and the findings grouped per code, views, mode, bone and anchor with their poses, scales, count and worst case.
 * - `--json`: prints the rows instead of the readable listing. `--quiet`: prints only the summary line per asset.
 * - Exit code 0 when no selected asset has a hard finding, 1 when one has, 2 on wrong usage, 130 when cancelled (SIGINT or SIGTERM stop after the running asset; the rows written so far stay).
 *
 * What it checks, per authored scale of the asset ({@link scanAsset}):
 * - every pose: the geometry findings of the matrix ({@link ./🧰️.ts geometryFindings}): empty views, the synchrony of the three views (`sync-*`), the per-bone synchrony of a shape and its twin (`sync-bone-*`, hard) and of groups or different parts (`group-drift-*`, reported only), the anchor landmarks against the shapes of the anchor's bone and of its descendant bones (`anchor-landmark`, with the place the anchor lies instead), and the dimensions of the default pose;
 * - the default pose: the band copies and in-band derived views (`derived-view-band-copy`, `derived-view-in-band`, honouring the two allowlists of the fixture, `bandCopyAllowlist` and `derivedViewAllowlist`), the paper box of fills and outline runs against the dimensions (`paper-width`, `paper-height`), a view the resolver refuses (`unavailable-*`) and the colour-independent cue of the thermal, forces and moments scenes (`analysis-cue-legend`).
 * Every finding names its code, scale, pose, views and, where it has one, bone, anchor and delta in millimetres.
 * It leaves clip instants, transitions, range ends, symbols and the German renders to the matrix (`📊️matrix.ts --assets <id>` confirms a repaired asset), so a clean scan is necessary for a clean matrix, not sufficient.
 *
 * @see ./🧰️.ts — the engine whose functions it calls
 * @see ./📊️matrix.ts — the full matrix
 */
import { appendFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Asset, Language, PlanarViewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState, View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { resolveViews } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { UNHATCHED_PERCENT, VIEWPOINT_LIST, authoredOf, cueFinding, derivedFindings, drawOptions, geometryFindings, loadCatalog, paperFindings, scaleSelector, toleranceM } from "./🧰️.ts";
import { distanceToShapes } from "./⚓️landmarks.ts";

/** 🏷️ The version of a report file. */
export const REPORT_VERSION = "semio.staffage.coverage.scan/v1";

/** 👪️ The families of codes a row is counted by. */
export const FAMILIES = ["sync", "anchor", "band-copy", "in-band", "cue", "paper", "dimension", "view", "error", "group-drift"] as const;

/** 🏠️ A family of codes. */
export type Family = (typeof FAMILIES)[number];

/** 🧾️ One finding of the scan: its code and family, the tier (`hard` fails the matrix, `reported` is the group-drift tier), where it was found and, where measured, what it concerns. `place` says where a failing anchor lies instead: on a shape of another bone or off every drawn shape. */
export type ScanFinding = { readonly code: string; readonly family: Family; readonly tier: "hard" | "reported"; readonly scale: string; readonly pose: string; readonly rest: boolean; readonly views: readonly PlanarViewpoint[]; readonly mode?: View; readonly bone?: string; readonly anchor?: string; readonly anchorKind?: string; readonly place?: "other-bone" | "off-shapes"; readonly deltaMm?: number; readonly message: string };

/** 🧮️ The counts of a row: hard findings per family, the synchrony findings at rest and in other poses, the anchors by the place they lie and by kind, and the reported group drift. */
export type ScanSummary = { readonly hard: number; readonly reported: number; readonly families: Readonly<Record<Family, number>>; readonly syncRest: number; readonly syncPosed: number; readonly anchorPlaces: Readonly<Record<string, number>>; readonly anchorKinds: Readonly<Record<string, number>>; readonly codes: Readonly<Record<string, number>> };

/** 📋️ The row of one asset. */
export type ScanRow = { readonly id: string; readonly category: string; readonly authored: readonly string[]; readonly unavailable: readonly string[]; readonly poses: number; readonly seconds: number; readonly clean: boolean; readonly summary: ScanSummary; readonly findings: readonly ScanFinding[] };

/** 🗂️ The family of a code. */
export function familyOf(code: string): Family {
  if (code.startsWith("group-drift-")) return "group-drift";
  if (code.startsWith("sync-")) return "sync";
  if (code === "anchor-landmark") return "anchor";
  if (code === "derived-view-band-copy") return "band-copy";
  if (code === "derived-view-in-band") return "in-band";
  if (code.startsWith("analysis-cue")) return "cue";
  if (code.startsWith("paper-")) return "paper";
  if (code.startsWith("dimension-")) return "dimension";
  if (code.endsWith("-error")) return "error";
  return "view";
}

const messageOf = (error: unknown): string => (error instanceof Error ? error.message : String(error)).split("\n")[0]!.slice(0, 220);

function placeOf(state: EvaluatedState, view: PlanarViewpoint, anchor: string, tolerance: number): "other-bone" | "off-shapes" {
  const { at, z } = state.anchors[anchor]!;
  const point = view === "elevation" ? { x: at.x, y: at.y } : view === "front" ? { x: z, y: at.y } : { x: at.x, y: z };
  return distanceToShapes(point, state.shapes[view].filter((shape) => shape.shape.role !== "ground")) <= tolerance ? "other-bone" : "off-shapes";
}

/** 📊️ The summary of findings. */
export function summarise(findings: readonly ScanFinding[]): ScanSummary {
  const families = Object.fromEntries(FAMILIES.map((family) => [family, 0])) as Record<Family, number>;
  const anchorPlaces: Record<string, number> = {};
  const anchorKinds: Record<string, number> = {};
  const codes: Record<string, number> = {};
  let syncRest = 0;
  let syncPosed = 0;
  for (const finding of findings) {
    families[finding.family]++;
    codes[finding.code] = (codes[finding.code] ?? 0) + 1;
    if (finding.family === "sync") finding.rest ? syncRest++ : syncPosed++;
    if (finding.family === "anchor") {
      anchorPlaces[finding.place ?? "off-shapes"] = (anchorPlaces[finding.place ?? "off-shapes"] ?? 0) + 1;
      anchorKinds[finding.anchorKind ?? "anchor"] = (anchorKinds[finding.anchorKind ?? "anchor"] ?? 0) + 1;
    }
  }
  const reported = findings.filter((finding) => finding.tier === "reported").length;
  return { hard: findings.length - reported, reported, families, syncRest, syncPosed, anchorPlaces, anchorKinds, codes };
}

/** 🩺️ Scans one asset: every pose of every authored scale with the geometry checks of the matrix, and the default pose with the derived-view, paper, availability and cue checks. `onScale` reports the progress before each authored scale. */
export function scanAsset(asset: Asset, onScale?: (index: number, count: number, scale: string) => void): ScanRow {
  const started = Date.now();
  const authored = Object.keys(asset.representations).filter((key) => authoredOf(asset, key) !== undefined);
  const tolerance = toleranceM(asset);
  const rest = asset.presentation.defaultPose;
  const findings: ScanFinding[] = [];
  authored.forEach((key, index) => {
    onScale?.(index, authored.length, key);
    const denominator = Number(key);
    const at = { scale: key, pose: rest, rest: true } as const;
    try {
      for (const finding of derivedFindings(asset, key, denominator)) findings.push({ ...at, code: finding.check, family: familyOf(finding.check), tier: "hard", views: [finding.view], message: finding.message });
    } catch (error) {
      findings.push({ ...at, code: "derived-view-error", family: "error", tier: "hard", views: VIEWPOINT_LIST, message: messageOf(error) });
    }
    for (const pose of asset.poses) {
      const where = { scale: key, pose: pose.id, rest: pose.id === rest } as const;
      let state: EvaluatedState;
      try {
        state = evaluate(asset, { kind: "pose", pose: pose.id }, scaleSelector(denominator));
      } catch (error) {
        findings.push({ ...where, code: "evaluate-error", family: "error", tier: "hard", views: VIEWPOINT_LIST, message: messageOf(error) });
        continue;
      }
      for (const finding of geometryFindings(asset, state, where.rest))
        findings.push({ ...where, code: finding.check, family: familyOf(finding.check), tier: finding.tier, views: finding.views, ...(finding.bone !== undefined ? { bone: finding.bone } : {}), ...(finding.anchor !== undefined ? { anchor: finding.anchor, anchorKind: finding.anchorKind!, place: placeOf(state, finding.views[0]!, finding.anchor, tolerance) } : {}), ...(finding.delta !== undefined ? { deltaMm: Math.round(finding.delta.deltaMm * 10) / 10 } : {}), message: finding.message });
    }
    for (const mode of ["light-line", ...(Object.keys(UNHATCHED_PERCENT) as View[])] as const) {
      try {
        const multi = resolveViews(asset, { kind: "pose", pose: rest }, { kind: "scale", denominator }, mode, drawOptions(mode, denominator));
        for (const view of VIEWPOINT_LIST) {
          const drawing = multi.views[view];
          if (drawing.kind === "unavailable") {
            if (mode === "light-line") findings.push({ ...at, code: `unavailable-${drawing.code}`, family: "view", tier: "hard", views: [view], mode, message: drawing.reason.en });
            continue;
          }
          if (mode === "light-line") for (const finding of paperFindings(asset, view, drawing.scene, drawing.state.shapes[view])) findings.push({ ...at, code: finding.check, family: "paper", tier: "hard", views: [view], mode, deltaMm: Math.round(finding.delta.deltaMm * 10) / 10, message: finding.message });
          const cue = cueFinding(mode, drawing.scene);
          if (cue !== undefined) findings.push({ ...at, code: cue.check, family: "cue", tier: "hard", views: [view], mode, message: cue.message });
        }
      } catch (error) {
        findings.push({ ...at, code: "resolve-error", family: "error", tier: "hard", views: VIEWPOINT_LIST, mode, message: messageOf(error) });
      }
    }
  });
  const summary = summarise(findings);
  return { id: asset.id, category: asset.category, authored, unavailable: Object.keys(asset.representations).filter((key) => !authored.includes(key)), poses: asset.poses.length, seconds: (Date.now() - started) / 1000, clean: summary.hard === 0, summary, findings };
}

const TEXT = {
  usage: { en: "usage: bun 🔎️scan.ts --language en|de (<asset id> … | --all | --categories a,b) [--out file.jsonl] [--fresh] [--report file.json] [--json] [--quiet]", de: "Aufruf: bun 🔎️scan.ts --language en|de (<Asset-Id> … | --all | --categories a,b) [--out Datei.jsonl] [--fresh] [--report Datei.json] [--json] [--quiet]" },
  unknown: { en: "the catalog has no asset", de: "der Katalog hat kein Asset" },
  clean: { en: "clean", de: "sauber" },
  hard: { en: "hard findings", de: "harte Befunde" },
  reported: { en: "reported (group drift)", de: "gemeldet (Gruppendrift)" },
  scales: { en: "scales", de: "Massstäbe" },
  poses: { en: "poses", de: "Posen" },
  pose: { en: "pose", de: "Pose" },
  bone: { en: "bone", de: "Knochen" },
  anchor: { en: "anchor", de: "Anker" },
  mode: { en: "mode", de: "Modus" },
  place: { "other-bone": { en: "on a shape of another bone", de: "auf einer Form eines anderen Knochens" }, "off-shapes": { en: "off every drawn shape", de: "neben jeder gezeichneten Form" } },
  worst: { en: "worst", de: "grösste Abweichung" },
  skipped: { en: "already in the output file, skipped", de: "bereits in der Ausgabedatei, übersprungen" },
  remaining: { en: "remaining about", de: "verbleibend etwa" },
  cancelled: { en: "cancelled: the rows written so far stay", de: "abgebrochen: die bisher geschriebenen Zeilen bleiben" },
  total: { en: (assets: number, clean: number, hard: number) => `${assets} assets: ${clean} clean, ${assets - clean} with a hard finding, ${hard} hard findings`, de: (assets: number, clean: number, hard: number) => `${assets} Assets: ${clean} sauber, ${assets - clean} mit hartem Befund, ${hard} harte Befunde` },
} as const;

/** 🧺️ The findings of one code that share views, mode, bone and anchor: the poses and scales they occur at, their count and the worst of them with its pose, scale, delta and message. */
export type ScanGroup = { readonly code: string; readonly family: Family; readonly tier: "hard" | "reported"; readonly views: readonly PlanarViewpoint[]; readonly mode?: View; readonly bone?: string; readonly anchor?: string; readonly anchorKind?: string; readonly place?: "other-bone" | "off-shapes"; readonly poses: readonly string[]; readonly scales: readonly string[]; readonly count: number; readonly worst: { readonly pose: string; readonly scale: string; readonly deltaMm?: number; readonly message: string } };

/** 🧶️ The findings of a row grouped per code, views, mode, bone and anchor ({@link ScanGroup}), hard codes first, then by code. */
export function groupsOf(findings: readonly ScanFinding[]): ScanGroup[] {
  const groups = new Map<string, ScanFinding[]>();
  for (const finding of findings) {
    const key = JSON.stringify([finding.code, finding.views, finding.family === "cue" ? finding.mode : undefined, finding.bone, finding.anchor, finding.place]);
    (groups.get(key) ?? groups.set(key, []).get(key)!).push(finding);
  }
  return [...groups.values()]
    .map((group): ScanGroup => {
      const first = group[0]!;
      const worst = group.reduce((best, finding) => ((finding.deltaMm ?? -1) > (best.deltaMm ?? -1) ? finding : best), first);
      return { code: first.code, family: first.family, tier: first.tier, views: first.views, ...(first.family === "cue" && first.mode !== undefined ? { mode: first.mode } : {}), ...(first.bone !== undefined ? { bone: first.bone } : {}), ...(first.anchor !== undefined ? { anchor: first.anchor, anchorKind: first.anchorKind!, place: first.place! } : {}), poses: [...new Set(group.map((finding) => finding.pose))], scales: [...new Set(group.map((finding) => finding.scale))], count: group.length, worst: { pose: worst.pose, scale: worst.scale, ...(worst.deltaMm !== undefined ? { deltaMm: worst.deltaMm } : {}), message: worst.message } };
    })
    .sort((a, b) => Number(a.tier === "reported") - Number(b.tier === "reported") || a.code.localeCompare(b.code));
}

/** 📑️ The compact form of a row for a report: the row with its findings as groups ({@link groupsOf}). */
export const compact = (row: ScanRow): Omit<ScanRow, "findings"> & { readonly groups: readonly ScanGroup[] } => {
  const { findings, ...rest } = row;
  return { ...rest, groups: groupsOf(findings) };
};

/** 🖨️ The readable listing of a row: per code the groups of findings ({@link groupsOf}) with their views, mode, bone and anchor, the poses and scales they occur at and the worst delta with its pose, scale and message. */
export function listing(row: ScanRow, language: Language, quiet = false): string {
  const head = `${row.id} (${row.category}) · ${TEXT.scales[language]} ${row.authored.join(" ")} · ${row.poses} ${TEXT.poses[language]} · ${row.seconds.toFixed(1)} s · ${row.clean ? TEXT.clean[language] : `${row.summary.hard} ${TEXT.hard[language]}`}${row.summary.reported > 0 ? ` · ${row.summary.reported} ${TEXT.reported[language]}` : ""}`;
  if (quiet) return head;
  const lines = [head];
  let code = "";
  for (const group of groupsOf(row.findings)) {
    if (group.code !== code) lines.push(`  ${group.code} ×${row.summary.codes[group.code]}${group.tier === "reported" ? ` (${TEXT.reported[language]})` : ""}`);
    code = group.code;
    const what = [group.views.join("/"), ...(group.mode !== undefined ? [`${TEXT.mode[language]} ${group.mode}`] : []), ...(group.bone !== undefined ? [`${TEXT.bone[language]} ${group.bone}`] : []), ...(group.anchor !== undefined ? [`${TEXT.anchor[language]} ${group.anchor} (${group.anchorKind}, ${TEXT.place[group.place ?? "off-shapes"][language]})`] : [])].join(" · ");
    lines.push(`    ${what} · ${TEXT.poses[language]} ${group.poses.join(", ")} · 1:${group.scales.join(", 1:")}${group.worst.deltaMm !== undefined ? ` · ${TEXT.worst[language]} ${group.worst.deltaMm} mm (${TEXT.pose[language]} ${group.worst.pose}, 1:${group.worst.scale})` : ""}`);
    lines.push(`      ${group.worst.message}`);
  }
  return lines.join("\n");
}

async function main(): Promise<number> {
  const args = process.argv.slice(2);
  const valued = new Set(["language", "out", "report", "categories"]);
  const options = new Map<string, string>();
  const switches = new Set<string>();
  const ids: string[] = [];
  for (let i = 0; i < args.length; i++) {
    const arg = args[i]!;
    if (!arg.startsWith("--")) ids.push(...arg.split(",").filter(Boolean));
    else if (valued.has(arg.slice(2))) options.set(arg.slice(2), args[++i] ?? "");
    else switches.add(arg.slice(2));
  }
  const language = options.get("language") as Language | undefined;
  const categories = options.get("categories")?.split(",");
  if ((language !== "en" && language !== "de") || (ids.length === 0 && !switches.has("all") && categories === undefined)) {
    console.error(`${TEXT.usage.en}\n${TEXT.usage.de}`);
    return 2;
  }
  const catalog = loadCatalog();
  const unknown = ids.filter((id) => !catalog.some((asset) => asset.id === id));
  if (unknown.length > 0) {
    console.error(`${TEXT.unknown[language]}: ${unknown.join(", ")}`);
    return 2;
  }
  const selection = catalog.filter((asset) => (ids.length === 0 || ids.includes(asset.id)) && (categories === undefined || categories.includes(asset.category)));
  const out = options.get("out") === undefined ? undefined : resolve(options.get("out")!);
  if (out !== undefined && switches.has("fresh")) writeFileSync(out, "");
  const known = new Map<string, ScanRow>(out !== undefined && existsSync(out) ? readFileSync(out, "utf8").split("\n").filter(Boolean).map((line) => JSON.parse(line) as ScanRow).map((row) => [row.id, row]) : []);
  const todo = selection.filter((asset) => !known.has(asset.id));
  let cancelled = false;
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => void (cancelled = true));
  const started = Date.now();
  const rows: ScanRow[] = [];
  const clock = (seconds: number): string => `${String(Math.floor(seconds / 3600)).padStart(2, "0")}:${String(Math.floor((seconds % 3600) / 60)).padStart(2, "0")}:${String(Math.floor(seconds % 60)).padStart(2, "0")}`;
  if (selection.length > todo.length) console.error(`[scan] ${selection.length - todo.length} ${TEXT.skipped[language]}`);
  for (const [index, asset] of todo.entries()) {
    await new Promise<void>((turn) => setImmediate(turn));
    if (cancelled) break;
    const row = scanAsset(asset);
    rows.push(row);
    known.set(row.id, row);
    if (out !== undefined) appendFileSync(out, `${JSON.stringify(row)}\n`);
    if (!switches.has("json")) console.log(listing(row, language, switches.has("quiet")));
    const left = ((Date.now() - started) / 1000 / (index + 1)) * (todo.length - index - 1);
    if (todo.length > 1) console.error(`[scan] [${index + 1}/${todo.length}] ${asset.id} ${row.seconds.toFixed(1)} s · ${row.clean ? TEXT.clean[language] : `${row.summary.hard} ${TEXT.hard[language]}`} · ${TEXT.remaining[language]} ${clock(left)}`);
  }
  const all = selection.flatMap((asset) => known.get(asset.id) ?? []);
  if (switches.has("json")) console.log(JSON.stringify(all));
  if (options.get("report") !== undefined && !cancelled) writeFileSync(resolve(options.get("report")!), JSON.stringify({ version: REPORT_VERSION, rows: all.map(compact) }));
  console.error(`[scan] ${TEXT.total[language](all.length, all.filter((row) => row.clean).length, all.reduce((sum, row) => sum + row.summary.hard, 0))}`);
  if (cancelled) {
    console.error(`[scan] ${TEXT.cancelled[language]}`);
    return 130;
  }
  return all.every((row) => row.clean) ? 0 : 1;
}

if (process.argv[1] !== undefined && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.exit(await main());
