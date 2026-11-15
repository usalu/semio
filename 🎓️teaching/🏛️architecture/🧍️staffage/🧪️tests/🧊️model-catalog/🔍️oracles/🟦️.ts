/** 🔍️ The oracles of the catalog-wide 3D model guard: one set of checks for a freshly built export and for a file read back from the batch output, so the in-process guard and the validation of a whole output directory are the same code.
 *
 * Every authored scale of an asset is checked: a GLB holds one glTF scene per scale and mode (`s50-body`, `s50-thermal`, `s50-structure`, …) whose triangles, extents and bounding box are read back by three, a model JSON the same scenes, an FBX one scene per clip, scale and view, and the triangles of an FBX equal those of the matching GLB scene.
 *
 * THE REFERENCES (tests only, no runtime dependency): the Khronos `gltf-validator`, three's `GLTFLoader` and glTF-Transform for a GLB, three's `FBXLoader` and assimp (`assimpjs`) for an FBX, Ajv with the `semio.model/v1` JSON Schema for a model document, three's skinned extent against `dimensions`, and an independent weld-and-count of edges for watertightness.
 *
 * Every check returns {@link Finding}s (check code, item, message) and facts; nothing here asserts, so a caller decides what is a failure and which finding is a stated limitation.
 *
 * @see ../🟦️.ts — the guard
 * @see ../🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { scenesOf, tessellateSolid, validateModel, type ModelDocument, type Solid } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../../🧬️schema/🟦️.ts";
import { VIEWS, type View } from "../../../🔨️modules/🔗️contracts/🟦️.ts";
import { qualityOf } from "../../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { MODES, authoredScales, modeOf, modelOfAsset, sceneIdOf } from "../../../🔨️modules/🧊️volume/🟦️.ts";
import { resolveScenes } from "../../../🔨️modules/📤️export/🗿️model/🧩️scenes/🟦️.ts";
import { type Extent, loadFbx, loadGlb, sceneExtent, validateGlb } from "../../🧰️support/🧊️.ts";
import { NodeIO } from "@gltf-transform/core";

/** 🏷️ The checks a finding can belong to. */
export type Check = "model" | "geometry" | "closed" | "bounds" | "glb" | "variants" | "fbx" | "json" | "budget" | "determinism" | "set";

/** 🚩️ One failed expectation: the check, the item it concerns (a pose, a clip, a solid) and what is wrong. */
export type Finding = { readonly check: Check; readonly item: string; readonly message: string };

/** 📏️ The authoring gate of `volume-drafting`: at most 8 % per axis (at least 15 mm). */
export const extentTolerance = (length: number): number => Math.max(0.08 * length, 0.015);

/** 🎚️ The preset scales the budget is measured at. */
export const SCALES = [50, 100, 200, 500] as const;

/** 🚨️ The budget of a model: a GLB above 8 MB or an asset above 400 000 triangles at 1:50 is an outlier. */
export const BUDGET = { glbBytes: 8 * 1024 * 1024, triangles: 400_000 } as const;

/** 🔤️ The seven variants of every GLB in file order. */
export const VARIANT_ORDER = ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"] as const;

const schemaUrl = new URL("../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🔣️.json", import.meta.url);
let modelValidator: ReturnType<Ajv["compile"]> | undefined;

/** 🧬️ Ajv errors of a document against `semio.model/v1` (first six, as `pointer message`). */
export function schemaErrors(document: unknown): string[] {
  modelValidator ??= new Ajv({ strict: false, allErrors: true }).compile(JSON.parse(readFileSync(schemaUrl, "utf8")));
  return modelValidator(document) ? [] : (modelValidator.errors ?? []).slice(0, 6).map((e) => `${e.instancePath} ${e.message}`);
}

/** 🎞️ The triangles and line segments of one glTF scene: every primitive of every mesh of the nodes reachable from its root nodes. */
export type SceneFacts = { readonly triangles: number; readonly segments: number };

/** 🧾️ The JSON chunk of a GLB container and the triangle, line segment and primitive counts it describes, in total and per glTF scene. */
export function glbFacts(bytes: Uint8Array): { readonly json: any; readonly triangles: number; readonly segments: number; readonly primitives: number; readonly variants: readonly string[]; readonly scenes: Readonly<Record<string, SceneFacts>> } {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (view.getUint32(0, true) !== 0x46546c67 || view.getUint32(16, true) !== 0x4e4f534a) throw new Error("not a GLB container");
  const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + view.getUint32(12, true))));
  let triangles = 0;
  let segments = 0;
  let primitives = 0;
  const counts = (mesh: any): SceneFacts => {
    let tri = 0;
    let seg = 0;
    for (const primitive of mesh.primitives) {
      const count: number = json.accessors[primitive.indices].count;
      if (primitive.mode === undefined || primitive.mode === 4) tri += count / 3;
      else if (primitive.mode === 1) seg += count / 2;
    }
    return { triangles: tri, segments: seg };
  };
  for (const mesh of json.meshes ?? []) {
    primitives += mesh.primitives.length;
    const c = counts(mesh);
    triangles += c.triangles;
    segments += c.segments;
  }
  const scenes: Record<string, SceneFacts> = {};
  for (const scene of json.scenes ?? []) {
    let tri = 0;
    let seg = 0;
    const walk = (index: number): void => {
      const node = json.nodes[index];
      if (node.mesh !== undefined) {
        const c = counts(json.meshes[node.mesh]);
        tri += c.triangles;
        seg += c.segments;
      }
      for (const child of node.children ?? []) walk(child);
    };
    for (const root of scene.nodes ?? []) walk(root);
    scenes[scene.name ?? `scene-${Object.keys(scenes).length}`] = { triangles: tri, segments: seg };
  }
  return { json, triangles, segments, primitives, variants: (json.extensions?.KHR_materials_variants?.variants ?? []).map((v: { name: string }) => v.name), scenes };
}

/** 🏛️ The scene ids a model of an asset must hold: one per authored scale and mode, scales finest first, then body, thermal, structure. */
export const expectedScenes = (asset: Asset): readonly string[] => authoredScales(asset.volumes).flatMap((scale) => MODES.map((mode) => sceneIdOf(scale, mode)));

/** 📏️ The extent tolerance at a scale: the authoring gate, never tighter than the smallest feature of the scale (0.15 mm of paper). */
export const extentToleranceAt = (length: number, scale: number): number => Math.max(extentTolerance(length), 0.00015 * scale);

/** 🧊️ The facts and findings of one GLB: Khronos validation, three and glTF-Transform loads, seven variants, one glTF scene per authored scale and mode, extent per scene in the staffage axes. */
export type GlbReport = { readonly facts: { readonly bytes: number; readonly triangles: number; readonly segments: number; readonly errors: number; readonly warnings: number; readonly extent: Extent; readonly scenes: Readonly<Record<string, SceneFacts & { readonly extent: Extent }>> }; readonly findings: Finding[] };

/** 🛑️ Checks a GLB of one pose: 0 errors and 0 warnings from the validator, all seven variants, exactly the scenes `scenes` (default one implicit scene) in that order with triangles in each, scenes that three and glTF-Transform both read, finite extent per scene, no animation. */
export async function checkGlb(bytes: Uint8Array, name: string, item: string, scenes?: readonly string[]): Promise<GlbReport> {
  const findings: Finding[] = [];
  const report = await validateGlb(bytes, name);
  for (const message of report.messages) findings.push({ check: "glb", item, message: `${message.code} ${message.pointer ?? ""} ${message.message}` });
  const facts = glbFacts(bytes);
  if (facts.variants.join() !== VARIANT_ORDER.join()) findings.push({ check: "variants", item, message: `variants ${facts.variants.join(",")}` });
  if (!(facts.triangles > 0)) findings.push({ check: "glb", item, message: "no triangles" });
  if ((facts.json.animations ?? []).length > 0) findings.push({ check: "glb", item, message: "a static pose carries an animation" });
  if (scenes !== undefined) {
    const found = Object.keys(facts.scenes);
    if (found.join() !== scenes.join()) findings.push({ check: "glb", item, message: `glTF scenes ${found.join(",")}, expected ${scenes.join(",")}` });
    for (const scene of scenes) if (!((facts.scenes[scene]?.triangles ?? 0) > 0)) findings.push({ check: "glb", item, message: `scene ${scene} has no triangles` });
    if (facts.json.scene !== 0) findings.push({ check: "glb", item, message: `default scene ${facts.json.scene}, expected 0` });
  }
  try {
    const document = await new NodeIO().readBinary(bytes);
    const names = document.getRoot().listScenes().map((s) => s.getName());
    if (scenes !== undefined && names.join() !== scenes.join()) findings.push({ check: "glb", item, message: `glTF-Transform reads scenes ${names.join(",")}` });
  } catch (error) {
    findings.push({ check: "glb", item, message: `glTF-Transform: ${(error as Error).message}` });
  }
  const sceneFacts: Record<string, SceneFacts & { extent: Extent }> = {};
  let extent: Extent = { min: [NaN, NaN, NaN], max: [NaN, NaN, NaN] };
  try {
    const loaded = await loadGlb(bytes);
    extent = sceneExtent(loaded.scene);
    for (const [index, scene] of loaded.scenes.entries()) {
      const key = facts.json.scenes[index]?.name ?? `scene-${index}`;
      const own = sceneExtent(scene);
      sceneFacts[key] = { ...(facts.scenes[key] ?? { triangles: 0, segments: 0 }), extent: own };
      if (![...own.min, ...own.max].every(Number.isFinite)) findings.push({ check: "glb", item, message: `scene ${key} has a non-finite extent` });
    }
  } catch (error) {
    findings.push({ check: "glb", item, message: `three GLTFLoader: ${(error as Error).message}` });
  }
  if (![...extent.min, ...extent.max].every(Number.isFinite)) findings.push({ check: "glb", item, message: "extent is not finite" });
  return { facts: { bytes: bytes.length, triangles: facts.triangles, segments: facts.segments, errors: report.errors, warnings: report.warnings, extent, scenes: sceneFacts }, findings };
}

/** ⛔️ Extent against `dimensions`: the worst relative miss in units of the tolerance (at most 1 passes) and the signed relative error per axis (ahead, up, lateral). */
export function extentMiss(asset: Asset, extent: Extent, scale?: number): { readonly miss: number; readonly error: readonly [number, number, number] } {
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM] as const;
  const size = dims.map((_, at) => extent.max[at]! - extent.min[at]!);
  return { miss: Math.max(...dims.map((d, at) => Math.abs(size[at]! - d) / (scale === undefined ? extentTolerance(d) : extentToleranceAt(d, scale)))), error: size.map((s, at) => (s - dims[at]!) / dims[at]!) as [number, number, number] };
}

/** 🌟️ The A6 bounds verdict: the presentation pose decides; where it misses, the best-matching authored pose is reported (never the rest pose), and the asset passes only when the presentation pose is within the tolerance. */
export function boundsVerdict(asset: Asset, poses: readonly { readonly id: string; readonly extent: Extent }[]): { readonly pose: string; readonly error: readonly number[]; readonly miss: number; readonly best: string; readonly bestMiss: number; readonly ok: boolean } {
  const scored = poses.map((p) => ({ id: p.id, ...extentMiss(asset, p.extent) }));
  const mine = scored.find((s) => s.id === asset.presentation.defaultPose) ?? scored[0]!;
  const best = scored.reduce((a, b) => (b.miss < a.miss ? b : a));
  return { pose: mine.id, error: mine.error, miss: mine.miss, best: best.id, bestMiss: best.miss, ok: mine.miss <= 1 };
}

let assimp: Promise<{ FileList: new () => { AddFile(n: string, c: Uint8Array): void }; ConvertFileList(l: unknown, f: "assjson"): { IsSuccess(): boolean; GetErrorCode(): string; GetFile(i: number): { GetContent(): Uint8Array } } }> | undefined;
const assimpModule = async () => (assimp ??= (async () => (await import(/* @vite-ignore */ "assimpjs" as string) as { default: () => Promise<any> }).default())());

/** 🎬️ What an FBX of one clip must contain. */
export type FbxExpectation = { readonly clip: string; readonly durationS: number; readonly fps: number; readonly bones: readonly string[] };

/** 📤️ The facts and findings of one FBX read by three's FBXLoader and by assimp. */
export type FbxReport = { readonly facts: { readonly bytes: number; readonly triangles: number; readonly bonesFound: number; readonly threeBones: number; readonly meshBones: number; readonly tracks: number; readonly durationS: number; readonly assimpDurationS: number; readonly assimpChannels: number }; readonly findings: Finding[] };

type AssimpNode = { name: string; children?: AssimpNode[] };

/** 🧮️ Checks an FBX: both loaders accept it, one non-empty animation of the clip's duration (within a frame), every rig bone present as a node in both readers, finite keys. */
export async function checkFbx(bytes: Uint8Array, item: string, expect: FbxExpectation): Promise<FbxReport> {
  const findings: Finding[] = [];
  const wanted = new Set(expect.bones);
  const durationOk = (d: number): boolean => Math.abs(d - expect.durationS) <= Math.max(1e-3, 1e-6 * expect.durationS);
  let tracks = 0;
  let durationS = NaN;
  let threeBones = 0;
  let bonesFound = 0;
  let triangles = 0;
  try {
    const root = loadFbx(bytes);
    const names = new Set<string>();
    root.traverse((object: any) => {
      names.add(object.name);
      if (object.isBone) threeBones += 1;
      if (object.isMesh && object.geometry?.attributes?.position) triangles += (object.geometry.index?.count ?? object.geometry.attributes.position.count) / 3;
    });
    for (const bone of wanted) if (names.has(bone)) bonesFound += 1;
    if (bonesFound !== wanted.size) findings.push({ check: "fbx", item, message: `three finds ${bonesFound} of ${wanted.size} rig bones as nodes` });
    if (root.animations.length !== 1) findings.push({ check: "fbx", item, message: `three loads ${root.animations.length} animations` });
    const clip = root.animations[0];
    if (clip !== undefined) {
      tracks = clip.tracks.length;
      durationS = clip.duration;
      if (tracks === 0) findings.push({ check: "fbx", item, message: "three: empty animation" });
      if (!durationOk(durationS)) findings.push({ check: "fbx", item, message: `three duration ${durationS} s, clip ${expect.durationS} s` });
      for (const track of clip.tracks) {
        if (!Array.from(track.values as ArrayLike<number>).every(Number.isFinite) || !Array.from(track.times as ArrayLike<number>).every(Number.isFinite)) {
          findings.push({ check: "fbx", item, message: `three: non-finite values in ${track.name}` });
          break;
        }
      }
    }
    if (!(triangles > 0)) findings.push({ check: "fbx", item, message: "three: no triangles" });
  } catch (error) {
    findings.push({ check: "fbx", item, message: `three FBXLoader: ${(error as Error).message}` });
  }
  let assimpDurationS = NaN;
  let assimpChannels = 0;
  let meshBones = 0;
  try {
    const module = await assimpModule();
    const list = new module.FileList();
    list.AddFile("model.fbx", bytes);
    const result = module.ConvertFileList(list, "assjson");
    if (!result.IsSuccess()) findings.push({ check: "fbx", item, message: `assimp: ${result.GetErrorCode()}` });
    else {
      const scene = JSON.parse(new TextDecoder().decode(result.GetFile(0).GetContent())) as { rootnode: AssimpNode; meshes?: { bones?: unknown[] }[]; animations?: { name: string; duration: number; tickspersecond: number; channels: { positionkeys: [number, number[]][]; rotationkeys: [number, number[]][]; scalingkeys: [number, number[]][] }[] }[] };
      const names = new Set<string>();
      const walk = (node: AssimpNode): void => {
        names.add(node.name);
        node.children?.forEach(walk);
      };
      walk(scene.rootnode);
      const present = [...wanted].filter((bone) => names.has(bone)).length;
      if (present !== wanted.size) findings.push({ check: "fbx", item, message: `assimp finds ${present} of ${wanted.size} rig bones as nodes` });
      meshBones = (scene.meshes ?? []).reduce((n, m) => n + (m.bones?.length ?? 0), 0);
      const animation = scene.animations?.[0];
      if ((scene.animations ?? []).length !== 1 || animation === undefined) findings.push({ check: "fbx", item, message: `assimp loads ${(scene.animations ?? []).length} animations` });
      else {
        assimpChannels = animation.channels.length;
        assimpDurationS = animation.duration / (animation.tickspersecond || 1);
        if (assimpChannels === 0) findings.push({ check: "fbx", item, message: "assimp: empty animation" });
        if (!durationOk(assimpDurationS)) findings.push({ check: "fbx", item, message: `assimp duration ${assimpDurationS} s, clip ${expect.durationS} s` });
        const keys = animation.channels.flatMap((c) => [...c.positionkeys, ...c.rotationkeys, ...c.scalingkeys]);
        if (!keys.every(([t, v]) => Number.isFinite(t) && v.every(Number.isFinite))) findings.push({ check: "fbx", item, message: "assimp: non-finite animation key" });
      }
    }
  } catch (error) {
    findings.push({ check: "fbx", item, message: `assimp: ${(error as Error).message}` });
  }
  return { facts: { bytes: bytes.length, triangles, bonesFound, threeBones, meshBones, tracks, durationS, assimpDurationS, assimpChannels }, findings };
}

/** 🗿️ Checks a model document text: JSON, Ajv against `semio.model/v1`, no kernel validation error, seven variants, the rig bones as nodes and, when `scenes` is given, exactly those kernel scenes in that order. */
export function checkModelText(text: string, item: string, bones: readonly string[], expectAnimation?: string, scenes?: readonly string[]): { readonly facts: { readonly bytes: number; readonly nodes: number; readonly solids: number }; readonly findings: Finding[] } {
  const findings: Finding[] = [];
  let document: ModelDocument;
  try {
    document = JSON.parse(text) as ModelDocument;
  } catch (error) {
    return { facts: { bytes: text.length, nodes: 0, solids: 0 }, findings: [{ check: "json", item, message: `not JSON: ${(error as Error).message}` }] };
  }
  for (const error of schemaErrors(document)) findings.push({ check: "json", item, message: `ajv ${error}` });
  for (const issue of validateModel(document).filter((i) => i.severity === "error")) findings.push({ check: "json", item, message: `validateModel ${issue.code} ${issue.path}` });
  if (document.version !== "semio.model/v1") findings.push({ check: "json", item, message: `version ${document.version}` });
  if ([...document.variants.map((v) => v.id)].sort().join() !== [...VIEWS].sort().join()) findings.push({ check: "json", item, message: `${document.variants.length} variants` });
  if (scenes !== undefined && (document.scenes ?? []).map((s) => s.id).join() !== scenes.join()) findings.push({ check: "json", item, message: `scenes ${(document.scenes ?? []).map((s) => s.id).join(",")}, expected ${scenes.join(",")}` });
  const ids = new Set(document.nodes.map((n) => n.id));
  const missing = bones.filter((b) => !ids.has(b));
  if (missing.length > 0) findings.push({ check: "json", item, message: `missing bone nodes ${missing.slice(0, 3).join(",")}` });
  if (expectAnimation !== undefined && !document.animations.some((a) => a.id === expectAnimation)) findings.push({ check: "json", item, message: `no animation ${expectAnimation}` });
  return { facts: { bytes: text.length, nodes: document.nodes.length, solids: document.solids.length }, findings };
}

/** 🔬️ The geometry facts of one tessellated solid. */
export type SolidReport = { readonly id: string; readonly kind: string; readonly triangles: number; readonly degenerate: number; readonly nonFinite: number; readonly boundary: number; readonly overlapped: number; readonly volume: number; readonly closedExpected: boolean; readonly glyph: boolean };

const WELD = 1e-7;
const DEGENERATE_AREA = 1e-12;

/** 🔒️ Whether the kernel promises a closed solid for this kind and parameters: boxes, ellipsoids, capsules, extrusions, full revolves and lofts capped at both ends; a partial sweep or an uncapped loft is open by design. */
export function closedKind(solid: Solid): boolean {
  switch (solid.kind) {
    case "revolve": return solid.sweepRad === undefined || solid.sweepRad >= 2 * Math.PI - 1e-9;
    case "loft": return solid.capStart && solid.capEnd;
    default: return true;
  }
}

/** 🔄️ Welds the vertices of a solid (0.1 micrometre), then counts degenerate triangles, non-finite numbers, directed edges without their reverse (boundary), directed edges used more than once (overlap) and the signed volume (positive = outward). */
export function solidReport(solid: Solid, geometry: { readonly positions: readonly number[]; readonly indices: readonly number[] }): SolidReport {
  const ids = new Map<string, number>();
  const weld: number[] = [];
  let nonFinite = 0;
  for (let i = 0; i < geometry.positions.length; i += 3) {
    const p = [geometry.positions[i]!, geometry.positions[i + 1]!, geometry.positions[i + 2]!];
    if (!p.every(Number.isFinite)) nonFinite += 1;
    const key = p.map((v) => Math.round(v / WELD)).join(",");
    let id = ids.get(key);
    if (id === undefined) ids.set(key, (id = ids.size));
    weld.push(id);
  }
  const edges = new Map<number, number>();
  const stride = 2 ** 26;
  let degenerate = 0;
  let volume = 0;
  for (let t = 0; t < geometry.indices.length; t += 3) {
    const [a, b, c] = [geometry.indices[t]!, geometry.indices[t + 1]!, geometry.indices[t + 2]!];
    const A = [geometry.positions[a * 3]!, geometry.positions[a * 3 + 1]!, geometry.positions[a * 3 + 2]!];
    const B = [geometry.positions[b * 3]!, geometry.positions[b * 3 + 1]!, geometry.positions[b * 3 + 2]!];
    const C = [geometry.positions[c * 3]!, geometry.positions[c * 3 + 1]!, geometry.positions[c * 3 + 2]!];
    const u = [B[0]! - A[0]!, B[1]! - A[1]!, B[2]! - A[2]!];
    const v = [C[0]! - A[0]!, C[1]! - A[1]!, C[2]! - A[2]!];
    const area = 0.5 * Math.hypot(u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!);
    volume += (A[0]! * (B[1]! * C[2]! - B[2]! * C[1]!) - A[1]! * (B[0]! * C[2]! - B[2]! * C[0]!) + A[2]! * (B[0]! * C[1]! - B[1]! * C[0]!)) / 6;
    const [wa, wb, wc] = [weld[a]!, weld[b]!, weld[c]!];
    if (!(area >= DEGENERATE_AREA) || wa === wb || wb === wc || wa === wc) {
      degenerate += 1;
      continue;
    }
    for (const [from, to] of [[wa, wb], [wb, wc], [wc, wa]] as const) edges.set(from * stride + to, (edges.get(from * stride + to) ?? 0) + 1);
  }
  let boundary = 0;
  let overlapped = 0;
  for (const [key, count] of edges) {
    const from = Math.floor(key / stride);
    const to = key - from * stride;
    if (!edges.has(to * stride + from)) boundary += 1;
    if (count > 1) overlapped += 1;
  }
  return { id: solid.id, kind: solid.kind, triangles: geometry.indices.length / 3, degenerate, nonFinite, boundary, overlapped, volume, closedExpected: closedKind(solid), glyph: solid.tags?.includes("glyph") === true };
}

/** 🔺️ The facts of one scene of the multi-scene model: its solids, and the triangles at the quality of its own scale. */
export type SceneGeometry = { readonly scene: string; readonly scale: number; readonly mode: string; readonly solids: number; readonly glyphs: number; readonly triangles: number };

/** 🗂️ The geometry audit of an asset over every authored scale and scene: the procedural multi-scene model validated, every scene tessellated at the quality of its scale (the kernel's own path, then joined) and solid by solid, the closed-kind and degenerate counts, the triangles per scene, and that the triangles of a mode never grow towards coarser scales. */
export async function auditGeometry(asset: Asset): Promise<{ readonly facts: { readonly nodes: number; readonly solids: number; readonly glyphs: number; readonly scenes: readonly SceneGeometry[]; readonly byKind: Record<string, { solids: number; closed: number; open: number; closedExpected: number }>; readonly degenerate: number; readonly nonFinite: number }; readonly solids: readonly SolidReport[]; readonly findings: Finding[] }> {
  const findings: Finding[] = [];
  const document = await modelOfAsset(asset, { poses: "all", clips: [] });
  for (const issue of validateModel(document).filter((i) => i.severity === "error")) findings.push({ check: "model", item: asset.id, message: `validateModel ${issue.code} ${issue.path}` });
  const sceneIds = scenesOf(document).map((s) => s.id);
  if (sceneIds.join() !== expectedScenes(asset).join()) findings.push({ check: "model", item: asset.id, message: `scenes ${sceneIds.join(",")}, expected ${expectedScenes(asset).join(",")}` });
  const resolved = await resolveScenes(document, (scale) => qualityOf(asset, scale), { report: () => {}, cancelled: () => false });
  const scaleOf = (scene: string): number => Number(/^s(\d+)-/u.exec(scene)![1]);
  const reports: SolidReport[] = [];
  const scenes: SceneGeometry[] = [];
  for (const scene of scenesOf(document)) {
    const quality = qualityOf(asset, scaleOf(scene.id));
    const members = new Set(scene.nodes);
    let glyphs = 0;
    let triangles = 0;
    const own = document.solids.filter((s) => members.has(s.node));
    for (const solid of own) {
      const report = solidReport(solid, tessellateSolid(solid, quality));
      reports.push(report);
      triangles += report.triangles;
      if (report.glyph) glyphs += 1;
    }
    scenes.push({ scene: scene.id, scale: scaleOf(scene.id), mode: scene.tags?.find((tag) => MODES.includes(tag as never)) ?? "?", solids: own.length, glyphs, triangles });
  }
  let wholeTriangles = 0;
  for (const mesh of resolved.meshes) for (const primitive of mesh.primitives) {
    wholeTriangles += primitive.indices.length / 3;
    if (!primitive.positions.every(Number.isFinite) || !(primitive.normals ?? []).every(Number.isFinite)) findings.push({ check: "geometry", item: mesh.id, message: "non-finite position or normal after tessellation" });
  }
  const solidTriangles = reports.reduce((n, r) => n + r.triangles, 0);
  if (wholeTriangles !== solidTriangles) findings.push({ check: "geometry", item: asset.id, message: `tessellate gives ${wholeTriangles} triangles, solid by solid ${solidTriangles}` });
  const byKind: Record<string, { solids: number; closed: number; open: number; closedExpected: number }> = {};
  for (const r of reports) {
    const k = (byKind[r.kind] ??= { solids: 0, closed: 0, open: 0, closedExpected: 0 });
    k.solids += 1;
    const closed = r.boundary === 0 && r.overlapped === 0;
    if (closed) k.closed += 1;
    else k.open += 1;
    if (r.closedExpected) k.closedExpected += 1;
    if (r.nonFinite > 0) findings.push({ check: "geometry", item: r.id, message: `${r.nonFinite} non-finite vertices` });
    if (r.degenerate > 0) findings.push({ check: "geometry", item: r.id, message: `${r.degenerate} degenerate triangles of ${r.triangles}` });
    if (r.closedExpected && !closed) findings.push({ check: "closed", item: r.id, message: `${r.kind} is not watertight: ${r.boundary} boundary and ${r.overlapped} overlapped edges` });
    if (r.closedExpected && closed && !(r.volume > 0)) findings.push({ check: "closed", item: r.id, message: `${r.kind} has signed volume ${r.volume} (inside out)` });
  }
  for (const mode of MODES) {
    const row = scenes.filter((s) => s.mode === mode).sort((a, b) => a.scale - b.scale);
    for (let i = 1; i < row.length; i++) if (row[i]!.triangles > row[i - 1]!.triangles) findings.push({ check: "budget", item: asset.id, message: `${mode} triangles grow from 1:${row[i - 1]!.scale} (${row[i - 1]!.triangles}) to 1:${row[i]!.scale} (${row[i]!.triangles})` });
    const finest = row[0];
    if (finest !== undefined && finest.triangles > BUDGET.triangles) findings.push({ check: "budget", item: asset.id, message: `${finest.triangles} ${mode} triangles at 1:${finest.scale} (budget ${BUDGET.triangles})` });
  }
  const glyphs = reports.filter((r) => r.glyph).length;
  return { facts: { nodes: document.nodes.length, solids: document.solids.length, glyphs, scenes, byKind, degenerate: reports.reduce((n, r) => n + r.degenerate, 0), nonFinite: reports.reduce((n, r) => n + r.nonFinite, 0) }, solids: reports, findings };
}

/** 🗜️ The member names of a ZIP read from its central directory (so a duplicated name is seen, which an unzip into a map hides). */
export function zipNames(bytes: Uint8Array): string[] {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let end = bytes.length - 22;
  while (end >= 0 && view.getUint32(end, true) !== 0x06054b50) end -= 1;
  if (end < 0) throw new Error("no end of central directory");
  const count = view.getUint16(end + 10, true);
  let at = view.getUint32(end + 16, true);
  const names: string[] = [];
  for (let i = 0; i < count; i++) {
    const length = view.getUint16(at + 28, true);
    const extra = view.getUint16(at + 30, true);
    const comment = view.getUint16(at + 32, true);
    names.push(new TextDecoder().decode(bytes.subarray(at + 46, at + 46 + length)));
    at += 46 + length + extra + comment;
  }
  return names;
}

/** 📁️ One file of a set as the manifest lists it. */
export type SetFile = { readonly asset: string; readonly subject: "pose" | "clip" | "asset"; readonly item: string; readonly scale: number | null; readonly view: string; readonly kind: "glb" | "fbx" | "model" | "zip"; readonly file: string; readonly bytes: number; readonly sha256: string; readonly triangles: number | null };

/** 📂️ The audit of the files of one asset in an output directory: manifest bytes and hashes, every pose GLB, every clip FBX, every model JSON, the ZIP and the budgets. */
export type FilesAudit = {
  readonly poses: readonly { readonly id: string; readonly bytes: number; readonly triangles: number; readonly segments: number; readonly extent: Extent }[];
  readonly clips: readonly { readonly id: string; readonly bytes: number; readonly triangles: number; readonly tracks: number; readonly durationS: number; readonly bonesFound: number; readonly meshBones: number; readonly assimpChannels: number }[];
  readonly models: number;
  readonly zipBytes: number;
  readonly bounds: ReturnType<typeof boundsVerdict>;
  readonly bytes: number;
  readonly findings: Finding[];
};

/** 🧪️ Reads the files of `asset` from `root` (paths as the manifest gives them, `/` separated) and checks each with the oracles above. `read` returns the bytes of a path under the root. */
export async function auditFiles(asset: Asset, files: readonly SetFile[], read: (file: string) => Uint8Array, plannedNames: readonly { readonly subject: string; readonly item: string; readonly kind: string; readonly name: string; readonly scale?: number | null; readonly view?: string }[]): Promise<FilesAudit> {
  const findings: Finding[] = [];
  const decoder = new TextDecoder();
  const bones = asset.rig.bones.map((b) => b.id);
  const mine = files.filter((f) => f.asset === asset.id);
  const byName = new Map(mine.map((f) => [f.file.slice(asset.id.length + 1), f]));
  const expectedNames = plannedNames.map((p) => p.name);
  const missing = expectedNames.filter((n) => !byName.has(n));
  const extra = [...byName.keys()].filter((n) => !expectedNames.includes(n));
  if (missing.length > 0) findings.push({ check: "set", item: asset.id, message: `manifest lacks ${missing.join(", ")}` });
  if (extra.length > 0) findings.push({ check: "set", item: asset.id, message: `manifest lists unplanned ${extra.join(", ")}` });
  const scenes = expectedScenes(asset);
  const sceneTriangles = new Map<string, number>();
  const poses: { id: string; bytes: number; triangles: number; segments: number; extent: Extent }[] = [];
  const clips: FilesAudit["clips"][number][] = [];
  let models = 0;
  let zipBytes = 0;
  let total = 0;
  const members = new Map<string, Uint8Array>();
  for (const planned of plannedNames) {
    const entry = byName.get(planned.name);
    if (entry === undefined) continue;
    const item = `${planned.subject}:${planned.item}`;
    let bytes: Uint8Array;
    try {
      bytes = read(entry.file);
    } catch (error) {
      findings.push({ check: "set", item, message: `unreadable ${entry.file}: ${(error as Error).message}` });
      continue;
    }
    total += bytes.length;
    if (bytes.length !== entry.bytes) findings.push({ check: "set", item, message: `${entry.file} has ${bytes.length} bytes, manifest ${entry.bytes}` });
    const { createHash } = await import("node:crypto");
    if (createHash("sha256").update(bytes).digest("hex") !== entry.sha256) findings.push({ check: "set", item, message: `${entry.file} does not hash to the manifest` });
    if (entry.file.length + 1 > MAX_PATH) findings.push({ check: "set", item, message: `relative path of ${entry.file.length} characters` });
    members.set(`${planned.kind === "zip" ? "" : planned.kind}/${planned.name}`, bytes);
    if (planned.kind === "glb") {
      const report = await checkGlb(bytes, planned.name, item, scenes);
      findings.push(...report.findings);
      const finest = authoredScales(asset.volumes)[0];
      const body = finest === undefined ? undefined : report.facts.scenes[sceneIdOf(finest, "body")];
      poses.push({ id: planned.item, bytes: bytes.length, triangles: report.facts.triangles, segments: report.facts.segments, extent: body?.extent ?? report.facts.extent });
      if (planned.item === asset.presentation.defaultPose || sceneTriangles.size === 0) {
        sceneTriangles.clear();
        for (const [scene, facts] of Object.entries(report.facts.scenes)) sceneTriangles.set(scene, facts.triangles);
      }
      if (planned.item === asset.presentation.defaultPose) {
        for (const scale of authoredScales(asset.volumes)) {
          const own = report.facts.scenes[sceneIdOf(scale, "body")];
          if (own === undefined) continue;
          const miss = extentMiss(asset, own.extent, scale);
          if (miss.miss > 1) findings.push({ check: "bounds", item, message: `1:${scale} body scene misses dimensions by ${miss.error.map((e) => `${(e * 100).toFixed(1)} %`).join(" / ")}` });
        }
      }
      if (report.facts.triangles !== entry.triangles) findings.push({ check: "set", item, message: `manifest says ${entry.triangles} triangles, file ${report.facts.triangles}` });
      if (bytes.length > BUDGET.glbBytes) findings.push({ check: "budget", item, message: `GLB of ${(bytes.length / 1048576).toFixed(1)} MB (budget 8 MB)` });
    } else if (planned.kind === "fbx") {
      const clip = asset.clips.find((c) => c.id === planned.item)!;
      const report = await checkFbx(bytes, item, { clip: clip.id, durationS: clip.durationS, fps: clip.fps, bones });
      findings.push(...report.findings);
      const glbScene = planned.scale === null || planned.scale === undefined || modeOf(planned.view as View) === "structure" ? undefined : sceneTriangles.get(sceneIdOf(planned.scale, modeOf(planned.view as View)));
      if (glbScene !== undefined && Math.round(report.facts.triangles) !== Math.round(glbScene)) findings.push({ check: "set", item, message: `FBX at 1:${planned.scale} ${planned.view} has ${report.facts.triangles} triangles, the GLB scene ${glbScene}` });
      clips.push({ id: clip.id, bytes: bytes.length, triangles: report.facts.triangles, tracks: report.facts.tracks, durationS: report.facts.durationS, bonesFound: report.facts.bonesFound, meshBones: report.facts.meshBones, assimpChannels: report.facts.assimpChannels });
      if (report.facts.triangles !== entry.triangles) findings.push({ check: "set", item, message: `manifest says ${entry.triangles} triangles, file ${report.facts.triangles}` });
    } else if (planned.kind === "model") {
      const report = checkModelText(decoder.decode(bytes), item, bones, planned.subject === "clip" ? planned.item : undefined, scenes);
      findings.push(...report.findings);
      models += 1;
    } else {
      zipBytes = bytes.length;
    }
  }
  const zipPlan = plannedNames.find((p) => p.kind === "zip");
  const zipFile = zipPlan === undefined ? undefined : byName.get(zipPlan.name);
  if (zipFile !== undefined) {
    const names = zipNames(read(zipFile.file));
    const wanted = plannedNames.filter((p) => p.kind !== "zip").map((p) => `${p.kind}/${p.name}`);
    if (new Set(names).size !== names.length) findings.push({ check: "set", item: "zip", message: `duplicate members: ${names.filter((n, i) => names.indexOf(n) !== i).join(", ")}` });
    if ([...names].sort().join("|") !== [...wanted].sort().join("|")) findings.push({ check: "set", item: "zip", message: `members differ: ${names.length} found, ${wanted.length} expected` });
    const { unzipSync } = await import("fflate");
    const unzipped = unzipSync(read(zipFile.file));
    for (const name of wanted) {
      const inFile = members.get(name);
      const inZip = unzipped[name];
      if (inFile === undefined || inZip === undefined || inFile.length !== inZip.length || !inFile.every((v, i) => v === inZip[i])) findings.push({ check: "set", item: name, message: "zip member differs from its file" });
    }
  }
  const bounds = boundsVerdict(asset, poses);
  if (!bounds.ok) findings.push({ check: "bounds", item: asset.id, message: `default pose ${bounds.pose} misses dimensions by ${bounds.error.map((e) => `${(e * 100).toFixed(1)} %`).join(" / ")} (best pose ${bounds.best}, miss ${bounds.bestMiss.toFixed(2)})` });
  return { poses, clips, models, zipBytes, bounds, bytes: total, findings };
}

const MAX_PATH = 256;

/** 🔺️ The parts of a parsed glTF the glyph check reads. */
export type GlbJson = { nodes: any[]; meshes: any[]; materials: any[]; extensions: { KHR_materials_variants: { variants: { name: string }[] } } };

const alphaOf = (json: GlbJson, material: number): number => json.materials[material].pbrMetallicRoughness?.baseColorFactor?.[3] ?? 1;

/** 👁️ Per variant, the glyph primitives that draw something: `triangles` of glyph surfaces and `lines` of glyph edges, each keyed by the glyph solid ids of its provenance. */
export function visibleGlyphs(json: GlbJson): Record<string, { triangles: Set<string>; lines: Set<string> }> {
  const glyphMeshes = new Set(json.nodes.filter((n) => n.mesh !== undefined && n.extras?.tags?.includes("glyph")).map((n) => n.mesh));
  const names = json.extensions.KHR_materials_variants.variants.map((v) => v.name);
  const out: Record<string, { triangles: Set<string>; lines: Set<string> }> = {};
  for (const [index, name] of names.entries()) {
    const entry = (out[name] = { triangles: new Set(), lines: new Set() });
    for (const mesh of glyphMeshes) {
      for (const primitive of json.meshes[mesh].primitives) {
        const mapping = (primitive.extensions?.KHR_materials_variants?.mappings ?? []).find((m: { variants: number[] }) => m.variants.includes(index));
        const material = mapping?.material ?? primitive.material;
        if (!(alphaOf(json, material) > 0)) continue;
        for (const solid of primitive.extras?.solids ?? [{ id: `mesh-${mesh}` }]) (primitive.mode === 1 ? entry.lines : entry.triangles).add(solid.id);
      }
    }
  }
  return out;
}


/** 📦️ The structure digest of a GLB that any implementation must reproduce: sizes, hash and the counts of the glTF JSON. */
export function glbDigest(bytes: Uint8Array, sha256: (bytes: Uint8Array) => string): Record<string, unknown> {
  const facts = glbFacts(bytes);
  const j = facts.json;
  return { byteLength: bytes.length, sha256: sha256(bytes), triangles: facts.triangles, segments: facts.segments, primitives: facts.primitives, variants: facts.variants, extensionsUsed: j.extensionsUsed ?? [], nodes: j.nodes.length, meshes: (j.meshes ?? []).length, materials: (j.materials ?? []).length, skins: (j.skins ?? []).length, animations: (j.animations ?? []).length, accessors: j.accessors.length };
}

/** 🗃️ The structure digest of an FBX that any implementation must reproduce: sizes, hash, the object counts of the file, the number of animation curve nodes and the notices of the encoder. */
export function fbxDigest(bytes: Uint8Array, notices: readonly string[], sha256: (bytes: Uint8Array) => string, counts: Record<string, number>): Record<string, unknown> {
  return { byteLength: bytes.length, sha256: sha256(bytes), objectCounts: counts, notices: [...notices] };
}
