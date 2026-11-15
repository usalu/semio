/** 🧊️ The structure digests of the GLB and FBX exports of the four-scale sample figure: per pose the byte length, SHA-256 and glTF counts including the scenes, per clip, scale and checked view the byte length, SHA-256, the FBX object counts and the notices. The fixture `🧫️fixtures/🧊️model-digests` records them; `pipeline-digests.ts` of the ticket regenerates it from this function.
 *
 * @see ../../🧊️model-digests/🟦️.ts — the test
 */
import { createHash } from "node:crypto";
import { objectCounts, readFbx } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts";
import type { Asset } from "../../../🧬️schema/🟦️.ts";
import type { View } from "../../../🔨️modules/🔗️contracts/🟦️.ts";
import { writeFbxDetailed } from "../../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { fbxName } from "../../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { authoredScales } from "../../../🔨️modules/🧊️volume/🟦️.ts";
import { fbxDigest, glbDigest, glbFacts } from "../../🧊️model-catalog/🔍️oracles/🟦️.ts";

/** 🎭️ The FBX views the digests cover: one per mode. */
export const DIGEST_VIEWS: readonly View[] = ["light-solid", "thermal", "forces"];

/** 📚️ The recorded digests. */
export type Digests = { readonly glb: Record<string, Record<string, unknown>>; readonly fbx: Record<string, Record<string, unknown>> };

const sha256 = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");

/** 🧮️ The digest of one GLB. */
export const glbDigestOf = (bytes: Uint8Array): Record<string, unknown> => ({ ...glbDigest(bytes, sha256), scenes: Object.fromEntries(Object.entries(glbFacts(bytes).scenes)) });

/** 🧮️ The digest of one FBX result. */
export const fbxDigestOf = (result: Awaited<ReturnType<typeof writeFbxDetailed>>): Record<string, unknown> => fbxDigest(result.bytes, result.notices.map((n) => `${n.code}:${n.subject}`), sha256, objectCounts(readFbx(result.bytes)));

/** 🧬️ The digests of an asset: a GLB per named pose, an FBX per clip, authored scale and {@link DIGEST_VIEWS}. */
export async function digestsOf(asset: Asset): Promise<Digests> {
  const glb: Digests["glb"] = {};
  for (const pose of asset.poses.filter((p) => p.key !== true)) glb[pose.id] = glbDigestOf(await writeGlb(asset, { pose: pose.id }));
  const fbx: Digests["fbx"] = {};
  for (const clip of asset.clips) for (const scale of authoredScales(asset.volumes)) for (const view of DIGEST_VIEWS) fbx[fbxName(asset.id, clip.id, scale, view)] = fbxDigestOf(await writeFbxDetailed(asset, { clip: clip.id, scale, view }));
  return { glb, fbx };
}
