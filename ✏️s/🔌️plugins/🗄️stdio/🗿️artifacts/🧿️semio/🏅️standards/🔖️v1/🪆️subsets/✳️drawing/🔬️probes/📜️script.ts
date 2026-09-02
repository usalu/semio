#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External carrier probes for `s.stdio.semio@v1/✳️drawing`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a carrier or computes a number. Every
// measurement comes out of a third-party library in the sibling `🦀️oracle-probe` crate — `quick-xml`
// parses the SVG, the IxMilia `dxf` crate parses the DXF, `lopdf` parses the PDF. This script only
// stamps the ProbeReport envelope around what they reported.
//
// WHY A RUST CHILD PROCESS RATHER THAN AN INLINE JS PARSER. The approved `test-oracle` readers for
// these three carriers are Rust crates (`🔒️dependencies.json`: `quick-xml 0.42`, `dxf 0.6`,
// `lopdf 0.44`). The XML and PDF parsers vendored in `node_modules` — `fast-xml-parser`, `sax`,
// `saxes`, `@xmldom/xmldom`, `pdfjs-dist` — are NOT approved oracle packages, and `pdfjs-dist` is
// production-reachable through `react-pdf`, so reading a carrier with it would put production code on
// the measurement path. The child crate declares its own `[workspace]` and links no repository crate
// at all, so it builds and runs independently of the plugin.
//
// THE CARRIER DECIDES WHAT IS CHECKABLE. A probe handed a carrier that cannot encode the property
// asked about returns `unsupported`, never an empty `ok`: DXF has no paint field for a path and PDF
// carries text alone, so an empty result read as ok would let a recolour pass against a file that
// never carried a colour.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts svg-structure  --input <a.svg>
//   bun 📜️script.ts dxf-entities   --input <a.dxf>
//   bun 📜️script.ts pdf-text       --input <a.pdf>
//   bun 📜️script.ts svg-compare    --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts style-compare  --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts dxf-compare    --input <expected.dxf> --input <actual.dxf>
//   bun 📜️script.ts pdf-compare    --input <expected.pdf> --input <actual.pdf>
//   bun 📜️script.ts gate-inputs    --out <dir>
//
// @see 🦀️oracle-probe/🦀️.rs — the readers themselves
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️bridge/📜️script.ts — the
//      spawn/offline/agent-scoped-target pattern this file mirrors

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { join } from "node:path";
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

/** ⚙️ Three DIFFERENT parser families, one per carrier. None of them is this repository's code, and no
 *  two of them share a kernel, so a defect in one cannot be confirmed by another. */
const ENGINES = {
  svg: { family: "quick-xml", implementation: "quick-xml pull reader", version: "0.42.0" },
  dxf: { family: "ixmilia-dxf", implementation: "IxMilia.Dxf rust port", version: "0.6.1" },
  pdf: { family: "lopdf", implementation: "lopdf document reader", version: "0.44.0" },
} as const;

const PROBE_VERSION = "quick-xml@0.42.0 + dxf@0.6.1 + lopdf@0.44.0";
const CRATE_BIN = "semio-drawing-oracle-probe";

/** ⚙️ Which reader family answers each probe — recorded per probe so the report names the engine that
 *  actually produced the numbers rather than a single blanket family for the whole subset. */
const PROBE_ENGINES: Record<string, (typeof ENGINES)[keyof typeof ENGINES]> = {
  "svg-structure": ENGINES.svg,
  "svg-compare": ENGINES.svg,
  "style-compare": ENGINES.svg,
  "dxf-entities": ENGINES.dxf,
  "dxf-compare": ENGINES.dxf,
  "pdf-text": ENGINES.pdf,
  "pdf-compare": ENGINES.pdf,
  "gate-inputs": ENGINES.svg,
};
//#endregion 🧬️Contract

//#region 🚀️Entry
/** 🏭️ `--offline` and an agent-scoped target directory: probes run inside a test sweep alongside peer
 *  sessions, and a shared cargo target directory is the single biggest source of lock contention. */
function cargoTargetDir(): string {
  return process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "🦀️oracle-probe", "target"), "oracle-probe");
}

function emit(report: ProbeReport): number {
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  return report.status === "failed" ? 1 : 0;
}

function main(argv: readonly string[]): number {
  const [probe = "", ...rest] = argv;
  const started = Date.now();
  const engine = PROBE_ENGINES[probe] ?? ENGINES.svg;
  const envelope = (status: ProbeReport["status"], measurements: Record<string, unknown>, diagnostics?: ProbeReport["diagnostics"]): ProbeReport => ({
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: PROBE_VERSION,
    engine,
    status,
    durationMs: Date.now() - started,
    measurements,
    ...(diagnostics ? { diagnostics } : {}),
  });

  if (PROBE_ENGINES[probe] === undefined) {
    return emit(envelope("failed", {}, [{ severity: "error", message: `unknown probe ${probe || "(none)"}`, detail: `known: ${Object.keys(PROBE_ENGINES).join(", ")}` }]));
  }

  const run = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", CRATE_BIN, "--", probe, ...rest], {
    cwd: join(import.meta.dir, "🦀️oracle-probe"),
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: cargoTargetDir() },
  });
  if (run.status !== 0) {
    // 🚫️A reader that cannot run must SAY SO and exit non-zero. Emitting a plausible-looking empty
    // measurement would let the pipeline compare nothing and call it agreement.
    return emit(envelope("failed", {}, [{ severity: "error", message: `${CRATE_BIN} exited ${run.status}`, detail: (run.stderr ?? "").trim().split("\n").slice(-6).join("\n") }]));
  }
  let parsed: { status: ProbeReport["status"]; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };
  try {
    parsed = JSON.parse((run.stdout ?? "").trim().split("\n").at(-1) ?? "") as typeof parsed;
  } catch (error) {
    return emit(envelope("failed", {}, [{ severity: "error", message: "probe output was not JSON", detail: String((error as Error).message ?? error) }]));
  }
  return emit(envelope(parsed.status, parsed.measurements, parsed.diagnostics));
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
//#endregion 🚀️Entry
