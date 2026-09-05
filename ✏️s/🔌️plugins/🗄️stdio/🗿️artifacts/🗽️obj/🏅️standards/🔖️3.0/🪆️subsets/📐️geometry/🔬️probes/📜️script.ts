#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.obj@3.0/📐️geometry`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual OBJ decode is performed by the sibling standalone `tobj-obj-reader` binary
// (`../🏭️generator/📖️tobj-obj-reader`, depends on nothing but the real `tobj` 4 — the SAME crate
// registered as `tobj-obj-3-0-mutate-reader`) via its `project` subcommand — this file only shells
// out to it and performs the GATING structural comparison itself. No OBJ semantics computed here,
// only projection + compare, mirroring the sibling `avi`/`bcf` probe suites' identical division of
// labor.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts obj-import  --input <a.obj>
import { readFileSync } from "node:fs";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
const objLoader = new OBJLoader();
//   bun 📜️script.ts obj-project --input <a.obj>
//   bun 📜️script.ts obj-compare --input <expected.obj> --input <actual.obj>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/📐️geometry/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from
// @see ../🏭️generator/📖️tobj-obj-reader/src/main.rs — the `project` subcommand this file calls, and
//      the module doc there for exactly which 12 of the 22 declared mutation kinds `tobj` (a MESH
//      reader) can witness at all

//#endregion 🧲️Header

//#region 🔌️Adapters
import { join } from "node:path";
import { spawnSync } from "node:child_process";
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

const ENGINE = { family: "tobj", implementation: "tobj-obj-reader (tobj 4, single_index+triangulate)", version: "tobj@4" } as const;
const PROBE_VERSION = "tobj@4";
const READER_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "📖️tobj-obj-reader", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type ObjModel = { name: string; vertexCount: number; triangleCount: number; positions: number[][]; texcoords: number[][]; normals: number[][]; triangles: number[][] };
type ObjDoc = { modelCount: number; totalTriangleCount: number; models: ObjModel[] };

/** 📥️ Runs the standalone reader's `project` subcommand — never computes OBJ semantics itself. */
function readObj(path: string): ObjDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--release", "--manifest-path", READER_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`tobj-obj-reader project ${path} failed (exit ${result.status}): ${result.stderr}`);
  return JSON.parse(result.stdout) as ObjDoc;
}
//#endregion 📥️Model

//#region ⚖️Compare
/** ⚖️ Positional structural equality — models/triangles/positions project as ORDERED arrays (the
 *  same convention the `avi` probe's own `diffAt` uses); `tobj` assigns model order by first
 *  appearance in the file, which is this format's own real order, not an incidental one. */
function diffAt(path: string, expected: unknown, actual: unknown, diffs: string[]): void {
  if (Array.isArray(expected) && Array.isArray(actual)) {
    const len = Math.max(expected.length, actual.length);
    for (let i = 0; i < len; i += 1) diffAt(`${path}[${i}]`, expected[i], actual[i], diffs);
    return;
  }
  if (typeof expected === "object" && expected !== null && typeof actual === "object" && actual !== null && !Array.isArray(expected)) {
    const keys = new Set([...Object.keys(expected as object), ...Object.keys(actual as object)]);
    for (const key of keys) diffAt(`${path}.${key}`, (expected as Record<string, unknown>)[key], (actual as Record<string, unknown>)[key], diffs);
    return;
  }
  if (JSON.stringify(expected) !== JSON.stringify(actual)) diffs.push(`${path}: ${JSON.stringify(expected)} ≠ ${JSON.stringify(actual)}`);
}

function compareDocs(expected: ObjDoc, actual: ObjDoc): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$", expected, actual, diffs);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"]; engine?: ProbeReport["engine"]; probeVersion?: string };

const DOCUMENT_ENGINE = { family: "threejs", implementation: "three OBJLoader (document statements: mtllib, usemtl, smoothing)", version: "0.182.0" } as const;
const DOCUMENT_PROBE_VERSION = "three@0.182.0";

/** 🧾️ The document-level statements `tobj` discards, as `three`'s OBJLoader recovers them. */
function documentProjection(absPath: string): Record<string, unknown> {
  const group = objLoader.parse(readFileSync(absPath, "utf8"));
  const children: Record<string, unknown>[] = [];
  group.traverse((child: { isMesh?: boolean; isLine?: boolean; isPoints?: boolean; name?: string; material?: unknown }) => {
    if (!child.isMesh && !child.isLine && !child.isPoints) return;
    const materials = Array.isArray(child.material) ? child.material : [child.material];
    // 📐️RESOLVED geometry, not the raw statement lists. OBJ face indices are ABSOLUTE, so inserting or
    // removing an element at the FRONT of the v/vt/vn list changes what every subsequent index resolves
    // to — which is how a mesh reader witnesses those kinds at all. An insertion PAST the last
    // referenced element would not move this projection; the fixtures exercise the front, where the
    // kind is observable, and the oracle's rationale says so rather than leaving it implied.
    const geometry = (child as { geometry?: { getAttribute(name: string): { array: ArrayLike<number> } | undefined } }).geometry;
    const attribute = (name: string): number[] | null => {
      const a = geometry?.getAttribute(name);
      return a ? Array.from(a.array as ArrayLike<number>).map((v) => Number(v.toFixed(4))) : null;
    };
    children.push({
      name: child.name ?? null,
      materialNames: materials.map((m: { name?: string } | undefined) => m?.name ?? null),
      flatShading: materials.map((m: { flatShading?: boolean } | undefined) => m?.flatShading ?? null),
      positions: attribute("position"),
      normals: attribute("normal"),
      uvs: attribute("uv"),
    });
  });
  return { materialLibraries: (group as { materialLibraries?: string[] }).materialLibraries ?? [], children };
}

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  // 🧾️DOCUMENT-LEVEL probes, on a different engine from the three above.
  //
  // `tobj` is a MESH reader: it resolves faces into buffers and drops everything that is not geometry,
  // so `mtllib`, `usemtl` and smoothing-group statements are invisible to it. That is why those kinds
  // were recorded `-uncarried` against it — honest about tobj, and not a general claim.
  //
  // `three`'s OBJLoader parses those same statements and keeps them: `materialLibraries` on the parsed
  // group, the material NAME per child, and `flatShading` derived from `s off`. Measured before it was
  // registered: `set-mtllib`, `set-usemtl` and `set-smoothing-groups` each move this projection, while
  // the vertex/texcoord/normal insert-and-remove kinds do NOT — an unreferenced element is dropped by
  // this loader too — and an unknown statement is skipped with a warning. Only the three that move are
  // claimed.
  "obj-document-project": async (inputs) => {
    requireInputs(inputs, 1, "obj-document-project");
    return { status: "ok", engine: DOCUMENT_ENGINE, probeVersion: DOCUMENT_PROBE_VERSION, measurements: documentProjection(inputs[0]!) } as ProbeResult;
  },
  "obj-document-compare": async (inputs) => {
    requireInputs(inputs, 2, "obj-document-compare");
    const expected = JSON.stringify(documentProjection(inputs[0]!));
    const actual = JSON.stringify(documentProjection(inputs[1]!));
    return { status: "ok", engine: DOCUMENT_ENGINE, probeVersion: DOCUMENT_PROBE_VERSION, measurements: { equal: expected === actual, expected: JSON.parse(expected), actual: JSON.parse(actual) } } as ProbeResult;
  },
  "obj-import": async (inputs) => {
    requireInputs(inputs, 1, "obj-import");
    const perInput = inputs.map((input) => {
      try {
        readObj(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "obj-project": async (inputs) => {
    requireInputs(inputs, 1, "obj-project");
    const doc = readObj(inputs[0]!);
    return { status: "ok", measurements: { modelCount: doc.modelCount, totalTriangleCount: doc.totalTriangleCount, projection: doc } };
  },
  "obj-compare": async (inputs) => {
    requireInputs(inputs, 2, "obj-compare");
    const expected = readObj(inputs[0]!);
    const actual = readObj(inputs[1]!);
    const verdict = compareDocs(expected, actual);
    return { status: "ok", measurements: { ...verdict, expected, actual } };
  },
};
//#endregion 🔬️Probes

//#region 🚀️Entry
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[] } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  for (let i = 0; i < rest.length; i += 1) if (rest[i] === "--input") inputs.push(rest[i + 1] ?? "");
  return { probe, inputs };
}

async function main(argv: readonly string[]): Promise<number> {
  const { probe, inputs } = parseArgv(argv);
  const started = Date.now();
  const emit = (report: ProbeReport): number => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return report.status === "failed" ? 1 : 0;
  };
  const budgetMs = Number(process.env.SEMIO_PROBE_TIMEOUT_MS ?? 60_000);
  const watchdog = new Promise<never>((_, reject) => setTimeout(() => reject(new Error(`probe exceeded ${budgetMs} ms`)), budgetMs).unref?.());
  const run = PROBES[probe];
  if (!run) return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  try {
    const result = await Promise.race([run(inputs), watchdog]);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: (result as { probeVersion?: string }).probeVersion ?? PROBE_VERSION, engine: (result as { engine?: typeof ENGINE }).engine ?? ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
