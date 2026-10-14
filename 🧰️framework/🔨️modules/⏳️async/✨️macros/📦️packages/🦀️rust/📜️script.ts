#!/usr/bin/env bun
import { executeCargoRequestV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { relative, resolve } from "node:path";
/** ✨️ `@semio-tech/async-macros-rs` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await executeCargoRequestV1({version:1,kind:"tests",manifest:relative(this.repoRoot,resolve(this.root,"Cargo.toml")).replaceAll("\\","/"),cwd:relative(this.repoRoot,this.root).replaceAll("\\","/"),packages:["semio-framework-async-macros"],arguments:segments},this.invocation);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
