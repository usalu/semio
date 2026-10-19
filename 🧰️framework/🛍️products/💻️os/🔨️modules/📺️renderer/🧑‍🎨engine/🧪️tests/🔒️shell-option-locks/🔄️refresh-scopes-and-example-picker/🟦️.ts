/** 🔄️ The refresh scope and example picker tests. */
import { SET_ACTIVE_EXAMPLE_ACTION_ID, buildActiveExampleAction } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { FRAMEWORK_HISTORY_BODY_KEY, resolveUiDirtyScope } from "@semio-tech/framework";
import { manifestExamplePickerFixture as examplePickerFixture } from "@semio-tech/framework/manifest/testing/fixtures";
import { examplesForApp, examplesForDialect, surfaceAppId, type AppRole } from "@semio-tech/framework";
import { readModuleTreeSource, sliceModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { expect, it } from "vitest";
import { browserActorDispatchUiScopeV1, browserActorWindowConfigDispatchUiScopeV1, hostEffectRefreshScopeV1, typedOperationCompletionRefreshV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { existsSync } from "node:fs";

/** 🧭 Registers the browser actor dispatch, typed operation, effect pass and example picker refresh scope tests. */
export function registerRefreshScopeAndExamplePickerTests(): void {
  it("dirties the whole shell when a direct browser-actor dispatch applied a mutation", () => {
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 1 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 7 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 })).toEqual({ kind: "none" });
    expect(browserActorDispatchUiScopeV1({ outcome: "rejected", mutationCount: 3 })).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridSpacing")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setVortexShow")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "worldPointerDown")).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "rejected", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "none" });
  });
  it("refreshes a typed operation on its completion's own scope and pays nothing for a completion that dirtied nothing", () => {
    const admission = browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 });
    expect(admission).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact", "puzzle.3d.play.inspector"], measures: true } as const;
    expect(typedOperationCompletionRefreshV1({ uiScope: documentScope, historyPatch: { cursor: 3, upserts: [{ seq: 3 }] }, requestedEffects: [] })).toEqual(documentScope);
    expect(typedOperationCompletionRefreshV1({ uiScope: undefined, historyPatch: undefined, requestedEffects: [] })).toEqual({ kind: "full" });
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: { cursor: 4, upserts: [{ seq: 4 }] }, requestedEffects: [] })).toEqual({
      kind: "partial",
      panelBodies: [FRAMEWORK_HISTORY_BODY_KEY],
    });
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [] })).toBeNull();
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: ["requestSync"] })).toEqual({ kind: "none" });
  });
  it("makes an effect that rewrites a guest render input earn its own pass, and leaves host chrome paying nothing", () => {
    const bodyKeys = ["puzzle.3d.play.viewport"] as const;
    const declared = typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }] });
    expect(declared, "the completion's own scope is still what the GUEST dirtied").toEqual({ kind: "none" });
    const earned = hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }], declared!, bodyKeys);
    expect(earned, "arming a utility is a host-owned render input — the pass that applies it owes the render that publishes it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: [],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(hostEffectRefreshScopeV1([{ setActiveTool: { toolId: "fill" } }], { kind: "none" }, bodyKeys), "a programmatic tool switch is the same shape as a utility arm").toEqual(earned);
    expect(hostEffectRefreshScopeV1([{ setPanel: { panelJson: "{}" } }], { kind: "none" }, bodyKeys), "`setPanel` names no body of its own — `panelJson` feeds every section").toEqual({ kind: "full" });

    expect(hostEffectRefreshScopeV1([{ notify: { message: "refused" } }], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    expect(hostEffectRefreshScopeV1([{ clipboardWrite: { text: "x" } }, { dispatchAction: { actionId: "x" } }, "requestSync"], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact"], measures: true } as const;
    expect(hostEffectRefreshScopeV1([{ notify: { message: "done" } }], documentScope, bodyKeys), "an effect that earns nothing must leave a declared scope exactly as it is").toBe(documentScope);
    expect(hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "w", utilityId: "brush" } }], documentScope, bodyKeys), "the earned scope is UNIONED with the declared one, never substituted for it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: ["puzzle.3d.play.artifact"],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
  });
  /** 🔁️ A pass's OWED effects are applied under the scope that pass declared — never a wider one.
   *
   * 🐛️ `ShellHost`'s ui-refresh lane handed `{ kind: "full" }` to `applyHostEffects` for every owed
   * application, so each React pick paid a whole extra full guest re-render on top of the narrowed
   * scope the interaction had just derived: `[TRACE] refreshUi lane {"decision":"owed","scope":{"kind":"full"}}`,
   * four of them per pick, which is why narrowing the pick's own scope moved its wall not at all
   * (`📓️interaction-scope-narrowing-2026-09-15.md` §5.3, whose owner this law names). The effects a
   * pass owes are `pending_effects` — `dispatchAction` shapes — and this proves they earn NOTHING of
   * their own, so `full` was the hardcode and not a derivation.
   */
  it("applies a pass's owed effects under that pass's own scope, never a hardcoded full one", () => {
    const bodyKeys = ["procedural.play.main", "procedural.play.preview", "procedural.play.generate-preview"] as const;
    const passScope = { kind: "partial", windowBodies: ["procedural.play.main", "procedural.play.preview"], panelBodies: [], utilities: false, tools: false, engagements: false, measures: false, labels: false } as const;
    const owedEffects = [{ dispatchAction: { req: 103, action: "flowEvalTick", args: {}, delayMs: 0 } }, { dispatchAction: { req: 104, action: "toolRunStart", args: { toolId: "previewEval" }, delayMs: 0 } }];
    expect(hostEffectRefreshScopeV1(owedEffects, passScope, bodyKeys), "a pass's owed pending_effects earn no scope of their own").toBe(passScope);
    expect(hostEffectRefreshScopeV1(owedEffects, { kind: "full" }, bodyKeys), "so the only thing that ever made the owed pass full was the caller").toEqual({ kind: "full" });

    const shellHostRelative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx";
    let shellHostRoot = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(`${shellHostRoot}/${shellHostRelative}`); hop += 1) shellHostRoot = `${shellHostRoot}/..`;
    expect(existsSync(`${shellHostRoot}/${shellHostRelative}`)).toBe(true);
    const shellHost = readModuleTreeSource(`${shellHostRoot}/${shellHostRelative}`);
    const lane = sliceModuleTreeSource(`${shellHostRoot}/${shellHostRelative}`, "uiRefreshLaneRef.current = createUiRefreshCoalescerV1", "const refreshUi = useCallback");
    expect(lane, "the lane applies the scope the pass recorded").toContain("applyHostEffectsRef.current(owedEffects.effects, owedEffects.session, owedEffects.scope, owedEffects.owner)");
    expect(lane.includes('{ kind: "full" }'), "and names no scope of its own").toBe(false);
    expect(shellHost, "the owed slot records the pass's scope when it records the effects").toContain("owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner, scope: scopeArg }");
  });
  it("resolves the example picker by dialect, so an editor and its viewer offer exactly the same examples", () => {
    
    
    for (const useCase of examplePickerFixture.cases) {
      expect(surfaceAppId(useCase.app.dialect, useCase.app.role as AppRole), useCase.name).toBe(useCase.app.id);
      expect(
        examplesForDialect(examplePickerFixture.examples, useCase.app.dialect).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
      expect(
        examplesForApp(examplePickerFixture.examples, useCase.app).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
    }
    const editor = examplePickerFixture.cases.find((useCase) => useCase.name === "editor-of-the-dialect-offers-every-example-of-it")!;
    const viewer = examplePickerFixture.cases.find((useCase) => useCase.name === "viewer-of-the-same-dialect-offers-exactly-the-same-picker")!;
    expect(viewer.expected).toEqual(editor.expected);
    expect(viewer.app.id).not.toBe(editor.app.id);
    expect(examplesForApp(examplePickerFixture.examples, {})).toEqual([]);
  });
  it("the example picker dispatches the chosen example id and its completion refreshes the whole shell", () => {
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#editor", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#editor",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#viewer", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#viewer",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    expect(buildActiveExampleAction("c", "")).toEqual({ controllerId: "c", action: "setActiveExample", args: { exampleId: "" } });
    expect(resolveUiDirtyScope({ kind: "full" })).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope(undefined)).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope({ kind: "none" })).toEqual({ kind: "none" });
  });
}
