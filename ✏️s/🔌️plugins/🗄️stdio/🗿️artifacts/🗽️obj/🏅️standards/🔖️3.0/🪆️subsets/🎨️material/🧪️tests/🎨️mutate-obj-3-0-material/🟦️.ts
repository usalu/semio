/** 🟦️ OBJ 3.0 `🎨️material` mutation case — the ORACLE half, `three-obj-3-0-document-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.obj` for a mutation, `⬅️before.obj` for its
 *  inverse — handed to the `obj-3-0-document-compare-v1` pipeline as `expected-obj`, where three's OBJLoader reads it
 *  and the subject's `actual-obj`. */

import { committedArtifact, defineTestAdapter } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.obj", "expected-obj", "model/obj") },
    inverse: { oracle: (ctx) => committedArtifact(ctx, "⬅️before.obj", "expected-obj", "model/obj") },
  },
});
//#endregion 🧭️Adapter
