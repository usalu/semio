/** 📥️ The crate source folding, the carve slices and the root item index of the artifact crate pack planner. */
import { dirname, join, relative, resolve } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import type { PlanOptions } from "../../🧾️model/🟦️.ts";
import { CRATE_HARD_BUDGET, CRATE_TARGET_BUDGET } from "../../💰️budget/🟦️.ts";
import { externCrateRows, readCargoLib, topLevelNames } from "../../📦️cargo-lib/🟦️.ts";
import { inlineMountTreeFiles } from "../../../🌲️inline-mounts/🟦️.ts";
import { parseRustSourceTree, type RustModuleNode, type RustSourceItem, useLeafNames } from "../../../🌳️module-tree/🟦️.ts";
import { transformTestClosure } from "../../../🧪️test-closure/🟦️.ts";
import { liftModuleKids } from "../../../🪜️lift-kids/🟦️.ts";
import { inlineRootFileModules } from "../../../🫗️inline-root/🟦️.ts";
import { convertMutationLeaves } from "../../../🧬️mutation-families/🟦️.ts";
import { buildSlices, type CarveSlice, collectDecls, type ModuleDecl } from "../../../🧱️decl-tree/🟦️.ts";
import { crateRelativeChains, qualifyModuleHeads, rootAliasModules } from "../../🧭️module-chains/🟦️.ts";
import { testOnlyGate } from "../../../🧰️testkit/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/📄️parse/🟦️.ts";

/** 📖 Reads the manifest and the crate root source, folds the mount trees and applies the test, lift and inline transforms. */
export function readCrateSource({ options, repoRoot, packageDirRel }: { readonly options: PlanOptions; readonly repoRoot: string; readonly packageDirRel: string }) {
  const target = options.target ?? CRATE_TARGET_BUDGET;
  const hard = options.hard ?? CRATE_HARD_BUDGET;
  const manifestPath = resolve(repoRoot, packageDirRel, "Cargo.toml");
  const lib = readCargoLib(manifestPath);
  const manifest = runtimeParseTomlV1(readFileSync(manifestPath, "utf8")) as { package?: { workspace?: string } };
  const oldRootDir = dirname(lib.entry);
  const artifactDir = resolve(dirname(manifestPath), manifest.package?.workspace ?? ".");
  const folded = inlineMountTreeFiles(lib.entry, readFileSync(lib.entry, "utf8"), (absolute) => (existsSync(absolute) ? readFileSync(absolute, "utf8") : null));
  const originalRootSource = folded.rootSource;
  const originalTree = parseRustSourceTree(originalRootSource);
  const rootItemNames = new Set<string>([...topLevelNames(originalRootSource, originalTree.items)].filter(([, item]) => item.kind !== "use").map(([name]) => name));
  for (const node of originalTree.modules) if (!node.inline) rootItemNames.add(node.name);
  const testTransform = options.tests ? transformTestClosure(repoRoot, lib.entry, originalRootSource, rootItemNames, options.tests) : null;
  const lifted = options.lift?.length ? liftModuleKids(testTransform, lib.entry, originalRootSource, options.lift) : testTransform;
  const transform = options.inlineRootModules ? inlineRootFileModules(lifted, lib.entry, originalRootSource) : lifted;
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
  return { lib, rootSource, readText, baseOverlay, tree, manifestPath, problems, target, hard, artifactDir, transform, folded };
}

/** 🍰 Converts mutation leaves, collects the declaration tree and builds the carvable slices with their gates and wrappers. */
export function buildCarveSlices({ convert, lib, rootSource, readText, baseOverlay, options, tree }: { readonly convert: boolean; readonly options: PlanOptions } & Pick<ReturnType<typeof readCrateSource>, "lib" | "rootSource" | "readText" | "baseOverlay" | "tree">) {
  const family = convert ? convertMutationLeaves(lib.entry, rootSource, readText) : null;
  for (const [file, text] of [...(family?.edits ?? []), ...(family?.creates ?? [])]) baseOverlay.set(file, text);
  const detaching = new Set<string>((family?.aggregates ?? []).filter((aggregate) => (family?.converted.length ?? 0) > 0).map((aggregate) => aggregate.name));
  const declTree = collectDecls(lib.entry, rootSource, readText);
  const built = buildSlices(declTree, readText, undefined, { detachTestsMentioning: detaching, detachAllTests: options.weave !== false });
  const rootRelatives = new Set(declTree.roots);
  const keptModules = tree.modules.filter((node) => !node.inline).map((node) => node.name);
  const shims = rootAliasModules(rootSource, tree.modules);
  const unnestShim = (chain: string[]): string[] => { const shim = chain[0] ? shims.get(chain[0]) : undefined; return shim && chain[1] !== undefined && shim.nested.has(chain[1]) ? [chain[0]!, ...chain.slice(2)] : chain; };
  const isShimDecl = (decl: ModuleDecl): boolean => rootRelatives.has(decl.id) && decl.node.inline && shims.has(decl.node.name);
  const carvable = built.slices.filter((slice) => !slice.attach.every((decl) => (rootRelatives.has(decl.id) && !decl.node.inline) || isShimDecl(decl)) && !slice.attach.some((decl) => decl.testOnly && rootRelatives.has(decl.id)));
  const sliceOfDecl = new Map<string, CarveSlice>();
  for (const slice of carvable) for (const decl of slice.decls) sliceOfDecl.set(decl.id, slice);
  const gates = new Map<string, readonly string[]>();
  for (const decl of declTree.decls.values()) {
    const cfgs = decl.node.attributeTexts.filter((text) => /^cfg\s*\(/u.test(text.trim()) && !/^cfg\s*\(\s*test\s*\)$/u.test(text.trim()));
    if (cfgs.length && !decl.testOnly) gates.set(decl.eff.join("::"), cfgs.map((text) => `#[${text}]`));
  }
  const wrapperNodes = new Map<string, RustModuleNode>();
  for (const id of built.namespaces) {
    const decl = declTree.decls.get(id)!;
    if (decl.inlineRoot && decl.node.inline) wrapperNodes.set(decl.eff.join("::"), decl.node);
  }
  return { declTree, built, sliceOfDecl, wrapperNodes, gates, carvable, shims, unnestShim, isShimDecl, keptModules, family };
}

/** 🗂 Indexes the crate root items: extern rows, gated uses, use targets, regions and the measurement root. */
export function indexRootItems({ rootSource, tree, declTree, manifestPath }: Pick<ReturnType<typeof readCrateSource>, "rootSource" | "tree" | "manifestPath"> & Pick<ReturnType<typeof buildCarveSlices>, "declTree">) {
  const externRows = externCrateRows(rootSource, tree.items);
  const rootNames = topLevelNames(rootSource, tree.items);
  const cfgAttributeLines = (item: RustSourceItem): string => item.attributeTexts.filter((attribute) => /^cfg\s*\(/u.test(attribute.trim())).map((attribute) => `#[${attribute}]\n`).join("");
  const gateOfName = new Map(tree.items.filter((item) => item.name && testOnlyGate(item.attributeTexts) !== null).map((item) => [item.name!, testOnlyGate(item.attributeTexts)!]));
  const gatedUses = (path: string, names: Iterable<string>): string[] => {
    const sorted = [...names].sort();
    const byGate = new Map<string | null, string[]>();
    for (const name of sorted) byGate.set(gateOfName.get(name) ?? null, [...(byGate.get(gateOfName.get(name) ?? null) ?? []), name]);
    const order = [...byGate.keys()].sort((a, b) => (a === null ? -1 : b === null ? 1 : a < b ? -1 : a > b ? 1 : 0));
    return order.map((gate) => `${gate === null ? "" : `#[${gate}]\n`}pub use ${path}::{${byGate.get(gate)!.join(", ")}};`);
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
  return { measureRoot, rootModuleNames, rootNames, rootUseTargets, itemByName, regionOf, qualifyRootHeads, cfgAttributeLines, gatedUses, externRows };
}
