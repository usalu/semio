#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { lstat, readFile } from "node:fs/promises";
import { join, resolve, relative } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { stageRepositoryArtifacts, withRepositoryArtifactDestinationV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { FONT_ASSET, validateFontAsset } from "../../../../../../🔨️modules/🖼️canvas/🔤️fonts/🟦️.ts";

import { withRepositoryCacheStoreV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import { withArtifactScratchV1, withArtifactDestinationTaskV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🟦️.ts";
import { withRepositoryPublishedCargoExecutableV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { scriptInvocationEnvironment } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { scriptCompilerAdmissionControlV1, acquireCompilerCapacityLeaseV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandResultV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import { fontAssetPublicationRootV1 } from "../../../../../../🔨️modules/🖼️canvas/🔤️fonts/📦️publication/🟦️.ts";

import fontStorageSchema from "../../🔤️fonts/👓️reading/🧬️schema/🔣️.json" with { type: "json" };
const fontProtocol = fontStorageSchema.definitions.Protocol.const;
const ROOT = import.meta.dir;
const FONT_TOOL = "pack-typst-font-assets";

class FontsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("fonts takes no arguments");
    const workspace = this.repoRoot, mode = readBuildMode(process.env.SEMIO_BUILD_MODE), authority = readNativeCallerAuthorityV1(this.invocation.capabilities), phaseControl = scriptCompilerAdmissionControlV1(this.invocation, "font-assets:publication", fontProtocol.maximumElapsedMilliseconds), manifest = resolve(workspace, "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/Cargo.toml"), owner = relative(workspace, ROOT).replaceAll("\\", "/");
    if (mode !== authority.original.buildMode) throw Error("Original font asset build mode required");
    await withRepositoryCacheStoreV1(workspace, "products", this.invocation, phaseControl,products => withRepositoryCacheStoreV1(workspace, "diagnostics", this.invocation, products.control,diagnostics => withArtifactScratchV1({ workspace, invocation: this.invocation, storage: diagnostics.storage, control: diagnostics.control, phaseControl, request: { version: 1, mode, owner } }, destination => withArtifactDestinationTaskV1(destination, this.invocation, async sink => {
      const asset = join(sink.directory, FONT_ASSET);
      await withRepositoryPublishedCargoExecutableV1(manifest, FONT_TOOL + (process.platform === "win32" ? ".exe" : ""), workspace, mode, this.invocation, sink.control, async (artifact, control) => {
        const environment = scriptInvocationEnvironment(this.invocation, { ...artifact.environment, ...sink.environment }), lease = await acquireCompilerCapacityLeaseV1({ directory: join(workspace, ".cache/b/l/m/data"), control, environment, requestedParallelism: 1 });
        try {
          const result = await captureOwnedCommandResultV1(artifact.path, [asset], ROOT, { control, env: environment, output: { maximumBytes: Math.min(1048576, authority.transport.maximumBytes), maximumLines: Math.min(65536, authority.transport.maximumLines) } });
          if (result.status !== 0 || result.signal !== null) throw Error("Font asset tool failed: " + result.stderr.slice(-65536));
        } finally { lease.release(); }
      });
      const facts = await lstat(asset);
      if (!facts.isFile() || facts.isSymbolicLink() || facts.nlink !== 1 || facts.size > fontProtocol.maximumAssetBytes) throw Error("Finite physical font output required");
      const bytes = await readFile(asset, { signal: sink.control.signal }), count = validateFontAsset(bytes);
      await withRepositoryArtifactDestinationV1({ workspace, invocation: this.invocation, storage: products.storage, control: products.control, phaseControl: sink.control, directory: fontAssetPublicationRootV1(products.root, mode) }, target => stageRepositoryArtifacts(target, fontProtocol.owner, new Map([[FONT_ASSET, asset]])));
      console.log(`[DEBUG] Staged ${count} bounded fonts (${bytes.byteLength} bytes) mode=${mode}`);
    }))));
  }
}

/**
 * 🌍️ The two module trees the wgpu renderer actually paints from this crate — `world::` (board,
 * mesh3d, action queue, camera) and everything named `terrain`. The wgpu renderer parity lane runs
 * exactly this slice, so it is a target rather than a remembered command line; `--lib` because the
 * laws live in `#[cfg(test)]` modules of the library, not in `[[test]]` binaries.
 */
class TestWgpuWorldTerrainScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([], this.root, this.invocation, ["--lib", ...rest, "world::", "terrain"]);
  }
}

const router = new ScriptRouter(ROOT).register("fonts", FontsScript).register("test-wgpu-world-terrain", TestWgpuWorldTerrainScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
