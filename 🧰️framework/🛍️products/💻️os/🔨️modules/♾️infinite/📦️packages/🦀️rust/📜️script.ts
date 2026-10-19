#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ROOT, BuildScript, CheckScript, TestScript, FontsScript, TestWgpuWorldTerrainScript, OriginalBoardFillSourceScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


const router = new ScriptRouter(ROOT).register("build", BuildScript).register("check", CheckScript).register("test", TestScript).register("fonts", FontsScript).register("test-wgpu-world-terrain", TestWgpuWorldTerrainScript).register("test-original-board-fill-source",OriginalBoardFillSourceScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));