import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import { rustTokenPairs, rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { readRustSourceClosureV1, type RustSourceInputV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { identifierSet, parseRustSourceTree, demoteDanglingOuterDocs, prunedUse, rustItemStartWithComments, rustOuterDocStart, useLeafNames, type RustModuleNode, type RustSourceItem, type RustSourceTree } from "../🌳️module-tree/🟦️.ts";
import { classifyPackageSource, loadCatalogTaxonomy } from "../../../🔍️discovery/🟦️.ts";
import { buildSlices, collectDecls, type CarveSlice, type ModuleDecl } from "../🧱️decl-tree/🟦️.ts";
import { rebaseManifestDirPaths, rebaseRustIncludes, renderMirrorFiles, renderMirrorRoot, relativePosix, type MirrorImport, type MirrorSpec, type MirrorUnit } from "../📝️render/🟦️.ts";
import { transformTestClosure, type TestTransformOptions } from "../🧪️test-closure/🟦️.ts";
import { convertMutationLeaves, flattenUse } from "../🧬️mutation-families/🟦️.ts";
import { MeasureUnion, totalsFit, type FileMeasure } from "../📏️measure-union/🟦️.ts";

/** 📏️ Source-gate budget of one compilation unit. */
export type CrateBudget = Readonly<{ files: number; bytes: number; lines: number }>;

/** 🎯️ The size every authored crate aims for; the hard gate leaves headroom for growth. */
export const CRATE_TARGET_BUDGET: CrateBudget = { files: 28, bytes: 230000, lines: 3500 };

/** 🚧️ The compiler-ingress source boundary. */
export const CRATE_HARD_BUDGET: CrateBudget = { files: 32, bytes: 262144, lines: 4000 };

/** 📐️ Measured closure of one crate in one cfg. */
export type CrateMeasure = Readonly<{ files: number; bytes: number; lines: number; embedded: number; paths: readonly string[]; sourcePaths: readonly string[]; entries: ReadonlyMap<string, FileMeasure> }>;

/** 🍰️ One planned crate: the slices it owns and everything its mirror root needs. */
export type PlannedCrate = {
  readonly id: string;
  readonly name: string;
  readonly packageDir: string;
  readonly description: string;
  readonly doc: string;
  readonly slices: readonly CarveSlice[];
  readonly attach: readonly ModuleDecl[];
  readonly dependsOn: readonly string[];
  readonly imports: readonly MirrorImport[];
  readonly itemImportLines: readonly string[];
  readonly externLines: readonly string[];
  readonly innerAttributes: readonly string[];
  readonly movedItemsText: string;
  readonly hoisted?: boolean;
  readonly lib: CrateMeasure | null;
  readonly test: CrateMeasure | null;
};

/** ✂️ A declaration in a file that moves to another crate and is replaced by a re-export. */
export type DeclCut = { readonly file: string; readonly start: number; readonly end: number; readonly text: string; readonly anchor?: { readonly eff: readonly string[]; readonly crate: string; readonly visibility: string; readonly component: boolean } };

/** 🗺️ The complete carve plan of one artifact. */
export type ArtifactCratePlan = {
  readonly repoRoot: string;
  readonly artifactDir: string;
  readonly artifactCrate: string;
  readonly packageDir: string;
  readonly rootFile: string;
  readonly tree: RustSourceTree;
  readonly crates: readonly PlannedCrate[];
  readonly wrapperNodes: ReadonlyMap<string, RustModuleNode>;
  readonly rootEdits: readonly RootEdit[];
  readonly weave: readonly WovenExport[];
  readonly placement: ReadonlyMap<string, string>;
  readonly fileCuts: readonly DeclCut[];
  readonly movedUnits: readonly { readonly names: readonly string[]; readonly blocks: readonly { readonly start: number; readonly end: number }[]; readonly to: string }[];
  readonly movedAppends: readonly { readonly file: string; readonly text: string; readonly anchor: string }[];
  readonly keptModules: readonly string[];
  readonly overlay: { readonly edits: ReadonlyMap<string, string>; readonly creates: ReadonlyMap<string, string>; readonly deletes: ReadonlySet<string>; readonly notes: readonly string[] };
  readonly rootMeasure: { readonly lib: CrateMeasure; readonly test: CrateMeasure } | null;
  readonly problems: readonly string[];
  readonly blockers: readonly PlanBlocker[];
  readonly packing: PackingStats;
};

/** ✂️ An edit of the artifact root; edits that replace a carved declaration remember it so the facade can weave children into the replacement. */
export type RootEdit = { readonly start: number; readonly end: number; readonly text: string; readonly anchor?: { readonly eff: readonly string[]; readonly crate: string; readonly visibility: string; readonly component: boolean } };

/** 🧵️ A module whose mount was cut out of its parent because the parent never uses it; the facade exports it at its original path. */
export type WovenExport = { readonly eff: readonly string[]; readonly crate: string };

/** 🧱️ A set of slices the orphan rule or a dependency cycle forces into one crate although together they exceed the budget. */
export type PlanBlocker = { readonly slices: number; readonly files: number; readonly bytes: number; readonly lines: number; readonly coherence: readonly string[]; readonly cycles: readonly string[]; readonly sample: readonly string[]; readonly causes: readonly string[]; readonly edges: readonly string[]; readonly rings: readonly string[]; readonly groupReport: readonly string[] };

/** 📦️ How the packing loop decided: merges rejected by the per-file lower bound, merges that needed a full re-measure of the merged closure, and merges accepted. */
export type PackingStats = { pruned: number; measured: number; merged: number };

const HOST_CFG: { value: string[] | null } = { value: null };

/** 🧭️ Plans the crate split of one artifact; when the plain split cannot meet the budget and the artifact has mutation aggregates, the plan is retried with the mutation leaves converted to leaves generic over their aggregate and the plan with fewer problems wins. */
export async function planArtifactCrates(repoRoot: string, packageDirRel: string, options: PlanOptions = {}): Promise<ArtifactCratePlan> {
  const mode = options.mutationFamilies ?? "auto";
  if (mode === "on") return planArtifactCratesOnce(repoRoot, packageDirRel, options, true);
  const plain = await planArtifactCratesOnce(repoRoot, packageDirRel, options, false);
  if (mode === "off" || !plain.problems.some((problem) => problem.includes("exceeds the hard budget"))) return plain;
  const converted = await planArtifactCratesOnce(repoRoot, packageDirRel, options, true).catch(() => null);
  if (converted === null) return plain;
  return converted.overlay.notes.some((note) => note.startsWith("mutation aggregate")) && converted.problems.length < plain.problems.length ? converted : plain;
}

/** 🖥️ Host cfg snapshot used by the census and the gate. */
export function crateTreeHostCfg(): readonly string[] {
  if (HOST_CFG.value) return HOST_CFG.value;
  const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
  const cfg = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((value) => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
  return (HOST_CFG.value = [...cfg, "debug_assertions"]);
}

/** 📐️ Measures a crate root through the canonical source-closure counter, with optional in-memory files. */
export async function measureRustCrate(repoRoot: string, entry: string, withTest: boolean, overlay: ReadonlyMap<string, string> = new Map(), manifestDirectory?: string): Promise<CrateMeasure> {
  const read = async (path: string): Promise<Uint8Array | null> => {
    const virtual = overlay.get(path);
    if (virtual !== undefined) return Buffer.from(virtual, "utf8");
    try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
  };
  const cfg = [...crateTreeHostCfg(), ...(withTest ? ["test"] : [])];
  const inputs = await readRustSourceClosureV1([entry], repoRoot, cfg, read, { manifestDirectory: manifestDirectory ?? dirname(entry), generated: () => undefined });
  return summarize(inputs);
}

function summarize(inputs: readonly RustSourceInputV1[]): CrateMeasure {
  return { files: inputs.length, bytes: inputs.reduce((sum, input) => sum + input.bytes, 0), lines: inputs.reduce((sum, input) => sum + input.activeLines, 0), embedded: inputs.filter((input) => input.kind === "embedded").length, paths: inputs.map((input) => input.path), sourcePaths: inputs.filter((input) => input.kind === "source").map((input) => input.path), entries: new Map(inputs.map((input) => [input.path, { bytes: input.bytes, lines: input.activeLines }])) };
}

/** ✅️ Whether a measure fits a budget. */
export function measureFits(measure: CrateMeasure, budget: CrateBudget): boolean {
  return measure.files <= budget.files && measure.bytes <= budget.bytes && measure.lines <= budget.lines;
}

/** 🧭️ Absolute directory that nested `#[path]` attributes of an inline module resolve against. */
export function inlineModuleBase(parentBase: string, node: RustModuleNode): string {
  return resolve(parentBase, node.pathAttribute ?? node.name);
}

/** 📂️ Absolute file of an external module declared in a file or inline scope. */
export function externalModuleFile(fileDir: string, inlineBase: string, inFileScope: boolean, node: RustModuleNode): string | null {
  return node.pathAttribute === null ? null : resolve(inFileScope ? fileDir : inlineBase, node.pathAttribute);
}

/** 🔗️ Chains of `crate::a::b` paths in one source text; group members yield one chain per first segment. */
export function crateRelativeChains(source: string): string[][] {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const chains: string[][] = [];
  for (let index = 0; index + 2 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "crate" || tokens[index + 1]!.text !== "::" || (index > 0 && tokens[index - 1]!.text === "::" && tokens[index - 2]?.kind === "identifier" && tokens[index - 2]!.text !== "self")) continue;
    if (tokens[index - 1]?.text === "extern") continue;
    let cursor = index + 2;
    const chain: string[] = [];
    while (tokens[cursor]?.kind === "identifier") {
      chain.push(tokens[cursor]!.text);
      if (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") cursor += 2;
      else { cursor += 1; break; }
    }
    if (chain.length) chains.push(chain);
    if (tokens[cursor - 0]?.text === "::" && tokens[cursor + 1]?.text === "{") {
      const close = pairs.get(cursor + 1);
      if (close !== undefined) groupHeads(tokens, pairs, cursor + 2, close, chain, chains);
    } else if (!chain.length && tokens[index + 2]?.text === "{") {
      const close = pairs.get(index + 2);
      if (close !== undefined) groupHeads(tokens, pairs, index + 3, close, [], chains);
    }
  }
  return chains;
}

function groupHeads(tokens: ReturnType<typeof rustTokens>, pairs: ReadonlyMap<number, number>, from: number, to: number, prefix: readonly string[], chains: string[][]): void {
  for (let cursor = from; cursor < to;) {
    if (tokens[cursor]!.text === ",") { cursor += 1; continue; }
    const chain = [...prefix];
    let step = cursor;
    while (tokens[step]?.kind === "identifier" && step < to) {
      chain.push(tokens[step]!.text);
      if (tokens[step + 1]?.text === "::" && step + 2 < to) {
        if (tokens[step + 2]!.text === "{") {
          const close = pairs.get(step + 2);
          if (close !== undefined) groupHeads(tokens, pairs, step + 3, close, chain, chains);
          step = (close ?? step + 2) + 1;
          chain.length = 0;
          break;
        }
        step += 2;
      } else { step += 1; break; }
    }
    if (chain.length > prefix.length) chains.push(chain);
    while (step < to && tokens[step]!.text !== ",") step = (pairs.get(step) !== undefined && pairs.get(step)! > step) ? pairs.get(step)! + 1 : step + 1;
    cursor = step;
  }
}

/** 🔍️ Finds the Cargo package whose library owns a directory: the nearest ancestor package, or the artifact package of the nearest workspace root. */
export function resolveOwningCrate(repoRoot: string, directoryRel: string): { manifest: string; name: string; entry: string } | null {
  let cursor = resolve(repoRoot, directoryRel);
  const root = resolve(repoRoot);
  while (cursor.startsWith(root) && cursor !== dirname(root)) {
    const manifest = join(cursor, "Cargo.toml");
    if (existsSync(manifest)) {
      const text = readFileSync(manifest, "utf8");
      if (/^\[package\]/mu.test(text)) { const lib = readCargoLib(manifest); return { manifest, ...lib }; }
      const packageManifest = join(cursor, "📦️packages/🦀️rust/Cargo.toml");
      if (/^\[workspace\]/mu.test(text) && existsSync(packageManifest)) { const lib = readCargoLib(packageManifest); return { manifest: packageManifest, ...lib }; }
    }
    const parent = dirname(cursor);
    if (parent === cursor) break;
    cursor = parent;
  }
  return null;
}

/** 🔍️ Reads the lib package name and entry file of a Cargo manifest. */
export function readCargoLib(manifestPath: string): { name: string; entry: string } {
  const parsed = Bun.TOML.parse(readFileSync(manifestPath, "utf8")) as { package?: { name?: string }; lib?: { path?: string } };
  if (!parsed.package?.name) throw new Error(`Cargo manifest has no package name: ${manifestPath}`);
  return { name: parsed.package.name, entry: resolve(dirname(manifestPath), parsed.lib?.path ?? "src/lib.rs") };
}

/** 🏷️ Slug of a facet directory name, stripping emoji. */
export function facetSlug(dirName: string): string {
  return dirName.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^a-z0-9]+/giu, "-").replace(/^-|-$/gu, "").toLowerCase();
}

/** 🔤️ Cargo crate name to its Rust identifier. */
export function crateIdent(name: string): string {
  return name.replaceAll("-", "_");
}

/** 📝️ Collects `extern crate X as Y;` rows of a root source. */
export function externCrateRows(source: string, items: readonly RustSourceItem[]): { crate: string; alias: string; start: number; end: number }[] {
  const rows: { crate: string; alias: string; start: number; end: number }[] = [];
  for (const item of items) {
    if (item.kind !== "extern-crate") continue;
    const match = /extern\s+crate\s+([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?\s*;/u.exec(source.slice(item.declStart, item.end));
    if (match) rows.push({ crate: match[1]!, alias: match[2] ?? match[1]!, start: item.start, end: item.end });
  }
  return rows;
}

/** 🏷️ Names defined or imported at the top level of a module body, with their items. */
export function topLevelNames(source: string, items: readonly RustSourceItem[]): Map<string, RustSourceItem> {
  const names = new Map<string, RustSourceItem>();
  for (const item of items) {
    if (item.name && item.kind !== "use" && item.kind !== "extern-crate" && item.kind !== "impl") names.set(item.name, item);
    if (item.kind === "use") for (const leaf of useLeafNames(source.slice(item.declStart, item.end))) names.set(leaf, item);
  }
  return names;
}

/** 🧪️ Source text of every file of a measure, keyed by repo-relative path. */
export function readMeasureSources(repoRoot: string, measure: CrateMeasure, overlay: ReadonlyMap<string, string> = new Map()): Map<string, string> {
  const out = new Map<string, string>();
  for (const path of measure.sourcePaths) {
    if (!path.endsWith(".rs")) continue;
    const absolute = resolve(repoRoot, path);
    const virtual = overlay.get(absolute);
    if (virtual !== undefined) out.set(path, virtual);
    else if (existsSync(absolute)) out.set(path, readFileSync(absolute, "utf8"));
  }
  return out;
}


/** 🧮️ Options of the planner. */
export type PlanOptions = Readonly<{ target?: CrateBudget; hard?: CrateBudget; tests?: TestTransformOptions; mutationFamilies?: "off" | "on" | "auto"; weave?: boolean; incrementalMeasure?: boolean }>;

function lcaDirectory(files: readonly string[]): string {
  const parts = files.map((file) => dirname(file).split("/"));
  const first = parts[0] ?? [];
  let length = first.length;
  for (const other of parts) { let i = 0; while (i < length && i < other.length && other[i] === first[i]) i += 1; length = i; }
  return first.slice(0, length).join("/");
}

function facetEmoji(dirName: string): string {
  const match = /^(\p{Extended_Pictographic}\uFE0F?|\p{Emoji_Presentation})/u.exec(dirName);
  return match ? match[1]! : "🧩️";
}

export function allItems(text: string): RustSourceItem[] {
  const tree = parseRustSourceTree(text);
  const out: RustSourceItem[] = [...tree.items];
  const visit = (nodes: readonly RustModuleNode[]): void => { for (const node of nodes) if (node.inline) { out.push(...node.items); visit(node.children); } };
  visit(tree.modules);
  return out;
}

function removeRange(text: string, start: number, end: number): { start: number; end: number } {
  let right = end;
  while (text[right] === " " || text[right] === "\t") right += 1;
  if (text[right] === "\n") right += 1;
  return { start: rustOuterDocStart(text, start), end: right };
}

type SliceInfo = {
  slice: CarveSlice;
  index: number;
  effs: string[];
  lib: CrateMeasure | null;
  test: CrateMeasure | null;
  libEdges: Set<string>;
  testEdges: Set<string>;
  rootRefs: Set<string>;
  testRootRefs: Set<string>;
  keptRefs: Set<string>;
  shimRefs: Set<string>;
  testShimRefs: Set<string>;
  heads: Set<string>;
  coherence: Set<string>;
  why: Map<string, string>;
  firstFile: string;
};

/** 🗺️ Plans the path-preserving carve of one artifact into crates cut along declarations, honouring coherence and the budget. */
async function planArtifactCratesOnce(repoRoot: string, packageDirRel: string, options: PlanOptions, convert: boolean): Promise<ArtifactCratePlan> {
  const target = options.target ?? CRATE_TARGET_BUDGET;
  const hard = options.hard ?? CRATE_HARD_BUDGET;
  const manifestPath = resolve(repoRoot, packageDirRel, "Cargo.toml");
  const lib = readCargoLib(manifestPath);
  const manifest = Bun.TOML.parse(readFileSync(manifestPath, "utf8")) as { package?: { workspace?: string } };
  const oldRootDir = dirname(lib.entry);
  const artifactDir = resolve(dirname(manifestPath), manifest.package?.workspace ?? ".");
  const originalRootSource = readFileSync(lib.entry, "utf8");
  const originalTree = parseRustSourceTree(originalRootSource);
  const rootItemNames = new Set<string>([...topLevelNames(originalRootSource, originalTree.items)].filter(([, item]) => item.kind !== "use").map(([name]) => name));
  for (const node of originalTree.modules) if (!node.inline) rootItemNames.add(node.name);
  const transform = options.tests ? transformTestClosure(repoRoot, lib.entry, originalRootSource, rootItemNames, options.tests) : null;
  const rootSource = transform?.rootSource ?? originalRootSource;
  const tree = parseRustSourceTree(rootSource);
  const problems: string[] = [];
  const baseOverlay = new Map<string, string>([...(transform?.edits ?? []), ...(transform?.creates ?? [])]);
  const readText = (absolute: string): string | null => {
    if (absolute === lib.entry) return rootSource;
    const virtual = baseOverlay.get(absolute);
    if (virtual !== undefined) return virtual;
    return existsSync(absolute) ? readFileSync(absolute, "utf8") : null;
  };
  const family = convert ? convertMutationLeaves(lib.entry, rootSource, readText) : null;
  for (const [file, text] of family?.edits ?? []) baseOverlay.set(file, text);
  const detaching = new Set<string>((family?.aggregates ?? []).filter((aggregate) => (family?.converted.length ?? 0) > 0).map((aggregate) => aggregate.name));
  const declTree = collectDecls(lib.entry, rootSource, readText);
  const built = buildSlices(declTree, readText, undefined, { detachTestsMentioning: detaching, detachAllTests: options.weave !== false });
  const rootRelatives = new Set(declTree.roots);
  const keptModules = tree.modules.filter((node) => !node.inline).map((node) => node.name);
  const shims = rootAliasModules(rootSource, tree.modules);
  const isShimDecl = (decl: ModuleDecl): boolean => rootRelatives.has(decl.id) && decl.node.inline && shims.has(decl.node.name);
  const carvable = built.slices.filter((slice) => !slice.attach.every((decl) => (rootRelatives.has(decl.id) && !decl.node.inline) || isShimDecl(decl)) && !slice.attach.some((decl) => decl.testOnly && rootRelatives.has(decl.id)));
  const sliceOfDecl = new Map<string, CarveSlice>();
  for (const slice of carvable) for (const decl of slice.decls) sliceOfDecl.set(decl.id, slice);
  const wrapperNodes = new Map<string, RustModuleNode>();
  for (const id of built.namespaces) {
    const decl = declTree.decls.get(id)!;
    if (decl.inlineRoot && decl.node.inline) wrapperNodes.set(decl.eff.join("::"), decl.node);
  }
  const externRows = externCrateRows(rootSource, tree.items);
  const rootNames = topLevelNames(rootSource, tree.items);
  const testGatedNames = new Set(tree.items.filter((item) => item.name && item.attributeTexts.some((attribute) => /cfg\(\s*test\s*\)/u.test(attribute))).map((item) => item.name!));
  const gatedUses = (path: string, names: Iterable<string>): string[] => {
    const sorted = [...names].sort();
    const plain = sorted.filter((name) => !testGatedNames.has(name));
    const gated = sorted.filter((name) => testGatedNames.has(name));
    return [...(plain.length ? [`pub use ${path}::{${plain.join(", ")}};`] : []), ...(gated.length ? [`#[cfg(test)]\npub use ${path}::{${gated.join(", ")}};`] : [])];
  };
  const rootModuleNames = new Set(tree.modules.map((node) => node.name));
  const rootUseTargets = new Map<string, string[][]>();
  for (const item of tree.items) {
    if (item.kind !== "use") continue;
    const written = rootSource.slice(item.declStart, item.end);
    const relative = /^((?:pub(?:\([^)]*\))?\s+)?use\s+)([A-Za-z_][A-Za-z0-9_]*)::/u.exec(written);
    const statement = relative && declTree.roots.some((id) => declTree.decls.get(id)!.node.name === relative[2]) ? written.replace(relative[0], `${relative[1]}crate::${relative[2]}::`) : written;
    if (!/^(?:pub(?:\([^)]*\))?\s+)?use\s+crate::/u.test(statement)) continue;
    for (const leaf of useLeafNames(statement)) rootUseTargets.set(leaf, crateRelativeChains(statement));
  }
  const qualifyRootHeads = (text: string, exclude: ReadonlySet<string> = new Set()): string => qualifyModuleHeads(text, rootModuleNames, exclude);
  const regions: { name: string; start: number; end: number }[] = [];
  for (const match of rootSource.matchAll(/^[ \t]*\/\/#region[^\n]*$/gmu)) {
    const label = match[0].replace(/^[ \t]*\/\/#region\s*/u, "").trim();
    const closing = new RegExp(`^[ \\t]*//#endregion\\s*${label.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}[ \\t]*$`, "mu").exec(rootSource.slice(match.index!));
    if (closing) regions.push({ name: label, start: match.index!, end: match.index! + closing.index + closing[0].length });
  }
  const itemByName = new Map<string, RustSourceItem>();
  for (const [name, item] of rootNames) if (item.kind !== "use") itemByName.set(name, item);
  const regionOf = (item: RustSourceItem) => regions.filter((region) => region.start <= item.start && item.end <= region.end).sort((a, b) => (b.end - b.start) - (a.end - a.start))[0] ?? null;
  const measureRoot = join(dirname(dirname(manifestPath)), "🦀️carve-measure");
  const unitsOf = (members: readonly CarveSlice[]): { owned: MirrorUnit[]; attach: ModuleDecl[] } => {
    const memberDecls = new Set(members.flatMap((slice) => slice.decls.map((decl) => decl.id)));
    const attach = members.flatMap((slice) => slice.attach.filter((decl) => !decl.parentId || !memberDecls.has(decl.parentId) || built.namespaces.has(decl.parentId)));
    const owned = attach.map((decl): MirrorUnit => ({ path: decl.eff, node: decl.node, sourceText: decl.declText, oldScope: decl.scope, component: decl.isComponent, fileDir: dirname(decl.declFile) }));
    return { owned, attach };
  };
  const cutOverlay = (members: readonly CarveSlice[]): Map<string, string> => {
    const memberSlices = new Set(members.map((slice) => slice.id));
    const memberDecls = new Set(members.flatMap((slice) => slice.decls.map((decl) => decl.id)));
    const byFile = new Map<string, { start: number; end: number }[]>();
    for (const slice of members) for (const decl of slice.decls) {
      if (decl.node.inline) continue;
      for (const childId of decl.children) {
        const child = declTree.decls.get(childId)!;
        const childSlice = sliceOfDecl.get(childId);
        if (!childSlice || memberSlices.has(childSlice.id) || memberDecls.has(childId)) continue;
        const text = child.declText;
        byFile.set(child.declFile, [...(byFile.get(child.declFile) ?? []), removeRange(text, child.node.start, child.node.end)]);
      }
    }
    const out = new Map<string, string>();
    for (const [file, ranges] of byFile) {
      let text = readText(file) ?? "";
      for (const range of ranges.sort((a, b) => b.start - a.start)) text = text.slice(0, range.start) + text.slice(range.end);
      out.set(file, demoteDanglingOuterDocs(text));
    }
    return out;
  };
  const effWrappersSpec = (members: readonly CarveSlice[], extra: Partial<MirrorSpec> = {}): { spec: MirrorSpec; attach: ModuleDecl[] } => {
    const { owned, attach } = unitsOf(members);
    return { spec: { doc: "//! 🪞️ planning mirror", newRootDir: measureRoot, innerAttributes: [], externLines: [], movedItemsText: "", itemImports: [], owned, imported: [], wrapperNodes, ...extra }, attach };
  };
  const measureMembers = async (members: readonly CarveSlice[], withTest: boolean): Promise<CrateMeasure> => {
    const { spec } = effWrappersSpec(members);
    const entry = join(measureRoot, "🦀️.rs");
    const overlay = new Map<string, string>([...baseOverlay, ...cutOverlay(members), [entry, renderMirrorRoot(spec)]]);
    return measureRustCrate(repoRoot, entry, withTest, overlay, measureRoot);
  };
  const coherenceType = new Map<string, string>();
  const traitNames = new Set<string>();
  const typeDefs = new Map<string, Set<string>>();
  const infos: SliceInfo[] = [];
  const infoOf = new Map<string, SliceInfo>();
  for (const [index, slice] of carvable.entries()) {
    const info: SliceInfo = { slice, index, effs: slice.attach.map((decl) => decl.eff.join("::")), lib: null, test: null, libEdges: new Set(), testEdges: new Set(), rootRefs: new Set(), testRootRefs: new Set(), keptRefs: new Set(), shimRefs: new Set(), testShimRefs: new Set(), heads: new Set(), coherence: new Set(), why: new Map(), firstFile: slice.attach.map((decl) => decl.file).find((file): file is string => file !== null) ?? "" };
    info.lib = await measureMembers([slice], false);
    info.test = await measureMembers([slice], true);
    infos.push(info);
    infoOf.set(slice.id, info);
  }
  const ownerByEff = new Map<string, SliceInfo>();
  let longestEff = 0;
  for (const info of infos) for (const decl of info.slice.attach) {
    const key = decl.eff.join("\0");
    if (!ownerByEff.has(key)) ownerByEff.set(key, info);
    longestEff = Math.max(longestEff, decl.eff.length);
  }
  const ownerOfChain = (chain: readonly string[]): SliceInfo | null => {
    for (let length = Math.min(chain.length, longestEff); length >= 0; length--) {
      const owner = ownerByEff.get(chain.slice(0, length).join("\0"));
      if (owner) return owner;
    }
    return null;
  };
  const underPath = (eff: readonly string[], prefix: readonly string[]): boolean => eff.length >= prefix.length && prefix.every((segment, index) => segment === eff[index]);
  const memberVisibility = new Map<string, Map<string, string>>();
  const membersOf = (decl: ModuleDecl): Map<string, string> => {
    const known = memberVisibility.get(decl.id);
    if (known) return known;
    const out = new Map<string, string>();
    const text = decl.node.inline ? null : decl.file ? readText(decl.file) : null;
    const parsed = text !== null ? parseRustSourceTree(text) : null;
    for (const item of parsed ? parsed.items : decl.node.items) if (item.name && item.kind !== "impl") out.set(item.name, item.visibility);
    for (const child of parsed ? parsed.modules : decl.node.children) out.set(child.name, child.visibility);
    memberVisibility.set(decl.id, out);
    return out;
  };
  const restrictedTarget = (owner: SliceInfo, chain: readonly string[]): string | null => {
    if (chain.includes("component")) return `${chain.join("::")} (private component module)`;
    const decls = owner.slice.decls.filter((decl) => decl.eff.length <= chain.length && decl.eff.every((segment, index) => segment === chain[index]));
    const deepest = Math.max(0, ...decls.map((decl) => decl.eff.length));
    for (const decl of decls.filter((candidate) => candidate.eff.length === deepest)) {
      if (!decl.isComponent && decl.node.visibility !== "pub" && deepest > 0) return `${chain.join("::")} (module ${decl.node.visibility})`;
      const first = chain[decl.eff.length];
      const visibility = first === undefined ? undefined : membersOf(decl).get(first);
      if (visibility !== undefined && visibility !== "pub") return `${chain.slice(0, decl.eff.length + 1).join("::")} (${visibility})`;
    }
    return null;
  };
  const sourceTexts = new WeakMap<CrateMeasure, Map<string, string>>();
  const sourcesOf = (info: SliceInfo, measure: CrateMeasure): Map<string, string> => {
    const known = sourceTexts.get(measure);
    if (known) return known;
    const entry = join(measureRoot, "🦀️.rs");
    const overlay = new Map<string, string>([...baseOverlay, ...cutOverlay([info.slice]), [entry, renderMirrorRoot(effWrappersSpec([info.slice]).spec)]]);
    const texts = readMeasureSources(repoRoot, measure, overlay);
    sourceTexts.set(measure, texts);
    return texts;
  };
  const noteWhy = (info: SliceInfo, target: SliceInfo, chain: readonly string[], file: string): void => { if (!info.why.has(target.slice.id)) info.why.set(target.slice.id, `${chain.join("::")} @ ${file.split("/").slice(-4).join("/")}`); };
  const viaShim = (info: SliceInfo, chain: readonly string[], edges: Set<string>, shimRefs: Set<string>, file = ""): boolean => {
    const shim = chain[0] ? shims.get(chain[0]) : undefined;
    if (!shim) return false;
    shimRefs.add(chain[0]!);
    for (const target of shim.targets) {
      const owner = ownerOfChain([...target, ...chain.slice(1)]) ?? ownerOfChain(target);
      if (owner && owner !== info) { edges.add(owner.slice.id); noteWhy(info, owner, chain, file); const hidden = restrictedTarget(owner, [...target, ...chain.slice(1)]); if (hidden) { info.coherence.add(owner.slice.id); coherenceType.set(`${info.slice.id}>${owner.slice.id}`, `visibility ${hidden}`); } }
      else if (!owner && target[0] && rootModuleNames.has(target[0]) && !declTree.decls.has(target[0])) info.keptRefs.add(`${chain[0]}->${target.slice(0, 2).join("::")}`);
    }
    return true;
  };
  for (const info of infos) {
    const scan = (measure: CrateMeasure, edges: Set<string>, refs: Set<string>, shimRefs: Set<string>): void => {
      for (const [file, text] of sourcesOf(info, measure)) {
        for (const chain of [...crateRelativeChains(text), ...aliasChains(text)]) {
          if (viaShim(info, chain, edges, shimRefs, file)) continue;
          const owner = ownerOfChain(chain);
          if (owner && owner !== info) { edges.add(owner.slice.id); noteWhy(info, owner, chain, file); const hidden = restrictedTarget(owner, chain); if (hidden) { info.coherence.add(owner.slice.id); coherenceType.set(`${info.slice.id}>${owner.slice.id}`, `visibility ${hidden}`); } }
          if (chain.length > 1) {
            const parentChain = chain.slice(0, -1);
            const parent = ownerOfChain(parentChain);
            if (parent && parent !== info && parent !== owner && !owner?.slice.decls.some((decl) => decl.eff.length === chain.length && decl.eff.every((segment, index) => segment === chain[index])) && parent.slice.decls.some((decl) => decl.eff.length === parentChain.length && decl.eff.every((segment, index) => segment === parentChain[index]) && membersOf(decl).has(chain.at(-1)!))) { edges.add(parent.slice.id); noteWhy(info, parent, chain, file); }
          }
          if (!owner && chain[0] && rootNames.has(chain[0]) && !rootModuleNames.has(chain[0])) {
            refs.add(chain[0]);
            for (const targetChain of rootUseTargets.get(chain[0]) ?? []) {
              if (viaShim(info, targetChain, edges, shimRefs)) continue;
              const via = ownerOfChain(targetChain);
              if (via && via !== info) edges.add(via.slice.id);
            }
          } else if (!owner && chain[0] && rootModuleNames.has(chain[0]) && !declTree.decls.has(chain[0]) ) info.keptRefs.add(chain.slice(0, 2).join("::"));
        }
      }
    };
    scan(info.lib!, info.libEdges, info.rootRefs, info.shimRefs);
    scan(info.test!, info.testEdges, info.testRootRefs, info.testShimRefs);
    for (const decl of info.slice.decls) {
      if (!decl.node.inline || isShimDecl(decl)) continue;
      const nested = decl.node.children.filter((child) => child.inline).map((child): [number, number] => [child.start, child.end]);
      for (const chain of superChains(decl.declText, decl.node.bodyStart, decl.node.bodyEnd, nested, decl.eff)) {
        const owner = ownerOfChain(chain);
        if (owner && owner !== info) (decl.testOnly ? info.testEdges : info.libEdges).add(owner.slice.id);
      }
    }
    for (const measure of [info.lib!, info.test!]) for (const [, text] of sourcesOf(info, measure)) for (const name of pathHeadNames(text)) info.heads.add(name);
    for (const decl of info.slice.decls) for (const name of info.heads) {
      const childEff = [...decl.eff, name];
      const owner = ownerOfChain(childEff);
      if (owner && owner !== info && owner.slice.attach.some((owned) => underPath(owned.eff, childEff))) info.libEdges.add(owner.slice.id);
    }
    for (const [measure, edges] of [[info.lib!, info.libEdges], [info.test!, info.testEdges]] as const) {
      const relative: string[][] = [];
      for (const [, text] of sourcesOf(info, measure)) relative.push(...relativeHeadChains(text));
      if (!relative.length) continue;
      for (const decl of info.slice.decls) {
        const childNames = new Set(decl.children.map((id) => declTree.decls.get(id)?.node.name));
        for (const chain of relative) {
          if (!childNames.has(chain[0])) continue;
          for (let k = 2; k <= chain.length; k += 1) {
            const childEff = [...decl.eff, ...chain.slice(0, k)];
            const owner = ownerOfChain(childEff);
            if (owner && owner !== info && owner.slice.attach.some((owned) => underPath(owned.eff, childEff))) edges.add(owner.slice.id);
          }
        }
      }
    }
    for (const [, text] of sourcesOf(info, info.lib!)) {
      for (const item of allItems(text)) {
        if (item.kind === "trait" && item.name) traitNames.add(item.name);
        if ((item.kind === "struct" || item.kind === "enum" || item.kind === "type") && item.name) typeDefs.set(item.name, new Set([...(typeDefs.get(item.name) ?? []), info.slice.id]));
      }
    }
    if (info.keptRefs.size) problems.push(`slice ${info.effs.join(",")} references modules kept in the artifact root: ${[...info.keptRefs].join(", ")}`);
  }
  const chainOwners = (chain: readonly string[]): SliceInfo[] => {
    const shim = chain[0] ? shims.get(chain[0]) : undefined;
    if (shim) return shim.targets.map((target) => ownerOfChain([...target, ...chain.slice(1)])).filter((owner): owner is SliceInfo => owner !== null);
    const owner = ownerOfChain(chain);
    return owner ? [owner] : [];
  };
  const rootItemClosure = new Map<string, Set<string>>();
  const closureOf = (name: string): Set<string> => {
    const known = rootItemClosure.get(name);
    if (known) return known;
    const out = new Set<string>();
    const visited = new Set<string>();
    const queue = [name];
    while (queue.length) {
      const current = queue.pop()!;
      if (visited.has(current)) continue;
      visited.add(current);
      const item = itemByName.get(current);
      if (!item) continue;
      const region = regionOf(item);
      const members = region ? tree.items.filter((other) => region.start <= other.start && other.end <= region.end) : [item];
      for (const member of members) {
        const text = qualifyRootHeads(rootSource.slice(member.declStart, member.end));
        for (const chain of [...crateRelativeChains(text), ...aliasChains(text)]) for (const owner of chainOwners(chain)) out.add(owner.slice.id);
        for (const identifier of identifierSet(text)) if (itemByName.has(identifier)) queue.push(identifier);
      }
    }
    rootItemClosure.set(name, out);
    return out;
  };
  for (const info of infos) {
    for (const name of info.rootRefs) if (itemByName.has(name)) for (const id of closureOf(name)) if (id !== info.slice.id) info.libEdges.add(id);
    for (const name of info.testRootRefs) if (itemByName.has(name)) for (const id of closureOf(name)) if (id !== info.slice.id) info.testEdges.add(id);
  }
  const commonPrefix = (left: readonly string[], right: readonly string[]): number => { let length = 0; while (length < left.length && length < right.length && left[length] === right[length]) length += 1; return length; };
  const closestOwners = (info: SliceInfo, candidates: ReadonlySet<string>): ReadonlySet<string> => {
    const scored = [...candidates].map((id) => ({ id, score: Math.max(0, ...info.effs.flatMap((left) => infoOf.get(id)!.effs.map((right) => commonPrefix(left.split("::"), right.split("::"))))) })).sort((x, y) => y.score - x.score);
    return scored.length > 1 && scored[0]!.score > scored[1]!.score ? new Set([scored[0]!.id]) : candidates;
  };
  for (const info of infos) {
    for (const [implFile, text] of sourcesOf(info, info.lib!)) {
      for (const item of allItems(text)) {
        if (item.kind !== "impl") continue;
        const header = implHeader(text.slice(item.declStart, item.end));
        if (!header || (header.trait && traitNames.has(header.trait)) || !header.self) continue;
        const declared = typeDefs.get(header.self);
        const owners = declared && declared.size > 1 && !declared.has(info.slice.id) ? closestOwners(info, declared) : declared;
        if (!owners || owners.has(info.slice.id) || owners.size !== 1) { if (owners && owners.size > 1 && !owners.has(info.slice.id)) problems.push(`ambiguous coherence: ${info.effs.join(",")} implements for ${header.self}, defined in ${owners.size} slices`); continue; }
        info.coherence.add([...owners][0]!);
        coherenceType.set(`${info.slice.id}>${[...owners][0]!}`, `${header.self} @ ${implFile.split("/").slice(-4).join("/")}`);
      }
    }
  }
  const woven = new Set<string>();
  const droppedTests = new Set<string>();
  const rootItemPaths: string[][] = tree.items.filter((item) => item.kind !== "use").flatMap((item) => {
    const text = qualifyRootHeads(rootSource.slice(item.declStart, item.end));
    return [...crateRelativeChains(text), ...aliasChains(text)].flatMap((chain) => { const shim = chain[0] ? shims.get(chain[0]) : undefined; return shim ? shim.targets.map((target) => [...target, ...chain.slice(1)]) : [chain]; });
  });
  const namedByRootItems = (child: ModuleDecl): boolean => rootItemPaths.some((path) => underPath(path, child.eff) || underPath(child.eff, path));
  const usedBelow = (parent: ModuleDecl, name: string): boolean => infos.some((other) => other.heads.has(name) && other.slice.attach.some((decl) => decl.eff.length >= parent.eff.length && parent.eff.every((segment, index) => segment === decl.eff[index])));
  const referencedFromOutside = (child: ModuleDecl, childSlice: CarveSlice): boolean => infos.some((other) => other.slice.id !== childSlice.id && !other.slice.attach.some((owned) => underPath(owned.eff, child.eff)) && [...other.libEdges, ...other.testEdges].some((id) => infoOf.get(id)?.slice.attach.some((owned) => underPath(owned.eff, child.eff))));
  for (const info of infos) for (const slice of [info.slice]) for (const decl of slice.decls) for (const childId of decl.children) {
    const childSlice = sliceOfDecl.get(childId);
    if (!childSlice || childSlice.id === slice.id || decl.node.inline) continue;
    const child = declTree.decls.get(childId)!;
    if (child.testOnly && options.weave !== false) droppedTests.add(childId);
    else if (child.isComponent || options.weave === false || usedBelow(decl, child.node.name) || namedByRootItems(child) || referencedFromOutside(child, childSlice)) info.libEdges.add(childSlice.id);
    else woven.add(childId);
  }
  const mergeCauses: { a: string; b: string; kind: "coherence" | "cycle"; type?: string; ring?: string[] }[] = [];
  const edgesOf = (info: SliceInfo): Set<string> => new Set([...info.libEdges, ...info.testEdges]);
  const parentOf = new Map<string, string>();
  const find = (id: string): string => { let current = id; while (parentOf.has(current) && parentOf.get(current) !== current) current = parentOf.get(current)!; return current; };
  const merge = (a: string, b: string): void => { const left = find(a); const right = find(b); if (left !== right) parentOf.set(right, left); };
  for (const info of infos) parentOf.set(info.slice.id, info.slice.id);
  for (const info of infos) for (const other of info.coherence) { merge(info.slice.id, other); mergeCauses.push({ a: info.slice.id, b: other, kind: "coherence", type: coherenceType.get(`${info.slice.id}>${other}`) }); }
  const mergeCycles = (): boolean => {
    const adjacency = new Map<string, Set<string>>();
    for (const info of infos) adjacency.set(find(info.slice.id), new Set());
    for (const info of infos) for (const id of edgesOf(info)) {
      const from = find(info.slice.id);
      const to = find(id);
      if (from !== to) adjacency.get(from)!.add(to);
    }
    const index = new Map<string, number>();
    const low = new Map<string, number>();
    const onStack = new Set<string>();
    const stack: string[] = [];
    let counter = 0;
    let changed = false;
    const visit = (node: string): void => {
      index.set(node, counter);
      low.set(node, counter);
      counter += 1;
      stack.push(node);
      onStack.add(node);
      for (const next of adjacency.get(node) ?? []) {
        if (!index.has(next)) { visit(next); low.set(node, Math.min(low.get(node)!, low.get(next)!)); }
        else if (onStack.has(next)) low.set(node, Math.min(low.get(node)!, index.get(next)!));
      }
      if (low.get(node) !== index.get(node)) return;
      const members: string[] = [];
      for (let member = stack.pop()!; ; member = stack.pop()!) {
        onStack.delete(member);
        members.push(member);
        if (member === node) break;
      }
      for (const member of members.slice(1)) { merge(members[0]!, member); mergeCauses.push({ a: members[0]!, b: member, kind: "cycle", ring: members.map((id) => infoOf.get(id)?.effs[0]?.split("::").slice(-4).join("::") ?? id) }); changed = true; }
    };
    for (const node of adjacency.keys()) if (!index.has(node)) visit(node);
    return changed;
  };
  const mergeFileChains = (): boolean => {
    let changed = false;
    for (const a of infos) for (const outer of a.slice.attach) {
      if (outer.node.inline || outer.isComponent) continue;
      for (const b of infos) {
        if (find(a.slice.id) !== find(b.slice.id)) continue;
        for (const inner of b.slice.attach) {
          if (inner.eff.length <= outer.eff.length || !outer.eff.every((segment, index) => segment === inner.eff[index])) continue;
          for (const c of infos) {
            if (find(c.slice.id) === find(a.slice.id)) continue;
            if (c.slice.decls.some((decl) => decl.eff.length > outer.eff.length && decl.eff.length <= inner.eff.length && decl.eff.every((segment, index) => segment === inner.eff[index]))) { merge(a.slice.id, c.slice.id); mergeCauses.push({ a: a.slice.id, b: c.slice.id, kind: "cycle" }); changed = true; }
          }
        }
      }
    }
    return changed;
  };
  const preGroup = new Map<string, string>(infos.map((info) => [info.slice.id, find(info.slice.id)]));
  for (let changed = true; changed;) changed = mergeCycles() || mergeFileChains();
  const components = new Map<string, SliceInfo[]>();
  for (const info of infos) components.set(find(info.slice.id), [...(components.get(find(info.slice.id)) ?? []), info]);
  const componentList = [...components.values()];
  const blockers: PlanBlocker[] = [];
  for (const members of componentList) {
    if (members.length < 2) continue;
    const files = members.reduce((n, m) => n + (m.lib?.files ?? 0), 0);
    const bytes = members.reduce((n, m) => n + (m.lib?.bytes ?? 0), 0);
    const lines = members.reduce((n, m) => n + (m.lib?.lines ?? 0), 0);
    if (files <= target.files && bytes <= target.bytes && lines <= target.lines) continue;
    const ids = new Set(members.map((m) => m.slice.id));
    const inside = mergeCauses.filter((cause) => ids.has(cause.a) || ids.has(cause.b));
    const effOfSlice = (id: string): string => infoOf.get(id)?.effs[0] ?? id;
    blockers.push({
      slices: members.length, files, bytes, lines,
      coherence: [...new Set(inside.filter((cause) => cause.kind === "coherence").map((cause) => cause.type ?? "?"))].sort(),
      cycles: [...new Set(inside.filter((cause) => cause.kind === "cycle").map((cause) => { const left = effOfSlice(cause.a), right = effOfSlice(cause.b); return left.startsWith(right) || right.startsWith(left) ? "parent-child" : /mutations/u.test(left + right) ? "mutation" : "other"; }))].sort(),
      sample: members.slice(0, 6).map((m) => m.effs[0]!.split("::").slice(-4).join("::")),
      edges: members.flatMap((member) => [...[...member.libEdges].filter((id) => ids.has(id)).map((id) => [id, ""] as const), ...[...member.testEdges].filter((id) => ids.has(id) && !member.libEdges.has(id)).map((id) => [id, " [test]"] as const)].map(([id, mark]) => `${effOfSlice(member.slice.id).split("::").slice(-5).join("::")} -> ${effOfSlice(id).split("::").slice(-5).join("::")}${mark}${member.why.has(id) ? ` (${member.why.get(id)})` : ""}`)),
      rings: [...new Set(inside.filter((cause) => cause.ring).map((cause) => cause.ring!.join(" | ")))],
      groupReport: (() => {
        const groups = new Map<string, SliceInfo[]>();
        for (const member of members) groups.set(preGroup.get(member.slice.id)!, [...(groups.get(preGroup.get(member.slice.id)!) ?? []), member]);
        const short = (info: SliceInfo): string => info.effs[0]!.split("::").slice(-3).join("::");
        const name = (rep: string): string => { const list = groups.get(rep)!; const cap = Number(process.env.F3_GROUP_NAMES ?? 3); return `{${list.slice(0, cap).map(short).join(" + ")}${list.length > cap ? ` +${list.length - cap}` : ""}}`; };
        const reasons = new Map<string, Map<string, string>>();
        const graph = new Map<string, Set<string>>([...groups.keys()].map((rep) => [rep, new Set<string>()]));
        for (const member of members) for (const id of edgesOf(member)) {
          const from = preGroup.get(member.slice.id)!, to = preGroup.get(id);
          if (!to || to === from || !groups.has(to)) continue;
          graph.get(from)!.add(to);
          if (!reasons.has(from)) reasons.set(from, new Map());
          if (!reasons.get(from)!.has(to)) reasons.get(from)!.set(to, `${short(member)} -> ${member.why.get(id) ?? effOfSlice(id).split("::").slice(-3).join("::")}`);
        }
        const lines = [...groups].map(([rep, list]) => `group ${name(rep)} files=${list.reduce((n, m) => n + (m.lib?.files ?? 0), 0)} bytes=${list.reduce((n, m) => n + (m.lib?.bytes ?? 0), 0)} lines=${list.reduce((n, m) => n + (m.lib?.lines ?? 0), 0)}`);
        for (const ring of stronglyConnected([...groups.keys()], graph)) {
          if (ring.length < 2) continue;
          lines.push(`cycle of ${ring.length} groups: ${ring.map(name).join(" , ")}`);
          for (const from of ring) for (const to of graph.get(from)!) if (ring.includes(to)) lines.push(`  ${name(from)} => ${name(to)} via ${reasons.get(from)!.get(to)}`);
        }
        return lines;
      })(),
      causes: inside.map((cause) => `${cause.kind}${cause.type ? "(" + cause.type + ")" : ""}: ${effOfSlice(cause.a).split("::").slice(-5).join("::")} <-> ${effOfSlice(cause.b).split("::").slice(-5).join("::")}`),
    });
  }
  const componentIndex = new Map<string, number>();
  componentList.forEach((members, i) => members.forEach((member) => componentIndex.set(member.slice.id, i)));
  const componentDeps = componentList.map((members, i) => new Set(members.flatMap((member) => [...edgesOf(member)].map((id) => componentIndex.get(id)!)).filter((dep) => dep !== undefined && dep !== i)));
  const layer = componentList.map(() => -1);
  const depth = (i: number, seen = new Set<number>()): number => {
    if (layer[i]! >= 0) return layer[i]!;
    if (seen.has(i)) throw new Error("cyclic component graph");
    seen.add(i);
    layer[i] = 1 + Math.max(-1, ...[...componentDeps[i]!].map((dep) => depth(dep, seen)));
    return layer[i]!;
  };
  componentList.forEach((_, i) => depth(i));
  const order = componentList.map((_, i) => i).sort((a, b) => layer[a]! - layer[b]! || Math.min(...componentList[a]!.map((m) => m.index)) - Math.min(...componentList[b]!.map((m) => m.index)));
  const mirrorEntry = relative(repoRoot, join(measureRoot, "🦀️.rs")).replaceAll("\\", "/");
  const rootOnly = new Set([mirrorEntry]);
  const rootFloor = { files: 1, bytes: 0, lines: 0 };
  type PackedGroup = { components: number[]; infos: SliceInfo[]; lib: MeasureUnion; test: MeasureUnion };
  const startGroup = (index: number, members: readonly SliceInfo[]): PackedGroup => {
    const group: PackedGroup = { components: [index], infos: [...members], lib: new MeasureUnion(rootOnly, rootFloor), test: new MeasureUnion(rootOnly, rootFloor) };
    group.lib.add(members.map((member) => member.lib!.entries));
    group.test.add(members.map((member) => member.test!.entries));
    return group;
  };
  const groups: PackedGroup[] = [];
  const packing: PackingStats = { pruned: 0, measured: 0, merged: 0 };
  const incremental = options.incrementalMeasure !== false;
  const groupOfSlice = new Map<string, number>();
  const acyclic = (grouping: { components: number[] }[]): boolean => {
    const owner = new Map<number, number>();
    grouping.forEach((group, i) => group.components.forEach((c) => owner.set(c, i)));
    const edges = grouping.map((group) => new Set(group.components.flatMap((c) => [...componentDeps[c]!].map((dep) => owner.get(dep)!)).filter((dep) => dep !== undefined)));
    const state = grouping.map(() => 0);
    const visit = (i: number): boolean => { if (state[i] === 1) return false; if (state[i] === 2) return true; state[i] = 1; for (const next of edges[i]!) if (next !== i && !visit(next)) return false; state[i] = 2; return true; };
    return grouping.every((_, i) => visit(i));
  };
  const mirrorValid = (members: readonly CarveSlice[]): boolean => {
    const attached = members.flatMap((slice) => slice.attach);
    return !attached.some((outer) => !outer.node.inline && !outer.isComponent && attached.some((inner) => inner !== outer && inner.eff.length > outer.eff.length && outer.eff.every((segment, index) => segment === inner.eff[index])));
  };
  for (const index of order) {
    const members = componentList[index]!;
    let placed = false;
    const parentSliceIds = new Set(members.flatMap((member) => member.slice.attach.map((decl) => (decl.parentId ? sliceOfDecl.get(decl.parentId)?.id : undefined)).filter((id): id is string => id !== undefined)));
    const parentGroups = new Set<number>();
    const sharedGroups = new Set<number>();
    for (const id of parentSliceIds) { const at = groupOfSlice.get(id); if (at !== undefined) parentGroups.add(at); }
    for (const member of members) for (const id of edgesOf(member)) { const at = groupOfSlice.get(id); if (at !== undefined) sharedGroups.add(at); }
    const affinity = (i: number): number => (parentGroups.has(i) ? 2 : 0) + (sharedGroups.has(i) ? 1 : 0);
    const candidates = groups.map((group, i) => i).sort((a, b) => affinity(b) - affinity(a) || b - a);
    for (const candidate of candidates) {
      const group = groups[candidate]!;
      if (incremental && (!totalsFit(group.lib.peek(members.map((member) => member.lib!.entries)), target) || !totalsFit(group.test.peek(members.map((member) => member.test!.entries)), target))) { packing.pruned += 1; continue; }
      const trial = groups.map((entry, i) => (i === candidate ? { components: [...entry.components, index] } : { components: entry.components }));
      if (!acyclic(trial)) continue;
      const merged = [...group.infos, ...members].map((info) => info.slice);
      if (!mirrorValid(merged)) continue;
      packing.measured += 1;
      if (!measureFits(await measureMembers(merged, false), target) || !measureFits(await measureMembers(merged, true), target)) continue;
      packing.merged += 1;
      group.components.push(index);
      group.infos.push(...members);
      group.lib.add(members.map((member) => member.lib!.entries));
      group.test.add(members.map((member) => member.test!.entries));
      for (const member of members) groupOfSlice.set(member.slice.id, candidate);
      placed = true;
      break;
    }
    if (!placed) {
      for (const member of members) groupOfSlice.set(member.slice.id, groups.length);
      groups.push(startGroup(index, members));
    }
  }
  const crateOfSlice = new Map<string, number>();
  groups.forEach((group, i) => group.infos.forEach((info) => crateOfSlice.set(info.slice.id, i)));
  const crateDeps = groups.map(() => new Set<number>());
  groups.forEach((group, i) => group.infos.forEach((info) => edgesOf(info).forEach((id) => { const to = crateOfSlice.get(id); if (to !== undefined && to !== i) crateDeps[i]!.add(to); })));
  const referencing = groups.map((group) => new Set(group.infos.flatMap((info) => [...info.rootRefs, ...[...info.testRootRefs].filter((name) => rootNames.get(name)?.kind !== "use")])));
  const referenced = new Set(referencing.flatMap((set) => [...set]));
  const movedNames = new Set<string>();
  for (const name of referenced) if (rootNames.has(name) && rootNames.get(name)!.kind !== "use") movedNames.add(name);
  if (!tree.items.some((item) => item.kind === "macro")) for (const [name, item] of itemByName) if (["fn", "const", "static", "struct", "enum", "type", "trait"].includes(item.kind)) movedNames.add(name);
  const movedItems = new Set<RustSourceItem>();
  for (let changed = true; changed;) {
    changed = false;
    for (const name of [...movedNames]) {
      const item = itemByName.get(name);
      if (!item || movedItems.has(item)) continue;
      const region = regionOf(item);
      const members = region ? tree.items.filter((other) => region.start <= other.start && other.end <= region.end) : [item];
      for (const member of members) {
        if (movedItems.has(member)) continue;
        movedItems.add(member);
        if (member.name) movedNames.add(member.name);
        for (const identifier of identifierSet(rootSource.slice(member.declStart, member.end))) {
          const other = itemByName.get(identifier);
          if (other && !movedItems.has(other) && !movedNames.has(identifier) && identifier !== member.name) { movedNames.add(identifier); changed = true; }
        }
        changed = true;
      }
    }
  }
  const pieceOf = new Map<RustSourceItem, string>();
  const pieceItems = new Map<string, RustSourceItem[]>();
  for (const item of [...movedItems].sort((a, b) => a.start - b.start)) {
    const region = regionOf(item);
    const key = region ? `r${region.start}` : `i${item.start}`;
    pieceOf.set(item, key);
    pieceItems.set(key, [...(pieceItems.get(key) ?? []), item]);
  }
  const pieceOfName = new Map<string, string>();
  for (const [item, key] of pieceOf) if (item.name && item.kind !== "impl" && item.kind !== "macro") pieceOfName.set(item.name, key);
  for (const [item, key] of pieceOf) if (item.kind === "use" && item.visibility.startsWith("pub")) for (const leaf of useLeafNames(rootSource.slice(item.declStart, item.end))) pieceOfName.set(leaf, key);
  const pieceDeps = new Map<string, Set<string>>();
  for (const [key, items] of pieceItems) {
    const deps = new Set<string>();
    for (const item of items) for (const identifier of identifierSet(rootSource.slice(item.declStart, item.end))) {
      const other = pieceOfName.get(identifier);
      if (other && other !== key) deps.add(other);
    }
    pieceDeps.set(key, deps);
  }
  const unitPieces = stronglyConnected([...pieceItems.keys()], pieceDeps);
  const unitOfPiece = new Map<string, number>();
  unitPieces.forEach((keys, index) => keys.forEach((key) => unitOfPiece.set(key, index)));
  type MovedUnit = { index: number; names: Set<string>; exports: Set<string>; blocks: { start: number; end: number }[]; text: string; module: string; edgeSlices: Set<string>; shimRefs: Set<string>; dependsOn: Set<number>; crossNames: Map<number, string[]>; host: number };
  const movedUnits: MovedUnit[] = unitPieces.map((keys, index) => {
    const items = keys.flatMap((key) => pieceItems.get(key)!).sort((a, b) => a.start - b.start);
    const blocks: { start: number; end: number }[] = [];
    const widen: { at: number; remove: number; insert: string }[] = [];
    for (const item of items) {
      const region = regionOf(item);
      const block = region ? { start: region.start, end: region.end } : { start: rustItemStartWithComments(rootSource, item.start), end: item.end };
      if (!blocks.some((existing) => existing.start === block.start && existing.end === block.end)) blocks.push(block);
      const visibilityText = /^pub(?:\([^)]*\))?\s*/u.exec(rootSource.slice(item.declStart, item.end));
      if (["fn", "const", "static", "struct", "enum", "type", "trait"].includes(item.kind)) widen.push({ at: item.declStart, remove: visibilityText ? visibilityText[0].length : 0, insert: "pub " });
    }
    const merged: { start: number; end: number }[] = [];
    for (const block of blocks.sort((left, right) => left.start - right.start || right.end - left.end)) {
      const last = merged.at(-1);
      if (last && block.start < last.end) last.end = Math.max(last.end, block.end);
      else merged.push({ ...block });
    }
    const body = merged.map((block) => {
      let text = rootSource.slice(block.start, block.end);
      for (const edit of widen.filter((entry) => entry.at >= block.start && entry.at < block.end).sort((a, b) => b.at - a.at)) text = text.slice(0, edit.at - block.start) + edit.insert + text.slice(edit.at - block.start + edit.remove);
      return text;
    }).join("\n\n");
    const identifiers = identifierSet(body);
    const useLines: string[] = [];
    const providedByOtherUnit = (leaf: string): boolean => { const piece = pieceOfName.get(leaf); return piece !== undefined && unitOfPiece.get(piece) !== index; };
    for (const item of tree.items) {
      if (item.kind !== "use" || items.includes(item)) continue;
      const statement = rootSource.slice(item.declStart, item.end);
      const leaves = useLeafNames(statement).filter((leaf) => identifiers.has(leaf) && !providedByOtherUnit(leaf));
      if (leaves.length) useLines.push(prunedUse(statement, leaves));
    }
    const names = new Set(items.flatMap((item) => (item.name && item.kind !== "impl" ? [item.name] : [])));
    const exports = new Set([...names, ...items.filter((item) => item.kind === "use" && item.visibility.startsWith("pub")).flatMap((item) => useLeafNames(rootSource.slice(item.declStart, item.end)))]);
    const dependsOn = new Set<number>();
    for (const key of keys) for (const other of pieceDeps.get(key) ?? []) { const unit = unitOfPiece.get(other)!; if (unit !== index) dependsOn.add(unit); }
    return { index, names, exports, blocks: merged, text: [...useLines, "", body].join("\n"), module: index === 0 ? "artifact_items" : `artifact_items_${index + 1}`, edgeSlices: new Set<string>(), shimRefs: new Set<string>(), dependsOn, crossNames: new Map<number, string[]>(), host: -1 };
  });
  for (const unit of movedUnits) {
    const identifiers = identifierSet(unit.text);
    const cross = [...unit.dependsOn].map((dep) => { const used = [...movedUnits[dep]!.exports].filter((name) => identifiers.has(name)).sort(); unit.crossNames.set(dep, used); return used; }).filter((used) => used.length);
    const lines = cross.map((used) => `use crate::{${used.join(", ")}};`);
    const label = unit.index === 0 ? "🧩️ArtifactItems" : `🧩️ArtifactItems${unit.index + 1}`;
    unit.text = `\n//#region ${label}\npub mod ${unit.module} {\n${[...lines, qualifyRootHeads(unit.text, movedNames)].join("\n").trim()}\n}\n//#endregion ${label}\n`;
    for (const chain of [...crateRelativeChains(unit.text), ...aliasChains(unit.text)]) {
      const shim = chain[0] ? shims.get(chain[0]) : undefined;
      if (shim) {
        unit.shimRefs.add(chain[0]!);
        for (const target of shim.targets) { const owner = ownerOfChain([...target, ...chain.slice(1)]); if (owner) unit.edgeSlices.add(owner.slice.id); }
      } else { const owner = ownerOfChain(chain); if (owner) unit.edgeSlices.add(owner.slice.id); }
    }
  }
  const reachesCrate = (from: number, to: number, seen = new Set<number>()): boolean => {
    if (from === to) return true;
    if (seen.has(from)) return false;
    seen.add(from);
    return [...crateDeps[from]!].some((next) => reachesCrate(next, to, seen));
  };
  const innerAttributes = tree.innerAttributes.map((attribute) => attribute.text);
  const hostable = groups.map((_, i) => i).filter((i) => groups[i]!.infos.some((info) => info.slice.attach.some((decl) => decl.file !== null && !decl.testOnly)));
  const required = new Set<number>(movedUnits.filter((unit) => [...unit.names].some((name) => referenced.has(name))).map((unit) => unit.index));
  for (let changed = true; changed;) { changed = false; for (const unit of movedUnits) if (required.has(unit.index)) for (const dep of unit.dependsOn) if (!required.has(dep)) { required.add(dep); changed = true; } }
  const assign = (unit: MovedUnit, refs: Set<number>, footprint: Set<number>): boolean => {
    const valid = (host: number): boolean => [...footprint].every((dep) => dep === host || !reachesCrate(dep, host)) && [...refs].every((ref) => ref === host || (!reachesCrate(host, ref) && ![...footprint].some((dep) => dep !== host && (dep === ref || reachesCrate(dep, ref)))));
    const cost = (host: number): number => [...refs].filter((ref) => ref !== host && !reachesCrate(ref, host)).length + [...footprint].filter((dep) => dep !== host && !reachesCrate(host, dep)).length;
    const candidates = hostable.filter(valid).sort((a, b) => cost(a) - cost(b) || Number(refs.has(b)) - Number(refs.has(a)) || a - b);
    const host = candidates[0];
    if (host === undefined) return false;
    unit.host = host;
    for (const ref of refs) if (ref !== host) crateDeps[ref]!.add(host);
    for (const dep of footprint) if (dep !== host) crateDeps[host]!.add(dep);
    return true;
  };
  const footprintOf = (unit: MovedUnit): Set<number> => new Set<number>([...unit.edgeSlices].map((id) => crateOfSlice.get(id)).filter((index): index is number => index !== undefined));
  for (let at = movedUnits.length - 1; at >= 0; at -= 1) {
    const unit = movedUnits[at]!;
    if (!required.has(unit.index)) continue;
    const refs = new Set<number>(groups.map((_, i) => i).filter((i) => [...referencing[i]!].some((name) => unit.names.has(name))));
    for (const other of movedUnits) if (other.dependsOn.has(unit.index) && other.host >= 0) refs.add(other.host);
    if (!assign(unit, refs, footprintOf(unit))) {
      const fallback = [...refs].find((ref) => hostable.includes(ref)) ?? hostable[0] ?? 0;
      unit.host = fallback;
      problems.push(`moved root items ${[...unit.names].join(", ")} cannot be hosted without a crate cycle (candidate ${groups[fallback]!.infos[0]!.effs[0]})`);
    }
  }
  for (const unit of movedUnits) {
    if (required.has(unit.index)) continue;
    const footprint = footprintOf(unit);
    let missing = false;
    for (const dep of unit.dependsOn) { const host = movedUnits[dep]!.host; if (host < 0) missing = true; else footprint.add(host); }
    if (missing || !assign(unit, new Set<number>(), footprint)) unit.host = -1;
  }
  const movedPlaced = movedUnits.filter((unit) => unit.host >= 0);
  const hostedAt = (i: number): MovedUnit[] => movedUnits.filter((unit) => unit.host === i);
  const draft = groups.map((group, i) => {
    const slices = group.infos.map((info) => info.slice).sort((a, b) => carvable.indexOf(a) - carvable.indexOf(b));
    const { attach } = unitsOf(slices);
    const labelOf = (decl: ModuleDecl): string => {
      const eff = decl.eff;
      const tail = eff[0] === "standards" ? eff.slice(4) : eff;
      const segment = (tail.length > 1 && eff[0] === "standards" ? tail[tail.length - 1] : tail[0]) ?? eff.at(-1) ?? "x";
      return (facetSlug(segment.replaceAll("_", "-")) || "x");
    };
    const isSchema = (slice: CarveSlice): boolean => slice.attach.some((decl) => decl.eff[0] === "standards" && decl.eff[4] === "schema");
    const depthOf = (slice: CarveSlice): number => Math.min(...slice.attach.map((decl) => decl.eff.length));
    const biggest = [...slices].sort((a, b) => Number(isSchema(b)) - Number(isSchema(a)) || depthOf(a) - depthOf(b) || b.decls.length - a.decls.length)[0]!;
    const familyOf = (slice: CarveSlice): string | null => {
      const eff = slice.attach[0]?.eff ?? [];
      const at = eff.indexOf("mutations");
      const subset = eff.indexOf("subsets");
      return at >= 0 && eff.length > at + 1 ? (eff[subset >= 0 ? subset + 1 : at + 1] ?? "") : null;
    };
    const families = slices.map(familyOf);
    const dominant = families.every((family) => family !== null) ? `mut-${facetSlug([...new Set(families)].length === 1 ? families[0]!.replaceAll("_", "-") : "family")}`.replace(/-$/u, "") : labelOf(biggest.attach[0]!);
    const firstFile = (members: readonly ModuleDecl[]): string | null => [...members].filter((decl) => decl.file !== null).sort((a, b) => a.raw.length - b.raw.length)[0]?.file ?? null;
    const anchorFile = firstFile(biggest.decls) ?? firstFile(attach) ?? null;
    const anchor = attach[0]!;
    const dirOf = anchorFile ? dirname(anchorFile) : lcaDirectory(slices.flatMap((slice) => slice.decls.map((decl) => decl.file ?? "")).filter(Boolean));
    return { i, slices, attach, label: dominant, dir: dirOf, anchor };
  });
  const usedNames = new Set<string>();
  const usedDirs = new Set<string>();
  const crates: PlannedCrate[] = [];
  for (const entry of draft) {
    let name = `${lib.name}-${entry.label}`;
    for (let n = 2; usedNames.has(name); n += 1) name = `${lib.name}-${entry.label}-${n}`;
    usedNames.add(name);
    let dir = entry.dir;
    for (const decl of entry.attach) { if (!usedDirs.has(dir)) break; if (decl.file && !usedDirs.has(dirname(decl.file))) dir = dirname(decl.file); }
    void entry.anchor;
    usedDirs.add(dir);
    const imports: MirrorImport[] = [];
    const deps = new Set(crateDeps[entry.i]);
    const attachPaths = [...entry.attach.filter((decl) => !decl.isComponent), ...entry.slices.flatMap((slice) => slice.decls.filter((decl) => !decl.node.inline && !decl.isComponent))].map((decl) => decl.eff.join("::"));
    for (const id of [...groups[entry.i]!.infos.flatMap((info) => [...edgesOf(info)]), ...hostedAt(entry.i).flatMap((unit) => [...unit.edgeSlices])]) {
      const to = crateOfSlice.get(id);
      if (to === undefined || to === entry.i) continue;
      for (const decl of infoOf.get(id)!.slice.attach) {
        if (decl.testOnly) continue;
        const key = decl.eff.join("::");
        if (attachPaths.some((own) => key === own || key.startsWith(own + "::"))) continue;
        if (imports.some((existing) => existing.glob === decl.isComponent && existing.path.join("::") === key)) continue;
        imports.push({ path: decl.eff, crateIdent: String(to), visibility: decl.node.visibility === "private" ? "pub" : decl.node.visibility, order: decl.node.start, glob: decl.isComponent });
      }
    }
    const pruned = imports.filter((entryImport) => !imports.some((other) => other !== entryImport && !other.glob && other.crateIdent === entryImport.crateIdent && other.path.length < entryImport.path.length && other.path.every((segment, k) => segment === entryImport.path[k])));
    const nestedGlobs: MirrorImport[] = [];
    for (const globImport of [...pruned]) {
      const depPaths = groups[Number(globImport.crateIdent)]!.infos.flatMap((info) => info.slice.decls.filter((decl) => !decl.testOnly).map((decl) => decl.eff));
      for (const owner of entry.attach) {
        if (owner.eff.length <= globImport.path.length + 1 || !globImport.path.every((segment, k) => segment === owner.eff[k])) continue;
        for (let k = globImport.path.length + 1; k < owner.eff.length; k += 1) {
          const through = owner.eff.slice(0, k);
          const same = (other: MirrorImport): boolean => other.crateIdent === globImport.crateIdent && other.path.length === k && other.path.every((segment, j) => segment === through[j]);
          if (depPaths.some((path) => path.length >= k && through.every((segment, j) => segment === path[j])) && ![...pruned, ...nestedGlobs].some(same)) nestedGlobs.push({ ...globImport, path: through, glob: true });
        }
      }
    }
    pruned.push(...nestedGlobs);
    const copiedNames = new Set([...referencing[entry.i]!].filter((n) => rootNames.get(n)?.kind === "use"));
    for (const name of copiedNames) {
      const item = rootNames.get(name)!;
      const statement = rootSource.slice(item.declStart, item.end);
      for (const head of [...crateRelativeChains(statement).map((chain) => chain[0]!), ...pathHeadNames(statement)]) if (rootNames.get(head)?.kind === "use" && head !== name && !copiedNames.has(head)) copiedNames.add(head);
    }
    const copiedUses = [...new Set([...copiedNames].map((n) => { const item = rootNames.get(n)!; return prunedUse(rootSource.slice(item.declStart, item.end), [n]); }).filter(Boolean))];
    const hosted = hostedAt(entry.i);
    const importedFrom = new Map<number, Set<string>>();
    for (const unit of movedPlaced) {
      if (unit.host === entry.i) continue;
      const needed = new Set<string>([...referencing[entry.i]!].filter((name) => unit.names.has(name)));
      for (const own of hosted) for (const name of own.crossNames.get(unit.index) ?? []) needed.add(name);
      for (const name of [...needed]) if (copiedNames.has(name)) needed.delete(name);
      if (needed.size) importedFrom.set(unit.host, new Set([...(importedFrom.get(unit.host) ?? []), ...needed]));
    }
    const unitImports = [...importedFrom].sort((a, b) => a[0] - b[0]).flatMap(([host, names]) => gatedUses(`${crateIdentPlaceholder(host)}`, names));
    const libShims = new Set([...entry.slices.flatMap((slice) => [...infoOf.get(slice.id)!.shimRefs]), ...hosted.flatMap((unit) => [...unit.shimRefs])]);
    const testShims = new Set(entry.slices.flatMap((slice) => [...infoOf.get(slice.id)!.testShimRefs]).filter((name) => !libShims.has(name)));
    const shimCopies = [...[...libShims].sort().map((name) => shims.get(name)!.text), ...[...testShims].sort().map((name) => `#[cfg(test)]\n${shims.get(name)!.text}`)];
    const takenNames = new Set<string>([...importedFrom.values()].flatMap((names) => [...names]).concat(hosted.flatMap((unit) => [...unit.exports])));
    const testOnlyUses = [...new Set(entry.slices.flatMap((slice) => [...infoOf.get(slice.id)!.testRootRefs]).filter((n) => !copiedNames.has(n) && !takenNames.has(n) && rootNames.get(n)?.kind === "use").map((n) => { const item = rootNames.get(n)!; return `#[cfg(test)]\n${prunedUse(rootSource.slice(item.declStart, item.end), [n])}`; }))];
    crates.push({ id: String(entry.i), name, packageDir: relativePosix(repoRoot, join(dir, "📦️packages/🦀️rust")), description: `Component crate of ${lib.name}: ${entry.label}`, doc: "", slices: entry.slices, attach: entry.attach, dependsOn: [...deps].sort((a, b) => a - b).map(String), imports: pruned, itemImportLines: [...copiedUses, ...testOnlyUses, ...shimCopies, ...unitImports], externLines: [], innerAttributes, movedItemsText: "", lib: null, test: null });
  }
  const identOf = new Map(crates.map((crate) => [crate.id, crateIdent(crate.name)]));
  const resolved: PlannedCrate[] = [];
  const movedAppends: { file: string; text: string; anchor: string }[] = [];
  for (const crate of crates) {
    const imports = crate.imports.map((entry) => ({ ...entry, crateIdent: identOf.get(entry.crateIdent)! }));
    const itemImportLines = crate.itemImportLines.map((line) => line.replace(/__CRATE_(\d+)__/gu, (_, index: string) => identOf.get(index) ?? ""));
    const packageDirAbs = resolve(repoRoot, crate.packageDir);
    const entry = join(packageDirAbs, "🦀️.rs");
    const { owned } = unitsOf(crate.slices);
    const hostedHere = hostedAt(Number(crate.id));
    const anchorDecl = hostedHere.length ? [...crate.attach].filter((decl) => decl.file !== null && !decl.testOnly).sort((a, b) => Number(b.isComponent) - Number(a.isComponent) || a.eff.length - b.eff.length)[0] ?? null : null;
    if (hostedHere.length && !anchorDecl) problems.push(`crate ${crate.name} owns the moved root items but mounts no file to host them`);
    const destinationImport = anchorDecl ? hostedHere.flatMap((unit) => gatedUses(`crate::${anchorDecl.eff.join("::")}::${unit.module}`, [...unit.exports].filter((name) => unit.names.has(name) || !crate.itemImportLines.some((line) => new RegExp(`\\b${name}\\b`, "u").test(line))))) : [];
    const importLines = [...itemImportLines, ...destinationImport];
    const specBase: MirrorSpec = { doc: "//! 🪞️ planning mirror", newRootDir: packageDirAbs, innerAttributes, externLines: [], movedItemsText: "", itemImports: importLines, owned, imported: imports, wrapperNodes };
    const cuts = cutOverlay(crate.slices);
    for (const slice of crate.slices) for (const decl of slice.decls) {
      if (decl.file === null) continue;
      const text = cuts.get(decl.file) ?? readText(decl.file) ?? "";
      const rebased = rebaseManifestDirPaths(text, dirname(manifestPath), packageDirAbs);
      if (rebased !== text) cuts.set(decl.file, rebased);
    }
    const movedText = hostedHere.map((unit) => unit.text).join("");
    const movedHere = anchorDecl?.file ? rebaseRustIncludes(movedText, dirname(lib.entry), dirname(anchorDecl.file)) : movedText;
    if (anchorDecl?.file && movedText) cuts.set(anchorDecl.file, (cuts.get(anchorDecl.file) ?? readText(anchorDecl.file) ?? "") + movedHere);
    const used = new Set<string>();
    const draftText = renderMirrorRoot(specBase);
    for (const withTest of [false, true]) {
      const overlay = new Map<string, string>([...baseOverlay, ...cuts, [entry, draftText]]);
      const measured = await measureRustCrate(repoRoot, entry, withTest, overlay, packageDirAbs);
      for (const [, text] of readMeasureSources(repoRoot, measured, overlay)) for (const identifier of identifierSet(text)) used.add(identifier);
    }
    for (const identifier of identifierSet(movedText)) used.add(identifier);
    const externLines = externRows.filter((row) => used.has(row.alias)).map((row) => rootSource.slice(row.start, row.end).trim());
    const slugLabel = crate.name.slice(lib.name.length + 1);
    const doc = `//! ${facetEmoji(crate.packageDir.split("/").slice(-3)[0] ?? "")} ${slugLabel} of \`${lib.name}\` — a path-preserving carve of the artifact root mounts.`;
    const lineBudget = loadCatalogTaxonomy().libWiringLineBudget;
    const grammar = loadCatalogTaxonomy().packageGlueGrammar.rust!;
    let finalFiles = renderMirrorFiles({ ...specBase, externLines, doc });
    const hoisted = finalFiles.root.split("\n").length > lineBudget || classifyPackageSource(finalFiles.root, grammar).role !== "declaration";
    if (hoisted) finalFiles = renderMirrorFiles({ ...specBase, externLines, doc, hoist: true });
    const finalText = finalFiles.root;
    const finalOverlay = new Map<string, string>([...baseOverlay, ...cuts, [entry, finalText], ...(finalFiles.body !== null ? [[join(packageDirAbs, "🪞️mirror/🦀️.rs"), finalFiles.body] as [string, string]] : [])]);
    const glue = classifyPackageSource(finalText, grammar);
    if (glue.role !== "declaration") problems.push(`mirror root of ${crate.name} is not pure package glue (${glue.role}: ${glue.evidence})`);
    if (finalText.split("\n").length > lineBudget) problems.push(`mirror root of ${crate.name} has ${finalText.split("\n").length} lines, over the ${lineBudget}-line wiring budget; hoist inline namespaces into component files`);
    for (const outer of crate.attach) {
      if (outer.node.inline || outer.isComponent) continue;
      const inner = crate.attach.find((other) => other !== outer && other.eff.length > outer.eff.length && outer.eff.every((segment, index) => segment === other.eff[index]));
      if (inner) { problems.push(`crate ${crate.name} cannot be mirrored: file module ${outer.eff.join("::")} also owns the deeper unit ${inner.eff.join("::")} whose parent chain is cut elsewhere`); break; }
    }
    const lib0 = await measureRustCrate(repoRoot, entry, false, finalOverlay, packageDirAbs);
    const test0 = await measureRustCrate(repoRoot, entry, true, finalOverlay, packageDirAbs);
    if (!measureFits(lib0, hard) || !measureFits(test0, hard)) problems.push(`crate ${crate.name} exceeds the hard budget: lib ${lib0.files}/${lib0.bytes}/${lib0.lines}, test ${test0.files}/${test0.bytes}/${test0.lines}`);
    resolved.push({ ...crate, imports, itemImportLines: importLines, externLines, doc, hoisted, lib: lib0, test: test0 });
    if (anchorDecl?.file && movedText) movedAppends.push({ file: anchorDecl.file, text: movedHere, anchor: anchorDecl.eff.join("::") });
  }
  const crateIdentOfSlice = (sliceId: string): string => identOf.get(String(crateOfSlice.get(sliceId)))!;
  const rootEdits: RootEdit[] = [];
  const fileCuts: DeclCut[] = [];
  const weave: WovenExport[] = [];
  const placement = new Map<string, string>();
  for (const slice of carvable) for (const decl of slice.decls) placement.set(decl.eff.join("::"), crateIdentOfSlice(slice.id));
  for (const info of infos) {
    const crateIdentName = crateIdentOfSlice(info.slice.id);
    const crateIndex = crateOfSlice.get(info.slice.id)!;
    const crateAttach = new Set(resolved[crateIndex]!.attach.map((decl) => decl.id));
    for (const decl of info.slice.attach) {
      const parentCrate = decl.parentId ? crateOfSlice.get(sliceOfDecl.get(decl.parentId)?.id ?? "") : undefined;
      const parentIsNamespace = decl.parentId === null || built.namespaces.has(decl.parentId);
      if (!parentIsNamespace && parentCrate === crateIndex) continue;
      if (!crateAttach.has(decl.id) && !parentIsNamespace) continue;
      const visibility = decl.node.visibility === "private" ? "pub" : decl.node.visibility;
      if (decl.testOnly && !decl.isComponent) {
        if (decl.declFile === lib.entry) rootEdits.push({ start: decl.node.start, end: decl.node.end, text: "" });
        else fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text: "" });
        continue;
      }
      if (decl.isComponent) {
        const wrapperPath = decl.eff.join("::");
        const replacement = `pub use ${crateIdentName}::${wrapperPath}::*;`;
        if (decl.declFile === lib.entry) {
          rootEdits.push({ start: decl.node.start, end: decl.node.end, text: replacement, anchor: { eff: decl.eff, crate: crateIdentName, visibility: "pub", component: true } });
          if (decl.globItem) rootEdits.push({ start: decl.globItem.start, end: decl.globItem.end, text: "" });
        }
        continue;
      }
      const text = `${visibility} use ${crateIdentName}::${decl.eff.join("::")};`;
      if (decl.declFile === lib.entry) rootEdits.push({ start: decl.node.start, end: decl.node.end, text, anchor: { eff: decl.eff, crate: crateIdentName, visibility: visibility === "private" ? "pub" : visibility.startsWith("pub(") ? "pub" : visibility, component: false } });
      else if (droppedTests.has(decl.id) && !parentIsNamespace) fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text: "" });
      else if (woven.has(decl.id) && !parentIsNamespace) {
        fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text: "" });
        weave.push({ eff: decl.eff, crate: crateIdentName });
      } else fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text, anchor: { eff: decl.eff, crate: crateIdentName, visibility: visibility === "private" || visibility.startsWith("pub(") ? "pub" : visibility, component: false } });
    }
  }
  for (const edit of rootEdits) {
    if (edit.end === edit.start) continue;
    const hit = movedPlaced.flatMap((unit) => unit.blocks).find((block) => edit.start < block.end && block.start < edit.end);
    if (hit) problems.push(`the carved root module ${edit.anchor?.eff.join("::") ?? `at offset ${edit.start}`} overlaps a root region that moves into artifact_items; split the region first`);
  }
  if (weave.length) problems.push(...weaveRootEdits(rootEdits, wrapperNodes, weave, placement, fileCuts).problems);
  const plan: ArtifactCratePlan = { repoRoot, artifactDir, artifactCrate: lib.name, packageDir: packageDirRel, rootFile: relativePosix(repoRoot, lib.entry), tree, crates: resolved, wrapperNodes, rootEdits, weave, placement, fileCuts, movedUnits: movedPlaced.map((unit) => ({ names: [...unit.exports].sort(), blocks: unit.blocks, to: crates[unit.host]!.id })), movedAppends, keptModules, overlay: { edits: new Map([...(transform?.edits ?? []), ...(family?.edits ?? [])]), creates: transform?.creates ?? new Map(), deletes: transform?.deletes ?? new Set(), notes: [...(transform?.notes ?? []), ...(family?.notes ?? [])] }, rootMeasure: null, problems, blockers, packing };
  return plan;
}

function crateIdentPlaceholder(index: number): string {
  return `__CRATE_${index}__`;
}

/** 🔍️ Trait and self type names of one `impl` item. */
type WeaveNode = { crate: string | null; children: Map<string, WeaveNode> };

/** 🧵️ Rewrites the artifact root so every module whose mount was cut out of its parent is exported again at its original path: module-level `pub use` and, where a module has woven children, a local `pub mod` that glob-re-exports the module of the crate that owns it (a local module shadows the glob, so no path changes). */
export function weaveRootEdits(rootEdits: readonly RootEdit[], wrapperNodes: ReadonlyMap<string, RustModuleNode>, weave: readonly WovenExport[], placement: ReadonlyMap<string, string>, fileCuts: readonly DeclCut[] = []): { edits: RootEdit[]; cuts: DeclCut[]; problems: string[] } {
  const edits: RootEdit[] = [...rootEdits.map((edit) => ({ ...edit })), ...fileCuts.map((cut) => ({ start: cut.start, end: cut.end, text: cut.text, anchor: cut.anchor }))];
  const cutCount = fileCuts.length;
  const problems: string[] = [];
  const prefixOf = (eff: readonly string[], candidate: readonly string[]): boolean => candidate.length < eff.length && candidate.every((segment, index) => segment === eff[index]);
  const groups = new Map<string, { anchorIndex: number | null; wrapper: RustModuleNode | null; anchorEff: readonly string[]; root: WeaveNode }>();
  for (const entry of [...weave].sort((left, right) => left.eff.join("::").localeCompare(right.eff.join("::")))) {
    let best: { length: number; anchorIndex: number | null; wrapper: RustModuleNode | null; eff: readonly string[] } | null = null;
    edits.forEach((edit, index) => { if (edit.anchor && prefixOf(entry.eff, edit.anchor.eff) && (!best || edit.anchor.eff.length > best.length)) best = { length: edit.anchor.eff.length, anchorIndex: index, wrapper: null, eff: edit.anchor.eff }; });
    for (const [key, node] of wrapperNodes) {
      const eff = key.split("::");
      if (prefixOf(entry.eff, eff) && (!best || eff.length > best.length)) best = { length: eff.length, anchorIndex: null, wrapper: node, eff };
    }
    if (!best) { problems.push(`cannot weave ${entry.eff.join("::")}: no root module or wrapper encloses it`); continue; }
    const key = `${best.anchorIndex ?? "w"}:${best.eff.join("::")}`;
    const group = groups.get(key) ?? { anchorIndex: best.anchorIndex, wrapper: best.wrapper, anchorEff: best.eff, root: { crate: null, children: new Map() } };
    groups.set(key, group);
    let cursor = group.root;
    entry.eff.slice(best.length).forEach((segment, index, all) => {
      let next = cursor.children.get(segment);
      if (!next) cursor.children.set(segment, (next = { crate: null, children: new Map() }));
      if (index === all.length - 1) next.crate = entry.crate;
      cursor = next;
    });
  }
  const render = (node: WeaveNode, path: readonly string[], indent: string): string[] => {
    const lines: string[] = [];
    for (const [name, child] of [...node.children].sort((left, right) => Buffer.compare(Buffer.from(left[0]), Buffer.from(right[0])))) {
      const here = [...path, name];
      if (!child.children.size && child.crate) { lines.push(`${indent}pub use ${child.crate}::${here.join("::")};`); continue; }
      const owner = child.crate ?? placement.get(here.join("::")) ?? null;
      if (!owner) { problems.push(`cannot weave below ${here.join("::")}: no crate owns that module`); continue; }
      lines.push(`${indent}pub mod ${name} {`, `${indent}    pub use ${owner}::${here.join("::")}::*;`, ...render(child, here, `${indent}    `), `${indent}}`);
    }
    return lines;
  };
  const inserts: RootEdit[] = [];
  for (const group of groups.values()) {
    const lines = render(group.root, group.anchorEff, group.anchorIndex === null ? "    " : "");
    if (!lines.length) continue;
    if (group.anchorIndex !== null) {
      const edit = edits[group.anchorIndex]!;
      const anchor = edit.anchor!;
      const head = anchor.component ? edit.text : `${anchor.visibility} mod ${anchor.eff.at(-1)} {\n    pub use ${anchor.crate}::${anchor.eff.join("::")}::*;\n${lines.map((line) => `    ${line}`).join("\n")}\n}`;
      edits[group.anchorIndex] = { ...edit, text: anchor.component ? `${head}\n${lines.join("\n")}` : head };
    } else {
      const node = group.wrapper!;
      inserts.push({ start: node.bodyEnd, end: node.bodyEnd, text: `${lines.join("\n")}\n` });
    }
  }
  const rootPart = edits.slice(0, rootEdits.length);
  const cutPart = edits.slice(rootEdits.length, rootEdits.length + cutCount).map((edit, index) => ({ ...fileCuts[index]!, text: edit.text }));
  return { edits: [...rootPart, ...inserts], cuts: cutPart, problems };
}



/** 🔗️ Chains that go through a `use crate::..::name;` alias: `name::tail` is `crate::..::name::tail`. */
export function aliasChains(source: string): string[][] {
  const tree = parseRustSourceTree(source);
  const aliases = new Map<string, string[]>();
  for (const item of tree.items) {
    if (item.kind !== "use") continue;
    for (const entry of flattenUse(source.slice(item.declStart, item.end))) if (entry.path[0] === "crate" && entry.path.length > 1 && entry.leaf !== "*") aliases.set(entry.leaf, entry.path.slice(1));
  }
  if (!aliases.size) return [];
  const tokens = rustTokens(source);
  const out: string[][] = [];
  for (let i = 0; i + 2 < tokens.length; i += 1) {
    const base = aliases.get(tokens[i]!.text);
    if (!base || tokens[i]!.kind !== "identifier" || tokens[i + 1]!.text !== "::" || ["::", ".", "use"].includes(tokens[i - 1]?.text ?? "")) continue;
    const chain = [...base];
    let k = i + 2;
    while (tokens[k]?.kind === "identifier") { chain.push(tokens[k]!.text); if (tokens[k + 1]?.text !== "::") break; k += 2; }
    out.push(chain);
  }
  return out;
}

/** 🪞️ Root inline modules that only re-export `crate::`/`super::` paths (`pub mod schema { pub use super::a::b::*; }`): the paths each one aliases and the text to copy into every crate that goes through it. */
export function rootAliasModules(source: string, modules: readonly RustModuleNode[]): Map<string, { targets: string[][]; text: string }> {
  const out = new Map<string, { targets: string[][]; text: string }>();
  for (const node of modules) {
    if (!node.inline || node.pathAttribute !== null || node.children.length || !node.items.length || node.items.some((item) => item.kind !== "use")) continue;
    const targets: string[][] = [];
    for (const item of node.items) {
      const match = /^(?:pub(?:\([^)]*\))?\s+)?use\s+(?:crate|super)::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)(?:::\*)?\s*;$/u.exec(source.slice(item.declStart, item.end).trim());
      if (match) targets.push(match[1]!.split("::")); else { targets.length = 0; break; }
    }
    if (targets.length) out.set(node.name, { targets, text: source.slice(node.start, node.end) });
  }
  return out;
}

/** 🔗️ Strongly connected components of a graph where an edge means "depends on"; dependencies come before their dependents. */
export function stronglyConnected(nodes: readonly string[], edges: ReadonlyMap<string, ReadonlySet<string>>): string[][] {
  const index = new Map<string, number>();
  const low = new Map<string, number>();
  const onStack = new Set<string>();
  const stack: string[] = [];
  const out: string[][] = [];
  let counter = 0;
  const visit = (node: string): void => {
    index.set(node, counter);
    low.set(node, counter);
    counter += 1;
    stack.push(node);
    onStack.add(node);
    for (const next of edges.get(node) ?? []) {
      if (!index.has(next)) { visit(next); low.set(node, Math.min(low.get(node)!, low.get(next)!)); }
      else if (onStack.has(next)) low.set(node, Math.min(low.get(node)!, index.get(next)!));
    }
    if (low.get(node) !== index.get(node)) return;
    const members: string[] = [];
    for (let member = stack.pop()!; ; member = stack.pop()!) {
      onStack.delete(member);
      members.push(member);
      if (member === node) break;
    }
    out.push(members);
  };
  for (const node of nodes) if (!index.has(node)) visit(node);
  return out;
}

/** 🧷️ Prefixes `crate::` to every path that starts with one of the given root module names, which is how root items keep resolving after they move into a module of their own; the entries of a `use` group that follows a path prefix continue that path and are never heads. */
export function qualifyModuleHeads(text: string, modules: ReadonlySet<string>, exclude: ReadonlySet<string> = new Set()): string {
  const tokens = rustTokens(text);
  const at: number[] = [];
  let inUse = false;
  const groups: boolean[] = [];
  for (let k = 0; k + 1 < tokens.length; k += 1) {
    const token = tokens[k]!;
    const previous = tokens[k - 1]?.text;
    if (token.text === "use" && previous !== "::" && previous !== ".") { inUse = true; groups.length = 0; }
    else if (inUse && token.text === ";") inUse = false;
    else if (inUse && token.text === "{") groups.push(previous === "::");
    else if (inUse && token.text === "}") groups.pop();
    const insideGroupOfPath = inUse && (previous === "{" || previous === ",") && groups.at(-1) === true;
    if (token.kind === "identifier" && tokens[k + 1]!.text === "::" && previous !== "::" && previous !== "." && previous !== "$" && !insideGroupOfPath && modules.has(token.text) && !exclude.has(token.text)) at.push(token.start);
  }
  let out = text;
  for (const position of at.sort((a, b) => b - a)) out = `${out.slice(0, position)}crate::${out.slice(position)}`;
  return out;
}

/** 🔝️ Absolute module chains of the `super::` paths written directly in the body of one inline module (nested inline modules are walked as modules of their own). */
export function superChains(text: string, from: number, to: number, excluded: readonly (readonly [number, number])[], base: readonly string[]): string[][] {
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const out: string[][] = [];
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.text !== "super" || token.start < from || token.start >= to || tokens[i - 1]?.text === "::" || excluded.some(([a, b]) => token.start >= a && token.start < b)) continue;
    let up = 1;
    let cursor = i;
    while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.text === "super") { up += 1; cursor += 2; }
    if (up > base.length) continue;
    const chain = base.slice(0, base.length - up);
    let step = cursor + 1;
    while (tokens[step]?.text === "::" && tokens[step + 1]?.kind === "identifier") { chain.push(tokens[step + 1]!.text); step += 2; }
    if (tokens[step]?.text === "::" && tokens[step + 1]?.text === "{") {
      const close = pairs.get(step + 1);
      if (close !== undefined) groupHeads(tokens, pairs, step + 2, close, chain, out);
    } else out.push(chain);
  }
  return out;
}

/** 🪜️ Relative paths of two or more segments (`child::grandchild::item`) that start at a name written without `crate::`, `self::` or `super::`; they reach the grandchildren of the module the text lives in. */
export function relativeHeadChains(text: string): string[][] {
  const tokens = rustTokens(text);
  const out: string[][] = [];
  for (let i = 0; i + 2 < tokens.length; i += 1) {
    const token = tokens[i]!;
    const previous = tokens[i - 1]?.text;
    if (token.kind !== "identifier" || ["crate", "self", "super"].includes(token.text) || tokens[i + 1]!.text !== "::" || previous === "::" || previous === "." || previous === "$") continue;
    const chain = [token.text];
    let cursor = i;
    while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") { chain.push(tokens[cursor + 2]!.text); cursor += 2; }
    if (chain.length >= 2) out.push(chain);
  }
  return out;
}

/** 🧭️ Names a source uses as the head of a path (`name::..`, `self::name`, `super::name`), which is how a module reaches a module it mounts. */
export function pathHeadNames(text: string): Set<string> {
  const tokens = rustTokens(text);
  const out = new Set<string>();
  let inUse = false;
  for (let i = 0; i + 1 < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.text === "use") inUse = true;
    else if (token.text === ";") inUse = false;
    if (token.kind !== "identifier") continue;
    const previous = tokens[i - 1]?.text;
    const next = tokens[i + 1]!.text;
    if (next === "::" && previous !== "." && (previous !== "::" || ["self", "super"].includes(tokens[i - 2]?.text ?? ""))) out.add(token.text);
    else if (previous === "use" && (next === ";" || next === "as")) out.add(token.text);
    else if (inUse && (previous === "{" || previous === ",") && (next === "," || next === "}")) out.add(token.text);
  }
  return out;
}

export function implHeader(text: string): { trait: string | null; self: string | null } | null {
  const tokens = rustTokens(text);
  const start = tokens.findIndex((token) => token.text === "impl");
  if (start < 0) return null;
  let cursor = start + 1;
  if (tokens[cursor]?.text === "<") {
    let depth = 0;
    for (; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "<") depth += 1;
      else if (tokens[cursor]!.text === ">") { depth -= 1; if (depth === 0) { cursor += 1; break; } }
    }
  }
  let end = cursor;
  let depth = 0;
  let forAt = -1;
  for (; end < tokens.length; end += 1) {
    const token = tokens[end]!.text;
    if (token === "<") depth += 1;
    else if (token === ">") depth -= 1;
    else if (depth === 0 && token === "for" && forAt < 0) forAt = end;
    else if (depth === 0 && (token === "{" || token === "where")) break;
  }
  const lastIdent = (from: number, to: number): string | null => {
    let name: string | null = null;
    let nested = 0;
    for (let i = from; i < to; i += 1) {
      const token = tokens[i]!;
      if (token.text === "<") nested += 1;
      else if (token.text === ">") nested -= 1;
      else if (nested === 0 && token.kind === "identifier" && !["dyn", "mut", "impl", "for"].includes(token.text)) name = token.text;
    }
    return name;
  };
  if (forAt >= 0) return { trait: lastIdent(cursor, forAt), self: lastIdent(forAt + 1, end) };
  return { trait: null, self: lastIdent(cursor, end) };
}

export { identifierSet, useLeafNames };
