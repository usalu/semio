#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { PrintTokenPreviewScript } from "../../🔨️modules/🎨print-design-token-paints/📜️script.ts";
import { CheckScript, FontsScript, GenerateScript, TestScript } from "../../🧪️tests/🔬️script-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dirname)
  .register("fonts", FontsScript)
  .register("generate", GenerateScript)
  .register("preview-generated", PrintTokenPreviewScript)
  .register("check",CheckScript)
  .register("test", TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));
