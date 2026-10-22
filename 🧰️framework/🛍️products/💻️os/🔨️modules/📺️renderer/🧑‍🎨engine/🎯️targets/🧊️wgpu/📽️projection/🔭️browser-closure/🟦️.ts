import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative } from "node:path";
import { parseSemanticPackageBrowserProfile } from "@semio-tech/repo-lib/inventory";
import { runtimeBundleV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/🔬️source/🟦️.ts";
import { runtimeResolveV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";

/** 🔭️ One schema-owned workspace package binding of the browser profile. */
export interface WgpuBrowserBinding {
  readonly manifestPath: string;
  readonly entryPath: string;
}

/** 🔭️ The exact module set and workspace package bindings that the bundler reads for the profile entries. */
export interface WgpuBrowserClosure {
  readonly modules: readonly string[];
  readonly workspaceImports: Readonly<Record<string, WgpuBrowserBinding>>;
}

/** 🔭️ The two authorities that declare the closure: the generator contract and the Nx project inputs. */
export interface WgpuBrowserAuthority {
  readonly taxonomy: WgpuBrowserTaxonomy;
  readonly project: WgpuBrowserProject;
}

interface WgpuBrowserProfile {
  ownerPath: string;
  inlineTestDefine: string;
  entries: { sourceRelativePath: string }[];
  workspaceImports: Record<string, WgpuBrowserBinding>;
  sourceModulePaths: string[];
}

interface WgpuBrowserTaxonomy {
  pathEmojiPolicy: { genericEmojiIdentities: string[] };
  generatorContracts: Record<string, { inputPatterns: string[]; packageGeneration: { browserProfile: WgpuBrowserProfile } }>;
}

interface WgpuBrowserProject {
  namedInputs: { frameWorkerSources: unknown[] };
}

const contractId = "wgpu-frame-worker";
const generatedSegment = "/🤖️generated/";
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const taxonomyPath = `${library}/🔣️taxonomy.json`;
const projectPath = "📦️packages/🟦️typescript/📋️project.json";
const workspaceRootInput = "{workspaceRoot}/";
const byteCompare = (left: string, right: string): number => Buffer.from(left).compare(Buffer.from(right));
const posix = (path: string): string => path.replaceAll("\\", "/");
const packageName = (specifier: string): string => specifier.split("/").slice(0, specifier.startsWith("@") ? 2 : 1).join("/");

function nearestManifest(repoRoot: string, entryPath: string): string {
  for (let directory = dirname(entryPath); directory !== "."; directory = dirname(directory)) if (existsSync(join(repoRoot, directory, "package.json"))) return `${directory}/package.json`;
  return "package.json";
}

/** 🔭️ Reads the closure from the bundler itself: every module it loads and every bare specifier it resolves. */
export async function deriveWgpuBrowserClosure(repoRoot: string, profile: Pick<WgpuBrowserProfile, "ownerPath" | "inlineTestDefine" | "entries">): Promise<WgpuBrowserClosure> {
  const modules = new Set<string>(), specifiers = new Map<string, string>();
  for (const entry of profile.entries) {
    const result = await runtimeBundleV1({ root: repoRoot, entrypoints: [join(repoRoot, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": profile.inlineTestDefine }, plugins: [{ name: "wgpu-browser-closure", setup(builder) {
      builder.onResolve({ filter: /.*/u }, (request) => {
        if (!request.path.startsWith(".") && !isAbsolute(request.path)) specifiers.set(request.path, runtimeResolveV1(request.path, dirname(request.importer)));
        return undefined;
      });
      builder.onLoad({ filter: /.*/u }, (request) => {
        modules.add(posix(relative(repoRoot, request.path)));
        return undefined;
      });
    } }] });
    if (!result.success || result.logs.length) throw new Error(`WGPU browser closure of ${entry.sourceRelativePath} did not bundle cleanly: ${result.logs.map(String).join(" | ")}`);
  }
  const outside = [...modules].filter((path) => path.startsWith("..") || path.split("/").includes("node_modules"));
  if (outside.length) throw new Error(`WGPU browser closure reads modules outside the workspace sources: ${outside.join(" | ")}`);
  const workspaceImports: Record<string, WgpuBrowserBinding> = {};
  for (const specifier of [...specifiers.keys()].sort(byteCompare)) {
    const entryPath = posix(relative(repoRoot, specifiers.get(specifier)!)), manifestPath = nearestManifest(repoRoot, entryPath);
    const manifest = JSON.parse(readFileSync(join(repoRoot, manifestPath), "utf8")) as { name?: string; exports?: Record<string, unknown> };
    const name = packageName(specifier), subpath = `.${specifier.slice(name.length)}`;
    if (!modules.has(entryPath) || manifest.name !== name || manifest.exports?.[subpath] !== `./${posix(relative(dirname(manifestPath), entryPath))}`) throw new Error(`WGPU browser import ${specifier} is not an exact workspace package export: ${entryPath}`);
    workspaceImports[specifier] = { manifestPath, entryPath };
  }
  return { modules: [...modules].sort(byteCompare), workspaceImports };
}

/** 🧮️ Projects the closure onto the contract and the Nx inputs; anything not previously derived stays as authored. */
export function projectWgpuBrowserClosure(authority: WgpuBrowserAuthority, closure: WgpuBrowserClosure): WgpuBrowserAuthority {
  const { taxonomy, project } = structuredClone(authority), contract = taxonomy.generatorContracts[contractId]!, profile = contract.packageGeneration.browserProfile;
  const previous = new Set(profile.sourceModulePaths), source = closure.modules.filter((path) => !path.includes(generatedSegment));
  profile.sourceModulePaths = [...closure.modules];
  profile.workspaceImports = { ...closure.workspaceImports };
  contract.inputPatterns = [...new Set([...contract.inputPatterns.filter((path) => !previous.has(path)), ...closure.modules])].sort();
  const inputs = project.namedInputs.frameWorkerSources, derived = (input: unknown): boolean => typeof input === "string" && input.startsWith(workspaceRootInput) && previous.has(input.slice(workspaceRootInput.length));
  project.namedInputs.frameWorkerSources = [...inputs.filter((input) => !derived(input)), ...source.map((path) => workspaceRootInput + path)];
  return { taxonomy, project };
}

function readAuthority(repoRoot: string): { authority: WgpuBrowserAuthority; projectFile: string; taxonomyText: string; projectText: string } {
  const taxonomyText = readFileSync(join(repoRoot, taxonomyPath), "utf8"), taxonomy = JSON.parse(taxonomyText) as WgpuBrowserTaxonomy;
  const projectFile = join(repoRoot, taxonomy.generatorContracts[contractId]!.packageGeneration.browserProfile.ownerPath, projectPath), projectText = readFileSync(projectFile, "utf8");
  return { authority: { taxonomy, project: JSON.parse(projectText) as WgpuBrowserProject }, projectFile, taxonomyText, projectText };
}

const render = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;
const profileOf = (authority: WgpuBrowserAuthority): WgpuBrowserProfile => authority.taxonomy.generatorContracts[contractId]!.packageGeneration.browserProfile;

async function projectCurrent(repoRoot: string): Promise<{ fresh: ReturnType<typeof readAuthority>; projected: WgpuBrowserAuthority }> {
  const closure = await deriveWgpuBrowserClosure(repoRoot, profileOf(readAuthority(repoRoot).authority)), fresh = readAuthority(repoRoot), projected = projectWgpuBrowserClosure(fresh.authority, closure);
  parseSemanticPackageBrowserProfile(profileOf(projected), projected.taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  return { fresh, projected };
}

/** ✍️ Rewrites the contract and the Nx inputs from the bundler closure; the authorities are read again right before the write so concurrent edits survive. */
export async function generateWgpuBrowserClosure(repoRoot: string): Promise<{ taxonomy: boolean; project: boolean }> {
  const { fresh, projected } = await projectCurrent(repoRoot), taxonomyText = render(projected.taxonomy), projectText = render(projected.project);
  const changed = { taxonomy: taxonomyText !== fresh.taxonomyText, project: projectText !== fresh.projectText };
  if (changed.taxonomy) writeFileSync(join(repoRoot, taxonomyPath), taxonomyText);
  if (changed.project) writeFileSync(fresh.projectFile, projectText);
  return changed;
}

const difference = (left: readonly string[], right: readonly string[]): string[] => {
  const known = new Set(right);
  return left.filter((value) => !known.has(value));
};

/** ⚖️ Fails with the exact drift when the declared closure is not the bundler closure. */
export async function checkWgpuBrowserClosure(repoRoot: string): Promise<void> {
  const { fresh, projected } = await projectCurrent(repoRoot), declared = profileOf(fresh.authority), derived = profileOf(projected);
  const rows: [string, string[]][] = [
    ["modules missing from sourceModulePaths", difference(derived.sourceModulePaths, declared.sourceModulePaths)],
    ["modules the bundler does not read in sourceModulePaths", difference(declared.sourceModulePaths, derived.sourceModulePaths)],
    ["workspace imports missing", difference(Object.keys(derived.workspaceImports), Object.keys(declared.workspaceImports))],
    ["workspace imports the bundler does not read", difference(Object.keys(declared.workspaceImports), Object.keys(derived.workspaceImports))],
    ["inputPatterns missing", difference(projected.taxonomy.generatorContracts[contractId]!.inputPatterns, fresh.authority.taxonomy.generatorContracts[contractId]!.inputPatterns)],
    ["frameWorkerSources missing", difference(projected.project.namedInputs.frameWorkerSources.map(String), fresh.authority.project.namedInputs.frameWorkerSources.map(String))],
  ];
  if (render(projected.taxonomy) === fresh.taxonomyText && render(projected.project) === fresh.projectText) return;
  const problems = rows.filter(([, values]) => values.length).map(([name, values]) => `${name}: ${values.length} (${values.slice(0, 3).join(" | ")})`);
  throw new Error(`The WGPU browser closure declared in the generator contract is stale; run repo run @semio-tech/framework-renderer-wgpu:generate-browser-closure\n${problems.join("\n")}`);
}
