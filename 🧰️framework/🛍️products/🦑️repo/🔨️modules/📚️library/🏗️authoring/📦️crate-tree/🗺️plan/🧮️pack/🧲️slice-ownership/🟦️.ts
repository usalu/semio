/** 🧲️ The mirror measurement closures, the slice infos and the chain ownership lookups of the artifact crate pack planner. */
import { dirname, join } from "node:path";
import type { CrateMeasure, SliceInfo } from "../../🧾️model/🟦️.ts";
import { demoteDanglingOuterDocs, parseRustSourceTree } from "../../../🌳️module-tree/🟦️.ts";
import { type CarveSlice, type ModuleDecl } from "../../../🧱️decl-tree/🟦️.ts";
import { type MirrorSpec, type MirrorUnit, renderMirrorRoot } from "../../../📝️render/🟦️.ts";
import { removeRange } from "../../🧰️tree-helpers/🟦️.ts";
import { crateTreeWasmCfg, measureRustCrate, readMeasureSources } from "../../📏️measure/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";

/** 📐 Defines the unit, cut overlay, wrapper spec and measurement closures that mirror one set of slices as a crate. */
export function defineMirrorMeasurement({ built, declTree, sliceOfDecl, readText, measureRoot, wrapperNodes, gates, baseOverlay, repoRoot }: Pick<ReturnType<typeof buildCarveSlices>, "built" | "declTree" | "sliceOfDecl" | "wrapperNodes" | "gates"> & Pick<ReturnType<typeof readCrateSource>, "readText" | "baseOverlay"> & Pick<ReturnType<typeof indexRootItems>, "measureRoot"> & { readonly repoRoot: string }) {
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
    return { spec: { doc: "//! 🪞️ planning mirror", newRootDir: measureRoot, innerAttributes: [], externLines: [], movedItemsText: "", itemImports: [], owned, imported: [], wrapperNodes, gates, ...extra }, attach };
  };
  const measureMembers = async (members: readonly CarveSlice[], withTest: boolean, cfgOverride?: readonly string[]): Promise<CrateMeasure> => {
    const { spec } = effWrappersSpec(members);
    const entry = join(measureRoot, "🦀️.rs");
    const overlay = new Map<string, string>([...baseOverlay, ...cutOverlay(members), [entry, renderMirrorRoot(spec)]]);
    return measureRustCrate(repoRoot, entry, withTest, overlay, measureRoot, cfgOverride);
  };
  return { measureMembers, cutOverlay, effWrappersSpec, unitsOf };
}

/** 🧮 Collects the coherence types, trait names, type definitions and one info per carvable slice. */
export async function collectSliceInfos({ rootSource, declTree, readText, carvable, measureMembers }: Pick<ReturnType<typeof readCrateSource>, "rootSource" | "readText"> & Pick<ReturnType<typeof buildCarveSlices>, "declTree" | "carvable"> & Pick<ReturnType<typeof defineMirrorMeasurement>, "measureMembers">) {
  const wasmGated = /target_arch\s*=\s*"wasm32"/u.test(rootSource) || [...declTree.decls.values()].some((decl) => !decl.node.inline && decl.file !== null && /target_arch\s*=\s*"wasm32"/u.test(readText(decl.file) ?? ""));
  const coherenceType = new Map<string, string>();
  const traitNames = new Set<string>();
  const typeDefs = new Map<string, Set<string>>();
  const infos: SliceInfo[] = [];
  const infoOf = new Map<string, SliceInfo>();
  for (const [index, slice] of carvable.entries()) {
    const info: SliceInfo = { slice, index, effs: slice.attach.map((decl) => decl.eff.join("::")), lib: null, test: null, wasm: null, libEdges: new Set(), testEdges: new Set(), rootRefs: new Set(), testRootRefs: new Set(), keptRefs: new Set(), shimRefs: new Set(), testShimRefs: new Set(), heads: new Set(), coherence: new Set(), why: new Map(), firstFile: slice.attach.map((decl) => decl.file).find((file): file is string => file !== null) ?? "" };
    info.lib = await measureMembers([slice], false);
    info.test = await measureMembers([slice], true);
    if (wasmGated) info.wasm = await measureMembers([slice], false, crateTreeWasmCfg());
    infos.push(info);
    infoOf.set(slice.id, info);
  }
  return { infos, coherenceType, traitNames, typeDefs, infoOf };
}

/** 🧭 Indexes the owners of declaration paths and defines the member visibility, source text and shim lookups of path chains. */
export function defineChainOwnership({ infos, declTree, readText, measureRoot, baseOverlay, cutOverlay, effWrappersSpec, repoRoot }: Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos"> & Pick<ReturnType<typeof buildCarveSlices>, "declTree"> & Pick<ReturnType<typeof readCrateSource>, "readText" | "baseOverlay"> & Pick<ReturnType<typeof indexRootItems>, "measureRoot"> & Pick<ReturnType<typeof defineMirrorMeasurement>, "cutOverlay" | "effWrappersSpec"> & { readonly repoRoot: string }) {
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
  const declEffKeys = new Set<string>([...declTree.decls.values()].map((decl) => decl.eff.join("\0")));
  const ownerOfReachedChain = (chain: readonly string[]): SliceInfo | null => {
    for (let length = Math.min(chain.length, longestEff); length >= 0; length--) {
      const owner = ownerByEff.get(chain.slice(0, length).join("\0"));
      if (!owner) continue;
      const next = chain.slice(0, length + 1).join("\0");
      return length < chain.length && declEffKeys.has(next) && !owner.slice.decls.some((decl) => decl.eff.join("\0") === next) ? null : owner;
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
    const gated = decls.find((decl) => decl.testOnly && decl.eff.length > 0);
    if (gated) return `${gated.eff.join("::")} (cfg(test) module)`;
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
  return { ownerOfChain, noteWhy, restrictedTarget, sourcesOf, ownerOfReachedChain, membersOf, underPath };
}
