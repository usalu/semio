/** 🧊️ The structure digests of the GLB and FBX exports of the sample figure: per pose the byte length, SHA-256 and glTF counts, per clip the byte length, SHA-256, the FBX object counts and the notices, as language-agnostic vectors under `🧫️fixtures/🧊️model-digests`.
 *
 * THE REFERENCES are the Khronos `gltf-validator` and three's `GLTFLoader` for each GLB, three's `FBXLoader` and assimp (`assimpjs`) for each FBX, node's SHA-256, and the kernel's own binary FBX reader for the object counts. A second implementation reproduces the counts and the loaded structure (the bytes belong to this encoder).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { describe, expect, it } from "vitest";
import { fbxName } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { writeFbxDetailed } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { authoredScales } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { checkFbx, checkGlb, expectedScenes } from "../🧊️model-catalog/🔍️oracles/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { DIGEST_VIEWS, type Digests, fbxDigestOf, glbDigestOf } from "../🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const vectors = fixture<Digests>("🧊️model-digests");
const asset = scaledFigure();
const poses = asset.poses.filter((p) => p.key !== true).map((p) => p.id);
const fbxCases = asset.clips.flatMap((clip) => authoredScales(asset.volumes).flatMap((scale) => DIGEST_VIEWS.map((view) => [fbxName(asset.id, clip.id, scale, view), clip, scale, view] as const)));

describe("🧊️ GLB and FBX structure digests of the four-scale sample figure", () => {
  it("covers every pose, and every clip at every authored scale and checked view", () => {
    expect(Object.keys(vectors.glb).sort()).toEqual([...poses].sort());
    expect(Object.keys(vectors.fbx).sort()).toEqual(fbxCases.map(([name]) => name).sort());
  });

  it.each(poses)("pose %s: the GLB reproduces its digest and passes the Khronos validator, three's GLTFLoader and glTF-Transform with a scene per scale and mode", async (pose) => {
    const bytes = await writeGlb(asset, { pose });
    expect(glbDigestOf(bytes)).toEqual(vectors.glb[pose]);
    const report = await checkGlb(bytes, `${pose}.glb`, pose, expectedScenes(asset));
    expect(report.findings).toEqual([]);
    expect(report.facts.errors).toBe(0);
    expect(report.facts.warnings).toBe(0);
  });

  it.each(fbxCases)("%s: the FBX reproduces its digest and loads in three's FBXLoader and in assimp with every rig bone", async (name, clip, scale, view) => {
    const result = await writeFbxDetailed(asset, { clip: clip.id, scale, view });
    expect(fbxDigestOf(result)).toEqual(vectors.fbx[name]);
    const report = await checkFbx(result.bytes, name, { clip: clip.id, durationS: clip.durationS, fps: clip.fps, bones: asset.rig.bones.map((b) => b.id) });
    expect(report.findings).toEqual([]);
  });
});
