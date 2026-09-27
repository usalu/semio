import Ajv2020 from "ajv/dist/2020";
import { describe, expect, test } from "vitest";
import fixture from "../../🧫️fixtures/📝️explicit-draft/🔣️.json";
import schema from "../../🧬️schema/📝️explicit-draft/🔣️.json";
import { changeTextEditorExplicitDraft, parseTextEditorExplicitDraftSettings, reconcileTextEditorExplicitDraft, textEditorExplicitDraftAction } from "../../🟦️.tsx";

describe("text editor explicit drafts", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);

  test("matches the language-neutral schema and builds artifact-owned actions", () => {
    for (const row of fixture.cases) {
      expect(validate(row.settings), row.id).toBe(true);
      const settings = parseTextEditorExplicitDraftSettings(JSON.stringify(row.settings));
      expect(settings, row.id).not.toBeNull();
      const action = textEditorExplicitDraftAction(settings!, row.draft);
      expect({ action: action.action, args: action.args }, row.id).toEqual(row.expected);
    }
    for (const row of fixture.rejectedSettings) {
      expect(validate(row.settings), row.id).toBe(false);
      expect(parseTextEditorExplicitDraftSettings(JSON.stringify(row.settings)), row.id).toBeNull();
    }
  });

  test("preserves an invalid draft through refusal and unrelated rerender until discard", () => {
    const row = fixture.preservation;
    const dirty = { surfaceId: "source", base: row.initial, draft: row.draft, dirty: true, conflicted: false } as const;
    const unrelated = reconcileTextEditorExplicitDraft(dirty, "source", row.unrelatedScene);
    expect(unrelated.draft).toBe(row.afterUnrelatedScene);
    expect(unrelated.conflicted).toBe(false);
    const conflicted = reconcileTextEditorExplicitDraft(dirty, "source", row.collaboratorScene);
    expect(conflicted.draft).toBe(row.afterConflict);
    expect(conflicted.base).toBe(row.initial);
    expect(conflicted.conflicted).toBe(true);
    expect(reconcileTextEditorExplicitDraft(dirty, "source", row.initial).draft).toBe(row.afterRefusal);
    expect(reconcileTextEditorExplicitDraft({ ...dirty, dirty: false }, "source", row.initial).draft).toBe(row.afterDiscard);
  });

  test("clears a collaborator conflict when the local draft matches the persisted value", () => {
    const row = fixture.conflictResolution;
    const local = changeTextEditorExplicitDraft({ surfaceId: "source", base: row.initial, draft: row.initial, dirty: false, conflicted: false }, "source", row.initial, row.localDraft);
    const conflicted = reconcileTextEditorExplicitDraft(local, "source", row.collaboratorScene);
    expect(conflicted.conflicted).toBe(true);
    expect(changeTextEditorExplicitDraft(conflicted, "source", row.collaboratorScene, row.matchingDraft)).toEqual({
      surfaceId: "source",
      base: row.collaboratorScene,
      draft: row.matchingDraft,
      dirty: false,
      conflicted: false,
    });
  });
});
