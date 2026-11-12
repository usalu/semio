/** 🧊️ The structure digests of the GLB and FBX exports of the sample figure: per pose the byte length, SHA-256 and glTF counts, per clip the byte length, SHA-256, the FBX object counts and the notices, as language-agnostic vectors under `🧫️fixtures/🧊️model-digests`.
 *
 * THE REFERENCES are the Khronos `gltf-validator` and three's `GLTFLoader` for each GLB, three's `FBXLoader` and assimp (`assimpjs`) for each FBX, node's SHA-256, and the kernel's own binary FBX reader for the object counts. A second implementation reproduces the counts and the loaded structure (the bytes belong to this encoder).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import { objectCounts, readFbx } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts";
import { writeFbxDetailed } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { checkFbx, checkGlb, fbxDigest, glbDigest } from "../🧊️model-catalog/🔍️oracles/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { sampleFigure } from "../🧰️support/🧊️.ts";

const sha256 = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
type Vectors = { readonly scale: number; readonly glb: Record<string, Record<string, unknown>>; readonly fbx: Record<string, Record<string, unknown>> };
const vectors = fixture<Vectors>("🧊️model-digests");
const asset = sampleFigure();
const poses = asset.poses.filter((p) => p.key !== true).map((p) => p.id);

/** 🧬️ The digests of the sample figure, as the fixture records them. */
export async function digestsOfSampleFigure(): Promise<Vectors> {
  const glb: Vectors["glb"] = {};
  for (const pose of poses) glb[pose] = glbDigest(await writeGlb(asset, { pose, scale: vectors.scale }), sha256);
  const fbx: Vectors["fbx"] = {};
  for (const clip of asset.clips) {
    const result = await writeFbxDetailed(asset, { clip: clip.id, scale: vectors.scale });
    fbx[clip.id] = fbxDigest(result.bytes, result.notices.map((n) => `${n.code}:${n.subject}`), sha256, objectCounts(readFbx(result.bytes)));
  }
  return { scale: vectors.scale, glb, fbx };
}

describe("🧊️ GLB and FBX structure digests of the sample figure", () => {
  it("covers every pose and clip", () => {
    expect(Object.keys(vectors.glb).sort()).toEqual([...poses].sort());
    expect(Object.keys(vectors.fbx).sort()).toEqual(asset.clips.map((c) => c.id).sort());
  });

  it.each(poses)("pose %s: the GLB reproduces its digest and passes the Khronos validator and three's GLTFLoader", async (pose) => {
    const bytes = await writeGlb(asset, { pose, scale: vectors.scale });
    expect(glbDigest(bytes, sha256)).toEqual(vectors.glb[pose]);
    const report = await checkGlb(bytes, `${pose}.glb`, pose);
    expect(report.findings).toEqual([]);
    expect(report.facts.errors).toBe(0);
    expect(report.facts.warnings).toBe(0);
  });

  it.each(asset.clips.map((c) => [c.id, c] as const))("clip %s: the FBX reproduces its digest and loads in three's FBXLoader and in assimp with every rig bone", async (id, clip) => {
    const result = await writeFbxDetailed(asset, { clip: id, scale: vectors.scale });
    expect(fbxDigest(result.bytes, result.notices.map((n) => `${n.code}:${n.subject}`), sha256, objectCounts(readFbx(result.bytes)))).toEqual(vectors.fbx[id]);
    const report = await checkFbx(result.bytes, id, { clip: id, durationS: clip.durationS, fps: clip.fps, bones: asset.rig.bones.map((b) => b.id) });
    expect(report.findings).toEqual([]);
  });
});
