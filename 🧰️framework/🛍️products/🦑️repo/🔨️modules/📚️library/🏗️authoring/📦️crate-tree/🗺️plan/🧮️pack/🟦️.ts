import type { ArtifactCratePlan, PlanOptions } from "../🧾️model/🟦️.ts";
import { buildCarveSlices, indexRootItems, readCrateSource } from "./📥️source-and-slices/🟦️.ts";
import { collectSliceInfos, defineChainOwnership, defineMirrorMeasurement } from "./🧲️slice-ownership/🟦️.ts";
import { closeRootItemReferences, resolveSliceEdges } from "./🔗️edges-and-closures/🟦️.ts";
import { collectComponentBlockers, mergeCoupledComponents, planRootItemWeaving } from "./🧵️weave-merge-and-blockers/🟦️.ts";
import { packComponents, prepareComponentPacking } from "./🗜️packing/🟦️.ts";
import { buildMovedUnits, findMovedRootItems, hostMovedUnits, pieceMovedRootItems } from "./🚚️moved-items/🟦️.ts";
import { draftCrates, nameAndPlaceCrates, renderCrateMirrors } from "./🏗️crate-drafts/🟦️.ts";
import { collectRootEditsAndPlan } from "./🖇️root-edits/🟦️.ts";
import { crateUnchanged } from "../../♻️recarve-delta/🟦️.ts";
import { readCarvedLayout } from "../../🧱️carved-layout/🟦️.ts";

/** 🧭️ Plans the crate split of one artifact; when the plain split cannot meet the budget and the artifact has mutation aggregates, the plan is retried with the mutation leaves converted to leaves generic over their aggregate and the plan with fewer problems wins. An artifact that is already carved is read back first: its carved crates keep their names, directories and declarations, and only the declarations no carved crate owns are packed on top of them (no test rewrites and no mutation conversion run again). */
export async function planArtifactCrates(repoRoot: string, packageDirRel: string, requested: PlanOptions = {}): Promise<ArtifactCratePlan> {
  const carved = requested.carved === undefined ? readCarvedLayout(repoRoot, packageDirRel) : requested.carved;
  const options: PlanOptions = carved ? { ...requested, carved, tests: undefined, mutationFamilies: "off" } : { ...requested, carved: null };
  const mode = options.mutationFamilies ?? "auto";
  if (mode === "on") return planArtifactCratesOnce(repoRoot, packageDirRel, options, true);
  let plain = relaxedBudgets(await planArtifactCratesOnce(repoRoot, packageDirRel, options, false));
  const over = plain.problems.flatMap((problem) => /^crate (\S+) exceeds the hard budget/u.exec(problem)?.[1] ?? []).filter((name) => plain.carved?.crates.some((crate) => crate.name === name));
  if (over.length) plain = relaxedBudgets(await planArtifactCratesOnce(repoRoot, packageDirRel, { ...options, tight: over }, false));
  if (mode === "off" || !plain.problems.some((problem) => problem.includes("exceeds the hard budget"))) return plain;
  const converted = await planArtifactCratesOnce(repoRoot, packageDirRel, options, true).catch(() => null);
  if (converted === null) return plain;
  return converted.overlay.notes.some((note) => note.startsWith("mutation aggregate")) && converted.problems.length < plain.problems.length ? converted : plain;
}
/** 🪙️ Re-carve only: a carved crate that keeps what it owns was inside the hard budget when it was carved, so a budget that the re-plan measures a file or two higher (the test rewrites of the first carve do not run again) is reported as a note instead of a problem; crates that changed stay judged. */
function relaxedBudgets(plan: ArtifactCratePlan): ArtifactCratePlan {
  if (!plan.carved) return plan;
  const relaxed = plan.crates.filter((crate) => crateUnchanged(plan, crate)).map((crate) => `crate ${crate.name} exceeds the hard budget`);
  const moved = plan.problems.filter((problem) => relaxed.some((prefix) => problem.startsWith(prefix)));
  return moved.length ? { ...plan, problems: plan.problems.filter((problem) => !moved.includes(problem)), overlay: { ...plan.overlay, notes: [...plan.overlay.notes, ...moved.map((problem) => `${problem} (unchanged carved crate)`)] } } : plan;
}

/** 🗺️ Plans the path-preserving carve of one artifact into crates cut along declarations, honouring coherence and the budget. */
async function planArtifactCratesOnce(repoRoot: string, packageDirRel: string, options: PlanOptions, convert: boolean): Promise<ArtifactCratePlan> {
  const { lib, rootSource, readText, baseOverlay, tree, manifestPath, problems, target, hard, artifactDir, transform, folded } = readCrateSource({ options, repoRoot, packageDirRel });
  const { declTree, built, sliceOfDecl, wrapperNodes, gates, carvable, shims, unnestShim, isShimDecl, keptModules, family } = buildCarveSlices({ convert, lib, rootSource, readText, baseOverlay, options, tree });
  const { measureRoot, rootModuleNames, rootNames, rootUseTargets, itemByName, regionOf, qualifyRootHeads, cfgAttributeLines, gatedUses, externRows } = indexRootItems({ rootSource, tree, declTree, manifestPath });
  const { measureMembers, cutOverlay, effWrappersSpec, unitsOf } = defineMirrorMeasurement({ built, declTree, sliceOfDecl, readText, measureRoot, wrapperNodes, gates, baseOverlay, repoRoot });
  const { infos, coherenceType, traitNames, typeDefs, infoOf } = await collectSliceInfos({ rootSource, declTree, readText, carvable, measureMembers });
  const { ownerOfChain, noteWhy, restrictedTarget, sourcesOf, ownerOfReachedChain, membersOf, underPath, widenings } = defineChainOwnership({ infos, declTree, readText, measureRoot, baseOverlay, cutOverlay, effWrappersSpec, repoRoot });
  const { chainOwners } = resolveSliceEdges({ shims, ownerOfChain, noteWhy, restrictedTarget, coherenceType, rootModuleNames, declTree, infos, sourcesOf, unnestShim, ownerOfReachedChain, membersOf, rootNames, rootUseTargets, isShimDecl, underPath, traitNames, typeDefs, problems });
  closeRootItemReferences({ itemByName, regionOf, tree, qualifyRootHeads, rootSource, unnestShim, chainOwners, infos, infoOf, sourcesOf, traitNames, typeDefs, problems, coherenceType });
  const { droppedTests, woven } = planRootItemWeaving({ tree, qualifyRootHeads, rootSource, unnestShim, shims, readText, lib, underPath, infos, infoOf, sliceOfDecl, declTree, options });
  const { find, mergeCauses, preGroup, edgesOf } = mergeCoupledComponents({ infos, coherenceType, infoOf, options });
  const { componentList, blockers } = collectComponentBlockers({ infos, find, target, mergeCauses, infoOf, preGroup, edgesOf });
  const { order, groupOfSlice, groups, incremental, packing, acyclic, mirrorValid, startGroup, pinned, settle } = prepareComponentPacking({ componentList, edgesOf, repoRoot, measureRoot, options });
  await packComponents({ order, componentList, sliceOfDecl, groupOfSlice, edgesOf, groups, incremental, target, packing, acyclic, mirrorValid, measureMembers, startGroup, pinned, settle });
  const { movedItems, movedNames, crateDeps, referenced, crateOfSlice, referencing } = findMovedRootItems({ groups, edgesOf, rootNames, tree, itemByName, regionOf, rootSource });
  const { unitPieces, pieceItems, pieceOfName, unitOfPiece, pieceDeps } = pieceMovedRootItems({ movedItems, regionOf, rootSource });
  const { movedUnits } = buildMovedUnits({ unitPieces, pieceItems, regionOf, rootSource, pieceOfName, unitOfPiece, tree, cfgAttributeLines, pieceDeps, qualifyRootHeads, movedNames, shims, ownerOfChain });
  const { hostedAt, movedPlaced, innerAttributes } = hostMovedUnits({ crateDeps, tree, groups, movedUnits, referenced, crateOfSlice, referencing, problems });
  const { draft } = draftCrates({ groups, carvable, unitsOf });
  const { crates } = nameAndPlaceCrates({ repoRoot, packageDirRel, draft, lib, crateDeps, groups, edgesOf, hostedAt, crateOfSlice, infoOf, sliceOfDecl, referencing, rootNames, rootSource, cfgAttributeLines, movedPlaced, gatedUses, shims, innerAttributes });
  const { identOf, resolved, movedAppends } = await renderCrateMirrors({ crates, repoRoot, unitsOf, hostedAt, declTree, problems, gatedUses, innerAttributes, wrapperNodes, gates, cutOverlay, readText, manifestPath, lib, baseOverlay, externRows, rootSource, hard });
  const { plan } = collectRootEditsAndPlan({ identOf, crateOfSlice, carvable, infos, resolved, sliceOfDecl, built, lib, gates, droppedTests, woven, movedPlaced, problems, wrapperNodes, repoRoot, artifactDir, packageDirRel, tree, crates, movedAppends, keptModules, transform, family, folded, blockers, packing, carved: options.carved ?? null, widen: options.carved ? widenings : new Map() });
  return plan;
}
