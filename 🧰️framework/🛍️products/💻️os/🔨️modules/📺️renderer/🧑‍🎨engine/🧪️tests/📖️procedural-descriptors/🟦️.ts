/** 📖 Procedural descriptors: contributions push, window-kind scoping, stable dom ids, undeclared actions and example switch. */
import { historyPatchShouldApplyV1 } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { historyRefreshNeededV1 } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { undeclaredActionDiagnostic } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { FRAMEWORK_HISTORY_BODY_KEY } from "@semio-tech/framework";
import { uiNodeDomId } from "../../🧱️elements/🗣️Interpreter/🪪️stable-dom-ids/🟦️.ts";
import { describe, expect, it } from "vitest";
import { applyUiRefreshResponseToCache, buildUiRefreshRequest, mergeRecordPreservingIdentity, type UiRefreshCache } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { mergeUiDirtyScopeV1, typedOperationCompletionRefreshV1 } from "../../🧱️elements/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";
import { appCommandTakesPageRun, appOwnsCommand } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";


//#region 📄️ContributionsPushDeclaration
/** 📄️ One plugin descriptor, read off disk rather than imported, because the procedural manifest is
 * ~1 MB and belongs in no test bundle. */
const readPluginManifest = (relativeUrl: string): { readonly apps?: readonly { readonly id: string; readonly commands?: readonly { readonly id: string }[] }[] } | undefined => {
  const path = fileURLToPath(new URL(relativeUrl, import.meta.url));
  if (!existsSync(path)) return undefined;
  const descriptor = JSON.parse(readFileSync(path, "utf8")) as { readonly manifest?: unknown };
  return (descriptor.manifest ?? descriptor) as { readonly apps?: readonly { readonly id: string; readonly commands?: readonly { readonly id: string }[] }[] };
};

const PROCEDURAL_SOURCE_DESCRIPTOR = pathToFileURL(process.env.SEMIO_PROCEDURAL_COMPOSITION_DESCRIPTOR ?? "/unsupplied/SEMIO_PROCEDURAL_COMPOSITION_DESCRIPTOR.json").href;

/** 📄️ The dev server's own copy — a build artifact, so it is asserted only when a dev tree has one.
 * A stale copy is exactly how a served boot ends up receiving one unpaged 293 KiB command the guest
 * refuses as `command contains an oversized string`. */
const PROCEDURAL_SERVED_DESCRIPTOR = "../../../../🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/🔣️.json";

describe("contributions push declaration", () => {
  /** ⚖️ LAW: every procedural app that opts into the host's `setContributions` push declares the
   * `pageCount` argument so the addressed schema matches. The shell sends page 0 of 1 as one
   * pack-encoded `handleCommand` (`PluginRuntime` `encodePackValue`), not 4 KiB JSON pages.
   * The gate is `appCommandTakesPageRun`, so the law runs THAT, never a copy of it
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("declares pageCount on every procedural app the shell pushes contributions to", () => {
    for (const [label, relativeUrl] of [
      ["source", PROCEDURAL_SOURCE_DESCRIPTOR],
      ["served", PROCEDURAL_SERVED_DESCRIPTOR],
    ] as const) {
      const manifest = readPluginManifest(relativeUrl);
      if (!manifest) {
        expect(label).toBe("served");
        continue;
      }
      const receivers = (manifest.apps ?? []).filter((app) => appOwnsCommand(app as never, "setContributions"));
      expect(receivers.map((app) => app.id).sort()).toEqual(["s.procedural.generation2d@1/*#editor", "s.procedural.generation3d@1/*#editor", "s.procedural.generation3d@1/*#viewer"]);
      for (const app of receivers) {
        expect(`${label}:${app.id}:${appCommandTakesPageRun(app as never, "setContributions")}`).toBe(`${label}:${app.id}:true`);
      }
    }
  });
});
//#endregion 📄️ContributionsPushDeclaration


//#region 📇️WindowKindActionScoping
type DescriptorWindowKind = { readonly id: string; readonly actions?: readonly { readonly id: string }[] };

type DescriptorApp = { readonly id: string; readonly actions?: readonly { readonly id: string }[]; readonly windowKinds?: readonly DescriptorWindowKind[] };

const windowKindsDeclaring = (app: DescriptorApp, actionId: string): readonly string[] => (app.windowKinds ?? []).filter((kind) => (kind.actions ?? []).some((action) => action.id === actionId)).map((kind) => kind.id);

describe("window-kind action scoping", () => {
  /** ⚖️ LAW: a window-scoped action is declared on the window kinds that dispatch it and on NO other.
   * `WindowKindDefinition.actions` is what `ShellHost`'s `declaredAction` gate reads before it will call
   * `plugin.handleAction`, what the focused-window keybinding table resolves against, and what a window's
   * chrome menu renders — and until this ticket every generation3d/generation2d window carried a copy of
   * the whole app action list, because `build_definition` copies every UNOWNED app action onto every
   * window (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`) and no window owned
   * anything. This reads the published descriptor, so it fails on a stale
   * `@semio-tech/procedural-plugin:describe` as well as on a lost `.window_kind_action_refs(...)`
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("scopes each generation window's own actions to that window in the published procedural descriptor", () => {
    const manifest = readPluginManifest(PROCEDURAL_SOURCE_DESCRIPTOR) as { readonly apps?: readonly DescriptorApp[] } | undefined;
    expect(manifest).toBeDefined();
    const apps = new Map((manifest?.apps ?? []).map((app) => [app.id, app]));
    const expectations: readonly (readonly [string, string, readonly string[]])[] = [
      ["s.procedural.generation3d@1/*#editor", "addGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "selectGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "renameGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "removeGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "updateGenerationValues", ["generation3d-generate-form"]],
      ["s.procedural.generation3d@1/*#editor", "setLodMode", ["procedural-main"]],
      ["s.procedural.generation3d@1/*#editor", "nodeGraphEdit", ["procedural-main"]],
      ["s.procedural.generation3d@1/*#editor", "setShowMode", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#editor", "setSunAzimuth", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#editor", "translateSelection", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#viewer", "setShowMode", ["procedural-view-preview"]],
      ["s.procedural.generation3d@1/*#viewer", "setCamera", ["procedural-view-preview"]],
      ["s.procedural.generation2d@1/*#editor", "addGeneration", ["generation2d-generations"]],
      ["s.procedural.generation2d@1/*#editor", "updateGenerationValues", ["generation2d-generate-form"]],
      ["s.procedural.generation2d@1/*#editor", "nodeGraphViewport", ["generation2d-main"]],
      ["s.procedural.generation2d@1/*#editor", "canvasWheel", ["generation2d-generate-preview", "generation2d-preview"]],
    ];
    for (const [appId, actionId, owners] of expectations) {
      const app = apps.get(appId);
      expect(`${appId}:declared`).toBe(app ? `${appId}:declared` : `${appId}:missing`);
      expect(
        `${appId}:${actionId}:${windowKindsDeclaring(app as DescriptorApp, actionId)
          .slice()
          .sort()
          .join(",")}`,
      ).toBe(`${appId}:${actionId}:${owners.slice().sort().join(",")}`);
    }
  });

  /** ⚖️ App-scoped verbs have one app declaration and pass the production dispatch gate from every window. */
  it("declares procedural app-scoped verbs once and accepts them from every window kind", () => {
    const manifest = readPluginManifest(PROCEDURAL_SOURCE_DESCRIPTOR) as { readonly apps?: readonly DescriptorApp[] } | undefined;
    const apps = new Map((manifest?.apps ?? []).map((app) => [app.id, app]));
    for (const [appId, actionId] of [
      ["s.procedural.generation3d@1/*#editor", "setActiveExample"],
      ["s.procedural.generation3d@1/*#editor", "addWidget"],
      ["s.procedural.generation3d@1/*#editor", "undo"],
      ["s.procedural.generation2d@1/*#editor", "addWidget"],
      ["s.procedural.generation2d@1/*#editor", "undo"],
    ] as const) {
      const app = apps.get(appId) as DescriptorApp;
      expect(app, appId).toBeDefined();
      expect(
        app.actions?.filter((action) => action.id === actionId),
        `${appId}:${actionId} app owner`,
      ).toHaveLength(1);
      expect(windowKindsDeclaring(app, actionId), `${appId}:${actionId} explicit window owners`).toEqual([]);
      expect(app.windowKinds?.length).toBeGreaterThan(0);
      for (const windowKindId of [null, ...(app.windowKinds ?? []).map((kind) => kind.id)]) {
        expect(undeclaredActionDiagnostic(appId, actionId, app.windowKinds ?? [], windowKindId, app.actions), `${appId}:${actionId}:${windowKindId}`).toBeNull();
        if (actionId !== "undo")
          expect(
            undeclaredActionDiagnostic(
              appId,
              actionId,
              app.windowKinds ?? [],
              windowKindId,
              app.actions?.filter((action) => action.id !== actionId),
            ),
          ).not.toBeNull();
      }
    }
  });
});
//#endregion 📇️WindowKindActionScoping


//#region 🪪️StableUiNodeDomIds
describe("stable ui node dom ids", () => {
  /** ⚖️ LAW: a bound node's DOM id is a function of the SURFACE and the node's own Rust-authored `key`,
   * never of `UiNodeRecord.id` — that integer is re-minted in DFS order on every full-body
   * reconciliation (`builtNodeToSnapshot`), so an id built from it names a different row after the next
   * refresh and a scripted or assistive click silently targets the wrong node
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("derives the dom id from the surface and the authored key, not the reconciliation ordinal", () => {
    expect(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 5)).toBe("procedural.play.generations/procedural3d-play-generate.add-generation");
    expect(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 12)).toBe(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 5));
  });

  /** ⚖️ LAW: the surface is the namespace — two windows of one app rendering the same authored key get
   * two distinct DOM ids. */
  it("namespaces the same authored key per window surface", () => {
    expect(uiNodeDomId("procedural.play.generations", "generation.g1", 1)).not.toBe(uiNodeDomId("procedural.play.generate-form", "generation.g1", 1));
  });

  /** ⚖️ LAW: a keyless node still gets a usable id — the volatile ordinal is the FALLBACK, never the
   * default. */
  it("falls back to the reconciliation ordinal only for a keyless node", () => {
    expect(uiNodeDomId("procedural.play.main", "", 7)).toBe("node-7");
  });

  /** ⚖️ LAW: every authored key of one rendered body mints a distinct DOM id, so `getElementById` is
   * unambiguous across the whole window body. */
  it("mints one distinct dom id per authored key of a body", () => {
    const keys = ["procedural3d-play-generate", "procedural3d-play-generate.generations", "procedural3d-play-generate.generation.generation-1", "procedural3d-play-generate.actions", "procedural3d-play-generate.add-generation"];
    const ids = new Set(keys.map((key, index) => uiNodeDomId("procedural.play.generations", key, index + 1)));
    expect(ids.size).toBe(keys.length);
  });
});
//#endregion 🪪️StableUiNodeDomIds


//#region 🚨️UndeclaredActionDiagnostic
describe("undeclared action diagnostic", () => {
  const windowKinds = [
    { id: "generation3d-generations", actions: [{ id: "addGeneration" }] },
    { id: "procedural-main", actions: [{ id: "nodeGraphEdit" }] },
  ];

  /** ⚖️ LAW: the drop is DESCRIBED, not silent — the message names the app, the action and the window
   * kind the dispatch came from, and is not `[TRACE]`-prefixed, because it is the only signal a fully
   * wired binding died (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("names the app, the action and the dispatching window kind", () => {
    const diagnostic = undeclaredActionDiagnostic("generation3d", "setActiveExample", windowKinds, "procedural-main");
    expect(diagnostic).not.toBeNull();
    expect(diagnostic?.action).toBe("setActiveExample");
    expect(diagnostic?.windowKindId).toBe("procedural-main");
    expect(diagnostic?.windowKindIds).toEqual(["generation3d-generations", "procedural-main"]);
    expect(diagnostic?.message.includes("[TRACE]")).toBe(false);
    for (const fragment of ["generation3d", "setActiveExample", "procedural-main", "window_kind_action_refs"]) expect(diagnostic?.message.includes(fragment)).toBe(true);
  });

  /** ⚖️ LAW: the gate is app-wide — an action owned by ONE window kind is dispatchable from anywhere
   * (a context menu, the palette, a keybinding), so declaring `addGeneration` only on the Generations
   * window must not turn a context-menu dispatch from the flow window into a drop. */
  it("accepts an action declared on any window kind, whatever window dispatched it", () => {
    expect(undeclaredActionDiagnostic("generation3d", "addGeneration", windowKinds, "procedural-main")).toBeNull();
    expect(undeclaredActionDiagnostic("generation3d", "addGeneration", windowKinds, null)).toBeNull();
  });

  /** ⚖️ LAW: the framework's own reserved verbs are never a plugin's to declare. */
  it("never reports a framework-reserved verb", () => {
    for (const action of ["undo", "redo", "setActiveUtility", "setActiveTool"]) expect(undeclaredActionDiagnostic("generation3d", action, windowKinds)).toBeNull();
  });

  /** ⚖️ LAW: an app with no window kinds at all still produces a readable message rather than an empty list. */
  it("says so when the app declares no window kinds", () => {
    expect(undeclaredActionDiagnostic("empty", "addGeneration", [])?.message.includes("window kinds: none")).toBe(true);
  });
});
//#endregion 🚨️UndeclaredActionDiagnostic

describe("history patch apply", () => {
  it("applies an equal-cursor history patch when it carries upserts", () => {
    expect(historyPatchShouldApplyV1(0, { cursor: 0, upserts: [{ seq: 1 }] })).toBe(true);
    expect(historyPatchShouldApplyV1(0, { cursor: 1, upserts: [{ seq: 1 }] })).toBe(true);
    expect(historyPatchShouldApplyV1(1, { cursor: 0, upserts: [{ seq: 1 }] })).toBe(false);
    expect(historyPatchShouldApplyV1(1, { cursor: 1 })).toBe(false);
    expect(historyPatchShouldApplyV1(1, { cursor: 0 }, true)).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", undefined)).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", { upserts: [] })).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", { upserts: [{ seq: 1 }] })).toBe(false);
    expect(historyRefreshNeededV1("undo", undefined)).toBe(false);
  });
});

//#region 🎨️ExampleSwitchHostCaching
// 🎨️ The HOST half of the example-switch runtime law (`🧫️fixtures/🎨️example-switch.json` answers the
// guest half in Rust). Picking an example replaces the artifact's fixture through a RETAINED typed
// operation, so the flow window's new body reaches the shell only through that operation's
// completion — and only if the completion's own `UiDirtyScope` asks for the window body. The trap
// this block pins: a completion that dirties nothing but carries a history patch refreshes ONLY the
// reserved history panel, which repaints the History list while the flow window keeps the previous
// example's graph and nothing faults anywhere (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
describe("example switch — the completion's scope is what re-takes the flow window body", () => {
  const generation3dWindows = [
    { id: "procedural-main", bodyKey: "procedural.play.main" },
    { id: "procedural-preview", bodyKey: "procedural.play.preview" },
  ];
  const historyPanel = [{ kind: { kind: "app" as const, id: "framework.panel.history" }, bodyKey: FRAMEWORK_HISTORY_BODY_KEY }];

  it("a full-scope completion re-takes the flow window body, and a history-only completion never does", () => {
    const full = typedOperationCompletionRefreshV1({ uiScope: { kind: "full" }, historyPatch: { cursor: 4 }, requestedEffects: [] });
    expect(full).toEqual({ kind: "full" });
    expect(buildUiRefreshRequest(full!, generation3dWindows, historyPanel, {}, new Map())?.windows?.map((entry) => entry.bodyKey)).toEqual(["procedural.play.main", "procedural.play.preview"]);
    const historyOnly = typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: { cursor: 4 }, requestedEffects: [] });
    expect(historyOnly).toEqual({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] });
    expect(buildUiRefreshRequest(historyOnly!, generation3dWindows, historyPanel, {}, new Map())?.windows ?? []).toEqual([]);
  });

  it("the guest's new graph replaces the cached flow body while the untouched preview body keeps its identity", () => {
    const cache: UiRefreshCache = new Map([
      ["window:procedural-main", { hash: "hex-column-hash", value: { type: "nodeGraph", value: "hexagonal-mushroom-column" } }],
      ["window:procedural-preview", { hash: "preview-hash", value: { type: "world3d", value: "preview" } }],
    ]);
    const previousBodies = { "procedural-main": cache.get("window:procedural-main")!.value, "procedural-preview": cache.get("window:procedural-preview")!.value } as Readonly<Record<string, unknown>>;
    const request = buildUiRefreshRequest({ kind: "full" }, generation3dWindows, historyPanel, {}, cache);
    expect(request?.windows?.find((entry) => entry.key === "procedural-main")?.hash).toBe("hex-column-hash");
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "procedural-main", hash: "box-shell-hash", value: { type: "nodeGraph", value: "box-shell-preview" } },
        { key: "procedural-preview", hash: "preview-hash" },
      ],
    });
    const merged = mergeRecordPreservingIdentity(
      previousBodies,
      generation3dWindows.map((instance) => [instance.id, cache.get(`window:${instance.id}`)?.value ?? previousBodies[instance.id]] as const),
    );
    expect(merged["procedural-main"]).toEqual({ type: "nodeGraph", value: "box-shell-preview" });
    expect(merged["procedural-preview"]).toBe(previousBodies["procedural-preview"]);
  });

  it("a guest that re-renders the PREVIOUS example answers the same hash, so the shell keeps showing it — the failure this law names", () => {
    const cache: UiRefreshCache = new Map([["window:procedural-main", { hash: "hex-column-hash", value: { type: "nodeGraph", value: "hexagonal-mushroom-column" } }]]);
    applyUiRefreshResponseToCache(cache, { windows: [{ key: "procedural-main", hash: "hex-column-hash" }] });
    expect(cache.get("window:procedural-main")?.value).toEqual({ type: "nodeGraph", value: "hexagonal-mushroom-column" });
  });

  it("a refresh asked for while one is already crossing is coalesced onto the union, never dropped", () => {
    expect(mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "partial", windowBodies: ["procedural.play.preview"], measures: true })).toEqual({
      kind: "partial",
      windowBodies: ["procedural.play.main", "procedural.play.preview"],
      panelBodies: [],
      utilities: false,
      tools: false,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "full" })).toEqual({ kind: "full" });
    expect(mergeUiDirtyScopeV1({ kind: "full" }, { kind: "partial", windowBodies: [] })).toEqual({ kind: "full" });
    expect(mergeUiDirtyScopeV1({ kind: "none" }, { kind: "partial", windowBodies: ["procedural.play.main"] })).toEqual({ kind: "partial", windowBodies: ["procedural.play.main"] });
    expect(mergeUiDirtyScopeV1({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] }, { kind: "none" })).toEqual({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] });
    // ⚖️ The union of two window-body passes still asks the guest for BOTH bodies — the whole point
    // of coalescing instead of letting the newer pass supersede (and discard) the older one.
    const union = mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "partial", windowBodies: ["procedural.play.preview"] });
    expect(buildUiRefreshRequest(union, generation3dWindows, historyPanel, {}, new Map())?.windows?.map((entry) => entry.bodyKey)).toEqual(["procedural.play.main", "procedural.play.preview"]);
  });
});
//#endregion 🎨️ExampleSwitchHostCaching
