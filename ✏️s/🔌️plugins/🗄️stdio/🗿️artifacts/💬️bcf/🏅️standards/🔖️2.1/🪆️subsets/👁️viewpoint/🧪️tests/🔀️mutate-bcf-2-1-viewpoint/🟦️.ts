/** 🟦️ BCF 2.1 👁️viewpoint mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is its committed pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its inverse,
 *  handed to the `bcf-2-1-jszip-compare-v1` pipeline as `expected-bcf`, where the jszip reader reads it and the
 *  subject's `actual-bcf`. */

/** 🟦️ BCF 2.1 👁️viewpoint mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is its committed pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its inverse,
 *  handed to the `bcf-2-1-jszip-compare-v1` pipeline as `expected-bcf`, where the jszip reader reads it and the
 *  subject's `actual-bcf`. */
import { defineTestAdapter } from "../../../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
/** 🟦️ BCF 2.1 👁️viewpoint mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is its committed pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its inverse,
 *  handed to the `bcf-2-1-jszip-compare-v1` pipeline as `expected-bcf`, where the jszip reader reads it and the
 *  subject's `actual-bcf`. */
import { committedArtifact } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.bcf", "expected-bcf", "application/octet-stream") },
    inverse: { oracle: (ctx) => committedArtifact(ctx, "⬅️before.bcf", "expected-bcf", "application/octet-stream") },
  },
});
//#endregion 🧭️Adapter
