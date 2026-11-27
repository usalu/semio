#!/usr/bin/env repo
import { dirname, join, relative } from "node:path";
import { decodeCargoProvenanceV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { withCapturedCargoArtifactsTaskV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { buildAdmittedCargoArtifactsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { requestedCompilerAdmissionControlV1, scriptCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import { requireHostPlatformBuildArgumentsV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { readNativeCallerAuthorityV1, stageRepositoryArtifacts, withArtifactDestinationTaskV1, withArtifactScratchV1, withRepositoryArtifactDestinationV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { nativeTranspileBinaryNameV1, nativeTranspileBinaryV1, nativeTranspilePublicationPathV1, nativeTranspileRequiresBuildV1 } from "./🟦️.ts";

async function buildNativeTranspileV1(workspace: string, invocation: ScriptInvocation, platform: string, control: CompilerAdmissionControl = scriptCompilerAdmissionControlV1(invocation, "cargo:build", buildBudgetMs()), sourceRoot: string = workspace): Promise<void> {
  const authority = readNativeCallerAuthorityV1(invocation.capabilities);
  if (workspace !== authority.repositoryRoot) throw Error("Actual native transpile workspace required");
  const name = nativeTranspileBinaryNameV1(platform);
  const destination = nativeTranspilePublicationPathV1(workspace, authority.original.buildMode, platform);
  await buildAdmittedCargoArtifactsV1(join(sourceRoot, relative(workspace, import.meta.dirname), "Cargo.toml"), ["--bin", "semio-native-transpile"], workspace, {
    producer: "artifact",
    sourceRoot,
    invocation,
    control,
    consume: capture => withCapturedCargoArtifactsTaskV1(capture, async view => {
      const observation = decodeCargoProvenanceV1(view.observation);
      if (observation.status !== 0 || observation.cancelled) throw new Error("Completed native-transpile Cargo observation required");
      const executable = view.files.get(name);
      if (!executable) throw new Error(`cargo built no ${name} executable`);
      const mode = authority.original.buildMode;
      await withRepositoryCacheStoreV1(workspace, "products", invocation, view.control, (store, storePhase) => withArtifactScratchV1({ workspace, invocation, storage: store.storage, control: store.control, phaseControl: storePhase, request: { version: 1, mode, owner: "browser/tools/native-transpile" } }, scratch => withArtifactDestinationTaskV1(scratch, invocation, async sink => {
        await view.recheck();
        await withRepositoryArtifactDestinationV1({ workspace, invocation, storage: store.storage, control: store.control, phaseControl: sink.control, directory: dirname(destination) }, target => stageRepositoryArtifacts(target, "native-transpile", new Map([[name, executable]])));
      })));
      console.log(destination);
    }),
  });
}

/** 🥾️ Ensures the playground's actual build mode has a runnable native transpiler before component admission. */
export async function ensureNativeTranspileV1(workspace: string, invocation: ScriptInvocation, control: CompilerAdmissionControl, environment: Readonly<Record<string, string | undefined>> = process.env, sourceRoot: string = workspace): Promise<void> {
  const mode = readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode;
  const platform = process.platform;
  if (!nativeTranspileRequiresBuildV1(workspace, mode, environment, platform)) return;
  await buildNativeTranspileV1(workspace, invocation, platform, requestedCompilerAdmissionControlV1(control, buildBudgetMs()), sourceRoot);
  nativeTranspileBinaryV1(workspace, mode, environment, platform);
}

/** 🏗️ Builds the native transpile in-process under the received script invocation and stages the executable beside the other browser tools.
 * @see {@link ./🟦️.ts} */
class NativeTranspileBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const platform = segments.length ? requireHostPlatformBuildArgumentsV1(segments, process.platform) : process.platform;
    await buildNativeTranspileV1(this.repoRoot, this.invocation, platform);
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("build", NativeTranspileBuildScript)).run(process.argv.slice(2), original));
