/** 🗜️ The component layering and the packing of components into crates of the artifact crate pack planner. */
import { join, relative } from "node:path";
import type { PackingStats, PlanOptions, SliceInfo } from "../../🧾️model/🟦️.ts";
import { type CarveSlice } from "../../../🧱️decl-tree/🟦️.ts";
import { measureFits } from "../../📏️measure/🟦️.ts";
import { CRATE_HARD_BUDGET } from "../../💰️budget/🟦️.ts";
import { MeasureUnion, totalsFit } from "../../../📏️measure-union/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { defineMirrorMeasurement } from "../🧲️slice-ownership/🟦️.ts";
import type { collectComponentBlockers, mergeCoupledComponents } from "../🧵️weave-merge-and-blockers/🟦️.ts";
import type { PackedGroup } from "../📚️pack-types/🟦️.ts";
import type { CarvedCrate } from "../../../🧱️carved-layout/🟦️.ts";
import { stronglyConnected } from "../../🧭️module-chains/🟦️.ts";

/** 🪜 Layers the components by dependency depth and defines the group, acyclicity and mirror validity helpers. */
export function prepareComponentPacking({ componentList, edgesOf, repoRoot, measureRoot, options }: Pick<ReturnType<typeof collectComponentBlockers>, "componentList"> & Pick<ReturnType<typeof mergeCoupledComponents>, "edgesOf"> & { readonly repoRoot: string; readonly options: PlanOptions } & Pick<ReturnType<typeof indexRootItems>, "measureRoot">) {
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
  const pinDeps = new Map<string, readonly string[]>((options.carved?.crates ?? []).filter((crate) => crate.signed).map((crate) => [crate.name, crate.dependsOn]));
  const groupEdges = (grouping: readonly { components: number[]; pin?: CarvedCrate }[]): Set<number>[] => {
    const owner = new Map<number, number>();
    const pinOwner = new Map<string, number>();
    grouping.forEach((group, i) => { group.components.forEach((c) => owner.set(c, i)); if (group.pin) for (const crate of [group.pin, ...((group as PackedGroup).supersedes ?? [])]) pinOwner.set(crate.name, i); });
    const names = (group: { pin?: CarvedCrate }): string[] => [group.pin, ...((group as PackedGroup).supersedes ?? [])].flatMap((crate) => (crate ? [crate.name] : []));
    const reaches = (from: string, to: string, seen = new Set<string>()): boolean => !seen.has(from) && (seen.add(from), (pinDeps.get(from) ?? []).some((next) => next === to || reaches(next, to, seen)));
    const reverse = (i: number, j: number): boolean => names(grouping[j]!).some((b) => names(grouping[i]!).some((a) => reaches(b, a)));
    const detached = (c: number): boolean => componentList[c]!.every((member) => member.slice.attach.every((decl) => decl.testOnly));
    return grouping.map((group, i) => new Set([...group.components.flatMap((c) => (detached(c) && group.pin ? [] : [...componentDeps[c]!]).map((dep) => owner.get(dep)!)).filter((dep) => dep === undefined || !(group.pin && grouping[dep]?.pin && reverse(i, dep))), ...[group.pin, ...((group as PackedGroup).supersedes ?? [])].flatMap((crate) => (crate ? pinDeps.get(crate.name) ?? [] : []).map((name) => pinOwner.get(name)!))].filter((dep) => dep !== undefined && dep !== i)));
  };
  const acyclic = (grouping: readonly { components: number[]; pin?: CarvedCrate }[]): boolean => {
    const edges = groupEdges(grouping);
    const state = grouping.map(() => 0);
    const visit = (i: number): boolean => { if (state[i] === 1) return false; if (state[i] === 2) return true; state[i] = 1; for (const next of edges[i]!) if (!visit(next)) return false; state[i] = 2; return true; };
    return grouping.every((_, i) => visit(i));
  };
  const mirrorValid = (members: readonly CarveSlice[]): boolean => {
    const attached = members.flatMap((slice) => slice.attach);
    return !attached.some((outer) => !outer.node.inline && !outer.isComponent && attached.some((inner) => inner !== outer && inner.eff.length > outer.eff.length && outer.eff.every((segment, index) => segment === inner.eff[index])));
  };
  const pinned = new Set<number>();
  const carved = (options.carved?.crates ?? []).filter((crate) => crate.signed);
  const ownerOf = new Map<string, CarvedCrate>(carved.flatMap((crate) => (crate.recorded ?? crate.owned).map((eff): [string, CarvedCrate] => [eff, crate])));
  const parent = new Map<CarvedCrate, CarvedCrate>(carved.map((crate) => [crate, crate]));
  const rootOf = (crate: CarvedCrate): CarvedCrate => { let at = crate; while (parent.get(at) !== at) at = parent.get(at)!; return at; };
  const slicePin = new Map<string, CarvedCrate[]>();
  const declOwner = new Map<string, CarvedCrate>();
  const allInfos = componentList.flat();
  for (const info of allInfos) {
    const named = (decls: readonly { eff: readonly string[] }[]): CarvedCrate[] => [...new Set(decls.map((decl) => ownerOf.get(decl.eff.join("::"))).filter((entry): entry is CarvedCrate => entry !== undefined))];
    const crates = named(info.slice.attach).length ? named(info.slice.attach) : named(info.slice.decls);
    if (crates.length) { slicePin.set(info.slice.id, crates); for (const decl of info.slice.decls) declOwner.set(decl.id, crates[0]!); }
  }
  for (let changed = true; changed;) {
    changed = false;
    for (const info of allInfos) {
      if (slicePin.has(info.slice.id)) continue;
      const inherited = info.slice.attach.map((decl) => (decl.parentId ? declOwner.get(decl.parentId) : undefined)).find((entry) => entry !== undefined && entry.recorded === null);
      if (!inherited) continue;
      slicePin.set(info.slice.id, [inherited]);
      for (const decl of info.slice.decls) declOwner.set(decl.id, inherited);
      changed = true;
    }
  }
  const crateOfComponent = (index: number): CarvedCrate[] => [...new Set(componentList[index]!.flatMap((member) => slicePin.get(member.slice.id) ?? []))];
  for (const index of order) { const crates = crateOfComponent(index); for (const other of crates.slice(1)) parent.set(rootOf(other), rootOf(crates[0]!)); }
  const byCrate = new Map<CarvedCrate, number[]>();
  for (const index of order) { const crates = crateOfComponent(index); if (crates.length) byCrate.set(rootOf(crates[0]!), [...(byCrate.get(rootOf(crates[0]!)) ?? []), index]); }
  for (const crate of carved) {
    const components = byCrate.get(crate);
    if (!components) continue;
    const group = startGroup(components[0]!, componentList[components[0]!]!);
    for (const index of components.slice(1)) { group.components.push(index); group.infos.push(...componentList[index]!); group.lib.add(componentList[index]!.map((member) => member.lib!.entries)); group.test.add(componentList[index]!.map((member) => member.test!.entries)); }
    const hard = options.hard ?? CRATE_HARD_BUDGET;
    const limit = { ...hard, files: hard.files - 1 };
    const fits = (g: PackedGroup): boolean => totalsFit(g.lib.peek([]), limit) && totalsFit(g.test.peek([]), limit);
    let kept = [...components];
    let current = group;
    while (options.tight?.includes(crate.name) && !fits(current) && kept.length > 1) {
      const testOnly = (index: number): number => Number(componentList[index]!.every((m) => m.slice.attach.every((d) => d.testOnly)));
      const drop = [...kept].sort((a, b) => testOnly(b) - testOnly(a) || b - a)[0]!;
      kept = kept.filter((index) => index !== drop);
      current = startGroup(kept[0]!, componentList[kept[0]!]!);
      for (const index of kept.slice(1)) { current.components.push(index); current.infos.push(...componentList[index]!); current.lib.add(componentList[index]!.map((member) => member.lib!.entries)); current.test.add(componentList[index]!.map((member) => member.test!.entries)); }
    }
    Object.assign(group, current);
    components.splice(0, components.length, ...kept);
    group.pin = crate;
    group.supersedes = carved.filter((other) => other !== crate && rootOf(other) === crate);
    groups.push(group);
    for (const index of components) pinned.add(index);
  }
  const settle = (): void => {
    const rings = stronglyConnected(groups.map((_, i) => String(i)), new Map(groupEdges(groups).map((edges, i) => [String(i), new Set([...edges].map(String))]))).filter((ring) => ring.length > 1);
    const absorbed = new Set<number>();
    for (const ring of rings) {
      const [head, ...rest] = ring.map(Number).sort((a, b) => a - b);
      const into = groups[head!]!;
      for (const at of rest) {
        const from = groups[at]!;
        for (const index of from.components) into.components.push(index);
        into.infos.push(...from.infos);
        into.lib.add(from.infos.map((info) => info.lib!.entries));
        into.test.add(from.infos.map((info) => info.test!.entries));
        into.supersedes = [...(into.supersedes ?? []), ...(from.pin ? [from.pin] : []), ...(from.supersedes ?? [])];
        absorbed.add(at);
      }
    }
    if (!absorbed.size) return;
    const kept = groups.filter((_, i) => !absorbed.has(i));
    groups.splice(0, groups.length, ...kept);
    groups.forEach((group, i) => group.infos.forEach((info) => groupOfSlice.set(info.slice.id, i)));
  };
  settle();
  return { order, groupOfSlice, groups, incremental, packing, acyclic, mirrorValid, startGroup, pinned, settle };
}

/** 🧳 Packs the components into crate groups by the measured budget. */
export async function packComponents({ order, componentList, sliceOfDecl, groupOfSlice, edgesOf, groups, incremental, target, packing, acyclic, mirrorValid, measureMembers, startGroup, pinned, settle }: Pick<ReturnType<typeof prepareComponentPacking>, "order" | "groupOfSlice" | "groups" | "incremental" | "packing" | "acyclic" | "mirrorValid" | "startGroup" | "pinned" | "settle"> & Pick<ReturnType<typeof collectComponentBlockers>, "componentList"> & Pick<ReturnType<typeof buildCarveSlices>, "sliceOfDecl"> & Pick<ReturnType<typeof mergeCoupledComponents>, "edgesOf"> & Pick<ReturnType<typeof readCrateSource>, "target"> & Pick<ReturnType<typeof defineMirrorMeasurement>, "measureMembers">) {
  for (const index of order) {
    if (pinned.has(index)) continue;
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
      const limit = group.pin ? { ...target, files: target.files - 2 } : target;
      if (incremental && (!totalsFit(group.lib.peek(members.map((member) => member.lib!.entries)), limit) || !totalsFit(group.test.peek(members.map((member) => member.test!.entries)), limit))) { packing.pruned += 1; continue; }
      const trial = groups.map((entry, i) => ({ ...entry, components: i === candidate ? [...entry.components, index] : entry.components }));
      if (!acyclic(trial)) continue;
      const merged = [...group.infos, ...members].map((info) => info.slice);
      if (!mirrorValid(merged)) continue;
      packing.measured += 1;
      if (!measureFits(await measureMembers(merged, false), limit) || !measureFits(await measureMembers(merged, true), limit)) continue;
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
  settle();
}
