#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { NativeScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("check accepts no arguments");
    const controller = new AbortController(), cancel = (): void => controller.abort();
    process.on("SIGINT", cancel);
    process.on("SIGTERM", cancel);
    try {
      const manifest = join(this.root, "Cargo.toml");
      await runRepositoryCommand("cargo", ["metadata", "--offline", "--format-version=1", "--manifest-path", manifest], this.root, "s-dev-native-resolution", 900_000, { signal: controller.signal, stdout: "ignore" });
      await runRepositoryCommand("cargo", ["check", "--locked", "--offline", "--manifest-path", manifest, "--all-targets", "--all-features"], this.root, "s-dev-native-check", 900_000, { signal: controller.signal, onLine: line => console.log(line) });
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("native", NativeScript).register("check", CheckScript), { invocation: original }));
