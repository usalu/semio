import { readFileSync } from "node:fs";
import { join } from "node:path";
import { startNativeProgress } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { cargoDirectionInventory, cargoDirectionMetadata, cargoDependencyDirectionReport, type CargoDirectionPolicy } from "../🟦️.ts";

/** 🦀️ Checks every authored Cargo declaration with independent manifest and metadata inventories. */
export async function verifyCargoDependencyDirection(repoRoot: string): Promise<void> {
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
  const policy: CargoDirectionPolicy = { areaLayers: taxonomy.areaLayers, ...taxonomy.cargoDependencyDirections };
  const inventory = cargoDirectionInventory(repoRoot);
  console.log(`[cargo-dependency-direction] resolving all declaration kinds; inventoriedPackages=${inventory.length}`);
  const child = Bun.spawn(["cargo", "metadata", "--format-version", "1", "--no-deps", "--offline", "--locked"], { cwd: repoRoot, stdout: "pipe", stderr: "pipe" });
  let stopped = "";
  const stop = (reason: string): void => { stopped ||= reason; child.kill(); };
  const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
  process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
  const timeout = setTimeout(() => stop("timeout 30000ms"), 30_000), progress = startNativeProgress("cargo-dependency-direction");
  let stdout = "";
  try {
    const result = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    if (stopped || result[2] !== 0) throw new Error(stopped || result[1] || "Cargo direction metadata scan failed");
    stdout = result[0];
  } finally {
    clearTimeout(timeout); progress(); process.off("SIGINT", interrupt); process.off("SIGTERM", terminate);
  }
  const report = cargoDependencyDirectionReport(cargoDirectionMetadata(JSON.parse(stdout), repoRoot), inventory, policy);
  for (const problem of report.problems) console.error(`[cargo-dependency-direction] ${problem.code}: ${problem.owner}`);
  for (const edge of report.violations) console.error(`[cargo-dependency-direction] ${edge.rule}: ${edge.from} → ${edge.to}; alias=${edge.alias}; kind=${edge.kind}; optional=${edge.optional}; platform=${edge.platform ?? "all"}`);
  if (report.problems.length || report.violations.length) throw new Error(`Cargo dependency direction failed: ${report.violations.length} strict violations, ${report.problems.length} metadata problems, ${report.localDependencies} local declarations, ${report.packages} packages`);
  console.log(`[cargo-dependency-direction] passed; localDeclarations=${report.localDependencies}; packages=${report.packages}`);
}
