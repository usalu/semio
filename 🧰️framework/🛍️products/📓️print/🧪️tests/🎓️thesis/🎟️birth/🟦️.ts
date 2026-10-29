import "../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🪪️native-birth/🟦️.ts";
import assert from "node:assert/strict";
import { join } from "node:path";
import { readFile } from "node:fs/promises";
import { declareBuildTestEnvelopeV1, type BuildTestEnvelopeV1 } from "@semio-tech/process/testing/invocation";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { withJavascriptDependencyStorageV1, prepareJavascriptDependencies } from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
import type { ScriptInvocation } from "@semio-tech/process/routing";
import { renderPrintLatexTokenStylesheet } from "../../../🔨️modules/🎨print-design-token-paints/🟦️.ts";
import { preparePrintMaterialV1 } from "../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import type { PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";

/** 🪪️ Declares authored test data through the existing registered native build caller issuer. */
export function declareThesisTestBirth(value: unknown): void {
  declareBuildTestEnvelopeV1(value as BuildTestEnvelopeV1);
}

/** 🧰️ Checks the production-rendered private tokens and prepares pinned compiler, TeX and fonts through owned stores. */
export async function prepareThesisTestInputs(workspace: string, _invocation: ScriptInvocation, execution: PrintExecutionV1): Promise<void> {
  assert.equal(await readFile(join(workspace, "🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty"), "utf8"), renderPrintLatexTokenStylesheet(), "Private token stylesheet uses the actual production renderer");
  for (const kind of ["tectonic", "tex", "fonts"] as const) {
    await preparePrintMaterialV1(execution.operation, kind, execution.phaseControl);
    console.log("[print] Prepared thesis input " + kind);
  }
}

/** 🌱️ Produces the private fixture's genuine empty JavaScript dependency generation before ordinary readers open it. */
export async function prepareThesisPrivateDependencies(workspace: string, invocation: ScriptInvocation, control: ResourceLeaseControl): Promise<void> {
  let completed = false;
  await withJavascriptDependencyStorageV1(workspace, invocation, control, async (store, actual) => {
    await prepareJavascriptDependencies("sync", workspace, store, actual, { kind: "all" });
    completed = true;
  });
  assert.ok(completed, "Genuine private dependency generation completed");
}
