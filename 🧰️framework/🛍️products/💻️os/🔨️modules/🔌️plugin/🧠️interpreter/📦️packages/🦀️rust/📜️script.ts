
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo, resolveTypescriptCompilerV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptInvocationBudget } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolve } from "node:path";
import { runTestJobExportsNativeScript, runTestPollInputNativeScript, runTestCompletedNativeScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧠️ Type-checks the owned interpreter crate. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

/** 🧪️ Runs the core interpreter's library laws. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

/** 🔌️ Verifies the authored job export contract against the actual selector and independent source oracles. */
class TestJobExportsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Job export contract checks accept no arguments");
    const tests = [resolve(this.root, "../../🧪️tests/🔌️exports/🟦️.ts"), resolve(this.root, "../../../⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🧪️tests/🟦️.ts")];
    await runOwnedCommand(process.execPath, [resolveTypescriptCompilerV1(this.repoRoot), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun,node", ...tests], this.repoRoot, "plugin-interpreter:job-exports:types", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
    await runOwnedCommand(process.execPath, ["test", ...tests], this.repoRoot, "plugin-interpreter:job-exports:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

/** 🦀️ Runs the actual selector and allocation-free fixed receipt through their original exact native owner. */
class TestJobExportsNativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runTestJobExportsNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 📥️ Verifies resident original ingress against the closed independent byte oracle. */
class TestPollInputSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original poll input accepts no arguments");const source=resolve(this.root,"../../../⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests/🟦️.ts");for(const command of [[resolveTypescriptCompilerV1(this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",source],["test",source]])await runOwnedCommand(process.execPath,command,this.repoRoot,"plugin-interpreter:poll-input:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});}
}
/** 🦀️ Runs the production resident Entry and its original caller/System conservation. */
class TestPollInputNativeScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runTestPollInputNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 📦️ Qualifies the canonical Completed donor, bounded decoder and full native Fault source. */
class TestCompletedSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original Completed source accepts no arguments");const source=resolve(this.root,"../../../⚛️reactor/🔄️turn/📥️receiving/📦️completed/🧪️tests/🟦️.ts");for(const command of [[resolveTypescriptCompilerV1(this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",source],["test",source]])await runOwnedCommand(process.execPath,command,this.repoRoot,"plugin-interpreter:completed:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});}
}
/** 🦀️ Runs the genuine Completed receiving owner with original source and System conservation. */
class TestCompletedNativeScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runTestCompletedNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-completed-source",TestCompletedSourceScript).register("test-completed-native",TestCompletedNativeScript).register("test-poll-input-source",TestPollInputSourceScript).register("test-poll-input-native",TestPollInputNativeScript).register("check", CheckScript).register("test", TestScript).register("test-job-exports", TestJobExportsScript).register("test-job-exports-native", TestJobExportsNativeScript), { invocation: original }));
