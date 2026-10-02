/** 🟦️ OBJ 3.0 `🎨️material` mutation case — the ORACLE half, `three-obj-3-0-document-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.obj` for a mutation, `⬅️before.obj` for its
 *  inverse — handed to the `obj-3-0-document-compare-v1` pipeline as `expected-obj`, where three's OBJLoader reads it
 *  and the subject's `actual-obj`. */

/** 🟦️ OBJ 3.0 `🎨️material` mutation case — the ORACLE half, `three-obj-3-0-document-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.obj` for a mutation, `⬅️before.obj` for its
 *  inverse — handed to the `obj-3-0-document-compare-v1` pipeline as `expected-obj`, where three's OBJLoader reads it
 *  and the subject's `actual-obj`. */
import { defineTestAdapter } from "../../../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
/** 🟦️ OBJ 3.0 `🎨️material` mutation case — the ORACLE half, `three-obj-3-0-document-reader`. A reader oracle computes
 *  nothing: each row's expected document is the COMMITTED fixture — `➡️after.obj` for a mutation, `⬅️before.obj` for its
 *  inverse — handed to the `obj-3-0-document-compare-v1` pipeline as `expected-obj`, where three's OBJLoader reads it
 *  and the subject's `actual-obj`. */
import { committedArtifact } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.obj", "expected-obj", "model/obj") },
    inverse: { oracle: (ctx) => committedArtifact(ctx, "⬅️before.obj", "expected-obj", "model/obj") },
  },
});
//#endregion 🧭️Adapter
