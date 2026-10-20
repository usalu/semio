/** 🔗️ The slice edge resolution and the root item closures of the artifact crate pack planner. */
import { relative } from "node:path";
import type { CrateMeasure, SliceInfo } from "../../🧾️model/🟦️.ts";
import { referencedItemNames } from "../../../🌳️module-tree/🟦️.ts";
import { aliasChains, crateRelativeChains, implHeader, pathHeadNames, relativeHeadChains, superChains } from "../../🧭️module-chains/🟦️.ts";
import { allItems } from "../../🧰️tree-helpers/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { collectSliceInfos, defineChainOwnership } from "../🧲️slice-ownership/🟦️.ts";

/** 🕸 Resolves the library and test edges, root references and coherence of every slice info. */
export function resolveSliceEdges({ shims, ownerOfChain, noteWhy, restrictedTarget, coherenceType, rootModuleNames, declTree, infos, sourcesOf, unnestShim, ownerOfReachedChain, membersOf, rootNames, rootUseTargets, isShimDecl, underPath, traitNames, typeDefs, problems }: Pick<ReturnType<typeof buildCarveSlices>, "shims" | "declTree" | "unnestShim" | "isShimDecl"> & Pick<ReturnType<typeof defineChainOwnership>, "ownerOfChain" | "noteWhy" | "restrictedTarget" | "sourcesOf" | "ownerOfReachedChain" | "membersOf" | "underPath"> & Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "coherenceType" | "infos" | "traitNames" | "typeDefs"> & Pick<ReturnType<typeof indexRootItems>, "rootModuleNames" | "rootNames" | "rootUseTargets"> & Pick<ReturnType<typeof readCrateSource>, "problems">) {
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
        for (const chain of [...crateRelativeChains(text), ...aliasChains(text)].map(unnestShim)) {
          if (viaShim(info, chain, edges, shimRefs, file)) continue;
          const owner = ownerOfReachedChain(chain);
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
              const via = ownerOfChain([...targetChain, ...chain.slice(1)]);
              if (via && via !== info) edges.add(via.slice.id);
            }
          } else if (!owner && chain[0] && rootModuleNames.has(chain[0]) && !declTree.decls.has(chain[0]) ) info.keptRefs.add(chain.slice(0, 2).join("::"));
        }
      }
    };
    scan(info.lib!, info.libEdges, info.rootRefs, info.shimRefs);
    if (info.wasm) scan(info.wasm, info.libEdges, info.rootRefs, info.shimRefs);
    scan(info.test!, info.testEdges, info.testRootRefs, info.testShimRefs);
    for (const decl of info.slice.decls) {
      if (!decl.node.inline || isShimDecl(decl)) continue;
      const nested = decl.node.children.filter((child) => child.inline).map((child): [number, number] => [child.start, child.end]);
      for (const chain of superChains(decl.declText, decl.node.bodyStart, decl.node.bodyEnd, nested, decl.eff)) {
        const owner = ownerOfChain(chain);
        if (owner && owner !== info) (decl.testOnly ? info.testEdges : info.libEdges).add(owner.slice.id);
      }
    }
    for (const measure of [info.lib!, info.test!, ...(info.wasm ? [info.wasm] : [])]) for (const [, text] of sourcesOf(info, measure)) for (const name of pathHeadNames(text)) info.heads.add(name);
    for (const decl of info.slice.decls) for (const name of info.heads) {
      const childEff = [...decl.eff, name];
      const owner = ownerOfChain(childEff);
      if (owner && owner !== info && owner.slice.attach.some((owned) => underPath(owned.eff, childEff))) info.libEdges.add(owner.slice.id);
    }
    for (const [measure, edges] of [[info.lib!, info.libEdges], [info.test!, info.testEdges], ...(info.wasm ? [[info.wasm, info.libEdges] as const] : [])] as const) {
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
  return { chainOwners };
}

/** 🔁 Computes the transitive closure of the root items every slice references and picks the closest owners. */
export function closeRootItemReferences({ itemByName, regionOf, tree, qualifyRootHeads, rootSource, unnestShim, chainOwners, infos, infoOf, sourcesOf, traitNames, typeDefs, problems, coherenceType }: Pick<ReturnType<typeof indexRootItems>, "itemByName" | "regionOf" | "qualifyRootHeads"> & Pick<ReturnType<typeof readCrateSource>, "tree" | "rootSource" | "problems"> & Pick<ReturnType<typeof buildCarveSlices>, "unnestShim"> & Pick<ReturnType<typeof resolveSliceEdges>, "chainOwners"> & Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos" | "infoOf" | "traitNames" | "typeDefs" | "coherenceType"> & Pick<ReturnType<typeof defineChainOwnership>, "sourcesOf">) {
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
        for (const chain of [...crateRelativeChains(text), ...aliasChains(text)].map(unnestShim)) for (const owner of chainOwners(chain)) { out.add(owner.slice.id) }
        for (const identifier of referencedItemNames(text)) if (itemByName.has(identifier)) { queue.push(identifier) }
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
        if (header.traitArgs.some((argument) => typeDefs.get(argument)?.has(info.slice.id))) continue;
        const declared = typeDefs.get(header.self);
        const owners = declared && declared.size > 1 && !declared.has(info.slice.id) ? closestOwners(info, declared) : declared;
        if (!owners || owners.has(info.slice.id) || owners.size !== 1) { if (owners && owners.size > 1 && !owners.has(info.slice.id)) problems.push(`ambiguous coherence: ${info.effs.join(",")} implements for ${header.self}, defined in ${owners.size} slices`); continue; }
        info.coherence.add([...owners][0]!);
        coherenceType.set(`${info.slice.id}>${[...owners][0]!}`, `${header.self} @ ${implFile.split("/").slice(-4).join("/")}`);
      }
    }
  }
}
