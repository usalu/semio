#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-graph` task router. */
import { ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { CheckGeneratedScript, GenerateScript, LintScript, ManifestContractScript, PreviewGeneratedScript, TestScript } from "../../🛂️manifest/🏃️execution/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript)
  .register("test", TestScript)
  .register("test-manifest-contract", ManifestContractScript)
  .register("lint", LintScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "generate" });
