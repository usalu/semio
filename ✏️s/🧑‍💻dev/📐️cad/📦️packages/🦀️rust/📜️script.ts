#!/usr/bin/env bun
/** 🌐️ Outward CAD browser composition and exact geometry ownership commands. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runWasmPackWebBuild, runExactCargoLaws, buildBudgetMs } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { geometryOwnershipLaws } from "../../🧪️tests/🌐️geometry-ownership/🟦️.ts";
import { stepBrowserLaws } from "../../🧪️tests/📐️step-session/🟦️.ts";
class BrowserScript extends BundleScript { async run():Promise<void> { geometryOwnershipLaws(); await stepBrowserLaws(); } }
class SourceScript extends BundleScript { async run():Promise<void> { geometryOwnershipLaws(); } }
class CheckScript extends BundleScript { async run(args:string[]):Promise<void> { await runCargo(["check","-p","semio-s-dev-cad",...args],this.repoRoot); } }
class TestScript extends BundleScript { async run():Promise<void> { geometryOwnershipLaws(); await runExactCargoLaws({cwd:this.repoRoot,buildBudgetMs:buildBudgetMs(),lawBudgetMs:120_000,groups:[{package:"semio-s-dev-cad",target:{kind:"test",name:"step_session"},laws:["neutral_session_does_not_publish_step_operations","step_round_trip_preserves_prior_handles_and_import_claims","step_operations_obey_retirement_and_closed_authority","step_mesh_volume_agrees_with_independent_parry_oracle"]}]}); } }
class WasmScript extends BundleScript { async run():Promise<void> { await runWasmPackWebBuild({rsDir:import.meta.dir,logPrefix:"s/dev/cad",wasmBaseName:"cad_geometry",outputDirectory:"🕸️bindings",shipProfile:"wasm-release",pkg:{name:"@semio-tech/s-cad-geometry-browser",files:["cad_geometry.js","cad_geometry_bg.wasm","cad_geometry.d.ts"],main:"cad_geometry.js",module:"cad_geometry.js",types:"cad_geometry.d.ts"}}); } }
const router = new ScriptRouter(import.meta.dir).register("source-check",SourceScript).register("check",CheckScript).register("test",TestScript).register("wasm",WasmScript).register("browser-test",BrowserScript);
if (import.meta.main) await runBundleScriptMain(router,import.meta.url,{ defaultCommand:"source-check" });
