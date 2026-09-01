#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.gltf@2.0/✳️any`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. Every fact in a `gltf-project` report was recovered by `three` 0.182.0's own `GLTFLoader` —
// either from the SCENE it builds (node hierarchy, transforms, cameras, skins, morph weights,
// animations, materials — real semantic interpretation of the bytes) or from `parser.json`, the
// document `GLTFLoader.parse()` itself validated (glTF version/magic, GLB chunk framing, JSON syntax)
// before this file ever touches it — for the structural facts (array order, extras, extension data,
// asset fields) the scene graph does not model. `gltf-compare` performs the GATING structural equality
// over two such projections and computes no mutation semantics of its own.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts gltf-import  --input <a.gltf|a.glb>
//   bun 📜️script.ts gltf-project --input <a.gltf|a.glb>
//   bun 📜️script.ts gltf-compare --input <expected.gltf> --input <actual.gltf>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling this file's
//      CLI/dispatch/compare shape is mirrored from (both hand structural equality to this file itself)
// @see ../🏭️generator/📜️script.ts — the write half; this file never writes, only reads

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { NodeIO } from "@gltf-transform/core";
//#endregion 🔌️Adapters

//#region 🩹️Shims
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).ProgressEvent ??= class ShimProgressEvent extends Event {
  lengthComputable = false;
  loaded = 0;
  total = 0;
};
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).requestAnimationFrame ??= (cb: () => void) => {
  cb();
  return 0;
};
//#endregion 🩹️Shims

//#region 🧬️Contract
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

const ENGINE = { family: "threejs", implementation: "three GLTFLoader (scene graph + parser.json)", version: "0.182.0" } as const;
const PROBE_VERSION = "three@0.182.0";

/** 🧱️ The SECOND reader, and a different KIND of reader — not a second opinion from the same vantage.
 *
 *  `three`'s GLTFLoader builds a SCENE GRAPH, so a resource nothing references never becomes an object:
 *  an unreferenced accessor, buffer, bufferView, image, sampler or texture is carried through as opaque
 *  JSON and interpreted by nothing. `projectDocument` says so and deliberately omits those six as
 *  standalone array facts. That is honest about three — and it is why this subset's twelve
 *  `create-*`/`delete-*` kinds over exactly those six resource types were recorded `-uncarried`.
 *
 *  `@gltf-transform/core` is a DOCUMENT-level implementation of glTF 2.0: it models those six as
 *  first-class typed resources with their own accessors, parses and re-serialises them whether or not
 *  anything references them, and so genuinely witnesses their creation and removal. Verified before it
 *  was registered: an accessor bound to no primitive round-trips through `writeJSON`/`readJSON` and comes
 *  back listed by `listAccessors()`.
 *
 *  So the twelve kinds were never unwitnessable — they were unwitnessable BY THREE, which is a different
 *  claim and became wrong the moment a document-level reader was available. */
const TRANSFORM_ENGINE = { family: "gltf-transform", implementation: "@gltf-transform/core NodeIO (document-level resource graph)", version: "4.4.2" } as const;
const TRANSFORM_PROBE_VERSION = "@gltf-transform/core@4.4.2";

/** 🧱️ Document-level projection: the six resource lists three cannot interpret, as STANDALONE facts.
 *  Deliberately disjoint from `projectDocument` — this reader answers the question that one cannot,
 *  and duplicating its scene-graph findings here would add agreement, not evidence. */
/** 📐️ Image dimensions when the payload is decodable, `null` when it is not — never a throw. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function sizeOrNull(texture: { getSize(): [number, number] | null }): [number, number] | null {
  try {
    return texture.getSize() ?? null;
  } catch {
    return null;
  }
}

async function projectResources(absPath: string): Promise<Record<string, unknown>> {
  const document = await new NodeIO().read(absPath);
  const root = document.getRoot();
  return {
    accessors: root.listAccessors().map((a) => ({ name: a.getName(), type: a.getType(), componentType: a.getComponentType(), count: a.getCount(), normalized: a.getNormalized() })),
    // 🚫️`getURI()` is NOT projected for either: it carries the OUTPUT FILENAME the writer chose
    // (`before.bin` against `after.bin`), so projecting it would report a difference on every pair
    // regardless of what the mutation did — a gate that always fires is a gate nobody reads. Only
    // resource identity and payload shape are semantic here.
    buffers: root.listBuffers().map((b) => ({ name: b.getName() })),
    // 🛡️`getSize()` DECODES the image to read its dimensions and throws on a payload it cannot parse.
    // A reader probe must report what it could not measure, never die on it — otherwise a malformed
    // image inside a fixture is indistinguishable from a broken probe.
    textures: root.listTextures().map((t) => ({ name: t.getName(), mimeType: t.getMimeType() ?? null, size: sizeOrNull(t), imageBytes: t.getImage()?.byteLength ?? null })),
    materialTextureBindings: root.listMaterials().map((m) => ({ name: m.getName(), baseColor: m.getBaseColorTexture()?.getName() ?? null })),
    counts: { accessors: root.listAccessors().length, buffers: root.listBuffers().length, textures: root.listTextures().length, materials: root.listMaterials().length, meshes: root.listMeshes().length, nodes: root.listNodes().length },
  };
}

//#endregion 🧬️Contract

//#region 📥️Read
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type ParsedGltf = { scene: THREE.Group; scenes: THREE.Group[]; animations: THREE.AnimationClip[]; cameras: THREE.Camera[]; parser: { json: any } };

async function readGltf(absPath: string): Promise<ParsedGltf> {
  const bytes = readFileSync(absPath);
  const text = new TextDecoder().decode(bytes);
  return new Promise((resolve, reject) => {
    new GLTFLoader().parse(text, "", (result) => resolve(result as unknown as ParsedGltf), reject);
  });
}

/** 🌳 A node's position in the WHOLE document's node index space, needed because scene-graph traversal
 *  alone loses the flat `nodes[]` array's own order for anything not scene-graph-reachable. Read from
 *  `parser.json`, three's own validated parse of the document — never recomputed. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function nodeProjection(nodeDef: any, index: number): Record<string, unknown> {
  return {
    index,
    name: nodeDef.name ?? null,
    children: nodeDef.children ?? [],
    mesh: nodeDef.mesh ?? null,
    camera: nodeDef.camera ?? null,
    skin: nodeDef.skin ?? null,
    weights: nodeDef.weights ?? null,
    translation: nodeDef.translation ?? null,
    rotation: nodeDef.rotation ?? null,
    scale: nodeDef.scale ?? null,
    matrix: nodeDef.matrix ?? null,
    extras: nodeDef.extras ?? null,
    extensions: nodeDef.extensions ?? null,
  };
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function primitiveProjection(primitiveDef: any): Record<string, unknown> {
  return {
    attributes: primitiveDef.attributes ?? {},
    attributeOrder: Object.keys(primitiveDef.attributes ?? {}),
    indices: primitiveDef.indices ?? null,
    material: primitiveDef.material ?? null,
    mode: primitiveDef.mode ?? 4,
    targets: (primitiveDef.targets ?? []).map((t: Record<string, unknown>) => ({ ...t, order: Object.keys(t) })),
    extras: primitiveDef.extras ?? null,
    extensions: primitiveDef.extensions ?? null,
  };
}

/** 📄 The FULL structural projection this subset's reader witnesses — every top-level entity list plus
 *  the fields each of this subset's 96 registered mutation kinds can touch, read from `parser.json`
 *  (post-validation) for pure structure and from the built scene for real semantic interpretation
 *  (camera params, skin binding, morph influence, animation tracks). Deliberately excludes buffers,
 *  bufferViews, accessors, samplers, textures and images as STANDALONE array facts — three does no real
 *  interpretation of an unreferenced one of those beyond carrying the JSON through unchanged, which is
 *  not independent evidence (see the oracle's own `rationale`). */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function projectDocument(gltf: ParsedGltf): Record<string, unknown> {
  const json = gltf.parser.json;
  return {
    asset: json.asset ?? null,
    extras: json.extras ?? null,
    extensions: json.extensions ?? null,
    extensionsUsed: json.extensionsUsed ?? [],
    extensionsRequired: json.extensionsRequired ?? [],
    defaultScene: json.scene ?? null,
    scenes: (json.scenes ?? []).map((s: Record<string, unknown>) => ({ name: s.name ?? null, nodes: s.nodes ?? [], extras: s.extras ?? null, extensions: s.extensions ?? null })),
    nodes: (json.nodes ?? []).map((n: Record<string, unknown>, i: number) => nodeProjection(n, i)),
    meshes: (json.meshes ?? []).map((m: Record<string, unknown>) => ({ name: m.name ?? null, weights: m.weights ?? null, extras: m.extras ?? null, extensions: m.extensions ?? null, primitives: (m.primitives as Record<string, unknown>[]).map(primitiveProjection) })),
    materials: (json.materials ?? []).map((m: Record<string, unknown>) => ({ name: m.name ?? null, alphaMode: m.alphaMode ?? "OPAQUE", doubleSided: m.doubleSided ?? false })),
    cameras: (json.cameras ?? []).map((c: Record<string, unknown>) => c),
    skins: (json.skins ?? []).map((s: Record<string, unknown>) => ({ joints: s.joints ?? [], skeleton: s.skeleton ?? null })),
    animations: (json.animations ?? []).map((a: Record<string, unknown>) => ({ name: a.name ?? null, channelTargets: (a.channels as Record<string, unknown>[]).map((c) => (c.target as Record<string, unknown>)) })),
    // 🎥️Camera and skin SEMANTICS as three actually interpreted them — independent confirmation that a
    // camera-bearing/skin-bearing node round-trips as the right kind of THREE object, not just a JSON blob.
    sceneCameraCount: countCameras(gltf.scene),
    sceneSkinnedMeshCount: countSkinned(gltf.scene),
    animationClipNames: gltf.animations.map((a) => a.name).sort(),
  };
}

function countCameras(root: THREE.Object3D): number {
  let n = 0;
  root.traverse((c) => {
    if ((c as THREE.Camera).isCamera) n += 1;
  });
  return n;
}
function countSkinned(root: THREE.Object3D): number {
  let n = 0;
  root.traverse((c) => {
    if ((c as THREE.SkinnedMesh).isSkinnedMesh) n += 1;
  });
  return n;
}
//#endregion 📥️Read

//#region ⚖️Compare
function canonicalize(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (value && typeof value === "object") {
    const out: Record<string, unknown> = {};
    for (const key of Object.keys(value as Record<string, unknown>).sort()) out[key] = canonicalize((value as Record<string, unknown>)[key]);
    return out;
  }
  return value;
}

function diffPaths(a: unknown, b: unknown, path: string): string[] {
  if (JSON.stringify(a) === JSON.stringify(b)) return [];
  if (Array.isArray(a) && Array.isArray(b)) {
    const diffs: string[] = [];
    const n = Math.max(a.length, b.length);
    for (let i = 0; i < n; i += 1) diffs.push(...diffPaths(a[i], b[i], `${path}[${i}]`));
    return diffs;
  }
  if (a && b && typeof a === "object" && typeof b === "object" && !Array.isArray(a) && !Array.isArray(b)) {
    const keys = new Set([...Object.keys(a as object), ...Object.keys(b as object)]);
    const diffs: string[] = [];
    for (const key of keys) diffs.push(...diffPaths((a as Record<string, unknown>)[key], (b as Record<string, unknown>)[key], `${path}.${key}`));
    return diffs;
  }
  return [`${path}: ${JSON.stringify(a)} ≠ ${JSON.stringify(b)}`];
}
//#endregion ⚖️Compare

//#region 🔬️Probes
type Probe = (inputs: string[]) => Promise<Pick<ProbeReport, "status" | "measurements"> & { diagnostics?: ProbeReport["diagnostics"]; engine?: ProbeReport["engine"]; probeVersion?: string }>;

function requireInputs(inputs: readonly string[], n: number, probe: string): void {
  if (inputs.length < n) throw new Error(`${probe} needs ${n} --input path(s), got ${inputs.length}`);
}

const PROBES: Record<string, Probe> = {
  "gltf-resources-import": async (inputs) => {
    requireInputs(inputs, 1, "gltf-resources-import");
    const projection = await projectResources(inputs[0]!);
    return { status: "ok", engine: TRANSFORM_ENGINE, probeVersion: TRANSFORM_PROBE_VERSION, measurements: { parsed: true, ...(projection.counts as Record<string, unknown>) } };
  },
  "gltf-resources-project": async (inputs) => {
    requireInputs(inputs, 1, "gltf-resources-project");
    return { status: "ok", engine: TRANSFORM_ENGINE, probeVersion: TRANSFORM_PROBE_VERSION, measurements: await projectResources(inputs[0]!) };
  },
  "gltf-resources-compare": async (inputs) => {
    requireInputs(inputs, 2, "gltf-resources-compare");
    const expected = await projectResources(inputs[0]!);
    const actual = await projectResources(inputs[1]!);
    const diffs = diffPaths(canonicalize(expected), canonicalize(actual), "$");
    return {
      status: "ok",
      engine: TRANSFORM_ENGINE,
      probeVersion: TRANSFORM_PROBE_VERSION,
      measurements: { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 40) },
    };
  },
  "gltf-import": async (inputs) => {
    requireInputs(inputs, 1, "gltf-import");
    const gltf = await readGltf(inputs[0]!);
    const json = gltf.parser.json;
    return { status: "ok", measurements: { parsed: true, sceneCount: (json.scenes ?? []).length, nodeCount: (json.nodes ?? []).length, meshCount: (json.meshes ?? []).length, materialCount: (json.materials ?? []).length, assetVersion: json.asset?.version ?? null } };
  },
  "gltf-project": async (inputs) => {
    requireInputs(inputs, 1, "gltf-project");
    const gltf = await readGltf(inputs[0]!);
    return { status: "ok", measurements: projectDocument(gltf) };
  },
  "gltf-compare": async (inputs) => {
    requireInputs(inputs, 2, "gltf-compare");
    const expected = projectDocument(await readGltf(inputs[0]!));
    const actual = projectDocument(await readGltf(inputs[1]!));
    const diffs = diffPaths(canonicalize(expected), canonicalize(actual), "$");
    return { status: "ok", measurements: { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50), expected, actual } };
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
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: result.probeVersion ?? PROBE_VERSION, engine: result.engine ?? ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
