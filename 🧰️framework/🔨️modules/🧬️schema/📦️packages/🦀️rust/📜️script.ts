#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";
import { TestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";


const router = new ScriptRouter(import.meta.dirname)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript)
  .register("test", TestScript);

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "generate" }));