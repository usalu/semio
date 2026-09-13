#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-schema` task router. */
import { ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
