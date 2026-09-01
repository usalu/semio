#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.png@1.2/✳️any`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual PNG decode is performed by the sibling standalone `png-codec` binary
// (`../🏭️generator/🦀️png-codec`, depends on nothing but `png` 0.18.1) via its `project` subcommand
// — this file only shells out to it, hashes the decoded pixel sample buffer it returns into a
// size+digest pair (per `semantic-png-1-2-v1`'s own opaque-binary-payload treatment, the same
// treatment the sibling `avi` probe gives a movi chunk payload), and performs the GATING structural
// comparison itself — no PNG semantics computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts png-import  --input <a.png>
//   bun 📜️script.ts png-project --input <a.png>
//   bun 📜️script.ts png-compare --input <expected.png> --input <actual.png>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the
//      sibling probe suite this file's CLI/dispatch/compare shape is mirrored from
// @see ../🏭️generator/🦀️png-codec/src/main.rs — the `project` subcommand this file calls

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

const ENGINE = { family: "png", implementation: "png-codec (png 0.18.1)", version: "png@0.18.1" } as const;
const PROBE_VERSION = "png@0.18.1";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️png-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type Header = { width: number; height: number; bitDepth: number; colorType: string; interlaced: boolean };
type Chromaticities = { white: [number, number]; red: [number, number]; green: [number, number]; blue: [number, number] };
type PhysicalDims = { xppu: number; yppu: number; unit: string };
type TextChunk = { keyword: string; text: string };

/** 🌳 What `png-codec project` emits verbatim — raw `pixelsHex` still present. */
type RawDoc = {
  header: Header;
  palette: [number, number, number][] | null;
  trns: string | null;
  gamma: number | null;
  chromaticities: Chromaticities | null;
  srgbIntent: string | null;
  physicalDims: PhysicalDims | null;
  background: string | null;
  textChunks: TextChunk[];
  pixelsHex: string;
};

/** ⚖️ The comparisonProfile's own projection: the decoded pixel sample buffer becomes size+digest,
 *  never raw bytes — every other field is small and structured, so it stays a real typed value,
 *  the same split `semantic-avi-v1` makes between a movi chunk payload and a stream header. */
type PngDoc = Omit<RawDoc, "pixelsHex"> & { pixels: { size: number; digest: string } };
//#endregion 📥️Model

//#region 🔓️Read
function digestHex(hex: string): { size: number; digest: string } {
  const bytes = Buffer.from(hex, "hex");
  return { size: bytes.length, digest: `sha256:${createHash("sha256").update(bytes).digest("hex")}` };
}

/** 📥️ Runs the standalone codec's `project` subcommand and turns its raw pixel hex payload into the
 *  profile's own size+digest projection — never raw pixel bytes past this function. */
function readPng(path: string): PngDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`png-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  const raw = JSON.parse(result.stdout) as RawDoc;
  const { pixelsHex, ...rest } = raw;
  return { ...rest, pixels: digestHex(pixelsHex) };
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Structural equality over the whole projected document — every field named above is either a
 *  scalar, a small ordered array (palette entries, tEXt chunks — both meaningfully positional in
 *  PNG: palette index IS the pixel sample for an Indexed image, and tEXt chunk order is file order)
 *  or the pixel buffer's own size+digest pair. */
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

function compareDocs(expected: PngDoc, actual: PngDoc): { equal: boolean; diffCount: number; diffs: string[] } {
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

const CHUNK_ENGINE = { family: "pillow", implementation: "Pillow 11.3.0 PngImagePlugin.ChunkStream (CRC-checked chunk walk)", version: "11.3.0" } as const;
const CHUNK_PROBE_VERSION = "pillow@11.3.0";

/** 🧱️ The CHUNK level, which the `png` crate's decode does not surface.
 *
 *  `png` 0.18 models a decoded image: it has no `tIME` field and skips unrecognised ancillary chunks
 *  (`stream.rs`), so a timestamp change and an unknown-chunk insert or remove are invisible to it —
 *  which is why those three kinds were `-uncarried` against it.
 *
 *  Pillow's `PngImagePlugin.ChunkStream` walks the chunk sequence and validates each CRC, so every
 *  chunk is observable by type, order and payload. Pillow also WRITES both (`PngInfo.add`), so writer
 *  and reader are one library — the precedent gif already sets here. */
const CHUNK_READER = String.raw`
import io, json, sys
from PIL import PngImagePlugin

PNG_SIGNATURE = bytes([137, 80, 78, 71, 13, 10, 26, 10])

def chunks(path):
    data = open(path, 'rb').read()
    if data[:8] != PNG_SIGNATURE:
        raise SystemExit('not a PNG')
    stream = io.BytesIO(data); stream.seek(8)
    walker = PngImagePlugin.ChunkStream(stream)
    out = []
    while True:
        cid, pos, length = walker.read()
        walker.fp.seek(pos)
        payload = walker.fp.read(length)
        walker.crc(cid, payload)
        entry = {"type": cid.decode('ascii', 'replace'), "length": length}
        if entry["type"] != "IDAT":
            entry["payloadHex"] = payload.hex()
        out.append(entry)
        if entry["type"] == "IEND":
            break
    return out

paths = sys.argv[1:]
if len(paths) == 1:
    print(json.dumps(chunks(paths[0])))
else:
    a, b = chunks(paths[0]), chunks(paths[1])
    print(json.dumps({"equal": a == b, "expected": a, "actual": b}))
`;

function chunkRun(paths: readonly string[]): unknown {
  const result = spawnSync("python3", ["-c", CHUNK_READER, ...paths], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`pillow chunk reader failed: ${result.stderr}`);
  return JSON.parse(result.stdout);
}

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "png-chunk-project": async (inputs) => {
    requireInputs(inputs, 1, "png-chunk-project");
    return { status: "ok", engine: CHUNK_ENGINE, probeVersion: CHUNK_PROBE_VERSION, measurements: { chunks: chunkRun([inputs[0]!]) } } as never;
  },
  "png-chunk-compare": async (inputs) => {
    requireInputs(inputs, 2, "png-chunk-compare");
    return { status: "ok", engine: CHUNK_ENGINE, probeVersion: CHUNK_PROBE_VERSION, measurements: chunkRun([inputs[0]!, inputs[1]!]) as Record<string, unknown> } as never;
  },

  "png-import": async (inputs) => {
    requireInputs(inputs, 1, "png-import");
    const perInput = inputs.map((input) => {
      try {
        readPng(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "png-project": async (inputs) => {
    requireInputs(inputs, 1, "png-project");
    const doc = readPng(inputs[0]!);
    return { status: "ok", measurements: { width: doc.header.width, height: doc.header.height, colorType: doc.header.colorType, textChunkCount: doc.textChunks.length, hasPalette: doc.palette !== null, projection: doc } };
  },
  "png-compare": async (inputs) => {
    requireInputs(inputs, 2, "png-compare");
    const expected = readPng(inputs[0]!);
    const actual = readPng(inputs[1]!);
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
