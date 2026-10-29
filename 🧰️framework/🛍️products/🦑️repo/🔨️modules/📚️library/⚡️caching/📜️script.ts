#!/usr/bin/env repo
import { join } from "node:path";
import { AuditScript, PolicyScript } from "./📇️inventory/📋️orchestration/🟦️.ts";
import { GraphScript } from "./🕸️graph/✅️verification/🟦️.ts";
import { DiskScript } from "./💾️storage/📊️report/🟦️.ts";
import { DoctorScript } from "./🩺️environment/📋️inspection/🟦️.ts";
import { CacheVerifyScript } from "./🔁️verification/📋️orchestration/🟦️.ts";
import { CachePruneScript, CacheReportScript } from "./🧹️pruning/📋️orchestration/🟦️.ts";
import { CargoProvenanceScript } from "./🦀️cargo/🧾️provenance/🟦️.ts";

import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";


/** 🧪️ Runs one declared process suite, function proof, or explicit contracts leaf. */
class CacheTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { cachingTestRootV1, readCachingTestTableV1 } = await import("./🗂️suites/🟦️.ts"), table = readCachingTestTableV1(),{selectCanonicalCacheTestV1}=await import("./🗂️suites/🧮️aggregate/🟦️.ts"),selected=selectCanonicalCacheTestV1(args,table);
    if (selected === "suite") {
      const { processTestCheckoutV1, runProcessTestSuiteV1, withProcessTestOutputV1 } = await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🏃️run/🟦️.ts"), checkout = processTestCheckoutV1(this.root, this.repoRoot), { genuineBirthLaunchersV1 } = await import("../🧪️testing/🌱️genuine-birth/🚀️launchers/🟦️.ts"), { testWorkerLearningV1 } = await import("../🏗️build/🌡️host-capacity/🧪️workers/🟦️.ts"), launchers = genuineBirthLaunchersV1(join(this.root, "../🧪️testing/🌱️genuine-birth/📜️script.ts"), checkout);
      await withProcessTestOutputV1(checkout, "cache-contract", output => runProcessTestSuiteV1({ invocation: this.invocation, root: cachingTestRootV1, checkout, table, suite: args[0]!, owner: "cache", output, environment: process.env, launchers, workers: { learning: testWorkerLearningV1(process.env) } }), table.suites.find(suite => suite.id === args[0])?.scratch);
      return;
    }
    const { TestScript } = await import("./🧪️tests/🔬️script-commands/🟦️.ts");
    await new TestScript(this.root, this.repoRoot, this.invocation).run(args);
  }
}

/** 🔒️ Loads the lease command only for its explicit lease command. */
class CacheLeaseScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { LeaseScript } = await import("./🧪️tests/🔬️script-commands/🟦️.ts");
    await new LeaseScript(this.root, this.repoRoot, this.invocation).run(args);
  }
}

/** 📦️ Loads the artifact package proof only for its explicit verification command. */
class ArtifactPackageVerificationScript extends BundleScript {
  async run(): Promise<void> {
    const { ArtifactPackageContractScript } = await import("./🧪️tests/📦️artifact-packages/🟦️.ts");
    await new ArtifactPackageContractScript(this.root, this.repoRoot, this.invocation).run();
  }
}


const router = new ScriptRouter(import.meta.dirname)
  .register("test", CacheTestScript)
  .register("lease", CacheLeaseScript)
  .register("audit", AuditScript)
  .register("policy-check", PolicyScript)
  .register("artifact-check", PolicyScript)
  .register("artifact-package-contract", ArtifactPackageVerificationScript)
  .register("graph-check", GraphScript)
  .register("doctor", DoctorScript)
  .register("disk-report", DiskScript)
  .register("cache-verify", CacheVerifyScript)
  .register("cache-report", CacheReportScript)
  .register("cache-prune", CachePruneScript)
  .register("cargo-provenance", CargoProvenanceScript);

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
