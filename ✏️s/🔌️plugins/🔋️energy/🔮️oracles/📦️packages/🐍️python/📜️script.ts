#!/usr/bin/env bun
/** 🔮️ Energy oracle package command router. */
import { ScriptRouter, runBundleScriptMain } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { DepsScript, EmitScript, EpJsonScript, NativeScript, RunScript, SetupScript, StatusScript, TestScript } from "../../🏃️execution/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("deps", DepsScript)
  .register("setup", SetupScript)
  .register("status", StatusScript)
  .register("run", RunScript)
  .register("native", NativeScript)
  .register("epjson", EpJsonScript)
  .register("emit", EmitScript)
  .register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "status" });
