/** 🌱️ Space artifact creation polling (`🔣️.json`): the browser follows a creation for as long as the hub answers — a slow hub
 * is waited for with a bounded backoff, never concluded — and concludes `indeterminate` only once the hub has not answered for
 * the contract's `unreachableBoundMs` (ticket 26/09/23 S15: a 2d.puzzle creation the hub kept `accepted` read "The creation
 * outcome is unknown" after a fixed 120 s client deadline while the hub still answered every poll). */

import contract from "./🔣️.json" with { type: "json" };

export const SPACE_ARTIFACT_CREATION_POLLING_V1 = contract;

/** ⏳️ The wait before poll `attempt` (0-based): `pollInitialMs` doubling to `pollMaxMs`. */
export function spaceArtifactCreationPollDelayV1(attempt: number): number {
  return Math.min(contract.pollMaxMs, contract.pollInitialMs * 2 ** Math.min(attempt, 30));
}

/** 🛑️ Whether a creation must be concluded `indeterminate`: the hub has not answered since `lastAnsweredAtMs`. */
export function spaceArtifactCreationUnreachableV1(lastAnsweredAtMs: number, nowMs: number): boolean {
  return nowMs - lastAnsweredAtMs >= contract.unreachableBoundMs;
}
