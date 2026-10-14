#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { NativeScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("check accepts no arguments");
    const manifest = join(this.root, "Cargo.toml");
    await runRepositoryCommand(this.invocation,"cargo",["metadata", "--offline", "--format-version=1", "--manifest-path", manifest],this.root,"s-dev-native-resolution",{version:1,maximumElapsedMilliseconds:900000,output:{maximumBytes:134217728,maximumLines:65536}},{stdout:"ignore"});
    await runRepositoryCommand(this.invocation,"cargo",["check", "--locked", "--offline", "--manifest-path", manifest, "--all-targets", "--all-features"],this.root,"s-dev-native-check",{version:1,maximumElapsedMilliseconds:900000,output:{maximumBytes:134217728,maximumLines:65536}},{onLine:line=>console.log(line)});
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("native", NativeScript).register("check", CheckScript), { invocation: original }));
