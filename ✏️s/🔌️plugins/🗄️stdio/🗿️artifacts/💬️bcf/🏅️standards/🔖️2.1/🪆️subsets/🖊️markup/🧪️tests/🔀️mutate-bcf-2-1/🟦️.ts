/** 🟦️ BCF 2.1 markup mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is COMMITTED — its generator pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its
 *  inverse, the real coordination review itself for the identity round trip — handed to the `bcf-2-1-jszip-compare-v1`
 *  pipeline as `expected-bcf`, where the jszip reader reads it and the subject's `actual-bcf`. */

/** 🟦️ BCF 2.1 markup mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is COMMITTED — its generator pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its
 *  inverse, the real coordination review itself for the identity round trip — handed to the `bcf-2-1-jszip-compare-v1`
 *  pipeline as `expected-bcf`, where the jszip reader reads it and the subject's `actual-bcf`. */
import { defineTestAdapter, type AdapterContext } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
/** 🟦️ BCF 2.1 markup mutation case — the ORACLE half, `jszip-bcf-2-1-mutate-reader`. A reader oracle computes nothing:
 *  each row's expected archive is COMMITTED — its generator pair's `➡️after.bcf` for a mutation, `⬅️before.bcf` for its
 *  inverse, the real coordination review itself for the identity round trip — handed to the `bcf-2-1-jszip-compare-v1`
 *  pipeline as `expected-bcf`, where the jszip reader reads it and the subject's `actual-bcf`. */
import { committedArtifact } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
const after = { oracle: (ctx: AdapterContext) => committedArtifact(ctx, "➡️after.bcf", "expected-bcf", "application/octet-stream") };
const before = { oracle: (ctx: AdapterContext) => committedArtifact(ctx, "⬅️before.bcf", "expected-bcf", "application/octet-stream") };

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: after,
    inverse: before,
    "identity-round-trip": { oracle: (ctx) => committedArtifact(ctx, "🏥️wellness-center-coordination-review.bcf", "expected-bcf", "application/octet-stream") },
  },
});
//#endregion 🧭️Adapter
