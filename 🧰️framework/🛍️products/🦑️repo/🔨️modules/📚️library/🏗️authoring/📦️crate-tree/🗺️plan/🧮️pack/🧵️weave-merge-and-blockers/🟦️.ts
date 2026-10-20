/** 🧵️ The root item weaving, the component merging and the blockers of the artifact crate pack planner. */
import { dirname, resolve } from "node:path";
import type { PlanBlocker, PlanOptions, SliceInfo } from "../../🧾️model/🟦️.ts";
import { type CarveSlice, type ModuleDecl } from "../../../🧱️decl-tree/🟦️.ts";
import { aliasChains, crateRelativeChains, stronglyConnected } from "../../🧭️module-chains/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { collectSliceInfos, defineChainOwnership } from "../🧲️slice-ownership/🟦️.ts";

/** 🧶 Plans which root module items are woven into the slices and which tests are dropped. */
export function planRootItemWeaving({ tree, qualifyRootHeads, rootSource, unnestShim, shims, readText, lib, underPath, infos, infoOf, sliceOfDecl, declTree, options }: Pick<ReturnType<typeof readCrateSource>, "tree" | "rootSource" | "readText" | "lib"> & Pick<ReturnType<typeof indexRootItems>, "qualifyRootHeads"> & Pick<ReturnType<typeof buildCarveSlices>, "unnestShim" | "shims" | "sliceOfDecl" | "declTree"> & Pick<ReturnType<typeof defineChainOwnership>, "underPath"> & Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos" | "infoOf"> & { readonly options: PlanOptions }) {
  const woven = new Set<string>();
  const droppedTests = new Set<string>();
  const rootItemPaths: string[][] = tree.items.filter((item) => item.kind !== "use").flatMap((item) => {
    const text = qualifyRootHeads(rootSource.slice(item.declStart, item.end));
    return [...crateRelativeChains(text), ...aliasChains(text)].map(unnestShim).flatMap((chain) => { const shim = chain[0] ? shims.get(chain[0]) : undefined; return shim ? shim.targets.map((target) => [...target, ...chain.slice(1)]) : [chain]; });
  });
  for (const node of tree.modules) {
    if (node.inline || node.conditionalTest || node.pathAttribute === null) continue;
    const keptText = readText(resolve(dirname(lib.entry), node.pathAttribute));
    if (keptText === null) continue;
    const qualified = qualifyRootHeads(keptText);
    for (const chain of [...crateRelativeChains(qualified), ...aliasChains(qualified)].map(unnestShim)) {
      const shim = chain[0] ? shims.get(chain[0]) : undefined;
      if (shim) for (const target of shim.targets) rootItemPaths.push([...target, ...chain.slice(1)]);
      else rootItemPaths.push(chain);
    }
  }
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
  return { droppedTests, woven };
}

/** 🧩 Merges slices that are coupled by coherence, import cycles and file chains into components. */
export function mergeCoupledComponents({ infos, coherenceType, infoOf }: Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos" | "coherenceType" | "infoOf">) {
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
    for (const a of infos) for (const outer of a.slice.decls) {
      if (outer.node.inline || outer.isComponent || outer.file === null) continue;
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
  return { find, mergeCauses, preGroup, edgesOf };
}

/** 🚧 Groups the merged slices into components and collects the blockers that keep a component from being carved. */
export function collectComponentBlockers({ infos, find, target, mergeCauses, infoOf, preGroup, edgesOf }: Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos" | "infoOf"> & Pick<ReturnType<typeof mergeCoupledComponents>, "find" | "mergeCauses" | "preGroup" | "edgesOf"> & Pick<ReturnType<typeof readCrateSource>, "target">) {
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
        if (process.env.F3_SLICES) for (const member of members) lines.push(`slice ${member.effs.map((eff) => eff.split("::").slice(-4).join("::")).join(" + ")} [${name(preGroup.get(member.slice.id)!)}] files=${member.lib?.files ?? 0} bytes=${member.lib?.bytes ?? 0} lines=${member.lib?.lines ?? 0} -> ${[...edgesOf(member)].filter((id) => ids.has(id)).map((id) => `${effOfSlice(id).split("::").slice(-4).join("::")}${member.libEdges.has(id) ? "" : " [test]"} (${member.why.get(id) ?? "?"})`).join(" ; ")}`);
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
  return { componentList, blockers };
}
