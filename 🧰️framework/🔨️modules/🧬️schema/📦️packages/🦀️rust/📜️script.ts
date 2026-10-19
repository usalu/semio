#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";
import { TestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript)
  .register("test", TestScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "generate" }));