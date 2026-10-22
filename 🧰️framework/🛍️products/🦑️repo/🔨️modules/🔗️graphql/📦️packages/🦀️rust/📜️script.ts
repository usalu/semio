#!/usr/bin/env repo
import { resolve } from "node:path";
import { GenerateScript, CheckScript, PreviewGeneratedScript } from "../../🔤️types/🏃️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { scriptResourceProcessEnvironmentV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runDeclaredRustBuildV1, runRepositoryCargoTests } from "../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-graphql-rs` router: `repo ./📜️script.ts <build|test>`. */
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const CRATE = "semio-framework-repo-graphql";
const SCHEMA_CRATE = "semio-framework-repo-graphql-schema";

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDeclaredRustBuildV1(this.repoRoot, "@semio-tech/repo-graphql-rs", this.root, this.invocation, segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([SCHEMA_CRATE, CRATE], this.repoRoot, this.invocation, rest);
  }
}

/** 🧪️ Checks canonical GraphQL documents and generated types against graphql-js and TypeScript. */
class TypeSourceTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("test-types-source accepts no arguments");
    const control = scriptCompilerAdmissionControlV1(this.invocation, "graphql:types-source", 30000);
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔤️types/🧪️tests/🟦️.ts")], this.repoRoot, "graphql:types-source", compilerAdmissionBudgetV1(control), { signal: control.signal, env: scriptResourceProcessEnvironmentV1(control, { ...process.env, BUN_RUNTIME_TRANSPILER_CACHE_PATH: "0", NODE_DISABLE_COMPILE_CACHE: "1" }) });
  }
}

const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("test", TestScript).register("generate-types", GenerateScript).register("check-types", CheckScript).register("preview-types", PreviewGeneratedScript).register("test-types-source", TypeSourceTestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
