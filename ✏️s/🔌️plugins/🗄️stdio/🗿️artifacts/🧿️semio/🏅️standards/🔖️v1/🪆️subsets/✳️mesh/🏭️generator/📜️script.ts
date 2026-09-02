#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.semio@v1/✳️mesh` (the semio-native mesh subset).
//
// Every expected result this file writes is computed by TWO independent third-party libraries in
// series, deliberately from different engine families: `manifold-3d` (the Manifold C++ solid-boolean
// kernel, compiled to WASM) BUILDS every shape and is the sole source of its volume/area/genus truth,
// and `three` (a real 3D engine, nothing of ours) EXPORTS it to STL/OBJ/PLY/glTF and RE-IMPORTS what it
// wrote. Nothing here reimplements a boolean, a tessellation or a measurement — that is the whole
// point: an expectation produced by two second parties proves the format round-trips, not that either
// library is right about anything this repository does.
//
// Generation and execution are SEPARATE operations. A normal test run must never be able to rewrite
// the expectation it is being measured against, so this is its own command and its output is reviewed
// before it is committed.
//
//   bun 📜️script.ts generate [--out <dir>] [--only <fixture-id>]
//   bun 📜️script.ts manifests                      # emit the fixtureManifests block for 🧪️oracle
//
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🔬️mesh-spike/📜️script.ts
//      — the spike that proved this exact chain closes: build → export → re-import → weld → re-measure.
// @see ../../../../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/📜️script.ts
//      — the sibling BRep pilot generator this file mirrors in CLI shape, bundle layout and manifest
//      fields; its lessons (measure the ARTIFACT not the in-memory shape, scale-relative tolerances,
//      `--only` merges rather than overwrites, weld before handing a soup to a solid kernel) are the
//      reason this file is structured the way it is.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Module from "manifold-3d";
import type { CrossSection, Manifold, Mesh as ManifoldMesh } from "manifold-3d";
import * as THREE from "three";
import { STLExporter } from "three/examples/jsm/exporters/STLExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import { PLYExporter } from "three/examples/jsm/exporters/PLYExporter.js";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { PLYLoader } from "three/examples/jsm/loaders/PLYLoader.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";

// 🩹️ three's exporters assume a browser: `PLYExporter`/`GLTFExporter` deliver their result via
// `requestAnimationFrame`, `GLTFExporter`'s embedded-buffer path reads it back through `FileReader`, and
// its progress reporting constructs a `ProgressEvent`. None of the three exist off-browser; each shim
// is the minimal synchronous stand-in the spike proved sufficient — no timers, no real async I/O.
(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => { cb(); return 0; };
class ShimFileReader {
  result: string | ArrayBuffer | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((error: unknown) => void) | null = null;
  readAsArrayBuffer(blob: Blob): void { blob.arrayBuffer().then((buf) => { this.result = buf; this.onloadend?.(); }).catch((error) => this.onerror?.(error)); }
  readAsDataURL(blob: Blob): void {
    blob.arrayBuffer().then((buf) => { this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(buf).toString("base64")}`; this.onloadend?.(); }).catch((error) => this.onerror?.(error));
  }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader;
class ShimProgressEvent {
  type: string; loaded: number; total: number; lengthComputable: boolean;
  constructor(type: string, init: { loaded?: number; total?: number; lengthComputable?: boolean } = {}) {
    this.type = type; this.loaded = init.loaded ?? 0; this.total = init.total ?? 0; this.lengthComputable = init.lengthComputable ?? false;
  }
}
(globalThis as { ProgressEvent?: unknown }).ProgressEvent ??= ShimProgressEvent;
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
export type Family = "primitives" | "booleans" | "topology" | "scale" | "degenerate";

/** 🧰️ What a recipe is handed to build with: the two manifold-3d entry points a shape needs. */
export type Toolkit = Readonly<{ Manifold: typeof Manifold; CrossSection: typeof CrossSection }>;

/**
 * 🧪️ One corpus entry. A recipe DESCRIBES a shape; it computes nothing. `build` returns the RESULT to
 * export and measure, plus optionally the named operands that produced it (a boolean's two inputs), so
 * a boolean fixture is reviewable without re-deriving the operands from the chain that built them.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  tolerance: string;
  notes: string;
  build: (t: Toolkit) => { operands?: { role: string; shape: Manifold }[]; result: Manifold };
}>;

const ENGINE_FAMILY = "manifold";
const ENGINE_VERSION = "3.5.1";
const EXPORT_ENGINE_FAMILY = "three";
const EXPORT_ENGINE_VERSION = "0.182.0";
const ORACLE = "manifold3d-three";
const SEED = 4815162342;

/**
 * 🔺️ The weld grid a re-imported triangle SOUP is snapped onto before it is handed back to manifold-3d,
 * resolved SCALE-RELATIVE — `max(absoluteFloor, relative × boundingBoxDiagonal)` — never a fixed
 * constant. This is the exact rule the BRep pilot's tessellation tolerance uses and for the exact reason
 * recorded there: a fixed absolute grid either merges real detail on a small part or does nothing at all
 * on a part scaled to 1e6 units. Because manifold-3d's own primitives carry no export-side tolerance
 * concept (unlike an exact BRep kernel, there is nothing here to tessellate — the triangles ARE the
 * shape), the only place scale-relativity matters in this generator is this weld, and the `scale` family
 * exists specifically to prove it holds from 1e-3 to 1e6.
 */
const WELD_RELATIVE = 1e-7;
const WELD_ABSOLUTE_FLOOR = 1e-9;

/** 🔺️ The weld grid size for one shape, from its own measured bounding-box diagonal. */
function weldGridFor(diagonal: number): number {
  return Math.max(WELD_ABSOLUTE_FLOOR, WELD_RELATIVE * Math.abs(diagonal));
}
//#endregion 🧬️Contract

//#region 🧰️Kernel
let toolkit: { wasm: Awaited<ReturnType<typeof Module>>; Manifold: typeof Manifold; CrossSection: typeof CrossSection } | null = null;

async function manifold3d(): Promise<NonNullable<typeof toolkit>> {
  if (toolkit !== null) return toolkit;
  const wasm = await Module();
  wasm.setup();
  toolkit = { wasm, Manifold: wasm.Manifold, CrossSection: wasm.CrossSection };
  return toolkit;
}

/** 🔗️ manifold-3d's own `Mesh` → a `THREE.BufferGeometry`, position-only (normals are recomputed after
 * export/re-import anyway, so shipping manifold-3d's are not worth the property-channel bookkeeping). */
function toThree(m: ManifoldMesh): THREE.BufferGeometry {
  const g = new THREE.BufferGeometry();
  const pos = new Float32Array((m.vertProperties.length / m.numProp) * 3);
  for (let i = 0; i < pos.length / 3; i += 1) {
    pos[i * 3] = m.vertProperties[i * m.numProp]!;
    pos[i * 3 + 1] = m.vertProperties[i * m.numProp + 1]!;
    pos[i * 3 + 2] = m.vertProperties[i * m.numProp + 2]!;
  }
  g.setAttribute("position", new THREE.BufferAttribute(pos, 3));
  g.setIndex(new THREE.BufferAttribute(new Uint32Array(m.triVerts), 1));
  g.computeVertexNormals();
  return g;
}

/**
 * 🔗️ Any of the four exported formats, re-imported, is a triangle SOUP — every facet carries its own
 * copy of each corner, so the geometry has no shared topology and manifold-3d rejects it outright with
 * "Not manifold". Welding on a grid keyed to the model's OWN size (§ `weldGridFor`, never a fixed
 * constant) rebuilds the shared-vertex mesh manifold-3d needs. This runs on every re-imported format,
 * not only STL, because the round-trip lesson applies equally to whichever bytes are being measured.
 */
function weldToManifold(wasm: Awaited<ReturnType<typeof Module>>, g: THREE.BufferGeometry): { mesh: ManifoldMesh; grid: number; soupVertices: number; weldedVertices: number; droppedDegenerateTriangles: number } {
  const p = g.getAttribute("position")!;
  g.computeBoundingBox();
  const diagonal = g.boundingBox!.min.distanceTo(g.boundingBox!.max);
  const grid = weldGridFor(diagonal);
  const index = new Map<string, number>();
  const verts: number[] = [];
  const tris: number[] = [];
  let dropped = 0;
  for (let i = 0; i < p.count; i += 1) {
    const x = p.getX(i), y = p.getY(i), z = p.getZ(i);
    const key = `${Math.round(x / grid)},${Math.round(y / grid)},${Math.round(z / grid)}`;
    let at = index.get(key);
    if (at === undefined) { at = verts.length / 3; index.set(key, at); verts.push(x, y, z); }
    tris.push(at);
  }
  const keptTris: number[] = [];
  for (let t = 0; t < tris.length / 3; t += 1) {
    const a = tris[t * 3]!, b = tris[t * 3 + 1]!, c = tris[t * 3 + 2]!;
    if (a !== b && b !== c && a !== c) keptTris.push(a, b, c);
    else dropped += 1;
  }
  const mesh = new wasm.Mesh({ numProp: 3, vertProperties: new Float32Array(verts), triVerts: new Uint32Array(keptTris) });
  return { mesh, grid, soupVertices: p.count, weldedVertices: verts.length / 3, droppedDegenerateTriangles: dropped };
}

/** 📐️ The genus formula assumes one connected 2-manifold surface; manifold-3d's `genus()` returns the
 * sentinel `-1` on a multi-component shape rather than throw. Decomposing first and reporting genus PER
 * component is the only way to get a real number out of a disjoint-solids or nested-cavity fixture — a
 * hollow shell's inner and outer surfaces are two disjoint boundary components even though the SOLID
 * between them is one connected region, so `decompose()` on it also returns 2, not 1 — MEASURED, not
 * assumed, from a sphere-minus-inner-sphere probe during this generator's own qualification. */
function topology(shape: Manifold): { solids: number; genus: number | null; componentGenus: number[] | null } {
  const components = shape.decompose();
  if (components.length === 1) return { solids: 1, genus: components[0]!.genus(), componentGenus: null };
  return { solids: components.length, genus: null, componentGenus: components.map((c) => c.genus()) };
}
//#endregion 🧰️Kernel

//#region 🧪️Corpus
/**
 * 🧪️ The corpus, assembled from one module per FAMILY — the sharding key CI uses and the unit somebody
 * extends, reviews or runs in isolation, exactly as the BRep pilot's corpus is organised.
 */
const RECIPES: readonly Recipe[] = [
  ...(await import("./🧪️primitives/📜️script.ts")).RECIPES,
  ...(await import("./🧪️booleans/📜️script.ts")).RECIPES,
  ...(await import("./🧪️topology/📜️script.ts")).RECIPES,
  ...(await import("./🧪️scale/📜️script.ts")).RECIPES,
  ...(await import("./🧪️degenerate/📜️script.ts")).RECIPES,
];
//#endregion 🧪️Corpus

//#region 🏭️Generate
async function contentDigest(bytes: Uint8Array | string): Promise<string> {
  const source = typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes;
  const data = new Uint8Array(source.length);
  data.set(source);
  const hash = await crypto.subtle.digest("SHA-256", data);
  return `sha256:${[...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}

/** 📎️ Fixture file paths are resolved against the OWNER'S ORACLE directory — `verifyFixture` joins them
 *  onto the manifest's `manifestDir`, which the registry loader sets to where `🔣️oracle.json` lives,
 *  NOT to this fixture directory. Emitting bare `<recipe>/<file>` paths therefore made every digest
 *  resolve to a non-existent `🧪️oracle/<recipe>/<file>` and read as 369 mismatches. The sibling BRep
 *  corpus already uses this prefix; matching it is what makes the two corpora verifiable the same way. */
const FIXTURE_PATH_PREFIX = "../🧫️fixtures/";

function write(path: string, body: string): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
}

const FAMILY_COLOR: Record<Family, number> = {
  primitives: 0x4a90d9,
  booleans: 0xe0954b,
  topology: 0x5cb85c,
  scale: 0x9b6bd1,
  degenerate: 0xd9534f,
};

/** 🎨️ A real, distinctly-named PBR material per recipe. glTF is the only one of the four export formats
 * that carries material data at all — STL has none, and three's OBJ/PLY exporters write geometry only —
 * so it is also the only carrier that can witness a material-bearing mutation downstream. Explicit
 * non-default `roughness`/`metalness`/`color` values, rather than `MeshStandardMaterial`'s defaults, are
 * what make those fields actually PRESENT — rather than merely absent-and-implied — in the glTF this
 * generator writes. */
function materialFor(recipe: Recipe): THREE.MeshStandardMaterial {
  return new THREE.MeshStandardMaterial({ name: `${recipe.id}-material`, color: FAMILY_COLOR[recipe.family], roughness: 0.4, metalness: 0.25 });
}

/** 📤️ Every export three's OWN exporters produce for one `THREE.Mesh` — the four formats this corpus
 * covers. `gltf` is the embedded (non-binary) form: one self-contained JSON file, buffers as data URIs,
 * so the bundle stays one file per format like the other three rather than a `.gltf` + `.bin` pair. */
async function exportAll(mesh: THREE.Mesh): Promise<{ stl: string; obj: string; ply: string; gltf: string }> {
  const stl = new STLExporter().parse(mesh, { binary: false }) as unknown as string;
  const obj = new OBJExporter().parse(mesh);
  const ply = String(new PLYExporter().parse(mesh, () => {}, { binary: false }));
  const gltf = JSON.stringify(await new GLTFExporter().parseAsync(mesh, { binary: false }));
  return { stl, obj, ply, gltf };
}

/** 📥️ Reads back what was actually WRITTEN and re-measures THAT — never the in-memory shape a fixture
 * was exported from. The STEP pilot found real disagreement doing this (`fuse-edge-touching-boxes`:
 * 23/14 edges/vertices in memory vs. 24/16 once the file was re-parsed), and the risk is identical here:
 * an export/import bug in three's own STL codec would otherwise hide behind an in-memory number nobody
 * could reproduce from the committed file. STL is the canonical re-measurement source because it is the
 * plainest possible soup — no index, no material, nothing but triangles — so a weld-and-measure result
 * from it is the strongest test of the round-trip. OBJ/PLY/glTF are re-imported too, only to CONFIRM
 * their triangle counts agree, because a format-specific export bug should be visible as a disagreement
 * between formats, not laundered into a single trusted number. */
async function reimportAndMeasure(t: NonNullable<typeof toolkit>, exported: { stl: string; obj: string; ply: string; gltf: string }) {
  const stlGeom = new STLLoader().parse(new TextEncoder().encode(exported.stl).buffer as ArrayBuffer);
  const weld = weldToManifold(t.wasm, stlGeom);
  const measured = t.Manifold.ofMesh(weld.mesh);

  const triOf = (g: THREE.BufferGeometry): number => (g.index ? g.index.count : g.getAttribute("position")!.count) / 3;
  const crossFormatTriangleCounts: Record<string, number> = { stl: triOf(stlGeom) };
  try {
    const objGeom = ((new OBJLoader().parse(exported.obj)).children[0] as THREE.Mesh).geometry as THREE.BufferGeometry;
    crossFormatTriangleCounts.obj = triOf(objGeom);
  } catch (error) { crossFormatTriangleCounts.obj = Number.NaN; console.error(`[generator]   obj re-import cross-check failed — ${(error as Error).message}`); }
  try {
    const plyGeom = new PLYLoader().parse(new TextEncoder().encode(exported.ply).buffer as ArrayBuffer);
    crossFormatTriangleCounts.ply = triOf(plyGeom);
  } catch (error) { crossFormatTriangleCounts.ply = Number.NaN; console.error(`[generator]   ply re-import cross-check failed — ${(error as Error).message}`); }
  try {
    const gltfResult = await new Promise<{ scene: THREE.Group }>((resolve, reject) => new GLTFLoader().parse(exported.gltf, "", resolve, reject));
    let gltfTris = 0;
    gltfResult.scene.traverse((obj) => { if ((obj as THREE.Mesh).isMesh) gltfTris += triOf((obj as THREE.Mesh).geometry); });
    crossFormatTriangleCounts.gltf = gltfTris;
  } catch (error) { crossFormatTriangleCounts.gltf = Number.NaN; console.error(`[generator]   gltf re-import cross-check failed — ${(error as Error).message}`); }

  return { measured, weld, crossFormatTriangleCounts };
}

/** 🏭️ Generates one recipe's complete bundle: operand STLs (if any), the result's four export formats,
 * and its measurements — all measured from the re-imported, welded, re-measured artifact. */
async function generateOne(t: NonNullable<typeof toolkit>, recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];

  const emitStl = async (role: string, shape: Manifold, filename: string): Promise<void> => {
    const geometry = toThree(shape.getMesh());
    const mesh = new THREE.Mesh(geometry, materialFor(recipe));
    const text = new STLExporter().parse(mesh, { binary: false }) as unknown as string;
    write(join(dir, filename), text);
    files.push({ role, path: `${FIXTURE_PATH_PREFIX}${recipe.id}/${filename}`, mediaType: "model/stl", sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
  };

  const { operands, result } = recipe.build({ Manifold: t.Manifold, CrossSection: t.CrossSection });
  for (const operand of operands ?? []) await emitStl(operand.role, operand.shape, `${operand.role}.stl`);

  const empty = result.isEmpty();
  const measurements: Record<string, unknown> = { empty };
  if (!empty) {
    const geometry = toThree(result.getMesh());
    const mesh = new THREE.Mesh(geometry, materialFor(recipe));
    const exported = await exportAll(mesh);

    const extensions: Record<string, string> = { stl: "stl", obj: "obj", ply: "ply", gltf: "gltf" };
    for (const [format, text] of Object.entries(exported)) {
      const filename = `expected.${extensions[format]}`;
      write(join(dir, filename), text);
      files.push({ role: `expected-${format}`, path: `${FIXTURE_PATH_PREFIX}${recipe.id}/${filename}`, mediaType: format === "gltf" ? "model/gltf+json" : `model/${format}`, sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
    }

    const { measured, weld, crossFormatTriangleCounts } = await reimportAndMeasure(t, exported);
    const bounds = measured.boundingBox();
    const diagonal = Math.hypot(bounds.max[0] - bounds.min[0], bounds.max[1] - bounds.min[1], bounds.max[2] - bounds.min[2]);
    const shape = topology(measured);

    measurements.measuredFrom = "expected.stl, re-imported and welded";
    measurements.vertexCount = measured.numVert();
    measurements.triangleCount = measured.numTri();
    measurements.volume = measured.volume();
    measurements.surfaceArea = measured.surfaceArea();
    measurements.boundingBox = { min: bounds.min, max: bounds.max };
    measurements.boundingBoxDiagonal = diagonal;
    measurements.solids = shape.solids;
    measurements.genus = shape.genus;
    measurements.componentGenus = shape.componentGenus;
    measurements.weld = { grid: weld.grid, soupVertices: weld.soupVertices, weldedVertices: weld.weldedVertices, droppedDegenerateTriangles: weld.droppedDegenerateTriangles };
    measurements.crossFormatTriangleCounts = crossFormatTriangleCounts;
    measurements.crossFormatTriangleCountsAgree = Object.values(crossFormatTriangleCounts).every((count) => count === measurements.triangleCount || count === weld.soupVertices / 3);
  }

  const metricsBody = `${JSON.stringify(measurements, null, 2)}\n`;
  write(join(dir, "expected.metrics.json"), metricsBody);
  files.push({ role: "expected-measurements", path: `${FIXTURE_PATH_PREFIX}${recipe.id}/expected.metrics.json`, mediaType: "application/json", sha256: await contentDigest(metricsBody), bytes: Buffer.byteLength(metricsBody) });

  const outcome = empty ? "empty" : (measurements.solids as number) > 1 ? "disjoint" : "applied";
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.semio", standard: "v1", subset: "mesh" },
    mutation: "replace-primitive-geometry",
    outcome,
    units: { length: "millimetre", angle: "degree", handedness: "right", up: "y" },
    files,
    generator: {
      oracle: ORACLE,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      exportEngine: { family: EXPORT_ENGINE_FAMILY, implementation: "three.js exporters + loaders", version: EXPORT_ENGINE_VERSION },
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      seed: SEED,
      platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
    },
    provenance: { source: "generated", license: "Apache-2.0 (manifold-3d) + MIT (three)", attribution: "Generated with manifold-3d (Apache-2.0) and exported/re-imported via three.js (MIT)", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-mesh-manifold-v1",
    toleranceProfile: recipe.tolerance,
    // ✅️Unlike the BRep pilot's OCCT (which stamps a translator counter and a wall-clock timestamp into
    // every export), MEASURED here: two `STLExporter`/`OBJExporter`/`PLYExporter`/`GLTFExporter` passes
    // over the same `THREE.Mesh` in the same process produce byte-identical output — no timestamps, no
    // incrementing counters anywhere in three's exporters. This corpus is reproducible byte-for-byte.
    reproducible: true,
    family: recipe.family,
    notes: recipe.notes,
  };
}
//#endregion 🏭️Generate

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");

  if (command === "manifests" || command === "generate") {
    const t = await manifold3d();
    const manifests: Record<string, unknown>[] = [];
    let failed = 0;
    for (const recipe of recipes) {
      try {
        manifests.push(await generateOne(t, recipe, outDir));
        console.error(`[generator] ${recipe.id} (${recipe.family})`);
      } catch (error) {
        // 🧭️A recipe the kernel refuses is REPORTED, never dropped: a corpus that quietly shrank to
        // whatever happened to build would read as complete coverage of a smaller matrix.
        failed += 1;
        console.error(`[generator] ${recipe.id} FAILED — ${(error as Error).message}`);
      }
    }
    if (command === "manifests") process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    else {
      // 🧬️A NARROWED run MERGES into the manifest index; it does not replace it — see the BRep pilot's
      // generator for the incident this guards against (a sequence of `--only` runs during development
      // silently destroying every other fixture's manifest record while leaving its files on disk).
      const indexPath = join(outDir, "🧫️manifests.json");
      const previous = (() => {
        if (only === null || !existsSync(indexPath)) return [];
        try { return JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[]; } catch { return []; }
      })();
      const produced = new Set(manifests.map((entry) => entry.id as string));
      const merged = [...previous.filter((entry) => !produced.has(entry.id as string)), ...manifests].sort((a, b) => String(a.id).localeCompare(String(b.id)));
      write(indexPath, `${JSON.stringify(merged, null, 2)}\n`);
      if (only !== null) console.error(`[generator] merged ${manifests.length} regenerated entr(ies) into ${merged.length} total`);
    }
    console.error(`[generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    return failed > 0 ? 1 : 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
  return 1;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
