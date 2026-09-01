//#region 🏠️AuthoredLocalInteraction
import type { LocalInteractionDomainPatch, LocalInteractionState } from "../../../📡️replication/📡️wire/🏠️local-interaction/🟦️.ts";

export type TutorialLocalInteractionChange = { readonly kind: "localInteractionDomain"; readonly domainId: string; readonly patch: LocalInteractionDomainPatch };

function own<T>(map: Readonly<Record<string, T>>, domain: string): T | null { return Object.hasOwn(map, domain) ? map[domain]! : null; }
function patchField<T>(map: Readonly<Record<string, T>>, domain: string, value: T | null): Readonly<Record<string, T>> {
  if (own(map, domain) === value) return map;
  if (value !== null) return { ...map, [domain]: value };
  const next = { ...map }; delete next[domain]; return next;
}
function selectionEquals(left: LocalInteractionDomainPatch["selection"], right: LocalInteractionDomainPatch["selection"]): boolean {
  return left === right || left !== null && right !== null && left.granularity === right.granularity && left.anchorId === right.anchorId && left.ids.length === right.ids.length && left.ids.every((id, index) => id === right.ids[index]);
}

/** 🧊️ Composes validated authored content only; it neither captures live state nor publishes a restore. */
export function applyTutorialLocalInteractionCold(before: LocalInteractionState, change: TutorialLocalInteractionChange): LocalInteractionState {
  return { selection: patchField(before.selection, change.domainId, change.patch.selection), activeMode: patchField(before.activeMode, change.domainId, change.patch.activeMode), activeGranularity: patchField(before.activeGranularity, change.domainId, change.patch.activeGranularity) };
}

/** 🧊️ Exact cold authored diff; collection traversal is not a retained recording or publication step. */
export function diffTutorialLocalInteractionCold(before: LocalInteractionState, after: LocalInteractionState): readonly TutorialLocalInteractionChange[] {
  const domains = new Set([...Object.keys(before.selection), ...Object.keys(after.selection), ...Object.keys(before.activeMode), ...Object.keys(after.activeMode), ...Object.keys(before.activeGranularity), ...Object.keys(after.activeGranularity)]);
  const changes: TutorialLocalInteractionChange[] = [];
  for (const domainId of [...domains].sort()) {
    const selection = own(after.selection, domainId); const activeMode = own(after.activeMode, domainId); const activeGranularity = own(after.activeGranularity, domainId);
    if (!selectionEquals(own(before.selection, domainId), selection) || own(before.activeMode, domainId) !== activeMode || own(before.activeGranularity, domainId) !== activeGranularity) changes.push({ kind: "localInteractionDomain", domainId, patch: { selection, activeMode, activeGranularity } });
  }
  return changes;
}
//#endregion 🏠️AuthoredLocalInteraction
