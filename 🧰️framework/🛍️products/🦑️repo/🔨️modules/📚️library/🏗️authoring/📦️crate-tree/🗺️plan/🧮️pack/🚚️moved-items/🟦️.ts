/** 🚚️ The root items that move into crates and their hosting in the artifact crate pack planner. */
import { identifierSet, prunedUse, referencedItemNames, rustItemStartWithComments, type RustSourceItem, useLeafNames } from "../../../🌳️module-tree/🟦️.ts";
import { aliasChains, crateRelativeChains, stronglyConnected } from "../../🧭️module-chains/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { defineChainOwnership } from "../🧲️slice-ownership/🟦️.ts";
import type { mergeCoupledComponents } from "../🧵️weave-merge-and-blockers/🟦️.ts";
import type { prepareComponentPacking } from "../🗜️packing/🟦️.ts";
import type { MovedUnit } from "../📚️pack-types/🟦️.ts";

/** 🔎 Finds the root items that move into the crates the groups reference. */
export function findMovedRootItems({ groups, edgesOf, rootNames, tree, itemByName, regionOf, rootSource }: Pick<ReturnType<typeof prepareComponentPacking>, "groups"> & Pick<ReturnType<typeof mergeCoupledComponents>, "edgesOf"> & Pick<ReturnType<typeof indexRootItems>, "rootNames" | "itemByName" | "regionOf"> & Pick<ReturnType<typeof readCrateSource>, "tree" | "rootSource">) {
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
        for (const identifier of referencedItemNames(rootSource.slice(member.declStart, member.end))) {
          const other = itemByName.get(identifier);
          if (other && !movedItems.has(other) && !movedNames.has(identifier) && identifier !== member.name) { movedNames.add(identifier); changed = true; }
        }
        changed = true;
      }
    }
  }
  return { movedItems, movedNames, crateDeps, referenced, crateOfSlice, referencing };
}

/** 🧱 Cuts the moved items into pieces and computes their dependencies. */
export function pieceMovedRootItems({ movedItems, regionOf, rootSource }: Pick<ReturnType<typeof findMovedRootItems>, "movedItems"> & Pick<ReturnType<typeof indexRootItems>, "regionOf"> & Pick<ReturnType<typeof readCrateSource>, "rootSource">) {
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
    for (const item of items) for (const identifier of referencedItemNames(rootSource.slice(item.declStart, item.end))) {
      const other = pieceOfName.get(identifier);
      if (other && other !== key) deps.add(other);
    }
    pieceDeps.set(key, deps);
  }
  const unitPieces = stronglyConnected([...pieceItems.keys()], pieceDeps);
  const unitOfPiece = new Map<string, number>();
  unitPieces.forEach((keys, index) => keys.forEach((key) => unitOfPiece.set(key, index)));
  return { unitPieces, pieceItems, pieceOfName, unitOfPiece, pieceDeps };
}

/** 📋 Builds the moved units from the strongly connected pieces. */
export function buildMovedUnits({ unitPieces, pieceItems, regionOf, rootSource, pieceOfName, unitOfPiece, tree, cfgAttributeLines, pieceDeps, qualifyRootHeads, movedNames, shims, ownerOfChain }: Pick<ReturnType<typeof pieceMovedRootItems>, "unitPieces" | "pieceItems" | "pieceOfName" | "unitOfPiece" | "pieceDeps"> & Pick<ReturnType<typeof indexRootItems>, "regionOf" | "cfgAttributeLines" | "qualifyRootHeads"> & Pick<ReturnType<typeof readCrateSource>, "rootSource" | "tree"> & Pick<ReturnType<typeof findMovedRootItems>, "movedNames"> & Pick<ReturnType<typeof buildCarveSlices>, "shims"> & Pick<ReturnType<typeof defineChainOwnership>, "ownerOfChain">) {
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
      if (leaves.length) useLines.push(`${cfgAttributeLines(item)}${prunedUse(statement, leaves)}`);
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
  return { movedUnits };
}

/** 🏠 Assigns every moved unit a hosting crate. */
export function hostMovedUnits({ crateDeps, tree, groups, movedUnits, referenced, crateOfSlice, referencing, problems }: Pick<ReturnType<typeof findMovedRootItems>, "crateDeps" | "referenced" | "crateOfSlice" | "referencing"> & Pick<ReturnType<typeof readCrateSource>, "tree" | "problems"> & Pick<ReturnType<typeof prepareComponentPacking>, "groups"> & Pick<ReturnType<typeof buildMovedUnits>, "movedUnits">) {
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
    if (footprint.size) { unit.host = -1; continue; }
    let missing = false;
    for (const dep of unit.dependsOn) { const host = movedUnits[dep]!.host; if (host < 0) missing = true; else footprint.add(host); }
    if (missing || !assign(unit, new Set<number>(), footprint)) unit.host = -1;
  }
  const movedPlaced = movedUnits.filter((unit) => unit.host >= 0);
  const hostedAt = (i: number): MovedUnit[] => movedUnits.filter((unit) => unit.host === i);
  return { hostedAt, movedPlaced, innerAttributes };
}
