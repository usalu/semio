/** 🎛️ Commands of the catalog verification: the registry test, catalog contract test, native catalog selection check and catalog complete scripts that package routing lazily loads. */
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { existsSync } from "node:fs";
import { isAbsolute } from "node:path";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runVitest } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { auditPluginCatalogSources } from "../🔎️source-audit/🟦️.ts";
import { createFreshCatalogBuildVerifier } from "../🏗️fresh-build-verifier/🟦️.ts";
import { executeCatalogVerificationPlan } from "../🗺️verification-plan/🟦️.ts";
import { nativeCatalogSelectionOracleV1 } from "../🎯️native-selection/🟦️.ts";

export class RegistryTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🧬️ Verifies portable catalog admission independently of installed component deliverables. */
export class CatalogContractTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, ["🧪️tests/✅️catalog-publication/🟦️.ts", ...rest], "./🧪️tests/🎚️config/🟦️.ts");
  }
}


//#endregion 🔖️DescriptorGate

export class NativeCatalogSelectionCheckScript extends BundleScript {
  async run(): Promise<void> {
    await nativeCatalogSelectionOracleV1();
  }
}

/** 🛂️ Fails closed unless every discovered source and explicit fresh-root artifact verifies. */
export class CatalogCompleteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const repoRoot = getWorkspaceRoot();
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const buildRoot = option("--build-root") ?? process.env.SEMIO_CATALOG_FRESH_BUILD_ROOT;
    const cancelFile = option("--cancel-file");
    if (!buildRoot || !isAbsolute(buildRoot)) throw new Error("usage: catalog-complete --build-root <absolute fresh build root> [--cancel-file <path>]");
    const cancelled = (): boolean => cancelFile !== undefined && existsSync(cancelFile);
    const audit = auditPluginCatalogSources(repoRoot, {
      cancelled,
      descriptorAdmission: "source",
      progress(completed, total, path) { console.log(`catalog-complete source ${completed}/${total}: ${path}`); },
    });
    if (audit.issues.length > 0) {
      console.error(`catalog-complete source preflight failed (${audit.issues.length} issue(s), ${audit.manifestCount} manifests):`);
      for (const issue of audit.issues) console.error(`  - [${issue.code}] ${issue.pluginId ?? issue.path}: ${issue.diagnostic}`);
      throw new Error(`catalog-complete refused unverified source catalog (${audit.issues.length} issue(s))`);
    }
    const verifier = createFreshCatalogBuildVerifier(repoRoot, buildRoot);
    const result = await executeCatalogVerificationPlan(audit.entries, {
      async verify(node) {
        const entry = audit.entries.find(({ pluginId }) => pluginId === node.pluginId);
        if (!entry) throw new Error(`${node.pluginId}: Cargo source identity is absent`);
        return verifier.verify(entry, { cancelled });
      },
    }, {
      cancelled,
      progress(event) { console.log(`catalog-complete artifact ${event.completed}/${event.total}: ${event.pluginId} ${event.status}`); },
    });
    if (result.results.some(({ status }) => status !== "verified")) {
      for (const row of result.results.filter(({ status }) => status !== "verified")) console.error(`  - [${row.status}] ${row.pluginId}: ${row.diagnostic ?? "unverified"}`);
      throw new Error("catalog-complete refused unverified fresh build artifacts");
    }
    console.log(`plugin catalog is complete: ${audit.manifestCount} source identities and fresh raw/descriptor/receipt artifacts verified from ${verifier.root}.`);
  }
}
