/** 🏠️ Cold diff and apply of the tutorial local interaction state keep the exact three-map authored changes; immer (third party) is the independent oracle for every fixture row. */
import { expect, test } from "bun:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { enableMapSet, produce } from "immer";
import { applyTutorialLocalInteractionCold, diffTutorialLocalInteractionCold, type TutorialLocalInteractionChange } from "../../🏠️local-interaction/🟦️.ts";
import { tutorialLocalInteractionFixtureUrl } from "../🧫️fixtures/🟦️.ts";

type State = Parameters<typeof diffTutorialLocalInteractionCold>[0];
type Row = { name: string; before: State; after: State; changes: TutorialLocalInteractionChange[] };

test("TutorialLocalInteraction preserves exact three-map authored changes against Immer", () => {
  const fixture = JSON.parse(readFileSync(tutorialLocalInteractionFixtureUrl, "utf8")) as { cases: Row[] };
  enableMapSet();
  expect(typeof diffTutorialLocalInteractionCold).toBe("function");
  expect(typeof applyTutorialLocalInteractionCold).toBe("function");
  for (const row of fixture.cases) {
    const before = JSON.stringify(row.before);
    const changes = diffTutorialLocalInteractionCold(row.before, row.after);
    assert.deepStrictEqual(changes, row.changes, row.name);
    let actual = row.before;
    let oracle = { selection: new Map(Object.entries(row.before.selection)), activeMode: new Map(Object.entries(row.before.activeMode)), activeGranularity: new Map(Object.entries(row.before.activeGranularity)) };
    for (const change of changes) {
      actual = applyTutorialLocalInteractionCold(actual, change);
      oracle = produce(oracle, draft => {
        if (change.patch.selection === null) draft.selection.delete(change.domainId); else draft.selection.set(change.domainId, { ...change.patch.selection, ids: [...change.patch.selection.ids] });
        if (change.patch.activeMode === null) draft.activeMode.delete(change.domainId); else draft.activeMode.set(change.domainId, change.patch.activeMode);
        if (change.patch.activeGranularity === null) draft.activeGranularity.delete(change.domainId); else draft.activeGranularity.set(change.domainId, change.patch.activeGranularity);
      });
    }
    assert.deepStrictEqual(actual, { selection: Object.fromEntries(oracle.selection), activeMode: Object.fromEntries(oracle.activeMode), activeGranularity: Object.fromEntries(oracle.activeGranularity) }, row.name);
    assert.deepStrictEqual(actual, row.after, row.name);
    expect(JSON.stringify(row.before)).toBe(before);
    expect(diffTutorialLocalInteractionCold(row.after, row.after)).toEqual([]);
  }
});
