import { type UnknownRecord } from "@semio-tech/schema-validator-ts";
import { normalizeRelative, splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import { PLAN_COMMIT_ID, PLAN_HASH, PLAN_OPERATION_ID, type TaxonomyDestinationAncestorPreimage, type TaxonomyEmbeddedTicketRootDisposition, type TaxonomyEmbeddedTicketRootRelocation, type TaxonomyEvidenceRemoval, type TaxonomyPlan, type TaxonomySymlinkTargetEdit, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { dispositionOperationId, generatorPathCompare, parseLeafPreimage, parseMove, parseNoFollowTreeDigest, parseOpaqueDigest, parsePathPreimage, parsePlanViolation, parseReferenceEdit, parseRegeneration, parseRemovalAuthority, planPath, planRecord, planString, planStringArray } from "../🧾️plan-parsing/🟦️.ts";
import { taxonomyPlanDigest } from "../🗃️affected-state/🟦️.ts";

/** 🧿️ Strictly parses a complete schema-v2 taxonomy plan without defaults or v1 compatibility. */
export function parseTaxonomyPlan(value: unknown): TaxonomyPlan {
  const row = planRecord(value, "taxonomy plan", ["schemaVersion", "taxonomySchemaVersion", "baselineCommit", "sourceTreeDigest", "excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved", "expectedAffectedPreStateDigest", "expectedPostStateDigest", "planDigest"], ["scope"]);
  if (row.schemaVersion !== 2 || row.taxonomySchemaVersion !== 7) throw new Error("Taxonomy plan must use schemaVersion 2 and taxonomySchemaVersion 7");
  if (!["excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved"].every((key) => Array.isArray(row[key]))) throw new Error("Taxonomy plan operation and evidence fields must be arrays");
  const destinationAncestorPreimages = (row.destinationAncestorPreimages as unknown[]).map((value, index) => {
    const name = `taxonomy plan destinationAncestorPreimages[${index}]`;
    const entry = planRecord(value, name, ["path", "state"]);
    if (entry.state !== "absent" && entry.state !== "directory") throw new Error(`${name}.state is invalid`);
    return { path: planPath(entry.path, `${name}.path`), state: entry.state } as TaxonomyDestinationAncestorPreimage;
  });
  if (destinationAncestorPreimages.some((entry, index) => index > 0 && generatorPathCompare(requireTaxonomyValue(destinationAncestorPreimages[index - 1],"destinationAncestorPreimages[index - 1]").path, entry.path) >= 0)) throw new Error("Taxonomy plan destinationAncestorPreimages must be unique and bytewise sorted");
  const parseOperationId = (entry: UnknownRecord, name: string): string => planString(entry.operationId, `${name}.operationId`, PLAN_OPERATION_ID);
  const embeddedTicketRootRelocations = (row.embeddedTicketRootRelocations as unknown[]).map((value, index) => {
    const name = `taxonomy plan embeddedTicketRootRelocations[${index}]`;
    const entry = planRecord(value, name, ["operationId", "embeddedTicketRootId", "sourcePath", "destinationPath", "relativeEvidencePath", "preimage", "ownerId", "rationaleRule"], ["fixedContractId"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEmbeddedTicketRootRelocation = { operationId, embeddedTicketRootId: planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), destinationPath: planPath(entry.destinationPath, `${name}.destinationPath`), relativeEvidencePath: planPath(entry.relativeEvidencePath, `${name}.relativeEvidencePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), fixedContractId: entry.fixedContractId === undefined ? undefined : planString(entry.fixedContractId, `${name}.fixedContractId`), ownerId: planString(entry.ownerId, `${name}.ownerId`), rationaleRule: entry.rationaleRule };
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("embedded-ticket-root-relocation", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const evidenceRemovals = (row.evidenceRemovals as unknown[]).map((value, index) => {
    const name = `taxonomy plan evidenceRemovals[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourcePath", "preimage", "authority", "rationaleRule", "ownerId"], ["embeddedTicketRootId"]);
    if (entry.rationaleRule !== "redundant-ticket-evidence-v1" && entry.rationaleRule !== "ticket-important-closed-empty-v1" && entry.rationaleRule !== "ticket-important-exact-empty-residue-v1" && entry.rationaleRule !== "exact-owner-generated-source-retirement-v1" && entry.rationaleRule !== "nested-cargo-generated-source-retirement-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEvidenceRemoval = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), authority: parseRemovalAuthority(entry.authority, `${name}.authority`), embeddedTicketRootId: entry.embeddedTicketRootId === undefined ? undefined : planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    if (result.authority.kind === "nested-cargo-generated-source") {
      if (result.rationaleRule !== "nested-cargo-generated-source-retirement-v1" || result.embeddedTicketRootId !== undefined || canonicalJson(result.preimage) !== canonicalJson(result.authority.sourcePreimage)) throw new Error(name + " nested Cargo generated source is not bound to its frozen preimage");
    } else if (result.authority.kind === "exact-owner-generated-source") {
      if (result.rationaleRule !== "exact-owner-generated-source-retirement-v1" || result.embeddedTicketRootId !== undefined || canonicalJson(result.preimage) !== canonicalJson(result.authority.outputPreimage)) throw new Error(name + " generated source retirement is not byte-and-mode-preserving");
    } else if (result.authority.kind === "byte-and-mode-identical") {
      const removals = result.authority.members.filter((member) => member.disposition === "remove" && member.sourcePath === result.sourcePath && canonicalJson(member.preimage) === canonicalJson(result.preimage));
      if (result.rationaleRule !== "redundant-ticket-evidence-v1" || removals.length !== 1) throw new Error(`${name} is not bound to exactly one redundant evidence member`);
    } else if (result.authority.kind === "owner-manifest-status") {
      if (result.rationaleRule !== "ticket-important-closed-empty-v1" || result.embeddedTicketRootId !== undefined || dirname(result.sourcePath) !== result.authority.ownerPath || result.authority.manifestPath !== `${result.authority.ownerPath}/🎫️ticket.json` || basename(result.sourcePath) !== "📌️important.md" || result.preimage.nodeKind !== "file" || result.preimage.size !== 0) throw new Error(`${name} ticket lifecycle authority is not bound to its exact owner and zero-byte source`);
    } else if (result.authority.kind === "exact-path-mutation") {
      if (result.rationaleRule !== "ticket-important-exact-empty-residue-v1" || result.embeddedTicketRootId !== undefined || result.sourcePath !== result.authority.sourcePath || canonicalJson(result.preimage) !== canonicalJson(result.authority.sourcePreimage)) throw new Error(`${name} exact path authority is not bound to its frozen removal`);
    }
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("evidence-removal", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const symlinkTargetEdits = (row.symlinkTargetEdits as unknown[]).map((value, index) => {
    const name = `taxonomy plan symlinkTargetEdits[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourcePath", "finalPath", "oldTarget", "newTarget", "oldTargetHash", "newTargetHash", "logicalTargetSourcePath", "logicalTargetFinalPath", "logicalTargetPreimage", "windowsLinkType", "sourceTargetDigest", "rationaleRule", "ownerId"]);
    if (entry.rationaleRule !== "repository-local-symlink-target-v2" || (entry.windowsLinkType !== "file" && entry.windowsLinkType !== "dir")) throw new Error(`${name} has invalid literals`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomySymlinkTargetEdit = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), finalPath: planPath(entry.finalPath, `${name}.finalPath`), oldTarget: planString(entry.oldTarget, `${name}.oldTarget`), newTarget: planString(entry.newTarget, `${name}.newTarget`), oldTargetHash: planString(entry.oldTargetHash, `${name}.oldTargetHash`, PLAN_HASH), newTargetHash: planString(entry.newTargetHash, `${name}.newTargetHash`, PLAN_HASH), logicalTargetSourcePath: planPath(entry.logicalTargetSourcePath, `${name}.logicalTargetSourcePath`), logicalTargetFinalPath: planPath(entry.logicalTargetFinalPath, `${name}.logicalTargetFinalPath`), logicalTargetPreimage: parsePathPreimage(entry.logicalTargetPreimage, `${name}.logicalTargetPreimage`), windowsLinkType: entry.windowsLinkType, sourceTargetDigest: planString(entry.sourceTargetDigest, `${name}.sourceTargetDigest`, PLAN_HASH), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    const absoluteTarget = result.oldTarget.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(result.oldTarget) || /^(?:\\\\|\/\/)[^\\/]+[\\/][^\\/]+/u.test(result.oldTarget);
    if (result.oldTarget === "" || result.oldTarget.includes("\u0000") || result.newTarget === "" || result.newTarget.includes("\\") || result.newTarget.startsWith("/") || /^[A-Za-z]:/u.test(result.newTarget)) throw new Error(`${name} target syntax is invalid`);
    if (!absoluteTarget) {
      let resolvedOldTarget: string;
      try { resolvedOldTarget = normalizeRelative(posix.join(posix.dirname(result.sourcePath), result.oldTarget.replaceAll("\\", "/"))); } catch { throw new Error(`${name}.oldTarget does not resolve inside the repository`); }
      if (resolvedOldTarget !== result.logicalTargetSourcePath) throw new Error(`${name}.oldTarget does not resolve to logicalTargetSourcePath`);
    }
    const resolvedNewTarget = posix.normalize(posix.join(posix.dirname(result.finalPath), result.newTarget));
    if (resolvedNewTarget !== result.logicalTargetFinalPath) throw new Error(`${name}.newTarget does not resolve to logicalTargetFinalPath`);
    const targetDigestible = { sourcePath: result.sourcePath, finalPath: result.finalPath, oldTarget: result.oldTarget, newTarget: result.newTarget, logicalTargetSourcePath: result.logicalTargetSourcePath, logicalTargetFinalPath: result.logicalTargetFinalPath, logicalTargetPreimage: result.logicalTargetPreimage };
    if (result.oldTargetHash !== sha256(result.oldTarget) || result.newTargetHash !== sha256(result.newTarget) || result.sourceTargetDigest !== sha256(canonicalJson(targetDigestible))) throw new Error(`${name} target hashes do not match raw targets`);
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("symlink-target-edit", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const embeddedTicketRoots = (row.embeddedTicketRoots as unknown[]).map((value, index) => {
    const name = `taxonomy plan embeddedTicketRoots[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourceMetadataRoot", "sourceTicketRoot", "canonicalTicketRoot", "ticketId", "sourceTreeDigest", "residualTreeDigest", "incomingReferenceDigest", "relocationOperationIds", "removalOperationIds", "rationaleRule"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEmbeddedTicketRootDisposition = { operationId, sourceMetadataRoot: planPath(entry.sourceMetadataRoot, `${name}.sourceMetadataRoot`), sourceTicketRoot: planPath(entry.sourceTicketRoot, `${name}.sourceTicketRoot`), canonicalTicketRoot: planPath(entry.canonicalTicketRoot, `${name}.canonicalTicketRoot`), ticketId: planString(entry.ticketId, `${name}.ticketId`), sourceTreeDigest: parseNoFollowTreeDigest(entry.sourceTreeDigest, `${name}.sourceTreeDigest`), residualTreeDigest: parseNoFollowTreeDigest(entry.residualTreeDigest, `${name}.residualTreeDigest`), incomingReferenceDigest: planString(entry.incomingReferenceDigest, `${name}.incomingReferenceDigest`, PLAN_HASH), relocationOperationIds: planStringArray(entry.relocationOperationIds, `${name}.relocationOperationIds`, PLAN_OPERATION_ID), removalOperationIds: planStringArray(entry.removalOperationIds, `${name}.removalOperationIds`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule };
    const { operationId: _id, relocationOperationIds: _relocations, removalOperationIds: _removals, ...digestible } = result;
    if (operationId !== dispositionOperationId("embedded-ticket-root", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const allOperationIds = [...(row.moves as UnknownRecord[]).map((entry, index) => planString(entry.operationId, `taxonomy plan moves[${index}].operationId`, PLAN_OPERATION_ID)), ...embeddedTicketRoots.map((entry) => entry.operationId), ...embeddedTicketRootRelocations.map((entry) => entry.operationId), ...symlinkTargetEdits.map((entry) => entry.operationId), ...evidenceRemovals.map((entry) => entry.operationId), ...(row.regenerations as UnknownRecord[]).map((entry, index) => planString(entry.id, `taxonomy plan regenerations[${index}].id`, PLAN_OPERATION_ID))];
  if (new Set(allOperationIds).size !== allOperationIds.length) throw new Error("Taxonomy plan operation IDs are not globally unique");
  const relocationIds = new Set(embeddedTicketRootRelocations.map((entry) => entry.operationId));
  const removalIds = new Set(evidenceRemovals.map((entry) => entry.operationId));
  const rootIds = new Set(embeddedTicketRoots.map((entry) => entry.operationId));
  if (embeddedTicketRootRelocations.some((entry) => !rootIds.has(entry.embeddedTicketRootId)) || evidenceRemovals.some((entry) => entry.embeddedTicketRootId !== undefined && !rootIds.has(entry.embeddedTicketRootId))) throw new Error("Embedded ticket disposition references an unknown root");
  for (const root of embeddedTicketRoots) {
    if (!root.sourceTicketRoot.startsWith(`${root.sourceMetadataRoot}/`)) throw new Error(`Embedded ticket root ${root.operationId} source ticket root escapes its metadata root`);
    const ticketSegments = root.canonicalTicketRoot.split("/").slice(-4);
    const expectedTicketId = `${splitLeadingEmoji(ticketSegments[0] ?? "").rest}/${splitLeadingEmoji(ticketSegments[1] ?? "").rest}/${splitLeadingEmoji(ticketSegments[2] ?? "").rest}/${ticketSegments[3] ?? ""}`;
    if (root.ticketId !== expectedTicketId || !/^[0-9]{2}\/[0-9]{2}\/[0-9]{2}\/.+/u.test(root.ticketId)) throw new Error(`Embedded ticket root ${root.operationId} ticketId does not match its canonical root`);
    const orderedChildren = [...root.relocationOperationIds, ...root.removalOperationIds];
    if ([root.relocationOperationIds, root.removalOperationIds].some((ids) => ids.some((id, index) => index > 0 && Buffer.from(requireTaxonomyValue(ids[index - 1],"ids[index - 1]")).compare(Buffer.from(id)) >= 0))) throw new Error(`Embedded ticket root ${root.operationId} child IDs are not unique and bytewise sorted`);
    if (root.relocationOperationIds.some((id) => !relocationIds.has(id)) || root.removalOperationIds.some((id) => !removalIds.has(id))) throw new Error(`Embedded ticket root ${root.operationId} has dangling disposition IDs`);
    const actualChildren = [...embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId), ...evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId)];
    if (new Set(orderedChildren).size !== orderedChildren.length || canonicalJson(orderedChildren.sort(generatorPathCompare)) !== canonicalJson(actualChildren.sort(generatorPathCompare))) throw new Error(`Embedded ticket root ${root.operationId} does not exhaust its child dispositions`);
    if (actualChildren.length !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks || root.sourceTreeDigest.others !== 0 || root.residualTreeDigest.files !== 0 || root.residualTreeDigest.symlinks !== 0 || root.residualTreeDigest.others !== 0) throw new Error(`Embedded ticket root ${root.operationId} tree closure does not equal its child dispositions`);
    for (const relocation of embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId)) if (!relocation.sourcePath.startsWith(`${root.sourceTicketRoot}/`) || relocation.destinationPath !== `${root.canonicalTicketRoot}/${relocation.relativeEvidencePath}` || relocation.relativeEvidencePath !== relocation.sourcePath.slice(root.sourceTicketRoot.length + 1)) throw new Error(`Embedded ticket root ${root.operationId} relocation escapes its frozen roots`);
    for (const removal of evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId)) if (!removal.sourcePath.startsWith(`${root.sourceTicketRoot}/`)) throw new Error(`Embedded ticket root ${root.operationId} removal escapes its source ticket root`);
  }
  if (new Set(embeddedTicketRoots.map((entry) => entry.sourceMetadataRoot)).size !== embeddedTicketRoots.length || new Set(embeddedTicketRoots.map((entry) => entry.sourceTicketRoot)).size !== embeddedTicketRoots.length) throw new Error("Embedded ticket root source roots are not unique");
  const excludedTreeDigests = (row.excludedTreeDigests as unknown[]).map((entry, index) => parseOpaqueDigest(entry, `taxonomy plan excludedTreeDigests[${index}]`));
  const moves = (row.moves as unknown[]).map((entry, index) => parseMove(entry, `taxonomy plan moves[${index}]`));
  for (const edit of symlinkTargetEdits) {
    const owningMoves = moves.filter((move) => move.sourcePath === edit.sourcePath && move.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && owningMoves.length !== 1) throw new Error(`Symlink target edit ${edit.operationId} finalPath is not its exact move destination`);
  }
  const edits = (row.edits as unknown[]).map((entry, index) => parseReferenceEdit(entry, `taxonomy plan edits[${index}]`));
  const regenerations = (row.regenerations as unknown[]).map((entry, index) => parseRegeneration(entry, `taxonomy plan regenerations[${index}]`));
  const unresolved = (row.unresolved as unknown[]).map((entry, index) => parsePlanViolation(entry, `taxonomy plan unresolved[${index}]`));
  const result: TaxonomyPlan = { schemaVersion: 2, taxonomySchemaVersion: 7, baselineCommit: planString(row.baselineCommit, "taxonomy plan baselineCommit", PLAN_COMMIT_ID), scope: row.scope === undefined ? undefined : planPath(row.scope, "taxonomy plan scope"), sourceTreeDigest: planString(row.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH), excludedTreeDigests, moves, embeddedTicketRoots, embeddedTicketRootRelocations, symlinkTargetEdits, evidenceRemovals, destinationAncestorPreimages, edits, regenerations, unresolved, expectedAffectedPreStateDigest: planString(row.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH), expectedPostStateDigest: planString(row.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH), planDigest: planString(row.planDigest, "taxonomy plan planDigest", PLAN_HASH) };
  const requiredAncestorPaths = new Set<string>();
  for (const destination of [...moves.map((entry) => entry.destinationPath), ...embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...symlinkTargetEdits.map((entry) => entry.finalPath), ...regenerations.flatMap((entry) => entry.outputRoots)]) for (let path = posix.dirname(destination); path !== "." && path !== ""; path = posix.dirname(path)) requiredAncestorPaths.add(path);
  if (canonicalJson(destinationAncestorPreimages.map((entry) => entry.path)) !== canonicalJson([...requiredAncestorPaths].sort(generatorPathCompare))) throw new Error("Taxonomy plan destinationAncestorPreimages do not exhaust mutation destination parents");
  planString(result.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH);
  planString(result.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH);
  planString(result.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH);
  planString(result.planDigest, "taxonomy plan planDigest", PLAN_HASH);
  if (result.scope !== undefined) planPath(result.scope, "taxonomy plan scope");
  if (taxonomyPlanDigest(result) !== result.planDigest) throw new Error("Taxonomy plan digest does not match canonical plan bytes");
  return result;
}
