#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.step@ap214/✳️cc6` (advanced B-Rep).
//
// Everything here MARSHALS and INVOKES; nothing here computes geometry. Every number this file emits
// comes out of `brepjs`'s OpenCASCADE kernel — reading a STEP file, classifying a shape, measuring an
// exact volume or tessellating at a declared tolerance. The comparison pipeline evaluates the emitted
// `measurements` against its declared assertions and performs no arithmetic of its own beyond
// comparison, which is what keeps the reference genuinely external.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts step-import       --input <a.step> [--input <b.step>]
//   bun 📜️script.ts brep-validity     --input <a.step>
//   bun 📜️script.ts measure           --input <a.step>
//   bun 📜️script.ts topology          --input <a.step>
//   bun 📜️script.ts reimport-compare  --input <expected.step> --input <actual.step>
//   bun 📜️script.ts tessellate        --input <a.step> --tolerance 1e-3 --out <mesh.json>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️component.json — ProbeReport
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️w4-brepjs-qualification.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  seed?: string | number;
  durationMs: number;
  measurements: Record<string, unknown>;
  outputs?: { role: string; path: string; mediaType: string; sha256: string }[];
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

/** ⚙️ The engine family independence is accounted in. Two OCCT wrappers are ONE family, not two. */
const ENGINE = { family: "opencascade", implementation: "brepjs-opencascade wasm", version: "0.15.6" } as const;
const PROBE_VERSION = "brepjs@18.119.8";
//#endregion 🧬️Contract

//#region 🧰️Kernel
type Kernel = Record<string, (...args: unknown[]) => unknown>;

let kernel: Kernel | null = null;

/** ⚙️ Loads and initializes the OCCT WASM kernel once per process. Never reaches the network. */
async function brep(): Promise<Kernel> {
  if (kernel !== null) return kernel;
  const loaded = (await import("brepjs")) as unknown as Kernel;
  await (loaded.init as () => Promise<void>)();
  kernel = loaded;
  return loaded;
}

/**
 * 📦️ brepjs returns a `Result`-shaped `{ok, value}` from most entry points. Unwrapping HERE — once,
 * loudly — is the difference between a probe that reports `failed` with the kernel's own error and
 * one that silently measures an error object as if it were a shape.
 */
function unwrap(value: unknown, what: string): unknown {
  if (value !== null && typeof value === "object" && "ok" in (value as Record<string, unknown>)) {
    const result = value as { ok: boolean; value?: unknown; error?: unknown };
    if (!result.ok) throw new Error(`${what}: ${JSON.stringify(result.error)}`);
    return result.value;
  }
  return value;
}

/** 📥️ Imports one STEP file through the external reader. `importSTEP` takes the Blob `exportSTEP` emits. */
async function importStep(absPath: string): Promise<unknown> {
  const b = await brep();
  const text = readFileSync(absPath, "utf8");
  const imported = unwrap(await (b.importSTEP as (blob: Blob) => unknown)(new Blob([text])), `importSTEP ${absPath}`);
  const resolved = imported instanceof Promise ? unwrap(await imported, `importSTEP await ${absPath}`) : imported;
  if (Array.isArray(resolved)) return resolved[0];
  const record = resolved as { shape?: unknown };
  return record.shape ?? resolved;
}
//#endregion 🧰️Kernel

//#region #⃣Digest
async function contentDigest(absPath: string): Promise<string> {
  const bytes = readFileSync(absPath);
  const hash = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${[...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion #⃣Digest

//#region 🔬️Probes
/** 📐️ Exact-shape measurements of one solid, straight out of the kernel. */
async function measureShape(shape: unknown): Promise<Record<string, number>> {
  const b = await brep();
  const bounds = unwrap((b.getBounds as (s: unknown) => unknown)(shape), "getBounds") as Record<string, number>;
  const diagonal = Math.hypot(bounds.xMax! - bounds.xMin!, bounds.yMax! - bounds.yMin!, bounds.zMax! - bounds.zMin!);
  return {
    volume: unwrap((b.measureVolume as (s: unknown) => unknown)(shape), "measureVolume") as number,
    area: unwrap((b.measureArea as (s: unknown) => unknown)(shape), "measureArea") as number,
    boundingBoxDiagonal: diagonal,
    xMin: bounds.xMin!,
    xMax: bounds.xMax!,
    yMin: bounds.yMin!,
    yMax: bounds.yMax!,
    zMin: bounds.zMin!,
    zMax: bounds.zMax!,
  };
}

/** 🔢️ Topology counts. Asserted only where a mutation's semantics make them normative. */
async function topologyOf(shape: unknown): Promise<Record<string, number>> {
  const b = await brep();
  return {
    solids: ((b.getSolids as (s: unknown) => unknown[])(shape) ?? []).length,
    shells: ((b.getShells as (s: unknown) => unknown[])(shape) ?? []).length,
    faces: ((b.getFaces as (s: unknown) => unknown[])(shape) ?? []).length,
    edges: ((b.getEdges as (s: unknown) => unknown[])(shape) ?? []).length,
    vertices: ((b.getVertices as (s: unknown) => unknown[])(shape) ?? []).length,
  };
}

const PROBES: Record<string, (inputs: string[], options: Record<string, string>) => Promise<Omit<ProbeReport, "schema" | "probe" | "probeVersion" | "engine" | "durationMs">>> = {
  /** 📥️ Does an INDEPENDENT reader accept both files at all? Nothing downstream means anything if not. */
  "step-import": async (inputs) => {
    const outcomes = await Promise.all(inputs.map(async (input) => ({ input, ok: await importStep(input).then(() => true).catch(() => false) })));
    return { status: "ok", measurements: { bothImport: outcomes.every((entry) => entry.ok), imported: outcomes.filter((entry) => entry.ok).length, inputs: outcomes.length, perInput: outcomes } };
  },

  /** ✅️ Is the imported shape a valid solid? A mesh that looks right can hide an invalid seam. */
  "brep-validity": async (inputs) => {
    const b = await brep();
    const outcomes = await Promise.all(
      inputs.map(async (input) => {
        const shape = await importStep(input);
        return { input, valid: (b.isValidSolid as (s: unknown) => boolean)(shape) === true, solids: ((b.getSolids as (s: unknown) => unknown[])(shape) ?? []).length };
      }),
    );
    return { status: "ok", measurements: { bothValid: outcomes.every((entry) => entry.valid), allValid: outcomes.every((entry) => entry.valid), perInput: outcomes } };
  },

  /** 📐️ Exact volume, area and bounding box of each input, measured by the kernel on the EXACT shape. */
  measure: async (inputs) => {
    const outcomes = await Promise.all(inputs.map(async (input) => ({ input, ...(await measureShape(await importStep(input))) })));
    return { status: "ok", measurements: { perInput: outcomes, ...(outcomes.length === 1 ? outcomes[0] : {}) } };
  },

  /** 🔢️ Topology counts of each input. */
  topology: async (inputs) => {
    const outcomes = await Promise.all(inputs.map(async (input) => ({ input, ...(await topologyOf(await importStep(input))) })));
    return { status: "ok", measurements: { perInput: outcomes, ...(outcomes.length === 1 ? outcomes[0] : {}) } };
  },

  /**
   * ⚖️ The operative BRep gate while no external STEP canonicalizer is qualified: reimport BOTH files
   * through the same independent reader and compare what the kernel measures on the EXACT shapes.
   * Byte equality is deliberately not attempted — the qualification spike measured OCCT stamping an
   * incrementing translator counter and a wall-clock timestamp into every export, so even a single
   * writer is not self-deterministic at the byte level.
   */
  "reimport-compare": async (inputs) => {
    if (inputs.length !== 2) return { status: "failed", measurements: {}, diagnostics: [{ severity: "error", message: `reimport-compare needs exactly two inputs, got ${inputs.length}` }] };
    const [expectedPath, actualPath] = inputs as [string, string];
    const expected = await importStep(expectedPath);
    const actual = await importStep(actualPath);
    const expectedMetrics = await measureShape(expected);
    const actualMetrics = await measureShape(actual);
    const expectedTopology = await topologyOf(expected);
    const actualTopology = await topologyOf(actual);
    const reference = Math.max(expectedMetrics.boundingBoxDiagonal!, Number.EPSILON);
    const relative = (a: number, b: number): number => (Math.abs(b) < Number.EPSILON ? Math.abs(a - b) : Math.abs(a - b) / Math.abs(b));
    const centroidDistance = Math.hypot(
      (actualMetrics.xMin! + actualMetrics.xMax!) / 2 - (expectedMetrics.xMin! + expectedMetrics.xMax!) / 2,
      (actualMetrics.yMin! + actualMetrics.yMax!) / 2 - (expectedMetrics.yMin! + expectedMetrics.yMax!) / 2,
      (actualMetrics.zMin! + actualMetrics.zMax!) / 2 - (expectedMetrics.zMin! + expectedMetrics.zMax!) / 2,
    );
    return {
      status: "ok",
      measurements: {
        bothImport: true,
        bothValid: expectedTopology.solids! > 0 && actualTopology.solids! > 0,
        relativeVolumeError: relative(actualMetrics.volume!, expectedMetrics.volume!),
        relativeAreaError: relative(actualMetrics.area!, expectedMetrics.area!),
        normalizedCentroidDistance: centroidDistance / reference,
        normalizedBoundingBoxDiagonalError: Math.abs(actualMetrics.boundingBoxDiagonal! - expectedMetrics.boundingBoxDiagonal!) / reference,
        connectedComponentsEqual: expectedTopology.solids === actualTopology.solids,
        referenceScale: reference,
        expected: { ...expectedMetrics, ...expectedTopology },
        actual: { ...actualMetrics, ...actualTopology },
      },
    };
  },

  /** 🔺️ Tessellates at a DECLARED tolerance and writes an indexed mesh beside the report. */
  tessellate: async (inputs, options) => {
    const b = await brep();
    const tolerance = Number(options.tolerance ?? "1e-3");
    const angular = Number(options.angularTolerance ?? "0.1");
    const outputs: ProbeReport["outputs"] = [];
    const perInput: Record<string, unknown>[] = [];
    for (const [index, input] of inputs.entries()) {
      const shape = await importStep(input);
      const meshed = unwrap((b.mesh as (s: unknown, o: unknown) => unknown)(shape, { tolerance, angularTolerance: angular }), `mesh ${input}`) as Record<string, ArrayLike<number>>;
      const vertices = meshed.vertices ?? meshed.positions!;
      const triangles = meshed.triangles ?? meshed.indices!;
      perInput.push({ input, vertexCount: vertices.length / 3, triangleCount: triangles.length / 3, tolerance, angularTolerance: angular });
      const out = options.out;
      if (out !== undefined) {
        const path = inputs.length === 1 ? out : out.replace(/(\.[^.]+)?$/, `.${index}$1`);
        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, `${JSON.stringify({ vertices: Array.from(vertices), triangles: Array.from(triangles), tolerance, angularTolerance: angular })}\n`);
        outputs.push({ role: `mesh-${index}`, path, mediaType: "application/json", sha256: await contentDigest(path) });
      }
    }
    return { status: "ok", measurements: { perInput, ...(perInput.length === 1 ? perInput[0]! : {}) }, outputs };
  },
};
//#endregion 🔬️Probes

//#region 🚪️Entry
/** 🎛️ Parses `--flag value` pairs, collecting every `--input` rather than keeping only the last. */
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[]; options: Record<string, string> } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  const options: Record<string, string> = {};
  for (let index = 0; index < rest.length; index += 1) {
    const token = rest[index]!;
    if (!token.startsWith("--")) continue;
    const value = rest[index + 1] ?? "";
    if (token === "--input") inputs.push(value);
    else options[token.slice(2)] = value;
    index += 1;
  }
  return { probe, inputs, options };
}

async function main(argv: readonly string[]): Promise<number> {
  const { probe, inputs, options } = parseArgv(argv);
  const started = Date.now();
  const handler = PROBES[probe];
  const emit = (report: ProbeReport): void => {
    if (options.report !== undefined) {
      mkdirSync(dirname(options.report), { recursive: true });
      writeFileSync(options.report, `${JSON.stringify(report, null, 2)}\n`);
    }
    process.stdout.write(`${JSON.stringify(report)}\n`);
  };
  const base = { schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE } as const;
  if (handler === undefined) {
    emit({ ...base, status: "unsupported", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${JSON.stringify(probe)} — expected one of ${Object.keys(PROBES).join(", ")}` }] });
    return 2;
  }
  try {
    const outcome = await handler(inputs, options);
    emit({ ...base, ...outcome, durationMs: Date.now() - started });
    return outcome.status === "ok" ? 0 : 1;
  } catch (error) {
    // 🔬️A probe that cannot measure reports `failed` WITH the kernel's own message. Silence here would
    // let an unmeasured assertion read as green, which is the failure mode the pipeline exists to stop.
    emit({ ...base, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: (error as Error).message, detail: (error as Error).stack ?? "" }] });
    return 1;
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
