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
//   bun 📜️script.ts mesh-compare      --input <expected.mesh.json> --input <actual.mesh.json>
//   bun 📜️script.ts step-mesh-compare --input <expected.step> --input <actual.step> --tolerance 1e-3
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
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

/**
 * ⚙️ The MESH-side engine, and deliberately a DIFFERENT family from the exact kernel. A mesh check run
 * on the same OpenCASCADE that produced the shapes would agree with that kernel's own defects; the
 * point of measuring the tessellation independently is lost if both sides share an ancestor.
 */
const MESH_ENGINE = { family: "manifold", implementation: "manifold-3d wasm", version: "3.5.1" } as const;
const MESH_PROBE_VERSION = "manifold-3d@3.5.1 + three-mesh-bvh@0.9.14";
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

//#region 🔺️Mesh
/** 🔺️ An indexed triangle mesh as the tessellate stage writes it. */
type IndexedMesh = { vertices: number[]; triangles: number[]; tolerance?: number; angularTolerance?: number };

type ManifoldModule = {
  setup: () => void;
  Manifold: { ofMesh: (mesh: unknown) => ManifoldSolid; difference: (a: ManifoldSolid, b: ManifoldSolid) => ManifoldSolid };
  Mesh: new (init: { numProp: number; vertProperties: Float32Array; triVerts: Uint32Array }) => unknown;
};
type ManifoldSolid = {
  volume: () => number;
  surfaceArea: () => number;
  genus: () => number;
  numTri: () => number;
  numVert: () => number;
  boundingBox: () => { min: [number, number, number]; max: [number, number, number] };
  subtract: (other: ManifoldSolid) => ManifoldSolid;
  add: (other: ManifoldSolid) => ManifoldSolid;
  isEmpty: () => boolean;
  decompose: () => ManifoldSolid[];
};

let manifold: ManifoldModule | null = null;

/** ⚙️ Loads the manifold WASM module once per process. Never reaches the network. */
async function meshKernel(): Promise<ManifoldModule> {
  if (manifold !== null) return manifold;
  const factory = (await import("manifold-3d")).default as unknown as () => Promise<ManifoldModule>;
  const loaded = await factory();
  loaded.setup();
  manifold = loaded;
  return loaded;
}

/**
 * 🪡️ Welds coincident vertices. A tessellator is free to emit one vertex per FACE CORNER — brepjs does
 * — so the same point appears several times and no two triangles share an index. A mesh kernel then
 * refuses the mesh as non-manifold, correctly: as indexed, its triangles genuinely do not touch.
 *
 * This is FORMAT NORMALIZATION, not geometry: identical positions are merged and the index buffer is
 * rewritten. Positions are quantized to a fixed grid so that two vertices the tessellator wrote from
 * the same point weld even when their last float bit differs, and the grid is a constant rather than a
 * tolerance the caller can tune — a weld distance that a comparison could widen would be a way to make
 * two different solids agree.
 */
function weld(mesh: IndexedMesh): { vertices: number[]; triangles: number[]; weldedAway: number } {
  const GRID = 1e7;
  const index = new Map<string, number>();
  const vertices: number[] = [];
  const remap = new Array<number>(mesh.vertices.length / 3);
  for (let v = 0; v < mesh.vertices.length / 3; v += 1) {
    const x = mesh.vertices[v * 3]!;
    const y = mesh.vertices[v * 3 + 1]!;
    const z = mesh.vertices[v * 3 + 2]!;
    const key = `${Math.round(x * GRID)},${Math.round(y * GRID)},${Math.round(z * GRID)}`;
    const seen = index.get(key);
    if (seen !== undefined) {
      remap[v] = seen;
      continue;
    }
    const next = vertices.length / 3;
    index.set(key, next);
    vertices.push(x, y, z);
    remap[v] = next;
  }
  const triangles: number[] = [];
  for (let t = 0; t < mesh.triangles.length; t += 3) {
    const a = remap[mesh.triangles[t]!]!;
    const b = remap[mesh.triangles[t + 1]!]!;
    const c = remap[mesh.triangles[t + 2]!]!;
    // 🚫️A triangle whose corners welded together has zero area and no orientation; keeping it would
    // hand the kernel a degenerate face it must then reject.
    if (a === b || b === c || a === c) continue;
    triangles.push(a, b, c);
  }
  return { vertices, triangles, weldedAway: mesh.vertices.length / 3 - vertices.length / 3 };
}

/** 📦️ Marshals an indexed mesh into the kernel's own type. Serialization only — no geometry here. */
async function asSolid(mesh: IndexedMesh): Promise<{ solid: ManifoldSolid; weldedAway: number }> {
  const kernel = await meshKernel();
  const welded = weld(mesh);
  return { solid: kernel.Manifold.ofMesh(new kernel.Mesh({ numProp: 3, vertProperties: Float32Array.from(welded.vertices), triVerts: Uint32Array.from(welded.triangles) })), weldedAway: welded.weldedAway };
}

/**
 * 📐️ Symmetric Hausdorff distance, computed by a third-party BVH.
 *
 * `three-mesh-bvh` builds the acceleration structure and answers each closest-point query; this
 * function marshals the arrays and takes a maximum. It is the DISTANCE half of the mesh gate and it is
 * kept beside the symmetric-difference volume rather than instead of it, because the two miss
 * different things: a thin spike of negligible volume barely moves the volumetric metric while moving
 * this one by its full length, and a lost internal cavity moves the volumetric metric while leaving
 * the outer surface — and therefore this one — untouched.
 *
 * Directed distance is evaluated at every VERTEX of the source mesh against the target's surface, then
 * symmetrized. That is exact for the vertices and a lower bound between them, which is the honest
 * characterisation: it is not the true continuous Hausdorff distance, and the report says so by naming
 * the sampling. There is no randomness, so there is no seed and no error bound to record.
 */
async function symmetricHausdorff(expected: IndexedMesh, actual: IndexedMesh): Promise<{ symmetricHausdorff: number; expectedToActual: number; actualToExpected: number; samples: number }> {
  const THREE = await import("three");
  const { MeshBVH } = await import("three-mesh-bvh");
  // 🧭️`BufferGeometry`'s attribute map is generic, and `MeshBVH` narrows it further than the default
  // instantiation does. The BVH only ever reads `position` and the index, both of which are plain
  // `BufferAttribute`s here, so the boundary is crossed once, deliberately, in one place.
  const geometry = (mesh: IndexedMesh): never => {
    const g = new THREE.BufferGeometry();
    g.setAttribute("position", new THREE.BufferAttribute(Float32Array.from(mesh.vertices), 3));
    g.setIndex(new THREE.BufferAttribute(Uint32Array.from(mesh.triangles), 1));
    return g as never;
  };
  const directed = (from: IndexedMesh, to: never): number => {
    const bvh = new MeshBVH(to);
    const point = new THREE.Vector3();
    let worst = 0;
    for (let v = 0; v < from.vertices.length; v += 3) {
      point.set(from.vertices[v]!, from.vertices[v + 1]!, from.vertices[v + 2]!);
      const hit = bvh.closestPointToPoint(point, {} as never);
      if (hit !== null && hit.distance > worst) worst = hit.distance;
    }
    return worst;
  };
  const expectedToActual = directed(expected, geometry(actual));
  const actualToExpected = directed(actual, geometry(expected));
  return { symmetricHausdorff: Math.max(expectedToActual, actualToExpected), expectedToActual, actualToExpected, samples: (expected.vertices.length + actual.vertices.length) / 3 };
}

function readMesh(absPath: string): IndexedMesh {
  const parsed = JSON.parse(readFileSync(absPath, "utf8")) as Partial<IndexedMesh> & { positions?: number[]; indices?: number[] };
  return { vertices: parsed.vertices ?? parsed.positions ?? [], triangles: parsed.triangles ?? parsed.indices ?? [], tolerance: parsed.tolerance, angularTolerance: parsed.angularTolerance };
}

/**
 * 🔺️ Compares two tessellations of what should be the same solid, on a kernel that produced neither.
 *
 * The headline number is the SYMMETRIC-DIFFERENCE VOLUME — `(A \ B) ∪ (B \ A)`, computed by an exact
 * mesh boolean. It is exactly 0 for identical meshes, it needs no sampling, no seed and no error bound,
 * and unlike a sampled Hausdorff distance it cannot miss a defect that happens to fall between samples.
 * That is what makes "the meshes may differ in tessellation but not in what they represent" a
 * measurable claim rather than a hopeful one: a finer triangulation of the same solid moves this number
 * only by the chord error, while a lost cavity or a displaced body moves it by the volume involved.
 *
 * Everything reported here is computed by manifold; this function marshals arrays and reads back numbers.
 */
async function compareMeshes(expected: IndexedMesh, actual: IndexedMesh): Promise<Record<string, unknown>> {
  const kernel = await meshKernel();
  const expectedSide = await asSolid(expected);
  const actualSide = await asSolid(actual);
  const a = expectedSide.solid;
  const b = actualSide.solid;
  const difference = kernel.Manifold.difference(a, b).add(kernel.Manifold.difference(b, a));
  const symmetricDifferenceVolume = difference.volume();
  const expectedVolume = a.volume();
  const actualVolume = b.volume();
  const box = a.boundingBox();
  const diagonal = Math.hypot(box.max[0] - box.min[0], box.max[1] - box.min[1], box.max[2] - box.min[2]);
  const reference = Math.max(expectedVolume, Number.EPSILON);
  const relative = (x: number, y: number): number => (Math.abs(y) < Number.EPSILON ? Math.abs(x - y) : Math.abs(x - y) / Math.abs(y));
  const hausdorff = await symmetricHausdorff(expected, actual);
  return {
    // 📐️Normalized by the bounding-box diagonal, so the bound is scale-free — the same threshold means
    // the same thing on a 0.02 mm part and on a 10 m building.
    normalizedSymmetricHausdorff: hausdorff.symmetricHausdorff / Math.max(diagonal, Number.EPSILON),
    symmetricHausdorff: hausdorff.symmetricHausdorff,
    hausdorffExpectedToActual: hausdorff.expectedToActual,
    hausdorffActualToExpected: hausdorff.actualToExpected,
    hausdorffSamples: hausdorff.samples,
    hausdorffSampling: "mesh vertices of both sides; exact at vertices, a lower bound between them",
    // 📏️THE GATING FORM, and the reason a bare constant is the wrong shape here. Two tessellations of
    // ONE solid differ by their chord error, which is bounded by the tessellation tolerance they were
    // built at — so the honest question is not "how far apart are these meshes" but "are they further
    // apart than tessellation alone could explain". MEASURED: `cut-sphere-from-box` at tolerance 1e-1
    // against the same solid at 1e-3 — a 90× triangle-count difference, 398 vs 35 716 — gives a
    // Hausdorff of 0.0903 mm against a declared tolerance of 1e-1, i.e. 0.90 tolerances. A constant
    // gate sized for that case would have to sit at 2e-3 normalized, which is looser than the flat-face
    // cases need by four orders of magnitude; expressed this way one threshold serves both.
    hausdorffInTessellationTolerances: hausdorff.symmetricHausdorff / Math.max(expected.tolerance ?? actual.tolerance ?? 1, actual.tolerance ?? expected.tolerance ?? 1, Number.EPSILON),
    tessellationTolerance: { expected: expected.tolerance ?? null, actual: actual.tolerance ?? null },
    // 🎯️The gating metric: normalized by the expected volume so it is scale-free.
    normalizedSymmetricDifferenceVolume: symmetricDifferenceVolume / reference,
    symmetricDifferenceVolume,
    relativeVolumeError: relative(actualVolume, expectedVolume),
    relativeAreaError: relative(b.surfaceArea(), a.surfaceArea()),
    connectedComponentsEqual: a.decompose().length === b.decompose().length,
    genusEqual: a.genus() === b.genus(),
    // 🔺️Tessellation is EXPECTED to differ, so the counts are reported and never asserted.
    expected: { volume: expectedVolume, area: a.surfaceArea(), genus: a.genus(), triangles: a.numTri(), vertices: a.numVert(), components: a.decompose().length },
    actual: { volume: actualVolume, area: b.surfaceArea(), genus: b.genus(), triangles: b.numTri(), vertices: b.numVert(), components: b.decompose().length },
    boundingBoxDiagonal: diagonal,
    tessellationDiffers: expected.triangles.length !== actual.triangles.length,
    weldedAway: { expected: expectedSide.weldedAway, actual: actualSide.weldedAway },
  };
}
//#endregion 🔺️Mesh

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

  /**
   * 🔺️ Compares two indexed meshes on an INDEPENDENT engine family. This is the stage that makes
   * "different tessellation is allowed, a different solid is not" enforceable.
   */
  "mesh-compare": async (inputs) => {
    if (inputs.length !== 2) return { status: "failed", measurements: {}, diagnostics: [{ severity: "error", message: `mesh-compare needs exactly two inputs, got ${inputs.length}` }] };
    return { status: "ok", measurements: await compareMeshes(readMesh(inputs[0]!), readMesh(inputs[1]!)) };
  },

  /**
   * 🔺️ Tessellates both STEP files at ONE declared tolerance and compares the results — the whole
   * mesh half of the gate in a single invocation, so the two sides can never be measured at two
   * different tessellation settings.
   */
  "step-mesh-compare": async (inputs, options) => {
    if (inputs.length !== 2) return { status: "failed", measurements: {}, diagnostics: [{ severity: "error", message: `step-mesh-compare needs exactly two inputs, got ${inputs.length}` }] };
    const b = await brep();
    const tolerance = Number(options.tolerance ?? "1e-3");
    const angular = Number(options.angularTolerance ?? "0.1");
    const tessellate = async (input: string): Promise<IndexedMesh> => {
      const shape = await importStep(input);
      const meshed = unwrap((b.mesh as (s: unknown, o: unknown) => unknown)(shape, { tolerance, angularTolerance: angular }), `mesh ${input}`) as Record<string, ArrayLike<number>>;
      return { vertices: Array.from(meshed.vertices ?? meshed.positions!), triangles: Array.from(meshed.triangles ?? meshed.indices!), tolerance, angularTolerance: angular };
    };
    const measurements = await compareMeshes(await tessellate(inputs[0]!), await tessellate(inputs[1]!));
    return { status: "ok", measurements: { ...measurements, tolerance, angularTolerance: angular } };
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
  // ⚙️A mesh probe reports the MESH engine family, not the exact kernel's — independence accounting
  // reads this field, and mislabelling it would make two different engines look like one.
  const meshSide = probe === "mesh-compare" || probe === "step-mesh-compare";
  const base = { schema: "semio.repository-test.probe-report/v2", probe, probeVersion: meshSide ? MESH_PROBE_VERSION : PROBE_VERSION, engine: meshSide ? MESH_ENGINE : ENGINE } as const;
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
