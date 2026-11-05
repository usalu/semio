import {playgroundActivationTargetV1} from "../🎯️target/🟦️.ts";
import {checkScriptInvocation,findWorkspaceRoot,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildBudgetMs} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import {runRepositoryCommand} from "@semio-tech/process/execution";

/** ♻️ Brings one playground variant's `renderer` runtime up to a publishable activation state by
 * running the build target that OWNS that closure — `activate-<variant>-<renderer>-<profile>`, whose declared
 * `dependsOn` (`…🦑️repo/🔨️modules/📚️library/🟨️.mjs` `playgroundPreparationTargets`) is the single
 * source of truth for the chain: every selected plugin's `component-<profile>` → `materialize-<profile>`,
 * `@semio-tech/framework-plugin-web:support-<profile>`, `semio-os-infinite:fonts`, the renderer's
 * own `wasm` producer(s), `@semio-tech/plugin-registry:session-<variant>`, then `prepare` and `activate`.
 * Delegating rather than re-listing that closure here is what makes "reuse whatever is already fresh"
 * the build cache's decision instead of a second, drifting freshness rule — react and wgpu share this one path. */
export async function activatePlaygroundRuntime(invocation:ScriptInvocation,variant: string, profile: "prototype" | "release", renderer: "react" | "wgpu"): Promise<void> {
  checkScriptInvocation(invocation);
  const target = playgroundActivationTargetV1(variant, profile, renderer);
  console.log(`[dev] activating ${variant} ${renderer} ${profile} via ${target}`);
  await runRepositoryCommand(invocation,"repo", ["run", target], findWorkspaceRoot(), `playground-activation:${variant}:${renderer}:${profile}`, {version:1,maximumElapsedMilliseconds:buildBudgetMs(),output:null});
}

