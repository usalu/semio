import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { canonicalJson } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
import { loadCatalogTaxonomy, parseCanonicalWgpuPackageCatalog, parseSemanticPackageBrowserProfile, registryCatalogInputView, resolveWorkspaceTaxonomyAuthority, validateTaxonomy, type RegistryCatalogInputView } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

/** 🌐️ Builds browser artifacts entirely in memory from exact no-follow source or projected inputs. */
export async function renderWgpuBrowserBundles(repoRoot: string, input: unknown, options: { readonly taxonomy?: ReturnType<typeof loadCatalogTaxonomy>; readonly view?: RegistryCatalogInputView; readonly isCancelled?: () => boolean; readonly progress?: (event: { readonly phase: "module-input" | "bundle"; readonly completed: number; readonly total: number }) => void; readonly entryIds?: readonly string[] } = {}): Promise<{ readonly nodes: readonly { readonly path: string; readonly content: string; readonly mode: number; readonly inclusion: "tracked" | "ignored" }[]; readonly inputs: readonly string[] }> {
  const taxonomy = options.taxonomy ?? loadCatalogTaxonomy(), profile = parseSemanticPackageBrowserProfile(input, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  const view = options.view ?? registryCatalogInputView(repoRoot, taxonomy);
  const allowed = new Set(profile.sourceModulePaths), modules = new Set<string>(), contents = new Map<string, string>();
  const nodes: { path: string; content: string; mode: number; inclusion: "tracked" | "ignored" }[] = [];
  const check = (): void => { if (options.isCancelled?.()) throw new Error("WGPU browser generation cancelled"); };
  const read = (path: string): string => {
    check();
    const cached = contents.get(path);
    if (cached !== undefined) return cached;
    if (view.kind(path) !== "file") throw new Error("WGPU browser input is missing or not a no-follow file: " + path);
    const content = view.readText(path);
    contents.set(path, content);
    return content;
  };
  check();
  const entries = options.entryIds ? profile.entries.filter((entry) => options.entryIds!.includes(entry.id)) : profile.entries;
  if (!entries.length || options.entryIds?.some((id) => !entries.some((entry) => entry.id === id))) throw new Error("Unknown WGPU browser entry selection");
  for (const entry of entries) {
    check();
    const entryPath = profile.ownerPath + "/" + entry.sourceRelativePath, entryAbsolute = join(repoRoot, entryPath);
    const result = await Bun.build({ root: repoRoot, entrypoints: [entryAbsolute], target: "browser", format: "esm", define: { "import.meta.vitest": profile.inlineTestDefine }, plugins: [{ name: "semantic-wgpu-owned-inputs", setup(builder) {
      builder.onResolve({ filter: /.*/u }, (request) => {
        check();
        const binding = profile.workspaceImports[request.path];
        if (binding) {
          const manifest = JSON.parse(read(binding.manifestPath));
          if (manifest.name !== request.path || manifest.exports?.["."] !== "./" + relative(dirname(binding.manifestPath), binding.entryPath).replaceAll("\\", "/")) throw new Error("WGPU browser workspace manifest drift: " + request.path);
        }
        const path = request.path === entryAbsolute ? entryPath : request.path.startsWith(".") ? relative(repoRoot, resolve(repoRoot, dirname(request.importer), request.path)).replaceAll("\\", "/") : binding?.entryPath;
        if (!path || !allowed.has(path)) throw new Error("WGPU browser import is not schema-owned: " + request.path + " in " + request.importer);
        return { path, namespace: "owned-wgpu" };
      });
      builder.onLoad({ filter: /.*/u, namespace: "owned-wgpu" }, (request) => {
        if (!allowed.has(request.path)) throw new Error("WGPU browser module is not schema-owned: " + request.path);
        const content = read(request.path);
        modules.add(request.path);
        options.progress?.({ phase: "module-input", completed: modules.size, total: allowed.size });
        check();
        return { contents: content, loader: request.path.endsWith(".tsx") ? "tsx" : request.path.endsWith(".json") ? "json" : "ts" };
      });
    } }] });
    check();
    if (!result.success || result.logs.length || result.outputs.length !== 1) throw new Error("WGPU browser compilation did not produce exactly one clean artifact: " + entry.id);
    nodes.push({ path: profile.ownerPath + "/" + entry.outputRelativePath, content: await result.outputs[0]!.text(), mode: 0o644, inclusion: entry.inclusion });
    options.progress?.({ phase: "bundle", completed: nodes.length, total: entries.length });
  }
  check();
  const unused = [...allowed].filter((path) => !modules.has(path));
  if (!options.entryIds && unused.length) throw new Error("WGPU browser module authority includes unread inputs: " + unused.join(" | "));
  for (const [path, content] of contents) if (view.kind(path) !== "file" || createHash("sha256").update(view.readText(path)).digest("hex") !== createHash("sha256").update(content).digest("hex")) throw new Error("WGPU browser input changed during generation: " + path);
  const compare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
  return { nodes: nodes.sort((left, right) => compare(left.path, right.path)), inputs: [...contents.keys()].sort(compare) };
}

/** 🥖️ Requires one exact root package-manager identity for all WGPU artifact commands. */
export function assertWgpuPackageToolchain(packageManager: unknown, actualVersion: string = Bun.version): string {
  const expected = typeof packageManager === "string" ? /^bun@(\d+\.\d+\.\d+)$/u.exec(packageManager)?.[1] : undefined;
  if (!expected || actualVersion !== expected) throw new Error("WGPU generation requires the exact root Bun packageManager identity");
  return expected;
}

export function loadWgpuPackageTaxonomy(repoRoot: string): ReturnType<typeof loadCatalogTaxonomy> {
  const taxonomy = JSON.parse(readFileSync(resolveWorkspaceTaxonomyAuthority(repoRoot).taxonomyPath, "utf8"));
  const problems = validateTaxonomy(taxonomy);
  if (problems.length) throw new Error("WGPU package schema is invalid: " + problems.join(" | "));
  return taxonomy;
}

/** 🏗️ Renders all canonical artifacts or one exclusive Nx producer from the same package authority. */
export async function renderWgpuPackageArtifacts(repoRoot: string, options: { readonly taxonomy?: ReturnType<typeof loadCatalogTaxonomy>; readonly view?: RegistryCatalogInputView; readonly isCancelled?: () => boolean; readonly progress?: (event: { readonly phase: "module-input" | "bundle"; readonly completed: number; readonly total: number }) => void; readonly producerTarget?: string } = {}) {
  const taxonomy: ReturnType<typeof loadCatalogTaxonomy> = options.taxonomy ?? (options.producerTarget ? JSON.parse(readFileSync(resolveWorkspaceTaxonomyAuthority(repoRoot).taxonomyPath, "utf8")) : loadWgpuPackageTaxonomy(repoRoot)), contract = taxonomy.generatorContracts["wgpu-frame-worker"];
  if (!contract?.packageGeneration) throw new Error("WGPU package generation lacks exact current catalog authority");
  const generation = contract.packageGeneration, view = options.view ?? registryCatalogInputView(repoRoot, taxonomy), contents = new Map<string, string>();
  const check = (): void => { if (options.isCancelled?.()) throw new Error("WGPU package generation cancelled"); };
  const read = (path: string): string => { check(); if (view.kind(path) !== "file") throw new Error("WGPU canonical input is missing or not a no-follow file: " + path); const content = view.readText(path); contents.set(path, content); return content; };
  check();
  const roots = contract.outputRoots.filter((output) => !options.producerTarget || (output.producer?.target ?? contract.target) === options.producerTarget);
  if (!roots.length) throw new Error("WGPU producer owns no outputs: " + options.producerTarget);
  const paths = new Set(roots.map((output) => output.path));
  const profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  const browserIds = profile.entries.filter((entry) => paths.has(profile.ownerPath + "/" + entry.outputRelativePath)).map((entry) => entry.id);
  const packageSelected = roots.some((output) => !profile.entries.some((entry) => output.path === profile.ownerPath + "/" + entry.outputRelativePath));
  const declarations: { path: string; content: string; mode: number; inclusion: "tracked" }[] = [];
  if (packageSelected) {
    const problems = validateTaxonomy(taxonomy);
    if (problems.length) throw new Error("WGPU package schema is invalid: " + problems.join(" | "));
    const owner = parseCanonicalWgpuPackageCatalog(read(generation.catalogPath), generation.catalogSha256, generation.browserProfile, taxonomy);
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json");
    const cargoPackageRoot = owner.ownerPath + "/" + owner.packageRelativePath, nodePackageRoot = owner.ownerPath + "/" + owner.nodePackageRelativePath;
    const cargo = Bun.TOML.parse(read(cargoPackageRoot + "/Cargo.toml")) as { package?: { name?: string; build?: string }; lib?: { path?: string }; bin?: { name?: string; path?: string }[] };
    const node = JSON.parse(read(nodePackageRoot + "/package.json")), nx = JSON.parse(read(nodePackageRoot + "/📋️project.json"));
    const invalidCwd = Object.values(nx?.targets ?? {}).some((target: any) => target.options?.cwd !== undefined && ![nodePackageRoot, ".", "{projectRoot}"].includes(target.options.cwd));
    if (cargo.package?.name !== owner.identity.cargoPackageName || cargo.package?.build !== owner.entryPaths.cargoBuild || cargo.lib?.path !== owner.entryPaths.cargoLibrary || !cargo.bin?.some((entry) => entry.name === "semio-wgpu-native" && entry.path === owner.entryPaths.cargoBinary) || node.name !== owner.identity.nodePackageName || node.exports?.["."] !== "./" + owner.entryPaths.nodeLibrary || nx.name !== owner.identity.nxProjectName || nx.sourceRoot !== owner.ownerPath || !nx.targets || invalidCwd) throw new Error("WGPU current package manifest identity drift");
    read(cargoPackageRoot + "/" + owner.entryPaths.cargoLibrary);
    read(cargoPackageRoot + "/" + owner.entryPaths.cargoBuild);
    for (const artifact of owner.artifacts) if (artifact.targetRelativePath) read(owner.ownerPath + "/" + artifact.targetRelativePath);
    declarations.push(...owner.artifacts.filter((artifact) => paths.has(owner.ownerPath + "/" + artifact.relativePath)).map((artifact) => ({ path: owner.ownerPath + "/" + artifact.relativePath, content: artifact.content, mode: 0o644, inclusion: "tracked" as const })));
  }
  const rootManifest = read("package.json");
  assertWgpuPackageToolchain(JSON.parse(rootManifest).packageManager);
  const browser = browserIds.length ? await renderWgpuBrowserBundles(repoRoot, generation.browserProfile, { ...options, taxonomy, view, ...(options.producerTarget ? { entryIds: browserIds } : {}) }) : { nodes: [], inputs: [] };
  const nodes = [...declarations, ...browser.nodes].sort((left, right) => Buffer.compare(Buffer.from(left.path), Buffer.from(right.path)));
  if (canonicalJson(nodes.map(({ path, inclusion }) => ({ path, inclusion }))) !== canonicalJson(roots.map(({ path, inclusion }) => ({ path, inclusion })))) throw new Error("WGPU generated roots disagree with the exact package catalog");
  for (const node of nodes) { const kind = view.kind(node.path); if (kind !== null && kind !== "file") throw new Error("WGPU generated output is not a no-follow file: " + node.path); }
  for (const [path, content] of contents) if (view.kind(path) !== "file" || createHash("sha256").update(view.readText(path)).digest("hex") !== createHash("sha256").update(content).digest("hex")) throw new Error("WGPU current catalog input changed during generation: " + path);
  check();
  return { nodes, inputs: [...new Set([...contents.keys(), ...browser.inputs])].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right))) };
}
