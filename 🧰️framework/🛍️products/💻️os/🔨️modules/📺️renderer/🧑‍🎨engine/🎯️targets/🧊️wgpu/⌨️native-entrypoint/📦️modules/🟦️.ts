import { open } from "node:fs/promises";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";
import { stageRepositoryArtifacts } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { fileDigest } from "../../../../../../🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { COMPONENT_RECEIPT_MAX_BYTES, componentReceiptBindsV1, parseComponentReceiptV1 } from "../../../../../../🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts";

/** 🧩️ One native component input: its shipped bytes, the JSON projection of its semantic descriptor, the semantic descriptor pack and
 * the `ComponentReceiptV1` binding that pack to those bytes. */
export type NativeModuleSource = { readonly pluginId: string; readonly wasm: string; readonly descriptor: string; readonly descriptorPack: string; readonly receipt: string };

/** 📂️ Isolates a completed native runtime by variant and compilation profile. */
export function nativeRuntimeDirectory(packageRoot: string, variant: string, profile: string): string {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant) || !["prototype", "release"].includes(profile)) throw new Error("Select a native runtime variant and prototype/release profile");
  return join(packageRoot, "dist/runtime/native", profile, variant);
}

/** 📦️ Publishes portable references to verified component-model WASM and matching descriptors. */
export async function publishNativeRuntime(packageRoot: string, variant: string, profile: string, modules: readonly NativeModuleSource[], signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  const output = nativeRuntimeDirectory(packageRoot, variant, profile), files = new Map<string, string>(), entries: { pluginId: string; wasmSha256: string; wasmPath: string; descriptorPath: string }[] = [];
  if (!modules.length || modules.length > 1024) throw new Error("Native runtime requires between 1 and 1024 components");
  for (const module of [...modules].sort((a, b) => a.pluginId.localeCompare(b.pluginId))) {
    signal.throwIfAborted();
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(module.pluginId) || entries.some(entry => entry.pluginId === module.pluginId)) throw new Error("Invalid or duplicate native plugin ID");
    for (const path of [module.wasm, module.descriptor, module.descriptorPack, module.receipt]) if (!lstatSync(path).isFile()) throw new Error("Native runtime inputs must be regular files");
    const handle = await open(module.wasm, "r");
    try {
      const header = Buffer.alloc(8), read = await handle.read(header, 0, 8, 0);
      if (read.bytesRead !== 8 || header.toString("hex") !== "0061736d0d000100") throw new Error("Native runtime requires component-model WASM");
    } finally { await handle.close(); }
    const wasmSha256 = await fileDigest(module.wasm, signal), descriptor = JSON.parse(readFileSync(module.descriptor, "utf8"));
    if (lstatSync(module.receipt).size > COMPONENT_RECEIPT_MAX_BYTES) throw new Error("Native component receipt exceeds its bound");
    const receipt = parseComponentReceiptV1(readFileSync(module.receipt, "utf8"));
    const binding = { pluginId: module.pluginId, packageId: receipt.packageId, wasmSha256, semanticDescriptorSha256: await fileDigest(module.descriptorPack, signal) };
    if (descriptor.manifest?.pluginId !== module.pluginId || "hashes" in descriptor || !componentReceiptBindsV1(receipt, binding) || receipt.wasmBytes !== lstatSync(module.wasm).size) throw new Error("Native component descriptor identity or digest mismatch");
    const paths = { wasmPath: relative(output, module.wasm).split(sep).join("/"), descriptorPath: relative(output, module.descriptor).split(sep).join("/") };
    if (Object.values(paths).some(path => !path || [...path].length > 4096 || path.startsWith("/") || /[\\:<>"|?*\u0000-\u001f]/.test(path) || path.split("/").some(part => !part || part === "."))) throw new Error("Native runtime inputs must have portable relative paths");
    entries.push({ pluginId: module.pluginId, wasmSha256, ...paths });
  }
  signal.throwIfAborted();
  const content = JSON.stringify({ version: 1, variant, profile, modules: entries }) + "\n";
  if (Buffer.byteLength(content) > 1024 * 1024) throw new Error("Native runtime manifest exceeds 1 MiB");
  mkdirSync(dirname(output), { recursive: true });
  const temporary = mkdtempSync(output + ".manifest-");
  try {
    const manifest = join(temporary, "🔣️runtime.json");
    writeFileSync(manifest, content);
    files.set("🔣️runtime.json", manifest);
    await stageRepositoryArtifacts(output, `native-runtime:${variant}:${profile}`, files, { signal });
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}
