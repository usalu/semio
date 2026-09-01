#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.bmp@v3/✳️any`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual BMP decode is performed by the sibling standalone `image-bmp-codec` binary
// (`../🏭️generator/🦀️image-bmp-codec`, depends on nothing but `image` 0.25's own `bmp` feature)
// via its `project` subcommand — this file only shells out to it, hashes the hex payload it
// returns for the (potentially large) index/pixel buffer into a size+digest pair — the same
// opaque-binary-payload treatment the AVI probe gives a movi chunk's payload — and performs the
// GATING structural comparison itself. No BMP semantics computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts bmp-import  --input <a.bmp>
//   bun 📜️script.ts bmp-project --input <a.bmp>
//   bun 📜️script.ts bmp-compare --input <expected.bmp> --input <actual.bmp>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts —
//      the sibling probe suite this file's CLI/dispatch/compare shape is mirrored from.
// @see ../🏭️generator/🦀️image-bmp-codec/src/main.rs — the `project` subcommand this file calls

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

const ENGINE = { family: "image-rs", implementation: "image-bmp-codec (image 0.25's own bmp feature)", version: "image@0.25.10" } as const;
const PROBE_VERSION = "image@0.25.10";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️image-bmp-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type RawDoc = { format: "bmp"; width: number; height: number; storage: "indexed" | "direct"; palette: { r: number; g: number; b: number }[] | null; indicesHex: string | null; pixelsHex: string | null };

/** ⚖️ The comparisonProfile's own projection: the (potentially large) index/pixel buffer becomes
 *  size+digest, never raw hex — the palette table stays a real typed array so a diff names the
 *  exact entry, per this ticket's own opaque-payload convention. */
type BmpDoc = { format: "bmp"; width: number; height: number; storage: "indexed" | "direct"; palette: { r: number; g: number; b: number }[] | null; indices: { size: number; digest: string } | null; pixels: { size: number; digest: string } | null };
//#endregion 📥️Model

//#region 🔓️Read
function digestHex(hex: string): { size: number; digest: string } {
  const bytes = Buffer.from(hex, "hex");
  return { size: bytes.length, digest: `sha256:${createHash("sha256").update(bytes).digest("hex")}` };
}

/** 📥️ Runs the standalone codec's `project` subcommand and turns its raw hex payload into the
 *  profile's own size+digest projection — never raw bytes past this function. */
function readBmp(path: string): BmpDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`image-bmp-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  const raw = JSON.parse(result.stdout) as RawDoc;
  return {
    format: raw.format,
    width: raw.width,
    height: raw.height,
    storage: raw.storage,
    palette: raw.palette,
    indices: raw.indicesHex === null ? null : digestHex(raw.indicesHex),
    pixels: raw.pixelsHex === null ? null : digestHex(raw.pixelsHex),
  };
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Structural equality over the whole projected document — mirrors `semantic-bmp-reader-v1`'s
 *  own rule that a palette is an ORDERED array (palette-entry position is semantic identity for
 *  `insert-palette-entry`/`remove-palette-entry`, which shift every entry after the touched index). */
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

function compareDocs(expected: BmpDoc, actual: BmpDoc): { equal: boolean; diffCount: number; diffs: string[] } {
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

const HEADER_ENGINE = { family: "pillow", implementation: "Pillow 11.3.0 BMP BITMAPINFOHEADER reader", version: "11.3.0" } as const;
const HEADER_PROBE_VERSION = "pillow@11.3.0";

/** 🧾️ The BITMAPINFOHEADER fields. `image`-rs decodes a BMP to PIXELS: the header's resolution fields
 *  and colour-table bookkeeping are consumed on the way and never surfaced, which is why
 *  `change-header-fields` was `-uncarried` against it. Pillow both WRITES them (`dpi=` sets
 *  biXPelsPerMeter / biYPelsPerMeter) and READS them back (`im.info['dpi']`). */
const HEADER_READER = String.raw`
import json, struct, sys
from PIL import Image

def project(path):
    raw = open(path, 'rb').read()
    # 📐️BITMAPINFOHEADER begins at byte 14; these ten fields are its whole fixed layout.
    width, height, planes, bpp, compression, image_size, xppm, yppm, used, important = struct.unpack('<iihhIIiiII', raw[18:54])
    im = Image.open(path)
    return {
        "width": width, "height": height, "planes": planes, "bitsPerPixel": bpp,
        "compression": compression, "imageSize": image_size,
        "xPixelsPerMetre": xppm, "yPixelsPerMetre": yppm,
        "coloursUsed": used, "coloursImportant": important,
        "mode": im.mode, "size": list(im.size),
    }

paths = sys.argv[1:]
if len(paths) == 1:
    print(json.dumps(project(paths[0]), sort_keys=True))
else:
    a, b = project(paths[0]), project(paths[1])
    print(json.dumps({"equal": a == b, "expected": a, "actual": b}, sort_keys=True))
`;

function headerRun(paths: readonly string[]): Record<string, unknown> {
  const result = spawnSync("python3", ["-c", HEADER_READER, ...paths], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`pillow bmp header reader failed: ${result.stderr}`);
  return JSON.parse(result.stdout) as Record<string, unknown>;
}

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "bmp-header-project": async (inputs) => {
    requireInputs(inputs, 1, "bmp-header-project");
    return { status: "ok", engine: HEADER_ENGINE, probeVersion: HEADER_PROBE_VERSION, measurements: headerRun([inputs[0]!]) } as never;
  },
  "bmp-header-compare": async (inputs) => {
    requireInputs(inputs, 2, "bmp-header-compare");
    return { status: "ok", engine: HEADER_ENGINE, probeVersion: HEADER_PROBE_VERSION, measurements: headerRun([inputs[0]!, inputs[1]!]) } as never;
  },

  "bmp-import": async (inputs) => {
    requireInputs(inputs, 1, "bmp-import");
    const perInput = inputs.map((input) => {
      try {
        readBmp(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "bmp-project": async (inputs) => {
    requireInputs(inputs, 1, "bmp-project");
    const doc = readBmp(inputs[0]!);
    return { status: "ok", measurements: { storage: doc.storage, width: doc.width, height: doc.height, paletteEntries: doc.palette?.length ?? 0, projection: doc } };
  },
  "bmp-compare": async (inputs) => {
    requireInputs(inputs, 2, "bmp-compare");
    const expected = readBmp(inputs[0]!);
    const actual = readBmp(inputs[1]!);
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
