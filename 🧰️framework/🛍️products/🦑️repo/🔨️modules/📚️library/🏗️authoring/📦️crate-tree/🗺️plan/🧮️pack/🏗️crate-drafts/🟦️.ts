/** 🏗️ The crate drafts, their names and their mirrored files of the artifact crate pack planner. */
import { dirname, join, resolve } from "node:path";
import type { PlannedCrate } from "../../🧾️model/🟦️.ts";
import { identifierSet, prunedUse } from "../../../🌳️module-tree/🟦️.ts";
import { type CarveSlice, type ModuleDecl } from "../../../🧱️decl-tree/🟦️.ts";
import { crateRelativeChains, pathHeadNames } from "../../🧭️module-chains/🟦️.ts";
import { type MirrorImport, type MirrorSpec, rebaseManifestDirPaths, rebaseRustIncludes, relativePosix, renderMirrorFiles, renderMirrorRoot } from "../../../📝️render/🟦️.ts";
import { lcaDirectory } from "../../🧰️tree-helpers/🟦️.ts";
import { measureFits, measureRustCrate, readMeasureSources } from "../../📏️measure/🟦️.ts";
import { crateIdent, crateIdentPlaceholder, facetEmoji, facetSlug, isMutationCrateLabel, mutationCrateLabel, mutationFamilyOf } from "../../🏷️naming/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { classifyPackageSource } from "../../../../../🔍️discovery/🏷️package-source-classification/🟦️.ts";
import type { buildCarveSlices, indexRootItems, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { collectSliceInfos, defineMirrorMeasurement } from "../🧲️slice-ownership/🟦️.ts";
import type { mergeCoupledComponents } from "../🧵️weave-merge-and-blockers/🟦️.ts";
import type { prepareComponentPacking } from "../🗜️packing/🟦️.ts";
import type { findMovedRootItems, hostMovedUnits } from "../🚚️moved-items/🟦️.ts";

/** 🏷 Drafts one crate per group and labels the drafts. */
export function draftCrates({ groups, carvable, unitsOf }: Pick<ReturnType<typeof prepareComponentPacking>, "groups"> & Pick<ReturnType<typeof buildCarveSlices>, "carvable"> & Pick<ReturnType<typeof defineMirrorMeasurement>, "unitsOf">) {
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
    const familyOf = (slice: CarveSlice): string | null => mutationFamilyOf(slice.attach[0]?.eff ?? []);
    const familySlices = slices.filter((slice) => familyOf(slice) !== null);
    const dominant = mutationCrateLabel(slices.map(familyOf)) ?? labelOf(biggest.attach[0]!);
    const firstFile = (members: readonly ModuleDecl[]): string | null => [...members].filter((decl) => decl.file !== null).sort((a, b) => a.raw.length - b.raw.length)[0]?.file ?? null;
    const anchorFile = firstFile(biggest.decls) ?? firstFile(attach) ?? null;
    const anchor = attach[0]!;
    const familyDir = isMutationCrateLabel(dominant) && familySlices.length > 1 ? lcaDirectory(familySlices.flatMap((slice) => slice.decls.map((decl) => decl.file ?? "")).filter(Boolean)) : "";
    const dirOf = familyDir || (anchorFile ? dirname(anchorFile) : lcaDirectory(slices.flatMap((slice) => slice.decls.map((decl) => decl.file ?? "")).filter(Boolean)));
    const named = biggest.attach[0]!.eff;
    return { i, slices, attach, label: dominant, dir: dirOf, anchor, eff: named };
  });
  const sharedLabels = new Map<string, number>();
  for (const entry of draft) sharedLabels.set(entry.label, (sharedLabels.get(entry.label) ?? 0) + 1);
  const labelSet = new Set(draft.map((entry) => entry.label));
  for (const entry of draft) {
    if ((sharedLabels.get(entry.label) ?? 0) < 2 || isMutationCrateLabel(entry.label) || entry.eff[0] === "standards" || entry.eff.length < 2) continue;
    const last = facetSlug(entry.eff.at(-1)!.replaceAll("_", "-"));
    const pair = facetSlug(entry.eff.slice(-2).join("-").replaceAll("_", "-"));
    entry.label = last && !labelSet.has(last) ? last : pair || entry.label;
    labelSet.add(entry.label);
  }
  return { draft };
}

/** 📛 Names every draft crate, picks its directory and plans its dependencies. */
export function nameAndPlaceCrates({ repoRoot, packageDirRel, draft, lib, crateDeps, groups, edgesOf, hostedAt, crateOfSlice, infoOf, sliceOfDecl, referencing, rootNames, rootSource, cfgAttributeLines, movedPlaced, gatedUses, shims, innerAttributes }: { readonly repoRoot: string; readonly packageDirRel: string } & Pick<ReturnType<typeof draftCrates>, "draft"> & Pick<ReturnType<typeof readCrateSource>, "lib" | "rootSource"> & Pick<ReturnType<typeof findMovedRootItems>, "crateDeps" | "crateOfSlice" | "referencing"> & Pick<ReturnType<typeof prepareComponentPacking>, "groups"> & Pick<ReturnType<typeof mergeCoupledComponents>, "edgesOf"> & Pick<ReturnType<typeof hostMovedUnits>, "hostedAt" | "movedPlaced" | "innerAttributes"> & Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infoOf"> & Pick<ReturnType<typeof buildCarveSlices>, "sliceOfDecl" | "shims"> & Pick<ReturnType<typeof indexRootItems>, "rootNames" | "cfgAttributeLines" | "gatedUses">) {
  const usedNames = new Set<string>();
  const usedDirs = new Set<string>([dirname(dirname(resolve(repoRoot, packageDirRel)))]);
  const crates: PlannedCrate[] = [];
  for (const entry of draft) {
    let name = `${lib.name}-${entry.label}`;
    for (let n = 2; usedNames.has(name); n += 1) name = `${lib.name}-${entry.label}-${n}`;
    usedNames.add(name);
    let dir = entry.dir;
    for (const decl of entry.attach) { if (!usedDirs.has(dir)) break; if (decl.file && !usedDirs.has(dirname(decl.file))) dir = dirname(decl.file); }
    if (usedDirs.has(dir)) dir = join(dir, `🧩️${entry.label}`);
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
    for (const tested of entry.slices.flatMap((slice) => slice.attach).filter((decl) => decl.testOnly && decl.parentId)) {
      const hostSlice = sliceOfDecl.get(tested.parentId!);
      const hostCrate = hostSlice ? crateOfSlice.get(hostSlice.id) : undefined;
      const host = hostSlice?.decls.find((decl) => decl.id === tested.parentId);
      if (hostCrate === undefined || hostCrate === entry.i || !host || host.testOnly) continue;
      const key = host.eff.join("::");
      if (attachPaths.includes(key)) continue;
      if (imports.some((existing) => existing.glob === host.isComponent && existing.path.join("::") === key)) continue;
      imports.push({ path: host.eff, crateIdent: String(hostCrate), visibility: host.node.visibility === "private" ? "pub" : host.node.visibility, order: host.node.start, glob: host.isComponent });
    }
    const pruned = imports.filter((entryImport) => !imports.some((other) => other !== entryImport && !other.glob && other.crateIdent === entryImport.crateIdent && other.path.length < entryImport.path.length && other.path.every((segment, k) => segment === entryImport.path[k])));
    const nestedGlobs: MirrorImport[] = [];
    for (const globImport of [...pruned]) {
      const depPaths = groups[Number(globImport.crateIdent)]!.infos.flatMap((info) => info.slice.decls.filter((decl) => !decl.testOnly).map((decl) => decl.eff));
      const deeperPaths: (readonly string[])[] = [...entry.attach.map((decl) => decl.eff), ...pruned.filter((other) => !other.glob && other.crateIdent !== globImport.crateIdent).map((other) => other.path)];
      for (const ownerPath of deeperPaths) {
        if (ownerPath.length <= globImport.path.length + 1 || !globImport.path.every((segment, k) => segment === ownerPath[k])) continue;
        for (let k = globImport.path.length + 1; k < ownerPath.length; k += 1) {
          const through = ownerPath.slice(0, k);
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
    const copiedUses = [...new Set([...copiedNames].map((n) => { const item = rootNames.get(n)!; const pruned = prunedUse(rootSource.slice(item.declStart, item.end), [n]); return pruned ? `${cfgAttributeLines(item)}${pruned}` : ""; }).filter(Boolean))];
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
  return { crates };
}

/** 🪞 Renders the mirrored files of every planned crate and resolves their moved item appends. */
export async function renderCrateMirrors({ crates, repoRoot, unitsOf, hostedAt, declTree, problems, gatedUses, innerAttributes, wrapperNodes, gates, cutOverlay, readText, manifestPath, lib, baseOverlay, externRows, rootSource, hard }: Pick<ReturnType<typeof nameAndPlaceCrates>, "crates"> & { readonly repoRoot: string } & Pick<ReturnType<typeof defineMirrorMeasurement>, "unitsOf" | "cutOverlay"> & Pick<ReturnType<typeof hostMovedUnits>, "hostedAt" | "innerAttributes"> & Pick<ReturnType<typeof buildCarveSlices>, "declTree" | "wrapperNodes" | "gates"> & Pick<ReturnType<typeof readCrateSource>, "problems" | "readText" | "manifestPath" | "lib" | "baseOverlay" | "rootSource" | "hard"> & Pick<ReturnType<typeof indexRootItems>, "gatedUses" | "externRows">) {
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
    const globComponent = (decl: ModuleDecl): boolean => {
      if (decl.isComponent) return true;
      const parent = decl.parentId === null ? undefined : declTree.decls.get(decl.parentId);
      return parent !== undefined && parent.node.items.some((item) => item.kind === "use" && new RegExp(`^pub\\s+use\\s+${decl.node.name}::\\*\\s*;`, "u").test(parent.declText.slice(item.declStart, item.end)));
    };
    const anchorChoices = (decls: readonly ModuleDecl[]): ModuleDecl[] => decls.filter((decl) => decl.file !== null && !decl.testOnly).sort((a, b) => Number(b.isComponent) - Number(a.isComponent) || a.eff.length - b.eff.length);
    const anchorDecl = hostedHere.length ? anchorChoices(crate.attach)[0] ?? anchorChoices(crate.slices.flatMap((slice) => slice.decls))[0] ?? null : null;
    const anchorPath = anchorDecl && globComponent(anchorDecl) && !anchorDecl.isComponent ? anchorDecl.eff.slice(0, -1) : anchorDecl?.eff ?? [];
    if (hostedHere.length && !anchorDecl) problems.push(`crate ${crate.name} owns the moved root items but mounts no file to host them`);
    const destinationImport = anchorDecl ? hostedHere.flatMap((unit) => gatedUses(`crate::${anchorPath.join("::")}::${unit.module}`, [...unit.exports].filter((name) => unit.names.has(name) || !crate.itemImportLines.some((line) => new RegExp(`\\b${name}\\b`, "u").test(line))))) : [];
    const importLines = [...itemImportLines, ...destinationImport];
    const specBase: MirrorSpec = { doc: "//! 🪞️ planning mirror", newRootDir: packageDirAbs, innerAttributes, externLines: [], movedItemsText: "", itemImports: importLines, owned, imported: imports, wrapperNodes, gates };
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
  return { identOf, resolved, movedAppends };
}
