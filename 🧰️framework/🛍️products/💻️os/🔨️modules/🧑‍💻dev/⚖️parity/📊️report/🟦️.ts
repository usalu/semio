/** 🧩️ Semantic parity report owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { ParityPlaygroundReport } from "../🏗️structure/🟦️.ts";
import { parityVerdict } from "../✅️verdict/🟦️.ts";



//#endregion 🔖️ServerPool

//#region 🔖️Report
function parityOutDir(): string {
  const configured = process.env.PARITY_OUT_DIR ?? ".🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY";
  const dir = resolve(repoRoot, configured);
  mkdirSync(dir, { recursive: true });
  return dir;
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
  const counts = ["PASS", "FAIL", "INCOMPLETE", "STALE-BRIDGE"].map(verdict => `${reports.filter(report => parityVerdict(report) === verdict).length}/${reports.length} ${verdict}`);
  lines.push("", `**${counts.join(" · ")}**`);
  writeFileSync(join(outDir, "parity-report-v2.md"), lines.join("\n"), "utf8");
}

export { parityOutDir, writeParityReport };
