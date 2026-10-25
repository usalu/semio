#!/usr/bin/env repo
/** 🛑️ Fixed first-party Print compiler-refusal inspector entry: `repo ./📜️script.ts <request path> <request sha256>` composes the compiled-probe runner into the inspector, so the compile modules reach no other Print command. */
import { isMainModuleV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import { withCompiledPrintProbeV1 } from "../🟦️.ts";
import { runPrintCompilerRefusalInspectorV1 } from "./🟦️.ts";

if(isMainModuleV1(import.meta.url))await runPrintCompilerRefusalInspectorV1(process.env,process.argv.slice(2),withCompiledPrintProbeV1);
