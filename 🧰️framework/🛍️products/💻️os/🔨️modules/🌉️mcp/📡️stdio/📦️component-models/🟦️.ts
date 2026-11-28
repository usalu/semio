import { lstat, readFile } from "node:fs/promises";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { readNativeCallerAuthorityV1, withRepositoryArtifactGroupTaskV1, withRepositoryArtifactGroupV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { repositoryCachePartitionDirectoryV1, withRepositoryCacheStoreV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { componentModelDirectoryKeyV1, readComponentModelIntentV1 } from "../../../🔌️plugin/📦️packages/🦀️rust/📦️model/📋️contract/🟦️.ts";
import { resolveComponentModelRequestV1 } from "../../../🔌️plugin/📦️model/🧭️head/🟦️.ts";
import modelSchema from "../../../🔌️plugin/📦️model/🧬️schema/🔣️.json" with { type: "json" };
import { readRetainedComponentModelV1 } from "../../../🔌️plugin/📦️packages/🦀️rust/📦️model/👓️reading/🟦️.ts";
import { readNativeCatalogV1 } from "../../../🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import schema from "./🔣️.json" with { type: "json" };

const policy = schema.definitions.Protocol.const, layoutPolicy = modelSchema.definitions.Layout.const;
type Mode = "prototype" | "release";
type ModelLocation = Readonly<{ pluginId: string; cratePath: string; root: string; preparedActor?: { imageSha256: string; imageBytes: number } }>;
type ModelManifest = Readonly<{ version: 1; mode: Mode; models: readonly ModelLocation[] }>;

/** 📋️ Admits only mode-bound model locations for the native child. */
export function encodeMcpComponentModelManifestV1(value: unknown): string {
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Closed MCP component model manifest required");
  const manifest = value as ModelManifest;
  const seen = new Set<string>();
  for (const row of manifest.models) {
    const expected = join("browser", manifest.mode, layoutPolicy.directory, componentModelDirectoryKeyV1(row.cratePath));
    if (seen.has(row.pluginId) || !isAbsolute(row.root) || resolve(row.root) !== row.root || !dirname(row.root).endsWith(join("/", expected)) || !new RegExp(`^[a-f0-9]{${layoutPolicy.generationLength}}$`, "u").test(basename(row.root))) throw Error("Exact MCP component model location required");
    seen.add(row.pluginId);
  }
  const encoded = JSON.stringify(manifest);
  if (Buffer.byteLength(encoded) > policy.maximumBytes) throw Error("Finite MCP component model manifest required");
  return encoded;
}

/** 📦️ Retains and verifies the selected catalog's installed component models for the native child. */
export async function withMcpComponentEnvironmentV1<T>(workspace: string, invocation: ScriptInvocation, phase: ResourceLeaseControl, environment: NodeJS.ProcessEnv, body: (environment: NodeJS.ProcessEnv, control: ResourceLeaseControl) => Promise<T>): Promise<T> {
  requireIssuedResourceControlInvocationV1(phase, invocation);
  const catalogRoot = environment.SEMIO_PLUGIN_CATALOG;
  const childEnvironment = { ...environment };
  delete childEnvironment[policy.environment];
  if (!catalogRoot) return body(childEnvironment, phase);
  const authority = readNativeCallerAuthorityV1(invocation.capabilities);
  if (resolve(workspace) !== authority.repositoryRoot) throw Error("Actual MCP component workspace required");
  const mode = authority.original.buildMode;
  const catalog = readNativeCatalogV1(JSON.parse(await readFile(join(catalogRoot, "🦀️catalog.json"), { encoding: "utf8", signal: phase.signal })));
  try {
    const partition = await lstat(repositoryCachePartitionDirectoryV1(workspace, "products"));
    if (!partition.isDirectory()) throw Error("Original MCP products partition directory required");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    phase.signal.throwIfAborted();
    return body(childEnvironment, phase);
  }
  return withRepositoryCacheStoreV1(workspace, "products", invocation, phase, async (store, control) => {
    const available: { intent: ReturnType<typeof readComponentModelIntentV1>; request: Awaited<ReturnType<typeof resolveComponentModelRequestV1>> }[] = [];
    for (const row of catalog.plugins) {
      control.signal.throwIfAborted();
      const intent = readComponentModelIntentV1({ version: 1, mode, pluginId: row.pluginId, cratePath: row.cratePath });
      try {
        const request = await resolveComponentModelRequestV1(store.root, intent, control.signal);
        await lstat(request.path);
        available.push({ intent, request });
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      }
    }
    if (!available.length) return body(childEnvironment, control);
    return withRepositoryArtifactGroupV1({ workspace, invocation, storage: store.storage, control: store.control, phaseControl: control, requests: available.map(row => row.request) }, group =>
      withRepositoryArtifactGroupTaskV1(group, invocation, async retained => {
        const byPath = new Map(retained.artifacts.map(artifact => [artifact.path, artifact]));
        const models: ModelLocation[] = [];
        for (const { intent, request } of available) {
          const artifact = byPath.get(request.path);
          if (!artifact) throw Error("Retained MCP component model required");
          const model = await readRetainedComponentModelV1(intent, artifact);
          const { preparation } = model;
          const preparedActor = preparation ? { imageSha256: preparation.imageSha256, imageBytes: preparation.imageBytes } : undefined;
          models.push({ pluginId: intent.pluginId, cratePath: intent.cratePath, root: model.root, ...(preparedActor ? { preparedActor } : {}) });
          await retained.control.publish({ resource: "mcp:component-models", mode: "shared", elapsedMs: 0 });
        }
        childEnvironment[policy.environment] = encodeMcpComponentModelManifestV1({ version: 1, mode, models });
        return body(childEnvironment, retained.control);
      }));
  });
}

/** 🏗️ Builds and publishes the component model of one plugin through the owning component driver (facet "model"); a failed or cancelled build rejects. */
export async function buildMcpComponentModelV1(workspace: string, invocation: ScriptInvocation, pluginId: string, progress: (line: string) => void, signal: AbortSignal): Promise<void> {
  const [{ buildComponentClosureV1 }, { repositoryCargoNetworkV1 }, { scriptCompilerAdmissionControlV1 }] = await Promise.all([
    import("../../../🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"),
    import("../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts"),
    import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"),
  ]);
  signal.throwIfAborted();
  await buildComponentClosureV1(workspace, new Set([pluginId]), { invocation, facet: "model", network: repositoryCargoNetworkV1(invocation, workspace), control: scriptCompilerAdmissionControlV1(invocation, `mcp:component-model:${pluginId}`, 0, signal), log: progress });
  signal.throwIfAborted();
}
