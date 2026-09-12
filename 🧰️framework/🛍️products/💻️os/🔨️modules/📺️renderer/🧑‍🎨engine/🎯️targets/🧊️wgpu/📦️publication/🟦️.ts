import { lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { generatorProjectedInputView, loadCatalogTaxonomy, parseGeneratorInputProjection, registryCatalogInputView, validateTaxonomy } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { canonicalJson } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
import { loadWgpuPackageTaxonomy, renderWgpuPackageArtifacts } from "../📽️projection/🟦️.ts";

/** ⚙️ Executes exact preview, generation, or freshness commands without writing outside six owned leaves. */
export async function runWgpuPackageGenerator(repoRoot: string, mode: string, options: { readonly taxonomy?: ReturnType<typeof loadCatalogTaxonomy>; readonly isCancelled?: () => boolean } = {}): Promise<void> {
  if (!["preview", "generate", "check"].includes(mode)) throw new Error("WGPU package mode must be preview, generate, or check");
  const taxonomy = options.taxonomy ?? loadWgpuPackageTaxonomy(repoRoot);
  const problems = validateTaxonomy(taxonomy);
  if (problems.length) throw new Error("WGPU package schema is invalid: " + problems.join(" | "));
  const contract = taxonomy.generatorContracts["wgpu-frame-worker"], protocol = contract?.packageGeneration?.previewInput;
  if (!protocol) throw new Error("WGPU package preview authority is absent");
  const cancelFile = process.env.SEMIO_GENERATOR_PREVIEW_CANCEL_FILE, base = registryCatalogInputView(repoRoot, taxonomy);
  const cancelPath = cancelFile ? relative(repoRoot, resolve(cancelFile)).replaceAll("\\", "/") : undefined;
  if (cancelFile && (!cancelPath || cancelPath.startsWith("../") || cancelPath === ".." || isAbsolute(cancelPath))) throw new Error("WGPU cancellation path is outside the repository");
  const isCancelled = (): boolean => Boolean(options.isCancelled?.() || cancelPath && registryCatalogInputView(repoRoot, taxonomy).kind(cancelPath) !== null);
  let view = base;
  const selected = process.env.SEMIO_GENERATOR_PREVIEW_PROTOCOL;
  if (selected) {
    if (mode !== "preview" || selected !== protocol.protocol) throw new Error("WGPU projected inputs require the exact preview protocol");
    const chunks: Uint8Array[] = [];
    let size = 0;
    for await (const chunk of Bun.stdin.stream()) {
      if (isCancelled()) throw new Error("WGPU package preview cancelled");
      size += chunk.byteLength;
      if (size > protocol.maxBytes) throw new Error("WGPU projected inputs exceed the declared byte bound");
      chunks.push(chunk);
    }
    view = generatorProjectedInputView(repoRoot, taxonomy, parseGeneratorInputProjection(Buffer.concat(chunks).toString("utf8"), taxonomy, "wgpu-frame-worker"), base);
  }
  const rendered = await renderWgpuPackageArtifacts(repoRoot, { taxonomy, view, isCancelled, ...(mode === "generate" ? { producerTarget: contract.target! } : {}) });
  if (mode === "preview") {
    const nodes = rendered.nodes.map(({ path, content, mode }) => ({ path, nodeKind: "file", mode, bytesBase64: Buffer.from(content).toString("base64") }));
    process.stdout.write(canonicalJson({ contractId: "wgpu-frame-worker", schemaVersion: 1, nodes, staleRemovals: [] }) + "\n");
    return;
  }
  const changed = rendered.nodes.filter((node) => {
    const kind = registryCatalogInputView(repoRoot, taxonomy).kind(node.path);
    if (kind === null) return true;
    const path = join(repoRoot, node.path);
    if (kind !== "file" || (lstatSync(path).mode & 0o7777) !== node.mode) throw new Error("WGPU output mode or no-follow kind drift: " + node.path);
    return readFileSync(path, "utf8") !== node.content;
  });
  if (mode === "check" && changed.length) throw new Error("WGPU package output is absent or stale: " + changed[0]!.path);
  for (const node of changed) {
    if (isCancelled()) throw new Error("WGPU package generation cancelled");
    const path = join(repoRoot, node.path);
    registryCatalogInputView(repoRoot, taxonomy).kind(node.path);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, node.content, { mode: node.mode });
  }
  process.stderr.write(`[wgpu] ${mode}: ${rendered.nodes.length} exact artifacts; ${changed.length} changed\n`);
}
