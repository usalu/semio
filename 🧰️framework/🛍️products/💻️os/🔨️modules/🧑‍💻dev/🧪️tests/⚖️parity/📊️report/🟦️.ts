/** 🧩️ Semantic parity report owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { ParityPlaygroundReport } from "../🏗️structure/🟦️.ts";



//#endregion 🔖️ServerPool

//#region 🔖️Report
function parityOutDir(): string {
  const configured = process.env.PARITY_OUT_DIR ?? ".🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY";
  const dir = resolve(repoRoot, configured);
  mkdirSync(dir, { recursive: true });
  return dir;
}

/** 🪜️terra-parity-rebaseline: a playground is `STALE-BRIDGE` if EITHER renderer's boot rung landed
 * there — the point is "not a real regression, don't count it against the architecture", and either
 * side hitting it is sufficient to know that. Checked before the generic `failed` filter below so a
 * stale-bridge variant is never double-counted as both. */
function isParityStaleBridge(r: ParityPlaygroundReport): boolean {
  return r.boot.react === "STALE-BRIDGE" || r.boot.wgpu === "STALE-BRIDGE";
}

function writeParityReport(reports: readonly ParityPlaygroundReport[]): void {
  const outDir = parityOutDir();
  writeFileSync(join(outDir, "parity-report-v2.json"), JSON.stringify(reports, null, 2), "utf8");
  const lines = ["# Wgpu Parity Report (v2 harness)", "", `Generated: ${reports.length} playground(s)`, "", "| Variant | React Boot | Wgpu Boot | Structural | Pixel | State | Action | React Δ | Wgpu Δ |", "|---|---|---|---|---|---|---|---:|---:|"];
  for (const r of reports) {
    const evidence = r.behavioral?.steps.find((step) => step.state)?.state;
    lines.push(
      `| ${r.variant} | ${r.boot.react} | ${r.boot.wgpu} | ${r.structural?.status ?? "-"} | ${r.pixel?.status ?? "-"} | ${r.behavioral?.status ?? "-"} | ${evidence ? `\`${evidence.actionKind}\` \`${evidence.actionPath}\`` : "-"} | ${evidence?.react.changedPaths.length ?? "-"} | ${evidence?.wgpu.changedPaths.length ?? "-"} |`,
    );
  }
  // 🪜️terra-parity-rebaseline: STALE-BRIDGE split OUT of `failed` — see `isParityStaleBridge`'s doc. A
  // blended "X/Y PASS" line conflates "the architecture regressed" with "the fleet hasn't regenerated
  // this bridge yet", which is exactly the false-regression risk 📌️important.md's re-baseline task
  // called out; the three-way split below is what makes a re-run after `sdk-green` lands legible.
  const staleBridge = reports.filter(isParityStaleBridge);
  const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
  const passed = reports.length - failed.length - staleBridge.length;
  lines.push("", `**${passed}/${reports.length} PASS · ${staleBridge.length}/${reports.length} STALE-BRIDGE (excluded from the architecture verdict) · ${failed.length}/${reports.length} FAIL**`);
  if (staleBridge.length > 0) lines.push("", `Stale-bridge variants (expected until the fleet's own bridge regenerates — 📌️important.md): ${staleBridge.map((r) => r.variant).join(", ")}`);
  writeFileSync(join(outDir, "parity-report-v2.md"), lines.join("\n"), "utf8");
}

export { isParityStaleBridge, parityOutDir, writeParityReport };
