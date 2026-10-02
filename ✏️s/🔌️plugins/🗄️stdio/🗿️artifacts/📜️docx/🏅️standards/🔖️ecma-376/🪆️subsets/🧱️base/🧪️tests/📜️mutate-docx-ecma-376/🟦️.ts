/** 🟦️ DOCX ECMA-376 mutation case — the ORACLE half, `jszip-docx-ecma-376-mutate-reader`. A reader oracle computes
 *  nothing: each mutation row's — and the whole-document `set-snapshot`'s — expected package is its COMMITTED python-docx
 *  `➡️after.docx`, and the real README itself is the expected package of every inverse and the identity round trip — handed to the
 *  `docx-ecma-376-jszip-compare-v1` pipeline as `expected-docx`, where the jszip reader reads it and the subject's
 *  `actual-docx`. */

/** 🟦️ DOCX ECMA-376 mutation case — the ORACLE half, `jszip-docx-ecma-376-mutate-reader`. A reader oracle computes
 *  nothing: each mutation row's — and the whole-document `set-snapshot`'s — expected package is its COMMITTED python-docx
 *  `➡️after.docx`, and the real README itself is the expected package of every inverse and the identity round trip — handed to the
 *  `docx-ecma-376-jszip-compare-v1` pipeline as `expected-docx`, where the jszip reader reads it and the subject's
 *  `actual-docx`. */
import { defineTestAdapter, type AdapterContext } from "../../../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
/** 🟦️ DOCX ECMA-376 mutation case — the ORACLE half, `jszip-docx-ecma-376-mutate-reader`. A reader oracle computes
 *  nothing: each mutation row's — and the whole-document `set-snapshot`'s — expected package is its COMMITTED python-docx
 *  `➡️after.docx`, and the real README itself is the expected package of every inverse and the identity round trip — handed to the
 *  `docx-ecma-376-jszip-compare-v1` pipeline as `expected-docx`, where the jszip reader reads it and the subject's
 *  `actual-docx`. */
import { committedArtifact } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧭️Adapter
const DOCX = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const readme = { oracle: (ctx: AdapterContext) => committedArtifact(ctx, "📜️example-readme.docx", "expected-docx", DOCX) };
const after = { oracle: (ctx: AdapterContext) => committedArtifact(ctx, "➡️after.docx", "expected-docx", DOCX) };

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: after,
    inverse: readme,
    "mutate-set-snapshot": after,
    "inverse-set-snapshot": readme,
    "identity-round-trip": readme,
  },
});
//#endregion 🧭️Adapter
