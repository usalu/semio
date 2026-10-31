/** 🩺️ Catalog check CLI v2: validates assets and runs the whole pipeline at every authored scale, reporting issues, non-finite values, equilibrium residuals and print suitability.
 *
 * `repo tools exec bun -- <this file> [--asset <id>] [--category <id>] [--file <asset json>] [--catalog <root>] [--samples 9] [--profile <id>] [--no-physics]`
 *
 * Per asset: `validateAsset`; for every authored representation the measured suitability under the print profile (default `office-laser`); every named pose and `samples` instants per clip
 * are evaluated at every authored scale, solved thermally and structurally once (the physics does not depend on the scale) and depicted in all seven views at every authored scale; unavailable
 * scales are listed with their reason. Without `--asset`/`--category` the manifest is validated against all assets too. `✓`/`✗` per asset; exit code 1 when any asset fails.
 *
 * @see ../🟦️.ts — semantic validation
 * @see ../../🖨️representation/🟦️.ts — suitability
 * @see ../../🔗️contracts/🟦️.ts — the pipeline contracts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";
import type { Asset, AssetDocument, PrintProfile, ScaleDenominator } from "../../../🧬️schema/🟦️.ts";
import { SCALE_DENOMINATORS } from "../../../🧬️schema/🟦️.ts";
import { VIEWS, type EvaluationQuery, type JobControl, type Suitability } from "../../🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🎨️depiction/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, PRINT_PROFILES, measureSuitability } from "../../🖨️representation/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../🌡️thermal/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../📥️files/🟦️.ts";
import { type Issue, validateAsset, validateManifest } from "../🟦️.ts";

/** 🎛️ What a check run needs besides the asset. */
export type CheckOptions = { readonly samples?: number; readonly profile?: PrintProfile; readonly assets?: ReadonlySet<string>; readonly physics?: boolean };

/** 📋️ The outcome of checking one asset: problems (empty = ✓) and report lines (suitability and unavailable scales). */
export type CheckReport = { readonly problems: readonly string[]; readonly lines: readonly string[]; readonly issues: readonly Issue[]; readonly scales: readonly ScaleReport[] };

/** 📏️ The decision and measured suitability of one preset scale of an asset. */
export type ScaleReport = { readonly denominator: ScaleDenominator; readonly status: "authored" | "unavailable"; readonly suitability?: Suitability; readonly error?: string };

const UNCHECKED: PrintProfile = { id: "unchecked", title: { en: "Unchecked", de: "Ungeprüft" }, description: { en: "Diagnostic profile without limits.", de: "Diagnoseprofil ohne Grenzen." }, minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0, source: "illustrative" };

function nonFinite(value: unknown, path = ""): string | null {
  if (typeof value === "number") return Number.isFinite(value) ? null : path;
  if (value === null || typeof value !== "object") return null;
  for (const [key, child] of Object.entries(value)) {
    if (key === "asset") continue;
    const found = nonFinite(child, `${path}.${key}`);
    if (found) return found;
  }
  return null;
}

function ballistic(asset: Asset, query: EvaluationQuery, phase: string | null): boolean {
  if (query.kind === "pose") return asset.poses.find((pose) => pose.id === query.pose)?.contacts.length === 0 && asset.structure.supports.length === 0;
  if (query.kind !== "clip") return false;
  return asset.clips.find((clip) => clip.id === query.clip)?.phases.some((candidate) => candidate.id === phase && candidate.ballistic === true) ?? false;
}

function queriesOf(asset: Asset, samples: number): EvaluationQuery[] {
  return [
    ...asset.poses.filter((pose) => !pose.key).map((pose): EvaluationQuery => ({ kind: "pose", pose: pose.id })),
    ...asset.clips.flatMap((clip) => Array.from({ length: samples }, (_, i): EvaluationQuery => ({ kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / Math.max(1, samples - 1) }))),
  ];
}

function describe(suitability: Suitability): string {
  const mm = (value: number): string => (Number.isFinite(value) ? value.toFixed(3) : "–");
  return `${suitability.suitable ? "suitable" : "UNSUITABLE"} under ${suitability.profile.id}: min stroke ${mm(suitability.minStrokeMm)} mm, min gap ${mm(suitability.minGapMm)} mm, min feature ${mm(suitability.minFeatureMm)} mm`;
}

/** 🩺️ Checks one asset end to end: validation, suitability per authored scale, evaluation, thermal, structure and all seven views at every authored scale. */
export function checkAssetDetailed(asset: Asset, options: CheckOptions = {}, control: JobControl = {}): CheckReport {
  const samples = options.samples ?? 9;
  const physics = options.physics ?? true;
  const profile = options.profile ?? DEFAULT_PRINT_PROFILE;
  const lines: string[] = [];
  const issues = validateAsset(asset, options.assets === undefined ? {} : { assets: options.assets });
  const problems = issues.map((issue) => `${issue.code} ${issue.path}: ${issue.message.en}`);
  const scales: ScaleReport[] = [];
  for (const support of asset.structure.supports) {
    const offInPose = asset.poses.some((pose) => pose.contacts.includes(support)) && asset.poses.some((pose) => pose.contacts.length > 0 && !pose.contacts.includes(support));
    const offInClip = asset.clips.some((clip) => {
      const spans = clip.contacts.filter((span) => span.anchor === support);
      return spans.length > 0 && spans.reduce((sum, span) => sum + span.toS - span.fromS, 0) < clip.durationS - 1e-9;
    });
    if (offInPose || offInClip) problems.push(`support-switched structure.supports: ${support} is permanent but switched off by a pose or clip contact span`);
  }
  const authored: ScaleDenominator[] = [];
  for (const denominator of SCALE_DENOMINATORS) {
    const rep = asset.representations[`${denominator}`];
    if (rep === undefined) continue;
    if (rep.status === "unavailable") {
      scales.push({ denominator, status: "unavailable" });
      lines.push(`1:${denominator} unavailable (${rep.limitingFeature.en}, ${rep.limitingSizeM} m): ${rep.reason.en}`);
      continue;
    }
    authored.push(denominator);
    try {
      const suitability = measureSuitability(asset, rep, denominator, profile);
      scales.push({ denominator, status: "authored", suitability });
      lines.push(`1:${denominator} authored, ${describe(suitability)}`);
      for (const violation of suitability.violations.slice(0, 6)) problems.push(`suitability-${violation.check} 1:${denominator} ${violation.shapes.join("+")}: ${violation.measuredMm.toFixed(3)} mm < ${violation.requiredMm} mm`);
    } catch (error) {
      scales.push({ denominator, status: "authored", error: (error as Error).message });
      lines.push(`1:${denominator} authored, suitability not measurable: ${(error as Error).message}`);
      problems.push(`suitability-error 1:${denominator}: ${(error as Error).message}`);
    }
  }
  if (issues.length) return { problems, lines, issues, scales };
  const queries = queriesOf(asset, samples);
  queries.forEach((query, index) => {
    control.signal?.throwIfAborted();
    control.progress?.(index, queries.length, asset.title);
    const label = query.kind === "pose" ? `pose ${query.pose}` : query.kind === "clip" ? `clip ${query.clip}@${query.timeS.toFixed(3)}` : "transition";
    try {
      const physical = evaluate(asset, query);
      const thermal = physics ? solveThermalAt(asset, query) : undefined;
      const structure = physics ? solveStructure(physical) : undefined;
      if (structure !== undefined) {
        const weight = structure.totalMassKg * structure.gravityMS2;
        const tolerance = Math.max(1e-6, weight * 1e-6);
        const supported = structure.reactions.some((reaction) => reaction.active);
        if (!supported && !ballistic(asset, query, physical.phase)) problems.push(`${label}: no active support outside a ballistic phase`);
        if (supported && (Math.abs(structure.residual.fxN) > tolerance || Math.abs(structure.residual.fyN) > tolerance)) problems.push(`${label}: force residual ${structure.residual.fxN.toExponential(2)}, ${structure.residual.fyN.toExponential(2)} N`);
      }
      for (const [name, value] of [["state", physical], ["thermal", thermal], ["structure", structure]] as const) {
        const path = nonFinite(value);
        if (path) problems.push(`${label}: non-finite ${name}${path}`);
      }
      for (const denominator of authored) {
        const state = evaluate(asset, query, { kind: "scale", scale: { denominator } });
        for (const view of physics ? VIEWS : VIEWS.slice(0, 4)) {
          const scene = depict(state, { ...defaultDepictionOptions(view, "en", denominator), profile: UNCHECKED }, { ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}) });
          const path = nonFinite(scene.layers);
          if (path) problems.push(`${label} 1:${denominator} ${view}: non-finite scene${path}`);
          if (scene.layers.every((layer) => layer.items.length === 0)) problems.push(`${label} 1:${denominator} ${view}: empty scene`);
        }
      }
    } catch (error) {
      problems.push(`${label}: ${(error as Error).message}`);
    }
  });
  return { problems, lines, issues, scales };
}

/** 🩺️ The problems of {@link checkAssetDetailed}, for callers that only need the verdict. */
export function checkAsset(asset: Asset, samples = 9, options: Omit<CheckOptions, "samples"> = {}): string[] {
  return [...checkAssetDetailed(asset, { ...options, samples }).problems];
}

if (import.meta.main) {
  const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, category: { type: "string" }, catalog: { type: "string" }, samples: { type: "string", default: "9" }, profile: { type: "string" }, "no-physics": { type: "boolean", default: false }, quiet: { type: "boolean", default: false } } });
  const profile = values.profile === undefined ? DEFAULT_PRINT_PROFILE : (PRINT_PROFILES.profiles.find((candidate) => candidate.id === values.profile) ?? (() => { throw new Error(`unknown profile "${values.profile}"`); })());
  const catalog = values.file ? null : await loadCatalogFromFiles(values.catalog ? resolve(values.catalog) : DEFAULT_CATALOG_ROOT);
  const assets: readonly Asset[] = values.file ? [(JSON.parse(readFileSync(resolve(values.file), "utf8")) as AssetDocument).asset] : catalog!.assets;
  const known = new Set(assets.map((asset) => asset.id));
  const selected = assets.filter((asset) => (!values.asset || asset.id === values.asset) && (!values.category || asset.category === values.category));
  if (selected.length === 0) console.log("✗ no asset selected");
  let failed = selected.length === 0 ? 1 : 0;
  if (catalog && !values.asset && !values.category) {
    const issues = [...validateManifest(catalog.manifest, assets)];
    for (const entry of catalog.loaded) {
      const category = catalog.manifest.categories.find((candidate) => candidate.id === entry.asset.category);
      if (entry.folder !== entry.asset.id) issues.push({ path: entry.path, code: "asset-folder", message: { en: `Asset "${entry.asset.id}" lives in folder "${entry.folder}".`, de: `Asset "${entry.asset.id}" liegt im Ordner "${entry.folder}".` } });
      if (category !== undefined && category.folder !== entry.categoryFolder) issues.push({ path: entry.path, code: "category-folder", message: { en: `Asset "${entry.asset.id}" of category "${entry.asset.category}" lives under "${entry.categoryFolder}".`, de: `Asset "${entry.asset.id}" der Kategorie "${entry.asset.category}" liegt unter "${entry.categoryFolder}".` } });
    }
    console.log(`${issues.length ? "✗" : "✓"} manifest${issues.length ? `\n  ${issues.slice(0, 40).map((issue) => `${issue.code} ${issue.path}: ${issue.message.en}`).join("\n  ")}` : ""}`);
    failed += issues.length ? 1 : 0;
  }
  for (const [index, asset] of selected.entries()) {
    const report = checkAssetDetailed(asset, { samples: Number(values.samples), profile, assets: known, physics: !values["no-physics"] });
    failed += report.problems.length ? 1 : 0;
    console.log(`${report.problems.length ? "✗" : "✓"} ${asset.id} (${index + 1}/${selected.length})${values.quiet ? "" : report.lines.map((line) => `\n  ${line}`).join("")}${report.problems.length ? `\n  ${report.problems.slice(0, 30).join("\n  ")}` : ""}`);
  }
  process.exitCode = failed ? 1 : 0;
}
