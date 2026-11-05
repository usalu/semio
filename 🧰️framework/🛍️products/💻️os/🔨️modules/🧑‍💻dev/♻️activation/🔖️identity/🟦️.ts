import { createReadStream, statSync } from "node:fs";
import { createHash } from "node:crypto";
import { writeStagedSourceFreshness } from "@semio-tech/framework-plugin/build/freshness";
import type { ActivationArtifact } from "../🟦️.ts";

/** 🔏️ Includes every completed module/support byte in its runtime activation identity. */
async function activationFilesDigest(files: ReadonlyMap<string, string>, signal: AbortSignal): Promise<string> {
  const hash = createHash("sha256");
  for (const [name, path] of [...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) {
    signal.throwIfAborted();
    hash.update(JSON.stringify([name.replaceAll("\\", "/"), statSync(path).size]) + "\n");
    for await (const chunk of createReadStream(path, { signal })) hash.update(chunk);
  }
  return hash.digest("hex");
}

/** 🧮️ Observes completed module bytes and current bounded owner sources before receipt reuse. */
export async function collectActivationArtifactsV1(components: readonly Readonly<{ pluginId: string; files: ReadonlyMap<string, string>; sourceRoot: string; freshnessDirectory: string }>[], supportDigest: string, signal: AbortSignal): Promise<readonly ActivationArtifact[]> {
  const completed: ActivationArtifact[] = [];
  for (const component of components) {
    signal.throwIfAborted();
    const digest = await activationFilesDigest(component.files, signal);
    const sourceContentSha256 = await writeStagedSourceFreshness(component.freshnessDirectory, component.sourceRoot, signal);
    completed.push({ pluginId: component.pluginId, artifactSha256: createHash("sha256").update(supportDigest + digest).digest("hex"), sourceContentSha256 });
  }
  return completed;
}

export { activationFilesDigest };
