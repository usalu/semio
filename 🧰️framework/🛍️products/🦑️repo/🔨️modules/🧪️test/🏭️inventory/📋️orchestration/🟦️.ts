import { matchesTarget, readSelectors } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import { type MutationManifest, type RuntimeMutationInventory, compareInventories, loadOracleRegistry, subsetCoordinate, writeRuntimeInventory } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script, runProbe, testLevelBudgetMs } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync } from "node:fs";
import { join, relative, sep } from "node:path";

/**
 * 🏭️ The language-neutral production mutation bridge an owner exposes. It is a plain executable
 * beside the owner that answers `listMutations(artifact, standard, subset)` on stdout. Keeping it a
 * process rather than a linked entry point is what lets one gate cover Rust, TypeScript and every
 * other implementation without the framework knowing which language an artifact is written in.
 */
export const MUTATION_BRIDGE_REL = "🏭️bridge/📜️script.ts";

export function mutationBridgeFor(repoRoot: string, owner: string, manifest: MutationManifest): { command: string; args: string[] } | null {
  // 🧭️The bridge is looked up at the owner and then at each ancestor, so a subset inherits the one its
  // artifact publishes instead of every subset needing its own copy.
  let candidate = owner;
  for (;;) {
    const abs = join(repoRoot, candidate, MUTATION_BRIDGE_REL);
    if (existsSync(abs)) return { command: "bun", args: [abs, "list-mutations", manifest.artifact, manifest.standard, manifest.subset] };
    const parent = candidate.split("/").slice(0, -1).join("/");
    if (parent === "" || parent === candidate) return null;
    candidate = parent;
  }
}

/**
 * 🏭️ The runtime mutation inventory phase. Runs each owner's PRODUCTION dispatch bridge and records
 * what it actually offers, so completeness becomes a measurement rather than a claim. This is the
 * phase v1 had no equivalent of: its mutation audit compared a catalog with checked-in evidence and
 * never consulted dispatch, so a mutation reachable in production and missing from the catalog left
 * no trace anywhere.
 */
export class InventoryScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const manifests = registry.contributions.flatMap((contribution) => contribution.mutationManifests.map((manifest) => ({ contribution, manifest }))).filter(({ manifest }) => matchesTarget(manifest, selectors));
    if (manifests.length === 0) {
      console.error("[inventory] no mutation manifest matches the selection — declare one in the owner's 🔮️oracles collection");
      process.exit(1);
    }
    let failed = 0;
    for (const { contribution, manifest } of manifests) {
      const coordinate = subsetCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset: manifest.subset });
      const bridge = mutationBridgeFor(this.repoRoot, contribution.owner, manifest);
      if (bridge === null) {
        console.error(`[inventory] ${coordinate}: no production mutation bridge — expected an executable at ${MUTATION_BRIDGE_REL} beside the owner`);
        failed += 1;
        continue;
      }
      const probe = runProbe(bridge.command, bridge.args, {
        cwd: this.repoRoot,
        budgetMs: testLevelBudgetMs("long"),
        env: { ...process.env, SEMIO_MUTATION_ARTIFACT: manifest.artifact, SEMIO_MUTATION_STANDARD: manifest.standard, SEMIO_MUTATION_SUBSET: manifest.subset },
      });
      if ((probe.status ?? 1) !== 0) {
        console.error(`[inventory] ${coordinate}: bridge exited ${probe.status} — ${probe.stderr.trim().split("\n").slice(-3).join(" | ")}`);
        failed += 1;
        continue;
      }
      let inventory: RuntimeMutationInventory;
      try {
        inventory = JSON.parse(probe.stdout) as RuntimeMutationInventory;
      } catch (error) {
        console.error(`[inventory] ${coordinate}: bridge did not emit a runtime inventory (${(error as Error).message})`);
        failed += 1;
        continue;
      }
      if (inventory.schema !== "semio.repository-test.runtime-inventory/v2") {
        console.error(`[inventory] ${coordinate}: bridge emitted schema ${JSON.stringify(inventory.schema)}`);
        failed += 1;
        continue;
      }
      const path = writeRuntimeInventory(this.repoRoot, inventory);
      const equality = compareInventories(manifest, inventory, []);
      const drift = equality.runtimeOnly.length + equality.manifestOnly.length + equality.outcomeMismatches.length + equality.variantMismatches.length;
      console.log(`[inventory] ${coordinate}: ${inventory.mutations.length} runtime mutation(s), ${manifest.mutations.length} declared, ${drift} difference(s) → ${relative(this.repoRoot, path).split(sep).join("/")}`);
      for (const id of equality.runtimeOnly) console.error(`[inventory] ${coordinate}: runtime-only ${id}`);
      for (const id of equality.manifestOnly) console.error(`[inventory] ${coordinate}: manifest-only ${id}`);
      if (drift > 0) failed += 1;
    }
    if (failed > 0) process.exit(1);
  }
}
