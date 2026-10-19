import type { ScriptInvocation } from "@semio-tech/process/routing";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { publishTrustedStdioCatalogV1 } from "./✅️trusted-stdio-catalog/🟦️.ts";
import { join } from "node:path";

/** 🏃️ Runs the `TrustedCatalogPublishScript` command under the caller's roots and invocation. */
export function runTrustedCatalogPublishScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): void {
  const option = (name: string): string | undefined => {
    const index = segments.indexOf(name);
    return index < 0 ? undefined : segments[index + 1];
  };
  const outDir = option("--out") ?? join(context.root, "🤖️generated");
  const receipt = publishTrustedStdioCatalogV1({ outDir, repoRoot: findWorkspaceRoot() });
  console.log(`trusted stdio catalog ${receipt.publication}: ${receipt.catalog.nativeCodecs.length} codecs, open=${receipt.catalog.openTargets[0]?.artifactKind}, hubBundle=${receipt.catalog.hubBundle}, component=${receipt.catalog.componentAdmission.status}, sha256=${receipt.catalogSha256} -> ${receipt.outPath}`);
}
