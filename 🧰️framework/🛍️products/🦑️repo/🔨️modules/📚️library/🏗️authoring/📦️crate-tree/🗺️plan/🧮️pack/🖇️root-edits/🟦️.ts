/** 🖇️ The root edits and the final plan of the artifact crate pack planner. */
import type { ArtifactCratePlan, DeclCut, RootEdit, WovenExport } from "../../🧾️model/🟦️.ts";
import { relativePosix } from "../../../📝️render/🟦️.ts";
import { weaveRootEdits } from "../../🪢️weave/🟦️.ts";
import type { buildCarveSlices, readCrateSource } from "../📥️source-and-slices/🟦️.ts";
import type { collectSliceInfos } from "../🧲️slice-ownership/🟦️.ts";
import type { collectComponentBlockers, planRootItemWeaving } from "../🧵️weave-merge-and-blockers/🟦️.ts";
import type { prepareComponentPacking } from "../🗜️packing/🟦️.ts";
import type { findMovedRootItems, hostMovedUnits } from "../🚚️moved-items/🟦️.ts";
import type { nameAndPlaceCrates, renderCrateMirrors } from "../🏗️crate-drafts/🟦️.ts";

/** ✂ Collects the root edits, file cuts and woven exports and assembles the crate plan. */
export function collectRootEditsAndPlan({ identOf, crateOfSlice, carvable, infos, resolved, sliceOfDecl, built, lib, gates, droppedTests, woven, movedPlaced, problems, wrapperNodes, repoRoot, artifactDir, packageDirRel, tree, crates, movedAppends, keptModules, transform, family, folded, blockers, packing }: Pick<Awaited<ReturnType<typeof renderCrateMirrors>>, "identOf" | "resolved" | "movedAppends"> & Pick<ReturnType<typeof findMovedRootItems>, "crateOfSlice"> & Pick<ReturnType<typeof buildCarveSlices>, "carvable" | "sliceOfDecl" | "built" | "gates" | "wrapperNodes" | "keptModules" | "family"> & Pick<Awaited<ReturnType<typeof collectSliceInfos>>, "infos"> & Pick<ReturnType<typeof readCrateSource>, "lib" | "problems" | "artifactDir" | "tree" | "transform" | "folded"> & Pick<ReturnType<typeof planRootItemWeaving>, "droppedTests" | "woven"> & Pick<ReturnType<typeof hostMovedUnits>, "movedPlaced"> & { readonly repoRoot: string; readonly packageDirRel: string } & Pick<ReturnType<typeof nameAndPlaceCrates>, "crates"> & Pick<ReturnType<typeof collectComponentBlockers>, "blockers"> & Pick<ReturnType<typeof prepareComponentPacking>, "packing">) {
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
      const text = `${(gates.get(decl.eff.join("::")) ?? []).map((gate) => `${gate}\n`).join("")}${visibility} use ${crateIdentName}::${decl.eff.join("::")};`;
      if (decl.declFile === lib.entry) rootEdits.push({ start: decl.node.start, end: decl.node.end, text, anchor: { eff: decl.eff, crate: crateIdentName, visibility: visibility === "private" ? "pub" : visibility.startsWith("pub(") ? "pub" : visibility, component: false } });
      else if (droppedTests.has(decl.id) && !parentIsNamespace) fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text: "" });
      else if (woven.has(decl.id) && !parentIsNamespace) {
        fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text: "" });
        if (decl.node.visibility !== "private") weave.push({ eff: decl.eff, crate: crateIdentName });
      } else fileCuts.push({ file: decl.declFile, start: decl.node.start, end: decl.node.end, text, anchor: { eff: decl.eff, crate: crateIdentName, visibility: visibility === "private" || visibility.startsWith("pub(") ? "pub" : visibility, component: false } });
    }
  }
  for (const edit of rootEdits) {
    if (edit.end === edit.start) continue;
    const hit = movedPlaced.flatMap((unit) => unit.blocks).find((block) => edit.start < block.end && block.start < edit.end);
    if (hit) problems.push(`the carved root module ${edit.anchor?.eff.join("::") ?? `at offset ${edit.start}`} overlaps a root region that moves into artifact_items; split the region first`);
  }
  if (weave.length) problems.push(...weaveRootEdits(rootEdits, wrapperNodes, weave, placement, fileCuts, gates).problems);
  const plan: ArtifactCratePlan = { repoRoot, artifactDir, artifactCrate: lib.name, packageDir: packageDirRel, rootFile: relativePosix(repoRoot, lib.entry), tree, crates: resolved, wrapperNodes, gates, rootEdits, weave, placement, fileCuts, movedUnits: movedPlaced.map((unit) => ({ names: [...unit.exports].sort(), blocks: unit.blocks, to: crates[unit.host]!.id })), movedAppends, keptModules, overlay: { edits: new Map([...(transform?.edits ?? []), ...(family?.edits ?? [])]), creates: new Map([...(transform?.creates ?? []), ...(family?.creates ?? [])]), deletes: new Set([...folded.deletes, ...(transform?.deletes ?? [])]), notes: [...folded.notes, ...(transform?.notes ?? []), ...(family?.notes ?? [])] }, rootMeasure: null, problems, blockers, packing };
  return { plan };
}
