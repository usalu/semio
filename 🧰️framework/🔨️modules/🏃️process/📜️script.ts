#!/usr/bin/env repo
import {scriptCompilerAdmissionControlV1} from "./🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "./🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🏃️ Runs neutral process ownership and execution contracts. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "./🧭️routing/🟦️.ts";
import { readProcessTestTableV1 } from "./🧪️testing/🗂️suites/🟦️.ts";
import { processTestCheckoutV1, runProcessTestSuiteV1, withProcessTestOutputV1 } from "./🧪️testing/🗂️suites/🏃️run/🟦️.ts";
import { runScriptMain } from "./🧭️routing/🚪️entrypoint/🟦️.ts";
import {executeCommandV1} from "./🧭️routing/🎛️command/🟦️.ts";
import {parseCommandArgumentsV1,resolveCommandConfigurationV1} from "./🧭️routing/🎛️command/⚙️configuration/🟦️.ts";

import { isMainModuleV1 } from "../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
/** 🎛️ Runs a configured neutral command with exact context and artifact custody. */
class CommandScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const plan=resolveCommandConfigurationV1(parseCommandArgumentsV1(segments),process.env,this.invocation);
  const result=await executeCommandV1(plan.request,plan.policy,{invocation:this.invocation,environment:plan.environment,onProgress:()=>{}});process.stdout.write(result.stdout);process.stderr.write(result.stderr);process.exitCode=result.reason==="exit"?result.status??1:1;
 }
}

/** 🧪️ Runs one declared process contract suite: every test file in its own process (one process receives one Original birth) beneath the authored capabilities its suite declares and a per-run test sandbox. */
class TestScript extends BundleScript {
  private get checkout(): string { return processTestCheckoutV1(this.root, this.repoRoot); }
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1) throw new Error("Expected one process contract suite");
    await withProcessTestOutputV1(this.checkout, "process-contract", output => this.suite(segments[0]!, output, process.env), readProcessTestTableV1().suites.find(row => row.id === segments[0])?.scratch);
  }

  private async suite(id: string, output: string, base: NodeJS.ProcessEnv): Promise<void> {
    if(id==="resource-leases"){await (await import("./🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts")).testResourceLeases(output,scriptCompilerAdmissionControlV1(this.invocation,"test:resource-leases",0),this.invocation);return;}
    if(id==="artifact-publication"){await (await import("./📦️artifacts/📤️publication/🧪️tests/🟦️.ts")).testArtifactPublication(output);return;}
    if (id === "artifact-files") {
      const { testArtifactFiles } = await import("./📦️artifacts/🗂️files/🧪️tests/🟦️.ts");
      await testArtifactFiles(output);
      return;
    }
    await runProcessTestSuiteV1({ invocation: this.invocation, root: this.root, checkout: this.checkout, table: readProcessTestTableV1(), suite: id, owner: "process", output, environment: base });
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname,resolve(import.meta.dirname,"../..")).register("command",CommandScript).register("test", TestScript), { invocation: original }));
