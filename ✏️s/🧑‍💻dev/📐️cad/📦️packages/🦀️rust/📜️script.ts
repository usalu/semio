#!/usr/bin/env repo
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {verifyProducedWasmPackageV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🌐️ Outward CAD browser composition and exact geometry ownership commands. */
import { runCargo } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { buildRepositoryWasmWebV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class BrowserScript extends BundleScript { async run():Promise<void> { const { geometryOwnershipLaws } = await import("../../🧪️tests/🌐️geometry-ownership/🟦️.ts"); geometryOwnershipLaws(); const { stepBrowserLaws } = await import("../../🧪️tests/📐️step-session/🟦️.ts"); await stepBrowserLaws(); } }
class SourceScript extends BundleScript { async run():Promise<void> { const { geometryOwnershipLaws } = await import("../../🧪️tests/🌐️geometry-ownership/🟦️.ts"); geometryOwnershipLaws(); } }
class CheckScript extends BundleScript { async run(args:string[]):Promise<void> { await runCargo(["check","-p","semio-s-dev-cad",...args],this.repoRoot); } }
class TestScript extends BundleScript { async run():Promise<void> { const { geometryOwnershipLaws } = await import("../../🧪️tests/🌐️geometry-ownership/🟦️.ts"); geometryOwnershipLaws(); await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 120_000 }, cwd:this.repoRoot, groups:[{package:"semio-s-dev-cad",target:{kind:"test",name:"step_session"},laws:["neutral_session_does_not_publish_step_operations","step_round_trip_preserves_prior_handles_and_import_claims","step_operations_obey_retirement_and_closed_authority","step_mesh_volume_agrees_with_independent_parry_oracle"]}] }); } }
class WasmScript extends BundleScript { async run():Promise<void> { await buildRepositoryWasmWebV1({producer:"browser-module",invocation:this.invocation,consume:verifyProducedWasmPackageV1, control: scriptCompilerAdmissionControlV1(this.invocation, "wasm:build", 0), rsDir:import.meta.dir,componentDir:import.meta.dir,logPrefix:"s/dev/cad",wasmBaseName:"cad_geometry",pkg:{name:"@semio-tech/s-cad-geometry-browser",files:["cad_geometry.js","cad_geometry_bg.wasm","cad_geometry.d.ts"],main:"cad_geometry.js",module:"cad_geometry.js",types:"cad_geometry.d.ts"}}); } }
const router = new ScriptRouter(import.meta.dir).register("source-check",SourceScript).register("check",CheckScript).register("test",TestScript).register("wasm",WasmScript).register("browser-test",BrowserScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand:"source-check" }) }));
