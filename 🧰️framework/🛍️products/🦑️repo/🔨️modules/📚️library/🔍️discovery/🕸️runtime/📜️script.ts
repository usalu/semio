#!/usr/bin/env bun
/** 🧮️ Runs the complete canonical compiler resource receiving laws in place. */
import {resolve} from "node:path";
import {mkdirSync} from "node:fs";
import {runOwnedCommand} from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
if(process.argv.slice(2).join(" ")!=="test")throw Error("Expected runtime resource test command");
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output||resolve(output).length>129)throw Error("Explicit bounded runtime fixture output required");
mkdirSync(resolve(output),{recursive:true});
await runOwnedCommand(process.execPath,["test",resolve(import.meta.dir,"🧪️tests/🟦️.ts"),resolve(import.meta.dir,"🎭️selection/🧪️tests/🟦️.ts")],resolve(import.meta.dir,"../../../../../../.."),"runtime:resources",600000,{env:process.env});
