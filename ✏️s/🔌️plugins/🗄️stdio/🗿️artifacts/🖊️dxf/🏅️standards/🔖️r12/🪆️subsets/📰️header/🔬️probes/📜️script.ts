#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.dxf@r12/📰️header`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual DXF decode is performed by the sibling standalone engine binary
// (`../🏭️generator/🦀️engine`, depends on nothing but `dxf` 0.6) via its `project` subcommand — this
// file only shells out to it and performs the GATING structural comparison itself against
// `semantic-dxf-r12-v1`'s own rules (LAYER/STYLE/LTYPE table rows are NAME-keyed; BLOCKS and
// ENTITIES — including a block's own nested entity list — are ORDER-significant, matching the real
// production dispatch's own index-addressed `DxfBlocksDiff`/`DxfEntitiesDiff`, read directly from
// `../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md`'s
// own citations of `🧬️schema/🔺️diff/🦀️.rs`) — no DXF semantics computed here beyond that
// comparison, only projection + compare. This subset's document has no large opaque binary
// payloads (unlike AVI's movi chunks), so unlike that sibling probe suite there is nothing here to
// hash: the engine's own typed projection is already the full comparable shape.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts dxf-import  --input <a.dxf>
//   bun 📜️script.ts dxf-project --input <a.dxf>
//   bun 📜️script.ts dxf-compare --input <expected.dxf> --input <actual.dxf>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../💬️avi/🏅️standards/🔖️1.0/🪆️subsets/📰️header/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from
// @see ../🏭️generator/🦀️engine/src/main.rs — the `project` subcommand this file calls

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

const ENGINE = { family: "dxf-rs", implementation: "engine (dxf 0.6 + this artifact's own semantic-dxf-r12-v1 projection)", version: "dxf@0.6.1" } as const;
const PROBE_VERSION = "dxf@0.6.1";
const ENGINE_BIN = join(import.meta.dir, "..", "🏭️generator", "🦀️engine", "target", "release", "generate");
const TOLERANCE = 0.0001;
/** ⚖️ `semantic-dxf-r12-v1`'s own named-vs-ordered split, per its `description` and this subset's
 *  real production dispatch (`validate_named_targets` for these three tables, `validate_indexed_targets`
 *  for blocks/entities). */
const NAME_KEYED_FIELDS = new Set(["layers", "styles", "linetypes"]);
//#endregion 🧬️Contract

//#region 📥️Model
type DxfDoc = {
  acadVersion: string;
  insertionBase: [number, number, number];
  layers: { name: string; color: number; linetype: string }[];
  styles: { name: string; font: string }[];
  linetypes: { name: string; description: string }[];
  blocks: { name: string; basePoint: [number, number, number]; entities: unknown[] }[];
  entities: unknown[];
};
//#endregion 📥️Model

//#region 🔓️Read
/** 📥️ Runs the standalone engine's `project` subcommand — `dxf` 0.6 parses the real bytes; this
 *  function only marshals the resulting JSON. */
function readDxf(path: string): DxfDoc {
  const result = spawnSync(ENGINE_BIN, ["project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`engine project ${path} failed (exit ${result.status}): ${result.stderr}`);
  return JSON.parse(result.stdout) as DxfDoc;
}
//#endregion 🔓️Read

//#region ⚖️Compare
function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** ⚖️ Structural equality: numbers compare within `semantic-dxf-r12-v1`'s own tolerance (1e-4);
 *  everything else compares exactly; arrays compare positionally UNLESS `nameKeyed` says the
 *  current path is one of the three named tables, in which case both sides are matched up by their
 *  own `name` field instead of by position — matching `validate_named_targets`'s name identity, not
 *  array position, and never emitting the redundant positional shift-noise a plain sort-then-diff
 *  would produce when one side has an extra row. */
function diffAt(path: string, expected: unknown, actual: unknown, diffs: string[], nameKeyedHere: boolean): void {
  if (typeof expected === "number" && typeof actual === "number") {
    if (Math.abs(expected - actual) > TOLERANCE) diffs.push(`${path}: ${expected} ≠ ${actual} (delta ${Math.abs(expected - actual)})`);
    return;
  }
  if (Array.isArray(expected) && Array.isArray(actual)) {
    if (nameKeyedHere) {
      const nameOf = (item: unknown): string => String(isPlainObject(item) ? item["name"] : "");
      const expectedByName = new Map(expected.map((item) => [nameOf(item), item]));
      const actualByName = new Map(actual.map((item) => [nameOf(item), item]));
      for (const [name, item] of expectedByName) {
        if (!actualByName.has(name)) diffs.push(`${path}[name=${name}]: present in expected, absent in actual`);
        else diffAt(`${path}[name=${name}]`, item, actualByName.get(name), diffs, false);
      }
      for (const name of actualByName.keys()) if (!expectedByName.has(name)) diffs.push(`${path}[name=${name}]: absent in expected, present in actual`);
      return;
    }
    const len = Math.max(expected.length, actual.length);
    for (let i = 0; i < len; i += 1) diffAt(`${path}[${i}]`, expected[i], actual[i], diffs, false);
    return;
  }
  if (isPlainObject(expected) && isPlainObject(actual)) {
    const keys = new Set([...Object.keys(expected), ...Object.keys(actual)]);
    for (const key of keys) diffAt(`${path}.${key}`, expected[key], actual[key], diffs, NAME_KEYED_FIELDS.has(key));
    return;
  }
  if (JSON.stringify(expected) !== JSON.stringify(actual)) diffs.push(`${path}: ${JSON.stringify(expected)} ≠ ${JSON.stringify(actual)}`);
}

function compareDocs(expected: DxfDoc, actual: DxfDoc): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$", expected, actual, diffs, false);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "dxf-import": async (inputs) => {
    requireInputs(inputs, 1, "dxf-import");
    const perInput = inputs.map((input) => {
      try {
        readDxf(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "dxf-project": async (inputs) => {
    requireInputs(inputs, 1, "dxf-project");
    const doc = readDxf(inputs[0]!);
    return { status: "ok", measurements: { layerCount: doc.layers.length, styleCount: doc.styles.length, linetypeCount: doc.linetypes.length, blockCount: doc.blocks.length, entityCount: doc.entities.length, projection: doc } };
  },
  "dxf-compare": async (inputs) => {
    requireInputs(inputs, 2, "dxf-compare");
    const expected = readDxf(inputs[0]!);
    const actual = readDxf(inputs[1]!);
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
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
