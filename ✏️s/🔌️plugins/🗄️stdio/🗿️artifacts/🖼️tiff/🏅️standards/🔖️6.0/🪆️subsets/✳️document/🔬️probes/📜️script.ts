#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.tiff@6.0/✳️document`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual TIFF IFD/tag decode is performed by the sibling standalone `tiff-ifd-codec`
// binary (`../🏭️generator/🦀️tiff-ifd-codec`, depends on nothing but `tiff` 0.11) via its `project`
// subcommand — this file only shells out to it and performs the GATING structural comparison
// itself (already-hashed `samplesDigest` for the raster, per the codec's own opaque-payload
// treatment of pixel data — never raw bytes cross this boundary either) — no TIFF semantics
// computed here, only marshalling + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts tiff-import  --input <a.tiff>
//   bun 📜️script.ts tiff-project --input <a.tiff>
//   bun 📜️script.ts tiff-compare --input <expected.tiff> --input <actual.tiff>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🦀️tiff-ifd-codec/src/main.rs — the `project` subcommand this file calls

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

const ENGINE = { family: "tiff", implementation: "tiff-ifd-codec (tiff 0.11.3 decoder/encoder, no hand-rolled field layout)", version: "tiff@0.11.3" } as const;
const PROBE_VERSION = "tiff@0.11.3";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️tiff-ifd-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
/** 🌳 What `tiff-ifd-codec project` emits verbatim — the projection profile IS the wire shape;
 *  large pixel data already arrives as a size+digest pair, never raw samples. */
type TypedValue = { kind: string; value?: unknown; n?: number; d?: number; items?: TypedValue[] };
type Entry = { tag: number; value: TypedValue };
type Raster = { width: number; height: number; colorType: string; sampleByteLength: number; samplesDigest: string } | null;
type Ifd = { index: number; entries: Entry[]; raster: Raster };
type TiffDoc = { format: "tiff"; byteOrder: "little-endian" | "big-endian"; ifdCount: number; ifds: Ifd[] };
//#endregion 📥️Model

//#region 🔓️Read
/** 📥️ Runs the standalone codec's `project` subcommand — never decodes a byte of TIFF itself. */
function readTiff(path: string): TiffDoc {
  const result = spawnSync("cargo", ["run", "--offline", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`tiff-ifd-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  return JSON.parse(result.stdout) as TiffDoc;
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Positional structural equality — IFD index and, within an IFD, tag id order are this
 *  format's own on-disk layout; the codec already sorts each IFD's entries by tag id (mirroring
 *  the sibling `🦀️oracle.rs`'s own `entries.sort_by_key(|t| t.tag)`), so this compares
 *  ordered arrays throughout — no reordering before diffing. */
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

function compareDocs(expected: TiffDoc, actual: TiffDoc): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$", expected, actual, diffs);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "tiff-import": async (inputs) => {
    requireInputs(inputs, 1, "tiff-import");
    const perInput = inputs.map((input) => {
      try {
        readTiff(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "tiff-project": async (inputs) => {
    requireInputs(inputs, 1, "tiff-project");
    const doc = readTiff(inputs[0]!);
    return { status: "ok", measurements: { byteOrder: doc.byteOrder, ifdCount: doc.ifdCount, tagCounts: doc.ifds.map((ifd) => ifd.entries.length), projection: doc } };
  },
  "tiff-compare": async (inputs) => {
    requireInputs(inputs, 2, "tiff-compare");
    const expected = readTiff(inputs[0]!);
    const actual = readTiff(inputs[1]!);
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
