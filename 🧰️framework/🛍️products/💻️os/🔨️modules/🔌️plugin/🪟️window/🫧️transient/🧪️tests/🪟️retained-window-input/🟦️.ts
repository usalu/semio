import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

/** 🧪️ Checks the neutral retained window input identity contract independently. */
export function testRetainedWindowInputOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪟️retained-window-input/🔣️.json", import.meta.url), "utf8"));
  const identities = new Set(fixture.cases.map((row: { windowId: string; windowKindId: string; generation: number; documentGeneration: number }) => JSON.stringify([row.windowId, row.windowKindId, row.generation, row.documentGeneration])));
  assert.equal(identities.size, fixture.expectedUniqueOwners);
  const replacement = fixture.documentReplacement;
  const reset = Object.fromEntries(Object.keys(replacement.before).map((id) => [id, 0]));
  assert.deepEqual(reset, replacement.after);
  assert.equal(replacement.documentGeneration, 1);
  assert.equal(replacement.oldPublicationAccepted, false);
  assert.equal(replacement.oldWorkCancelled, true);
  assert.equal(replacement.newWorkCancelled, false);
  const fairness = fixture.retirementFairness;
  const pending = new Map<string, boolean>(fairness.owners.map((owner: string) => [owner, fairness.blocked.includes(owner)]));
  for (const [owner, blocked] of pending) if (!blocked) pending.delete(owner);
  assert.deepEqual([...pending.keys()], fairness.expectedSurvivors);
  assert.equal(fairness.zeroGrantAdvancesCursor, false);
}
