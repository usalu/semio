/** 🖱️ World3dHost interaction domain arguments: the `interactionHover` and `interactionSelect` action arguments a world window sends for a framework interaction domain. */
import { type MergeMode } from "@semio-tech/framework";

//#region 🔖️World3dHost
//#region 🎯️WorldInteractionDomain
/** 🖱️ Mirrors `nodeGraphHoverActionArgs` (`NodeGraph/🟦️.tsx`) for a world window bound to a
 * framework interaction domain (`scene.domainId`), reporting the scene's own
 * `domainGranularityId` — the same shape a `domain_id`-aware `interactionHover` handler expects. */
export function world3dHoverActionArgs(domainId: string, granularity: string, id: string | null | undefined) {
  const targets = id ? [{ granularity, id }] : [];
  return { domainId, channel: "pointer", targets: JSON.stringify(targets) };
}

/** 🖱️ A selection is a SET of topology ids, so the target list this builds is one: the same id
 * can be handed in twice (several rendered instances of one channel, a pick batch the caller did not
 * run through {@link interactionTargetsForInstances}), and a `Select` must be idempotent per id —
 * the host never emits a duplicate target. Live defect 2026-09-12 (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-dedupe-2026-09-12.md`): the guest republished one
 * picked id twice, and only a set on BOTH sides makes that unrepresentable. */
/** 🎯️ `merge` is a `MergeMode` — the ONE vocabulary of `🕹️interaction/🧬️schema/🔣️.json`, put on the
 * wire verbatim. A host must NOT translate it: the framework's `parse_merge_mode` accepts exactly
 * those five words and faults on anything else. */
export function world3dSelectionActionArgs(domainId: string, granularity: string, ids: readonly string[], merge: MergeMode) {
  const targets = [...new Set(ids)].map((id) => ({ granularity, id }));
  return { domainId, targets: JSON.stringify(targets), merge, method: "pick" };
}

/** 🎯️ `interactionSelect` args over already-resolved `{ granularity, id }` targets. */
export function world3dSelectionTargetsActionArgs(domainId: string, targets: readonly { readonly granularity: string; readonly id: string }[], merge: MergeMode, method: "pick" | "rectangle" | "lasso" = "pick") {
  return { domainId, targets: JSON.stringify(targets), merge, method };
}
//#endregion 🎯️WorldInteractionDomain
//#endregion 🔖️World3dHost
