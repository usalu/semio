#!/usr/bin/env repo
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

if (isMainModuleV1(import.meta.url)) await runScriptProgram();
