/** 🩺️ Catalog check CLI: validates every asset of one category file (or all) and runs the full pipeline — evaluation at every pose and at
 * sampled clip times, thermal and structural solves and all seven views — reporting issues, non-finite values and equilibrium residuals.
 *
 * `repo tools exec bun -- <this file> [--file <category json>] [--asset <id>] [--samples 9]`
 *
 * @see ../🟦️.ts — semantic validation
 * @see ../../🔗️contracts/🟦️.ts — the pipeline contracts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";
import type { Asset, CategoryDocument } from "../../../🧬️schema/🟦️.ts";
import { VIEWS, type EvaluationQuery } from "../../🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🎨️depiction/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { solveThermal } from "../../🌡️thermal/🟦️.ts";
import { loadCatalogFromFiles } from "../📥️files/🟦️.ts";
import { validateAsset } from "../🟦️.ts";

/** 🔢️ The first non-finite number found in a value tree, as a dotted path. */
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

/** 🪂️ Whether an unsupported instant is declared: a pose authored without contacts, or a clip instant inside a `ballistic` phase. */
function ballistic(asset: Asset, query: EvaluationQuery, phase: string | null): boolean {
  if (query.kind === "pose") return asset.poses.find((pose) => pose.id === query.pose)?.contacts.length === 0 && asset.structure.supports.length === 0;
  if (query.kind !== "clip") return false;
  return asset.clips.find((clip) => clip.id === query.clip)?.phases.some((candidate) => candidate.id === phase && candidate.ballistic === true) ?? false;
}

/** 🧪️ Every query worth checking for one asset: all poses and `samples` instants per clip. */
function queriesOf(asset: Asset, samples: number): EvaluationQuery[] {
  return [
    ...asset.poses.map((pose): EvaluationQuery => ({ kind: "pose", pose: pose.id })),
    ...asset.clips.flatMap((clip) => Array.from({ length: samples }, (_, i): EvaluationQuery => ({ kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / Math.max(1, samples - 1) }))),
  ];
}

/** 🩺️ Checks one asset end to end and returns human-readable problems. */
export function checkAsset(asset: Asset, samples = 9): string[] {
  const problems = validateAsset(asset).map((issue) => `${issue.code} ${issue.path}: ${issue.message.en}`);
  for (const support of asset.structure.supports) {
    const offInPose = asset.poses.some((pose) => pose.contacts.includes(support)) && asset.poses.some((pose) => pose.contacts.length > 0 && !pose.contacts.includes(support));
    const offInClip = asset.clips.some((clip) => {
      const spans = clip.contacts.filter((span) => span.anchor === support);
      return spans.length > 0 && spans.reduce((sum, span) => sum + span.toS - span.fromS, 0) < clip.durationS - 1e-9;
    });
    if (offInPose || offInClip) problems.push(`support-switched structure.supports: ${support} is permanent but switched off by a pose or clip contact span`);
  }
  if (problems.length) return problems;
  for (const query of queriesOf(asset, samples)) {
    const label = query.kind === "pose" ? `pose ${query.pose}` : query.kind === "clip" ? `clip ${query.clip}@${query.timeS.toFixed(3)}` : "transition";
    try {
      const state = evaluate(asset, query);
      const thermal = solveThermal(state);
      const structure = solveStructure(state);
      const weight = structure.totalMassKg * structure.gravityMS2;
      const tolerance = Math.max(1e-6, weight * 1e-6);
      const supported = structure.reactions.some((reaction) => reaction.active);
      if (!supported && !ballistic(asset, query, state.phase)) problems.push(`${label}: no active support outside a ballistic phase`);
      if (supported && (Math.abs(structure.residual.fxN) > tolerance || Math.abs(structure.residual.fyN) > tolerance)) problems.push(`${label}: force residual ${structure.residual.fxN.toExponential(2)}, ${structure.residual.fyN.toExponential(2)} N`);
      for (const [name, value] of [["state", state], ["thermal", thermal], ["structure", structure]] as const) {
        const path = nonFinite(value);
        if (path) problems.push(`${label}: non-finite ${name}${path}`);
      }
      for (const view of VIEWS) {
        const scene = depict(state, defaultDepictionOptions(view, "en", 100), { thermal, structure });
        const path = nonFinite(scene.layers);
        if (path) problems.push(`${label} ${view}: non-finite scene${path}`);
      }
    } catch (error) {
      problems.push(`${label}: ${(error as Error).message}`);
    }
  }
  return problems;
}

if (import.meta.main) {
  const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, samples: { type: "string", default: "9" } } });
  const document = values.file ? (JSON.parse(readFileSync(resolve(values.file), "utf8")) as CategoryDocument | Asset) : null;
  const assets: readonly Asset[] = document ? ("assets" in document ? document.assets : [document]) : (await loadCatalogFromFiles()).assets;
  let failed = 0;
  for (const asset of assets.filter((candidate) => !values.asset || candidate.id === values.asset)) {
    const problems = checkAsset(asset, Number(values.samples));
    failed += problems.length ? 1 : 0;
    console.log(`${problems.length ? "✗" : "✓"} ${asset.id}${problems.length ? `\n  ${problems.slice(0, 20).join("\n  ")}` : ""}`);
  }
  process.exitCode = failed ? 1 : 0;
}
