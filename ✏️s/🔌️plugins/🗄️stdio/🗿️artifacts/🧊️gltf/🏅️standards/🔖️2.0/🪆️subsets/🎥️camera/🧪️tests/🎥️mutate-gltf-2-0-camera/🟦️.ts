/** 🟦️ glTF 2.0 `🎥️camera` mutation case — the ORACLE half, `three-gltf-2-0-mutate-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.gltf` for a mutation, `⬅️before.gltf` for
 *  its inverse — handed to the `gltf-2-0-three-compare-v1` pipeline as `expected-gltf`, where three's GLTFLoader reads it
 *  and the subject's `actual-gltf`. */

/** 🟦️ glTF 2.0 `🎥️camera` mutation case — the ORACLE half, `three-gltf-2-0-mutate-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.gltf` for a mutation, `⬅️before.gltf` for
 *  its inverse — handed to the `gltf-2-0-three-compare-v1` pipeline as `expected-gltf`, where three's GLTFLoader reads it
 *  and the subject's `actual-gltf`. */
import { defineTestAdapter } from "../../../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
/** 🟦️ glTF 2.0 `🎥️camera` mutation case — the ORACLE half, `three-gltf-2-0-mutate-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.gltf` for a mutation, `⬅️before.gltf` for
 *  its inverse — handed to the `gltf-2-0-three-compare-v1` pipeline as `expected-gltf`, where three's GLTFLoader reads it
 *  and the subject's `actual-gltf`. */
import { committedArtifact } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.gltf", "expected-gltf", "model/gltf+json") },
    inverse: { oracle: (ctx) => committedArtifact(ctx, "⬅️before.gltf", "expected-gltf", "model/gltf+json") },
  },
});
//#endregion 🧭️Adapter
