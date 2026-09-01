#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.note.note@1/✳️any`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a DXF/SVG/PDF file or projects a
// semantic JSON shape — every measurement comes out of `../🏭️generator/🦀️note-oracle-codec`, the
// SAME crate that writes the fixtures (`dxf` 0.6 / `quick-xml` 0.42 / `lopdf` 0.44, the three
// already-registered oracles in `../🧪️oracle/🔣️.json`). `*-project` takes one file and reports what
// it contains; `*-compare` takes EXPECTED and ACTUAL and reports whether they agree — there,
// identical readings ARE the pass, exactly the shape
// `…✳️cad/🔬️probes/📜️script.ts` already establishes.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts dxf-project  --input <a.dxf>
//   bun 📜️script.ts svg-project  --input <a.svg>
//   bun 📜️script.ts pdf-project  --input <a.pdf>
//   bun 📜️script.ts dxf-compare  --input <expected.dxf> --input <actual.dxf>
//   bun 📜️script.ts svg-compare  --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts pdf-compare  --input <expected.pdf> --input <actual.pdf>
//
// @see ../🏭️generator/🦀️note-oracle-codec/src/cli.rs — the binary that does the reading/comparing
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { isAbsolute, join, resolve } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
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

const PROBES = ["dxf-project", "svg-project", "pdf-project", "dxf-compare", "svg-compare", "pdf-compare"] as const;
const CRATE_DIR = join(import.meta.dir, "..", "🏭️generator", "🦀️note-oracle-codec");
//#endregion 🧬️Contract

//#region 🚪️Entry
function failed(probe: string, message: string, detail?: string): ProbeReport {
  return {
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: "dxf@0.6 + quick-xml@0.42 + lopdf@0.44",
    engine: { family: probe.startsWith("dxf") ? "dxf-rs" : probe.startsWith("svg") ? "quick-xml" : "lopdf", implementation: "unavailable", version: "0" },
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
  // 🏭️`--offline` skipped on purpose (unlike the cad probe): this crate has no lockfile checked in
  // against a pre-warmed offline registry mirror in every environment this runs in, and a stale
  // `--offline` failure here would read as "the carrier disagrees" rather than "cargo couldn't
  // resolve deps" — a worse failure mode than the network round-trip cargo's own cache makes cheap
  // on every call after the first.
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(CRATE_DIR, "target"), "probe");
  // 📎️cargo must run in the CRATE directory, so every caller-supplied path is resolved against the
  // caller's cwd FIRST — the same fix `…✳️cad/🔬️probes/📜️script.ts` needed for the same reason.
  const resolved = argv.map((argument, index) => (index > 0 && argv[index - 1] === "--input" && !isAbsolute(argument) ? resolve(process.cwd(), argument) : argument));
  const run = spawnSync("cargo", ["run", "--quiet", "--bin", "note-oracle-codec", "--", ...resolved], {
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
