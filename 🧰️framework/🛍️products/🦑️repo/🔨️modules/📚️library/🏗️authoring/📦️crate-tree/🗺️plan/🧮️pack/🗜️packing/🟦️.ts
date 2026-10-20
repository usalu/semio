/** 🗜️ The component layering and the packing of components into crates of the artifact crate pack planner. */
import { join, relative } from "node:path";
import type { PackingStats, PlanOptions, SliceInfo } from "../../🧾️model/🟦️.ts";
import { type CarveSlice } from "../../../🧱️decl-tree/🟦️.ts";
import { measureFits } from "../../📏️measure/🟦️.ts";
import { MeasureUnion, totalsFit } from "../../../📏️measure-union/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { defineMirrorMeasurement } from "../🧲️slice-ownership/🟦️.ts";
import type { collectComponentBlockers, mergeCoupledComponents } from "../🧵️weave-merge-and-blockers/🟦️.ts";
import type { PackedGroup } from "../📚️pack-types/🟦️.ts";

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
  return { order, groupOfSlice, groups, incremental, packing, acyclic, mirrorValid, startGroup };
}

/** 🧳 Packs the components into crate groups by the measured budget. */
export async function packComponents({ order, componentList, sliceOfDecl, groupOfSlice, edgesOf, groups, incremental, target, packing, acyclic, mirrorValid, measureMembers, startGroup }: Pick<ReturnType<typeof prepareComponentPacking>, "order" | "groupOfSlice" | "groups" | "incremental" | "packing" | "acyclic" | "mirrorValid" | "startGroup"> & Pick<ReturnType<typeof collectComponentBlockers>, "componentList"> & Pick<ReturnType<typeof buildCarveSlices>, "sliceOfDecl"> & Pick<ReturnType<typeof mergeCoupledComponents>, "edgesOf"> & Pick<ReturnType<typeof readCrateSource>, "target"> & Pick<ReturnType<typeof defineMirrorMeasurement>, "measureMembers">) {
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
}
