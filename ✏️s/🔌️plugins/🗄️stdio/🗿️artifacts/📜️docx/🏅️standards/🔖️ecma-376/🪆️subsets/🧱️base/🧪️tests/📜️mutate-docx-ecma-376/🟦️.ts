/** 🟦️ DOCX ECMA-376 mutation case — the ORACLE half, `jszip-docx-ecma-376-mutate-reader`. A reader oracle computes
 *  nothing: each mutation row's expected package is its COMMITTED python-docx `➡️after.docx`, and the real README itself
 *  is the expected package of every inverse row, both no-mutation baselines and the identity round trip — handed to the
 *  `docx-ecma-376-jszip-compare-v1` pipeline as `expected-docx`, where the jszip reader reads it and the subject's
 *  `actual-docx`. */

import { committedArtifact, defineTestAdapter, type AdapterContext } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
const DOCX = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const readme = { oracle: (ctx: AdapterContext) => committedArtifact(ctx, "📜️example-readme.docx", "expected-docx", DOCX) };

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.docx", "expected-docx", DOCX) },
    "no-mutation-baseline-mutate": readme,
    inverse: readme,
    "no-mutation-baseline-inverse": readme,
    "identity-round-trip": readme,
  },
});
//#endregion 🧭️Adapter
