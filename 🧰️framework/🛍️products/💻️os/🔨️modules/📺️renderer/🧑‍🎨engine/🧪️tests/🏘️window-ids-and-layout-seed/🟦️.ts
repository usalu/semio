/** 🏘 Per-window element ids, layout seeds, layout change classification and shell command notes. */
import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { childElementId, isElementId } from "@semio-tech/ui-react";
import { createWorldProjectionTemplates, worldProjectionSwitchTreeItems } from "@semio-tech/infinite-world-r3f";
import { windowMeasuresChrome, windowMeasureDomId, qualifyWindowMeasureIds } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { createInstance as createTranslationOracle } from "i18next";
import labelResolutionFixture from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🔣️label-resolution.json";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { type WindowMeasure } from "@semio-tech/framework";
import { uiDataLabel } from "@semio-tech/ui-react";
import { WorldOrbitProjectionSwitchPane, world3dProjectionPaneElementId } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { buildUiRefreshRequest, dispatchOsCommand, classifyWindowLayoutChange, buildNoteShellCommandAction, isShellOwnedCommandId, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS, resolveFrameworkLayoutSeed, retitleWindowLayoutNode, windowMeasureTreeContainsId, FrameworkOsShell, TutorialRecorder, synthesizeLocalizedLabel, resolveManifestLabel } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { encodeWorldProjectionTemplateId } from "@semio-tech/infinite-world-r3f";
import { rendererExport } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("per-window element ids", () => {
  const domIds = (markup: string): string[] => Array.from(markup.matchAll(/\sid="([^"]+)"/g)).map((match) => match[1]!);
  const duplicates = (ids: readonly string[]): string[] => ids.filter((id, index) => ids.indexOf(id) !== index);
  const threePoint = { mode: { kind: "threePoint" as const, fov: 50 }, orientation: { type: "free" as const } };
  const mountUnfoldedPane = (windowElementSegment: string): void => {
    render(createElement(WorldOrbitProjectionSwitchPane, { spec: threePoint, onSpecChange: () => undefined, windowElementSegment }) as ReactElement);
    const toggle = document.getElementById(childElementId(world3dProjectionPaneElementId(windowElementSegment), "pane", "fold"));
    expect(toggle).not.toBeNull();
    fireEvent.click(toggle!);
  };
  const mountedDomIds = (): string[] => Array.from(document.querySelectorAll("[id]")).map((element) => element.id);
  const authoredMeasures = (): WindowMeasure[] => [
    {
      kind: "group",
      id: "puzzle3d-measure-projection-parallel",
      label: "Parallel",
      defaultOpen: true,
      children: [
        { kind: "select", id: "puzzle3d-measure-projection-orthographic-view", value: "plan", items: [{ id: "plan", value: "plan", label: "Plan" }], onChange: { controllerId: "puzzle3d-play", action: "setProjection" } },
        { kind: "slider", id: "puzzle3d-measure-projection-oblique-angle", label: "Angle", value: 45, min: 5, max: 90, step: 1, onChange: { controllerId: "puzzle3d-play", action: "setProjectionParam" } },
      ],
    },
    { kind: "toggle", id: "puzzle3d-play-grid-visible", iconId: "layout-grid", label: "Visible", pressed: true, onChange: { controllerId: "puzzle3d-play", action: "setGridVisible" } },
  ];
  const measuresMarkup = (windowId: string): string => renderToStaticMarkup(windowMeasuresChrome(authoredMeasures(), undefined, windowId, () => undefined).measures as ReactElement);

  // 🪪️ Wave B41: `framework.worldOrbit.projection` was a HARDCODED id on a pane rendered once per world
  // surface, and a world surface is mounted once per open window instance — so the two-pane default layout
  // put the pane root and both of `Pane`'s derived fold-control ids in the document TWICE, and every row of
  // the projection switch tree with them. Two surfaces must share no DOM id at all.
  it("renders two world surfaces with no duplicate DOM id", () => {
    mountUnfoldedPane("puzzle3d-main-top");
    const top = mountedDomIds();
    mountUnfoldedPane("puzzle3d-main-perspective");
    const all = mountedDomIds();
    const perspective = all.filter((id) => !top.includes(id));
    // 🌲️ Unfolded, each pane carries its root, its fold toggle, its close control and every projection
    // switch row — the whole id surface the folded rail used to hide.
    expect(top.length).toBeGreaterThan(8);
    expect(perspective.length).toBe(top.length);
    expect(duplicates(all)).toEqual([]);
    expect(all).toContain(world3dProjectionPaneElementId("puzzle3d-main-top"));
    expect(all).toContain(world3dProjectionPaneElementId("puzzle3d-main-perspective"));
    expect(all.every((id) => isElementId(id))).toBe(true);
    cleanup();
  });

  it("styles the projection pane body like window options — transparent payload, no nested ribbon glass", () => {
    mountUnfoldedPane("puzzle3d-main-top");
    const paneId = world3dProjectionPaneElementId("puzzle3d-main-top");
    const body = document.getElementById(paneId)?.querySelector('[data-slot="pane-body"]') as HTMLElement | null;
    const switchRoot = document.querySelector("[data-world-projection-kind-switch]") as HTMLElement | null;
    expect(body?.className).toContain("p-tiny");
    expect(body?.hasAttribute("data-window-silhouette-content")).toBe(true);
    expect(switchRoot?.getAttribute("data-level")).toBeNull();
    expect(switchRoot?.className.includes("rounded-md")).toBe(false);
    cleanup();
  });

  // 🌲️ The pure half of the same invariant, so a regression is reported by the id builder and not only by
  // a mounted pane: every projection-switch row id is a child of the pane that owns it.
  it("qualifies every projection switch row id by its owning pane", () => {
    const templates = createWorldProjectionTemplates({ controllerId: "projection-switch" });
    const rows = (paneId: string): string[] => {
      const walk = (items: readonly { readonly id: string; readonly items?: readonly unknown[] }[]): string[] =>
        items.flatMap((item) => [item.id, ...walk((item.items ?? []) as readonly { readonly id: string; readonly items?: readonly unknown[] }[])]);
      return walk(worldProjectionSwitchTreeItems(paneId, templates, () => undefined) as readonly { readonly id: string; readonly items?: readonly unknown[] }[]);
    };
    const top = rows(world3dProjectionPaneElementId("puzzle3d-main-top"));
    const perspective = rows(world3dProjectionPaneElementId("puzzle3d-main-perspective"));
    expect(top.length).toBeGreaterThan(8);
    expect(duplicates([...top, ...perspective])).toEqual([]);
    expect(top.every((id) => id.startsWith(`${world3dProjectionPaneElementId("puzzle3d-main-top")}.`) && isElementId(id))).toBe(true);
  });

  // 🪪️ The same defect on the measures rail: a window kind's measure tree is authored ONCE for the kind
  // (puzzle3d passes the literal `"puzzle3d"` prefix to `world3d_projection_measures`) and rendered once
  // per instance, so the authored ids collided as soon as a second rail was unfolded.
  it("renders two windows' measures rails with no duplicate DOM id and keeps the authored id as the tail", () => {
    const top = domIds(measuresMarkup("puzzle3d-main-top"));
    const perspective = domIds(measuresMarkup("puzzle3d-main-perspective"));
    expect(top.length).toBeGreaterThan(1);
    expect(duplicates([...top, ...perspective])).toEqual([]);
    expect(top).toContain(windowMeasureDomId("puzzle3d-main-top", "puzzle3d-play-grid-visible"));
    expect(perspective).toContain(windowMeasureDomId("puzzle3d-main-perspective", "puzzle3d-play-grid-visible"));
    for (const authored of ["puzzle3d-play-grid-visible", "puzzle3d-measure-projection-orthographic-view"]) {
      expect(perspective.filter((id) => id.endsWith(`/${authored}`))).toHaveLength(1);
    }
  });

  // 🎓️ Qualification must not reach the AUTHORED tree: `windowMeasureTreeContainsId` (which decides whose
  // window an introduction anchor belongs to) and the program's own `activeUtilityId` routing both speak
  // the authored id, so the qualifier is a pure projection that leaves its input untouched.
  it("qualifies ids without mutating the authored measure tree or any other field", () => {
    const authored = authoredMeasures();
    const qualified = qualifyWindowMeasureIds(authored, "puzzle3d-main-top");
    expect(authored).toEqual(authoredMeasures());
    expect(windowMeasureTreeContainsId(authored, "puzzle3d-play-grid-visible")).toBe(true);
    const group = qualified[0] as Extract<WindowMeasure, { kind: "group" }>;
    expect(group.id).toBe("puzzle3d-main-top/puzzle3d-measure-projection-parallel");
    expect(group.label).toBe("Parallel");
    expect(group.children.map((child) => child.id)).toEqual(["puzzle3d-main-top/puzzle3d-measure-projection-orthographic-view", "puzzle3d-main-top/puzzle3d-measure-projection-oblique-angle"]);
    expect(group.children[0]!.onChange).toEqual(authoredMeasures()[0]!.kind === "group" ? (authoredMeasures()[0] as Extract<WindowMeasure, { kind: "group" }>).children[0]!.onChange : undefined);
  });
});

describe("resolveFrameworkLayoutSeed — multi-pane default layouts", () => {
  const emptyLabels = {
    windowKindLabels: {},
    panelTabLabels: {},
    modeLabels: {},
    actionLabels: {},
    utilityLabels: {},
    exampleLabels: {},
    actionArgLabels: {},
    dialogLabels: {},
    introductionLabels: {},
    groupLabels: {},
  };

  it("does not infer focus when an app has no explicit layout", () => {
    const seed = resolveFrameworkLayoutSeed(undefined, [{ id: "main", label: "Main" }], emptyLabels, "native", "en");
    expect(seed.modeLayout).toEqual({ kind: "stack", children: [{ kind: "window", id: "main" }] });
    expect(seed).not.toHaveProperty("activeWindowId");
  });

  it("hydrates Top (1/3) + Perspective (2/3) instances and projection templates", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
      "native",
      "en",
    );
    expect(seed.modeLayout).toEqual({
      kind: "row",
      children: [
        { kind: "stack", size: 100 / 3, children: [{ kind: "window", id: "puzzle3d-main-top", title: "Top" }] },
        { kind: "stack", size: 200 / 3, children: [{ kind: "window", id: "puzzle3d-main-perspective", title: "Perspective" }] },
      ],
    });
    expect(seed.extraInstances).toEqual([
      { id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" },
      { id: "puzzle3d-main-perspective", windowKindId: "puzzle3d-main", title: "Perspective" },
    ]);
    expect(seed).not.toHaveProperty("activeWindowId");
    expect(seed.pendingProjections).toEqual([
      { windowId: "puzzle3d-main-top", templateId: topTemplate },
      { windowId: "puzzle3d-main-perspective", templateId: perspectiveTemplate },
    ]);
  });

  it("treats instance-id panes as extras so the host fetches bodies keyed by instance id, not only by kind", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
      "native",
      "en",
    );
    // 🪟️ A refresh that only knows the bare kind id would leave Top/Perspective as "Fehlendes Fenster".
    // Live extras must be in the fetch list: base kind + each default-layout instance.
    const windowInstances = [{ id: "puzzle3d-main", bodyKey: "puzzle3d.play.composite" }, ...seed.extraInstances.map((entry) => ({ id: entry.id, bodyKey: "puzzle3d.play.composite" }))];
    const request = buildUiRefreshRequest({ kind: "full" }, windowInstances, [], {}, new Map());
    expect(request?.windows?.map((window) => window.key)).toEqual(["puzzle3d-main", "puzzle3d-main-top", "puzzle3d-main-perspective"]);
  });

  it("re-derives window titles from localized windowKind labels on locale/terminology switch", () => {
    const windowKinds = [
      {
        id: "main",
        label: {
          native: { en: "Main Window", de: "Hauptfenster" },
          reuse: { en: "Main Component", de: "Hauptkomponente" },
        },
      },
    ];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "main", title: uiDataLabel("Main Window") }],
    };

    const retitledDeNative = retitleWindowLayoutNode(layout, windowKinds, [], "native", "de", {});
    expect(retitledDeNative).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptfenster" }],
    });

    const retitledDeReuse = retitleWindowLayoutNode(layout, windowKinds, [], "reuse", "de", {});
    expect(retitledDeReuse).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptkomponente" }],
    });
  });

  it("preserves authored titles for extra window instances on locale changes", () => {
    const windowKinds = [
      {
        id: "puzzle3d-main",
        label: {
          native: { en: "3D Editor", de: "3D-Editor" },
          reuse: { en: "3D Component", de: "3D-Komponente" },
        },
      },
    ];
    const extraInstances = [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "puzzle3d-main-top", title: uiDataLabel("Top") }],
    };

    const retitled = retitleWindowLayoutNode(layout, windowKinds, extraInstances, "reuse", "de", {});
    expect(retitled).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "puzzle3d-main-top", title: "Top" }],
    });
  });
});

describe("classifyWindowLayoutChange", () => {
  const twoStackRow = (leftSize: number, rightSize: number) => ({
    kind: "row" as const,
    size: 100,
    children: [
      { kind: "stack" as const, size: leftSize, children: [{ kind: "window" as const, id: "a" }] },
      { kind: "stack" as const, size: rightSize, children: [{ kind: "window" as const, id: "b" }] },
    ],
  });

  it("returns null when the layout is identical (deep-equal, not just same reference)", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(50, 50))).toBeNull();
    const same = twoStackRow(50, 50);
    expect(classifyWindowLayoutChange(same, same)).toBeNull();
  });

  it("returns null for a pure active-window-flag change (skeleton and sizes both unchanged)", () => {
    const previous = {
      kind: "stack" as const,
      size: 100,
      activeId: "a",
      children: [
        { kind: "window" as const, id: "a" },
        { kind: "window" as const, id: "b" },
      ],
    };
    const next = { ...previous, activeId: "b" };
    expect(classifyWindowLayoutChange(previous, next)).toBeNull();
  });

  it("returns 'resize' when only pane sizes differ", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(30, 70))).toBe("resize");
  });

  it("returns 'rearrange' when window ids/nesting structure differ (drag-to-new-position, split, close)", () => {
    const previous = twoStackRow(50, 50);
    const swapped = { ...previous, children: [previous.children[1]!, previous.children[0]!] };
    expect(classifyWindowLayoutChange(previous, swapped)).toBe("rearrange");
    const closed = { kind: "stack" as const, children: [{ kind: "window" as const, id: "a" }] };
    expect(classifyWindowLayoutChange(previous, closed)).toBe("rearrange");
    expect(classifyWindowLayoutChange(null, previous)).toBe("rearrange");
    expect(classifyWindowLayoutChange(previous, null)).toBe("rearrange");
  });

  it("returns null when both are null", () => {
    expect(classifyWindowLayoutChange(null, null)).toBeNull();
  });
});

describe("noteShellCommand", () => {
  /** ⏪️ A chrome note is an UNDO TARGET only when the caller declares a real inverse. `detail` says
   * where the chrome WENT, never where it came from, so the old unconditional
   * `inverseCommandId: commandId` / `inverseArgs: detail` pair was an identity, not an inverse: the
   * guest pushed every window activation onto the chrome undo stack and the first `undo` after a
   * click popped it and asked the host to replay `shell.windowActivate` into the app, where the
   * window-kind gate refused it `undeclared-action` (ticket 26/09/18 §3.2). */
  it("buildNoteShellCommandAction carries no inverse unless the caller declares one", () => {
    expect(buildNoteShellCommandAction("puzzle3d-play", "shell.windowClose", { en: "Close Window", de: "Fenster schließen" }, { windowId: "w1" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "shell.windowClose", label: { en: "Close Window", de: "Fenster schließen" }, detail: { windowId: "w1" } },
    });
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.resetDock", { en: "Reset Panels", de: "Panels zurücksetzen" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.resetDock", label: { en: "Reset Panels", de: "Panels zurücksetzen" } },
    });
  });

  it("buildNoteShellCommandAction carries a declared inverse, and its arguments only when the inverse has some", () => {
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.setThemeId", { en: "Set Theme", de: "Design festlegen" }, { themeId: "dark" }, { commandId: "os.setThemeId", args: { themeId: "light" } })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.setThemeId", label: { en: "Set Theme", de: "Design festlegen" }, detail: { themeId: "dark" }, inverseCommandId: "os.setThemeId", inverseArgs: { themeId: "light" } },
    });
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.resetDock", { en: "Reset Panels", de: "Panels zurücksetzen" }, undefined, { commandId: "os.resetDock" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.resetDock", label: { en: "Reset Panels", de: "Panels zurücksetzen" }, inverseCommandId: "os.resetDock" },
    });
  });

  /** 🐚️ Every chrome id the React shell notes is shell-owned, so none of them may ever reach the
   * guest through `Effect::ReplayShellCommand` — a plugin action id (the `View`-kind rows) must. */
  it("isShellOwnedCommandId separates chrome the shell replays itself from plugin actions the guest replays", () => {
    for (const commandId of [
      "shell.windowActivate",
      "shell.windowResize",
      "shell.windowMove",
      "shell.windowClose",
      "shell.windowSplit",
      "shell.windowOpenInNewWindow",
      "shell.panelToggle",
      "shell.panelTab",
      "shell.dockMove",
      "os.setThemeId",
      "os.resetDock",
      "os.resizeWindow",
    ]) {
      expect(isShellOwnedCommandId(commandId)).toBe(true);
    }
    for (const actionId of ["setActiveExample", "patchNodes", "change-seed", "addNode"]) {
      expect(isShellOwnedCommandId(actionId)).toBe(false);
    }
  });

  /** 🎛️ …and every shell-owned id the shell notes through `dispatchOsCommand` really is routable
   * there, so the replay branch's "no shell route" warning means a genuinely missing route. */
  it("dispatchOsCommand reports whether it routed the command", () => {
    const noop = (): void => {};
    const stores = { reset: noop } as unknown as Parameters<typeof dispatchOsCommand>[4];
    expect(dispatchOsCommand("os.resetDock", undefined, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(true);
    expect(dispatchOsCommand("os.setThemeId", { themeId: "light" }, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(true);
    expect(dispatchOsCommand("shell.windowActivate", { windowId: "w1" }, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(false);
  });

  it("is excluded from tutorial recording, alongside world-navigation/introduction/tutorial-control action ids", () => {
    expect(TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has("noteShellCommand")).toBe(true);
  });
});

describe("TutorialRecorder LocalizedLabel synthesis", () => {
  it("synthesizeLocalizedLabel broadcasts a string across all 4 cells (native/reuse x en/de)", () => {
    const label = synthesizeLocalizedLabel("Test Chapter");
    expect(label).toEqual({
      native: { en: "Test Chapter", de: "Test Chapter" },
      reuse: { en: "Test Chapter", de: "Test Chapter" },
    });
    expect(synthesizeLocalizedLabel(label)).toBe(label);
  });

  it("resolves exact language-neutral label cells without a default locale", async () => {
    const validate = rendererExport("ShellLabelResolutionLanguages");
    expect(Object.values(labelResolutionFixture.matrix).every((labels) => validate(labels)), JSON.stringify(validate.errors)).toBe(true);
    const matrix = labelResolutionFixture.matrix;
    const oracle = createTranslationOracle();
    await oracle.init({
      fallbackLng: false,
      resources: {
        en: { native: { label: matrix.native.en }, reuse: { label: matrix.reuse.en } },
        de: { native: { label: matrix.native.de }, reuse: { label: matrix.reuse.de } },
      },
    });
    for (const row of labelResolutionFixture.cases) {
      expect(resolveManifestLabel(row.data ?? matrix, row.terminology, row.locale), row.id).toBe(row.expected);
      const expected = row.data ?? oracle.getResource(row.locale, row.terminology, "label") ?? "";
      expect(expected, row.id).toBe(row.expected);
    }
    for (const malformed of [null, 7, [], {}, { native: { en: "English" }, reuse: { en: "English" } }, { native: { en: 7, de: false }, reuse: { en: "Part", de: "Bauteil" } }]) {
      expect(resolveManifestLabel(malformed, "native", "de")).toBe("");
    }
    expect(validate({ en: "Only English" })).toBe(false);
  });

  it("TutorialRecorder synthesizes LocalizedLabel for addChapter and build titles", () => {
    const recorder = new TutorialRecorder({ activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false }, null);
    recorder.addChapter("Introduction");
    recorder.addChapter();
    const def = recorder.build("rec-1", "Recorded Tutorial");

    expect(def.title).toEqual({
      native: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
      reuse: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
    });
    expect(def.chapters[0].title).toEqual({
      native: { en: "Introduction", de: "Introduction" },
      reuse: { en: "Introduction", de: "Introduction" },
    });
    expect(def.chapters[1].title).toEqual({
      native: { en: "Chapter 2", de: "Chapter 2" },
      reuse: { en: "Chapter 2", de: "Chapter 2" },
    });

    expect(resolveManifestLabel(def.title, "native", "en")).toBe("Recorded Tutorial");
    expect(resolveManifestLabel(def.chapters[0].title, "reuse", "de")).toBe("Introduction");
  });

  it("FrameworkOsShell portal layer is unconstrained by z-tutorial so portaled elements sit above elevated windows", () => {
    if (!window.matchMedia) {
      window.matchMedia = (() => ({
        matches: false,
        media: "",
        onchange: null,
        addListener: () => {},
        removeListener: () => {},
        addEventListener: () => {},
        removeEventListener: () => {},
        dispatchEvent: () => false,
      })) as unknown as typeof window.matchMedia;
    }
    const backboneWorkerFactory = () => Object.assign(new EventTarget(), { onmessage: null, postMessage: vi.fn(), terminate: vi.fn() }) as unknown as Worker;
    const { container } = render(createElement(FrameworkOsShell, { catalog: { plugins: [], extensions: [], hosts: [], playgrounds: [], moduleUrl: () => { throw new Error("absent supplied module"); }, extensionModuleUrl: () => { throw new Error("absent supplied extension"); } }, plugins: [], appId: "test", backboneWorkerFactory }));
    const portalLayer = container.querySelector("[data-semio-portal-layer]");
    expect(portalLayer).toBeTruthy();
    expect(portalLayer?.className).not.toContain("z-tutorial");
  });
});
