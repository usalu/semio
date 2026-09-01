#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.jpg@jfif-1.01/✳️document`'s reader oracle.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual JPEG decode is performed by the sibling standalone `jpeg-jfif-codec` binary
// (`../🏭️generator/🦀️jpeg-jfif-codec`, depends on nothing but `image` 0.25) via its `project`
// subcommand — this file only shells out to it and performs the GATING structural comparison
// itself. No JPEG semantics are computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts jpg-import  --input <a.jpg>
//   bun 📜️script.ts jpg-project --input <a.jpg>
//   bun 📜️script.ts jpg-compare --input <expected.jpg> --input <actual.jpg>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling probe
//      suite this file's CLI/dispatch/compare shape is mirrored from.
// @see ../🏭️generator/🦀️jpeg-jfif-codec/src/main.rs — the `project` subcommand this file calls,
//      and its own module docstring recording exactly what `image` 0.25.10 can and cannot see.

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

const ENGINE = { family: "image-rs", implementation: "jpeg-jfif-codec (image 0.25.10, zune-jpeg 0.5.15 backend)", version: "image@0.25.10" } as const;
const PROBE_VERSION = "image@0.25.10";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️jpeg-jfif-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type OpaqueSegment = { present: boolean; size?: number; digest?: string };
type JpgDoc = { dimensions: string; colorType: string; raster: { size: number; digest: string }; xmp: OpaqueSegment; exif: OpaqueSegment; iptc: OpaqueSegment; iccProfile: OpaqueSegment };
//#endregion 📥️Model

//#region 🔓️Read
/** 📥️ Runs the standalone codec's `project` subcommand — every field it returns is ALREADY the
 *  profile's own opaque-payload projection (size+digest, never raw bytes; see that binary's own
 *  `project()` function), so nothing further happens to it here. */
function readJpg(path: string): JpgDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`jpeg-jfif-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  return JSON.parse(result.stdout) as JpgDoc;
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Structural equality over the whole projection — every field is either a small typed value
 *  (dimensions, colorType) or already an opaque size+digest pair, so a plain recursive diff is the
 *  operative equality; nothing here is reordered or tolerance-adjusted. */
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

function compareDocs(expected: JpgDoc, actual: JpgDoc): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$", expected, actual, diffs);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] ; engine?: ProbeReport["engine"]; probeVersion?: string };

const MARKER_ENGINE = { family: "pillow", implementation: "Pillow 11.3.0 JPEG marker reader (quantization tables, JFIF APP0)", version: "11.3.0" } as const;
const MARKER_PROBE_VERSION = "pillow@11.3.0";

/** 🐍️ Pillow reads back the MARKER-level facts `image`-rs does not surface. `image` decodes a JPEG to
 *  pixels — quantisation tables and the JFIF APP0 segment are consumed and discarded on the way, which
 *  is why those kinds were `-uncarried` against it. Pillow keeps both: `im.quantization` and
 *  `im.info['jfif_*']`.
 *
 *  Measured before registering: `replace-quant-table` and `change-jfif-header` each move this
 *  projection. `change-restart-interval` does NOT (Pillow does not read the DRI segment back) and the
 *  Huffman accessors return empty and are deprecated for removal in Pillow 12 — so those kinds stay
 *  `-uncarried` rather than being claimed. */
const MARKER_READER = String.raw`
import io, json, sys
from PIL import Image
def project(path):
    im = Image.open(path); im.load()
    info = im.info
    return {
        "quantTables": {str(k): list(v) for k, v in (im.quantization or {}).items()},
        "jfifVersion": list(info["jfif_version"]) if info.get("jfif_version") else None,
        "jfifUnit": info.get("jfif_unit"),
        "jfifDensity": list(info["jfif_density"]) if info.get("jfif_density") else None,
        "size": list(im.size),
        "mode": im.mode,
    }
paths = sys.argv[1:]
if len(paths) == 1:
    print(json.dumps(project(paths[0]), sort_keys=True))
else:
    a, b = project(paths[0]), project(paths[1])
    print(json.dumps({"equal": a == b, "expected": a, "actual": b}, sort_keys=True))
`;

function markerRun(paths: readonly string[]): Record<string, unknown> {
  const result = spawnSync("python3", ["-c", MARKER_READER, ...paths], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`pillow marker reader failed: ${result.stderr}`);
  return JSON.parse(result.stdout) as Record<string, unknown>;
}

const LIBJPEG_ENGINE = { family: "libjpeg-turbo", implementation: "djpeg -v -v marker dump (libjpeg-turbo 3.2.0)", version: "3.2.0" } as const;
const LIBJPEG_PROBE_VERSION = "libjpeg-turbo@3.2.0";

/** 🏷️ The MARKER STRUCTURE, as libjpeg's own decoder reports it.
 *
 *  Neither of this subset's other readers can reach it. `image`-rs decodes to pixels. Pillow's DRI
 *  handler is literally `Skip` and its Huffman accessors return empty and are deprecated for removal in
 *  Pillow 12. `zune-jpeg` parses the DRI marker but keeps `restart_interval` `pub(crate)`.
 *
 *  `djpeg -v -v` prints every marker it walks — quantisation tables with their values, Huffman tables
 *  with their code-length counts, the Start-of-Frame, and `Define Restart Interval N`. That is a
 *  THIRD-PARTY CLI, which Protocol v2 lists as a qualifying oracle kind alongside third-party-library.
 *
 *  The banner lines are dropped: they carry the tool's build date, which would make the projection
 *  differ between machines for reasons that have nothing to do with the fixture. */
const LIBJPEG_BANNER = /^(libjpeg-turbo version|Copyright|Emulating)/;

function libjpegProjection(absPath: string): Record<string, unknown> {
  const result = spawnSync("djpeg", ["-v", "-v", "-outfile", "/dev/null", absPath], { encoding: "utf8" });
  // 🧭️djpeg writes its marker dump to STDERR and the decoded image to the outfile; a non-zero status
  // means it refused the file, which is a real answer and not a probe failure to swallow.
  if (result.status !== 0) throw new Error(`djpeg refused ${absPath}: ${result.stderr}`);
  const lines = String(result.stderr).split("\n").map((line) => line.trimEnd()).filter((line) => line.length > 0 && !LIBJPEG_BANNER.test(line));
  const markers = lines.filter((line) => !line.startsWith(" "));
  const restart = lines.find((line) => line.startsWith("Define Restart Interval"));
  return {
    markerDump: lines,
    markers,
    huffmanTables: markers.filter((line) => line.startsWith("Define Huffman Table")).length,
    quantTables: markers.filter((line) => line.startsWith("Define Quantization Table")).length,
    restartInterval: restart ? Number(restart.replace(/\D+/g, "")) : null,
  };
}

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "jpg-libjpeg-project": async (inputs) => {
    requireInputs(inputs, 1, "jpg-libjpeg-project");
    return { status: "ok", engine: LIBJPEG_ENGINE, probeVersion: LIBJPEG_PROBE_VERSION, measurements: libjpegProjection(inputs[0]!) } as never;
  },
  "jpg-libjpeg-compare": async (inputs) => {
    requireInputs(inputs, 2, "jpg-libjpeg-compare");
    const expected = libjpegProjection(inputs[0]!);
    const actual = libjpegProjection(inputs[1]!);
    return { status: "ok", engine: LIBJPEG_ENGINE, probeVersion: LIBJPEG_PROBE_VERSION, measurements: { equal: JSON.stringify(expected) === JSON.stringify(actual), expected, actual } } as never;
  },

  "jpg-marker-project": async (inputs) => {
    requireInputs(inputs, 1, "jpg-marker-project");
    return { status: "ok", engine: MARKER_ENGINE, probeVersion: MARKER_PROBE_VERSION, measurements: markerRun([inputs[0]!]) } as never;
  },
  "jpg-marker-compare": async (inputs) => {
    requireInputs(inputs, 2, "jpg-marker-compare");
    return { status: "ok", engine: MARKER_ENGINE, probeVersion: MARKER_PROBE_VERSION, measurements: markerRun([inputs[0]!, inputs[1]!]) } as never;
  },

  "jpg-import": async (inputs) => {
    requireInputs(inputs, 1, "jpg-import");
    const perInput = inputs.map((input) => {
      try {
        readJpg(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "jpg-project": async (inputs) => {
    requireInputs(inputs, 1, "jpg-project");
    const doc = readJpg(inputs[0]!);
    return { status: "ok", measurements: { projection: doc } };
  },
  "jpg-compare": async (inputs) => {
    requireInputs(inputs, 2, "jpg-compare");
    const expected = readJpg(inputs[0]!);
    const actual = readJpg(inputs[1]!);
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
