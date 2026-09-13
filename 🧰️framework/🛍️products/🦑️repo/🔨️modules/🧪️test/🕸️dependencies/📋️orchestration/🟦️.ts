import { REPO_TEST_DOMAIN_REL } from "../../🧱️contract/🟦️.ts";
import {
  type ClassifiedDependency,
  type DependencyEcosystem,
  type OracleEntry,
  classifyLegacyKind,
  dependencyEcosystemOfRegistryValue,
  externalOracleHostPackages,
  isProductionClass,
  loadOracleRegistry,
  oracleLinkedPackages,
  ratchetDependencies,
  scanDeclaredDependencies,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🔒️ Loads and phase-classifies the committed dependency baseline, adding any registry oracle it lacks. */
export function loadClassifiedBaseline(repoRoot: string): ClassifiedDependency[] {
  const baselineRaw = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as {
    entries: { ecosystem: string; name: string; version: string; kinds: string[]; users: string[]; productionReachable?: boolean; oracleIds?: string[]; capabilities?: string[] }[];
  };
  const registry = loadOracleRegistry(repoRoot);
  // 🔒️Keyed by EVERY package an oracle links and holding EVERY oracle that links it. Keying on the
  // primary package alone missed the composed halves; keeping only one oracle per package made the
  // committed record depend on manifest discovery order, so the same repository could produce two
  // different baselines. Ids are sorted, so the file is a function of the registry and nothing else.
  const oraclesByLinkedPackage = new Map<string, OracleEntry[]>();
  for (const oracle of registry.oracles) for (const linked of oracleLinkedPackages(oracle)) oraclesByLinkedPackage.set(linked.package, [...(oraclesByLinkedPackage.get(linked.package) ?? []), oracle]);
  const classified: ClassifiedDependency[] = baselineRaw.entries.map((entry) => {
    const linking = oraclesByLinkedPackage.get(entry.name) ?? [];
    const oracleIds = linking.length > 0 ? [...new Set(linking.map((oracle) => oracle.id))].sort() : (entry.oracleIds ?? []);
    const kinds = [
      ...new Set(entry.kinds.map((kind) => (["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"].includes(kind) ? (kind as ClassifiedDependency["kinds"][number]) : classifyLegacyKind(kind, oracleIds)))),
    ];
    return {
      ecosystem: entry.ecosystem as DependencyEcosystem,
      name: entry.name,
      version: entry.version,
      kinds,
      users: entry.users,
      productionReachable: entry.productionReachable ?? kinds.some(isProductionClass),
      oracleIds: oracleIds.length > 0 ? oracleIds : undefined,
      capabilities: linking.length > 0 ? [...new Set(linking.flatMap((oracle) => oracle.capabilities))].sort() : entry.capabilities,
    };
  });
  // 🧩️EVERY package a registered oracle links, not only the one its id is named after. A composed
  // reference (reader + writer, archive + XML) links several, and a secondary package that never
  // reached this list would be linked into the host while the ratchet and the report both showed it
  // as absent — the exact blind spot registration exists to close.
  for (const oracle of registry.oracles) {
    for (const linked of oracleLinkedPackages(oracle)) {
      if (classified.some((entry) => entry.name === linked.package)) continue;
      const linking = oraclesByLinkedPackage.get(linked.package) ?? [oracle];
      classified.push({
        ecosystem: dependencyEcosystemOfRegistryValue(oracle.ecosystem),
        name: linked.package,
        version: linked.version,
        kinds: ["test-oracle"],
        users: [oracle.hostPath ?? REPO_TEST_DOMAIN_REL],
        productionReachable: false,
        oracleIds: [...new Set(linking.map((entry) => entry.id))].sort(),
        capabilities: [...new Set(linking.flatMap((entry) => entry.capabilities))].sort(),
      });
    }
  }
  // 🧩️An owner that puts an external distribution on a generated host's import path has added a
  // third-party test dependency, exactly as registering an oracle package does. Classifying it here
  // is what keeps the Python and npm hosts inside the same ratchet as the Rust one instead of
  // letting a manifest field become an unwatched channel for reference libraries.
  for (const host of externalOracleHostPackages(registry)) {
    const existing = classified.find((entry) => entry.ecosystem === host.ecosystem && entry.name === host.name);
    if (existing !== undefined) {
      classified[classified.indexOf(existing)] = { ...existing, users: [...new Set([...existing.users, ...host.users])] };
      continue;
    }
    classified.push({ ecosystem: host.ecosystem, name: host.name, version: host.version, kinds: ["test-oracle"], users: host.users, productionReachable: false });
  }
  return classified.sort((a, b) => a.ecosystem.localeCompare(b.ecosystem) || a.name.localeCompare(b.name));
}

/** 🔒️ Multi-ecosystem dependency classification with production reachability and a shrink-only ratchet. */
export class DependencyScript extends Script {
  run(segments: string[]): void {
    const sorted = loadClassifiedBaseline(this.repoRoot);
    const registry = loadOracleRegistry(this.repoRoot);
    if (segments.includes("write-baseline")) {
      const baselinePath = join(this.repoRoot, "🔒️dependencies.json");
      const baselineRaw = JSON.parse(readFileSync(baselinePath, "utf8")) as Record<string, unknown>;
      writeFileSync(baselinePath, `${JSON.stringify({ ...baselineRaw, schemaVersion: 2, entries: sorted }, null, 2)}\n`);
      console.log(`[dependency] baseline rewritten with ${sorted.length} classified entries`);
      return;
    }

    // 🔎️The ratchet is fed the COMMITTED baseline against a FRESH SCAN of the live tree. It used to be
    // handed the same array twice, which made `newProduction` and `unregisteredTestDeps` provably
    // always empty however many production dependencies a change added — a shrink-only gate that
    // could not see growth. `--scan` prints what the scan found, for when the two disagree.
    const scanned = scanDeclaredDependencies(this.repoRoot, registry);
    const verdict = ratchetDependencies(sorted, scanned, registry);
    if (segments.includes("--scan")) {
      console.log(`[dependency] live scan: ${scanned.length} declared external dependenc(ies), ${scanned.filter((entry) => entry.productionReachable).length} production-reachable`);
      for (const entry of scanned.filter((candidate) => !sorted.some((baseline) => baseline.ecosystem === candidate.ecosystem && baseline.name === candidate.name))) {
        console.log(`[dependency] scan-only ${entry.ecosystem}:${entry.name}@${entry.version} kinds=${entry.kinds.join(",")} users=${entry.users.slice(0, 2).join(",")}`);
      }
    }
    const production = sorted.filter((entry) => entry.productionReachable);
    const oracleDeps = sorted.filter((entry) => entry.kinds.includes("test-oracle"));
    console.log(`[dependency] ecosystems=${new Set(sorted.map((entry) => entry.ecosystem)).size} entries=${sorted.length} production-reachable=${production.length} test-oracle=${oracleDeps.length}`);
    for (const entry of oracleDeps) console.log(`[dependency] test-oracle ${entry.ecosystem}:${entry.name}@${entry.version} (${(entry.oracleIds ?? []).join(",")})`);
    // 🔒️Recorded debt is printed every run so it can never quietly become permanent; an UNRECORDED
    // production-reachable oracle is still a hard failure.
    const recorded = new Map(registry.oracles.filter((entry) => entry.productionDebt !== undefined).map((entry) => [entry.package, entry]));
    for (const [, entry] of recorded) console.log(`[dependency] production-debt ${entry.package} (oracle ${entry.id}) reachable from ${entry.productionDebt!.reachableFrom.join(", ")} — owner ${entry.productionDebt!.owner}`);
    const leaked = oracleDeps.filter((entry) => entry.productionReachable && !recorded.has(entry.name));
    for (const entry of leaked) console.error(`[dependency] oracle package ${entry.name} is production-reachable and is NOT recorded as debt — oracles must be test-only`);
    for (const name of verdict.newProduction) console.error(`[dependency] NEW production-reachable dependency ${name} is declared in the tree and absent from the committed baseline — the ratchet is shrink-only`);
    for (const name of verdict.unregisteredTestDeps) console.error(`[dependency] ${name} is declared as a test dependency but no oracle or probe registers it`);
    if (!verdict.ok || leaked.length > 0) process.exit(1);
  }
}
