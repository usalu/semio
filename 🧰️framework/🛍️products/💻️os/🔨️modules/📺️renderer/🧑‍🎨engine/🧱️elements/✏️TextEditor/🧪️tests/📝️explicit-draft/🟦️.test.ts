import Ajv2020 from "ajv/dist/2020";
import { describe, expect, test } from "vitest";
import fixture from "../../🧫️fixtures/📝️explicit-draft/🔣️.json";
import lifecycle from "../../🧫️fixtures/📝️explicit-draft/⚖️lifecycle.json";
import retention from "../../🧫️fixtures/📝️explicit-draft/🧭️retention.json";
import schema from "../../🧬️schema/📝️explicit-draft/🔣️.json";
import { localDocumentOwnerIdentityForSessionV1, LocalDocumentOwnerRegistryV1 } from "../../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { startedTypedOperationV1, typedOperationCancellationActionV1, typedOperationCancellationOwnerIsCurrentV1 } from "../../../🏛️ShellHost/🟦️.tsx";
import { TEXT_EDITOR_RETAINED_DRAFT_LIMIT_V1, acceptTextEditorExplicitDraftCancellation, beginTextEditorExplicitDraft, changeTextEditorExplicitDraft, createTextEditorExplicitDraft, parseTextEditorExplicitDraftSettings, reconcileTextEditorExplicitDraft, refuseTextEditorExplicitDraftCancellation, requestTextEditorExplicitDraftCancellation, settleTextEditorExplicitDraft, startTextEditorExplicitDraftOperation, textEditorExplicitDraftAction, textEditorExplicitDraftFailureV1 } from "../../🟦️.tsx";

const dirtyDraft = () => changeTextEditorExplicitDraft(createTextEditorExplicitDraft(lifecycle.source, lifecycle.base, lifecycle.baseRevision), lifecycle.source, lifecycle.base, lifecycle.draft, lifecycle.baseRevision);
const applied = { kind: "applied", inputSeq: 1, commit: { operation: lifecycle.operation, revision: lifecycle.ownRevision } } as const;

describe("text editor explicit drafts", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);

  test("matches the language-neutral schema and builds artifact-owned actions", () => {
    expect(TEXT_EDITOR_RETAINED_DRAFT_LIMIT_V1).toBe(retention.limits.draftsPerOwner);
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

  test("refuses owner-capacity exhaustion without retiring a current document", () => {
    const registry = new LocalDocumentOwnerRegistryV1(retention.limits.owners);
    const current = registry.acquire(retention.owner);
    let retired = false;
    current.subscribeRetirement(() => { retired = true; });
    registry.acquire({ ...retention.owner, runtimeKey: `${retention.owner.runtimeKey}-2` });
    registry.acquire({ ...retention.owner, runtimeKey: `${retention.owner.runtimeKey}-3` });
    expect(() => registry.acquire({ ...retention.owner, runtimeKey: `${retention.owner.runtimeKey}-4` })).toThrow("local document owner capacity is exhausted");
    expect(registry.acquire(retention.owner)).toBe(current);
    expect(current.active).toBe(true);
    expect(retired).toBe(false);
  });

  test("owns an initial native session and retires it when an opened document becomes authoritative", () => {
    const registry = new LocalDocumentOwnerRegistryV1();
    const initialIdentity = localDocumentOwnerIdentityForSessionV1(retention.initialSession);
    const initial = registry.acquire(initialIdentity);
    let retired = false;
    initial.subscribeRetirement(() => { retired = true; });
    expect(initial.active).toBe(true);
    expect(initial.identity.runtimeKey).toBe("shell.primary-session");
    const openedIdentity = localDocumentOwnerIdentityForSessionV1(retention.initialSession, retention.openedDocument);
    const opened = registry.acquire(openedIdentity);
    registry.retain([openedIdentity]);
    expect(opened.identity).toEqual(retention.owner);
    expect(opened.active).toBe(true);
    expect(initial.active).toBe(false);
    expect(retired).toBe(true);
  });

  test("does not acknowledge a foreign publication merely because its text matches the draft", () => {
    const published = reconcileTextEditorExplicitDraft(dirtyDraft(), lifecycle.source, lifecycle.draft, lifecycle.foreignRevision);
    expect(published.dirty).toBe(lifecycle.expected.foreignDirty);
    expect(published.conflicted).toBe(lifecycle.expected.foreignConflict);
  });

  test("preserves an invalid draft through refusal and unrelated rerender until discard", () => {
    const row = fixture.preservation;
    const dirty = changeTextEditorExplicitDraft(createTextEditorExplicitDraft(lifecycle.source, row.initial, lifecycle.baseRevision), lifecycle.source, row.initial, row.draft, lifecycle.baseRevision);
    const unrelated = reconcileTextEditorExplicitDraft(dirty, lifecycle.source, row.unrelatedScene, lifecycle.baseRevision);
    expect(unrelated.draft).toBe(row.afterUnrelatedScene);
    expect(unrelated.conflicted).toBe(false);
    const conflicted = reconcileTextEditorExplicitDraft(dirty, lifecycle.source, row.collaboratorScene, lifecycle.foreignRevision);
    expect(conflicted.draft).toBe(row.afterConflict);
    expect(conflicted.base).toBe(row.initial);
    expect(conflicted.conflicted).toBe(true);
    expect(reconcileTextEditorExplicitDraft(dirty, lifecycle.source, row.initial, lifecycle.baseRevision).draft).toBe(row.afterRefusal);
    expect(createTextEditorExplicitDraft(lifecycle.source, row.initial, lifecycle.baseRevision).draft).toBe(row.afterDiscard);
  });

  test("clears a collaborator conflict when the user explicitly matches the current publication", () => {
    const conflicted = reconcileTextEditorExplicitDraft(dirtyDraft(), lifecycle.source, lifecycle.newerDraft, lifecycle.foreignRevision);
    const matched = changeTextEditorExplicitDraft(conflicted, lifecycle.source, lifecycle.newerDraft, lifecycle.newerDraft, lifecycle.foreignRevision);
    expect(matched).toEqual(createTextEditorExplicitDraft(lifecycle.source, lifecycle.newerDraft, lifecycle.foreignRevision));
  });

  test("binds and cancels only the exact admitted operation with canonical u64 authority", () => {
    const started = beginTextEditorExplicitDraft(dirtyDraft());
    const owner = started.pending!;
    const operation = startedTypedOperationV1({ operationId: lifecycle.operation, generation: lifecycle.operationGeneration })!;
    const action = typedOperationCancellationActionV1({ controllerId: "document", action: "replaceSnapshotSource", args: {}, provenance: { windowId: lifecycle.source } } as never, operation);
    expect(action.action).toBe(lifecycle.cancellation.action);
    expect(action.args).toEqual({ operationId: lifecycle.cancellation.operationHex, generation: lifecycle.cancellation.generationHex });
    expect(action.controllerId).toBe("document");
    expect(action.provenance?.windowId).toBe(lifecycle.source);
    const control = { operationId: operation.operationId, generation: operation.generation, cancel: async () => ({ kind: "applied" }) } as const;
    const bound = startTextEditorExplicitDraftOperation(started, owner, control);
    expect(bound.operation).toBe(control);
    expect(startTextEditorExplicitDraftOperation(bound, owner, { ...control, operationId: "7" })).toBe(bound);
    const foreign = beginTextEditorExplicitDraft(dirtyDraft());
    expect(startTextEditorExplicitDraftOperation(foreign, owner, control)).toBe(foreign);
    const requested = requestTextEditorExplicitDraftCancellation(bound, owner);
    expect(requested.cancelling).toBe(true);
    expect(requested.draft).toBe(lifecycle.draft);
    expect(requestTextEditorExplicitDraftCancellation(requested, owner)).toBe(requested);
    const cancelled = acceptTextEditorExplicitDraftCancellation(requested, owner);
    expect(cancelled.cancellationAccepted).toBe(true);
    const terminal = settleTextEditorExplicitDraft(cancelled, owner, applied, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
    expect(terminal.pending).toBeNull();
    expect(terminal.draft).toBe(lifecycle.draft);
    expect(terminal.dirty).toBe(lifecycle.cancellation.preservesDraft);
  });

  test("a refused cancellation resumes the exact original completion without losing its draft", () => {
    const started = beginTextEditorExplicitDraft(dirtyDraft());
    const owner = started.pending!;
    const control = { operationId: lifecycle.operation, generation: lifecycle.operationGeneration, cancel: async () => ({ kind: "refused" }) } as const;
    const requested = requestTextEditorExplicitDraftCancellation(startTextEditorExplicitDraftOperation(started, owner, control), owner);
    const terminalFirst = settleTextEditorExplicitDraft(requested, owner, applied, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
    expect(terminalFirst.pending?.settled).toBe(true);
    expect(terminalFirst.draft).toBe(lifecycle.draft);
    const resumed = refuseTextEditorExplicitDraftCancellation(terminalFirst, owner, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
    expect(resumed.cancelling).toBe(false);
    expect(resumed.pending).not.toBeNull();
    expect(resumed.draft).toBe(lifecycle.draft);
    expect(resumed.failed).toBe(false);
  });

  test("an old operation handle cannot target a replacement program or document owner", () => {
    const admitted = { pluginId: retention.owner.pluginId, instanceId: retention.owner.sessionInstanceId, app: { id: retention.owner.appId, controllerId: "document" } };
    const replacement = { ...admitted, instanceId: admitted.instanceId + 1 };
    expect(typedOperationCancellationOwnerIsCurrentV1(admitted, admitted, true)).toBe(true);
    expect(typedOperationCancellationOwnerIsCurrentV1(admitted, replacement, true)).toBe(false);
    expect(typedOperationCancellationOwnerIsCurrentV1(admitted, admitted, false)).toBe(false);
  });

  for (const order of lifecycle.orders) {
    test(`acknowledges its exact native revision with normalized text: ${order}`, () => {
      let state = beginTextEditorExplicitDraft(dirtyDraft());
      const owner = state.pending!;
      expect(beginTextEditorExplicitDraft(state)).toBe(state);
      if (order === "publication-first") {
        state = reconcileTextEditorExplicitDraft(state, lifecycle.source, lifecycle.normalized, lifecycle.ownRevision);
        expect(state.dirty).toBe(true);
        expect(state.pending).toBe(owner);
        state = settleTextEditorExplicitDraft(state, owner, applied, lifecycle.source, lifecycle.normalized, lifecycle.ownRevision);
      } else {
        state = settleTextEditorExplicitDraft(state, owner, applied, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
        expect(state.dirty).toBe(true);
        expect(state.pending).not.toBeNull();
        state = reconcileTextEditorExplicitDraft(state, lifecycle.source, lifecycle.normalized, lifecycle.ownRevision);
      }
      expect(state.dirty).toBe(lifecycle.expected.ownDirty);
      expect(state.draft).toBe(lifecycle.normalized);
      expect(state.pending).toBeNull();
      expect(state.failed).toBe(false);
    });

    test(`preserves a foreign equal-text publication as a conflict: ${order}`, () => {
      let state = beginTextEditorExplicitDraft(dirtyDraft());
      const owner = state.pending!;
      if (order === "publication-first") {
        state = reconcileTextEditorExplicitDraft(state, lifecycle.source, lifecycle.draft, lifecycle.foreignRevision);
        state = settleTextEditorExplicitDraft(state, owner, applied, lifecycle.source, lifecycle.draft, lifecycle.foreignRevision);
      } else {
        state = settleTextEditorExplicitDraft(state, owner, applied, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
        state = reconcileTextEditorExplicitDraft(state, lifecycle.source, lifecycle.draft, lifecycle.foreignRevision);
      }
      expect(state.dirty).toBe(lifecycle.expected.foreignDirty);
      expect(state.conflicted).toBe(lifecycle.expected.foreignConflict);
      expect(state.draft).toBe(lifecycle.draft);
      expect(state.pending).toBeNull();
    });
  }

  for (const failure of lifecycle.failures) {
    test(`preserves the draft and releases Apply after ${failure}`, () => {
      const state = beginTextEditorExplicitDraft(dirtyDraft());
      const outcome = failure === "refused" ? { kind: "refused", inputSeq: 1, reason: "dispatch-failed", retryable: true } : failure === "superseded" ? { kind: "superseded", inputSeq: 1, by: 2 } : failure === "missing-receipt" ? { kind: "applied", inputSeq: 1 } : null;
      const settled = settleTextEditorExplicitDraft(state, state.pending!, outcome, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
      expect(settled.draft).toBe(lifecycle.draft);
      expect(settled.dirty).toBe(lifecycle.expected.failureDirty);
      expect(settled.failed).toBe(lifecycle.expected.failureFailed);
      expect(settled.pending).toBeNull();
      expect(beginTextEditorExplicitDraft(settled).pending).not.toBeNull();
    });
  }

  test("preserves a typed syntax refusal and formats its exact location through localized labels", () => {
    const settings = parseTextEditorExplicitDraftSettings(JSON.stringify(fixture.cases[0]!.settings))!;
    const state = beginTextEditorExplicitDraft(dirtyDraft());
    const outcome = { kind: "refused", inputSeq: 1, reason: "dispatch-failed", retryable: true, diagnostic: { code: "snapshot-edit.invalid-source", message: "unexpected byte", span: { line: 3, column: 3, length: 1 }, params: { path: "/rows/2/name" } } } as const;
    const settled = settleTextEditorExplicitDraft(state, state.pending!, outcome, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
    expect(settled.diagnostic).toEqual(outcome.diagnostic);
    expect(textEditorExplicitDraftFailureV1(settings, settled.diagnostic)).toBe("The source contains invalid syntax. Line 3, Column 3, Path /rows/2/name");
  });

  test("preserves newer local typing when an earlier Apply publishes", () => {
    const started = beginTextEditorExplicitDraft(dirtyDraft());
    const typed = changeTextEditorExplicitDraft(started, lifecycle.source, lifecycle.base, lifecycle.newerDraft, lifecycle.baseRevision);
    const settled = settleTextEditorExplicitDraft(typed, started.pending!, applied, lifecycle.source, lifecycle.normalized, lifecycle.ownRevision);
    expect(settled.draft).toBe(lifecycle.newerDraft);
    expect(settled.base).toBe(lifecycle.normalized);
    expect(settled.revision).toBe(lifecycle.ownRevision);
    expect(settled.dirty).toBe(true);
    expect(settled.conflicted).toBe(false);
  });

  test("a retired Apply owner cannot alter another document or a later retry", () => {
    const started = beginTextEditorExplicitDraft(dirtyDraft());
    const other = reconcileTextEditorExplicitDraft(started, lifecycle.otherSource, lifecycle.newerDraft, lifecycle.foreignRevision);
    expect(settleTextEditorExplicitDraft(other, started.pending!, applied, lifecycle.otherSource, lifecycle.newerDraft, lifecycle.foreignRevision)).toBe(other);
    const failed = settleTextEditorExplicitDraft(started, started.pending!, null, lifecycle.source, lifecycle.base, lifecycle.baseRevision);
    const retry = beginTextEditorExplicitDraft(failed);
    expect(settleTextEditorExplicitDraft(retry, started.pending!, applied, lifecycle.source, lifecycle.normalized, lifecycle.ownRevision)).toBe(retry);
  });
});
