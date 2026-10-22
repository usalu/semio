#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🔮️ Energy oracle package command router. */
import { ScriptRouter } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
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
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "status" }) }));
