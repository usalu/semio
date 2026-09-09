import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🧪️ Validates language-neutral target vectors and independently compares shipping coverage with Cargo metadata. */
export function rustWarningScopeChecks(root: string, rustWarningTargetScope: (root: string, target: string | undefined) => { packages: string[]; scopeArgs: string[]; targetArgs: string[]; packageArgs: Record<string, string[]> }): number {
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/🦀️rust-warnings/🔣️.json"), "utf8")) as {
    cases: { target: string; requiredPackages: string[]; scopeArgs: string[]; targetArgs: string[]; rendererFeatures: string[] }[];
    rejectedTargets: string[];
  };
  for (const row of fixture.cases) {
    const scope = rustWarningTargetScope(root, row.target);
    for (const pkg of row.requiredPackages) if (!scope.packages.includes(pkg)) throw new Error(`[verify rust-warnings] ${row.target} misses ${pkg}.`);
    for (const [actual, expected] of [[scope.scopeArgs, row.scopeArgs], [scope.targetArgs, row.targetArgs], [scope.packageArgs["semio-framework-os-renderer-wgpu"] ?? [], row.rendererFeatures]]) {
      if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`[verify rust-warnings] target arguments differ for ${row.target}.`);
    }
    if (new Set(scope.packages).size !== scope.packages.length) throw new Error(`[verify rust-warnings] duplicate package in ${row.target}.`);
  }
  for (const target of fixture.rejectedTargets) {
    let rejected = false;
    try { rustWarningTargetScope(root, target); } catch { rejected = true; }
    if (!rejected) throw new Error(`[verify rust-warnings] unsupported target ${target} was accepted.`);
  }
  const cargo = Bun.spawnSync(["cargo", "metadata", "--no-deps", "--format-version=1"], { cwd: root });
  if (cargo.exitCode !== 0) throw new Error(`[verify rust-warnings] Cargo metadata failed: ${cargo.stderr.toString()}`);
  const metadata = JSON.parse(cargo.stdout.toString()) as { packages: { name: string; metadata?: { semio?: { role?: string } } | null }[] };
  const oracle = metadata.packages.filter((pkg) => ["plugin", "extension"].includes(pkg.metadata?.semio?.role ?? "") || (pkg.name.startsWith("semio-s-artifact-") || pkg.name.startsWith("semio-framework-artifact-"))).map((pkg) => pkg.name).sort();
  const components = rustWarningTargetScope(root, "wasm32-wasip2").packages;
  if (JSON.stringify(components) !== JSON.stringify(oracle)) throw new Error(`[verify rust-warnings] repository discovery differs from Cargo's shipping package catalog.`);
  console.log(`[verify rust-warnings] ${fixture.cases.length} target vectors, ${fixture.rejectedTargets.length} rejection vectors, ${oracle.length} Cargo-verified component crates.`);
  return fixture.cases.length + fixture.rejectedTargets.length + 1;
}
