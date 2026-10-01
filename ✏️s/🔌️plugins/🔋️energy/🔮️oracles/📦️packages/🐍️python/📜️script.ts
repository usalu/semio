#!/usr/bin/env bun
/** 🔮️ Energy oracle package command router. */
import { ScriptRouter } from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
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
await runScriptMain(router, { defaultCommand: "status" });
