#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.semio@v1/🔺️mesh`.
//
// Everything here MARSHALS and INVOKES; nothing here computes geometry. Every number comes out of a
// third-party library — `three` parses the carrier, `manifold-3d` measures the solid, `three-mesh-bvh`
// answers closest-point queries. The pipeline compares the emitted `measurements` against declared
// assertions and performs no arithmetic of its own, which is what keeps the reference external.
//
// The carrier decides what is checkable, and that is why there are two groups of probes here. STL, OBJ
// and PLY carry triangles and nothing else, so they can witness the seven GEOMETRY mutations. glTF
// additionally carries PBR metallic-roughness materials and texture images, so it is the only carrier
// that can witness the ten MATERIAL and TEXTURE mutations — `gltf-materials` exists for exactly those.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts mesh-import      --input <a.stl|a.obj|a.ply|a.gltf|a.glb>
//   bun 📜️script.ts mesh-validity    --input <a.*>
//   bun 📜️script.ts measure          --input <a.*>
//   bun 📜️script.ts topology         --input <a.*>
//   bun 📜️script.ts gltf-materials   --input <a.gltf|a.glb>
//   bun 📜️script.ts mesh-compare     --input <expected.*> --input <actual.*>
//   bun 📜️script.ts material-compare --input <expected.gltf> --input <actual.gltf>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🔬️probes/📜️script.ts — the sibling BRep suite

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { extname } from "node:path";
import * as THREE from "three";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { PLYLoader } from "three/examples/jsm/loaders/PLYLoader.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { MeshBVH } from "three-mesh-bvh";
import Module from "manifold-3d";
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

/** ⚙️ Carrier parsing is three's. Measurement is manifold's. They are DIFFERENT engine families, which
 *  is what lets one check the other rather than confirming its own reading. */
const PARSE_ENGINE = { family: "threejs", implementation: "three loaders", version: "0.182.0" } as const;
const MEASURE_ENGINE = { family: "manifold", implementation: "manifold-3d wasm", version: "3.5.1" } as const;
const PROBE_VERSION = "three@0.182.0 + manifold-3d@3.5.1 + three-mesh-bvh@0.9.14";

type IndexedMesh = { positions: number[]; triangles: number[] };
//#endregion 🧬️Contract

//#region 📥️Carrier
/** 🩹️ three's loaders and exporters reach for browser globals. Each of these was found by a probe that
 *  FAILED WITHOUT THEM, and two of the three failed SILENTLY — `GLTFExporter` assigns `onloadend` rather
 *  than `onload`, and `GLTFLoader` dispatches a `ProgressEvent`; a shim missing either does not throw,
 *  the completion callback simply never fires and the run hangs. That is why `main` also carries a
 *  watchdog: in a test harness a hang is worse than a failure, because it reports nothing at all. */
(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => {
  cb();
  return 0;
};

class ShimProgressEvent extends Event {
  lengthComputable: boolean;
  loaded: number;
  total: number;
  constructor(type: string, init?: { lengthComputable?: boolean; loaded?: number; total?: number }) {
    super(type);
    this.lengthComputable = init?.lengthComputable ?? false;
    this.loaded = init?.loaded ?? 0;
    this.total = init?.total ?? 0;
  }
}
(globalThis as { ProgressEvent?: unknown }).ProgressEvent ??= ShimProgressEvent;

class ShimFileReader {
  result: unknown = null;
  onload: (() => void) | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((e: unknown) => void) | null = null;
  private done(): void {
    this.onloadend?.();
    this.onload?.();
  }
  readAsArrayBuffer(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = buffer;
      this.done();
    }, (error) => this.onerror?.(error));
  }
  readAsDataURL(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(buffer).toString("base64")}`;
      this.done();
    }, (error) => this.onerror?.(error));
  }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader as unknown as typeof FileReader;

function geometryOf(object: THREE.Object3D): THREE.BufferGeometry[] {
  const found: THREE.BufferGeometry[] = [];
  object.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (mesh.isMesh && mesh.geometry) found.push(mesh.geometry as THREE.BufferGeometry);
  });
  return found;
}

/** 📥️ Parse a carrier with THREE'S OWN loader for that format. The format is chosen by extension, and an
 *  unknown one is `unsupported` rather than a guess — a mis-parsed carrier would silently compare noise. */
async function readCarrier(absPath: string): Promise<{ geometries: THREE.BufferGeometry[]; scene?: THREE.Object3D }> {
  const extension = extname(absPath).toLowerCase();
  const bytes = readFileSync(absPath);
  const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
  if (extension === ".stl") return { geometries: [new STLLoader().parse(buffer)] };
  if (extension === ".ply") return { geometries: [new PLYLoader().parse(buffer)] };
  if (extension === ".obj") {
    const scene = new OBJLoader().parse(new TextDecoder().decode(bytes));
    return { geometries: geometryOf(scene), scene };
  }
  if (extension === ".gltf" || extension === ".glb") {
    const gltf = await new Promise<{ scene: THREE.Object3D }>((resolve, reject) => {
      new GLTFLoader().parse(extension === ".glb" ? buffer : new TextDecoder().decode(bytes), "", (result) => resolve(result as { scene: THREE.Object3D }), reject);
    });
    return { geometries: geometryOf(gltf.scene), scene: gltf.scene };
  }
  throw new Error(`unsupported carrier extension ${extension}`);
}

/** 🔺️ Flatten every geometry in the carrier into one indexed triangle set. */
function indexedOf(geometries: readonly THREE.BufferGeometry[]): IndexedMesh {
  const positions: number[] = [];
  const triangles: number[] = [];
  for (const geometry of geometries) {
    const base = positions.length / 3;
    const attribute = geometry.getAttribute("position");
    if (!attribute) continue;
    for (let i = 0; i < attribute.count; i += 1) positions.push(attribute.getX(i), attribute.getY(i), attribute.getZ(i));
    if (geometry.index) for (let i = 0; i < geometry.index.count; i += 1) triangles.push(base + (geometry.index.array[i] as number));
    else for (let i = 0; i < attribute.count; i += 1) triangles.push(base + i);
  }
  return { positions, triangles };
}
//#endregion 📥️Carrier

//#region 🔺️Mesh
type ManifoldModule = Awaited<ReturnType<typeof Module>>;
type ManifoldSolid = ReturnType<ManifoldModule["Manifold"]["cube"]>;
let kernelPromise: Promise<ManifoldModule> | undefined;
async function meshKernel(): Promise<ManifoldModule> {
  kernelPromise ??= (async () => {
    const wasm = await Module();
    wasm.setup();
    return wasm;
  })();
  return kernelPromise;
}

function boundsOf(positions: readonly number[]): { min: [number, number, number]; max: [number, number, number]; diagonal: number } {
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < positions.length; i += 3)
    for (let axis = 0; axis < 3; axis += 1) {
      const value = positions[i + axis] as number;
      if (value < min[axis]!) min[axis] = value;
      if (value > max[axis]!) max[axis] = value;
    }
  const diagonal = Math.hypot(max[0] - min[0], max[1] - min[1], max[2] - min[2]);
  return { min, max, diagonal: Number.isFinite(diagonal) ? diagonal : 0 };
}

/** 🔗️ STL and OBJ are triangle SOUPS — every facet carries its own copy of each corner, so the mesh has
 *  no shared topology and manifold refuses it outright. Welding rebuilds that topology on a grid keyed to
 *  the model's OWN size: a fixed constant merges real detail on a millimetre part and welds nothing at
 *  all on a kilometre one. MEASURED: a 20 mm cube with a bore re-imported from STL welds 6336 soup
 *  corners back to exactly the 1056 shared vertices manifold built it with, 0 degenerate triangles. */
function weld(mesh: IndexedMesh): { vertices: number[]; triangles: number[]; weldedAway: number; degenerate: number } {
  const grid = Math.max(1e-9, boundsOf(mesh.positions).diagonal * 1e-7);
  const index = new Map<string, number>();
  const vertices: number[] = [];
  const remapped: number[] = [];
  for (const corner of mesh.triangles) {
    const x = mesh.positions[corner * 3] as number;
    const y = mesh.positions[corner * 3 + 1] as number;
    const z = mesh.positions[corner * 3 + 2] as number;
    const key = `${Math.round(x / grid)},${Math.round(y / grid)},${Math.round(z / grid)}`;
    let at = index.get(key);
    if (at === undefined) {
      at = vertices.length / 3;
      index.set(key, at);
      vertices.push(x, y, z);
    }
    remapped.push(at);
  }
  let degenerate = 0;
  const triangles: number[] = [];
  for (let t = 0; t < remapped.length; t += 3) {
    const [a, b, c] = [remapped[t] as number, remapped[t + 1] as number, remapped[t + 2] as number];
    if (a === b || b === c || a === c) degenerate += 1;
    else triangles.push(a, b, c);
  }
  return { vertices, triangles, weldedAway: mesh.triangles.length - vertices.length / 3, degenerate };
}

async function asSolid(mesh: IndexedMesh): Promise<{ solid: ManifoldSolid; weldedAway: number; degenerate: number }> {
  const kernel = await meshKernel();
  const welded = weld(mesh);
  const solid = kernel.Manifold.ofMesh(new kernel.Mesh({ numProp: 3, vertProperties: new Float32Array(welded.vertices), triVerts: new Uint32Array(welded.triangles) }));
  return { solid, weldedAway: welded.weldedAway, degenerate: welded.degenerate };
}

/** 📐️ Symmetric Hausdorff via three-mesh-bvh closest-point queries. Exact at vertices and a LOWER BOUND
 *  between them — stated rather than hidden, because a sampled bound reported as exact would be a claim
 *  the measurement does not support. */
async function symmetricHausdorff(expected: IndexedMesh, actual: IndexedMesh): Promise<{ symmetricHausdorff: number; expectedToActual: number; actualToExpected: number; samples: number }> {
  const build = (mesh: IndexedMesh): THREE.BufferGeometry => {
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.BufferAttribute(new Float32Array(mesh.positions), 3));
    geometry.setIndex(new THREE.BufferAttribute(new Uint32Array(mesh.triangles), 1));
    return geometry.toNonIndexed();
  };
  const oneSided = (from: IndexedMesh, toward: THREE.BufferGeometry): number => {
    const bvh = new MeshBVH(toward);
    const point = new THREE.Vector3();
    const target = { point: new THREE.Vector3(), distance: 0 };
    let worst = 0;
    for (let i = 0; i < from.positions.length; i += 3) {
      point.set(from.positions[i] as number, from.positions[i + 1] as number, from.positions[i + 2] as number);
      const hit = bvh.closestPointToPoint(point, target as never);
      if (hit && hit.distance > worst) worst = hit.distance;
    }
    return worst;
  };
  const expectedToActual = oneSided(expected, build(actual));
  const actualToExpected = oneSided(actual, build(expected));
  return { symmetricHausdorff: Math.max(expectedToActual, actualToExpected), expectedToActual, actualToExpected, samples: expected.positions.length / 3 + actual.positions.length / 3 };
}

async function compareMeshes(expected: IndexedMesh, actual: IndexedMesh): Promise<Record<string, unknown>> {
  const kernel = await meshKernel();
  const expectedSide = await asSolid(expected);
  const actualSide = await asSolid(actual);
  const a = expectedSide.solid;
  const b = actualSide.solid;
  const difference = kernel.Manifold.difference(a, b).add(kernel.Manifold.difference(b, a));
  const symmetricDifferenceVolume = difference.volume();
  const expectedVolume = a.volume();
  const diagonal = boundsOf(expected.positions).diagonal;
  const relative = (x: number, y: number): number => (Math.abs(y) < Number.EPSILON ? Math.abs(x - y) : Math.abs(x - y) / Math.abs(y));
  const hausdorff = await symmetricHausdorff(expected, actual);
  return {
    // 📐️Normalized by the bounding-box diagonal, so the bound is scale-free — one threshold means the
    // same thing on a 0.02 mm part and on a 10 m building.
    normalizedSymmetricHausdorff: hausdorff.symmetricHausdorff / Math.max(diagonal, Number.EPSILON),
    symmetricHausdorff: hausdorff.symmetricHausdorff,
    hausdorffExpectedToActual: hausdorff.expectedToActual,
    hausdorffActualToExpected: hausdorff.actualToExpected,
    hausdorffSamples: hausdorff.samples,
    hausdorffSampling: "mesh vertices of both sides; exact at vertices, a lower bound between them",
    // 🎯️The gating metric, normalized by the expected volume so it is scale-free.
    normalizedSymmetricDifferenceVolume: symmetricDifferenceVolume / Math.max(expectedVolume, Number.EPSILON),
    symmetricDifferenceVolume,
    relativeVolumeError: relative(b.volume(), expectedVolume),
    relativeAreaError: relative(b.surfaceArea(), a.surfaceArea()),
    connectedComponentsEqual: a.decompose().length === b.decompose().length,
    genusEqual: a.genus() === b.genus(),
    // 🔺️A DIFFERENT TESSELLATION OF THE SAME SURFACE IS LEGITIMATE, so counts are reported, never asserted.
    expected: { volume: expectedVolume, area: a.surfaceArea(), genus: a.genus(), triangles: a.numTri(), vertices: a.numVert(), components: a.decompose().length },
    actual: { volume: b.volume(), area: b.surfaceArea(), genus: b.genus(), triangles: b.numTri(), vertices: b.numVert(), components: b.decompose().length },
    boundingBoxDiagonal: diagonal,
    tessellationDiffers: expected.triangles.length !== actual.triangles.length,
    weldedAway: { expected: expectedSide.weldedAway, actual: actualSide.weldedAway },
    degenerate: { expected: expectedSide.degenerate, actual: actualSide.degenerate },
  };
}
//#endregion 🔺️Mesh

//#region 🎨️Material
/** 🎨️ Read the PBR material and texture state three parsed out of a glTF. This is the ONLY carrier that
 *  encodes it, which is why the ten material and texture mutations are checkable here and nowhere else —
 *  STL, OBJ and PLY carry triangles, so a roughness change is invisible in them by construction. */
function materialsOf(scene: THREE.Object3D): Record<string, unknown> {
  const materials: Record<string, unknown>[] = [];
  const textures: Record<string, unknown>[] = [];
  scene.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (!mesh.isMesh) return;
    for (const raw of Array.isArray(mesh.material) ? mesh.material : [mesh.material]) {
      const material = raw as THREE.MeshStandardMaterial;
      if (!material) continue;
      materials.push({
        name: material.name,
        baseColor: material.color ? [material.color.r, material.color.g, material.color.b] : null,
        opacity: material.opacity ?? null,
        metallic: material.metalness ?? null,
        roughness: material.roughness ?? null,
        hasBaseColorTexture: Boolean(material.map),
      });
      if (material.map) textures.push({ name: material.map.name, mime: (material.map.userData as { mimeType?: string } | undefined)?.mimeType ?? null, width: material.map.image?.width ?? null, height: material.map.image?.height ?? null });
    }
  });
  const sorted = [...materials].sort((x, y) => String(x.name).localeCompare(String(y.name)));
  return {
    materialCount: materials.length,
    textureCount: textures.length,
    materials: sorted,
    textures: [...textures].sort((x, y) => String(x.name).localeCompare(String(y.name))),
    materialDigest: createHash("sha256").update(JSON.stringify(sorted)).digest("hex"),
  };
}
//#endregion 🎨️Material

//#region 🔬️Probes
type Probe = (inputs: string[]) => Promise<Pick<ProbeReport, "status" | "measurements"> & { diagnostics?: ProbeReport["diagnostics"]; engine?: ProbeReport["engine"] }>;

function requireInputs(inputs: readonly string[], n: number, probe: string): void {
  if (inputs.length < n) throw new Error(`${probe} needs ${n} --input path(s), got ${inputs.length}`);
}

const PROBES: Record<string, Probe> = {
  "mesh-import": async (inputs) => {
    requireInputs(inputs, 1, "mesh-import");
    const { geometries } = await readCarrier(inputs[0]!);
    const mesh = indexedOf(geometries);
    const bounds = boundsOf(mesh.positions);
    return { status: "ok", engine: PARSE_ENGINE, measurements: { parsed: true, geometryCount: geometries.length, vertices: mesh.positions.length / 3, triangles: mesh.triangles.length / 3, boundingBoxMin: bounds.min, boundingBoxMax: bounds.max, boundingBoxDiagonal: bounds.diagonal, sha256: createHash("sha256").update(readFileSync(inputs[0]!)).digest("hex") } };
  },
  "mesh-validity": async (inputs) => {
    requireInputs(inputs, 1, "mesh-validity");
    const { geometries } = await readCarrier(inputs[0]!);
    const mesh = indexedOf(geometries);
    const welded = weld(mesh);
    try {
      const side = await asSolid(mesh);
      return { status: "ok", measurements: { manifold: true, weldedAway: side.weldedAway, degenerateTriangles: side.degenerate, genus: side.solid.genus(), components: side.solid.decompose().length, volumeIsPositive: side.solid.volume() > 0 } };
    } catch (error) {
      // ✘️A mesh the kernel refuses is a REPORTED FACT, not a crash and not a skip: a carrier that cannot
      // form a solid is exactly what several mutations are expected to produce or to avoid producing.
      return { status: "ok", measurements: { manifold: false, weldedAway: welded.weldedAway, degenerateTriangles: welded.degenerate }, diagnostics: [{ severity: "warning", message: "manifold refused the welded mesh", detail: String((error as Error).message ?? error) }] };
    }
  },
  measure: async (inputs) => {
    requireInputs(inputs, 1, "measure");
    const { geometries } = await readCarrier(inputs[0]!);
    const mesh = indexedOf(geometries);
    const side = await asSolid(mesh);
    const bounds = boundsOf(mesh.positions);
    return { status: "ok", measurements: { volume: side.solid.volume(), surfaceArea: side.solid.surfaceArea(), boundingBoxMin: bounds.min, boundingBoxMax: bounds.max, boundingBoxDiagonal: bounds.diagonal } };
  },
  topology: async (inputs) => {
    requireInputs(inputs, 1, "topology");
    const { geometries } = await readCarrier(inputs[0]!);
    const side = await asSolid(indexedOf(geometries));
    return { status: "ok", measurements: { genus: side.solid.genus(), connectedComponents: side.solid.decompose().length, triangles: side.solid.numTri(), vertices: side.solid.numVert() } };
  },
  "gltf-materials": async (inputs) => {
    requireInputs(inputs, 1, "gltf-materials");
    const extension = extname(inputs[0]!).toLowerCase();
    // ✘️Asking a triangle-only carrier for PBR state is `unsupported`, never an empty `ok`. An empty
    // material list reported as ok would let a roughness mutation pass against an STL that never
    // carried roughness at all — a green result standing on the absence of the evidence.
    if (extension !== ".gltf" && extension !== ".glb") return { status: "unsupported", measurements: { reason: `${extension} carries triangles only; PBR material state is not encoded in it` } };
    const { scene } = await readCarrier(inputs[0]!);
    return { status: "ok", engine: PARSE_ENGINE, measurements: materialsOf(scene!) };
  },
  "mesh-compare": async (inputs) => {
    requireInputs(inputs, 2, "mesh-compare");
    const expected = indexedOf((await readCarrier(inputs[0]!)).geometries);
    const actual = indexedOf((await readCarrier(inputs[1]!)).geometries);
    return { status: "ok", measurements: await compareMeshes(expected, actual) };
  },
  "material-compare": async (inputs) => {
    requireInputs(inputs, 2, "material-compare");
    for (const input of inputs.slice(0, 2)) {
      const extension = extname(input).toLowerCase();
      if (extension !== ".gltf" && extension !== ".glb") return { status: "unsupported", measurements: { reason: `${extension} does not encode PBR material state` } };
    }
    const expected = materialsOf((await readCarrier(inputs[0]!)).scene!);
    const actual = materialsOf((await readCarrier(inputs[1]!)).scene!);
    return { status: "ok", engine: PARSE_ENGINE, measurements: { ...actual, materialCountEqual: expected.materialCount === actual.materialCount, textureCountEqual: expected.textureCount === actual.textureCount, materialDigestEqual: expected.materialDigest === actual.materialDigest, expected } };
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
  // ⏱️A probe that hangs reports nothing, which is strictly worse than one that fails. Bound every run.
  const budgetMs = Number(process.env.SEMIO_PROBE_TIMEOUT_MS ?? 120_000);
  const watchdog = new Promise<never>((_, reject) => setTimeout(() => reject(new Error(`probe exceeded ${budgetMs} ms`)), budgetMs).unref?.());
  const run = PROBES[probe];
  if (!run) return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: MEASURE_ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  try {
    const result = await Promise.race([run(inputs), watchdog]);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: result.engine ?? MEASURE_ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: MEASURE_ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
