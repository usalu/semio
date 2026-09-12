#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-graph` task router. */
import { ScriptRouter, runBundleScriptMain } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { CheckGeneratedScript, GenerateScript, LintScript, PreviewGeneratedScript, TestScript } from "../../🛂️manifest/🏭️generator/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript)
  .register("test", TestScript)
  .register("lint", LintScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
