#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.avi@1.0/🎛️hdrl`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual RIFF/AVI decode is performed by the sibling standalone `riff-avi-codec`
// binary (`../🏭️generator/🦀️riff-avi-codec`, depends on nothing but `riff` 2.0) via its `project`
// subcommand — this file only shells out to it, hashes each opaque chunk payload it returns into a
// size+digest pair (per `semantic-avi-v1`'s own "opaque binary payload" treatment, mirroring the
// BCF probe's identical `snapshotDigest` treatment of a PNG viewpoint snapshot), and performs the
// GATING structural comparison itself — no AVI semantics computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts avi-import  --input <a.avi>
//   bun 📜️script.ts avi-project --input <a.avi>
//   bun 📜️script.ts avi-compare --input <expected.avi> --input <actual.avi>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🦀️riff-avi-codec/src/main.rs — the `project` subcommand this file calls

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
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

const ENGINE = { family: "riff", implementation: "riff-avi-codec (riff 2.0 + this artifact's own AVI-1.0 field layout)", version: "riff@2.0.0" } as const;
const PROBE_VERSION = "riff@2.0.0";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️riff-avi-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type StreamFormat =
  | { format: "bitmapInfo"; size: number; width: number; height: number; planes: number; bitCount: number; compression: string; sizeImage: number; xPelsPerMeter: number; yPelsPerMeter: number; colorsUsed: number; colorsImportant: number }
  | { format: "waveFormat"; formatTag: number; channels: number; samplesPerSec: number; avgBytesPerSec: number; blockAlign: number; bitsPerSample: number; extraHex: string }
  | { format: "raw"; dataHex: string };

type StreamHeader = { fccType: string; fccHandler: string; flags: number; priority: number; language: number; initialFrames: number; scale: number; rate: number; start: number; length: number; suggestedBufferSize: number; quality: number; sampleSize: number; rcFrameLeft: number; rcFrameTop: number; rcFrameRight: number; rcFrameBottom: number };

type RawChunk = { fourcc: string; keyframe: boolean; dataHex: string };
type Stream = { strh: StreamHeader; strf: StreamFormat; chunks: RawChunk[] };
type UnknownChunk = { fourcc: string; dataHex: string };
type MainHeader = { microSecPerFrame: number; maxBytesPerSec: number; paddingGranularity: number; flags: number; totalFrames: number; initialFrames: number; streams: number; suggestedBufferSize: number; width: number; height: number; reserved: number[] };

/** 🌳 What `riff-avi-codec project` emits verbatim — raw `dataHex` still present. */
type RawDoc = { mainHeader: MainHeader; idx1Present: boolean; streams: Stream[]; unknownChunks: UnknownChunk[] };

/** ⚖️ The comparisonProfile's own projection: movi/unknown chunk PAYLOADS become size+digest, never
 *  raw bytes — the same opaque-binary-payload treatment the BCF probe gives a PNG snapshot. */
type ProjectedChunk = { fourcc: string; keyframe: boolean; size: number; digest: string };
type ProjectedUnknown = { fourcc: string; size: number; digest: string };
type ProjectedStream = { strh: StreamHeader; strf: Exclude<StreamFormat, { format: "waveFormat" } | { format: "raw" }> | { format: "waveFormat"; formatTag: number; channels: number; samplesPerSec: number; avgBytesPerSec: number; blockAlign: number; bitsPerSample: number; extraSize: number; extraDigest: string } | { format: "raw"; size: number; digest: string }; chunks: ProjectedChunk[] };
type AviDoc = { mainHeader: MainHeader; idx1Present: boolean; streams: ProjectedStream[]; unknownChunks: ProjectedUnknown[] };
//#endregion 📥️Model

//#region 🔓️Read
function digestHex(hex: string): { size: number; digest: string } {
  const bytes = Buffer.from(hex, "hex");
  return { size: bytes.length, digest: `sha256:${createHash("sha256").update(bytes).digest("hex")}` };
}

function projectStrf(strf: StreamFormat): ProjectedStream["strf"] {
  if (strf.format === "bitmapInfo") return strf;
  if (strf.format === "waveFormat") {
    const { extraHex, ...rest } = strf;
    const extra = digestHex(extraHex);
    return { ...rest, extraSize: extra.size, extraDigest: extra.digest };
  }
  const d = digestHex(strf.dataHex);
  return { format: "raw", size: d.size, digest: d.digest };
}

/** 📥️ Runs the standalone codec's `project` subcommand and turns its raw hex payloads into the
 *  profile's own size+digest projection — never raw bytes past this function. */
function readAvi(path: string): AviDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`riff-avi-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  const raw = JSON.parse(result.stdout) as RawDoc;
  return {
    mainHeader: raw.mainHeader,
    idx1Present: raw.idx1Present,
    streams: raw.streams.map((s) => ({ strh: s.strh, strf: projectStrf(s.strf), chunks: s.chunks.map((c) => ({ fourcc: c.fourcc, keyframe: c.keyframe, ...digestHex(c.dataHex) })) })),
    unknownChunks: raw.unknownChunks.map((u) => ({ fourcc: u.fourcc, ...digestHex(u.dataHex) })),
  };
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Positional structural equality — mirrors `semantic-avi-v1`'s own `arrays: "ordered"` rule
 *  (stream index and movi-chunk position are semantic identity in AVI, per that profile's own
 *  description; unlike BCF's guid-keyed set comparison, nothing here is reordered before diffing). */
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

function compareDocs(expected: AviDoc, actual: AviDoc): { equal: boolean; diffCount: number; diffs: string[] } {
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
  "avi-import": async (inputs) => {
    requireInputs(inputs, 1, "avi-import");
    const perInput = inputs.map((input) => {
      try {
        readAvi(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "avi-project": async (inputs) => {
    requireInputs(inputs, 1, "avi-project");
    const doc = readAvi(inputs[0]!);
    return { status: "ok", measurements: { streamCount: doc.streams.length, chunkCount: doc.streams.reduce((total, s) => total + s.chunks.length, 0), unknownChunkCount: doc.unknownChunks.length, projection: doc } };
  },
  "avi-compare": async (inputs) => {
    requireInputs(inputs, 2, "avi-compare");
    const expected = readAvi(inputs[0]!);
    const actual = readAvi(inputs[1]!);
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
