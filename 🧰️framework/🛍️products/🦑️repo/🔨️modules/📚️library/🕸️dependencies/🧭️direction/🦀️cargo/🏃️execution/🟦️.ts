import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { CargoMetadataCaptureBudget } from "./⏱️budget/🟦️.ts";
import { startNativeProgress } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { discoverCargoWorkspaces, prepareCargoWorkspaceInvocation } from "../../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {repositoryCargoPreparationStorageV1,withRepositoryCargoPreparationV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { cargoDirectionInventory, cargoDirectionMetadata, cargoDependencyDirectionReport, type CargoDirectionPolicy } from "../🟦️.ts";

/** 🦀️ Checks every authored Cargo declaration with independent manifest and metadata inventories. */
export async function verifyCargoDependencyDirection(repoRoot:string,invocation:ScriptInvocation,control:import("../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts").ResourceLeaseControl):Promise<void>{
 return withRepositoryCargoPreparationV1(repoRoot,repoRoot,invocation,control,process.env,async(environment,phase)=>{
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
  const policy: CargoDirectionPolicy = { areaLayers: taxonomy.areaLayers, ...taxonomy.cargoDependencyDirections };
  const inventory = cargoDirectionInventory(repoRoot);
  console.log(`[cargo-dependency-direction] resolving all declaration kinds; inventoriedPackages=${inventory.length}`);
  const metadata: unknown[] = [], budget = new CargoMetadataCaptureBudget(30_000);
  for (const scope of discoverCargoWorkspaces(repoRoot)) {
    const args = ["metadata", "--format-version", "1", "--no-deps", "--offline", "--locked", "--manifest-path", join(repoRoot, scope.manifest)];
    await prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(repoRoot),repoRoot,args,repoRoot,environment,phase.leaseControl);
    const remaining = budget.remainingMs;
    if (remaining <= 0) throw new Error("Cargo metadata capture budget exhausted");
    const captureStarted = performance.now();
    try{const output=await captureOwnedCommandV1("cargo",args,repoRoot,{control:phase.leaseControl,env:environment,output:{maximumBytes:16777216,maximumLines:65536}});metadata.push(JSON.parse(output));}finally{budget.charge(performance.now()-captureStarted);}
  }
  const report = cargoDependencyDirectionReport(cargoDirectionMetadata(metadata, repoRoot), inventory, policy);
  for (const problem of report.problems) console.error(`[cargo-dependency-direction] ${problem.code}: ${problem.owner}`);
  for (const edge of report.violations) console.error(`[cargo-dependency-direction] ${edge.rule}: ${edge.from} → ${edge.to}; alias=${edge.alias}; kind=${edge.kind}; optional=${edge.optional}; platform=${edge.platform ?? "all"}`);
  if (report.problems.length || report.violations.length) throw new Error(`Cargo dependency direction failed: ${report.violations.length} strict violations, ${report.problems.length} metadata problems, ${report.localDependencies} local declarations, ${report.packages} packages`);
  console.log(`[cargo-dependency-direction] passed; localDeclarations=${report.localDependencies}; packages=${report.packages}`);
 });
}
