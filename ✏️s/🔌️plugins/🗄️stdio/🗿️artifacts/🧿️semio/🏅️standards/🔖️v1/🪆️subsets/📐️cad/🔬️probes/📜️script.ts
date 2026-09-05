#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.semio@v1/📐️cad`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a CAD file or computes a number. Every
// measurement comes out of a third-party library in the sibling `🦀️oracle-probe` crate — `ruststep`
// parses Part-21 and resolves the entity graph, the `dxf` crate parses ASCII DXF. Neither is ever
// asked what a mutation SHOULD produce; both are asked only what a file CONTAINS.
//
// The oracle is Rust rather than TypeScript because of what is actually available, not by preference:
// `node_modules` carries no JavaScript DXF parser and no JavaScript Part-21 reader, and `brepjs` —
// the one STEP-capable JS package vendored here — was MEASURED against a cad-shaped STEP file and
// returns zero shapes, because OCCT transfers shapes through product/shape-representation structure
// and this subset's export emits bare LINE/CIRCLE primitives. See the ticket note for that run.
// The crate carries its own `[workspace]` and links nothing from this repository, so it builds while
// the main workspace does not — the same pattern `…✳️cc6/🏭️bridge/` already uses.
//
// TWO QUESTIONS, TWO PROBES. `*-witness` takes BEFORE and AFTER and asks whether the carrier encoded
// the mutation at all; identical readings mean it did not, and the answer is `unsupported` rather
// than an empty `ok`. `*-compare` takes EXPECTED and ACTUAL and asks whether they agree; there,
// identical readings ARE the pass.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts dxf-read     --input <a.dxf>
//   bun 📜️script.ts step-read    --input <a.step>
//   bun 📜️script.ts dxf-witness  --input <before.dxf>   --input <after.dxf>  --mutation <kind>
//   bun 📜️script.ts step-witness --input <before.step>  --input <after.step> --mutation <kind>
//   bun 📜️script.ts dxf-compare  --input <expected.dxf>  --input <actual.dxf>
//   bun 📜️script.ts step-compare --input <expected.step> --input <actual.step>
//
// @see 🦀️oracle-probe/src/main.rs — the binary that does the reading
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { isAbsolute, resolve, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

const PROBES = ["dxf-read", "dxf-witness", "dxf-compare", "step-read", "step-witness", "step-compare"] as const;
const CRATE_DIR = join(import.meta.dir, "🦀️oracle-probe");
//#endregion 🧬️Contract

//#region 🚪️Entry
/** 🚨️ A probe that cannot RUN must say so in the report's own shape, never by printing nothing. */
function failed(probe: string, message: string, detail?: string): ProbeReport {
  return {
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: "ruststep@0.4.0 + dxf@0.6.1",
    engine: { family: probe.startsWith("step") ? "ruststep" : "dxf-rs", implementation: "unavailable", version: "0" },
    status: "failed",
    durationMs: 0,
    measurements: {},
    diagnostics: [{ severity: "error", message, ...(detail === undefined ? {} : { detail }) }],
  };
}

function main(argv: readonly string[]): number {
  const [probe = ""] = argv;
  if (!(PROBES as readonly string[]).includes(probe)) {
    console.error(`[probe] unknown probe ${JSON.stringify(probe)} — expected ${PROBES.join(" | ")}`);
    return 2;
  }
  // 🏭️`--offline` and an agent-scoped target directory: probes run inside a test sweep alongside peer
  // sessions, and a shared target directory is the single biggest source of cargo lock contention here.
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(CRATE_DIR, "target"), "probe");
  // 📎️cargo must run in the CRATE directory, so every caller-supplied path is resolved against the
  // caller's cwd FIRST. Passing them through unresolved made every relative `--input` resolve inside
  // the crate instead and the probe reported "No such file or directory" for files that were there.
  const resolved = argv.map((argument, index) => (index > 0 && argv[index - 1] === "--input" && !isAbsolute(argument) ? resolve(process.cwd(), argument) : argument));
  const run = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-cad-oracle-probe", "--", ...resolved], {
    cwd: CRATE_DIR,
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: target },
  });
  if (run.status !== 0) {
    console.log(JSON.stringify(failed(probe, `oracle probe exited ${run.status}`, (run.stderr ?? "").trim().split("\n").slice(-6).join("\n"))));
    return 1;
  }
  process.stdout.write(run.stdout);
  return 0;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export type { ProbeReport };
//#endregion 🚪️Entry
