// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, render, screen } from "@semio-tech/ui-react/test";
import { Mode, uiDataLabel, type WindowLayoutNode } from "@semio-tech/ui-react";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { isIconName } from "@semio-tech/assets";
import { createWorldProjectionTemplates, decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId, worldProjectionSpecIconId, worldProjectionSpecLabel, worldProjectionTemplateApplySpec, worldProjectionTemplateSelectionId, type WorldProjectionSpec, type WorldProjectionTemplateDescriptor } from "@semio-tech/infinite-world-r3f";
import { afterEach, describe, expect, it, vi } from "vitest";
import { resolveFrameworkLayoutSeed, retitleWindowLayoutNode } from "../../🟦️.tsx";
import schema from "../../🧬️schema/🌐️instance-title/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🌐️instance-title/🔣️.json" with { type: "json" };
import { initialShellState, shellReducer, type ActiveSession } from "../../../🐚️Shell/🟦️.tsx";

function leaves(node: WindowLayoutNode): Extract<WindowLayoutNode, { kind: "window" }>[] {
  return node.kind === "window" ? [node] : node.children.flatMap(leaves);
}

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

describe("locale-stable instance titles", () => {
  it("paints the concrete projection title for a newly transferred window template", () => {
    vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
    vi.stubGlobal("MutationObserver", class { observe(): void {} disconnect(): void {} takeRecords(): MutationRecord[] { return []; } });
    const windows = fixture.instances.map(row => {
      const spec = decodeWorldProjectionTemplateId(encodeWorldProjectionTemplateId(row.initialProjection as WorldProjectionSpec))!;
      const title = worldProjectionSpecLabel(spec);
      const iconId = worldProjectionSpecIconId(spec);
      expect(title).toBe(row.projectionTitle);
      expect(iconId).toBe(row.projectionIcon);
      if (!isIconName(iconId)) throw new Error(`Unknown projection icon: ${iconId}`);
      return { id: row.id, title: uiDataLabel(title), iconId, children: <div>{row.id}</div> };
    });
    const layout: WindowLayoutNode = { kind: "row", children: windows.map(window => ({ kind: "stack", children: [{ kind: "window", id: window.id, title: window.title }] })) };
    render(<Mode windows={windows} layout={layout} activeWindowId={windows[0]!.id} />);
    for (const row of fixture.instances) expect(screen.getByRole("tab", { name: row.projectionTitle, exact: true })).toBeTruthy();
  });

  it("uses the concrete projection selected by each template branch for window chrome", () => {
    const flatten = (rows: readonly WorldProjectionTemplateDescriptor[]): readonly WorldProjectionTemplateDescriptor[] => rows.flatMap(row => [row, ...flatten(row.children ?? [])]);
    const templates = flatten(createWorldProjectionTemplates({ controllerId: "projection-fixture" }));
    for (const orientation of fixture.retainedOrientations) for (const row of fixture.projectionBranches) {
      const current: WorldProjectionSpec = { mode: { kind: "threePoint", fov: 50 }, orientation: orientation as WorldProjectionSpec["orientation"] };
      const spec = worldProjectionTemplateApplySpec(templates.find(template => template.id === row.pressed)!.args.spec, current);
      expect(worldProjectionTemplateSelectionId(spec)).toBe(row.effective);
      expect(spec.orientation).toEqual(orientation);
      expect(worldProjectionSpecLabel(spec)).toBe(row.title);
      expect(worldProjectionSpecIconId(spec)).toBe(row.icon);
    }
  });

  it("retains explicit base titles across locales while unrenamed singletons still localize", () => {
    const windowKinds = [
      { id: fixture.windowKind, label: "Puzzle 3D" },
      { id: fixture.singleton.id, label: { native: { en: fixture.singleton.en, de: fixture.singleton.de }, reuse: { en: fixture.singleton.en, de: fixture.singleton.de } } },
    ];
    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    state = shellReducer(state, { type: "SET_SHELL_LAYOUT", value: { kind: "row", children: windowKinds.map(kind => ({ kind: "stack", children: [{ kind: "window", id: kind.id }] })) } });
    state = shellReducer(state, { type: "SET_WINDOW_TITLE", windowId: fixture.windowKind, title: fixture.explicitBaseTitle });
    for (const transition of fixture.transitions) {
      const layout = retitleWindowLayoutNode(state.layout.shellLayout!, windowKinds, [], "native", transition.locale, state.layout.windowTitlesById);
      expect(leaves(layout).map(leaf => leaf.title)).toEqual([fixture.explicitBaseTitle, fixture.singleton[transition.locale as "en" | "de"]]);
      state = shellReducer(state, { type: "SET_SHELL_LAYOUT", value: layout });
    }
  });

  it("keeps chrome overrides within their session owner across updates and switches", () => {
    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    for (const owner of fixture.owners) {
      const session = { pluginId: owner.pluginId, instanceId: owner.instanceId, app: { id: owner.appId }, viewState: {} } as ActiveSession;
      state = shellReducer(state, { type: "SET_SESSION", value: session });
      expect(state.layout.windowTitlesById).toEqual({});
      expect(state.layout.windowIconsById).toEqual({});
      state = shellReducer(state, { type: "SET_WINDOW_TITLE", windowId: fixture.windowKind, title: fixture.explicitBaseTitle });
      const iconId = worldProjectionSpecIconId(fixture.instances[0]!.initialProjection as WorldProjectionSpec);
      expect(iconId).toBe(fixture.instances[0]!.projectionIcon);
      if (!isIconName(iconId)) throw new Error(`Unknown projection icon: ${iconId}`);
      state = shellReducer(state, { type: "SET_WINDOW_ICON", windowId: fixture.windowKind, iconId });
      const chrome = state.layout;
      state = shellReducer(state, { type: "SET_SESSION", value: current => current ? { ...current, viewState: { ...current.viewState, activeModeId: "other" } } : current });
      expect(state.layout).toBe(chrome);
    }
    state = shellReducer(state, { type: "SET_SESSION", value: null });
    expect(state.layout.windowTitlesById).toEqual({});
    expect(state.layout.windowIconsById).toEqual({});
  });

  it("records explicit title changes in the existing layout without changing siblings or geometry", () => {
    const layout: WindowLayoutNode = { kind: "row", children: [
      ...fixture.instances.map(instance => ({ kind: "stack" as const, size: 30, children: [{ kind: "window" as const, id: instance.id, title: uiDataLabel(instance.title) }] })),
      { kind: "stack" as const, size: 40, children: [{ kind: "window" as const, id: fixture.singleton.id, title: uiDataLabel(fixture.singleton.en) }] },
    ] };
    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    state = shellReducer(state, { type: "SET_EXTRA_WINDOW_INSTANCES", value: fixture.instances.map(instance => ({ id: instance.id, title: instance.title, windowKindId: fixture.windowKind })) });
    state = shellReducer(state, { type: "SET_SHELL_LAYOUT", value: layout });
    for (const rename of fixture.renames) {
      const before = state;
      state = shellReducer(state, { type: "SET_WINDOW_TITLE", ...rename });
      expect(state.layout.windowTitlesById[rename.windowId]).toBe(rename.title);
      expect(leaves(state.layout.shellLayout!).find(leaf => leaf.id === rename.windowId)?.title).toBe(rename.title);
      expect(leaves(state.layout.shellLayout!).filter(leaf => leaf.id !== rename.windowId)).toEqual(leaves(before.layout.shellLayout!).filter(leaf => leaf.id !== rename.windowId));
      const shellLayout = state.layout.shellLayout!;
      expect(shellLayout.kind === "row" && shellLayout.children.map(child => "size" in child && child.size)).toEqual([30, 30, 40]);
      expect(state.overlays).toBe(before.overlays);
    }
  });

  it("keeps authored instance names while the actual React Dock localizes singleton windows", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
    vi.stubGlobal("MutationObserver", class { observe(): void {} disconnect(): void {} takeRecords(): MutationRecord[] { return []; } });
    const windowKinds = [
      { id: fixture.windowKind, label: { native: { en: "Puzzle 3D", de: "Puzzle 3D" }, reuse: { en: "Puzzle 3D", de: "Puzzle 3D" } } },
      { id: fixture.singleton.id, label: { native: { en: fixture.singleton.en, de: fixture.singleton.de }, reuse: { en: fixture.singleton.en, de: fixture.singleton.de } } },
    ];
    const seed = resolveFrameworkLayoutSeed({
      root: {
        kind: "row",
        children: [
          ...fixture.instances.map(instance => ({ kind: "stack" as const, size: 30, children: [{ kind: "window" as const, windowKindId: fixture.windowKind, instanceId: instance.id, title: instance.title }] })),
          { kind: "stack", size: 40, children: [{ kind: "window", windowKindId: fixture.singleton.id }] },
        ],
      },
    }, windowKinds, { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} }, "native", "en");
    const instancesBefore = structuredClone(seed.extraInstances);
    expect(leaves(seed.modeLayout).map(node => node.title)).toEqual(fixture.transitions[0]!.titles);
    let layout = seed.modeLayout;
    const view = render(<div />);
    for (const transition of fixture.transitions) {
      layout = retitleWindowLayoutNode(layout, windowKinds, seed.extraInstances, "native", transition.locale, {});
      const windows = leaves(layout).map(node => ({ id: node.id, title: node.title!, iconId: "app-window" as const, children: <div>{node.id}</div> }));
      expect(windows.map(window => window.title)).toEqual(transition.titles);
      expect(seed.extraInstances).toEqual(instancesBefore);
      expect(layout.kind === "window" ? [] : layout.children.map(child => "size" in child ? child.size : undefined)).toEqual([30, 30, 40]);
      view.rerender(<Mode windows={windows} activeWindowId={fixture.instances[0]!.id} layout={layout} />);
      expect(screen.getAllByRole("tab")).toHaveLength(3);
      for (const title of transition.titles) expect(screen.getByRole("tab", { name: title, exact: true })).toBeTruthy();
    }
  }, 15_000);
});
