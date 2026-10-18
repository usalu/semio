import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import type { ReferenceEdit, TaxonomyMove, TaxonomyPlan } from "../📜️plan-contracts/🟦️.ts";
import type { MutableJournalRecord } from "../💾️durable-journal/🟦️.ts";
import { expectedBackupRecord } from "../🩹️recovery-authorities/🟦️.ts";
import { applyEditsToContent } from "../✂️reference-edits/🟦️.ts";
import { assertLeafPreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { referenceEditResult } from "../🛟️recovery/🟦️.ts";

/** 🪞️ Projects only immutable move/edit preimages during generated-source retirement preflight. */
export function removalReferenceProjection(repoRoot: string, plan: TaxonomyPlan, journal?: MutableJournalRecord): (path: string, bytes: Buffer, mode: number) => Readonly<{ path: string; bytes: Buffer }> {
  const sourceMoves = new Map(plan.moves.map((move) => [move.sourcePath, move])), destinationMoves = new Map(plan.moves.map((move) => [move.destinationPath, move]));
  const groups = new Map<string, ReferenceEdit[]>();
  for (const edit of plan.edits) groups.set(edit.path, [...(groups.get(edit.path) ?? []), edit]);
  for (const [path, edits] of groups) {
    const preimage = edits[0]!.preimage, move = destinationMoves.get(path);
    if (preimage.nodeKind !== "file" || edits.some((edit) => canonicalJson(edit.preimage) !== canonicalJson(preimage)) || move && (path !== move.destinationPath || canonicalJson(move.sourcePreimage) !== canonicalJson(preimage))) throw new Error(`Retirement reference has conflicting frozen preimages: ${path}`);
    const spans = edits.map((edit) => ({ start: Number(edit.structuredLocation.match(/@(\d+)$/u)?.[1] ?? NaN), length: edit.oldValue.length })).sort((left, right) => left.start - right.start);
    if (spans.some((span, index) => !Number.isSafeInteger(span.start) || span.start < 0 || span.length === 0 || index > 0 && span.start < spans[index - 1]!.start + spans[index - 1]!.length)) throw new Error(`Retirement reference has malformed or overlapping planned edits: ${path}`);
  }
  return (path, bytes, mode) => {
    const candidates = [...new Set([sourceMoves.get(path), destinationMoves.get(path)].filter((move): move is TaxonomyMove => Boolean(move)))];
    const active = candidates.filter((move) => path === (journal?.installedMoveIds.includes(move.operationId) ? move.destinationPath : journal?.stagedMoveIds.includes(move.operationId) ? null : move.sourcePath));
    if (candidates.length && active.length !== 1) throw new Error(`Retirement reference move phase differs from its journal: ${path}`);
    const move = active[0], finalPath = move?.destinationPath ?? path, edits = groups.get(finalPath) ?? [];
    if (!move && edits.length === 0) return { path, bytes };
    const preimage = edits[0]?.preimage ?? move!.sourcePreimage;
    if (preimage.nodeKind !== "file") throw new Error(`Retirement reference requires a frozen regular file: ${path}`);
    if (journal?.appliedEditPaths.includes(finalPath)) {
      const backup = journal.backups[finalPath];
      if (backup?.kind !== "file" || canonicalJson(backup) !== canonicalJson(expectedBackupRecord(finalPath, preimage))) throw new Error(`Retirement reference lacks its exact applied-edit backup: ${path}`);
      assertLeafPreimage(repoRoot, `${journal.backupRoot}/${backup.backupPath}`, preimage);
      const result = referenceEditResult(repoRoot, plan, journal, finalPath).bytes;
      if (path !== finalPath || mode !== preimage.mode || !bytes.equals(result)) throw new Error(`Retirement reference applied result drift: ${path}`);
      return { path, bytes };
    }
    if (mode !== preimage.mode || bytes.byteLength !== preimage.size || sha256(bytes) !== preimage.contentHash) throw new Error(`Retirement reference source preimage drift: ${path}`);
    return { path: finalPath, bytes: Buffer.from(applyEditsToContent(bytes.toString("utf8"), edits)) };
  };
}
