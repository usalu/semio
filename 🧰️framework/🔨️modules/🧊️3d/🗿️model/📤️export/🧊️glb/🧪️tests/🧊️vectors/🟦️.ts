/** 🧊️ Law of the owned GLB encoder (`../../🟦️.ts`) against the language-agnostic vectors in `../../🧫️fixtures/<case>/🔣️.json`:
 * exact JSON chunk, accessor counts, chunk byte lengths and SHA-256 of the bytes, container invariants (4-byte alignment, chunk framing), determinism, progress, cancellation and rejection of procedural or dangling documents.
 * The independent third-party oracles (Khronos validator, glTF-Transform and three round trips) are in `../🧪️oracles/🟦️.ts`.
 */
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";

import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { CancelledError } from "../../../../🔺️tessellation/🟦️.ts";
import { encodeGlb } from "../../🟦️.ts";
import type { GlbOptions } from "../../🟦️.ts";
import { quiet, vectors as loadVectors } from "../🧰️support/🟦️.ts";

type Vector = { schema: "semio.model.glb-vector/v1"; name: string; document: ModelDocument; options: GlbOptions; expected: { glbByteLength: number; jsonChunkByteLength: number; binChunkByteLength: number; sha256: string; accessorCounts: string[]; json: Record<string, unknown> } };

const vectors = loadVectors<Vector>();

const parse = (bytes: Uint8Array): { json: Record<string, any>; jsonLength: number; binLength: number } => {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  expect(view.getUint32(0, true)).toBe(0x46546c67);
  expect(view.getUint32(4, true)).toBe(2);
  expect(view.getUint32(8, true)).toBe(bytes.length);
  const jsonLength = view.getUint32(12, true);
  expect(view.getUint32(16, true)).toBe(0x4e4f534a);
  const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonLength)));
  let binLength = 0;
  if (bytes.length > 20 + jsonLength) {
    binLength = view.getUint32(20 + jsonLength, true);
    expect(view.getUint32(24 + jsonLength, true)).toBe(0x004e4942);
    expect(28 + jsonLength + binLength).toBe(bytes.length);
  }
  return { json, jsonLength, binLength };
};

const vertexCount = (document: ModelDocument): number[] => document.meshes.flatMap((mesh) => mesh.primitives.map((primitive) => primitive.positions.length / 3));

describe("🧊️ GLB encoder vectors", () => {
  it("carries the closed vector corpus", () => {
    expect(vectors.map((vector) => vector.name)).toEqual(["minimal-triangle", "rig-bent-pose", "rig-full", "rig-wave-quarter"]);
  });

  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s encodes to the recorded structure and bytes", (_name, vector) => {
    const bytes = encodeGlb(vector.document, vector.options, quiet);
    const { json, jsonLength, binLength } = parse(bytes);
    expect(json).toEqual(vector.expected.json);
    expect(bytes.length).toBe(vector.expected.glbByteLength);
    expect(jsonLength).toBe(vector.expected.jsonChunkByteLength);
    expect(binLength).toBe(vector.expected.binChunkByteLength);
    expect(createHash("sha256").update(bytes).digest("hex")).toBe(vector.expected.sha256);
    expect(json.accessors.map((a: { type: string; count: number }) => `${a.type}:${a.count}`)).toEqual(vector.expected.accessorCounts);
  });

  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s is deterministic and aligned", (_name, vector) => {
    const first = encodeGlb(vector.document, vector.options, quiet);
    const second = encodeGlb(structuredClone(vector.document), { ...vector.options }, quiet);
    expect(Buffer.from(first).equals(Buffer.from(second))).toBe(true);
    const { json, jsonLength, binLength } = parse(first);
    expect(jsonLength % 4).toBe(0);
    expect(binLength % 4).toBe(0);
    for (const view of json.bufferViews ?? []) expect(view.byteOffset % 4).toBe(0);
    expect(json.buffers?.[0]?.byteLength ?? 0).toBe(binLength);
  });

  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s counts accessors from the source document", (_name, vector) => {
    const { json } = parse(encodeGlb(vector.document, vector.options, quiet));
    const counts = vertexCount(vector.document);
    const positions = json.meshes.flatMap((mesh: any) => mesh.primitives.map((p: any) => json.accessors[p.attributes.POSITION]));
    expect(positions.map((a: any) => a.count)).toEqual(counts);
    for (const accessor of positions) {
      expect(accessor.min).toHaveLength(3);
      expect(accessor.max).toHaveLength(3);
    }
    const indices = json.meshes.flatMap((mesh: any) => mesh.primitives.map((p: any) => json.accessors[p.indices].count));
    expect(indices).toEqual(vector.document.meshes.flatMap((mesh) => mesh.primitives.map((p) => p.indices.length)));
    expect(json.extensionsUsed ?? []).toEqual([...(json.extensionsUsed ?? [])].sort());
  });

  it("writes the declared extensions only when used", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    expect(parse(encodeGlb(full.document, full.options, quiet)).json.extensionsUsed).toEqual(["KHR_animation_pointer", "KHR_materials_unlit", "KHR_materials_variants"]);
    expect(parse(encodeGlb(full.document, { animations: false, variants: false }, quiet)).json.extensionsUsed).toEqual(["KHR_materials_unlit"]);
    const minimal = vectors.find((vector) => vector.name === "minimal-triangle")!;
    expect(parse(encodeGlb(minimal.document, minimal.options, quiet)).json.extensionsUsed).toBeUndefined();
  });

  it("types vertex attributes and indices by their range", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const { json } = parse(encodeGlb(full.document, full.options, quiet));
    const primitive = json.meshes[0].primitives[0];
    expect(json.accessors[primitive.attributes.JOINTS_0].componentType).toBe(5121);
    expect(json.accessors[primitive.attributes.WEIGHTS_0].componentType).toBe(5126);
    expect(json.accessors[primitive.indices].componentType).toBe(5123);
    expect(json.meshes[1].primitives[0].mode).toBe(1);
    const big = structuredClone(full.document) as unknown as { meshes: { primitives: { indices: number[]; positions: number[] }[] }[] };
    const count = 70000;
    big.meshes[1]!.primitives[0]!.positions = new Array(count * 3).fill(0);
    big.meshes[1]!.primitives[0]!.indices = [0, count - 1];
    delete (big.meshes[1]!.primitives[0] as { solids?: unknown }).solids;
    const wide = parse(encodeGlb(big as unknown as ModelDocument, full.options, quiet)).json;
    expect(wide.accessors[wide.meshes[1].primitives[0].indices].componentType).toBe(5125);
  });

  it("bakes a stored pose and a clip time into node transforms", () => {
    const bent = vectors.find((vector) => vector.name === "rig-bent-pose")!;
    const { json } = parse(encodeGlb(bent.document, bent.options, quiet));
    expect(json.nodes[2].rotation[2]).toBeCloseTo(Math.SQRT1_2, 12);
    expect(json.nodes[2].rotation[3]).toBeCloseTo(Math.SQRT1_2, 12);
    expect(json.nodes[4].translation).toEqual([0.5, 0, 0]);
    const wave = vectors.find((vector) => vector.name === "rig-wave-quarter")!;
    const sampled = parse(encodeGlb(wave.document, wave.options, quiet)).json;
    const [x, y, z, w] = sampled.nodes[2].rotation as number[];
    const half = Math.PI / 8;
    expect(x).toBeCloseTo(0, 12);
    expect(y).toBeCloseTo(0, 12);
    expect(z).toBeCloseTo(Math.sin(half), 12);
    expect(w).toBeCloseTo(Math.cos(half), 12);
    expect(sampled.nodes[1].translation).toBeUndefined();
    expect(sampled.nodes[4].scale).toEqual([1.25, 1.25, 1.25]);
    expect(sampled.materials[0].pbrMetallicRoughness.baseColorFactor[0]).toBeCloseTo(0.9 - 0.4 * 0.25, 12);
    expect(sampled.materials[0].emissiveFactor).toBeUndefined();
    expect(sampled.animations).toBeUndefined();
  });

  it("hoists skinned mesh nodes to scene roots and keeps their source parent", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const { json } = parse(encodeGlb(full.document, full.options, quiet));
    expect(json.scenes[0].nodes).toEqual([0, 3]);
    expect(json.nodes[0].children).toEqual([1, 4]);
    expect(json.nodes[3].extras.parent).toBe("root");
    expect(json.nodes[4].extras).toBeUndefined();
  });

  it("keeps titles in both languages and the mechanism in extras", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const { json } = parse(encodeGlb(full.document, full.options, quiet));
    expect(json.asset.extras.title).toEqual({ en: "Rig", de: "Gerüst" });
    expect(json.nodes[2].extras.joint).toEqual({ type: "revolute", axis: [0, 0, 1], min: -2, max: 2 });
    expect(json.materials[0].extras.title).toEqual({ en: "Skin", de: "Haut" });
    expect(json.animations[0].extras).toEqual({ title: { en: "Wave", de: "Winken" }, durationS: 1, loop: true, events: [{ t: 0.5, id: "peak" }] });
    expect(json.extensions.KHR_materials_variants.variants.map((v: any) => v.name)).toEqual(["light-solid", "dark-solid"]);
  });

  it("writes the solid provenance of a primitive to its extras and nothing without it", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const { json } = parse(encodeGlb(full.document, full.options, quiet));
    expect(json.meshes[0].primitives[0].extras).toEqual({ solids: [{ id: "torso", first: 0, count: 6 }, { id: "belly", first: 6, count: 6 }] });
    expect(json.meshes[1].primitives[0].extras).toEqual({ solids: [{ id: "torso", first: 0, count: 2 }, { id: "belly", first: 2, count: 2 }] });
    const minimal = vectors.find((vector) => vector.name === "minimal-triangle")!;
    expect(parse(encodeGlb(minimal.document, minimal.options, quiet)).json.meshes[0].primitives[0].extras).toBeUndefined();
  });

  it("rejects solid ranges that are not contiguous, whole primitives or covering", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const broken = (solids: unknown): ModelDocument => {
      const doc = structuredClone(full.document) as unknown as { meshes: { primitives: { solids: unknown }[] }[] };
      doc.meshes[0]!.primitives[0]!.solids = solids;
      return doc as unknown as ModelDocument;
    };
    expect(() => encodeGlb(broken([{ id: "a", first: 0, count: 6 }, { id: "b", first: 7, count: 5 }]), full.options, quiet)).toThrow(/contiguous/);
    expect(() => encodeGlb(broken([{ id: "a", first: 0, count: 4 }, { id: "b", first: 4, count: 8 }]), full.options, quiet)).toThrow(/contiguous/);
    expect(() => encodeGlb(broken([{ id: "a", first: 0, count: 6 }]), full.options, quiet)).toThrow(/cover all 12 indices/);
  });

  it("reports monotonic progress ending at one and honours cancellation", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const seen: number[] = [];
    encodeGlb(full.document, full.options, { report: (f) => void seen.push(f), cancelled: () => false });
    expect(seen[0]).toBe(0);
    expect(seen.at(-1)).toBe(1);
    expect([...seen].sort((a, b) => a - b)).toEqual(seen);
    let calls = 0;
    expect(() => encodeGlb(full.document, full.options, { report: () => undefined, cancelled: () => ++calls > 3 })).toThrow(CancelledError);
  });

  it("rejects procedural documents and dangling references", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const procedural = { ...structuredClone(full.document), meshes: [], solids: [{ id: "b", kind: "box", node: "root", material: "skin", size: [1, 1, 1] }] };
    expect(() => encodeGlb(procedural as unknown as ModelDocument, full.options, quiet)).toThrow(/procedural/);
    const dangling = structuredClone(full.document) as unknown as { meshes: { primitives: { material: string }[] }[] };
    dangling.meshes[0]!.primitives[0]!.material = "nope";
    expect(() => encodeGlb(dangling as unknown as ModelDocument, full.options, quiet)).toThrow(/unknown material nope/);
    expect(() => encodeGlb(full.document, { pose: "missing", animations: false, variants: false }, quiet)).toThrow(/pose missing/);
    const badTimes = structuredClone(full.document) as unknown as { animations: { channels: { times: number[] }[] }[] };
    badTimes.animations[0]!.channels[0]!.times = [0, 0.5, 0.5];
    expect(() => encodeGlb(badTimes as unknown as ModelDocument, full.options, quiet)).toThrow(/strictly increasing/);
  });

  it("normalises skin weights and zero-weight joints", () => {
    const full = vectors.find((vector) => vector.name === "rig-full")!;
    const doc = structuredClone(full.document) as unknown as { meshes: { primitives: { weights: number[]; joints: number[] }[] }[] };
    doc.meshes[0]!.primitives[0]!.weights = [2, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 3, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
    const bytes = encodeGlb(doc as unknown as ModelDocument, full.options, quiet);
    const { json } = parse(bytes);
    const accessor = json.accessors[json.meshes[0].primitives[0].attributes.WEIGHTS_0];
    const view = json.bufferViews[accessor.bufferView];
    const jsonLength = new DataView(bytes.buffer, bytes.byteOffset).getUint32(12, true);
    const data = new DataView(bytes.buffer, bytes.byteOffset + 28 + jsonLength + view.byteOffset, view.byteLength);
    const weights = Array.from({ length: 24 }, (_, i) => data.getFloat32(i * 4, true));
    for (let v = 0; v < 6; v += 1) expect(weights.slice(v * 4, v * 4 + 4).reduce((a, b) => a + b, 0)).toBeCloseTo(1, 6);
    expect(weights.slice(0, 4)).toEqual([1, 0, 0, 0]);
    expect(weights[4]).toBe(1);
    expect(weights[12]).toBeCloseTo(0.25, 6);
    expect(weights[13]).toBeCloseTo(0.75, 6);
  });
});
